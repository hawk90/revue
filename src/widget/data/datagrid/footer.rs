//! DataGrid aggregation footer functionality

use super::core::DataGrid;
use super::types::{AggregationType, ColumnAggregation, FooterRow};

impl DataGrid {
    /// Add a footer row
    pub fn footer(mut self, row: FooterRow) -> Self {
        self.footer_rows.push(row);
        self.show_footer = true;
        self
    }

    /// Show/hide footer
    pub fn show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    /// Add a quick sum aggregation
    pub fn add_sum(mut self, column_key: impl Into<String>) -> Self {
        let key = column_key.into();
        if self.footer_rows.is_empty() {
            self.footer_rows.push(FooterRow::new("Total"));
        }
        if let Some(footer) = self.footer_rows.first_mut() {
            footer
                .aggregations
                .push(ColumnAggregation::new(key, AggregationType::Sum));
        }
        self.show_footer = true;
        self
    }

    /// Add a quick average aggregation
    pub fn add_average(mut self, column_key: impl Into<String>) -> Self {
        let key = column_key.into();
        if self.footer_rows.is_empty() {
            self.footer_rows.push(FooterRow::new("Average"));
        }
        if let Some(footer) = self.footer_rows.first_mut() {
            footer
                .aggregations
                .push(ColumnAggregation::new(key, AggregationType::Average));
        }
        self.show_footer = true;
        self
    }
}
