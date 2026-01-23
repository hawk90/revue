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
