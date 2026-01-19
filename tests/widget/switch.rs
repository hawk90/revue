//! Switch widget tests

use revue::event::Key;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{toggle, Switch, SwitchStyle, View};

#[test]
fn test_switch_new() {
    let s = Switch::new();
    assert!(!s.is_on());
}

#[test]
fn test_switch_on() {
    let s = Switch::new().on(true);
    assert!(s.is_on());
}

#[test]
fn test_switch_toggle() {
    let mut s = Switch::new();
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());

    s.toggle();
    assert!(!s.is_on());
}

#[test]
fn test_switch_disabled() {
    let mut s = Switch::new().disabled(true);
    assert!(!s.is_on());

    s.toggle();
    assert!(!s.is_on()); // Should not change when disabled
}

#[test]
fn test_switch_render_default() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(true);
    s.render(&mut ctx);
}

#[test]
fn test_switch_render_all_styles() {
    let styles = [
        SwitchStyle::Default,
        SwitchStyle::IOS,
        SwitchStyle::Material,
        SwitchStyle::Text,
        SwitchStyle::Emoji,
        SwitchStyle::Block,
    ];

    for style in styles {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Switch::new().style(style);
        s.render(&mut ctx);
    }
}

#[test]
fn test_switch_with_label() {
    use revue::widget::switch;
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = switch().label("Dark Mode");
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'D');
}

#[test]
fn test_switch_handle_key() {
    use revue::widget::switch;

    let mut s = switch().focused(true);
    assert!(!s.is_on());

    assert!(s.handle_key(&Key::Enter));
    assert!(s.is_on());

    assert!(s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_helper() {
    use revue::widget::switch;
    let s = switch().on(true);
    assert!(s.is_on());
}

#[test]
fn test_switch_checked_alias() {
    // Test checked() is an alias for on()
    let s = Switch::new().checked(true);
    assert!(s.is_on());
    assert!(s.is_checked());

    let s = Switch::new().checked(false);
    assert!(!s.is_on());
    assert!(!s.is_checked());
}

// New tests from main branch
#[test]
fn test_toggle_helper() {
    let s = toggle("Enable");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Enable"));
}

// =============================================================================
