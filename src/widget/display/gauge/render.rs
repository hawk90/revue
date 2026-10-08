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
        let crosses = |threshold: f64| {
            if self.thresholds_below {
                self.value <= threshold
            } else {
                self.value >= threshold
            }
        };
        if self.critical_threshold.is_some_and(crosses) {
            return self.critical_color;
        }
        if self.warning_threshold.is_some_and(crosses) {
            return self.warning_color;
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

    /// The columns and rows the gauge itself takes in an area of
    /// `avail_w` x `avail_h`, as each style draws it
    fn body_size(&self, avail_w: u16, avail_h: u16) -> (u16, u16) {
        let (w, h) = match self.style {
            GaugeStyle::Bar => (self.width.min(avail_w), 1),
            GaugeStyle::Battery => (self.width.min(avail_w).max(6), 1),
            GaugeStyle::Segments => (self.segments.min(avail_w / 2).saturating_mul(2), 1),
            GaugeStyle::Dots => (self.segments.min(avail_w), 1),
            GaugeStyle::Vertical => (1, self.height.min(avail_h)),
            GaugeStyle::Thermometer => (1, self.height.min(avail_h).max(3)),
            GaugeStyle::Arc => (self.width.min(avail_w).max(8), avail_h.min(3)),
            // `(` + five dots + `)`
            GaugeStyle::Circle => (7, 1),
        };
        (w.min(avail_w), h.min(avail_h))
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
        let outline = self.border_color.unwrap_or(Color::WHITE);
        let mut left = Cell::new('[');
        left.fg = Some(outline);
        ctx.set(0, 0, left);

        for x in 0..inner_width {
            let ch = if x < filled { '█' } else { ' ' };
            let fg = if x < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(1 + x, 0, cell);
        }

        let mut right = Cell::new(']');
        right.fg = Some(outline);
        ctx.set(1 + inner_width, 0, right);

        // Battery cap
        let mut cap = Cell::new('▌');
        cap.fg = Some(outline);
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

        // The arc has `width - 2` cells between its corners; fill as many as
        // the value covers, so 0 fills none and 1 fills all.
        let filled = (self.value * (width - 2) as f64).round() as u16;
        for x in 1..width - 1 {
            let is_filled = x - 1 < filled;
            let ch = if is_filled { '━' } else { '─' };
            let fg = if is_filled { color } else { self.empty_color };
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

            // Other positions are drawn by `render`
            if matches!(self.label_position, LabelPosition::Inside) {
                let mut dx: u16 = 0;
                for ch in label.chars() {
                    let cw = char_width(ch) as u16;
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(contrast_color(self.empty_bg.unwrap_or(Color::rgb(0, 0, 0))));
                    cell.modifier |= Modifier::BOLD;
                    ctx.set(label_x + dx, 1, cell);
                    dx += cw;
                }
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

        let outline = self.border_color.unwrap_or(Color::WHITE);
        let mut open = Cell::new('(');
        open.fg = Some(outline);
        ctx.set(0, 0, open);

        for i in 0..segments {
            let ch = if i < filled { '●' } else { '○' };
            let fg = if i < filled { color } else { self.empty_color };
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(1 + i, 0, cell);
        }

        let mut close = Cell::new(')');
        close.fg = Some(outline);
        ctx.set(1 + segments, 0, close);

        // Label (other positions are drawn by `render`)
        if !matches!(self.label_position, LabelPosition::Inside) {
            return;
        }
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
    /// same but at least 3. A label to the `Left` or `Right` adds its width
    /// and a space, one `Above` or `Below` a row. `Arc` and `Circle` fill
    /// what they are given.
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
        let label_w = display_width(&self.get_label()).min(u16::MAX as usize - 1) as u16;
        let (w, h) = match self.label_position {
            LabelPosition::Left | LabelPosition::Right => (w.saturating_add(label_w + 1), h),
            LabelPosition::Above | LabelPosition::Below => (w.max(label_w), h.saturating_add(1)),
            LabelPosition::None | LabelPosition::Inside => (w, h),
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

        let (mut x, mut y) = (0u16, y_offset);
        let (mut width, mut height) = (area.width, area.height.saturating_sub(y_offset));

        // A label outside the gauge takes its own columns or row
        let label = self.get_label();
        let label_w = display_width(&label).min(u16::MAX as usize - 1) as u16;
        let put_label = |ctx: &mut RenderContext, lx: u16, ly: u16| {
            if lx < area.width && ly < area.height {
                ctx.put_str_with(lx, ly, &label, area.width, |ch| {
                    Cell::new(ch).fg(Color::WHITE).bold()
                });
            }
        };
        match self.label_position {
            LabelPosition::Left => {
                put_label(ctx, 0, y);
                x = (label_w + 1).min(width);
                width -= x;
            }
            LabelPosition::Right => {
                width = width.saturating_sub(label_w + 1);
                let (body_w, _) = self.body_size(width, height);
                put_label(ctx, body_w + 1, y);
            }
            LabelPosition::Above => {
                put_label(ctx, 0, y);
                y += 1;
                height = height.saturating_sub(1);
            }
            LabelPosition::Below => {
                height = height.saturating_sub(1);
                let (_, body_h) = self.body_size(width, height);
                if height > 0 {
                    put_label(ctx, 0, y + body_h);
                }
            }
            LabelPosition::None | LabelPosition::Inside => {}
        }
        if width == 0 || height == 0 {
            return;
        }

        let adjusted_area = ctx.sub_area(x, y, width, height);

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

    fn render(g: &Gauge, w: u16, h: u16) -> Buffer {
        let mut buf = Buffer::new(w, h);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, w, h));
        g.render(&mut ctx);
        buf
    }

    #[test]
    fn battery_flags_a_low_charge_not_a_full_one() {
        // Cell 1 is the first fill cell inside `[`.
        let fill = |level: f64| {
            render(&super::super::battery(level), 12, 1)
                .get(1, 0)
                .unwrap()
                .fg
        };
        assert_eq!(
            fill(80.0),
            Some(Color::GREEN),
            "a full battery is not normal"
        );
        assert_eq!(
            fill(40.0),
            Some(Color::YELLOW),
            "a half-empty battery is not a warning"
        );
        assert_eq!(
            fill(10.0),
            Some(Color::RED),
            "an almost empty battery is not critical"
        );
    }

    #[test]
    fn arc_fills_nothing_at_zero_and_everything_at_one() {
        let top = |value: f64| -> String {
            let g = Gauge::new().style(GaugeStyle::Arc).width(12).value(value);
            let buf = render(&g, 12, 3);
            (1..11).map(|x| buf.get(x, 0).unwrap().symbol).collect()
        };
        assert_eq!(top(0.0), "──────────");
        assert_eq!(top(0.5), "━━━━━─────");
        assert_eq!(top(1.0), "━━━━━━━━━━");
    }

    fn rows(g: &Gauge, w: u16, h: u16) -> Vec<String> {
        let buf = render(g, w, h);
        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| buf.get(x, y).unwrap().symbol)
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect()
    }

    fn half(position: LabelPosition) -> Gauge {
        Gauge::new()
            .width(10)
            .percent(50.0)
            .label_position(position)
    }

    #[test]
    fn label_position_places_the_label_around_the_bar() {
        assert_eq!(rows(&half(LabelPosition::Inside), 20, 1), ["███50%░░░░"]);
        assert_eq!(rows(&half(LabelPosition::None), 20, 1), ["█████░░░░░"]);
        assert_eq!(rows(&half(LabelPosition::Left), 20, 1), ["50% █████░░░░░"]);
        assert_eq!(rows(&half(LabelPosition::Right), 20, 1), ["█████░░░░░ 50%"]);
        assert_eq!(
            rows(&half(LabelPosition::Above), 20, 2),
            ["50%", "█████░░░░░"]
        );
        assert_eq!(
            rows(&half(LabelPosition::Below), 20, 2),
            ["█████░░░░░", "50%"]
        );
    }

    #[test]
    fn label_position_outside_the_bar_is_measured() {
        assert_eq!(half(LabelPosition::Right).measure(80, 24), Some((14, 1)));
        assert_eq!(half(LabelPosition::Below).measure(80, 24), Some((10, 2)));
    }

    #[test]
    fn border_colors_the_battery_outline() {
        let g = super::super::battery(80.0).border(Color::BLUE);
        let buf = render(&g, 12, 1);
        assert_eq!(buf.get(0, 0).unwrap().fg, Some(Color::BLUE));
        assert_eq!(buf.get(11, 0).unwrap().fg, Some(Color::BLUE));
    }
}
