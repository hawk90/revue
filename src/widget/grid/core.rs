//! Grid widget core implementation

use super::types::{GridAlign, GridItem, TrackSize};
use crate::widget::traits::{View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Grid layout widget
pub struct Grid {
    /// Grid items
    pub items: Vec<GridItem>,
    /// Column definitions
    pub columns: Vec<TrackSize>,
    /// Row definitions
    pub rows: Vec<TrackSize>,
    /// Column gap
    pub col_gap: u16,
    /// Row gap
    pub row_gap: u16,
    /// Horizontal alignment
    pub justify_items: GridAlign,
    /// Vertical alignment
    pub align_items: GridAlign,
    /// Auto-flow direction
    pub auto_flow_row: bool,
    /// Auto column size
    pub auto_cols: TrackSize,
    /// Auto row size
    pub auto_rows: TrackSize,
    /// Widget properties
    pub props: WidgetProps,
}

impl Grid {
    /// Create a new grid
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            columns: Vec::new(),
            rows: Vec::new(),
            col_gap: 0,
            row_gap: 0,
            justify_items: GridAlign::Stretch,
            align_items: GridAlign::Stretch,
            auto_flow_row: true,
            auto_cols: TrackSize::Fr(1.0),
            auto_rows: TrackSize::Fr(1.0),
            props: WidgetProps::new(),
        }
    }

    /// Set column definitions
    pub fn columns(mut self, cols: Vec<TrackSize>) -> Self {
        self.columns = cols;
        self
    }

    /// Set row definitions
    pub fn rows(mut self, rows: Vec<TrackSize>) -> Self {
        self.rows = rows;
        self
    }

    /// Set equal columns
    pub fn cols(mut self, count: usize) -> Self {
        self.columns = vec![TrackSize::Fr(1.0); count];
        self
    }

    /// Set equal rows
    pub fn rows_count(mut self, count: usize) -> Self {
        self.rows = vec![TrackSize::Fr(1.0); count];
        self
    }

    /// Set column gap
    pub fn col_gap(mut self, gap: u16) -> Self {
        self.col_gap = gap;
        self
    }

    /// Set row gap
    pub fn row_gap(mut self, gap: u16) -> Self {
        self.row_gap = gap;
        self
    }

    /// Set both gaps
    pub fn gap(mut self, gap: u16) -> Self {
        self.col_gap = gap;
        self.row_gap = gap;
        self
    }

    /// Set horizontal alignment
    pub fn justify_items(mut self, align: GridAlign) -> Self {
        self.justify_items = align;
        self
    }

    /// Set vertical alignment
    pub fn align_items(mut self, align: GridAlign) -> Self {
        self.align_items = align;
        self
    }

    /// Set auto-flow to row (default)
    pub fn auto_flow_row(mut self) -> Self {
        self.auto_flow_row = true;
        self
    }

    /// Set auto-flow to column
    pub fn auto_flow_col(mut self) -> Self {
        self.auto_flow_row = false;
        self
    }

    /// Set auto column size
    pub fn auto_cols(mut self, size: TrackSize) -> Self {
        self.auto_cols = size;
        self
    }

    /// Set auto row size
    pub fn auto_rows(mut self, size: TrackSize) -> Self {
        self.auto_rows = size;
        self
    }

    /// Add a grid item
    pub fn item(mut self, item: GridItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add a widget (auto-placed)
    pub fn child(mut self, widget: impl View + 'static) -> Self {
        self.items.push(GridItem::new(widget));
        self
    }

    /// Add multiple children
    pub fn children(mut self, widgets: Vec<Box<dyn View>>) -> Self {
        for widget in widgets {
            self.items.push(GridItem {
                widget,
                placement: Default::default(),
            });
        }
        self
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Grid);
impl_props_builders!(Grid);
