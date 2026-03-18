//! Rendering implementation for the multi-select widget

use crate::impl_view_meta;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

use super::types::MultiSelect;

impl View for MultiSelect {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        let (fg, bg) =
            self.state
                .resolve_colors_interactive(ctx.style, Color::WHITE, Color::rgb(50, 50, 50));

        let width = self.display_width(area.width);

        // Draw background for header row
        for x in 0..width {
            let mut cell = Cell::new(' ');
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            ctx.set(x, 0, cell);
        }

        // Draw arrow
        let arrow = if self.open { '▲' } else { '▼' };
        ctx.draw_char(width - 1, 0, arrow, fg);

        // Draw tags or placeholder
        let mut x: u16 = 0;
        let max_x = width - 2; // Leave room for arrow

        if self.selected.is_empty() && !self.open {
            // Draw placeholder
            ctx.draw_text(x, 0, &self.placeholder, Color::rgb(128, 128, 128));
        } else {
            // Draw tags
            for (i, &opt_idx) in self.selected.iter().enumerate() {
                if x >= max_x {
                    break;
                }

                if let Some(opt) = self.options.get(opt_idx) {
                    let label = &opt.label;
                    let tag_len = (crate::utils::display_width(label) + 3) as u16; // "[label] "

                    if x + tag_len > max_x {
                        // Draw overflow indicator
                        ctx.draw_text(x, 0, "...", Color::rgb(150, 150, 150));
                        break;
                    }

                    let is_tag_selected = self.tag_cursor == Some(i);
                    let tag_fg = if is_tag_selected {
                        Color::WHITE
                    } else {
                        Color::rgb(200, 200, 200)
                    };
                    let tag_bg_color = if is_tag_selected {
                        Color::rgb(100, 100, 200)
                    } else {
                        self.tag_bg.unwrap_or(Color::rgb(60, 60, 140))
                    };

                    // Draw tag with brackets
                    ctx.draw_char_bg(x, 0, '[', tag_fg, tag_bg_color);
                    x += 1;

                    for ch in label.chars() {
                        let cw = crate::utils::char_width(ch) as u16;
                        if x + cw > max_x - 1 {
                            break;
                        }
                        ctx.draw_char_bg(x, 0, ch, tag_fg, tag_bg_color);
                        x += cw;
                    }

                    ctx.draw_char_bg(x, 0, ']', tag_fg, tag_bg_color);
                    x += 1;

                    // Space between tags
                    if x < max_x {
                        x += 1;
                    }
                }
            }

            // Draw search query if open
            if self.open && self.searchable && !self.query.is_empty() {
                let query_display = format!(" {}", self.query);
                ctx.draw_text(x.min(max_x), 0, &query_display, Color::CYAN);
            }
        }

        // Draw dropdown if open
        if self.open && area.height > 1 {
            let max_visible = (area.height - 1) as usize;

            for (row, &opt_idx) in self.filtered.iter().enumerate().take(max_visible) {
                let y = 1 + row as u16;
                let is_cursor = row == self.dropdown_cursor;
                let is_selected = self.is_selected(opt_idx);

                if let Some(opt) = self.options.get(opt_idx) {
                    let (row_fg, row_bg) = if is_cursor {
                        (Color::WHITE, Color::rgb(80, 80, 150))
                    } else {
                        (fg, bg)
                    };

                    // Draw row background
                    for dx in 0..width {
                        let mut cell = Cell::new(' ');
                        cell.fg = Some(row_fg);
                        cell.bg = Some(row_bg);
                        ctx.set(dx, y, cell);
                    }

                    // Draw checkbox
                    let checkbox_str = if is_selected { "[x]" } else { "[ ]" };
                    ctx.draw_text_bg(0, y, checkbox_str, row_fg, row_bg);

                    // Draw label with highlight
                    let match_indices: Vec<usize> = self
                        .get_match(&opt.label)
                        .map(|m| m.indices)
                        .unwrap_or_default();

                    let mut cx: u16 = 4;
                    for (j, ch) in opt.label.chars().enumerate() {
                        let cw = crate::utils::char_width(ch) as u16;
                        if cx + cw > width {
                            break;
                        }

                        let char_fg = if match_indices.contains(&j) {
                            self.highlight_fg.unwrap_or(Color::YELLOW)
                        } else if opt.disabled {
                            Color::rgb(100, 100, 100)
                        } else {
                            row_fg
                        };

                        ctx.draw_char_bg(cx, y, ch, char_fg, row_bg);
                        cx += cw;
                    }
                }
            }
        }
    }

    impl_view_meta!("MultiSelect");
}
