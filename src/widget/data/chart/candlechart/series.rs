//! Which candles are drawn (the scrolled window, raw or Heikin-Ashi) and the
//! price figures read from the data

use super::{Candle, CandleChart, ChartStyle};

impl CandleChart {
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

    /// Get visible candles
    pub(super) fn visible_candles(&self) -> &[Candle] {
        &self.data[self.visible_range(self.data.len())]
    }

    /// Index range of the visible candles in a series of `len` candles,
    /// empty when scrolled past the end
    fn visible_range(&self, len: usize) -> std::ops::Range<usize> {
        let start = self.offset.min(len);
        let end = start.saturating_add(self.width).min(len);
        start..end
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

    /// The candles drawn: the visible window of the raw candles, or of
    /// their Heikin-Ashi form
    pub(super) fn drawn_candles(&self) -> Vec<Candle> {
        if self.style == ChartStyle::HeikinAshi {
            let ha = self.to_heikin_ashi();
            ha[self.visible_range(ha.len())].to_vec()
        } else {
            self.visible_candles().to_vec()
        }
    }
}
