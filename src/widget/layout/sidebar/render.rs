//! Sidebar rendering

use super::types::{CollapseMode, FlattenedItem};
use super::Sidebar;
use crate::render::Cell;
use crate::utils::{char_width, display_width, truncate_to_width};
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
        for y in 0..area.height {
            for x in 0..content_width {
                let mut cell = Cell::new(' ');
                cell.bg = self.bg;
                ctx.set(x, y, cell);
            }
        }

        let mut y: u16 = 0;

        // Render header if present
        if let Some(header) = &self.header {
            if !is_collapsed {
                let display = truncate_to_width(header, content_width as usize - 2);
                let x_offset = (content_width as usize).saturating_sub(display_width(display)) / 2;
                let mut dx: u16 = 0;
                for ch in display.chars() {
                    let cw = char_width(ch) as u16;
                    if x_offset as u16 + dx + cw > content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch).bold();
                    cell.fg = self.fg;
                    cell.bg = self.bg;
                    ctx.set(x_offset as u16 + dx, y, cell);
                    dx += cw;
                }
            }
            y += 1;

            // Separator line after header
            for x in 0..content_width {
                let mut cell = Cell::new('─');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.set(x, y, cell);
            }
            y += 1;
        }

        // Calculate available height for items
        let footer_height = if self.footer.is_some() { 2 } else { 0 };
        let _available_height = area.height.saturating_sub(y + footer_height);

        // Get visible items
        let items = self.visible_items();

        // Render items
        for (idx, flat_item) in items.iter().skip(self.scroll).enumerate() {
            if y >= area.height - footer_height {
                break;
            }

            match flat_item {
                FlattenedItem::Section(title) => {
                    if !is_collapsed {
                        if let Some(title_text) = title {
                            // Section title
                            let display = truncate_to_width(title_text, content_width as usize - 2);
                            let mut dx: u16 = 0;
                            for ch in display.chars() {
                                let cw = char_width(ch) as u16;
                                if 1 + dx + cw > content_width {
                                    break;
                                }
                                let mut cell = Cell::new(ch);
                                cell.fg = self.section_fg;
                                cell.bg = self.bg;
                                ctx.set(1 + dx, y, cell);
                                dx += cw;
                            }
                        } else {
                            // Separator line
                            for x in 1..content_width - 1 {
                                let mut cell = Cell::new('─');
                                cell.fg = self.border_fg;
                                cell.bg = self.bg;
                                ctx.set(x, y, cell);
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
                    for x in 0..content_width {
                        let mut cell = Cell::new(' ');
                        cell.bg = bg;
                        ctx.set(x, y, cell);
                    }

                    let indent = if is_collapsed { 0 } else { (*depth as u16) * 2 };
                    let mut x: u16 = 1 + indent;

                    // Expand/collapse indicator for items with children
                    if item.has_children() && !is_collapsed {
                        let indicator = if item.expanded { '▼' } else { '▶' };
                        let mut cell = Cell::new(indicator);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.set(x, y, cell);
                        x += 2;
                    } else if !is_collapsed {
                        x += 2; // Align with items that have children
                    }

                    // Icon
                    if let Some(icon) = item.icon {
                        let mut cell = Cell::new(icon);
                        cell.fg = fg;
                        cell.bg = bg;
                        ctx.set(x, y, cell);
                        x += 2;
                    }

                    // Label (only if not collapsed)
                    if !is_collapsed {
                        let max_label_width = content_width.saturating_sub(x + 1);
                        let badge_space = item
                            .badge
                            .as_ref()
                            .map(|b| display_width(b) + 2)
                            .unwrap_or(0);
                        let label_width = (max_label_width as usize).saturating_sub(badge_space);
                        let display = truncate_to_width(&item.label, label_width);

                        for ch in display.chars() {
                            let cw = char_width(ch) as u16;
                            if x + cw > content_width.saturating_sub(badge_space as u16) {
                                break;
                            }
                            let mut cell = Cell::new(ch);
                            cell.fg = fg;
                            cell.bg = bg;
                            ctx.set(x, y, cell);
                            x += cw;
                        }

                        // Badge
                        if let Some(badge) = &item.badge {
                            let badge_x =
                                content_width.saturating_sub(display_width(badge) as u16 + 2);
                            let mut dx: u16 = 0;
                            for ch in badge.chars() {
                                let cw = char_width(ch) as u16;
                                if badge_x + dx + cw > content_width {
                                    break;
                                }
                                let mut cell = Cell::new(ch);
                                cell.fg = self.badge_fg;
                                cell.bg = self.badge_bg;
                                ctx.set(badge_x + dx, y, cell);
                                dx += cw;
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
            let footer_y = area.height - 2;
            for x in 0..content_width {
                let mut cell = Cell::new('─');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.set(x, footer_y, cell);
            }

            if !is_collapsed {
                let display = truncate_to_width(footer, content_width as usize - 2);
                let x_offset = (content_width as usize).saturating_sub(display_width(display)) / 2;
                let mut dx: u16 = 0;
                for ch in display.chars() {
                    let cw = char_width(ch) as u16;
                    if x_offset as u16 + dx + cw > content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = self.section_fg;
                    cell.bg = self.bg;
                    ctx.set(x_offset as u16 + dx, footer_y + 1, cell);
                    dx += cw;
                }
            }
        }

        // Right border
        for y in 0..area.height {
            let border_x = content_width - 1;
            if border_x < area.width {
                let mut cell = Cell::new('│');
                cell.fg = self.border_fg;
                cell.bg = self.bg;
                ctx.set(border_x, y, cell);
            }
        }
    }
}
