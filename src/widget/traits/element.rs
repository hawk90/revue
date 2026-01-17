//! Element enum for rendered widgets

use super::view::View;

/// A rendered element
#[derive(Default)]
pub enum Element {
    /// Empty element
    #[default]
    Empty,
    /// View element
    View(Box<dyn View>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::traits::render_context::RenderContext;

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
}
