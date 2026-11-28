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
    ///
    /// A horizontal divider draws it on its row, a space on each side. A
    /// vertical one draws it down its column, one grapheme per row, with a
    /// blank row above and below; a label with a wide (two-column) glyph makes
    /// the vertical divider two columns wide. When the label and its two line
    /// cells on each side do not fit, only the label is drawn, cut to the
    /// length.
    ///
    /// An empty label is no label: the divider draws its plain line in
    /// either orientation. A vertical divider offered one column is cut to
    /// it like any widget, so a row holding a wide glyph stays empty there.
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

    /// The label to draw: none for an empty one, as Modal skips an empty
    /// title and Form an empty label
    fn shown_label(&self) -> Option<&str> {
        self.label.as_deref().filter(|label| !label.is_empty())
    }

    /// The label's rows in a vertical divider: one grapheme each, leaving out
    /// the graphemes that take no column
    fn vertical_label_rows(&self) -> Vec<&str> {
        use unicode_segmentation::UnicodeSegmentation;
        self.shown_label()
            .map(|label| {
                label
                    .graphemes(true)
                    .filter(|g| crate::utils::unicode::display_width(g) > 0)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Columns a vertical divider takes: one, or two when a label row holds
    /// a wide glyph
    fn vertical_width(&self) -> u16 {
        self.vertical_label_rows()
            .iter()
            .map(|g| crate::utils::unicode::display_width_u16(g).min(2))
            .max()
            .unwrap_or(1)
            .max(1)
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
    /// vertical one is one column (two when its label holds a wide glyph) and
    /// runs the height. With a
    /// [`length`](Divider::length) set, it runs the margin plus that length.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let fixed = (self.length > 0).then(|| self.margin.saturating_add(self.length));
        Some(match self.orientation {
            Orientation::Horizontal => {
                (fixed.unwrap_or(max_width).min(max_width), 1.min(max_height))
            }
            Orientation::Vertical => (
                self.vertical_width().min(max_width),
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
                if let Some(label) = self.shown_label() {
                    // Line with label centered
                    let label_len = crate::utils::unicode::display_width_u16(label);
                    let total_width = end_x - start_x;

                    if label_len.saturating_add(4) <= total_width {
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

                if self.shown_label().is_none() {
                    ctx.draw_vline(0, start_y, end_y - start_y, line_char, color);
                    return;
                }

                // The vertical form of the horizontal label: one grapheme per
                // row, a blank row on each side, the line in the first column.
                let rows = self.vertical_label_rows();
                let label_width = self.vertical_width();
                let label_color = self.label_color.unwrap_or(color);
                let label_len = u16::try_from(rows.len()).unwrap_or(u16::MAX);
                let total = end_y - start_y;

                if label_len.saturating_add(4) <= total {
                    let label_start = start_y + (total - label_len) / 2 - 1;
                    let label_end = label_start + label_len + 2;

                    ctx.draw_vline(0, start_y, label_start - start_y, line_char, color);
                    ctx.draw_char(0, label_start, ' ', color);
                    for (i, g) in rows.iter().enumerate() {
                        let y = label_start + 1 + i as u16;
                        ctx.draw_text_clipped(0, y, g, label_color, label_width);
                    }
                    ctx.draw_char(0, label_end - 1, ' ', color);
                    ctx.draw_vline(0, label_end, end_y - label_end, line_char, color);
                } else {
                    // Not enough room: only the label, cut to the length
                    for (i, g) in rows.iter().take(usize::from(total)).enumerate() {
                        let y = start_y + i as u16;
                        ctx.draw_text_clipped(0, y, g, label_color, label_width);
                    }
                }
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
