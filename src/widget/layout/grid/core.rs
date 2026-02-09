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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::Text;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_grid_new() {
        let grid = Grid::new();
        assert!(grid.items.is_empty());
        assert!(grid.columns.is_empty());
        assert!(grid.rows.is_empty());
        assert_eq!(grid.col_gap, 0);
        assert_eq!(grid.row_gap, 0);
        assert_eq!(grid.justify_items, GridAlign::Stretch);
        assert_eq!(grid.align_items, GridAlign::Stretch);
        assert!(grid.auto_flow_row);
        assert_eq!(grid.auto_cols, TrackSize::Fr(1.0));
        assert_eq!(grid.auto_rows, TrackSize::Fr(1.0));
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_grid_columns() {
        let grid = Grid::new().columns(vec![TrackSize::Fixed(10), TrackSize::Fr(1.0)]);
        assert_eq!(grid.columns.len(), 2);
        assert_eq!(grid.columns[0], TrackSize::Fixed(10));
        assert_eq!(grid.columns[1], TrackSize::Fr(1.0));
    }

    #[test]
    fn test_grid_rows() {
        let grid = Grid::new().rows(vec![TrackSize::Auto, TrackSize::Fixed(5)]);
        assert_eq!(grid.rows.len(), 2);
        assert_eq!(grid.rows[0], TrackSize::Auto);
        assert_eq!(grid.rows[1], TrackSize::Fixed(5));
    }

    #[test]
    fn test_grid_cols() {
        let grid = Grid::new().cols(3);
        assert_eq!(grid.columns.len(), 3);
        assert_eq!(grid.columns[0], TrackSize::Fr(1.0));
        assert_eq!(grid.columns[1], TrackSize::Fr(1.0));
        assert_eq!(grid.columns[2], TrackSize::Fr(1.0));
    }

    #[test]
    fn test_grid_rows_count() {
        let grid = Grid::new().rows_count(4);
        assert_eq!(grid.rows.len(), 4);
        for row in &grid.rows {
            assert_eq!(*row, TrackSize::Fr(1.0));
        }
    }

    #[test]
    fn test_grid_col_gap() {
        let grid = Grid::new().col_gap(5);
        assert_eq!(grid.col_gap, 5);
    }

    #[test]
    fn test_grid_row_gap() {
        let grid = Grid::new().row_gap(3);
        assert_eq!(grid.row_gap, 3);
    }

    #[test]
    fn test_grid_gap() {
        let grid = Grid::new().gap(2);
        assert_eq!(grid.col_gap, 2);
        assert_eq!(grid.row_gap, 2);
    }

    #[test]
    fn test_grid_justify_items() {
        let grid = Grid::new().justify_items(GridAlign::Center);
        assert_eq!(grid.justify_items, GridAlign::Center);
    }

    #[test]
    fn test_grid_align_items() {
        let grid = Grid::new().align_items(GridAlign::End);
        assert_eq!(grid.align_items, GridAlign::End);
    }

    #[test]
    fn test_grid_auto_flow_row() {
        let grid = Grid::new().auto_flow_row();
        assert!(grid.auto_flow_row);
    }

    #[test]
    fn test_grid_auto_flow_col() {
        let grid = Grid::new().auto_flow_col();
        assert!(!grid.auto_flow_row);
    }

    #[test]
    fn test_grid_auto_cols() {
        let grid = Grid::new().auto_cols(TrackSize::Fixed(10));
        assert_eq!(grid.auto_cols, TrackSize::Fixed(10));
    }

    #[test]
    fn test_grid_auto_rows() {
        let grid = Grid::new().auto_rows(TrackSize::Auto);
        assert_eq!(grid.auto_rows, TrackSize::Auto);
    }

    // =========================================================================
    // Item builder tests
    // =========================================================================

    #[test]
    fn test_grid_item() {
        let item = GridItem::new(Text::new("Test"));
        let grid = Grid::new().item(item);
        assert_eq!(grid.items.len(), 1);
    }

    #[test]
    fn test_grid_child() {
        let grid = Grid::new().child(Text::new("Child"));
        assert_eq!(grid.items.len(), 1);
    }

    #[test]
    fn test_grid_children() {
        let widgets: Vec<Box<dyn View>> = vec![
            Box::new(Text::new("A")),
            Box::new(Text::new("B")),
            Box::new(Text::new("C")),
        ];
        let grid = Grid::new().children(widgets);
        assert_eq!(grid.items.len(), 3);
    }

    // =========================================================================
    // Default trait tests
    // =========================================================================

    #[test]
    fn test_grid_default() {
        let grid = Grid::default();
        assert!(grid.items.is_empty());
        assert!(grid.columns.is_empty());
        assert!(grid.rows.is_empty());
        assert_eq!(grid.col_gap, 0);
        assert_eq!(grid.row_gap, 0);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_grid_builder_chain() {
        let grid = Grid::new()
            .cols(3)
            .rows_count(2)
            .gap(2)
            .justify_items(GridAlign::Center)
            .align_items(GridAlign::Start)
            .child(Text::new("A"))
            .child(Text::new("B"));

        assert_eq!(grid.columns.len(), 3);
        assert_eq!(grid.rows.len(), 2);
        assert_eq!(grid.col_gap, 2);
        assert_eq!(grid.row_gap, 2);
        assert_eq!(grid.justify_items, GridAlign::Center);
        assert_eq!(grid.align_items, GridAlign::Start);
        assert_eq!(grid.items.len(), 2);
    }

    // =========================================================================
    // TrackSize tests
    // =========================================================================

    #[test]
    fn test_track_size_fixed() {
        let track = TrackSize::Fixed(10);
        assert_eq!(track, TrackSize::Fixed(10));
    }

    #[test]
    fn test_track_size_fr() {
        let track = TrackSize::Fr(2.0);
        assert_eq!(track, TrackSize::Fr(2.0));
    }

    #[test]
    fn test_track_size_auto() {
        let track = TrackSize::Auto;
        assert_eq!(track, TrackSize::Auto);
    }

    #[test]
    fn test_track_size_min_content() {
        let track = TrackSize::MinContent;
        assert_eq!(track, TrackSize::MinContent);
    }

    #[test]
    fn test_track_size_max_content() {
        let track = TrackSize::MaxContent;
        assert_eq!(track, TrackSize::MaxContent);
    }

    #[test]
    fn test_track_size_percent() {
        let track = TrackSize::Percent(50.0);
        assert_eq!(track, TrackSize::Percent(50.0));
    }

    #[test]
    fn test_track_size_default() {
        let track = TrackSize::default();
        assert_eq!(track, TrackSize::Fr(1.0));
    }

    #[test]
    fn test_track_size_clone() {
        let track1 = TrackSize::Fr(2.5);
        let track2 = track1.clone();
        assert_eq!(track1, track2);
    }

    #[test]
    fn test_track_size_copy() {
        let track1 = TrackSize::Fixed(10);
        let track2 = track1;
        assert_eq!(track1, TrackSize::Fixed(10));
        assert_eq!(track2, TrackSize::Fixed(10));
    }

    #[test]
    fn test_track_size_partial_eq() {
        assert_eq!(TrackSize::Auto, TrackSize::Auto);
        assert_eq!(TrackSize::Fixed(10), TrackSize::Fixed(10));
        assert_eq!(TrackSize::Fr(1.0), TrackSize::Fr(1.0));
        assert_ne!(TrackSize::Auto, TrackSize::Fixed(10));
    }

    // =========================================================================
    // GridAlign tests
    // =========================================================================

    #[test]
    fn test_grid_align_stretch() {
        let align = GridAlign::Stretch;
        assert_eq!(align, GridAlign::Stretch);
    }

    #[test]
    fn test_grid_align_start() {
        let align = GridAlign::Start;
        assert_eq!(align, GridAlign::Start);
    }

    #[test]
    fn test_grid_align_center() {
        let align = GridAlign::Center;
        assert_eq!(align, GridAlign::Center);
    }

    #[test]
    fn test_grid_align_end() {
        let align = GridAlign::End;
        assert_eq!(align, GridAlign::End);
    }

    #[test]
    fn test_grid_align_default() {
        let align = GridAlign::default();
        assert_eq!(align, GridAlign::Stretch);
    }

    #[test]
    fn test_grid_align_clone() {
        let align1 = GridAlign::Center;
        let align2 = align1.clone();
        assert_eq!(align1, align2);
    }

    #[test]
    fn test_grid_align_copy() {
        let align1 = GridAlign::End;
        let align2 = align1;
        assert_eq!(align1, GridAlign::End);
        assert_eq!(align2, GridAlign::End);
    }

    #[test]
    fn test_grid_align_partial_eq() {
        assert_eq!(GridAlign::Start, GridAlign::Start);
        assert_ne!(GridAlign::Start, GridAlign::End);
    }
}
