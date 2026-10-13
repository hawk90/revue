//! Volume bars under the candles

use super::{Candle, CandleChart};
use crate::style::Color;
use crate::widget::theme::{EDITOR_BG, SEPARATOR_COLOR};
use crate::widget::{hstack, Stack, Text};

impl CandleChart {
    /// The separator and one line per volume row, or `None` when no
    /// candle has a volume above 0
    pub(super) fn volume_lines(&self, candles: &[Candle]) -> Option<(Text, Vec<Stack>)> {
        let max_vol = candles
            .iter()
            .filter_map(|c| c.volume)
            .fold(0.0f64, f64::max);

        if max_vol <= 0.0 {
            return None;
        }

        let separator = Text::new("─".repeat(candles.len())).fg(SEPARATOR_COLOR);
        let mut lines = Vec::new();

        for row in 0..self.volume_height {
            let mut vol_line = hstack();

            if self.show_axis {
                vol_line = vol_line.child(Text::new(" ".repeat(self.axis_width())));
            }

            for candle in candles {
                let vol = candle.volume.unwrap_or(0.0);
                let vol_height = ((vol / max_vol) * self.volume_height as f64) as u16;
                let threshold = self.volume_height - row - 1;

                let (ch, color) = if vol_height > threshold {
                    let color = if candle.is_bullish() {
                        Color::rgb(0, 100, 0)
                    } else {
                        Color::rgb(100, 0, 0)
                    };
                    ('█', color)
                } else {
                    (' ', EDITOR_BG)
                };

                vol_line = vol_line.child(Text::new(ch.to_string()).fg(color));
            }

            lines.push(vol_line);
        }
        Some((separator, lines))
    }
}
