//! Helper functions for DOM rendering

use crate::dom::NodeState;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Style;
use crate::widget::RenderContext;

/// Helper to create a styled render context from DOM node
pub fn styled_context<'a>(
    buffer: &'a mut Buffer,
    area: Rect,
    style: &'a Style,
    state: &'a NodeState,
) -> RenderContext<'a> {
    RenderContext::full(buffer, area, style, state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_styled_context_creation() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let style = Style::default();
        let state = NodeState::default();

        let ctx = styled_context(&mut buffer, area, &style, &state);

        assert_eq!(ctx.area, area);
    }

    #[test]
    fn test_styled_context_with_empty_area() {
        let mut buffer = Buffer::new(0, 0);
        let area = Rect::new(0, 0, 0, 0);
        let style = Style::default();
        let state = NodeState::default();

        let ctx = styled_context(&mut buffer, area, &style, &state);

        assert_eq!(ctx.area, area);
    }

    #[test]
    fn test_styled_context_with_custom_area() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(5, 5, 15, 10);
        let style = Style::default();
        let state = NodeState::default();

        let ctx = styled_context(&mut buffer, area, &style, &state);

        assert_eq!(ctx.area, area);
    }

    #[test]
    fn test_styled_context_with_focused_state() {
        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let style = Style::default();
        let mut state = NodeState::default();
        state.focused = true;

        let ctx = styled_context(&mut buffer, area, &style, &state);

        assert_eq!(ctx.area, area);
    }

    #[test]
    fn test_styled_context_with_large_area() {
        let mut buffer = Buffer::new(100, 100);
        let area = Rect::new(0, 0, 100, 100);
        let style = Style::default();
        let state = NodeState::default();

        let ctx = styled_context(&mut buffer, area, &style, &state);

        assert_eq!(ctx.area, area);
    }

    #[test]
    fn test_styled_context_with_offset_area() {
        let mut buffer = Buffer::new(50, 50);
        let area = Rect::new(10, 20, 30, 25);
        let style = Style::default();
        let state = NodeState::default();

        let ctx = styled_context(&mut buffer, area, &style, &state);

        assert_eq!(ctx.area, area);
    }
}
