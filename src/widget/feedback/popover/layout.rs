use super::types::{PopoverArrow, PopoverPosition, PopoverStyle};
use super::Popover;
use crate::layout::anchored::{self, Align, Side};
use crate::layout::Rect;
use crate::text::wrap_words;
use crate::utils::unicode::display_width;

impl Popover {
    /// Word wrap content
    pub(super) fn wrap_content(&self, area_width: u16) -> Vec<String> {
        let max_width = if self.max_width > 0 {
            self.max_width as usize
        } else {
            40
        };
        // No wider than the area leaves inside the padding and border, so
        // the box keeps its right edge in a narrow area
        let chrome = if self.popover_style.border_chars().is_some() {
            4
        } else {
            2
        };
        let width = max_width
            .min((area_width as usize).saturating_sub(chrome))
            .max(1);
        // Widths are terminal columns, the unit the box is sized in; a word
        // wider than the line is broken by columns
        wrap_words(&self.content, width)
    }

    /// Calculate popover dimensions
    pub(super) fn calculate_dimensions(&self, area_width: u16) -> (u16, u16) {
        let lines = self.wrap_content(area_width);
        let has_border = self.popover_style.border_chars().is_some();
        let has_title = self.title.is_some();

        // Terminal columns, not bytes: a wide glyph takes two cells.
        let content_width = lines.iter().map(|l| display_width(l)).max().unwrap_or(0) as u16;
        let title_width = self
            .title
            .as_ref()
            .map(|t| display_width(t) as u16 + 2)
            .unwrap_or(0);
        let text_width = content_width.max(title_width);

        let chrome = if has_border { 4 } else { 2 };
        // No wider than the area, but wide enough for the widest line (at
        // least one column) even where the area is not: a wide glyph alone
        // on its line needs 2 (the box is cut at the area's edge); a title
        // wider than the box is cut at its edge
        let width = (text_width + chrome)
            .max(10)
            .min(area_width.max(content_width.max(1) + chrome));
        // The title takes a row above the content in every style (#1046)
        let height =
            lines.len() as u16 + if has_border { 2 } else { 0 } + if has_title { 1 } else { 0 };

        (width, height.max(3))
    }

    /// Where the box goes in an `area_width` x `area_height` area, and the
    /// side it took, by the shared anchored placement rules
    /// ([`crate::layout::anchored`]): the set side, or the other side of the
    /// anchor when only that one has room, or the roomier of the two with
    /// the box shortened to it. `Auto` tries below, above, right and left.
    /// The box (and an elevated box's shadow) never covers the anchor, and
    /// the arrow's cell stays free.
    /// `None` when neither side has a row (or column) to spare.
    pub(super) fn calculate_position(
        &self,
        area_width: u16,
        area_height: u16,
    ) -> Option<(Rect, PopoverPosition)> {
        let size = self.calculate_dimensions(area_width);
        let gap = if matches!(self.arrow, PopoverArrow::None) {
            0
        } else {
            1
        };
        let sides: &[Side] = match self.position {
            PopoverPosition::Top => &[Side::Top, Side::Bottom],
            PopoverPosition::Bottom => &[Side::Bottom, Side::Top],
            PopoverPosition::Left => &[Side::Left, Side::Right],
            PopoverPosition::Right => &[Side::Right, Side::Left],
            PopoverPosition::Auto => &[Side::Bottom, Side::Top, Side::Right, Side::Left],
        };
        // An elevated box is placed with its shadow (one column right, one
        // row below), so the shadow does not cover the anchor either
        let shadow = u16::from(matches!(self.popover_style, PopoverStyle::Elevated));
        let placed = anchored::place(
            self.anchor_rect(),
            (size.0 + shadow, size.1 + shadow),
            sides,
            Align::Center,
            gap,
            Rect::new(0, 0, area_width, area_height),
        )?;
        let rect = placed.rect;
        if rect.width <= shadow || rect.height <= shadow {
            return None;
        }
        let side = match placed.side {
            Side::Top => PopoverPosition::Top,
            Side::Bottom => PopoverPosition::Bottom,
            Side::Left => PopoverPosition::Left,
            Side::Right => PopoverPosition::Right,
        };
        let rect = Rect::new(rect.x, rect.y, rect.width - shadow, rect.height - shadow);
        Some((rect, side))
    }

    /// The anchor as a one-cell rect
    pub(super) fn anchor_rect(&self) -> Rect {
        Rect::new(self.anchor.0, self.anchor.1, 1, 1)
    }
}
