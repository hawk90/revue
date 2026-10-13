//! Autocomplete widget tests

mod builders;
mod filter;
mod input;
mod unicode;
mod view;

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::autocomplete::Autocomplete;
use revue::widget::traits::{RenderContext, View};

pub(super) fn key(a: &mut Autocomplete, k: Key) -> bool {
    a.handle_key(KeyEvent::new(k))
}

pub(super) fn type_str(a: &mut Autocomplete, s: &str) {
    for c in s.chars() {
        key(a, Key::Char(c));
    }
}

/// Render `a` into a `width` x `height` buffer.
pub(super) fn render(a: &Autocomplete, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    buffer
}

/// The row as a terminal shows it: a wide glyph's continuation cell is the
/// right half of that glyph, not a character of its own.
pub(super) fn row(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .filter_map(|x| match buffer.get(x, y) {
            Some(c) if c.is_continuation() => None,
            Some(c) => Some(c.symbol),
            None => Some(' '),
        })
        .collect()
}

pub(super) fn render_rows(a: &Autocomplete, width: u16, height: u16) -> Vec<String> {
    let buffer = render(a, width, height);
    (0..height).map(|y| row(&buffer, y, width)).collect()
}

/// Labels currently shown in the dropdown (rows below the input line).
pub(super) fn dropdown(a: &Autocomplete, width: u16, height: u16) -> Vec<String> {
    render_rows(a, width, height)
        .into_iter()
        .skip(1)
        .map(|r| r.trim_end().to_string())
        .filter(|r| !r.is_empty())
        .collect()
}

pub(super) fn fruits() -> Autocomplete {
    Autocomplete::new().suggestions(["apple", "application", "banana", "cherry"])
}

mod snapshots {

    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_autocomplete_basic() {
        use revue::widget::Autocomplete;

        let view = Autocomplete::new().placeholder("Search...").suggestions([
            "Apple",
            "Banana",
            "Cherry",
            "Date",
            "Elderberry",
        ]);

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("autocomplete_basic");
    }

    #[test]
    fn test_autocomplete_with_value() {
        use revue::widget::Autocomplete;

        let view = Autocomplete::new()
            .placeholder("Search fruits...")
            .suggestions(["Apple", "Apricot", "Avocado"])
            .value("Ap");

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("autocomplete_with_value");
    }
}
