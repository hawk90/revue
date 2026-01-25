//! Divider widget for visual separation

use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Orientation for the divider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    /// Horizontal divider (default)
    #[default]
    Horizontal,
    /// Vertical divider
    Vertical,
}

/// Divider style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerStyle {
    /// Solid line (default)
    #[default]
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// Double line
    Double,
    /// Thick line
    Thick,
}

/// A divider widget for visual separation
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// vstack()
///     .child(text("Section 1"))
///     .child(divider())
///     .child(text("Section 2"))
/// ```
pub struct Divider {
    /// Orientation
    orientation: Orientation,
    /// Style
    style: DividerStyle,
    /// Color
    color: Color,
    /// Label (centered in the divider)
    label: Option<String>,
    /// Label color
    label_color: Option<Color>,
    /// Margin (space before and after)
    margin: u16,
    /// Length (0 = auto/full width)
    length: u16,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl Divider {
    /// Create a new horizontal divider
    pub fn new() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            style: DividerStyle::Solid,
            color: Color::rgb(80, 80, 80),
            label: None,
            label_color: None,
            margin: 0,
            length: 0,
            props: WidgetProps::new(),
        }
    }

    /// Create a vertical divider
    pub fn vertical() -> Self {
        Self {
            orientation: Orientation::Vertical,
            ..Self::new()
        }
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set style
    pub fn style(mut self, style: DividerStyle) -> Self {
        self.style = style;
        self
    }

    /// Set color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set label (centered text)
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set label color
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = Some(color);
        self
    }

    /// Set margin (space before and after the line)
    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }

    /// Set length (0 = auto/full)
    pub fn length(mut self, length: u16) -> Self {
        self.length = length;
        self
    }

    /// Dashed style shorthand
    pub fn dashed(mut self) -> Self {
        self.style = DividerStyle::Dashed;
        self
    }

    /// Dotted style shorthand
    pub fn dotted(mut self) -> Self {
        self.style = DividerStyle::Dotted;
        self
    }

    /// Double line shorthand
    pub fn double(mut self) -> Self {
        self.style = DividerStyle::Double;
        self
    }

    /// Thick line shorthand
    pub fn thick(mut self) -> Self {
        self.style = DividerStyle::Thick;
        self
    }

    /// Get the line character based on style and orientation
    fn line_char(&self) -> char {
        match (self.orientation, self.style) {
            (Orientation::Horizontal, DividerStyle::Solid) => '─',
            (Orientation::Horizontal, DividerStyle::Dashed) => '╌',
            (Orientation::Horizontal, DividerStyle::Dotted) => '┄',
            (Orientation::Horizontal, DividerStyle::Double) => '═',
            (Orientation::Horizontal, DividerStyle::Thick) => '━',
            (Orientation::Vertical, DividerStyle::Solid) => '│',
            (Orientation::Vertical, DividerStyle::Dashed) => '╎',
            (Orientation::Vertical, DividerStyle::Dotted) => '┆',
            (Orientation::Vertical, DividerStyle::Double) => '║',
            (Orientation::Vertical, DividerStyle::Thick) => '┃',
        }
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Divider {
    crate::impl_view_meta!("Divider");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let line_char = self.line_char();

        match self.orientation {
            Orientation::Horizontal => {
                let y = area.y;
                let start_x = area.x + self.margin;
                let end_x = if self.length > 0 {
                    (start_x + self.length).min(area.x + area.width)
                } else {
                    area.x + area.width.saturating_sub(self.margin)
                };

                // Draw the line
                if let Some(ref label) = self.label {
                    // Line with label centered
                    let label_len = label.chars().count() as u16;
                    let total_width = end_x - start_x;

                    if label_len + 4 <= total_width {
                        let label_start = start_x + (total_width - label_len) / 2 - 1;
                        let label_end = label_start + label_len + 2;

                        // Left part
                        ctx.draw_hline(start_x, y, label_start - start_x, line_char, self.color);

                        // Space before label
                        ctx.draw_char(label_start, y, ' ', self.color);

                        // Label
                        let label_color = self.label_color.unwrap_or(self.color);
                        ctx.draw_text(label_start + 1, y, label, label_color);

                        // Space after label
                        ctx.draw_char(label_end - 1, y, ' ', self.color);

                        // Right part
                        ctx.draw_hline(label_end, y, end_x - label_end, line_char, self.color);
                    } else {
                        // Not enough space, just draw label (clipped)
                        let label_color = self.label_color.unwrap_or(self.color);
                        ctx.draw_text_clipped(start_x, y, label, label_color, end_x - start_x);
                    }
                } else {
                    // Simple line without label
                    ctx.draw_hline(start_x, y, end_x - start_x, line_char, self.color);
                }
            }
            Orientation::Vertical => {
                let x = area.x;
                let start_y = area.y + self.margin;
                let end_y = if self.length > 0 {
                    (start_y + self.length).min(area.y + area.height)
                } else {
                    area.y + area.height.saturating_sub(self.margin)
                };

                ctx.draw_vline(x, start_y, end_y - start_y, line_char, self.color);
            }
        }
    }
}

impl_styled_view!(Divider);
impl_props_builders!(Divider);

/// Create a new horizontal divider
pub fn divider() -> Divider {
    Divider::new()
}

/// Create a new vertical divider
pub fn vdivider() -> Divider {
    Divider::vertical()
}

// All tests moved to tests/widget/divider.rs
