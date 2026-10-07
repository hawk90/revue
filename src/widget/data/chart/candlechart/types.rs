//! Candlestick data point and chart style types

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
        let candle2 = candle1;
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
        let style2 = style1;
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
}
