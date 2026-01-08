//! DataGrid types and configurations
//!
//! This module contains all standalone types used by the DataGrid widget.

use crate::style::Color;

// ═══════════════════════════════════════════════════════════════════════════
// Grid Colors
// ═══════════════════════════════════════════════════════════════════════════

/// Grid color scheme
#[derive(Clone, Debug)]
pub struct GridColors {
    /// Header background color
    pub header_bg: Color,
    /// Header foreground color
    pub header_fg: Color,
    /// Normal row background
    pub row_bg: Color,
    /// Alternate row background (zebra striping)
    pub alt_row_bg: Color,
    /// Selected row background
    pub selected_bg: Color,
    /// Selected row foreground
    pub selected_fg: Color,
    /// Border/separator color
    pub border_color: Color,
}

impl Default for GridColors {
    fn default() -> Self {
        Self {
            header_bg: Color::rgb(60, 60, 80),
            header_fg: Color::WHITE,
            row_bg: Color::rgb(30, 30, 30),
            alt_row_bg: Color::rgb(40, 40, 40),
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            border_color: Color::rgb(80, 80, 80),
        }
    }
}

impl GridColors {
    /// Create a new color scheme
    pub fn new() -> Self {
        Self::default()
    }

    /// Dark theme (default)
    pub fn dark() -> Self {
        Self::default()
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            header_bg: Color::rgb(220, 220, 230),
            header_fg: Color::BLACK,
            row_bg: Color::rgb(255, 255, 255),
            alt_row_bg: Color::rgb(245, 245, 250),
            selected_bg: Color::rgb(100, 150, 220),
            selected_fg: Color::WHITE,
            border_color: Color::rgb(180, 180, 190),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Grid Options
// ═══════════════════════════════════════════════════════════════════════════

/// Grid display options
#[derive(Clone, Debug)]
pub struct GridOptions {
    /// Show header row
    pub show_header: bool,
    /// Show row numbers column
    pub show_row_numbers: bool,
    /// Enable multi-row selection
    pub multi_select: bool,
    /// Enable zebra striping (alternating row colors)
    pub zebra: bool,
    /// Use natural sorting for text (file2 < file10)
    pub use_natural_sort: bool,
    /// Enable virtual scrolling for large datasets
    pub virtual_scroll: bool,
    /// Row height in lines (for virtual scroll calculations)
    pub row_height: u16,
    /// Overscan rows (extra rows rendered above/below viewport for smooth scrolling)
    pub overscan: usize,
}

impl Default for GridOptions {
    fn default() -> Self {
        Self {
            show_header: true,
            show_row_numbers: false,
            multi_select: false,
            zebra: true,
            use_natural_sort: true,
            virtual_scroll: true,
            row_height: 1,
            overscan: 5,
        }
    }
}

impl GridOptions {
    /// Create new options with defaults
    pub fn new() -> Self {
        Self::default()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Column Types
// ═══════════════════════════════════════════════════════════════════════════

/// Column data type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnType {
    #[default]
    Text,
    Number,
    Date,
    Boolean,
    Custom,
}

/// Sort direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    /// Sort ascending (A-Z, 0-9)
    Ascending,
    /// Sort descending (Z-A, 9-0)
    Descending,
}

impl SortDirection {
    pub(super) fn toggle(&self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }

    pub(super) fn icon(&self) -> char {
        match self {
            SortDirection::Ascending => '▲',
            SortDirection::Descending => '▼',
        }
    }
}

/// Text alignment
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

// ═══════════════════════════════════════════════════════════════════════════
// Grid Column
// ═══════════════════════════════════════════════════════════════════════════

/// Grid column definition
#[derive(Clone)]
pub struct GridColumn {
    /// Column key/id
    pub key: String,
    /// Display title
    pub title: String,
    /// Column type
    pub col_type: ColumnType,
    /// Width (0 = auto)
    pub width: u16,
    /// Minimum width
    pub min_width: u16,
    /// Maximum width
    pub max_width: u16,
    /// Is sortable
    pub sortable: bool,
    /// Is filterable
    pub filterable: bool,
    /// Is editable
    pub editable: bool,
    /// Is visible
    pub visible: bool,
    /// Alignment
    pub align: Alignment,
    /// Is resizable (can drag to resize)
    pub resizable: bool,
    /// Is frozen (stays visible during horizontal scroll)
    pub frozen: bool,
}

impl GridColumn {
    /// Create a new column
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            col_type: ColumnType::Text,
            width: 0,
            min_width: 5,
            max_width: 50,
            sortable: true,
            filterable: true,
            editable: false,
            visible: true,
            align: Alignment::Left,
            resizable: true,
            frozen: false,
        }
    }

