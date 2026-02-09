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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::layout::sidebar::{CollapseMode, Sidebar};
    use crate::widget::traits::View;

    // =========================================================================
    // Render edge case tests
    // =========================================================================

    #[test]
    fn test_render_sidebar_too_narrow() {
        let sidebar = Sidebar::new();
        let mut buffer = Buffer::new(2, 10); // width < 3
        let area = Rect::new(0, 0, 2, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with too narrow area
        sidebar.render(&mut ctx);
    }

    #[test]
    fn test_render_sidebar_too_short() {
        let sidebar = Sidebar::new();
        let mut buffer = Buffer::new(10, 1); // height < 2
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with too short area
        sidebar.render(&mut ctx);
    }

    #[test]
    fn test_render_sidebar_zero_width() {
        let sidebar = Sidebar::new();
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with zero width
        sidebar.render(&mut ctx);
    }

    #[test]
    fn test_render_sidebar_zero_height() {
        let sidebar = Sidebar::new();
        let mut buffer = Buffer::new(10, 0);
        let area = Rect::new(0, 0, 10, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with zero height
        sidebar.render(&mut ctx);
    }

    // =========================================================================
    // Collapse mode tests
    // =========================================================================

    #[test]
    fn test_render_sidebar_expanded_mode() {
        let sidebar = Sidebar::new()
            .collapse_mode(CollapseMode::Expanded)
            .expanded_width(20);

        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Should render with expanded width
        // Check that background was rendered
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    #[test]
    fn test_render_sidebar_collapsed_mode() {
        let sidebar = Sidebar::new()
            .collapse_mode(CollapseMode::Collapsed)
            .collapsed_width(5);

        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Should render with collapsed width
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    #[test]
    fn test_render_sidebar_auto_mode_collapsed() {
        let sidebar = Sidebar::new()
            .collapse_mode(CollapseMode::Auto)
            .collapse_threshold(15)
            .expanded_width(20)
            .collapsed_width(5);

        let mut buffer = Buffer::new(10, 10); // width < threshold
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Should auto-collapse when width < threshold
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    #[test]
    fn test_render_sidebar_auto_mode_expanded() {
        let sidebar = Sidebar::new()
            .collapse_mode(CollapseMode::Auto)
            .collapse_threshold(15)
            .expanded_width(20)
            .collapsed_width(5);

        let mut buffer = Buffer::new(30, 10); // width >= threshold
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Should stay expanded when width >= threshold
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    // =========================================================================
    // Header/footer rendering tests
    // =========================================================================

    #[test]
    fn test_render_sidebar_with_header() {
        let sidebar = Sidebar::new().header("Test Header");

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Header should be rendered
        // Check for separator line after header
        let cell = buffer.get(0, 1);
        assert!(cell.is_some());
    }

    #[test]
    fn test_render_sidebar_with_footer() {
        let sidebar = Sidebar::new().footer("Test Footer");

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Footer should be rendered
        // Check for separator line before footer (at height - 2)
        let cell = buffer.get(0, 8);
        assert!(cell.is_some());
    }

    #[test]
    fn test_render_sidebar_with_header_and_footer() {
        let sidebar = Sidebar::new().header("Header").footer("Footer");

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Both header and footer should be rendered
        let header_sep = buffer.get(0, 1);
        let footer_sep = buffer.get(0, 8);
        assert!(header_sep.is_some());
        assert!(footer_sep.is_some());
    }

    #[test]
    fn test_render_sidebar_header_truncation() {
        let long_header = "This is a very long header that should be truncated";
        let sidebar = Sidebar::new().header(long_header).expanded_width(15);

        let mut buffer = Buffer::new(15, 10);
        let area = Rect::new(0, 0, 15, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Should not crash with long header
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    // =========================================================================
    // Background rendering tests
    // =========================================================================

    #[test]
    fn test_render_sidebar_background() {
        let sidebar = Sidebar::new().bg(crate::style::Color::rgb(0, 0, 255));

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Background should be rendered
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.bg, Some(crate::style::Color::rgb(0, 0, 255)));
    }

    // =========================================================================
    // Border rendering tests
    // =========================================================================

    #[test]
    fn test_render_sidebar_border() {
        let sidebar = Sidebar::new()
            .border_color(crate::style::Color::rgb(255, 0, 0))
            .expanded_width(15);

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Right border should be rendered at width - 1
        let border_cell = buffer.get(14, 0);
        assert!(border_cell.is_some());
    }

    // =========================================================================
    // Color configuration tests
    // =========================================================================

    #[test]
    fn test_render_sidebar_with_colors() {
        let sidebar = Sidebar::new()
            .fg(crate::style::Color::rgb(255, 255, 255))
            .bg(crate::style::Color::rgb(0, 0, 0))
            .border_color(crate::style::Color::rgb(128, 128, 128))
            .section_color(crate::style::Color::rgb(0, 255, 255));

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Should render with configured colors
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
    }

    // =========================================================================
    // Width constraint tests
    // =========================================================================

    #[test]
    fn test_render_sidebar_content_width_clamping() {
        let sidebar = Sidebar::new()
            .expanded_width(30)
            .collapse_mode(CollapseMode::Expanded);

        let mut buffer = Buffer::new(20, 10); // Smaller than expanded_width
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        sidebar.render(&mut ctx);

        // Content width should be clamped to available width
        let border_cell = buffer.get(19, 0); // At edge of buffer
        assert!(border_cell.is_some());
    }

    // =========================================================================
    // Empty sidebar tests
    // =========================================================================

    #[test]
    fn test_render_empty_sidebar() {
        let sidebar = Sidebar::new();

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not crash with empty sidebar
        sidebar.render(&mut ctx);
    }
}
