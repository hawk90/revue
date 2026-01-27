//! Color picker rendering implementation

use super::core::ColorPicker;
use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::render_border;
use crate::widget::traits::{RenderContext, View};

impl View for ColorPicker {
    crate::impl_view_meta!("ColorPicker");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 4 {
            return;
        }

        // Draw border if set
        if let Some(border_color) = self.border_color {
            render_border(ctx, area, border_color);
        }

        let content_area = if self.border_color.is_some() {
            Rect::new(
                area.x + 1,
                area.y + 1,
                area.width.saturating_sub(2),
                area.height.saturating_sub(2),
            )
        } else {
            area
        };

        match self.mode {
            super::types::ColorPickerMode::Palette => self.render_palette(ctx, content_area),
            super::types::ColorPickerMode::Rgb => self.render_rgb(ctx, content_area),
            super::types::ColorPickerMode::Hsl => self.render_hsl(ctx, content_area),
            super::types::ColorPickerMode::Hex => self.render_hex_mode(ctx, content_area),
        }

        // Render preview and hex if enabled
        if self.show_preview {
            self.render_preview(ctx, content_area);
        }
    }
}

impl ColorPicker {
    fn render_palette(&self, ctx: &mut RenderContext, area: Rect) {
        let colors = self.palette.colors();
        let (cols, _rows) = self.palette.grid_size();

        let mut x = area.x;
        let mut y = area.y;

        for (i, color) in colors.iter().enumerate() {
            if x + 2 > area.x + area.width {
                x = area.x;
                y += 1;
            }
            if y >= area.y + area.height.saturating_sub(2) {
                break;
            }

            let is_selected = i == self.palette_index;

            // Color block
            let ch = if is_selected { '█' } else { '▀' };
            let mut cell = Cell::new(ch);
            cell.fg = Some(*color);
            if is_selected {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(x, y, cell);

            let mut cell2 = Cell::new(if is_selected { '█' } else { '▄' });
            cell2.fg = Some(*color);
            ctx.buffer.set(x + 1, y, cell2);

            x += 2;
            if (i + 1) % cols == 0 {
                x = area.x;
                y += 1;
            }
        }
    }

    fn render_rgb(&self, ctx: &mut RenderContext, area: Rect) {
        let sliders = [
            ("R", self.r as f32 / 255.0, Color::RED),
            ("G", self.g as f32 / 255.0, Color::GREEN),
            ("B", self.b as f32 / 255.0, Color::BLUE),
        ];

        self.render_sliders(ctx, area, &sliders);
    }

    fn render_hsl(&self, ctx: &mut RenderContext, area: Rect) {
        let sliders = [
            ("H", self.h as f32 / 360.0, Color::MAGENTA),
            ("S", self.s as f32 / 100.0, Color::CYAN),
            ("L", self.l as f32 / 100.0, Color::YELLOW),
        ];

        self.render_sliders(ctx, area, &sliders);
    }

    fn render_sliders(&self, ctx: &mut RenderContext, area: Rect, sliders: &[(&str, f32, Color)]) {
        let slider_width = (area.width.saturating_sub(6)) as usize;

        for (i, (label, value, color)) in sliders.iter().enumerate() {
            let y = area.y + i as u16;
            if y >= area.y + area.height {
                break;
            }

            let is_active = i == self.active_slider;

            // Label
            let mut label_cell = Cell::new(label.chars().next().unwrap_or(' '));
            label_cell.fg = Some(if is_active { *color } else { Color::WHITE });
            if is_active {
                label_cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(area.x, y, label_cell);

            // Slider track
            let filled = (value * slider_width as f32) as usize;
            for j in 0..slider_width {
                let ch = if j < filled { '█' } else { '░' };
                let mut cell = Cell::new(ch);
                cell.fg = Some(if j < filled {
                    *color
                } else {
                    Color::rgb(60, 60, 60)
                });
                ctx.buffer.set(area.x + 2 + j as u16, y, cell);
            }

            // Value
            let val_str = match i {
                0 if self.mode == super::types::ColorPickerMode::Hsl => format!("{:3}", self.h),
                1 if self.mode == super::types::ColorPickerMode::Hsl => format!("{:3}", self.s),
                2 if self.mode == super::types::ColorPickerMode::Hsl => format!("{:3}", self.l),
                0 => format!("{:3}", self.r),
                1 => format!("{:3}", self.g),
                _ => format!("{:3}", self.b),
            };

            let val_x = area.x + 2 + slider_width as u16 + 1;
            for (j, ch) in val_str.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                ctx.buffer.set(val_x + j as u16, y, cell);
            }
        }
    }

    fn render_hex_mode(&self, ctx: &mut RenderContext, area: Rect) {
        // Label
        let label = "Hex: #";
        for (i, ch) in label.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            ctx.buffer.set(area.x + i as u16, area.y, cell);
        }

        // Input field
        let input_x = area.x + label.len() as u16;
        let input_len = self.hex_input.chars().count();
        // Iterate directly over chars for O(n) instead of O(n²) with .chars().nth(i) in loop
        for (i, ch) in self
            .hex_input
            .chars()
            .chain(std::iter::repeat('_'))
            .take(6)
            .enumerate()
        {
            let mut cell = Cell::new(ch);
            cell.fg = Some(if i < input_len {
                Color::CYAN
            } else {
                Color::rgb(60, 60, 60)
            });
            ctx.buffer.set(input_x + i as u16, area.y, cell);
        }

        // Current hex value
        let current = format!("Current: {}", self.hex_string());
        for (i, ch) in current.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(150, 150, 150));
            ctx.buffer.set(area.x + i as u16, area.y + 2, cell);
        }
    }

    fn render_preview(&self, ctx: &mut RenderContext, area: Rect) {
        let preview_y = area.y + area.height.saturating_sub(2);
        let preview_width = 6u16;

        // Preview block
        for dy in 0..2 {
            for dx in 0..preview_width {
                let mut cell = Cell::new('█');
                cell.fg = Some(self.color);
                ctx.buffer.set(area.x + dx, preview_y + dy, cell);
            }
        }

        // Hex value next to preview
        if self.show_hex {
            let hex = self.hex_string();
            for (i, ch) in hex.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                ctx.buffer
                    .set(area.x + preview_width + 1 + i as u16, preview_y, cell);
            }
        }
    }
}
