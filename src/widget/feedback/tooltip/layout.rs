//! Wrapping the text, sizing the box, and placing it next to the anchor

use super::{Tooltip, TooltipArrow, TooltipPosition};
use crate::utils::unicode::display_width;

impl Tooltip {
    /// Word wrap text
    pub(super) fn wrap_text(&self) -> Vec<String> {
        let max_width = if self.max_width > 0 {
            self.max_width as usize
        } else {
            40
        };

        let mut lines = Vec::new();
        // Widths are terminal columns, the unit the box is sized in.
        for line in self.text.lines() {
            if display_width(line) <= max_width {
                lines.push(line.to_string());
            } else {
                // Simple word wrap
                let mut current_line = String::new();
                for word in line.split_whitespace() {
                    if current_line.is_empty() {
                        current_line = word.to_string();
                    } else if display_width(&current_line) + 1 + display_width(word) <= max_width {
                        current_line.push(' ');
                        current_line.push_str(word);
                    } else {
                        lines.push(current_line);
                        current_line = word.to_string();
                    }
                }
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
            }
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    /// Calculate tooltip dimensions
    pub(super) fn calculate_dimensions(&self) -> (u16, u16) {
        let lines = self.wrap_text();
        let has_border = self.style.border_chars().is_some();
        let has_title = self.title.is_some();

        let content_width = lines.iter().map(|l| display_width(l)).max().unwrap_or(0) as u16;
        let title_width = self
            .title
            .as_ref()
            .map(|t| display_width(t) as u16 + 2)
            .unwrap_or(0);
        let text_width = content_width.max(title_width);

        let width = text_width + if has_border { 4 } else { 2 }; // padding + border
        let height = lines.len() as u16
            + if has_border { 2 } else { 0 }
            + if has_title && has_border { 1 } else { 0 };

        (width, height)
    }

    /// The gap the arrow takes between the anchor and the box: one cell, or
    /// none without an arrow.
    fn arrow_offset(&self) -> u16 {
        if matches!(self.arrow, TooltipArrow::None) {
            0
        } else {
            1
        }
    }

    /// The side the box goes on: the set one, or for `Auto` the first of
    /// above, below, right and left with room for the box and the arrow
    /// (above when none has). Never `Auto`.
    fn resolve_position(
        &self,
        area_width: u16,
        area_height: u16,
        tooltip_w: u16,
        tooltip_h: u16,
    ) -> TooltipPosition {
        if self.position != TooltipPosition::Auto {
            return self.position;
        }
        let (anchor_x, anchor_y) = self.anchor;
        let arrow_offset = self.arrow_offset();
        // Auto-detect best position based on available space
        let space_above = anchor_y;
        let space_below = area_height.saturating_sub(anchor_y + 1);
        let space_left = anchor_x;
        let space_right = area_width.saturating_sub(anchor_x + 1);

        if space_above >= tooltip_h + arrow_offset {
            TooltipPosition::Top
        } else if space_below >= tooltip_h + arrow_offset {
            TooltipPosition::Bottom
        } else if space_right >= tooltip_w + arrow_offset {
            TooltipPosition::Right
        } else if space_left >= tooltip_w + arrow_offset {
            TooltipPosition::Left
        } else {
            TooltipPosition::Top // Default fallback
        }
    }

    /// The box's top-left corner on `position`'s side of the anchor, before
    /// it is kept inside the area: centered across the anchor, one arrow
    /// cell away.
    fn place(&self, position: TooltipPosition, tooltip_w: u16, tooltip_h: u16) -> (u16, u16) {
        let (anchor_x, anchor_y) = self.anchor;
        let arrow_offset = self.arrow_offset();
        match position {
            // `resolve_position` never returns Auto.
            TooltipPosition::Top | TooltipPosition::Auto => {
                let x = anchor_x.saturating_sub(tooltip_w / 2);
                let y = anchor_y.saturating_sub(tooltip_h + arrow_offset);
                (x, y)
            }
            TooltipPosition::Bottom => {
                let x = anchor_x.saturating_sub(tooltip_w / 2);
                let y = anchor_y + 1 + arrow_offset;
                (x, y)
            }
            TooltipPosition::Left => {
                let x = anchor_x.saturating_sub(tooltip_w + arrow_offset);
                let y = anchor_y.saturating_sub(tooltip_h / 2);
                (x, y)
            }
            TooltipPosition::Right => {
                let x = anchor_x + 1 + arrow_offset;
                let y = anchor_y.saturating_sub(tooltip_h / 2);
                (x, y)
            }
        }
    }

    /// Calculate position based on anchor and available space
    pub(super) fn calculate_position(
        &self,
        area_width: u16,
        area_height: u16,
    ) -> (u16, u16, TooltipPosition) {
        let (tooltip_w, tooltip_h) = self.calculate_dimensions();
        let position = self.resolve_position(area_width, area_height, tooltip_w, tooltip_h);
        let (x, y) = self.place(position, tooltip_w, tooltip_h);

        // Clamp to screen bounds
        let x = x.min(area_width.saturating_sub(tooltip_w));
        let y = y.min(area_height.saturating_sub(tooltip_h));

        (x, y, position)
    }
}
