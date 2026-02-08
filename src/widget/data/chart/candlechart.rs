#![allow(clippy::needless_range_loop)]
//! Candlestick/OHLC chart widget
//!
//! Financial charting for stock prices with OHLC (Open, High, Low, Close) data.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{CandleChart, Candle, candle_chart};
//!
//! let data = vec![
//!     Candle::new(100.0, 105.0, 98.0, 103.0),
//!     Candle::new(103.0, 108.0, 102.0, 107.0),
//!     Candle::new(107.0, 110.0, 104.0, 105.0),
//! ];
//!
//! let chart = CandleChart::new(data)
//!     .title("AAPL")
//!     .show_volume(true);
//! ```

use crate::style::Color;
use crate::widget::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Single candlestick data point
#[derive(Clone, Copy, Debug)]
pub struct Candle {
    /// Opening price
    pub open: f64,
    /// Highest price
    pub high: f64,
    /// Lowest price
    pub low: f64,
    /// Closing price
    pub close: f64,
    /// Volume (optional)
    pub volume: Option<f64>,
    /// Timestamp (optional, for display)
    pub timestamp: Option<i64>,
}

impl Candle {
    /// Create new candle
    pub fn new(open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume: None,
            timestamp: None,
        }
    }

    /// Create with volume
    pub fn with_volume(open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume: Some(volume),
            timestamp: None,
        }
    }

    /// Set timestamp
    pub fn timestamp(mut self, ts: i64) -> Self {
        self.timestamp = Some(ts);
        self
    }

    /// Check if bullish (close > open)
    pub fn is_bullish(&self) -> bool {
        self.close >= self.open
    }

    /// Get body size
    pub fn body_size(&self) -> f64 {
        (self.close - self.open).abs()
    }

    /// Get upper shadow size
    pub fn upper_shadow(&self) -> f64 {
        self.high - self.open.max(self.close)
    }

    /// Get lower shadow size
    pub fn lower_shadow(&self) -> f64 {
        self.open.min(self.close) - self.low
    }

    /// Get range (high - low)
    pub fn range(&self) -> f64 {
        self.high - self.low
    }
}

/// Chart display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartStyle {
    /// Japanese candlesticks
    #[default]
    Candle,
    /// OHLC bars
    Ohlc,
    /// Hollow candles
    Hollow,
    /// Heikin-Ashi
    HeikinAshi,
}

/// Candlestick chart widget
#[derive(Clone, Debug)]
pub struct CandleChart {
    /// Candlestick data
    data: Vec<Candle>,
    /// Chart style
    style: ChartStyle,
    /// Chart height in rows
    height: u16,
    /// Chart width (number of candles)
    width: usize,
    /// Bullish color
    bullish_color: Color,
    /// Bearish color
    bearish_color: Color,
    /// Wick color
    wick_color: Color,
    /// Show volume bars
    show_volume: bool,
    /// Volume bar height
    volume_height: u16,
    /// Show price axis
    show_axis: bool,
    /// Show grid
    show_grid: bool,
    /// Title
    title: Option<String>,
    /// Show crosshair at index
    crosshair: Option<usize>,
    /// Price precision (decimal places)
    precision: usize,
    /// Min price (auto if None)
    min_price: Option<f64>,
    /// Max price (auto if None)
    max_price: Option<f64>,
    /// Scroll offset
    offset: usize,
    /// Widget properties
    props: WidgetProps,
}

impl CandleChart {
    /// Create new candle chart
    pub fn new(data: Vec<Candle>) -> Self {
        Self {
            data,
            style: ChartStyle::default(),
            height: 15,
            width: 40,
            bullish_color: Color::GREEN,
            bearish_color: Color::RED,
            wick_color: Color::rgb(150, 150, 150),
            show_volume: false,
            volume_height: 4,
            show_axis: true,
            show_grid: false,
            title: None,
            crosshair: None,
            precision: 2,
            min_price: None,
            max_price: None,
            offset: 0,
            props: WidgetProps::new(),
        }
    }

    /// Set chart style
    pub fn style(mut self, style: ChartStyle) -> Self {
        self.style = style;
        self
    }

