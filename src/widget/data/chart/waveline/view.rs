//! Waveline `View`: background, label, the data window and the baseline, then the wave

use super::{WaveStyle, Waveline};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl View for Waveline {
    /// With a set height, that height across the offered width. Without one
    /// the wave scales to the area it is given, so it fills.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        self.height.map(|h| (max_width, h.min(max_height)))
    }

    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::WIDTH
    }
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.unwrap_or(area.height);

        if area.width < 2 || height < 1 {
            return;
        }

        let mut chart_y = 0u16;
        let mut chart_height = height.min(area.height);

        // Background
        if let Some(bg) = self.bg_color {
            for y in 0..chart_height {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.set(x, y, cell);
                }
            }
        }

        // The waveform's own colors are computed per sample from the amplitude,
        // so there is no single color for a rule to set - the same reason
        // `GradientBox` is left alone. The label is ordinary text.
        let chrome = ctx.css_color(Color::WHITE);

        // Label
        if let Some(ref label) = self.label {
            let bg = self.bg_color;
            ctx.put_str_with(0, chart_y, label, u16::MAX, |ch| {
                let mut cell = Cell::new(ch);
                cell.fg = Some(chrome);
                cell.bg = bg;
                cell
            });
            chart_y += 1;
            chart_height = chart_height.saturating_sub(1);
        }

        if chart_height < 1 || self.data.is_empty() {
            return;
        }

        // Determine data range
        let data = if let Some(max) = self.max_points {
            if self.data.len() > max {
                &self.data[self.data.len() - max..]
            } else {
                &self.data[..]
            }
        } else {
            &self.data[..]
        };

        // Draw baseline
        if self.show_baseline {
            let baseline_row = ((1.0 - self.baseline) * (chart_height - 1) as f64) as u16;
            let y = chart_y + baseline_row;
            for x in 0..area.width {
                let mut cell = Cell::new('─');
                cell.bg = self.bg_color;
                cell.fg = Some(self.baseline_color);
                ctx.set(x, y, cell);
            }
        }

        match self.style {
            WaveStyle::Line | WaveStyle::Smooth => self.draw_line(ctx, data, chart_y, chart_height),
            WaveStyle::Filled => self.draw_filled(ctx, data, chart_y, chart_height),
            WaveStyle::Mirrored => self.draw_mirrored(ctx, data, chart_y, chart_height),
            WaveStyle::Bars => self.draw_bars(ctx, data, chart_y, chart_height),
            WaveStyle::Dots => self.draw_dots(ctx, data, chart_y, chart_height),
        }
    }

    crate::impl_view_meta!("Waveline");
}
