//! Candlestick chart rendering: price scaling, candle columns and the `View` impl

use super::{Candle, CandleChart, ChartStyle};
use crate::style::Color;
use crate::widget::theme::{
    DARK_BG, DISABLED_FG, EDITOR_BG, MUTED_TEXT, PLACEHOLDER_FG, SEPARATOR_COLOR,
};
use crate::widget::{RenderContext, View};

impl CandleChart {
    /// Get visible candles
    fn visible_candles(&self) -> &[Candle] {
        &self.data[self.visible_range(self.data.len())]
    }

    /// Index range of the visible candles in a series of `len` candles,
    /// empty when scrolled past the end
    fn visible_range(&self, len: usize) -> std::ops::Range<usize> {
        let start = self.offset.min(len);
        let end = start.saturating_add(self.width).min(len);
        start..end
    }

    /// Calculate price range
    fn get_price_range(&self) -> (f64, f64) {
        let candles = self.visible_candles();

        if candles.is_empty() {
            return (0.0, 100.0);
        }

        let min = self
            .min_price
            .unwrap_or_else(|| candles.iter().map(|c| c.low).fold(f64::INFINITY, f64::min));

        let max = self.max_price.unwrap_or_else(|| {
            candles
                .iter()
                .map(|c| c.high)
                .fold(f64::NEG_INFINITY, f64::max)
        });

        // Add padding
        let padding = (max - min) * 0.05;
        (min - padding, max + padding)
    }

    /// Map price to row
    fn price_to_row(&self, price: f64, min: f64, max: f64) -> usize {
        let range = max - min;
        if range == 0.0 {
            return self.height as usize / 2;
        }

        let normalized = (price - min) / range;
        let row = ((1.0 - normalized) * (self.height as f64 - 1.0)) as usize;
        row.min(self.height as usize - 1)
    }

    /// Render a single candle column
    fn render_candle(&self, candle: &Candle, min: f64, max: f64) -> Vec<(char, Color)> {
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

    /// Calculate Heikin-Ashi candles
    fn to_heikin_ashi(&self) -> Vec<Candle> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(self.data.len());
        let mut prev_ha: Option<Candle> = None;

        for candle in &self.data {
            let ha_close = (candle.open + candle.high + candle.low + candle.close) / 4.0;

            let ha_open = if let Some(prev) = prev_ha {
                (prev.open + prev.close) / 2.0
            } else {
                (candle.open + candle.close) / 2.0
            };

            let ha_high = candle.high.max(ha_open).max(ha_close);
            let ha_low = candle.low.min(ha_open).min(ha_close);

            let ha_candle = Candle {
                open: ha_open,
                high: ha_high,
                low: ha_low,
                close: ha_close,
                volume: candle.volume,
                timestamp: candle.timestamp,
            };

            prev_ha = Some(ha_candle);
            result.push(ha_candle);
        }

        result
    }
}

impl View for CandleChart {
    crate::impl_view_meta!("CandleChart");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::{hstack, vstack};
        use crate::widget::Text;

        let mut content = vstack();

        // Title and current price
        if let Some(title) = &self.title {
            let mut header = hstack();
            header = header.child(Text::new(title).bold());

            if let Some(price) = self.current_price() {
                header = header.child(Text::new(format!(
                    "  {:.prec$}",
                    price,
                    prec = self.precision
                )));

                if let Some((change, percent)) = self.price_change() {
                    let color = if change >= 0.0 {
                        self.bullish_color
                    } else {
                        self.bearish_color
                    };
                    let sign = if change >= 0.0 { "+" } else { "" };
                    header = header.child(
                        Text::new(format!(
                            "  {}{:.prec$} ({}{:.2}%)",
                            sign,
                            change,
                            sign,
                            percent,
                            prec = self.precision
                        ))
                        .fg(color),
                    );
                }
            }

            content = content.child(header);
        }

        // Get candles to render
        let candles = if self.style == ChartStyle::HeikinAshi {
            let ha = self.to_heikin_ashi();
            ha[self.visible_range(ha.len())].to_vec()
        } else {
            self.visible_candles().to_vec()
        };

        if candles.is_empty() {
            content = content.child(Text::new("No data").fg(PLACEHOLDER_FG));
            content.render(ctx);
            return;
        }

        let (min_price, max_price) = self.get_price_range();

        // Render candles
        let mut rows: Vec<Vec<(char, Color)>> = vec![Vec::new(); self.height as usize];

        for candle in &candles {
            let col = self.render_candle(candle, min_price, max_price);
            for (row_idx, (ch, color)) in col.into_iter().enumerate() {
                rows[row_idx].push((ch, color));
            }
        }

