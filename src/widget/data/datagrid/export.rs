//! DataGrid export functionality

use super::core::DataGrid;
use super::types::{ExportFormat, ExportOptions};

impl DataGrid {
    /// Export grid data with options
    pub fn export(&self, options: &ExportOptions) -> String {
        let separator = match options.format {
            ExportFormat::Csv => ',',
            ExportFormat::Tsv => '\t',
            ExportFormat::PlainText => ' ',
        };

        let mut output = String::new();

        // Get visible columns
        let visible_cols: Vec<_> = if options.visible_columns_only {
            self.columns
                .iter()
                .enumerate()
                .filter(|(_, c)| c.visible)
                .collect()
        } else {
            self.columns.iter().enumerate().collect()
        };

        // Export headers
        if options.include_headers {
            let headers: Vec<_> = visible_cols
                .iter()
                .map(|(_, c)| self.escape_value(&c.title, options.format))
                .collect();
            output.push_str(&headers.join(&separator.to_string()));
            output.push('\n');
        }

        // Get rows to export
        let row_indices: Vec<usize> = if options.selected_only {
            self.filtered_indices()
                .iter()
                .enumerate()
                .filter(|(_, &idx)| self.rows.get(idx).is_some_and(|r| r.selected))
                .map(|(i, _)| i)
                .collect()
        } else {
            (0..self.filtered_count()).collect()
        };

        // Export rows
        for row_idx in row_indices {
            if let Some(&actual_idx) = self.filtered_indices().get(row_idx) {
                if let Some(row) = self.rows.get(actual_idx) {
                    let values: Vec<_> = visible_cols
                        .iter()
                        .map(|(_, c)| {
                            let value = row.get(&c.key).unwrap_or("");
                            self.escape_value(value, options.format)
                        })
                        .collect();
                    output.push_str(&values.join(&separator.to_string()));
                    output.push('\n');
                }
            }
        }

        output
    }

    /// Export as CSV
    pub fn export_csv(&self) -> String {
        self.export(&ExportOptions::new().format(ExportFormat::Csv))
    }

    /// Export as TSV
    pub fn export_tsv(&self) -> String {
        self.export(&ExportOptions::new().format(ExportFormat::Tsv))
    }

    /// Copy current cell value
    pub fn copy_cell(&self) -> String {
        let visible_cols: Vec<_> = self.columns.iter().filter(|c| c.visible).collect();

        if let Some(col) = visible_cols.get(self.selected_col) {
            if let Some(&actual_idx) = self.filtered_indices().get(self.selected_row) {
                if let Some(row) = self.rows.get(actual_idx) {
                    return row.get(&col.key).unwrap_or("").to_string();
                }
            }
        }
        String::new()
    }

    /// Copy selected rows as CSV
    pub fn copy_selected(&self) -> String {
        self.export(&ExportOptions::new().selected_only(true))
    }

    /// Escape value for export format
    fn escape_value(&self, value: &str, format: ExportFormat) -> String {
        match format {
            ExportFormat::Csv => {
                if value.contains(',') || value.contains('"') || value.contains('\n') {
                    format!("\"{}\"", value.replace('"', "\"\""))
                } else {
                    value.to_string()
                }
            }
            ExportFormat::Tsv => {
                if value.contains('\t') || value.contains('\n') {
                    value
                        .chars()
                        .map(|c| if c == '\t' || c == '\n' { ' ' } else { c })
                        .collect()
                } else {
                    value.to_string()
                }
            }
            ExportFormat::PlainText => value.to_string(),
        }
    }
}
