//! Wrapping the text, sizing the box, and placing it next to the anchor

use super::{Tooltip, TooltipArrow, TooltipPosition};
use crate::layout::anchored::{self, Align, Placed, Side};
use crate::layout::Rect;
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

        // Padding and border
        let chrome = if has_border { 4 } else { 2 };
        // No wider than the area, but wide enough for the widest line (at
        // least one column) even where the area is not: a wide glyph alone
        // on its line needs 2 (the box is cut at the area's edge); a title
        // wider than the box is cut at its edge
        let width = (text_width + chrome).min(area_width.max(content_width.max(1) + chrome));
        let height = lines.len() as u16
            + if has_border { 2 } else { 0 }
            + if has_title && has_border { 1 } else { 0 };

        (width, height)
    }

    /// The gap the arrow takes between the anchor and the box: one cell, or
    /// none without an arrow.
    pub(super) fn arrow_offset(&self) -> u16 {
        if matches!(self.arrow, TooltipArrow::None) {
            0
        } else {
            1
        }
    }

    /// The anchor as a one-cell rect
    pub(super) fn anchor_rect(&self) -> Rect {
        Rect::new(self.anchor.0, self.anchor.1, 1, 1)
    }

    /// Where the box goes in an `area_width` x `area_height` area, in the
    /// area's coordinates, by the shared anchored placement rules
    /// ([`crate::layout::anchored`]): the set side, or the other side of the
    /// anchor when only that one has room, or the roomier of the two with
    /// the box shortened to it. `Auto` tries above, below, right and left.
    /// The box never covers the anchor, and the arrow's cell stays free.
    /// `None` when neither side has a row (or column) to spare.
    pub(super) fn calculate_position(&self, area_width: u16, area_height: u16) -> Option<Placed> {
        let sides: &[Side] = match self.position {
            TooltipPosition::Top => &[Side::Top, Side::Bottom],
            TooltipPosition::Bottom => &[Side::Bottom, Side::Top],
            TooltipPosition::Left => &[Side::Left, Side::Right],
            TooltipPosition::Right => &[Side::Right, Side::Left],
            TooltipPosition::Auto => &[Side::Top, Side::Bottom, Side::Right, Side::Left],
        };
        anchored::place(
            self.anchor_rect(),
            self.calculate_dimensions(area_width),
            sides,
            Align::Center,
            self.arrow_offset(),
            Rect::new(0, 0, area_width, area_height),
        )
    }
}

impl Tooltip {
    /// The side `side` names
    pub(super) fn position_of(side: Side) -> TooltipPosition {
        match side {
            Side::Top => TooltipPosition::Top,
            Side::Bottom => TooltipPosition::Bottom,
            Side::Left => TooltipPosition::Left,
            Side::Right => TooltipPosition::Right,
        }
    }
}
