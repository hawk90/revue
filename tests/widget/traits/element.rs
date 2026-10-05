//! Tests for Element

use revue::widget::traits::{Element, RenderContext, View};

struct DummyView;

impl View for DummyView {
    fn render(&self, _ctx: &mut RenderContext) {}

    fn id(&self) -> Option<&str> {
        Some("dummy")
    }
}

#[test]
fn test_element_default_is_empty() {
    assert!(matches!(Element::default(), Element::Empty));
}

#[test]
fn test_element_view_keeps_the_view() {
    let element = Element::View(Box::new(DummyView));
    match element {
        Element::View(view) => assert_eq!(view.id(), Some("dummy")),
        Element::Empty => panic!("expected Element::View"),
    }
}
