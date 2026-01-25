//! Aggregation types and configurations

/// Aggregation function type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AggregationType {
    /// Sum of all values
    #[default]
    Sum,
    /// Average (mean) of all values
    Average,
    /// Count of all values
    Count,
    /// Minimum value
    Min,
    /// Maximum value
    Max,
}

impl AggregationType {
    /// Get display label for aggregation type
    pub fn label(&self) -> &'static str {
        match self {
            AggregationType::Sum => "Sum",
            AggregationType::Average => "Avg",
            AggregationType::Count => "Count",
            AggregationType::Min => "Min",
            AggregationType::Max => "Max",
        }
    }
}

/// Column aggregation configuration
#[derive(Clone, Debug)]
pub struct ColumnAggregation {
    /// Column key to aggregate
    pub column_key: String,
    /// Aggregation type
    pub agg_type: AggregationType,
    /// Custom label (overrides default)
    pub label: Option<String>,
}

impl ColumnAggregation {
    /// Create new column aggregation
    pub fn new(column_key: impl Into<String>, agg_type: AggregationType) -> Self {
        Self {
            column_key: column_key.into(),
            agg_type,
            label: None,
        }
    }

    /// Set custom label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Footer row for aggregations
#[derive(Clone, Debug, Default)]
pub struct FooterRow {
    /// Row label (e.g., "Totals")
    pub label: String,
    /// Column aggregations
    pub aggregations: Vec<ColumnAggregation>,
}

impl FooterRow {
    /// Create new footer row
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            aggregations: Vec::new(),
        }
    }

    /// Add aggregation
    pub fn aggregation(mut self, agg: ColumnAggregation) -> Self {
        self.aggregations.push(agg);
        self
    }

    /// Add sum aggregation for column
    pub fn sum(mut self, column_key: impl Into<String>) -> Self {
        self.aggregations
            .push(ColumnAggregation::new(column_key, AggregationType::Sum));
        self
    }

    /// Add average aggregation for column
    pub fn average(mut self, column_key: impl Into<String>) -> Self {
        self.aggregations
            .push(ColumnAggregation::new(column_key, AggregationType::Average));
        self
    }

    /// Add count aggregation for column
    pub fn count(mut self, column_key: impl Into<String>) -> Self {
        self.aggregations
            .push(ColumnAggregation::new(column_key, AggregationType::Count));
        self
    }

    /// Add min aggregation for column
    pub fn min(mut self, column_key: impl Into<String>) -> Self {
        self.aggregations
            .push(ColumnAggregation::new(column_key, AggregationType::Min));
        self
    }

    /// Add max aggregation for column
    pub fn max(mut self, column_key: impl Into<String>) -> Self {
        self.aggregations
            .push(ColumnAggregation::new(column_key, AggregationType::Max));
        self
    }
}
