//! RadioGroup widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{RadioGroup, RadioLayout, View};

#[test]
fn test_radio_group_selection() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);

    assert_eq!(rg.selected_index(), 0);
    assert_eq!(rg.selected_value(), Some("A"));

    rg.select_next();
    assert_eq!(rg.selected_index(), 1);
    assert_eq!(rg.selected_value(), Some("B"));

    rg.select_next();
    assert_eq!(rg.selected_index(), 2);

    rg.select_next();
    assert_eq!(rg.selected_index(), 0); // Wraps around

    rg.select_prev();
    assert_eq!(rg.selected_index(), 2); // Wraps around
}

#[test]
fn test_radio_group_disabled_selection() {
    let mut rg = RadioGroup::new(vec!["A", "B"]).disabled(true);

    rg.select_next();
    assert_eq!(rg.selected_index(), 0); // Should not change
}

#[test]
fn test_radio_group_handle_key() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]);

    assert!(rg.handle_key(&Key::Down));
    assert_eq!(rg.selected_index(), 1);

    assert!(rg.handle_key(&Key::Up));
    assert_eq!(rg.selected_index(), 0);

    assert!(rg.handle_key(&Key::Char('j')));
    assert_eq!(rg.selected_index(), 1);

    assert!(rg.handle_key(&Key::Char('k')));
    assert_eq!(rg.selected_index(), 0);

    // Number keys
    assert!(rg.handle_key(&Key::Char('2')));
    assert_eq!(rg.selected_index(), 1);

    assert!(!rg.handle_key(&Key::Char('a'))); // Invalid key
}

#[test]
fn test_radio_group_horizontal_keys() {
    let mut rg = RadioGroup::new(vec!["A", "B", "C"]).layout(RadioLayout::Horizontal);

    assert!(rg.handle_key(&Key::Right));
    assert_eq!(rg.selected_index(), 1);

    assert!(rg.handle_key(&Key::Left));
    assert_eq!(rg.selected_index(), 0);
}

#[test]
fn test_radio_group_render() {
    let rg = RadioGroup::new(vec!["Option A", "Option B"]).selected(0);
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(1, 1, 25, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    rg.render(&mut ctx);
    // Smoke test - should render without panic
}

#[test]
fn test_radio_group_empty() {
    let rg = RadioGroup::new(Vec::<String>::new());
    assert_eq!(rg.selected_value(), None);
}

// =============================================================================
