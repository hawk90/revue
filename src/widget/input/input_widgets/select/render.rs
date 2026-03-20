//! View implementation for Select

use crate::render::Cell;
use crate::style::Color;
use crate::utils::truncate_to_width;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::{RenderContext, View};

use super::Select;

impl View for Select {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let width = self.display_width(area.width);
        let text_width = (width - 2) as usize;

        // Determine colors based on state
        let fg = if self.disabled {
            Some(PLACEHOLDER_FG)
        } else if self.focused {
            self.fg.or(Some(Color::CYAN))
        } else {
            self.fg
        };
        let bg = self.bg;

        // Render the closed/header row
        let display_text = if self.open && self.searchable && !self.query.is_empty() {
            // Show search query when searching
            &self.query
        } else {
            self.value().unwrap_or(&self.placeholder)
        };
        let arrow = if self.open { "▲" } else { "▼" };

        // Draw background for header
        ctx.fill_row(0, width, fg, bg);

        // Draw focus indicator (inside area bounds)
        if self.focused && !self.disabled {
            ctx.draw_focus_brackets(0, width, Color::CYAN);
        }

        // Draw arrow (or search icon when searching)
        let icon = if self.open && self.searchable {
            "🔍".chars().next().unwrap_or('?')
        } else {
            arrow.chars().next().unwrap_or('▼')
        };
        let mut cell = Cell::new(icon);
        cell.fg = fg;
        cell.bg = bg;
        ctx.set(0, 0, cell);

        // Draw text
        let truncated = truncate_to_width(display_text, text_width);
        let mut cx: u16 = 2;
        for ch in truncated.chars() {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            ctx.set(cx, 0, cell);
            cx += crate::utils::char_width(ch) as u16;
        }

        // If open, render dropdown options as overlay (escapes parent clipping)
        if self.open {
            // Determine which options to show
            let visible_options: Vec<(usize, &String)> = if self.query.is_empty() {
                self.options.iter().enumerate().collect()
            } else {
                self.filtered
                    .iter()
                    .filter_map(|&i| self.options.get(i).map(|opt| (i, opt)))
                    .collect()
            };

            // Calculate dropdown height (limited to 10 or option count)
            let dropdown_height = if visible_options.is_empty() {
                1u16 // "No results" row
            } else {
                (visible_options.len() as u16).min(10)
            };

            // Calculate absolute position for overlay, flip above if near bottom
            let (abs_x, abs_y) = ctx.absolute_position();
            let buf_height = ctx.buffer.height();
            let space_below = buf_height.saturating_sub(abs_y + 1);
            let overlay_y = if space_below >= dropdown_height {
                abs_y + 1 // Render below
            } else {
                abs_y.saturating_sub(dropdown_height) // Render above
            };
            let overlay_area = crate::layout::Rect::new(abs_x, overlay_y, width, dropdown_height);

            let mut entry = crate::widget::traits::OverlayEntry::new(100, overlay_area);

            if visible_options.is_empty() && !self.query.is_empty() {
                // "No results" message
                let msg = "No results";
                for (i, ch) in msg.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(PLACEHOLDER_FG);
                    entry.push(2 + i as u16, 0, cell);
                }
            }

            for (row, (option_idx, option)) in visible_options
                .iter()
                .enumerate()
                .take(dropdown_height as usize)
            {
                let y = row as u16;
                let is_selected = self.selection.is_selected(*option_idx);

                let (fg, bg) = if is_selected {
                    (self.selected_fg, self.selected_bg)
                } else {
                    (self.fg, self.bg)
                };

                // Draw background
                for x in 0..width {
                    let mut cell = Cell::new(' ');
                    cell.fg = fg;
                    cell.bg = bg;
                    entry.push(x, y, cell);
                }

                // Draw selection indicator
                let indicator = if is_selected { '›' } else { ' ' };
                let mut cell = Cell::new(indicator);
                cell.fg = fg;
                cell.bg = bg;
                entry.push(0, y, cell);

                // Get fuzzy match indices for highlighting (HashSet for O(1) lookup)
                let match_indices: std::collections::HashSet<usize> = self
                    .get_match(option)
                    .map(|m| m.indices.into_iter().collect())
                    .unwrap_or_default();

                // Draw option text with highlighting
                let truncated = truncate_to_width(option, text_width);
                let mut cx: u16 = 2;
                for (j, ch) in truncated.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.bg = bg;

                    if match_indices.contains(&j) {
                        cell.fg = self.highlight_fg;
                    } else {
                        cell.fg = fg;
                    }

                    entry.push(cx, y, cell);
                    cx += crate::utils::char_width(ch) as u16;
                }
            }

            // Queue as overlay; falls back to inline if no overlay support
            if !ctx.queue_overlay(entry.clone()) {
                // Fallback: render inline (clipped by parent)
                for oc in &entry.cells {
                    ctx.set(oc.x, oc.y + 1, oc.cell);
                }
            }
        }
    }

    crate::impl_view_meta!("Select");
}
