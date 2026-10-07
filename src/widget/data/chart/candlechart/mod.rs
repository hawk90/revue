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

// KEEP HERE - Private implementation tests (all tests access private fields: open, high, low, close, data, etc.)

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_helper_functions() {
        let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];

        let cc = candle_chart(data.clone());
        assert_eq!(cc.style, ChartStyle::Candle);

        let oc = ohlc_chart(data);
        assert_eq!(oc.style, ChartStyle::Ohlc);
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
        let (change, _percent) = chart.price_change().unwrap();
        assert_eq!(change, -3.0);
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

    // Render tests - KEEP HERE - private render tests
}
