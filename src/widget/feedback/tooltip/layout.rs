//! Wrapping the text, sizing the box, and placing it next to the anchor

use super::{Tooltip, TooltipArrow, TooltipPosition};
use crate::text::wrap_words;
use crate::utils::unicode::display_width;

impl Tooltip {
    /// Word wrap text
    pub(super) fn wrap_text(&self, area_width: u16) -> Vec<String> {
        let max_width = if self.max_width > 0 {
            self.max_width as usize
        } else {
            40
        };
        // No wider than the area leaves inside the padding and border, so
        // the box keeps its right edge in a narrow area
        let chrome = if self.style.border_chars().is_some() {
            4
        } else {
            2
        };
        let width = max_width
            .min((area_width as usize).saturating_sub(chrome))
            .max(1);
        // Widths are terminal columns, the unit the box is sized in; a word
        // wider than the line is broken by columns
        wrap_words(&self.text, width)
    }

    /// Calculate tooltip dimensions
    pub(super) fn calculate_dimensions(&self, area_width: u16) -> (u16, u16) {
        let lines = self.wrap_text(area_width);
        let has_border = self.style.border_chars().is_some();
        let has_title = self.title.is_some();

        let content_width = lines.iter().map(|l| display_width(l)).max().unwrap_or(0) as u16;
        let title_width = self
            .title
            .as_ref()
            .map(|t| display_width(t) as u16 + 2)
            .unwrap_or(0);
        let text_width = content_width.max(title_width);

        let chrome = if has_border { 4 } else { 2 }; // padding + border
                                                     // No wider than the area, but with a column for the text even in
                                                     // an area too narrow for one (the box is cut there); a title wider
                                                     // than the box is cut at its edge
        let width = (text_width + chrome).min(area_width.max(chrome + 1));
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
        let space_below = area_height.saturating_sub(anchor_y.saturating_add(1));
        let space_left = anchor_x;
        let space_right = area_width.saturating_sub(anchor_x.saturating_add(1));

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
                let y = anchor_y.saturating_add(1 + arrow_offset);
                (x, y)
            }
            TooltipPosition::Left => {
                let x = anchor_x.saturating_sub(tooltip_w + arrow_offset);
                let y = anchor_y.saturating_sub(tooltip_h / 2);
                (x, y)
            }
            TooltipPosition::Right => {
                let x = anchor_x.saturating_add(1 + arrow_offset);
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
        let (tooltip_w, tooltip_h) = self.calculate_dimensions(area_width);
        let position = self.resolve_position(area_width, area_height, tooltip_w, tooltip_h);
        let (x, y) = self.place(position, tooltip_w, tooltip_h);

        // Clamp to screen bounds
        let x = x.min(area_width.saturating_sub(tooltip_w));
        let y = y.min(area_height.saturating_sub(tooltip_h));

        (x, y, position)
    }
}