    /// Set chart dimensions
    pub fn size(mut self, width: usize, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set width (number of candles)
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Set bullish color
    pub fn bullish_color(mut self, color: Color) -> Self {
        self.bullish_color = color;
        self
    }

    /// Set bearish color
    pub fn bearish_color(mut self, color: Color) -> Self {
        self.bearish_color = color;
        self
    }

    /// Show volume bars
    pub fn show_volume(mut self, show: bool) -> Self {
        self.show_volume = show;
        self
    }

    /// Show price axis
    pub fn show_axis(mut self, show: bool) -> Self {
        self.show_axis = show;
        self
    }

    /// Show grid
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set crosshair position
    pub fn crosshair(mut self, index: usize) -> Self {
        self.crosshair = Some(index);
        self
    }

    /// Set price precision
    pub fn precision(mut self, decimals: usize) -> Self {
        self.precision = decimals;
        self
    }

    /// Set price range
    pub fn price_range(mut self, min: f64, max: f64) -> Self {
        self.min_price = Some(min);
        self.max_price = Some(max);
        self
    }

    /// Scroll to offset
    pub fn scroll(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Get visible candles
    fn visible_candles(&self) -> &[Candle] {
        let start = self.offset;
        let end = (start + self.width).min(self.data.len());
        &self.data[start..end]
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
        let mut column = vec![(' ', Color::rgb(40, 40, 40)); self.height as usize];

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

    /// Get current price (last close)
    pub fn current_price(&self) -> Option<f64> {
        self.data.last().map(|c| c.close)
    }

    /// Get price change
    pub fn price_change(&self) -> Option<(f64, f64)> {
        if self.data.len() < 2 {
            return None;
        }

        let prev = self.data[self.data.len() - 2].close;
        let curr = self.data.last()?.close;
        let change = curr - prev;
        let percent = (change / prev) * 100.0;

        Some((change, percent))
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
            let start = self.offset;
            let end = (start + self.width).min(ha.len());
            ha[start..end].to_vec()
        } else {
            self.visible_candles().to_vec()
        };

        if candles.is_empty() {
            content = content.child(Text::new("No data").fg(Color::rgb(128, 128, 128)));
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
                line = line.child(Text::new(label).fg(Color::rgb(100, 100, 100)));
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
                content =
                    content.child(Text::new("─".repeat(candles.len())).fg(Color::rgb(60, 60, 60)));

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
                            (' ', Color::rgb(30, 30, 30))
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
                content = content.child(Text::new(info).fg(Color::rgb(180, 180, 180)));
            }
        }

        content.render(ctx);
    }
}

impl_styled_view!(CandleChart);
impl_props_builders!(CandleChart);

/// Create a candle chart
pub fn candle_chart(data: Vec<Candle>) -> CandleChart {
    CandleChart::new(data)
}

/// Create an OHLC chart
pub fn ohlc_chart(data: Vec<Candle>) -> CandleChart {
    CandleChart::new(data).style(ChartStyle::Ohlc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candle_new() {
        let candle = Candle::new(100.0, 110.0, 95.0, 105.0);
        assert_eq!(candle.open, 100.0);
        assert_eq!(candle.high, 110.0);
        assert_eq!(candle.low, 95.0);
        assert_eq!(candle.close, 105.0);
    }

    #[test]
    fn test_candle_bullish() {
        let bullish = Candle::new(100.0, 110.0, 95.0, 108.0);
        assert!(bullish.is_bullish());

        let bearish = Candle::new(100.0, 110.0, 95.0, 92.0);
        assert!(!bearish.is_bullish());
    }

    #[test]
    fn test_candle_metrics() {
        let candle = Candle::new(100.0, 115.0, 90.0, 110.0);
        assert_eq!(candle.body_size(), 10.0);
        assert_eq!(candle.upper_shadow(), 5.0);
        assert_eq!(candle.lower_shadow(), 10.0);
        assert_eq!(candle.range(), 25.0);
    }

    #[test]
    fn test_chart_new() {
        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
        ];
        let chart = CandleChart::new(data);
        assert_eq!(chart.data.len(), 2);
    }

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
    fn test_price_change() {
        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 100.0),
            Candle::new(100.0, 110.0, 95.0, 110.0),
        ];
        let chart = CandleChart::new(data);

        let (change, percent) = chart.price_change().unwrap();
        assert_eq!(change, 10.0);
        assert!((percent - 10.0).abs() < 0.001);
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

