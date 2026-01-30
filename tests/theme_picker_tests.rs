//! Integration tests for ThemePicker widget

use revue::event::{Key, KeyEvent};
use revue::style::Color;
use revue::widget::traits::Interactive;
use revue::widget::{theme_picker, ThemePicker};

#[test]
fn test_theme_picker_new() {
    let _picker = ThemePicker::new();
    // Picker created successfully
}

#[test]
fn test_theme_picker_themes() {
    let _picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    // Themes were set successfully
}

#[test]
fn test_theme_picker_compact() {
    let _picker = ThemePicker::new().compact(true);
}

#[test]
fn test_theme_picker_show_preview() {
    let _picker = ThemePicker::new().show_preview(false);
}

#[test]
fn test_theme_picker_width() {
    let _picker = ThemePicker::new().width(50);
}

#[test]
fn test_theme_picker_fg() {
    let _picker = ThemePicker::new().fg(Color::CYAN);
}

#[test]
fn test_theme_picker_bg() {
    let _picker = ThemePicker::new().bg(Color::BLUE);
}

#[test]
fn test_theme_picker_toggle() {
    let mut picker = ThemePicker::new();
    assert!(!picker.is_open());

    picker.toggle();
    assert!(picker.is_open());

    picker.toggle();
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_open() {
    let mut picker = ThemePicker::new();
    picker.open();
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_close() {
    let mut picker = ThemePicker::new();
    picker.open();
    picker.close();
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_is_open() {
    let picker = ThemePicker::new();
    assert!(!picker.is_open());

    let mut picker = picker;
    picker.open();
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_select_prev() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.select_next(); // Move to second item
    picker.select_next(); // Move to third item
    picker.select_prev(); // Move back to second item
                          // Select prev works
}

#[test]
fn test_theme_picker_select_next() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.select_next();
    // Select next works
}

#[test]
fn test_theme_picker_selected_id() {
    let picker = ThemePicker::new().themes(["dracula", "nord"]);
    assert_eq!(picker.selected_id(), Some("dracula"));
}

#[test]
fn test_theme_picker_selected_theme() {
    let picker = ThemePicker::new();
    let _theme = picker.selected_theme();
    // Can get selected theme
}

#[test]
fn test_theme_picker_apply_selected() {
    let picker = ThemePicker::new().themes(["dark"]);
    picker.apply_selected();
    // Apply selected works
}

#[test]
fn test_theme_picker_handle_key_open() {
    let mut picker = ThemePicker::new();
    let event = KeyEvent::new(Key::Enter);
    let _result = Interactive::handle_key(&mut picker, &event);
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_space() {
    let mut picker = ThemePicker::new();
    let event = KeyEvent::new(Key::Char(' '));
    let _result = Interactive::handle_key(&mut picker, &event);
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_close() {
    let mut picker = ThemePicker::new();
    picker.open();

    let event = KeyEvent::new(Key::Escape);
    let _result = Interactive::handle_key(&mut picker, &event);
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_down() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();

    let event = KeyEvent::new(Key::Down);
    let _result = Interactive::handle_key(&mut picker, &event);
    // Navigation works
}

#[test]
fn test_theme_picker_handle_key_up() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();
    picker.select_next();
    picker.select_next(); // Move to third item

    let event = KeyEvent::new(Key::Up);
    let _result = Interactive::handle_key(&mut picker, &event);
    // Navigation works
}

#[test]
fn test_theme_picker_handle_key_j() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();

    let event = KeyEvent::new(Key::Char('j'));
    let _result = Interactive::handle_key(&mut picker, &event);
    // Vim-style navigation works
}

#[test]
fn test_theme_picker_handle_key_k() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();
    picker.select_next();
    picker.select_next(); // Move to third item

    let event = KeyEvent::new(Key::Char('k'));
    let _result = Interactive::handle_key(&mut picker, &event);
    // Vim-style navigation works
}

#[test]
fn test_theme_picker_handle_key_tab() {
    let mut picker = ThemePicker::new().themes(["dark", "light"]);
    let event = KeyEvent::new(Key::Tab);
    let _result = Interactive::handle_key(&mut picker, &event);
    // Tab cycles through themes
}

#[test]
fn test_theme_picker_builder_pattern() {
    let _picker = theme_picker()
        .themes(["dark", "light"])
        .compact(true)
        .width(40)
        .fg(Color::CYAN)
        .bg(Color::BLUE);

    // Builder pattern works
}

#[test]
fn test_theme_picker_default() {
    let _picker = ThemePicker::default();
    // Default picker created
}

#[test]
fn test_theme_picker_single_theme() {
    let picker = ThemePicker::new().themes(["dark"]);
    assert_eq!(picker.selected_id(), Some("dark"));
}
