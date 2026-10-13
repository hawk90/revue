//! Price scale: the padded price range of the drawn candles and the row of a price

use super::{Candle, CandleChart};

impl CandleChart {
    /// Calculate the price range of the candles drawn (the raw candles, or
    /// their Heikin-Ashi form)
    pub(super) fn get_price_range(&self, candles: &[Candle]) -> (f64, f64) {
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
    pub(super) fn price_to_row(&self, price: f64, min: f64, max: f64) -> usize {
        let range = max - min;
        if range == 0.0 {
            return self.height as usize / 2;
        }

        let normalized = (price - min) / range;
        let row = ((1.0 - normalized) * (self.height as f64 - 1.0)) as usize;
        row.min(self.height as usize - 1)
    }
}
