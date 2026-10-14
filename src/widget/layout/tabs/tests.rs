//! Tests of the private state

use super::*;
use crate::render::Buffer;
use crate::widget::traits::{RenderContext, View};

#[test]
fn label_spans_match_the_drawn_bar() {
    let labels = ["one", "日本", "x"];
    let t = Tabs::new().tabs(labels.to_vec());
    let mut buf = Buffer::new(30, 1);
    let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, 30, 1));
    t.render(&mut ctx);

    let spans = label_spans(labels);
    assert_eq!(spans, vec![(0, 5), (6, 12), (13, 16)]);
    for (i, &(start, end)) in spans.iter().enumerate() {
        assert_eq!(buf.get(start, 0).unwrap().symbol, ' ');
        assert_eq!(buf.get(end - 1, 0).unwrap().symbol, ' ');
        if i + 1 < spans.len() {
            assert_eq!(buf.get(end, 0).unwrap().symbol, '│');
        }
    }
}

#[test]
fn test_tabs_new_empty() {
    let t = Tabs::new();
    assert!(t.is_empty());
    assert_eq!(t.len(), 0);
    assert_eq!(t.selected_index(), 0);
    assert!(t.selected_label().is_none());
}

#[test]
fn test_tabs_add_tabs() {
    let t = Tabs::new().tab("One").tab("Two").tab("Three");
    assert_eq!(t.len(), 3);
    assert_eq!(t.selected_label(), Some("One"));
}

#[test]
fn test_tabs_selected() {
    let t = Tabs::new().tab("A").tab("B").tab("C").selected(2);
    assert_eq!(t.selected_index(), 2);
    assert_eq!(t.selected_label(), Some("C"));
}

#[test]
fn test_tabs_navigation() {
    let mut t = Tabs::new().tab("A").tab("B").tab("C");
    assert_eq!(t.selected_index(), 0);

    t.select_next();
    assert_eq!(t.selected_index(), 1);

    t.select_next();
    assert_eq!(t.selected_index(), 2);

    t.select_next(); // wraps
    assert_eq!(t.selected_index(), 0);

    t.select_prev(); // wraps back
    assert_eq!(t.selected_index(), 2);
}

#[test]
fn test_tabs_select_first_last() {
    let mut t = Tabs::new().tab("A").tab("B").tab("C").selected(1);
    t.select_first();
    assert_eq!(t.selected_index(), 0);

    t.select_last();
    assert_eq!(t.selected_index(), 2);
}

#[test]
fn test_tabs_handle_key() {
    use crate::event::Key;

    let mut t = Tabs::new().tab("A").tab("B").tab("C");

    assert!(t.handle_key(&Key::Right));
    assert_eq!(t.selected_index(), 1);

    assert!(t.handle_key(&Key::Left));
    assert_eq!(t.selected_index(), 0);

    assert!(t.handle_key(&Key::End));
    assert_eq!(t.selected_index(), 2);

    assert!(t.handle_key(&Key::Home));
    assert_eq!(t.selected_index(), 0);

    // Digit key selects tab (1-indexed)
    assert!(t.handle_key(&Key::Char('2')));
    assert_eq!(t.selected_index(), 1);

    // Unknown key returns false
    assert!(!t.handle_key(&Key::Char('x')));
}

#[test]
fn test_tabs_render_empty_no_panic() {
    let mut buf = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    let t = Tabs::new();
    t.render(&mut ctx);
}

#[test]
fn test_tabs_render_small_area_no_panic() {
    let mut buf = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buf, area);
    let t = Tabs::new().tab("Tab1").tab("Tab2");
    t.render(&mut ctx); // Width < 3, should return early
}

#[test]
fn test_tabs_helper_fn() {
    let t = tabs().tab("A");
    assert_eq!(t.len(), 1);
}

// Found by tests/widget/catalog/sequences/mod.rs: the shrunk sequence was `'0'`.
#[test]
fn test_tabs_zero_key_is_ignored() {
    let mut t = Tabs::new().tabs(vec!["One", "Two"]).selected(1);
    assert!(!t.handle_key(&crate::event::Key::Char('0')));
    assert_eq!(t.selected_index(), 1);
}
