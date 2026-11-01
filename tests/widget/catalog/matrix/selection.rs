//! Selected text looks the same in every field: the theme's colors (#1300).

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::use_theme;
use revue::widget::traits::{RenderContext, View};
use revue::widget::*;

/// Whether `view` draws a cell in the selection's colors, and only those.
fn draws_selection(name: &str, view: &dyn View) {
    let theme = use_theme().get();
    let (fg, bg) = (theme.colors.selection_text, theme.colors.selection);
    let mut buffer = Buffer::new(40, 3);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 3));
    view.render(&mut ctx);
    let found = (0..3)
        .flat_map(|y| (0..40).map(move |x| (x, y)))
        .filter_map(|(x, y)| buffer.get(x, y))
        .any(|c| c.fg == Some(fg) && c.bg == Some(bg) && c.symbol == 'a');
    assert!(found, "{name}");
}

#[test]
fn every_field_draws_a_selection_in_the_themes_colors() {
    let mut input = Input::new().value("ab");
    input.handle_key_event(&KeyEvent::new(Key::Home));
    input.select_all();
    draws_selection("Input", &input);

    let mut search = SearchBar::new();
    search.focus();
    search.handle_key_event(&KeyEvent::new(Key::Char('a')));
    search.handle_key_event(&KeyEvent::ctrl(Key::Char('a')));
    draws_selection("SearchBar", &search);

    let mut auto = Autocomplete::new().value("ab");
    auto.focus();
    auto.handle_key(KeyEvent::ctrl(Key::Char('a')));
    draws_selection("Autocomplete", &auto);

    let mut combo = Combobox::new().value("ab");
    combo.handle_key_event(&KeyEvent::ctrl(Key::Char('a')));
    draws_selection("Combobox", &combo);

    let mut masked = MaskedInput::new().mask_char('a').value("xy").focused(true);
    masked.handle_key_event(&KeyEvent::ctrl(Key::Char('a')));
    draws_selection("MaskedInput", &masked);
}

#[test]
fn a_builder_selection_color_still_wins_on_input() {
    use revue::style::Color;
    let mut input = Input::new().value("ab").selection_bg(Color::GREEN);
    input.select_all();
    let mut buffer = Buffer::new(10, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 1));
    input.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::GREEN));
}
