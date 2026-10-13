//! Candlestick chart `View`: the header, one line per chart row (price label
//! and candle cells), the volume bars and the crosshair line, stacked top to bottom

use super::{Candle, CandleChart};
use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, MUTED_TEXT, PLACEHOLDER_FG};
use crate::widget::{hstack, vstack, RenderContext, Stack, Text, View};

impl CandleChart {
    /// The title, the current price and the change from the previous close
    fn header(&self, title: &str) -> Stack {
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
        header
    }

    /// One line per chart row: the price label, then one cell per candle
    fn candle_lines(&self, candles: &[Candle], min_price: f64, max_price: f64) -> Vec<Stack> {
        let mut rows: Vec<Vec<(char, Color)>> = vec![Vec::new(); self.height as usize];

        for candle in candles {
            let col = self.render_candle(candle, min_price, max_price);
            for (row_idx, (ch, color)) in col.into_iter().enumerate() {
                rows[row_idx].push((ch, color));
            }
        }

        let mut lines = Vec::new();
        for (row_idx, row) in rows.iter().enumerate() {
            let mut line = hstack();

            // Price label
            if self.show_axis {
                let label = self.price_label(row_idx, min_price, max_price);
                line = line.child(Text::new(label).fg(DISABLED_FG));
            }

            // Candle data
            for (ch, color) in row {
                line = line.child(Text::new(ch.to_string()).fg(*color));
            }

            lines.push(line);
        }
        lines
    }

    /// The open, high, low, close (and volume) of the crosshair candle
    fn crosshair_line(&self, candles: &[Candle]) -> Option<Text> {
        let candle = candles.get(self.crosshair?)?;
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
        Some(Text::new(info).fg(MUTED_TEXT))
    }
}

impl View for CandleChart {
    crate::impl_view_meta!("CandleChart");

    fn render(&self, ctx: &mut RenderContext) {
        let mut content = vstack();

        // Title and current price
        if let Some(title) = &self.title {
            content = content.child(self.header(title));
        }

        let candles = self.drawn_candles();

        if candles.is_empty() {
            content = content.child(Text::new("No data").fg(PLACEHOLDER_FG));
            content.render(ctx);
            return;
        }

        let (min_price, max_price) = self.get_price_range(&candles);

        for line in self.candle_lines(&candles, min_price, max_price) {
            content = content.child(line);
        }

        // Volume bars
        if self.show_volume {
            if let Some((separator, lines)) = self.volume_lines(&candles) {
                content = content.child(separator);
                for line in lines {
                    content = content.child(line);
                }
            }
        }

        // Crosshair info
        if let Some(info) = self.crosshair_line(&candles) {
            content = content.child(info);
        }

        content.render(ctx);
    }
}
