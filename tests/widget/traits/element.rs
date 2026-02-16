//! Tests for Element trait
//!
//! Extracted from src/widget/traits/element.rs

use revue::widget::traits::element::Element;
use revue::widget::traits::render_context::RenderContext;
use revue::widget::traits::View;

struct DummyView;

impl View for DummyView {
    fn render(&self, _ctx: &mut RenderContext) {}
}

#[test]
fn test_element_default() {
    let element = Element::default();
    assert!(matches!(element, Element::Empty));
}

#[test]
fn test_element_empty_variant() {
    let element = Element::Empty;
    assert!(matches!(element, Element::Empty));
}

#[test]
fn test_element_view_variant() {
    let view: Box<dyn View> = Box::new(DummyView);
    let element = Element::View(view);
    assert!(matches!(element, Element::View(_)));
}
