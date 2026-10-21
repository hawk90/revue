//! Sidebar rendering: the header, the item rows, the footer and the right
//! border, inside the bar's content width

use super::types::{CollapseMode, FlattenedItem, SidebarItem};
use super::Sidebar;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::traits::RenderContext;

/// How the bar is drawn this frame: collapsed or not, and its width
#[derive(Clone, Copy)]
struct Bar {
    collapsed: bool,
    width: u16,
}

impl Sidebar {
    /// Render the sidebar
    pub(super) fn render_sidebar(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 2 {
            return;
        }

        let bar = self.bar(area.width);
        let (is_collapsed, content_width) = (bar.collapsed, bar.width);
        if content_width == 0 {
            return;
        }

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
                self.draw_centered(ctx, header, y, content_width, self.fg, true);
            }
            y += 1;

            // Separator line after header
            self.draw_separator(ctx, 0..content_width, y);
            y += 1;
        }

        // Rows above the footer
        let footer_height = if self.footer.is_some() { 2 } else { 0 };

        // Get visible items
        let items = self.visible_items();

        // Render items: the rows between the header and the footer, scrolled
        // to keep the hovered row in view (#882)
        let rows = (area.height.saturating_sub(footer_height).saturating_sub(y)) as usize;
        self.scroll.set_content(items.len());
        self.scroll.set_viewport(rows);
        if let Some(hovered) = self.hover.index() {
            self.scroll.track(hovered..hovered + 1);
        }
        let shown = self.scroll.visible();
        for (idx, flat_item) in items.iter().enumerate().skip(shown.start).take(shown.len()) {
            match flat_item {
                FlattenedItem::Section(title) => self.draw_section_row(ctx, bar, title, y),
                FlattenedItem::Item { item, depth } => {
                    self.draw_item_row(ctx, bar, item, *depth, idx, y)
                }
            }
            y += 1;
        }

        // Render footer if present
        if let Some(footer) = &self.footer {
            // Separator line before footer
            let footer_y = area.height - 2;
            self.draw_separator(ctx, 0..content_width, footer_y);

            if !is_collapsed {
                self.draw_centered(
                    ctx,
                    footer,
                    footer_y + 1,
                    content_width,
                    self.section_fg,
                    false,
                );
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

    /// Whether the bar is collapsed in an area `area_width` wide, and the
    /// width it takes there
    fn bar(&self, area_width: u16) -> Bar {
        // Determine if collapsed based on mode and available width
        let is_collapsed = match self.collapse_mode {
            CollapseMode::Expanded => false,
            CollapseMode::Collapsed => true,
            CollapseMode::Auto => area_width < self.collapse_threshold,
        };

        let content_width = if is_collapsed {
            self.collapsed_width.min(area_width)
        } else {
            self.expanded_width.min(area_width)
        };
        Bar {
            collapsed: is_collapsed,
            width: content_width,
        }
    }

    /// Draw `text` centered on row `y` of a bar `content_width` wide (the
    /// header and the footer)
    fn draw_centered(
        &self,
        ctx: &mut RenderContext,
        text: &str,
        y: u16,
        content_width: u16,
        fg: Option<Color>,
        bold: bool,
    ) {
        let display = truncate_to_width(text, (content_width as usize).saturating_sub(2));
        let x_offset = (content_width as usize).saturating_sub(display_width(display)) / 2;
        let mut dx: u16 = 0;
        for ch in display.chars() {
            let cw = char_width(ch) as u16;
            if x_offset as u16 + dx + cw > content_width {
                break;
            }
            let mut cell = if bold {
                Cell::new(ch).bold()
            } else {
                Cell::new(ch)
            };
            cell.fg = fg;
            cell.bg = self.bg;
            ctx.set(x_offset as u16 + dx, y, cell);
            dx += cw;
        }
    }

    /// Draw a separator line across columns `xs` of row `y`
    fn draw_separator(&self, ctx: &mut RenderContext, xs: std::ops::Range<u16>, y: u16) {
        for x in xs {
            let mut cell = Cell::new('─');
            cell.fg = self.border_fg;
            cell.bg = self.bg;
            ctx.set(x, y, cell);
        }
    }

    /// Draw a section row: its title, or a separator line for an untitled
    /// one; nothing while collapsed
    fn draw_section_row(&self, ctx: &mut RenderContext, bar: Bar, title: &Option<String>, y: u16) {
        let content_width = bar.width;
        if !bar.collapsed {
            if let Some(title_text) = title {
                // Section title
                let display =
                    truncate_to_width(title_text, (content_width as usize).saturating_sub(2));
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
                self.draw_separator(ctx, 1..content_width.saturating_sub(1), y);
            }
        }
    }

    /// Draw an item row: the expand indicator, the icon, the label and the
    /// badge, in the disabled, selected, hovered or plain colors
    fn draw_item_row(
        &self,
        ctx: &mut RenderContext,
        bar: Bar,
        item: &SidebarItem,
        depth: usize,
        index: usize,
        y: u16,
    ) {
        let (is_collapsed, content_width) = (bar.collapsed, bar.width);
        let is_selected = self.selected.as_ref() == Some(&item.id);
        let is_hovered = index == self.hovered_index();

        // Determine colors
        let (fg, bg) = if item.disabled {
            (self.disabled_fg, self.bg)
        } else if is_selected {
            (self.selected_fg, self.selected_bg)
        } else if is_hovered {
            (self.hover_fg, self.hover_bg)
        } else {
            // Unselected items take the stylesheet; the selected
            // and hovered ones keep their own.
            (
                self.fg.or_else(|| ctx.css_color_if_set()),
                self.bg.or_else(|| ctx.css_background_if_set()),
            )
        };

        // Fill row background
        for x in 0..content_width {
            let mut cell = Cell::new(' ');
            cell.bg = bg;
            ctx.set(x, y, cell);
        }

        let indent = if is_collapsed { 0 } else { (depth as u16) * 2 };
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

            if let Some(badge) = &item.badge {
                self.draw_badge(ctx, badge, y, content_width);
            }
        }
    }

    /// Draw an item's badge at the right of the bar, before the border column
    fn draw_badge(&self, ctx: &mut RenderContext, badge: &str, y: u16, content_width: u16) {
        let badge_x = content_width.saturating_sub(display_width(badge) as u16 + 2);
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
