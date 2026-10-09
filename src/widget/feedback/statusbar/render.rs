//! Drawing the bar: background, left/center/right sections and key hints

use super::{StatusBar, StatusSection};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl View for StatusBar {
    crate::impl_view_meta!("StatusBar");

    /// Its [`height`](StatusBar::height) in rows, across the full width.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, self.height.min(max_height)))
    }

    /// It stretches across the width it is offered.
    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::WIDTH
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let y = self.render_y(area.height);

        if y >= area.height {
            return;
        }

        // Fill background
        for row in 0..self.height {
            if y + row >= area.height {
                break;
            }
            for x in 0..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.bg);
                ctx.set(x, y + row, cell);
            }
        }

        // Leave out the least important sections that do not fit
        let [left, center, right] = self.visible_sections(area.width);

        // Calculate section widths
        let left_width = group_width(&left);
        let center_width = group_width(&center);
        let right_width = group_width(&right);

        // Render left sections
        let x = self.render_group(ctx, &left, 0, y);

        // Render center sections
        let center_start = (area.width.saturating_sub(center_width)) / 2;
        self.render_group(ctx, &center, center_start.max(x + 1), y);

        // Render right sections
        self.render_group(ctx, &right, area.width.saturating_sub(right_width), y);

        // Render key hints on second row if height > 1
        if self.height > 1 && !self.key_hints.is_empty() {
            self.render_key_hints(ctx, 0, y + 1, area.width);
        } else if self.height == 1 && !self.key_hints.is_empty() {
            // Render key hints in remaining space
            let hints_start = left_width + 2;
            let hints_end = area.width.saturating_sub(right_width).saturating_sub(2);
            if hints_start < hints_end {
                self.render_key_hints_inline(ctx, hints_start, y, hints_end - hints_start);
            }
        }
    }
}

/// Columns a group takes: each section plus the gap after it.
fn group_width(sections: &[&StatusSection]) -> u16 {
    sections.iter().map(|s| s.width() + 1).sum()
}

impl StatusBar {
    /// The left, center and right sections to draw in `width` columns.
    ///
    /// While the sections do not fit, the one with the lowest priority is
    /// left out; among equal priorities, the later one (left, then center,
    /// then right). Sections at the highest priority present are always
    /// kept, so a bar whose sections share one priority draws them all.
    fn visible_sections(&self, width: u16) -> [Vec<&StatusSection>; 3] {
        let groups = [&self.left, &self.center, &self.right];
        let mut shown: Vec<Vec<bool>> = groups.iter().map(|g| vec![true; g.len()]).collect();
        let top = groups
            .iter()
            .flat_map(|g| g.iter().map(|s| s.priority))
            .max()
            .unwrap_or(0);
        let needed = |shown: &[Vec<bool>]| -> u32 {
            groups
                .iter()
                .zip(shown)
                .flat_map(|(g, keep)| g.iter().zip(keep))
                .filter(|(_, keep)| **keep)
                .map(|(s, _)| u32::from(s.width()) + 1)
                .sum()
        };

        while needed(&shown) > u32::from(width) {
            let least = groups
                .iter()
                .enumerate()
                .flat_map(|(g, sections)| sections.iter().enumerate().map(move |(i, s)| (g, i, s)))
                .filter(|&(g, i, s)| shown[g][i] && s.priority < top)
                // lowest priority; among ties, the latest position
                .min_by_key(|&(g, i, s)| (s.priority, std::cmp::Reverse((g, i))));
            match least {
                Some((g, i, _)) => shown[g][i] = false,
                None => break,
            }
        }

        let pick = |g: usize| -> Vec<&StatusSection> {
            groups[g]
                .iter()
                .zip(&shown[g])
                .filter(|(_, keep)| **keep)
                .map(|(s, _)| s)
                .collect()
        };
        [pick(0), pick(1), pick(2)]
    }

