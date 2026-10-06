//! DataGrid width calculation

use super::core::DataGrid;

impl DataGrid {
    /// Widths of the visible columns in display order, using user-set widths
    /// if available
    ///
    /// `column_widths` is indexed like `columns`; a column it does not cover
    /// gets width 0.
    pub(super) fn get_display_widths(&self, available: u16) -> Vec<u16> {
        if !self.column_widths.is_empty() {
            self.visible_columns_in_order()
                .iter()
                .map(|&(orig, _)| self.column_widths.get(orig).copied().unwrap_or(0))
                .collect()
        } else {
            self.calculate_widths(available)
        }
    }

    /// Widths of every column, indexed like `columns`: the displayed width for
    /// visible columns and the configured width for hidden ones
    pub(super) fn widths_by_column(&self, available: u16) -> Vec<u16> {
        let mut widths: Vec<u16> = self
            .columns
            .iter()
            .map(|c| if c.width > 0 { c.width } else { c.min_width })
            .collect();
        let display = self.get_display_widths(available);
        for (&(orig, _), w) in self.visible_columns_in_order().iter().zip(display) {
            widths[orig] = w;
        }
        widths
    }

    /// Calculate the widths of the visible columns, in display order
    pub(crate) fn calculate_widths(&self, available: u16) -> Vec<u16> {
        let visible_cols: Vec<_> = self
            .visible_columns_in_order()
            .into_iter()
            .map(|(_, c)| c)
            .collect();

        if visible_cols.is_empty() {
            return vec![];
        }

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let borders = visible_cols.len() as u16 + 1;
        let available = available.saturating_sub(row_num_width + borders);

        // Start with fixed or min widths
        let mut widths: Vec<u16> = visible_cols
            .iter()
            .map(|c| if c.width > 0 { c.width } else { c.min_width })
            .collect();

        let total: u16 = widths.iter().sum();

        if total < available {
            // Distribute extra space only to auto-width columns (width = 0)
            let extra = available - total;
            let auto_cols: Vec<_> = visible_cols
                .iter()
                .enumerate()
                .filter(|(_, c)| c.width == 0)
                .collect();

            if !auto_cols.is_empty() {
                let per_col = extra / auto_cols.len() as u16;
                for &(i, col) in &auto_cols {
                    let new_width = widths[i] + per_col;
                    widths[i] = new_width.min(col.max_width);
                }
            }
        }

        widths
    }
}