        // Price axis
        let axis_width = if self.show_axis { 10 } else { 0 };

        for (row_idx, row) in rows.iter().enumerate() {
            let mut line = hstack();

            // Price label
            if self.show_axis {
                let price = max_price
                    - (row_idx as f64 / (self.height as f64 - 1.0)) * (max_price - min_price);
                let label = if row_idx == 0
                    || row_idx == self.height as usize - 1
                    || row_idx == self.height as usize / 2
                {
                    format!("{:>8.prec$} ", price, prec = self.precision)
                } else {
                    " ".repeat(axis_width)
                };
                line = line.child(Text::new(label).fg(DISABLED_FG));
            }

            // Candle data
            for (ch, color) in row {
                line = line.child(Text::new(ch.to_string()).fg(*color));
            }

            content = content.child(line);
        }

        // Volume bars
        if self.show_volume {
            let max_vol = candles
                .iter()
                .filter_map(|c| c.volume)
                .fold(0.0f64, f64::max);

            if max_vol > 0.0 {
                content = content.child(Text::new("─".repeat(candles.len())).fg(SEPARATOR_COLOR));

                for row in 0..self.volume_height {
                    let mut vol_line = hstack();

                    if self.show_axis {
                        vol_line = vol_line.child(Text::new(" ".repeat(axis_width)));
                    }

                    for candle in &candles {
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

                    content = content.child(vol_line);
                }
            }
        }

        // Crosshair info
        if let Some(idx) = self.crosshair {
            if let Some(candle) = candles.get(idx) {
                let info = format!(
                    "O:{:.prec$} H:{:.prec$} L:{:.prec$} C:{:.prec$}{}",
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close,
                    candle
                        .volume
                        .map(|v| format!(" V:{:.0}", v))
                        .unwrap_or_default(),
                    prec = self.precision
                );
                content = content.child(Text::new(info).fg(MUTED_TEXT));
            }
        }

        content.render(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_range() {
        let data = vec![
            Candle::new(100.0, 110.0, 95.0, 105.0),
            Candle::new(105.0, 120.0, 100.0, 115.0),
        ];
        let chart = CandleChart::new(data);
        let (min, max) = chart.get_price_range();

        // Should include padding
        assert!(min < 95.0);
        assert!(max > 120.0);
    }

    #[test]
    fn test_heikin_ashi() {
        let data = vec![
            Candle::new(100.0, 110.0, 95.0, 105.0),
            Candle::new(105.0, 115.0, 100.0, 110.0),
        ];
        let chart = CandleChart::new(data).style(ChartStyle::HeikinAshi);
        let ha = chart.to_heikin_ashi();

        assert_eq!(ha.len(), 2);
        // First HA candle close should be average
        assert!((ha[0].close - 102.5).abs() < 0.001);
    }

    // =========================================================================
    // CandleChart::visible_candles tests
    // =========================================================================

    #[test]
    fn test_visible_candles_partial() {
        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
            Candle::new(107.0, 112.0, 106.0, 111.0),
        ];
        let chart = CandleChart::new(data).width(2).scroll(1);
        let visible = chart.visible_candles();
        assert_eq!(visible.len(), 2);
    }

    #[test]
    fn test_visible_candles_empty() {
        let chart = CandleChart::new(vec![]);
        let visible = chart.visible_candles();
        assert_eq!(visible.len(), 0);
    }

    fn three_candles() -> Vec<Candle> {
        vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
            Candle::new(107.0, 112.0, 106.0, 111.0),
        ]
    }

    fn render_chart(chart: &CandleChart) {
        use crate::layout::Rect;
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 20);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 20));
        chart.render(&mut ctx);
    }

    #[test]
    fn test_visible_candles_scrolled_past_end_is_empty() {
        let chart = CandleChart::new(three_candles()).scroll(10);
        assert!(chart.visible_candles().is_empty());

        let chart = CandleChart::new(three_candles()).scroll(usize::MAX);
        assert!(chart.visible_candles().is_empty());
    }

    #[test]
    fn test_render_zero_height() {
        render_chart(&CandleChart::new(three_candles()).height(0));
    }

    #[test]
    fn test_render_scrolled_past_end() {
        for style in [ChartStyle::Candle, ChartStyle::HeikinAshi] {
            render_chart(&CandleChart::new(three_candles()).style(style).scroll(10));
            render_chart(
                &CandleChart::new(three_candles())
                    .style(style)
                    .scroll(usize::MAX),
            );
        }
    }
}
