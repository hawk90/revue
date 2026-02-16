//! Render context for widget rendering

mod css;
mod focus;
mod progress;
mod segments;
mod shapes;
mod text;
mod types;

pub use types::ProgressBarConfig;

use crate::dom::NodeState;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Style;

/// Render context passed to widgets
pub struct RenderContext<'a> {
    /// Buffer to render into
    pub buffer: &'a mut Buffer,
    /// Available area for rendering
    pub area: Rect,
    /// Computed style from CSS cascade
    pub style: Option<&'a Style>,
    /// Current widget state
    pub state: Option<&'a NodeState>,
    /// Transition values for animations (property name -> current value)
    transitions: Option<&'a std::collections::HashMap<String, f32>>,
}

impl<'a> RenderContext<'a> {
    /// Create a basic render context (without style/state)
    pub fn new(buffer: &'a mut Buffer, area: Rect) -> Self {
        Self {
            buffer,
            area,
            style: None,
            state: None,
            transitions: None,
        }
    }

    /// Create a render context with style
    pub fn with_style(buffer: &'a mut Buffer, area: Rect, style: &'a Style) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: None,
            transitions: None,
        }
    }

    /// Create a full render context
    pub fn full(
        buffer: &'a mut Buffer,
        area: Rect,
        style: &'a Style,
        state: &'a NodeState,
    ) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: Some(state),
            transitions: None,
        }
    }

    /// Set transition values for this render context
    pub fn with_transitions(
        mut self,
        transitions: &'a std::collections::HashMap<String, f32>,
    ) -> Self {
        self.transitions = Some(transitions);
        self
    }

    /// Get current transition value for a property
    pub fn transition(&self, property: &str) -> Option<f32> {
        self.transitions.and_then(|t| t.get(property).copied())
    }

    /// Get transition value with a default fallback
    pub fn transition_or(&self, property: &str, default: f32) -> f32 {
        self.transition(property).unwrap_or(default)
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.state.map(|s| s.focused).unwrap_or(false)
    }

    /// Check if hovered
    pub fn is_hovered(&self) -> bool {
        self.state.map(|s| s.hovered).unwrap_or(false)
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.state.map(|s| s.disabled).unwrap_or(false)
    }
}
