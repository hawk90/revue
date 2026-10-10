//! Slider keyboard handling

use revue::event::Key;
use revue::widget::slider;

#[test]
fn test_slider_ignores_keys_when_not_focused() {
    let mut s = slider().value(50.0);
    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_ignores_keys_when_disabled() {
    let mut s = slider().value(50.0).focused(true).disabled(true);
    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_horizontal_arrow_and_vim_keys() {
    let mut s = slider().step(1.0).value(50.0).focused(true);
    assert!(s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 51.0);
    assert!(s.handle_key(&Key::Char('l')));
    assert_eq!(s.get_value(), 52.0);
    assert!(s.handle_key(&Key::Left));
    assert!(s.handle_key(&Key::Char('h')));
    assert_eq!(s.get_value(), 50.0);
    // Vertical keys do nothing on a horizontal slider
    assert!(!s.handle_key(&Key::Up));
    assert!(!s.handle_key(&Key::Char('k')));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_vertical_arrow_and_vim_keys() {
    let mut s = slider().vertical().step(1.0).value(50.0).focused(true);
    assert!(s.handle_key(&Key::Up));
    assert!(s.handle_key(&Key::Char('k')));
    assert_eq!(s.get_value(), 52.0);
    assert!(s.handle_key(&Key::Down));
    assert!(s.handle_key(&Key::Char('j')));
    assert_eq!(s.get_value(), 50.0);
    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_home_and_end() {
    let mut s = slider().value(50.0).focused(true);
    assert!(s.handle_key(&Key::End));
    assert_eq!(s.get_value(), 100.0);
    assert!(s.handle_key(&Key::Home));
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_unhandled_key() {
    let mut s = slider().value(50.0).focused(true);
    assert!(!s.handle_key(&Key::Enter));
    assert_eq!(s.get_value(), 50.0);
}
