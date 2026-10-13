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

mod render;
mod types;

pub use types::{Candle, ChartStyle};

use crate::style::Color;
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

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
            wick_color: LIGHT_GRAY,
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
