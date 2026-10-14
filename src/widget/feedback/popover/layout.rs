use super::types::{PopoverArrow, PopoverPosition};
use super::Popover;
use crate::utils::unicode::display_width;

impl Popover {
    /// Word wrap content
    pub(super) fn wrap_content(&self) -> Vec<String> {
        let max_width = if self.max_width > 0 {
            self.max_width as usize
        } else {
            40
        };

        let mut lines = Vec::new();
        // Widths are terminal columns, the unit the box is sized in.
        for line in self.content.lines() {
            if display_width(line) <= max_width {
                lines.push(line.to_string());
            } else {
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

    /// Calculate popover dimensions
    pub(super) fn calculate_dimensions(&self) -> (u16, u16) {
        let lines = self.wrap_content();
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

        let width = text_width + if has_border { 4 } else { 2 };
        let height = lines.len() as u16
            + if has_border { 2 } else { 0 }
            + if has_title && has_border { 1 } else { 0 };

        (width.max(10), height.max(3))
    }

    /// Calculate position based on anchor and available space
    pub(super) fn calculate_position(
        &self,
        area_width: u16,
        area_height: u16,
    ) -> (u16, u16, PopoverPosition) {
        let (popup_w, popup_h) = self.calculate_dimensions();
        let (anchor_x, anchor_y) = self.anchor;
        let arrow_offset: u16 = if matches!(self.arrow, PopoverArrow::None) {
            0
        } else {
            1
        };

        let position = match self.position {
            PopoverPosition::Auto => {
                // Auto-detect best position based on available space
                let space_above = anchor_y;
                let space_below = area_height.saturating_sub(anchor_y.saturating_add(1));
                let space_left = anchor_x;
                let space_right = area_width.saturating_sub(anchor_x.saturating_add(1));

                if space_below >= popup_h + arrow_offset {
                    PopoverPosition::Bottom
                } else if space_above >= popup_h + arrow_offset {
                    PopoverPosition::Top
                } else if space_right >= popup_w + arrow_offset {
                    PopoverPosition::Right
                } else if space_left >= popup_w + arrow_offset {
                    PopoverPosition::Left
                } else {
                    PopoverPosition::Bottom
                }
            }
            side => side,
        };
        let (x, y) = Self::place(position, self.anchor, (popup_w, popup_h), arrow_offset);

        let x = x.min(area_width.saturating_sub(popup_w));
        let y = y.min(area_height.saturating_sub(popup_h));

        (x, y, position)
    }

    /// The top-left corner of a `size` box on `side` of `anchor`, before
    /// clamping into the area
    fn place(
        side: PopoverPosition,
        (anchor_x, anchor_y): (u16, u16),
        (popup_w, popup_h): (u16, u16),
        arrow_offset: u16,
    ) -> (u16, u16) {
        match side {
            PopoverPosition::Top => {
                let x = anchor_x.saturating_sub(popup_w / 2);
                let y = anchor_y.saturating_sub(popup_h + arrow_offset);
                (x, y)
            }
            PopoverPosition::Bottom => {
                let x = anchor_x.saturating_sub(popup_w / 2);
                let y = anchor_y.saturating_add(1 + arrow_offset);
                (x, y)
            }
            PopoverPosition::Left => {
                let x = anchor_x.saturating_sub(popup_w + arrow_offset);
                let y = anchor_y.saturating_sub(popup_h / 2);
                (x, y)
            }
            PopoverPosition::Right => {
                let x = anchor_x.saturating_add(1 + arrow_offset);
                let y = anchor_y.saturating_sub(popup_h / 2);
                (x, y)
            }
            // calculate_position resolves Auto to one of the four sides first
            PopoverPosition::Auto => {
                unreachable!("Auto position resolved to concrete position above")
            }
        }
    }
}
