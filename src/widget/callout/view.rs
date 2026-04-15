//! View implementation for Callout

use super::core::Callout;
use super::types::CalloutVariant;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::{MUTED_TEXT, SECONDARY_TEXT};
use crate::widget::traits::{RenderContext, View};
use unicode_width::UnicodeWidthChar;

impl View for Callout {
    crate::impl_view_meta!("Callout");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        let accent_color = self.callout_type.accent_color();
        let bg_color = self.callout_type.bg_color();
        let title_color = self.callout_type.title_color();

        match self.variant {
            CalloutVariant::Filled => {
                self.render_filled(ctx, accent_color, bg_color, title_color);
            }
            CalloutVariant::LeftBorder => {
                self.render_left_border(ctx, accent_color, title_color);
            }
            CalloutVariant::Minimal => {
                self.render_minimal(ctx, accent_color, title_color);
            }
        }
    }
}

impl Callout {
    fn render_filled(
        &self,
        ctx: &mut RenderContext,
        accent_color: Color,
        bg_color: Color,
        title_color: Color,
    ) {
        let area = ctx.area;

        // Fill background
        for y in 0..area.height {
            for x in 0..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg_color);
                ctx.set(x, y, cell);
            }
        }

        // Draw left accent border
        for y in 0..area.height {
            let mut cell = Cell::new('┃');
            cell.fg = Some(accent_color);
            cell.bg = Some(bg_color);
            ctx.set(0, y, cell);
        }

        // Header line
        let mut x: u16 = 2;

        // Collapse icon (if collapsible)
        if self.collapsible {
            let mut icon_cell = Cell::new(self.collapse_icon());
            icon_cell.fg = Some(title_color);
            icon_cell.bg = Some(bg_color);
            ctx.set(x, 0, icon_cell);
            x += 2;
        }

        // Type icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            icon_cell.bg = Some(bg_color);
            ctx.set(x, 0, icon_cell);
            x += 2;
        }

        // Title
        let title = self.get_title();
        let max_title_x = area.width - 1;
        for ch in title.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if x + char_width > max_title_x {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(title_color);
            cell.bg = Some(bg_color);
            cell.modifier |= Modifier::BOLD;
            ctx.set(x, 0, cell);
            // Set continuation cells for wide characters
            for i in 1..char_width {
                ctx.set(x + i, 0, Cell::continuation());
            }
            x += char_width;
        }

        // Content (if expanded or not collapsible)
        if !self.collapsible || self.expanded {
            let content_x: u16 = 2;
            let content_width = area.width.saturating_sub(3);

            for (i, line) in self.content.lines().enumerate() {
                let line_y = 1 + i as u16;
                if line_y >= area.height {
                    break;
                }

                let mut offset = 0u16;
                for ch in line.chars() {
                    let char_width = ch.width().unwrap_or(0) as u16;
                    if char_width == 0 {
                        continue;
                    }
                    if offset + char_width > content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(SECONDARY_TEXT);
                    cell.bg = Some(bg_color);
                    ctx.set(content_x + offset, line_y, cell);
                    for i in 1..char_width {
                        ctx.set(content_x + offset + i, line_y, Cell::continuation());
                    }
                    offset += char_width;
                }
            }
        }
    }

    fn render_left_border(&self, ctx: &mut RenderContext, accent_color: Color, title_color: Color) {
        let area = ctx.area;

        // Draw left accent border
        for y in 0..area.height {
            let mut cell = Cell::new('┃');
            cell.fg = Some(accent_color);
            ctx.set(0, y, cell);
        }

        // Header line
        let mut x: u16 = 2;

        // Collapse icon (if collapsible)
        if self.collapsible {
            let mut icon_cell = Cell::new(self.collapse_icon());
            icon_cell.fg = Some(title_color);
            ctx.set(x, 0, icon_cell);
            x += 2;
        }

        // Type icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            ctx.set(x, 0, icon_cell);
            x += 2;
        }

        // Title
        let title = self.get_title();
        let max_title_x = area.width;
        for ch in title.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if x + char_width > max_title_x {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(title_color);
            cell.modifier |= Modifier::BOLD;
            ctx.set(x, 0, cell);
            for i in 1..char_width {
                ctx.set(x + i, 0, Cell::continuation());
            }
            x += char_width;
        }

        // Content (if expanded or not collapsible)
        if !self.collapsible || self.expanded {
            let content_x: u16 = 2;
            let content_width = area.width.saturating_sub(3);

            for (i, line) in self.content.lines().enumerate() {
                let line_y = 1 + i as u16;
                if line_y >= area.height {
                    break;
                }

                let mut offset = 0u16;
                for ch in line.chars() {
                    let char_width = ch.width().unwrap_or(0) as u16;
                    if char_width == 0 {
                        continue;
                    }
                    if offset + char_width > content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(MUTED_TEXT);
                    ctx.set(content_x + offset, line_y, cell);
                    for i in 1..char_width {
                        ctx.set(content_x + offset + i, line_y, Cell::continuation());
                    }
                    offset += char_width;
                }
            }
        }
    }

    fn render_minimal(&self, ctx: &mut RenderContext, accent_color: Color, title_color: Color) {
        let area = ctx.area;

        // Header line
        let mut x: u16 = 0;

        // Collapse icon (if collapsible)
        if self.collapsible {
            let mut icon_cell = Cell::new(self.collapse_icon());
            icon_cell.fg = Some(title_color);
            ctx.set(x, 0, icon_cell);
            x += 2;
        }

        // Type icon
        if self.show_icon {
            let icon = self.get_icon();
            let mut icon_cell = Cell::new(icon);
            icon_cell.fg = Some(accent_color);
            ctx.set(x, 0, icon_cell);
            x += 2;
        }

        // Title
        let title = self.get_title();
        let max_title_x = area.width;
        for ch in title.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if x + char_width > max_title_x {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(title_color);
            cell.modifier |= Modifier::BOLD;
            ctx.set(x, 0, cell);
            for i in 1..char_width {
                ctx.set(x + i, 0, Cell::continuation());
            }
            x += char_width;
        }

        // Content (if expanded or not collapsible)
        if !self.collapsible || self.expanded {
            let content_x: u16 = if self.show_icon { 2 } else { 0 };
            let content_width = area
                .width
                .saturating_sub(if self.show_icon { 2 } else { 0 });

            for (i, line) in self.content.lines().enumerate() {
                let line_y = 1 + i as u16;
                if line_y >= area.height {
                    break;
                }

                let mut offset = 0u16;
                for ch in line.chars() {
                    let char_width = ch.width().unwrap_or(0) as u16;
                    if char_width == 0 {
                        continue;
                    }
                    if offset + char_width > content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(MUTED_TEXT);
                    ctx.set(content_x + offset, line_y, cell);
                    for i in 1..char_width {
                        ctx.set(content_x + offset + i, line_y, Cell::continuation());
                    }
                    offset += char_width;
                }
            }
        }
    }
}

crate::impl_styled_view!(Callout);
crate::impl_widget_builders!(Callout);
