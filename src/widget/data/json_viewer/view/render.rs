//! JsonViewer rendering: the `View` impl

use super::JsonViewer;
use crate::render::Cell;
use crate::widget::data::json_viewer::helpers::line_number_width;
use crate::widget::data::json_viewer::types::JsonType;
use crate::widget::traits::{RenderContext, View};

impl View for JsonViewer {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 2 {
            return;
        }

        let nodes = self.get_visible_nodes();
        let total_lines = nodes.len();
        let line_num_width = line_number_width(self.show_line_numbers, total_lines);
        let content_start_x = line_num_width;
        let content_width = area.width.saturating_sub(line_num_width);

        // Adjust scroll to keep selection visible
        let visible_rows = area.height as usize;
        let mut scroll = self.scroll;
        if self.selected < scroll {
            scroll = self.selected;
        } else if self.selected >= scroll + visible_rows {
            scroll = self.selected.saturating_sub(visible_rows - 1);
        }

        for (visible_idx, node) in nodes.iter().skip(scroll).take(visible_rows).enumerate() {
            let y = visible_idx as u16;
            if y >= area.height {
                break;
            }

            let line_idx = scroll + visible_idx;
            let is_selected = line_idx == self.selected;
            let is_match = self.search_state.search_matches.contains(&node.path);

            // Line number
            if self.show_line_numbers {
                let num_str = format!(
                    "{:>width$}",
                    line_idx + 1,
                    width = (line_num_width - 1) as usize
                );
                for (i, ch) in num_str.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = self.line_number_fg;
                    cell.bg = if is_selected {
                        self.selected_bg
                    } else {
                        self.bg
                    };
                    ctx.set(i as u16, y, cell);
                }
            }

            // Indent
            let indent = (node.depth as u16) * self.indent_size;
            let mut x = content_start_x + indent;

            // Draw collapse indicator for containers
            if node.is_container() {
                let indicator = if self.collapsed.contains(&node.path) {
                    "▶ "
                } else {
                    "▼ "
                };
                for ch in indicator.chars() {
                    if x < area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = self.bracket_fg;
                        cell.bg = if is_selected {
                            self.selected_bg
                        } else {
                            self.bg
                        };
                        ctx.set(x, y, cell);
                        x += 1;
                    }
                }
            }

            // Determine colors
            let (fg, bg) = if is_selected {
                (self.selected_fg, self.selected_bg)
            } else if is_match {
                (self.match_fg, self.match_bg)
            } else {
                // The base row takes the stylesheet; selection, matches and the
                // per-token colors keep theirs.
                (
                    self.fg.or_else(|| ctx.css_color_if_set()),
                    self.bg.or_else(|| ctx.css_background_if_set()),
                )
            };

            // Draw key if present
            if !node.key.is_empty() {
                let key_display = format!("\"{}\"", node.key);
                for ch in key_display.chars() {
                    if x < area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = if is_selected { fg } else { self.key_fg };
                        cell.bg = bg;
                        ctx.set(x, y, cell);
                        x += 1;
                    }
                }

                // Colon separator
                let sep = ": ";
                for ch in sep.chars() {
                    if x < area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = fg.or(self.fg);
                        cell.bg = bg;
                        ctx.set(x, y, cell);
                        x += 1;
                    }
                }
            }

            // Draw value or container brackets
            match &node.value_type {
                JsonType::Object => {
                    let text = if self.collapsed.contains(&node.path) {
                        format!("{{...}} ({} items)", node.child_count())
                    } else if node.children.is_empty() {
                        "{}".to_string()
                    } else {
                        "{".to_string()
                    };
                    for ch in text.chars() {
                        if x < area.width {
                            let mut cell = Cell::new(ch);
                            cell.fg = if is_selected { fg } else { self.bracket_fg };
                            cell.bg = bg;
                            ctx.set(x, y, cell);
                            x += 1;
                        }
                    }
                }
                JsonType::Array => {
                    let text = if self.collapsed.contains(&node.path) {
                        format!("[...] ({} items)", node.child_count())
                    } else if node.children.is_empty() {
                        "[]".to_string()
                    } else {
                        "[".to_string()
                    };
                    for ch in text.chars() {
                        if x < area.width {
                            let mut cell = Cell::new(ch);
                            cell.fg = if is_selected { fg } else { self.bracket_fg };
                            cell.bg = bg;
                            ctx.set(x, y, cell);
                            x += 1;
                        }
                    }
                }
                JsonType::String => {
                    if let Some(value) = &node.value {
                        let display = format!("\"{}\"", value);
                        let truncated: String = display
                            .chars()
                            .take((content_width.saturating_sub(indent + 2)) as usize)
                            .collect();
                        for ch in truncated.chars() {
                            if x < area.width {
                                let mut cell = Cell::new(ch);
                                cell.fg = if is_selected { fg } else { self.string_fg };
                                cell.bg = bg;
                                ctx.set(x, y, cell);
                                x += 1;
                            }
                        }
                    }
                }
                JsonType::Number => {
                    if let Some(value) = &node.value {
                        for ch in value.chars() {
                            if x < area.width {
                                let mut cell = Cell::new(ch);
                                cell.fg = if is_selected { fg } else { self.number_fg };
                                cell.bg = bg;
                                ctx.set(x, y, cell);
                                x += 1;
                            }
                        }
                    }
                }
                JsonType::Boolean => {
                    if let Some(value) = &node.value {
                        for ch in value.chars() {
                            if x < area.width {
                                let mut cell = Cell::new(ch);
                                cell.fg = if is_selected { fg } else { self.bool_fg };
                                cell.bg = bg;
                                ctx.set(x, y, cell);
                                x += 1;
                            }
                        }
                    }
                }
                JsonType::Null => {
                    for ch in "null".chars() {
                        if x < area.width {
                            let mut cell = Cell::new(ch);
                            cell.fg = if is_selected { fg } else { self.null_fg };
                            cell.bg = bg;
                            ctx.set(x, y, cell);
                            x += 1;
                        }
                    }
                }
            }

            // Fill rest of line with background
            while x < area.width {
                let mut cell = Cell::new(' ');
                cell.bg = bg;
                ctx.set(x, y, cell);
                x += 1;
            }
        }
    }

    crate::impl_view_meta!("JsonViewer");
}
