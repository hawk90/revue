//! What a widget does with the text it is given, over the whole catalog.
//!
//! The panic and out-of-area layers do not look at what is drawn. These
//! rules do, without knowing the expected screen:
//!
//! - a widget given a text shows it when there is room;
//! - a wide text lays out like an ASCII text of the same width: only the
//!   glyphs change, so a width counted in bytes or chars shows up;
//! - drawing again, or after drawing in a smaller area, draws the same;
//! - a wider area never hides a text a narrower one showed.
//!
//! Each rule lists the widgets it does not apply to, with the reason, and the
//! known bugs. A known bug that no longer fails fails the test, so the lists
//! only shrink.

use std::collections::BTreeSet;

use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};

use super::catalog::{catalog, Entry};
use super::sentinel::catch;
use super::Content;

/// Letters no widget draws on its own
const ASCII: &str = "QXZJ";
/// Two rare syllables, as wide as `ASCII`
const WIDE: &str = "갃갌";
const AREA: (u16, u16) = (80, 24);

/// Draws something else each time: the machine's load
const UNSTABLE: &[&str] = &["ProcessMonitor"];

fn draw(view: &dyn View, (w, h): (u16, u16)) -> Option<Buffer> {
    catch(|| {
        let mut buffer = Buffer::new(w, h);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        view.render(&mut ctx);
        buffer
    })
    .ok()
}

fn build(entry: &Entry, text: &str) -> Option<Box<dyn View>> {
    let content = Content::text("probe", text);
    catch(|| (entry.build)(&content, false)).ok()
}

fn drawn(entry: &Entry, text: &str, size: (u16, u16)) -> Option<Buffer> {
    build(entry, text).and_then(|view| draw(&*view, size))
}

fn cells(b: &Buffer) -> Vec<Cell> {
    (0..b.height())
        .flat_map(|y| (0..b.width()).map(move |x| (x, y)))
        .map(|(x, y)| *b.get(x, y).unwrap())
        .collect()
}

fn shows(b: &Buffer, text: &str) -> bool {
    (0..b.height()).any(|y| {
        let row: String = (0..b.width())
            .filter_map(|x| b.get(x, y))
            .filter(|c| !c.is_continuation())
            .map(|c| c.symbol)
            .collect();
        row.contains(text)
    })
}

/// The entries the rules run on
fn entries() -> Vec<Entry> {
    catalog()
        .into_iter()
        .filter(|e| !UNSTABLE.contains(&e.name))
        .collect()
}

/// Check that what fails is exactly the known bugs: a new failure and a known
/// bug that no longer fails both fail
fn ratchet(rule: &str, failing: Vec<&'static str>, known: &[(&str, &str)]) {
    let failing: BTreeSet<&str> = failing.into_iter().collect();
    let known: BTreeSet<&str> = known.iter().map(|(name, _)| *name).collect();
    let new: Vec<_> = failing.difference(&known).collect();
    let fixed: Vec<_> = known.difference(&failing).collect();
    assert!(
        new.is_empty() && fixed.is_empty(),
        "{rule}: new failures {new:?}; fixed, remove from the known list {fixed:?}"
    );
}

// ─── A widget given a text shows it ─────────────────────────────────────────

/// Widgets the text rule does not apply to
const SHOWS_NO_TEXT: &[(&str, &str)] = &[
    // The factory gives it no text
    ("Digits", "a number"),
    ("GradientBox", "no text"),
    ("Progress", "a ratio"),
    ("Skeleton", "placeholder lines"),
    ("HSplit", "no children"),
    ("VSplit", "no children"),
    ("Resizable", "no child"),
    ("ScrollView", "no child"),
    ("Scrollbar", "a position"),
    ("Splitter", "panes are ids"),
    ("ColorPicker", "no text"),
    ("Calendar", "a date"),
    ("DateTimePicker", "a date"),
    ("RangePicker", "dates"),
    ("Pagination", "a page count"),
    ("ThemePicker", "theme names"),
    ("DeclarativeRouter", "its own route text"),
    ("Timer", "a duration"),
    ("Sparkline", "numbers"),
    ("Canvas", "shapes"),
    ("BrailleCanvas", "shapes"),
    ("Image", "pixels"),
    ("DevToolsTimeTravel", "the text goes to other tabs"),
    // It draws the text in another form
    ("Avatar", "initials"),
    ("BigText", "block glyphs"),
    ("QrCodeWidget", "a QR code"),
    // It shows the text later, by design
    ("ToastQueue", "queued until the app ticks it"),
    ("AiStream", "typed out over time"),
    ("HttpClient", "rejects the URL and shows the error"),
];