    /// Draw one group of sections from `x`, each followed by a one-column
    /// gap; between two sections the gap holds the separator, if any.
    /// Returns the column after the group.
    fn render_group(
        &self,
        ctx: &mut RenderContext,
        sections: &[&StatusSection],
        mut x: u16,
        y: u16,
    ) -> u16 {
        for (i, section) in sections.iter().enumerate() {
            x = self.render_section(ctx, section, x, y);
            if x >= ctx.area.width {
                continue;
            }
            if let Some(sep) = self.separator.filter(|_| i + 1 < sections.len()) {
                let mut cell = Cell::new(sep);
                cell.fg = Some(self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE)));
                cell.bg = Some(self.bg);
                ctx.set(x, y, cell);
            }
            x += 1;
        }
        x
    }

    fn render_section(
        &self,
        ctx: &mut RenderContext,
        section: &StatusSection,
        x: u16,
        y: u16,
    ) -> u16 {
        // A section's own color wins; otherwise the stylesheet, then the bar's.
        let fg = section
            .fg
            .unwrap_or_else(|| self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE)));
        let bg = section.bg.unwrap_or(self.bg);

        // Advance by each character's column width, so a section takes the
        // columns `StatusSection::width` reports.
        let mut current_x = x;
        for ch in section.content.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            if cw == 0 {
                continue;
            }
            if current_x + cw > ctx.area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            if section.bold {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.set(current_x, y, cell);
            for i in 1..cw {
                let mut cont = Cell::continuation();
                cont.bg = Some(bg);
                ctx.set(current_x + i, y, cont);
            }
            current_x += cw;
        }

        // Pad to min_width
        while current_x < x + section.min_width && current_x < ctx.area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(bg);
            ctx.set(current_x, y, cell);
            current_x += 1;
        }

        current_x
    }

    /// Key hints on their own row, laid out in terminal columns and cut off
    /// at `x + width`
    fn render_key_hints(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16) {
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let (key_fg, key_bg, bg) = (self.key_fg, self.key_bg, self.bg);
        let end = x.saturating_add(width);
        let mut current_x = x;

        for hint in &self.key_hints {
            if current_x >= end {
                break;
            }

            // Render key
            current_x += ctx.put_str_with(current_x, y, &hint.key, end, |ch| {
                Cell::new(ch).fg(key_fg).bg(key_bg).bold()
            });

            // Render description
            let desc = format!(" {} ", hint.description);
            current_x +=
                ctx.put_str_with(current_x, y, &desc, end, |ch| Cell::new(ch).fg(fg).bg(bg));
        }
    }

    /// Key hints between the left and right sections: only hints that fit
    /// whole in `width` columns are drawn
    fn render_key_hints_inline(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16) {
        // Resolved once: the builder's color if it named one, else the
        // stylesheet's, else white.
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let (key_fg, key_bg, bg) = (self.key_fg, self.key_bg, self.bg);
        let end = x.saturating_add(width);
        let mut current_x = x;

        for hint in &self.key_hints {
            let hint_width = crate::utils::display_width(&hint.key)
                + crate::utils::display_width(&hint.description)
                + 3;
            if current_x as usize + hint_width > end as usize {
                break;
            }

            // Render key
            current_x += ctx.put_str_with(current_x, y, &hint.key, end, |ch| {
                Cell::new(ch).fg(key_fg).bg(key_bg)
            });

            // Space
            current_x += 1;

            // Render description
            current_x += ctx.put_str_with(current_x, y, &hint.description, end, |ch| {
                Cell::new(ch).fg(fg).bg(bg)
            });

            // Separator
            current_x += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    fn render_bar(bar: &StatusBar, width: u16) -> Buffer {
        let mut buffer = Buffer::new(20, 2);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 1));
        bar.render(&mut ctx);
        buffer
    }

    #[test]
    fn test_render_right_sections_wider_than_area() {
        let bar = StatusBar::new().right_text("a right section");
        let buffer = render_bar(&bar, 5);
        // Drawn from the left edge and cut at the area's width
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'a');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'g');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_render_inline_hints_without_room_beside_right_sections() {
        // The right section leaves one column, less than the hints' margin
        let bar = StatusBar::new().right_text("12345678").key("q", "Quit");
        let buffer = render_bar(&bar, 10);
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '1');
        assert_eq!(buffer.get(8, 0).unwrap().symbol, '8');
    }

    fn row(buffer: &Buffer, width: u16) -> String {
        (0..width)
            .map(|x| buffer.get(x, 0).unwrap().symbol)
            .collect()
    }

    #[test]
    fn test_render_sections_without_separator_keep_a_gap() {
        let bar = StatusBar::new().left_text("main").left_text("UTF-8");
        assert_eq!(row(&render_bar(&bar, 12), 12), "main UTF-8  ");
    }

    #[test]
    fn test_render_right_sections_without_separator_sit_where_separated_ones_do() {
        let plain = StatusBar::new().right_text("ab").right_text("cd");
        assert_eq!(row(&render_bar(&plain, 10), 10), "    ab cd ");

        let separated = StatusBar::new()
            .right_text("ab")
            .right_text("cd")
            .separator('|');
        assert_eq!(row(&render_bar(&separated, 10), 10), "    ab|cd ");
    }

    #[test]
    fn test_render_in_zero_width_area() {
        let bar = StatusBar::new()
            .left_text("L")
            .right_text("R")
            .key("q", "Quit");
        render_bar(&bar, 0);
    }
}
