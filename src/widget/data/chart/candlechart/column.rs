//! One candle drawn as a column of cells, in each chart style

use super::{Candle, CandleChart, ChartStyle};
use crate::style::Color;
use crate::widget::theme::DARK_BG;

impl CandleChart {
    /// Render a single candle column
    pub(super) fn render_candle(&self, candle: &Candle, min: f64, max: f64) -> Vec<(char, Color)> {
        let mut column = vec![(' ', DARK_BG); self.height as usize];
        if self.height == 0 {
            return column;
        }

        let high_row = self.price_to_row(candle.high, min, max);
        let low_row = self.price_to_row(candle.low, min, max);
        let open_row = self.price_to_row(candle.open, min, max);
        let close_row = self.price_to_row(candle.close, min, max);

        let body_top = open_row.min(close_row);
        let body_bottom = open_row.max(close_row);

        let color = if candle.is_bullish() {
            self.bullish_color
        } else {
            self.bearish_color
        };

        match self.style {
            ChartStyle::Candle | ChartStyle::HeikinAshi => {
                // Upper wick
                for row in high_row..body_top {
                    column[row] = ('│', self.wick_color);
                }

                // Body
                for row in body_top..=body_bottom {
                    column[row] = ('█', color);
                }

                // Lower wick
                for row in (body_bottom + 1)..=low_row {
                    column[row] = ('│', self.wick_color);
                }
            }
            ChartStyle::Ohlc => {
                // Vertical line
                for row in high_row..=low_row {
                    column[row] = ('│', color);
                }
                // Open tick (left)
                if open_row < self.height as usize {
                    column[open_row] = ('├', color);
                }
                // Close tick (right)
                if close_row < self.height as usize {
                    column[close_row] = ('┤', color);
                }
            }
            ChartStyle::Hollow => {
                // Upper wick
                for row in high_row..body_top {
                    column[row] = ('│', self.wick_color);
                }

                // Body (hollow if bullish, filled if bearish)
                if candle.is_bullish() {
                    if body_top == body_bottom {
                        column[body_top] = ('─', color);
                    } else {
                        column[body_top] = ('┌', color);
                        column[body_bottom] = ('└', color);
                        for row in (body_top + 1)..body_bottom {
                            column[row] = ('│', color);
                        }
                    }
                } else {
                    for row in body_top..=body_bottom {
                        column[row] = ('█', color);
                    }
                }

                // Lower wick
                for row in (body_bottom + 1)..=low_row {
                    column[row] = ('│', self.wick_color);
                }
            }
        }

        column
    }
}
