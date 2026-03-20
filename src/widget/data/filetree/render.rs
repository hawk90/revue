//! Rendering implementation for the FileTree widget

use super::FileTree;
use crate::render::{Cell, Modifier};
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::{RenderContext, View};

impl View for FileTree {
    crate::impl_view_meta!("FileTree");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let entries = self.visible_entries();
        let visible_height = if self.height > 0 {
            self.height
        } else {
            area.height
        } as usize;

        // Adjust scroll
        let scroll = if self.selected >= self.scroll + visible_height {
            self.selected - visible_height + 1
        } else if self.selected < self.scroll {
            self.selected
        } else {
            self.scroll
        };

        for (i, entry) in entries.iter().skip(scroll).take(visible_height).enumerate() {
            let y = i as u16;
            if y >= area.height {
                break;
            }

            let is_selected = scroll + i == self.selected;
            let indent = entry.depth as u16 * self.indent;

            // Clear line
            for x in 0..area.width {
                let mut cell = Cell::new(' ');
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                }
                ctx.set(x, y, cell);
            }

            let mut x = indent;

            // Draw expand/collapse indicator for directories
            if entry.is_dir() {
                let indicator = if entry.expanded { '▼' } else { '▶' };
                let mut cell = Cell::new(indicator);
                cell.fg = Some(self.dir_fg);
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                }
                ctx.set(x, y, cell);
                x += 2;
            } else {
                x += 2;
            }

            // Draw icon
            if self.show_icons {
                let icon = if self.simple_icons {
                    entry.file_type.simple_icon()
                } else {
                    entry.file_type.icon()
                };
                let mut cell = Cell::new(icon);
                cell.fg = Some(entry.file_type.color());
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                }
                ctx.set(x, y, cell);
                x += 2;
            }

            // Draw name
            let fg = if is_selected {
                self.selected_fg
            } else {
                entry.file_type.color()
            };

            for ch in entry.name.chars() {
                if x >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                }
                if entry.is_dir() {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.set(x, y, cell);
                x += 1;
            }

            // Draw size
            if self.show_sizes && !entry.is_dir() {
                let size_str = entry.format_size();
                let size_x = area.width - size_str.len() as u16 - 1;
                if size_x > x {
                    for (j, ch) in size_str.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(LIGHT_GRAY);
                        if is_selected {
                            cell.bg = Some(self.selected_bg);
                        }
                        ctx.set(size_x + j as u16, y, cell);
                    }
                }
            }
        }
    }
}
