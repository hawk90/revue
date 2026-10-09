//! Divider widget for visual separation

use crate::style::Color;
use crate::widget::theme::DARK_GRAY;
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
#[derive(Clone)]
pub struct Divider {
    /// Orientation
    orientation: Orientation,
    /// Style
    style: DividerStyle,
    /// Color
    /// The color the builder named, if it named one.
    ///
    /// `None` is not the same as `Some(DARK_GRAY)`. While this was a plain
    /// `Color`, naming the default explicitly was indistinguishable from
    /// saying nothing, so a stylesheet outranked a builder that had spoken.
    color: Option<Color>,
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
            color: None,
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
        self.color = Some(color);
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

    /// A horizontal divider is one row and runs the width it is offered; a
    /// vertical one is one column and runs the height. With a
    /// [`length`](Divider::length) set, it runs the margin plus that length.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let fixed = (self.length > 0).then(|| self.margin.saturating_add(self.length));
        Some(match self.orientation {
            Orientation::Horizontal => {
                (fixed.unwrap_or(max_width).min(max_width), 1.min(max_height))
            }
            Orientation::Vertical => (
                1.min(max_width),
                fixed.unwrap_or(max_height).min(max_height),
            ),
        })
    }

    /// It runs along its orientation as far as it is offered - unless a
    /// [`length`](Divider::length) fixes it.
    fn fills(&self) -> crate::widget::Fill {
        use crate::widget::Fill;
        match self.orientation {
            _ if self.length > 0 => Fill::NONE,
            Orientation::Horizontal => Fill::WIDTH,
            Orientation::Vertical => Fill::HEIGHT,
        }
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let line_char = self.line_char();
        // The builder can only outrank the stylesheet by moving off the initial
        // value - see [`RenderContext::color_or`].
        let color = self.color.unwrap_or_else(|| ctx.css_color(DARK_GRAY));

        match self.orientation {
            Orientation::Horizontal => {
                let start_x = self.margin;
                let end_x = if self.length > 0 {
                    start_x.saturating_add(self.length).min(area.width)
                } else {
                    area.width.saturating_sub(self.margin)
                };
                // Margins that meet or pass each other leave no room for a line.
                if end_x <= start_x {
                    return;
                }

                // Draw the line
                if let Some(ref label) = self.label {
                    // Line with label centered
                    let label_len = crate::utils::unicode::display_width(label) as u16;
                    let total_width = end_x - start_x;

                    if label_len + 4 <= total_width {
                        let label_start = start_x + (total_width - label_len) / 2 - 1;
                        let label_end = label_start + label_len + 2;

                        // Left part
                        ctx.draw_hline(start_x, 0, label_start - start_x, line_char, color);

                        // Space before label
                        ctx.draw_char(label_start, 0, ' ', color);

                        // Label
                        let label_color = self.label_color.unwrap_or(color);
                        ctx.draw_text(label_start + 1, 0, label, label_color);

                        // Space after label
                        ctx.draw_char(label_end - 1, 0, ' ', color);

                        // Right part
                        ctx.draw_hline(label_end, 0, end_x - label_end, line_char, color);
                    } else {
                        // Not enough space, just draw label (clipped)
                        let label_color = self.label_color.unwrap_or(color);
                        ctx.draw_text_clipped(start_x, 0, label, label_color, end_x - start_x);
                    }
                } else {
                    // Simple line without label
                    ctx.draw_hline(start_x, 0, end_x - start_x, line_char, color);
                }
            }
            Orientation::Vertical => {
                let start_y = self.margin;
                let end_y = if self.length > 0 {
                    start_y.saturating_add(self.length).min(area.height)
                } else {
                    area.height.saturating_sub(self.margin)
                };
                if end_y <= start_y {
                    return;
                }

                ctx.draw_vline(0, start_y, end_y - start_y, line_char, color);
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