    /// Set column type
    pub fn col_type(mut self, t: ColumnType) -> Self {
        self.col_type = t;
        self
    }

    /// Set width
    pub fn width(mut self, w: u16) -> Self {
        self.width = w;
        self
    }

    /// Set min width
    pub fn min_width(mut self, w: u16) -> Self {
        self.min_width = w;
        self
    }

    /// Set max width
    pub fn max_width(mut self, w: u16) -> Self {
        self.max_width = w;
        self
    }

    /// Set sortable
    pub fn sortable(mut self, s: bool) -> Self {
        self.sortable = s;
        self
    }

    /// Set editable
    pub fn editable(mut self, e: bool) -> Self {
        self.editable = e;
        self
    }

    /// Set alignment
    pub fn align(mut self, a: Alignment) -> Self {
        self.align = a;
        self
    }

    /// Right align
    pub fn right(mut self) -> Self {
        self.align = Alignment::Right;
        self
    }

    /// Center align
    pub fn center(mut self) -> Self {
        self.align = Alignment::Center;
        self
    }

    /// Set resizable (can drag to resize)
    pub fn resizable(mut self, r: bool) -> Self {
        self.resizable = r;
        self
    }

    /// Set frozen (stays visible during horizontal scroll)
    pub fn frozen(mut self, f: bool) -> Self {
        self.frozen = f;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Grid Row
// ═══════════════════════════════════════════════════════════════════════════

/// A row in the grid
#[derive(Clone, Debug)]
pub struct GridRow {
    /// Row data (key -> value)
    pub data: Vec<(String, String)>,
    /// Row is selected
    pub selected: bool,
    /// Row is expanded (for tree grids)
    pub expanded: bool,
    /// Child rows
    pub children: Vec<GridRow>,
}

impl GridRow {
    /// Create a new row
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            selected: false,
            expanded: false,
            children: Vec::new(),
        }
    }

    /// Add cell data
    pub fn cell(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.push((key.into(), value.into()));
        self
    }

    /// Get cell value by key
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// Add a child row (for tree grid)
    pub fn child(mut self, row: GridRow) -> Self {
        self.children.push(row);
        self
    }

    /// Add multiple child rows
    pub fn children(mut self, rows: Vec<GridRow>) -> Self {
        self.children.extend(rows);
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if row has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

impl Default for GridRow {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Export Types
// ═══════════════════════════════════════════════════════════════════════════

/// Export format for clipboard/file export
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExportFormat {
    #[default]
    Csv,
    Tsv,
    PlainText,
}

/// Export options
#[derive(Clone, Debug)]
pub struct ExportOptions {
    /// Output format
    pub format: ExportFormat,
    /// Include column headers
    pub include_headers: bool,
    /// Export only selected rows
    pub selected_only: bool,
    /// Export only visible columns
    pub visible_columns_only: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: ExportFormat::Csv,
            include_headers: true,
            selected_only: false,
            visible_columns_only: true,
        }
    }
}

impl ExportOptions {
    /// Create new export options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set format
    pub fn format(mut self, format: ExportFormat) -> Self {
        self.format = format;
        self
    }

    /// Include headers
    pub fn include_headers(mut self, include: bool) -> Self {
        self.include_headers = include;
        self
    }

    /// Export selected rows only
    pub fn selected_only(mut self, selected: bool) -> Self {
        self.selected_only = selected;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Aggregation Types
// ═══════════════════════════════════════════════════════════════════════════

/// Aggregation function type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AggregationType {
    #[default]
    Sum,
    Average,
    Count,
    Min,
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
