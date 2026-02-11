//! DataGrid width calculation

use super::core::DataGrid;

impl DataGrid {
    /// Get display widths, using user-set widths if available
    pub(super) fn get_display_widths(&self, available: u16) -> Vec<u16> {
        if !self.column_widths.is_empty() {
            self.column_widths.clone()
        } else {
            self.calculate_widths(available)
        }
    }

    /// Calculate column widths
    pub(crate) fn calculate_widths(&self, available: u16) -> Vec<u16> {
        let visible_cols: Vec<_> = self.columns.iter().filter(|c| c.visible).collect();

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
