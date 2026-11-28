//! DataGrid column widths

use crate::widget::data::datagrid::DataGrid;

impl DataGrid {
    /// Widths of the visible columns in display order, using user-set widths
    /// if available
    ///
    /// `column_widths` is indexed like `columns`; a column it does not cover
    /// gets width 0.
    pub(crate) fn get_display_widths(&self, available: u16) -> Vec<u16> {
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
    pub(crate) fn widths_by_column(&self, available: u16) -> Vec<u16> {
        let mut widths: Vec<u16> = self.columns.iter().map(|c| c.base_width()).collect();
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

        // Reserve the row-number gutter as drawn, each column's trailing
        // separator and the scrollbar column. Counted in usize: the column
        // count as u16 plus one overflows at 65535 columns and wraps at
        // 65536 (#1546); what is left is at most `available`.
        let reserved = usize::from(self.row_number_gutter_width()) + visible_cols.len() + 1;
        let available = usize::from(available).saturating_sub(reserved) as u16;

        // Start with the set widths (clamped into range) or the minimums
        let mut widths: Vec<u16> = visible_cols.iter().map(|c| c.base_width()).collect();

        let total = widths.iter().fold(0u16, |sum, w| sum.saturating_add(*w));

        // Share the free space among the auto-width columns (width = 0),
        // evenly, and pass what a capped column cannot take on to the
        // others until every one is capped or the space is used (#1594). A
        // remainder smaller than the number of growing columns stays free.
        let mut free = available.saturating_sub(total);
        let mut growing: Vec<usize> = visible_cols
            .iter()
            .enumerate()
            .filter(|&(i, c)| {
                c.width == 0 && c.clamp_width(widths[i].saturating_add(1)) > widths[i]
            })
            .map(|(i, _)| i)
            .collect();
        while free > 0 && !growing.is_empty() {
            let per_col = (usize::from(free) / growing.len()) as u16;
            if per_col == 0 {
                break;
            }
            growing.retain(|&i| {
                let grown = visible_cols[i].clamp_width(widths[i].saturating_add(per_col));
                free -= grown - widths[i];
                widths[i] = grown;
                // Still below its cap: it can take more next round
                visible_cols[i].clamp_width(grown.saturating_add(1)) > grown
            });
        }

        widths
    }
}
