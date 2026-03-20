//! Rendering implementation for the multi-select widget

use crate::impl_view_meta;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, PLACEHOLDER_FG};
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
        ctx.fill_row(0, width, Some(fg), Some(bg));

        // Draw arrow
        let arrow = if self.open { '▲' } else { '▼' };
        ctx.draw_char(width - 1, 0, arrow, fg);

        // Draw tags or placeholder
        let mut x: u16 = 0;
        let max_x = width - 2; // Leave room for arrow

        if self.selected.is_empty() && !self.open {
            // Draw placeholder
            ctx.draw_text(x, 0, &self.placeholder, PLACEHOLDER_FG);
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

        // Draw dropdown if open (as overlay to escape parent clipping)
        if self.open {
            let max_visible = self.filtered.len().min(10);
            let dropdown_h = max_visible.max(1) as u16;

            let (abs_x, abs_y) = ctx.absolute_position();
            let buf_height = ctx.buffer.height();
            let space_below = buf_height.saturating_sub(abs_y + 1);
            let overlay_y = if space_below >= dropdown_h {
                abs_y + 1
            } else {
                abs_y.saturating_sub(dropdown_h)
            };
            let overlay_area = crate::layout::Rect::new(abs_x, overlay_y, width, dropdown_h);
            let mut entry = crate::widget::traits::OverlayEntry::new(100, overlay_area);

            for (row, &opt_idx) in self.filtered.iter().enumerate().take(max_visible) {
                let y = row as u16;
                let is_cursor = row == self.dropdown_cursor;
                let is_selected = self.is_selected(opt_idx);

                if let Some(opt) = self.options.get(opt_idx) {
                    let (row_fg, row_bg) = if is_cursor {
                        (Color::WHITE, Color::rgb(80, 80, 150))
                    } else {
                        (fg, bg)
                    };

                    // Row background
                    for dx in 0..width {
                        let mut cell = Cell::new(' ');
                        cell.fg = Some(row_fg);
                        cell.bg = Some(row_bg);
                        entry.push(dx, y, cell);
                    }

                    // Checkbox
                    let checkbox_str = if is_selected { "[x]" } else { "[ ]" };
                    for (i, ch) in checkbox_str.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(row_fg);
                        cell.bg = Some(row_bg);
                        entry.push(i as u16, y, cell);
                    }

                    // Draw label with highlight (HashSet for O(1) lookup)
                    let match_indices: std::collections::HashSet<usize> = self
                        .get_match(&opt.label)
                        .map(|m| m.indices.into_iter().collect())
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
                            DISABLED_FG
                        } else {
                            row_fg
                        };

                        let mut cell = Cell::new(ch);
                        cell.fg = Some(char_fg);
                        cell.bg = Some(row_bg);
                        entry.push(cx, y, cell);
                        cx += cw;
                    }
                }
            }

            // Queue as overlay; fallback to inline
            if !ctx.queue_overlay(entry.clone()) {
                for oc in &entry.cells {
                    ctx.set(oc.x, oc.y + 1, oc.cell);
                }
            }
        }
    }

    impl_view_meta!("MultiSelect");
}
