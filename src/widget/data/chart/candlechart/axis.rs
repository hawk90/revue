//! Price axis: its width and the labels on the top, middle and bottom rows

use super::CandleChart;

impl CandleChart {
    /// Columns the price axis takes (0 when hidden)
    pub(super) fn axis_width(&self) -> usize {
        if self.show_axis {
            10
        } else {
            0
        }
    }

    /// Label of chart row `row_idx`: the price on the top, middle and
    /// bottom rows, blank on the others
    pub(super) fn price_label(&self, row_idx: usize, min_price: f64, max_price: f64) -> String {
        let price =
            max_price - (row_idx as f64 / (self.height as f64 - 1.0)) * (max_price - min_price);
        if row_idx == 0
            || row_idx == self.height as usize - 1
            || row_idx == self.height as usize / 2
        {
            format!("{:>8.prec$} ", price, prec = self.precision)
        } else {
            " ".repeat(self.axis_width())
        }
    }
}
