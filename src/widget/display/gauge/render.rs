//! Drawing the gauge in each style, with its threshold color and label

use super::{Gauge, GaugeStyle, LabelPosition};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::color::contrast_color;
use crate::utils::{char_width, display_width};
use crate::widget::traits::{RenderContext, View};

impl Gauge {
    /// Get current display color based on thresholds
    fn current_color(&self, ctx: &RenderContext) -> Color {
        if let Some(critical) = self.critical_threshold {
            if self.value >= critical {
                return self.critical_color;
            }
        }
        if let Some(warning) = self.warning_threshold {
            if self.value >= warning {
                return self.warning_color;
            }
        }
        // The normal fill takes `color`; the warning and critical thresholds
        // keep theirs - they are the reading, and a rule cannot address them
        // separately.
        self.fill_color
            .unwrap_or_else(|| ctx.css_color(Color::GREEN))
    }

    /// Get label text
    fn get_label(&self) -> String {
        if let Some(ref label) = self.label {
            label.clone()
        } else if self.show_percent {
            format!("{:.0}%", self.value * 100.0)
        } else {
            let display_value = self.min + self.value * (self.max - self.min);
            format!("{:.0}", display_value)
        }
    }

    /// Render bar style
    fn render_bar(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let width = self.width.min(area.width);
        let filled = (self.value * width as f64).round() as u16;
        let color = self.current_color(ctx);

        // Draw bar
        for x in 0..width {
            let is_filled = x < filled;
            let ch = if is_filled { '█' } else { '░' };
            let fg = if is_filled { color } else { self.empty_color };
            let bg = if is_filled {
                self.fill_bg.unwrap_or(color)
            } else {
                self.empty_bg.unwrap_or(self.empty_color)
            };

            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            ctx.set(x, 0, cell);
        }

        // Draw label inside
        if matches!(self.label_position, LabelPosition::Inside) {
            let label = self.get_label();
            let lw = display_width(&label) as u16;
            let label_x = (width.saturating_sub(lw)) / 2;
            let mut dx: u16 = 0;
            for ch in label.chars() {
                let cw = char_width(ch) as u16;
                let x = label_x + dx;
                if x + cw <= width {
                    let is_filled = x < filled;
                    let bg = if is_filled {
                        self.fill_bg.unwrap_or(color)
                    } else {
                        self.empty_bg.unwrap_or(self.empty_color)
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(contrast_color(bg));
                    cell.bg = Some(bg);
                    cell.modifier |= Modifier::BOLD;
                    ctx.set(x, 0, cell);
                }
                dx += cw;
            }
        }
    }

    /// Render battery style
    fn render_battery(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let width = self.width.min(area.width).max(6);
        let inner_width = width - 3; // Account for borders and cap
        let filled = (self.value * inner_width as f64).round() as u16;
        let color = self.current_color(ctx);

        // Battery body
        let mut left = Cell::new('[');
        left.fg = Some(Color::WHITE);
        ctx.set(0, 0, left);

        for x in 0..inner_width {
            let ch = if x < filled { '█' } else { ' ' };
            let fg = if x < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(1 + x, 0, cell);
        }

        let mut right = Cell::new(']');
        right.fg = Some(Color::WHITE);
        ctx.set(1 + inner_width, 0, right);

        // Battery cap
        let mut cap = Cell::new('▌');
        cap.fg = Some(Color::WHITE);
        ctx.set(2 + inner_width, 0, cap);
    }

    /// Render thermometer style
    fn render_thermometer(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.min(area.height).max(3);
        let filled = (self.value * (height - 1) as f64).round() as u16;
        let color = self.current_color(ctx);

        // Bulb at bottom
        let mut bulb = Cell::new('●');
        bulb.fg = Some(color);
        ctx.set(0, height - 1, bulb);

        // Tube
        for y in 0..height - 1 {
            let from_bottom = height - 2 - y;
            let ch = if from_bottom < filled { '█' } else { '│' };
            let fg = if from_bottom < filled {
                color
            } else {
                self.empty_color
            };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(0, y, cell);
        }
    }

    /// Render arc style
    fn render_arc(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let color = self.current_color(ctx);

        // Simple text-based arc: ╭───────╮
        //                        │ 75%   │
        //                        ╰───────╯
        let width = self.width.min(area.width).max(8);

        // Top arc
        let mut tl = Cell::new('╭');
        tl.fg = Some(color);
        ctx.set(0, 0, tl);

        for x in 1..width - 1 {
            let progress = (x - 1) as f64 / (width - 3) as f64;
            let ch = if progress <= self.value { '━' } else { '─' };
            let fg = if progress <= self.value {
                color
            } else {
                self.empty_color
            };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(x, 0, cell);
        }

        let mut tr = Cell::new('╮');
        tr.fg = Some(color);
        ctx.set(width - 1, 0, tr);

        // Middle with label
        if area.height > 1 {
            let label = self.get_label();
            let lw = display_width(&label) as u16;
            let label_x = (width.saturating_sub(lw)) / 2;

            let mut left = Cell::new('│');
            left.fg = Some(color);
            ctx.set(0, 1, left);

            let mut dx: u16 = 0;
            for ch in label.chars() {
                let cw = char_width(ch) as u16;
                let mut cell = Cell::new(ch);
                cell.fg = Some(contrast_color(self.empty_bg.unwrap_or(Color::rgb(0, 0, 0))));
                cell.modifier |= Modifier::BOLD;
                ctx.set(label_x + dx, 1, cell);
                dx += cw;
            }

            let mut right = Cell::new('│');
            right.fg = Some(color);
            ctx.set(width - 1, 1, right);
        }

        // Bottom arc
        if area.height > 2 {
            let mut bl = Cell::new('╰');
            bl.fg = Some(color);
            ctx.set(0, 2, bl);

            for x in 1..width - 1 {
                let mut cell = Cell::new('─');
                cell.fg = Some(self.empty_color);
                ctx.set(x, 2, cell);
            }

            let mut br = Cell::new('╯');
            br.fg = Some(color);
            ctx.set(width - 1, 2, br);
        }
    }

    /// Render circle style (text-based)
    fn render_circle(&self, ctx: &mut RenderContext) {
        let color = self.current_color(ctx);

        // Braille-based circle approximation
        // ⠀⢀⣴⣾⣿⣷⣦⡀⠀
        // ⠀⣿⣿⣿⣿⣿⣿⣿⠀
        // ⠀⠻⣿⣿⣿⣿⣿⠟⠀

        let label = self.get_label();

        // Simple representation: (●●●○○) 60%
        let segments = 5u16;
        let filled = (self.value * segments as f64).round() as u16;

        let mut open = Cell::new('(');
        open.fg = Some(Color::WHITE);
        ctx.set(0, 0, open);

        for i in 0..segments {
            let ch = if i < filled { '●' } else { '○' };
            let fg = if i < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(1 + i, 0, cell);
        }

        let mut close = Cell::new(')');
        close.fg = Some(Color::WHITE);
        ctx.set(1 + segments, 0, close);

        // Label
        let label_x = 3 + segments;
        let mut dx: u16 = 0;
        for ch in label.chars() {
            let cw = char_width(ch) as u16;
            let mut cell = Cell::new(ch);
            cell.fg = Some(contrast_color(self.empty_bg.unwrap_or(Color::rgb(0, 0, 0))));
            ctx.set(label_x + dx, 0, cell);
            dx += cw;
        }
    }

    /// Render vertical style
    fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.min(area.height);
        let filled = (self.value * height as f64).round() as u16;
        let color = self.current_color(ctx);

        for y in 0..height {
            let from_bottom = height - 1 - y;
            let ch = if from_bottom < filled { '█' } else { '░' };
            let fg = if from_bottom < filled {
                color
            } else {
                self.empty_color
            };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(0, y, cell);
        }
    }

