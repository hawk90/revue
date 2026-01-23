//! Progress bar drawing methods for RenderContext

use super::types::ProgressBarConfig;
use crate::style::Color;

impl RenderContext<'_> {
    /// Draw a horizontal progress bar
    pub fn draw_progress_bar(&mut self, config: &ProgressBarConfig) {
        let progress = config.progress.clamp(0.0, 1.0);
        let filled = (config.width as f32 * progress).round() as u16;

        for i in 0..config.width {
            let ch = if i < filled {
                config.filled_char
            } else {
                config.empty_char
            };
            self.draw_char(config.x + i, config.y, ch, config.fg);
        }
    }

    /// Draw a progress bar with percentage label
    pub fn draw_progress_bar_labeled(
        &mut self,
        x: u16,
        y: u16,
        bar_width: u16,
        progress: f32,
        fg: Color,
    ) {
        let progress = progress.clamp(0.0, 1.0);
        let percent = (progress * 100.0).round() as u8;
        let label = format!("{:>3}%", percent);

        self.draw_text(x, y, &label, fg);
        let bar_x = x + 4;
        self.draw_char(bar_x, y, '[', fg);
        self.draw_progress_bar(&ProgressBarConfig {
            x: bar_x + 1,
            y,
            width: bar_width,
            progress,
            filled_char: '█',
            empty_char: '░',
            fg,
        });
        self.draw_char(bar_x + 1 + bar_width, y, ']', fg);
    }
}

use crate::widget::traits::render_context::RenderContext;
