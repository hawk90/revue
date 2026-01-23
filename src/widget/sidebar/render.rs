//! Sidebar rendering

use super::types::{CollapseMode, FlattenedItem};
use super::Sidebar;
use crate::render::Cell;
use crate::widget::traits::RenderContext;

impl Sidebar {
    /// Render the sidebar
    pub(super) fn render_sidebar(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 2 {
            return;
        }

        // Determine if collapsed based on mode and available width
        let is_collapsed = match self.collapse_mode {
            CollapseMode::Expanded => false,
            CollapseMode::Collapsed => true,
            CollapseMode::Auto => area.width < self.collapse_threshold,
        };

        let content_width = if is_collapsed {
            self.collapsed_width.min(area.width)
        } else {
            self.expanded_width.min(area.width)
        };

        // Fill background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + content_width {
                let mut cell = Cell::new(' ');
                cell.bg = self.bg;
                ctx.buffer.set(x, y, cell);
            }
        }

        let mut y = area.y;

        // Render header if present
        if let Some(header) = &self.header {
            if !is_collapsed {
                let display: String = header.chars().take(content_width as usize - 2).collect();
                let x_offset = (content_width as usize - display.chars().count()) / 2;
                for (i, ch) in display.chars().enumerate() {
                    let mut cell = Cell::new(ch).bold();
                    cell.fg = self.fg;
                    cell.bg = self.bg;
                    ctx.buffer.set(area.x + x_offset as u16 + i as u16, y, cell);
                }
            }
            y += 1;

            // Separator line after header
            for x in area.x..area.x + content_width {
                let mut cell = Cell::new('─');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.buffer.set(x, y, cell);
            }
            y += 1;
        }

        // Calculate available height for items
        let footer_height = if self.footer.is_some() { 2 } else { 0 };
        let _available_height = area.height.saturating_sub(y - area.y + footer_height);

        // Get visible items
        let items = self.visible_items();

        // Render items
        for (idx, flat_item) in items.iter().skip(self.scroll).enumerate() {
            if y >= area.y + area.height - footer_height {
                break;
            }

            match flat_item {
                FlattenedItem::Section(title) => {
                    if !is_collapsed {
                        if let Some(title_text) = title {
                            // Section title
                            let display: String = title_text
                                .chars()
                                .take(content_width as usize - 2)
                                .collect();
                            for (i, ch) in display.chars().enumerate() {
                                let mut cell = Cell::new(ch);
                                cell.fg = self.section_fg;
                                cell.bg = self.bg;
                                ctx.buffer.set(area.x + 1 + i as u16, y, cell);
                            }
                        } else {
                            // Separator line
                            for x in area.x + 1..area.x + content_width - 1 {
                                let mut cell = Cell::new('─');
                                cell.fg = self.border_fg;
                                cell.bg = self.bg;
                                ctx.buffer.set(x, y, cell);
                            }
                        }
                    }
                    y += 1;
                }
                FlattenedItem::Item { item, depth } => {
                    let actual_idx = self.scroll + idx;
                    let is_selected = self.selected.as_ref() == Some(&item.id);
                    let is_hovered = actual_idx == self.hovered;

                    // Determine colors
                    let (fg, bg) = if item.disabled {
                        (self.disabled_fg, self.bg)
                    } else if is_selected {
                        (self.selected_fg, self.selected_bg)
                    } else if is_hovered {
                        (self.hover_fg, self.hover_bg)
                    } else {
                        (self.fg, self.bg)
                    };

                    // Fill row background
                    for x in area.x..area.x + content_width {
                        let mut cell = Cell::new(' ');
                        cell.bg = bg;
                        ctx.buffer.set(x, y, cell);
                    }

                    let indent = if is_collapsed { 0 } else { (*depth as u16) * 2 };
                    let mut x = area.x + 1 + indent;

                    // Expand/collapse indicator for items with children
                    if item.has_children() && !is_collapsed {
                        let indicator = if item.expanded { '▼' } else { '▶' };
                        let mut cell = Cell::new(indicator);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x, y, cell);
                        x += 2;
                    } else if !is_collapsed {
                        x += 2; // Align with items that have children
                    }

                    // Icon
                    if let Some(icon) = item.icon {
                        let mut cell = Cell::new(icon);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.buffer.set(x, y, cell);
                        x += 2;
                    }

                    // Label (only if not collapsed)
                    if !is_collapsed {
                        let max_label_width = content_width.saturating_sub(x - area.x + 1);
                        let badge_space = item.badge.as_ref().map(|b| b.len() + 2).unwrap_or(0);
                        let label_width = (max_label_width as usize).saturating_sub(badge_space);
                        let display: String = item.label.chars().take(label_width).collect();

                        for ch in display.chars() {
                            if x < area.x + content_width - badge_space as u16 {
                                let mut cell = Cell::new(ch);
                                cell.fg = fg;
                                cell.bg = bg;
                                ctx.buffer.set(x, y, cell);
                                x += 1;
                            }
                        }

                        // Badge
                        if let Some(badge) = &item.badge {
                            let badge_x = area.x + content_width - badge.len() as u16 - 2;
                            for (i, ch) in badge.chars().enumerate() {
                                let mut cell = Cell::new(ch);
                                cell.fg = self.badge_fg;
                                cell.bg = self.badge_bg;
                                ctx.buffer.set(badge_x + i as u16, y, cell);
                            }
                        }
                    }

                    y += 1;
                }
            }
        }

        // Render footer if present
        if let Some(footer) = &self.footer {
            // Separator line before footer
            let footer_y = area.y + area.height - 2;
            for x in area.x..area.x + content_width {
                let mut cell = Cell::new('─');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.buffer.set(x, footer_y, cell);
            }

            if !is_collapsed {
                let display: String = footer.chars().take(content_width as usize - 2).collect();
                let x_offset = (content_width as usize - display.chars().count()) / 2;
                for (i, ch) in display.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = self.section_fg;
                    cell.bg = self.bg;
                    ctx.buffer
                        .set(area.x + x_offset as u16 + i as u16, footer_y + 1, cell);
                }
            }
        }

        // Right border
        for y in area.y..area.y + area.height {
            let border_x = area.x + content_width - 1;
            if border_x < area.x + area.width {
                let mut cell = Cell::new('│');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.buffer.set(border_x, y, cell);
            }
        }
    }
}