    /// Render segments style
    fn render_segments(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let segments = self.segments.min(area.width / 2);
        let filled = (self.value * segments as f64).round() as u16;
        let color = self.current_color(ctx);

        for i in 0..segments {
            let ch = if i < filled { '▰' } else { '▱' };
            let fg = if i < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(i * 2, 0, cell);
        }
    }

    /// Render dots style
    fn render_dots(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let dots = self.segments.min(area.width);
        let filled = (self.value * dots as f64).round() as u16;
        let color = self.current_color(ctx);

        for i in 0..dots {
            let ch = if i < filled { '●' } else { '○' };
            let fg = if i < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(i, 0, cell);
        }
    }
}

impl View for Gauge {
    crate::impl_view_meta!("Gauge");

    /// The styles drawn at a set size answer with it, plus a row for the
    /// title: `Bar` is [`width`](Gauge::width) columns, `Battery` the same
    /// but at least 6, `Segments` two columns per segment, `Dots` one per
    /// dot, `Vertical` [`height`](Gauge::height) rows and `Thermometer` the
    /// same but at least 3. `Arc` and `Circle` fill what they are given.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let (w, h) = match self.style {
            GaugeStyle::Bar => (self.width, 1),
            GaugeStyle::Battery => (self.width.max(6), 1),
            GaugeStyle::Segments => (self.segments.saturating_mul(2), 1),
            GaugeStyle::Dots => (self.segments, 1),
            GaugeStyle::Vertical => (1, self.height),
            GaugeStyle::Thermometer => (1, self.height.max(3)),
            GaugeStyle::Arc | GaugeStyle::Circle => return None,
        };
        let (w, h) = match &self.title {
            Some(title) => (
                w.max(display_width(title).min(u16::MAX as usize) as u16),
                h.saturating_add(1),
            ),
            None => (w, h),
        };
        Some((w.min(max_width), h.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Draw title if present
        let mut y_offset = 0u16;
        if let Some(ref title) = self.title {
            ctx.put_str_with(0, 0, title, area.width, |ch| {
                Cell::new(ch).fg(Color::WHITE).bold()
            });
            y_offset = 1;
        }

        let adjusted_area = ctx.sub_area(
            0,
            y_offset,
            area.width,
            area.height.saturating_sub(y_offset),
        );

        let mut adjusted_ctx = ctx.sub_ctx(adjusted_area);

        match self.style {
            GaugeStyle::Bar => self.render_bar(&mut adjusted_ctx),
            GaugeStyle::Battery => self.render_battery(&mut adjusted_ctx),
            GaugeStyle::Thermometer => self.render_thermometer(&mut adjusted_ctx),
            GaugeStyle::Arc => self.render_arc(&mut adjusted_ctx),
            GaugeStyle::Circle => self.render_circle(&mut adjusted_ctx),
            GaugeStyle::Vertical => self.render_vertical(&mut adjusted_ctx),
            GaugeStyle::Segments => self.render_segments(&mut adjusted_ctx),
            GaugeStyle::Dots => self.render_dots(&mut adjusted_ctx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_gauge_render_no_panic() {
        let mut buf = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 3);
        let mut ctx = RenderContext::new(&mut buf, area);
        let g = Gauge::new().value(0.5).style(GaugeStyle::Bar);
        g.render(&mut ctx);
    }
}
