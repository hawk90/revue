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
