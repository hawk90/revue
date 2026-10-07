//! The two render invariants, usable from any test target: catching a panic
//! quietly, and rendering into a sentinel-filled buffer to catch writes
//! outside the area.
//!
//! Self-contained on purpose: `tests/event_sequences.rs` includes this file
//! with `#[path]`, so it must not depend on the rest of the matrix.

use std::cell::{Cell as StdCell, RefCell};
use std::panic::{self, AssertUnwindSafe};
use std::sync::Once;

use revue::dom::NodeState;
use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Style;
use revue::widget::{RenderContext, View};

const SENTINEL: char = '\u{E000}';

thread_local! {
    static QUIET: StdCell<bool> = const { StdCell::new(false) };
    static LOCATION: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Install (once) a panic hook that stays silent while a case runs on this
/// thread and defers to the previous hook otherwise. Tests run in parallel,
/// so the hook is never swapped back and forth.
fn install_quiet_hook() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if QUIET.with(StdCell::get) {
                let at = info
                    .location()
                    .map(|l| format!("{}:{}", l.file(), l.line()))
                    .unwrap_or_default();
                LOCATION.with(|l| *l.borrow_mut() = at);
            } else {
                previous(info);
            }
        }));
    });
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    let msg = if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic payload>".to_string()
    };
    msg.lines().next().unwrap_or("").chars().take(160).collect()
}

/// Run `f` with panics caught and the hook silenced; `Err` is
/// `"<message> @ <file:line>"`.
pub fn catch<R>(f: impl FnOnce() -> R) -> Result<R, String> {
    install_quiet_hook();
    let was = QUIET.with(|q| q.replace(true));
    let result = panic::catch_unwind(AssertUnwindSafe(f));
    QUIET.with(|q| q.set(was));
    result.map_err(|p| {
        let at = LOCATION.with(|l| std::mem::take(&mut *l.borrow_mut()));
        format!("{} @ {at}", panic_message(&*p))
    })
}

/// Render `view` into `area` of a `buffer`-sized buffer pre-filled with a
/// sentinel cell.
///
/// `Err` if rendering panicked; `Ok(Some(detail))` if a cell outside the
/// area changed; `Ok(None)` if both invariants hold.
pub fn render_checked(
    view: &dyn View,
    buffer: (u16, u16),
    area: Rect,
    focused: bool,
) -> Result<Option<String>, String> {
    let (bw, bh) = buffer;
    let mut buffer = Buffer::new(bw, bh);
    buffer.fill(0, 0, bw, bh, Cell::new(SENTINEL));
    let sentinel = *buffer.get(0, 0).expect("buffer has cells");

    catch(|| {
        if focused {
            let style = Style::default();
            let state = NodeState {
                focused: true,
                ..NodeState::default()
            };
            let mut ctx = RenderContext::full(&mut buffer, area, &style, &state);
            view.render(&mut ctx);
        } else {
            let mut ctx = RenderContext::new(&mut buffer, area);
            view.render(&mut ctx);
        }
    })?;

    // Rows entirely outside the area, then the margins of the rows inside it.
    let inside_x = area.x..area.x.saturating_add(area.width);
    let inside_y = area.y..area.y.saturating_add(area.height);
    for y in 0..bh {
        let row_inside = inside_y.contains(&y);
        for x in 0..bw {
            if row_inside && inside_x.contains(&x) {
                continue;
            }
            if buffer.get(x, y) != Some(&sentinel) {
                let what = match buffer.get(x, y) {
                    Some(c) if c.symbol == SENTINEL => "restyled the cell".to_string(),
                    Some(c) => format!("wrote {:?}", c.symbol),
                    None => "lost the cell".to_string(),
                };
                return Ok(Some(format!("{what} at ({x},{y}), area {area:?}")));
            }
        }
    }
    Ok(None)
}