    #[test]
    fn test_helper_functions() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];

        let cc = candle_chart(data.clone());
        assert_eq!(cc.style, ChartStyle::Candle);

        let oc = ohlc_chart(data);
        assert_eq!(oc.style, ChartStyle::Ohlc);
    }

    // =========================================================================
    // Candle::with_volume tests
    // =========================================================================

    #[test]
    fn test_candle_with_volume() {
        let candle = Candle::with_volume(100.0, 110.0, 95.0, 105.0, 5000.0);
        assert_eq!(candle.volume, Some(5000.0));
        assert_eq!(candle.open, 100.0);
    }

    #[test]
    fn test_candle_with_volume_none() {
        let candle = Candle::new(100.0, 110.0, 95.0, 105.0);
        assert!(candle.volume.is_none());
    }

    // =========================================================================
    // Candle::timestamp tests
    // =========================================================================

    #[test]
    fn test_candle_timestamp() {
        let candle = Candle::new(100.0, 110.0, 95.0, 105.0).timestamp(1234567890);
        assert_eq!(candle.timestamp, Some(1234567890));
    }

    #[test]
    fn test_candle_timestamp_none() {
        let candle = Candle::new(100.0, 110.0, 95.0, 105.0);
        assert!(candle.timestamp.is_none());
    }

    // =========================================================================
    // Candle::is_bullish edge cases
    // =========================================================================

    #[test]
    fn test_candle_is_bullish_equal() {
        let candle = Candle::new(100.0, 110.0, 95.0, 100.0);
        // close >= open means bullish
        assert!(candle.is_bullish());
    }

    // =========================================================================
    // Candle::body_size tests
    // =========================================================================

    #[test]
    fn test_candle_body_size_zero() {
        let candle = Candle::new(100.0, 110.0, 95.0, 100.0);
        assert_eq!(candle.body_size(), 0.0);
    }

    #[test]
    fn test_candle_body_size_bearish() {
        let candle = Candle::new(100.0, 110.0, 90.0, 95.0);
        assert_eq!(candle.body_size(), 5.0);
    }

    // =========================================================================
    // Candle::upper_shadow tests
    // =========================================================================

    #[test]
    fn test_candle_upper_shadow_zero() {
        let candle = Candle::new(100.0, 105.0, 95.0, 105.0);
        // high - max(open, close) = 105 - max(100, 105) = 105 - 105 = 0
        assert_eq!(candle.upper_shadow(), 0.0);
    }

    #[test]
    fn test_candle_upper_shadow_bearish() {
        let candle = Candle::new(100.0, 120.0, 90.0, 95.0);
        // high - max(open, close) = 120 - 100 = 20
        assert_eq!(candle.upper_shadow(), 20.0);
    }

    // =========================================================================
    // Candle::lower_shadow tests
    // =========================================================================

    #[test]
    fn test_candle_lower_shadow_zero() {
        let candle = Candle::new(100.0, 110.0, 100.0, 105.0);
        assert_eq!(candle.lower_shadow(), 0.0);
    }

    #[test]
    fn test_candle_lower_shadow_bullish() {
        let candle = Candle::new(100.0, 110.0, 80.0, 105.0);
        // min(open, close) - low = 100 - 80 = 20
        assert_eq!(candle.lower_shadow(), 20.0);
    }

    // =========================================================================
    // Candle::range tests
    // =========================================================================

    #[test]
    fn test_candle_range_zero() {
        let candle = Candle::new(100.0, 100.0, 100.0, 100.0);
        assert_eq!(candle.range(), 0.0);
    }

    // =========================================================================
    // Candle Clone trait
    // =========================================================================

    #[test]
    fn test_candle_clone() {
        let candle1 = Candle::new(100.0, 110.0, 95.0, 105.0).timestamp(123);
        let candle2 = candle1.clone();
        assert_eq!(candle1.open, candle2.open);
        assert_eq!(candle1.timestamp, candle2.timestamp);
    }

    // =========================================================================
    // Candle Copy trait (since all fields implement Copy)
    // =========================================================================

    #[test]
    fn test_candle_copy() {
        let candle1 = Candle::new(100.0, 110.0, 95.0, 105.0);
        let candle2 = candle1;
        assert_eq!(candle2.open, 100.0);
    }

    // =========================================================================
    // ChartStyle enum tests
    // =========================================================================

    #[test]
    fn test_chart_style_default() {
        assert_eq!(ChartStyle::default(), ChartStyle::Candle);
    }

    #[test]
    fn test_chart_style_clone() {
        let style1 = ChartStyle::Ohlc;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_chart_style_copy() {
        let style1 = ChartStyle::Hollow;
        let style2 = style1;
        assert_eq!(style2, ChartStyle::Hollow);
    }

    #[test]
    fn test_chart_style_partial_eq() {
        assert_eq!(ChartStyle::Candle, ChartStyle::Candle);
        assert_eq!(ChartStyle::Ohlc, ChartStyle::Ohlc);
        assert_ne!(ChartStyle::Candle, ChartStyle::HeikinAshi);
    }

    #[test]
    fn test_chart_style_debug() {
        let debug_str = format!("{:?}", ChartStyle::Candle);
        assert!(debug_str.contains("Candle"));
    }

    // =========================================================================
    // CandleChart::style tests
    // =========================================================================

    #[test]
    fn test_chart_style_ohlc() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).style(ChartStyle::Ohlc);
        assert_eq!(chart.style, ChartStyle::Ohlc);
    }

    #[test]
    fn test_chart_style_hollow() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).style(ChartStyle::Hollow);
        assert_eq!(chart.style, ChartStyle::Hollow);
    }

    #[test]
    fn test_chart_style_heikin_ashi() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).style(ChartStyle::HeikinAshi);
        assert_eq!(chart.style, ChartStyle::HeikinAshi);
    }

    // =========================================================================
    // CandleChart::size tests
    // =========================================================================

    #[test]
    fn test_chart_size() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).size(30, 12);
        assert_eq!(chart.width, 30);
        assert_eq!(chart.height, 12);
    }

    // =========================================================================
    // CandleChart::height tests
    // =========================================================================

    #[test]
    fn test_chart_height() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).height(20);
        assert_eq!(chart.height, 20);
    }

    // =========================================================================
    // CandleChart::width tests
    // =========================================================================

    #[test]
    fn test_chart_width() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).width(50);
        assert_eq!(chart.width, 50);
    }

    // =========================================================================
    // CandleChart::bullish_color tests
    // =========================================================================

    #[test]
    fn test_chart_bullish_color() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).bullish_color(Color::rgb(0, 255, 0));
        assert_eq!(chart.bullish_color, Color::rgb(0, 255, 0));
    }

    // =========================================================================
    // CandleChart::bearish_color tests
    // =========================================================================

    #[test]
    fn test_chart_bearish_color() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).bearish_color(Color::rgb(255, 0, 0));
        assert_eq!(chart.bearish_color, Color::rgb(255, 0, 0));
    }

    // =========================================================================
    // CandleChart::show_volume tests
    // =========================================================================

    #[test]
    fn test_chart_show_volume_true() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).show_volume(true);
        assert!(chart.show_volume);
    }

    #[test]
    fn test_chart_show_volume_false() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).show_volume(false);
        assert!(!chart.show_volume);
    }

    // =========================================================================
    // CandleChart::show_axis tests
    // =========================================================================

    #[test]
    fn test_chart_show_axis_true() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).show_axis(true);
        assert!(chart.show_axis);
    }

    #[test]
    fn test_chart_show_axis_false() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).show_axis(false);
        assert!(!chart.show_axis);
    }

    // =========================================================================
    // CandleChart::show_grid tests
    // =========================================================================

    #[test]
    fn test_chart_show_grid_true() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).show_grid(true);
        assert!(chart.show_grid);
    }

    // =========================================================================
    // CandleChart::title tests
    // =========================================================================

    #[test]
    fn test_chart_title_str() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).title("AAPL");
        assert_eq!(chart.title, Some("AAPL".to_string()));
    }

    #[test]
    fn test_chart_title_string() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).title(String::from("GOOGL"));
        assert_eq!(chart.title, Some("GOOGL".to_string()));
    }

    // =========================================================================
    // CandleChart::crosshair tests
    // =========================================================================

    #[test]
    fn test_chart_crosshair() {
        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
        ];
        let chart = CandleChart::new(data).crosshair(1);
        assert_eq!(chart.crosshair, Some(1));
    }

    // =========================================================================
    // CandleChart::precision tests
    // =========================================================================

    #[test]
    fn test_chart_precision() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).precision(4);
        assert_eq!(chart.precision, 4);
    }

    // =========================================================================
    // CandleChart::price_range tests
    // =========================================================================

    #[test]
    fn test_chart_price_range_method() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data).price_range(90.0, 120.0);
        assert_eq!(chart.min_price, Some(90.0));
        assert_eq!(chart.max_price, Some(120.0));
    }

    // =========================================================================
    // CandleChart::scroll tests
    // =========================================================================

    #[test]
    fn test_chart_scroll() {
        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
            Candle::new(107.0, 112.0, 106.0, 111.0),
        ];
        let chart = CandleChart::new(data).scroll(1);
        assert_eq!(chart.offset, 1);
    }

    // =========================================================================
    // CandleChart::current_price tests
    // =========================================================================

    #[test]
    fn test_current_price_some() {
        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
        ];
        let chart = CandleChart::new(data);
        assert_eq!(chart.current_price(), Some(107.0));
    }

    #[test]
    fn test_current_price_none() {
        let chart = CandleChart::new(vec![]);
        assert_eq!(chart.current_price(), None);
    }

    // =========================================================================
    // CandleChart::price_change tests
    // =========================================================================

    #[test]
    fn test_price_change_single_candle() {
        let data = vec![Candle::new(100.0, 105.0, 98.0, 103.0)];
        let chart = CandleChart::new(data);
        assert_eq!(chart.price_change(), None);
    }

    #[test]
    fn test_price_change_negative() {
        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 100.0),
        ];
        let chart = CandleChart::new(data);
        let (change, percent) = chart.price_change().unwrap();
        assert_eq!(change, -3.0);
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

    // =========================================================================
    // CandleChart Clone trait
    // =========================================================================

    #[test]
    fn test_chart_clone() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart1 = CandleChart::new(data).title("Test");
        let chart2 = chart1.clone();
        assert_eq!(chart1.title, chart2.title);
    }

    // =========================================================================
    // CandleChart Debug trait
    // =========================================================================

    #[test]
    fn test_chart_debug() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
        let chart = CandleChart::new(data);
        let debug_str = format!("{:?}", chart);
        assert!(debug_str.contains("CandleChart"));
    }

    // =========================================================================
    // CandleChart builder chain test
    // =========================================================================

    #[test]
    fn test_chart_builder_chain() {
        let data = vec![
            Candle::with_volume(100.0, 105.0, 98.0, 103.0, 1000.0),
            Candle::with_volume(103.0, 108.0, 102.0, 107.0, 1500.0),
        ];
        let chart = CandleChart::new(data)
            .title("Stock Chart")
            .style(ChartStyle::Ohlc)
            .size(40, 15)
            .bullish_color(Color::rgb(0, 200, 0))
            .bearish_color(Color::rgb(200, 0, 0))
            .show_volume(true)
            .show_axis(true)
            .show_grid(true)
            .crosshair(0)
            .precision(3)
            .price_range(95.0, 115.0)
            .scroll(0);

        assert_eq!(chart.title, Some("Stock Chart".to_string()));
        assert_eq!(chart.style, ChartStyle::Ohlc);
        assert_eq!(chart.width, 40);
        assert_eq!(chart.height, 15);
        assert!(chart.show_volume);
        assert!(chart.show_axis);
        assert!(chart.show_grid);
        assert_eq!(chart.crosshair, Some(0));
        assert_eq!(chart.precision, 3);
        assert_eq!(chart.min_price, Some(95.0));
        assert_eq!(chart.max_price, Some(115.0));
        assert_eq!(chart.offset, 0);
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_candlechart_render_basic() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
            Candle::new(107.0, 112.0, 106.0, 111.0),
        ];

        let chart = CandleChart::new(data);
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);
        // Should render without panic
    }

    #[test]
    fn test_candlechart_render_with_title() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![Candle::new(100.0, 105.0, 98.0, 103.0)];
        let chart = CandleChart::new(data).title("AAPL");

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);

        // Title should be rendered
        let mut title_found = false;
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'A' {
                    title_found = true;
                    break;
                }
            }
        }
        assert!(title_found);
    }

    #[test]
    fn test_candlechart_render_with_volume() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![
            Candle::with_volume(100.0, 105.0, 98.0, 103.0, 1000.0),
            Candle::with_volume(103.0, 108.0, 102.0, 107.0, 1500.0),
        ];
        let chart = CandleChart::new(data).show_volume(true);

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);
        // Should render volume bars
    }

    #[test]
    fn test_candlechart_render_empty() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let chart = CandleChart::new(vec![]);

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);
        // Should handle empty data gracefully
    }

    #[test]
    fn test_candlechart_render_ohlc() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
        ];
        let chart = CandleChart::new(data).style(ChartStyle::Ohlc);

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);
        // Should render OHLC bars
    }

    #[test]
    fn test_candlechart_render_hollow() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
        ];
        let chart = CandleChart::new(data).style(ChartStyle::Hollow);

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);
        // Should render hollow candles
    }

    #[test]
    fn test_candlechart_render_heikin_ashi() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![
            Candle::new(100.0, 105.0, 98.0, 103.0),
            Candle::new(103.0, 108.0, 102.0, 107.0),
        ];
        let chart = CandleChart::new(data).style(ChartStyle::HeikinAshi);

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);
        // Should render Heikin-Ashi candles
    }
}
