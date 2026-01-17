//! Tabs widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{tabs, Tabs, View};

#[test]
fn test_tabs_new() {
    let t = Tabs::new();
    assert!(t.is_empty());
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_builder() {
    let t = Tabs::new().tab("Home").tab("Settings").tab("Help");

    assert_eq!(t.len(), 3);
    assert_eq!(t.selected_label(), Some("Home"));
}

#[test]
fn test_tabs_from_vec() {
    let t = Tabs::new().tabs(vec!["A", "B", "C"]);

    assert_eq!(t.len(), 3);
}

#[test]
fn test_tabs_navigation() {
    let mut t = Tabs::new().tabs(vec!["One", "Two", "Three"]);

    assert_eq!(t.selected_index(), 0);

    t.select_next();
    assert_eq!(t.selected_index(), 1);

    t.select_next();
    assert_eq!(t.selected_index(), 2);

    t.select_next(); // Wraps around
    assert_eq!(t.selected_index(), 0);

    t.select_prev(); // Wraps around backward
    assert_eq!(t.selected_index(), 2);

    t.select_first();
    assert_eq!(t.selected_index(), 0);

    t.select_last();
    assert_eq!(t.selected_index(), 2);

    t.select(1);
    assert_eq!(t.selected_index(), 1);
}

#[test]
fn test_tabs_handle_key() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

    let changed = t.handle_key(&Key::Right);
    assert!(changed);
    assert_eq!(t.selected_index(), 1);

    let changed = t.handle_key(&Key::Left);
    assert!(changed);
    assert_eq!(t.selected_index(), 0);

    // Number keys (1-indexed)
    t.handle_key(&Key::Char('3'));
    assert_eq!(t.selected_index(), 2);

    t.handle_key(&Key::Char('1'));
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_render() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Files").tab("Edit");

    t.render(&mut ctx);

    // Check first tab label
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'F');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'i');
}

#[test]
fn test_tabs_selected_label() {
    let t = Tabs::new().tabs(vec!["Alpha", "Beta"]);

    assert_eq!(t.selected_label(), Some("Alpha"));
}

#[test]
fn test_tabs_helper() {
    let t = tabs().tab("Test");

    assert_eq!(t.len(), 1);
}

#[test]
fn test_tabs_default() {
    let t = Tabs::default();
    assert!(t.is_empty());
}

#[test]
fn test_tabs_handle_key_h_l() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

    // l for right
    t.handle_key(&Key::Char('l'));
    assert_eq!(t.selected_index(), 1);

    // h for left
    t.handle_key(&Key::Char('h'));
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_handle_key_home_end() {
    let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

    t.handle_key(&Key::End);
    assert_eq!(t.selected_index(), 2);

    t.handle_key(&Key::Home);
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_handle_key_number_out_of_range() {
    let mut t = Tabs::new().tabs(vec!["A", "B"]);

    // Pressing '9' when there are only 2 tabs should do nothing
    let changed = t.handle_key(&Key::Char('9'));
    assert!(!changed);
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_tabs_handle_key_unhandled() {
    let mut t = Tabs::new().tabs(vec!["A", "B"]);

    let changed = t.handle_key(&Key::Escape);
    assert!(!changed);
}

#[test]
fn test_tabs_selected_label_empty() {
    let t = Tabs::new();
    assert!(t.selected_label().is_none());
}

#[test]
fn test_tabs_render_empty() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new();
    t.render(&mut ctx);
    // Empty tabs should not panic
}

#[test]
fn test_tabs_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tabs::new().tab("Test");
    t.render(&mut ctx);
    // Small area should not panic
}

// =============================================================================
