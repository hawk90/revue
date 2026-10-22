//! Checkbox placement, the painted width against `measure`, and toggling
//! by keys

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{Interactive, RenderContext, View};
use revue::widget::{Checkbox, CheckboxStyle};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const STYLES: [CheckboxStyle; 4] = [
    CheckboxStyle::Square,
    CheckboxStyle::Unicode,
    CheckboxStyle::Filled,
    CheckboxStyle::Circle,
];

#[test]
fn test_checkbox_zero_areas_and_offset() {
    for style in STYLES {
        let c = Checkbox::new("체크 box")
            .style(style)
            .checked(true)
            .focused(true);
        assert_zero_areas_draw_nothing(&c);
        assert_same_when_offset(c, 16, 1);
    }
}

/// Columns painted on row 0: up to the last non-blank cell (a wide glyph's
/// continuation cell counts, so no width table is involved, #1026)
fn painted_width(c: &Checkbox) -> u16 {
    let mut buffer = Buffer::new(60, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 60, 1));
    c.render(&mut ctx);
    (0..60u16)
        .filter_map(|x| {
            let ch = buffer.get(x, 0).unwrap().symbol;
            (ch != ' ').then_some(x + 1)
        })
        .max()
        .unwrap_or(0)
}

#[test]
fn test_checkbox_symbol_label_takes_one_cell_per_symbol() {
    // ★ is one column on the terminal: no continuation cell after it, and
    // the measure counts one (#1026)
    let c = Checkbox::new("★ok");
    let mut buffer = Buffer::new(20, 1);
    c.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 1)));
    let row: String = (0..8).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
    assert_eq!(row, "[ ] ★ok ");
    assert_eq!(c.measure(20, 1).unwrap().0, 7);
}

proptest! {
    /// `measure` gives the painted width for any style, label, state and
    /// focus.
    #[test]
    fn prop_checkbox_measure_is_the_painted_width(
        style in 0usize..4,
        // No trailing space: it paints nothing
        label in "([a-z가]([a-z가 ]{0,6}[a-z가])?)?",
        checked in any::<bool>(),
        focused in any::<bool>(),
        disabled in any::<bool>(),
    ) {
        let c = Checkbox::new(label).style(STYLES[style]).checked(checked).focused(focused).disabled(disabled);
        let (w, h) = c.measure(60, 5).unwrap();
        prop_assert_eq!(h, 1);
        prop_assert_eq!(painted_width(&c), w);
    }

    /// Enter and Space toggle an enabled checkbox through both entry
    /// points; other keys do not, and a disabled one takes no key.
    #[test]
    fn prop_checkbox_keys_toggle(
        disabled in any::<bool>(),
        via_event in any::<bool>(),
        keys in prop::collection::vec(
            prop_oneof![Just(Key::Enter), Just(Key::Char(' ')), Just(Key::Char('x')), Just(Key::Tab), Just(Key::Escape)],
            0..20,
        ),
    ) {
        let mut c = Checkbox::new("x").disabled(disabled);
        let mut on = false;
        for k in &keys {
            let toggles = !disabled && matches!(k, Key::Enter | Key::Char(' '));
            let used = if via_event {
                Interactive::handle_key(&mut c, &KeyEvent::new(*k)).is_consumed()
            } else {
                c.handle_key(k)
            };
            prop_assert_eq!(used, toggles, "{:?}", k);
            on ^= toggles;
            prop_assert_eq!(c.is_checked(), on);
        }
    }
}

fn cell_fg(
    c: &Checkbox,
    x: u16,
    style: Option<&revue::style::Style>,
) -> Option<revue::style::Color> {
    let mut buffer = Buffer::new(12, 1);
    let area = Rect::new(0, 0, 12, 1);
    let mut ctx = match style {
        Some(s) => RenderContext::with_style(&mut buffer, area, s),
        None => RenderContext::new(&mut buffer, area),
    };
    c.render(&mut ctx);
    buffer.get(x, 0).unwrap().fg
}

#[test]
fn test_checkbox_disabled_marks_are_two_grays() {
    // A checked disabled mark is lighter than an unchecked one, and
    // neither is the enabled colors
    let checked = cell_fg(&Checkbox::new("a").checked(true).disabled(true), 1, None);
    let unchecked = cell_fg(&Checkbox::new("a").disabled(true), 1, None);
    assert_ne!(checked, unchecked);
    assert_ne!(checked, cell_fg(&Checkbox::new("a").checked(true), 1, None));
    assert_ne!(unchecked, cell_fg(&Checkbox::new("a"), 1, None));
}

#[test]
fn test_checkbox_label_takes_the_stylesheet_color() {
    use revue::style::{Color, Style, VisualStyle};
    let style = Style {
        visual: VisualStyle {
            color: Color::MAGENTA,
            ..VisualStyle::default()
        },
        ..Default::default()
    };
    // The label and the brackets; the mark keeps its own color
    let c = Checkbox::new("a").checked(true);
    assert_eq!(cell_fg(&c, 4, Some(&style)), Some(Color::MAGENTA));
    assert_eq!(cell_fg(&c, 0, Some(&style)), Some(Color::MAGENTA));
    assert_eq!(cell_fg(&c, 1, Some(&style)), Some(Color::GREEN));
}
