//! DataGrid footer aggregation over the shown rows

use crate::widget::data::datagrid::types::{AggregationType, FooterRow};
use crate::widget::data::datagrid::DataGrid;

impl DataGrid {
    /// Compute aggregation value for a column
    pub(crate) fn compute_aggregation(
        &self,
        column_key: &str,
        agg_type: AggregationType,
    ) -> Option<f64> {
        let values: Vec<f64> = self
            .filtered_indices()
            .iter()
            .filter_map(|&idx| {
                self.rows
                    .get(idx)
                    .and_then(|r| r.get(column_key))
                    .and_then(|v| v.parse::<f64>().ok())
            })
            .collect();

        if values.is_empty() {
            return None;
        }

        Some(match agg_type {
            AggregationType::Sum => values.iter().sum(),
            AggregationType::Average => values.iter().sum::<f64>() / values.len() as f64,
            AggregationType::Count => values.len() as f64,
            AggregationType::Min => values.iter().cloned().fold(f64::INFINITY, f64::min),
            AggregationType::Max => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        })
    }

    /// Get computed footer values for rendering
    pub(crate) fn get_footer_values(&self, footer: &FooterRow) -> Vec<(String, String)> {
        let mut values = Vec::new();

        for agg in &footer.aggregations {
            let label = agg
                .label
                .clone()
                .unwrap_or_else(|| agg.agg_type.label().to_string());

            let value = self
                .compute_aggregation(&agg.column_key, agg.agg_type)
                .map(|v| {
                    if v.fract() == 0.0 {
                        format!("{:.0}", v)
                    } else {
                        format!("{:.2}", v)
                    }
                })
                .unwrap_or_else(|| "—".to_string());

            values.push((agg.column_key.clone(), format!("{}: {}", label, value)));
        }

        values
    }
}