/// Known bugs of the text rule
const SHOWS_TEXT_BUGS: &[(&str, &str)] = &[
    (
        "StatusIndicator",
        "#1528: label() alone draws nothing, the default style is a dot",
    ),
    (
        "DividerVertical",
        "#1528: the label is drawn only when horizontal",
    ),
];

#[test]
fn a_widget_given_a_text_shows_it() {
    let failing = entries()
        .into_iter()
        .filter(|e| !SHOWS_NO_TEXT.iter().any(|(name, _)| *name == e.name))
        .filter(|e| drawn(e, ASCII, AREA).is_some_and(|b| !shows(&b, ASCII)))
        .map(|e| e.name)
        .collect();
    ratchet("shows text", failing, SHOWS_TEXT_BUGS);
}

// ─── A wide text lays out like a narrow one ─────────────────────────────────

type Masked = (char, Option<Color>, Option<Color>, u8);

/// The cells with the text's glyphs replaced by one mark. A wide glyph's
/// second cell has no style of its own, and the terminal draws it in the
/// first one's, so it takes its left neighbor's.
fn masked(b: &Buffer, glyphs: &str) -> Vec<Masked> {
    const MARK: char = '\u{E001}';
    let width = usize::from(b.width());
    let mut out: Vec<Masked> = Vec::new();
    for (i, c) in cells(b).into_iter().enumerate() {
        let m = if c.is_continuation() {
            let left = if i % width > 0 {
                out[i - 1]
            } else {
                (MARK, None, None, 0)
            };
            (MARK, left.1, left.2, left.3)
        } else if glyphs.contains(c.symbol) {
            (MARK, c.fg, c.bg, c.modifier.bits())
        } else {
            (c.symbol, c.fg, c.bg, c.modifier.bits())
        };
        out.push(m);
    }
    out
}

/// Widgets that draw something derived from the text itself
const DERIVED: &[(&str, &str)] = &[
    ("Avatar", "the color comes from the name"),
    ("BigText", "no block glyphs for Hangul"),
    ("QrCodeWidget", "another code"),
    ("MaskedInput", "one mask glyph per char"),
];

/// Known bugs of the width rule
const WIDTH_BUGS: &[(&str, &str)] = &[
    ("Chart", "#1527: legend box sized in bytes"),
    ("PieChart", "#1527: legend box sized in bytes"),
    ("ScatterChart", "#1527: legend box sized in bytes"),
    ("TimeSeries", "#1527: legend steps one column per char"),
];

#[test]
fn a_wide_text_lays_out_like_a_narrow_one() {
    let failing = entries()
        .into_iter()
        .filter(|e| !DERIVED.iter().any(|(name, _)| *name == e.name))
        .filter(|e| match (drawn(e, ASCII, AREA), drawn(e, WIDE, AREA)) {
            (Some(a), Some(w)) => masked(&a, ASCII) != masked(&w, WIDE),
            _ => false,
        })
        .map(|e| e.name)
        .collect();
    ratchet("width", failing, WIDTH_BUGS);
}

// ─── Drawing again draws the same ───────────────────────────────────────────

#[test]
fn drawing_again_draws_the_same() {
    let mut failing = Vec::new();
    for e in entries() {
        let Some(fresh) = drawn(&e, ASCII, AREA) else {
            continue;
        };
        let Some(view) = build(&e, ASCII) else {
            continue;
        };
        // Twice in the same area, then after a smaller area
        let again = draw(&*view, AREA).and_then(|_| draw(&*view, AREA));
        let after_small = draw(&*view, (20, 6)).and_then(|_| draw(&*view, AREA));
        let same = |b: Option<Buffer>| b.is_some_and(|b| cells(&b) == cells(&fresh));
        if !same(again) || !same(after_small) {
            failing.push(e.name);
        }
    }
    ratchet("draws the same", failing, &[]);
}

// ─── A wider area keeps the text ────────────────────────────────────────────

#[test]
fn a_wider_area_keeps_the_text() {
    let mut failing = Vec::new();
    for e in entries() {
        let mut shown = false;
        for width in 1..=AREA.0 {
            let Some(b) = drawn(&e, ASCII, (width, AREA.1)) else {
                break;
            };
            let now = shows(&b, ASCII);
            if shown && !now {
                failing.push(e.name);
                break;
            }
            shown |= now;
        }
    }
    ratchet("wider keeps the text", failing, &[]);
}
