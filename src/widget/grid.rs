//! Grid layout widget for CSS Grid-like layouts
//!
//! Provides a flexible grid system with rows, columns, gaps,
//! and span support for complex layouts.

use super::traits::{View, RenderContext, WidgetProps};
use crate::layout::Rect;
use crate::{impl_styled_view, impl_props_builders};

/// Maximum grid dimensions to prevent unbounded memory allocation
const MAX_GRID_SIZE: usize = 1000;

/// Grid track size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackSize {
    /// Fixed size in cells
    Fixed(u16),
    /// Fractional unit (like CSS `fr`)
    Fr(f32),
    /// Auto size (content-based)
    Auto,
    /// Minimum content size
    MinContent,
    /// Maximum content size
    MaxContent,
    /// Percentage of available space
    Percent(f32),
}

impl Default for TrackSize {
    fn default() -> Self {
        TrackSize::Fr(1.0)
    }
}

/// Grid item placement
#[derive(Clone, Copy, Debug, Default)]
pub struct GridPlacement {
    /// Column start (1-indexed, 0 = auto)
    pub col_start: u16,
    /// Column end (exclusive, 0 = auto)
    pub col_end: u16,
    /// Row start (1-indexed, 0 = auto)
    pub row_start: u16,
    /// Row end (exclusive, 0 = auto)
    pub row_end: u16,
}

impl GridPlacement {
    /// Create placement at specific cell
    pub fn cell(col: u16, row: u16) -> Self {
        Self {
            col_start: col,
            col_end: col + 1,
            row_start: row,
            row_end: row + 1,
        }
    }

    /// Create placement spanning columns
    pub fn col_span(col: u16, span: u16) -> Self {
        Self {
            col_start: col,
            col_end: col + span,
            row_start: 0,
            row_end: 0,
        }
    }

    /// Create placement spanning rows
    pub fn row_span(row: u16, span: u16) -> Self {
        Self {
            col_start: 0,
            col_end: 0,
            row_start: row,
            row_end: row + span,
        }
    }

    /// Create placement with full span
    pub fn area(col_start: u16, row_start: u16, col_span: u16, row_span: u16) -> Self {
        Self {
            col_start,
            col_end: col_start + col_span,
            row_start,
            row_end: row_start + row_span,
        }
    }

    /// Span multiple columns
    pub fn span_cols(mut self, span: u16) -> Self {
        if self.col_start > 0 {
            self.col_end = self.col_start + span;
        }
        self
    }

    /// Span multiple rows
    pub fn span_rows(mut self, span: u16) -> Self {
        if self.row_start > 0 {
            self.row_end = self.row_start + span;
        }
        self
    }
}

/// Grid item with placement
pub struct GridItem {
    /// The widget
    widget: Box<dyn View>,
    /// Placement in grid
    placement: GridPlacement,
}

impl GridItem {
    /// Create a new grid item
    pub fn new(widget: impl View + 'static) -> Self {
        Self {
            widget: Box::new(widget),
            placement: GridPlacement::default(),
        }
    }

    /// Set placement
    pub fn place(mut self, placement: GridPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Place at specific cell
    pub fn at(mut self, col: u16, row: u16) -> Self {
        self.placement = GridPlacement::cell(col, row);
        self
    }

    /// Span columns
    pub fn col_span(mut self, span: u16) -> Self {
        self.placement = self.placement.span_cols(span);
        self
    }

    /// Span rows
    pub fn row_span(mut self, span: u16) -> Self {
        self.placement = self.placement.span_rows(span);
        self
    }

    /// Set column
    pub fn col(mut self, col: u16) -> Self {
        self.placement.col_start = col;
        if self.placement.col_end == 0 {
            self.placement.col_end = col + 1;
        }
        self
    }

    /// Set row
    pub fn row(mut self, row: u16) -> Self {
        self.placement.row_start = row;
        if self.placement.row_end == 0 {
            self.placement.row_end = row + 1;
        }
        self
    }
}

/// Grid alignment for items
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridAlign {
    /// Stretch to fill cell
    #[default]
    Stretch,
    /// Align to start
    Start,
    /// Align to center
    Center,
    /// Align to end
    End,
}

/// Grid layout widget
pub struct Grid {
    /// Grid items
    items: Vec<GridItem>,
    /// Column definitions
    columns: Vec<TrackSize>,
    /// Row definitions
    rows: Vec<TrackSize>,
    /// Column gap
    col_gap: u16,
    /// Row gap
    row_gap: u16,
    /// Horizontal alignment
    justify_items: GridAlign,
    /// Vertical alignment
    align_items: GridAlign,
    /// Auto-flow direction
    auto_flow_row: bool,
    /// Auto column size
    auto_cols: TrackSize,
    /// Auto row size
    auto_rows: TrackSize,
    /// Widget properties
    props: WidgetProps,
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
                placement: GridPlacement::default(),
            });
        }
        self
    }

    /// Calculate track sizes
    fn calculate_tracks(&self, available: u16, tracks: &[TrackSize], _auto_size: TrackSize, gap: u16) -> Vec<u16> {
        if tracks.is_empty() {
            return vec![];
        }

        let total_gaps = (tracks.len().saturating_sub(1)) as u16 * gap;
        let available = available.saturating_sub(total_gaps);

        // First pass: calculate fixed sizes and collect fr units
        let mut sizes: Vec<u16> = vec![0; tracks.len()];
        let mut total_fr = 0.0f32;
        let mut remaining = available;

        for (i, track) in tracks.iter().enumerate() {
            match track {
                TrackSize::Fixed(size) => {
                    sizes[i] = *size;
                    remaining = remaining.saturating_sub(*size);
                }
                TrackSize::Percent(pct) => {
                    let size = ((available as f32) * pct / 100.0) as u16;
                    sizes[i] = size;
                    remaining = remaining.saturating_sub(size);
                }
                TrackSize::Fr(fr) => {
                    total_fr += fr;
                }
                TrackSize::Auto | TrackSize::MinContent | TrackSize::MaxContent => {
                    // For now, treat auto as 1fr
                    total_fr += 1.0;
                }
            }
        }

        // Second pass: distribute remaining space to fr units
        if total_fr > 0.0 {
            let per_fr = (remaining as f32) / total_fr;
            for (i, track) in tracks.iter().enumerate() {
                match track {
                    TrackSize::Fr(fr) => {
                        sizes[i] = (per_fr * fr) as u16;
                    }
                    TrackSize::Auto | TrackSize::MinContent | TrackSize::MaxContent => {
                        sizes[i] = per_fr as u16;
                    }
                    _ => {}
                }
            }
        }

        sizes
    }

    /// Get track positions (cumulative)
    fn track_positions(&self, sizes: &[u16], gap: u16) -> Vec<u16> {
        let mut positions = Vec::with_capacity(sizes.len() + 1);
        let mut pos = 0u16;
        positions.push(pos);

        for (i, &size) in sizes.iter().enumerate() {
            pos += size;
            if i < sizes.len() - 1 {
                pos += gap;
            }
            positions.push(pos);
        }

        positions
    }

    /// Auto-place items without explicit placement
    ///
    /// Grid dimensions are limited to `MAX_GRID_SIZE` to prevent unbounded memory allocation.
    fn auto_place_items(&self, col_count: usize, row_count: usize) -> Vec<(usize, GridPlacement)> {
        let mut placements = Vec::new();

        // Clamp initial dimensions to prevent excessive allocation
        let col_count = col_count.clamp(1, MAX_GRID_SIZE);
        let row_count = row_count.clamp(1, MAX_GRID_SIZE);

        let mut grid: Vec<Vec<bool>> = vec![vec![false; col_count]; row_count];
        let mut auto_col = 0usize;
        let mut auto_row = 0usize;

        for (idx, item) in self.items.iter().enumerate() {
            let placement = &item.placement;

            // Validate explicit placement is within bounds
            if placement.col_start > 0 && placement.col_start as usize > MAX_GRID_SIZE {
                continue; // Skip items placed beyond max grid size
            }
            if placement.row_start > 0 && placement.row_start as usize > MAX_GRID_SIZE {
                continue;
            }

            // Determine actual placement
            let (col_start, col_end, row_start, row_end) = if placement.col_start > 0 && placement.row_start > 0 {
                // Explicit placement - clamp to max grid size
                (
                    ((placement.col_start - 1) as usize).min(MAX_GRID_SIZE - 1),
                    ((placement.col_end - 1) as usize).min(MAX_GRID_SIZE),
                    ((placement.row_start - 1) as usize).min(MAX_GRID_SIZE - 1),
                    ((placement.row_end - 1) as usize).min(MAX_GRID_SIZE),
                )
            } else {
                // Auto-placement
                let col_span = if placement.col_end > placement.col_start {
                    ((placement.col_end - placement.col_start) as usize).min(col_count)
                } else {
                    1
                };
                let row_span = if placement.row_end > placement.row_start {
                    ((placement.row_end - placement.row_start) as usize).min(MAX_GRID_SIZE)
                } else {
                    1
                };

                // Find next available slot with bounded iterations
                let max_iterations = MAX_GRID_SIZE * MAX_GRID_SIZE;
                let mut iterations = 0;
                loop {
                    iterations += 1;
                    if iterations > max_iterations {
                        // Couldn't find slot within bounds, skip item
                        break;
                    }

                    if self.auto_flow_row {
                        if auto_col + col_span <= col_count {
                            let fits = (auto_row..auto_row + row_span).all(|r| {
                                r < grid.len() && (auto_col..auto_col + col_span).all(|c| c < grid[r].len() && !grid[r][c])
                            });
                            if fits {
                                break;
                            }
                        }
                        auto_col += 1;
                        if auto_col >= col_count {
                            auto_col = 0;
                            auto_row += 1;
                            // Expand grid if needed (with bounds check)
                            while grid.len() <= auto_row + row_span && grid.len() < MAX_GRID_SIZE {
                                grid.push(vec![false; col_count]);
                            }
                            if auto_row + row_span > MAX_GRID_SIZE {
                                break; // Can't expand further
                            }
                        }
                    } else {
                        if auto_row + row_span <= grid.len() {
                            let fits = (auto_col..auto_col + col_span).all(|c| {
                                (auto_row..auto_row + row_span).all(|r| r < grid.len() && c < grid[r].len() && !grid[r][c])
                            });
                            if fits {
                                break;
                            }
                        }
                        auto_row += 1;
                        if auto_row >= grid.len() {
                            auto_row = 0;
                            auto_col += 1;
                            if auto_col >= col_count && col_count < MAX_GRID_SIZE {
                                // Expand grid columns
                                for row in &mut grid {
                                    if row.len() < MAX_GRID_SIZE {
                                        row.push(false);
                                    }
                                }
                            } else if auto_col >= MAX_GRID_SIZE {
                                break; // Can't expand further
                            }
                        }
                    }
                }

                if iterations > max_iterations {
                    continue; // Skip this item
                }

                (auto_col, auto_col + col_span, auto_row, auto_row + row_span)
            };

            // Mark cells as occupied (with bounds checking)
            for r in row_start..row_end.min(MAX_GRID_SIZE) {
                while grid.len() <= r && grid.len() < MAX_GRID_SIZE {
                    grid.push(vec![false; col_count]);
                }
                if r >= grid.len() {
                    break;
                }
                for c in col_start..col_end.min(MAX_GRID_SIZE) {
                    while grid[r].len() <= c && grid[r].len() < MAX_GRID_SIZE {
                        grid[r].push(false);
                    }
                    if c < grid[r].len() {
                        grid[r][c] = true;
                    }
                }
            }

            placements.push((idx, GridPlacement {
                col_start: (col_start + 1) as u16,
                col_end: (col_end + 1) as u16,
                row_start: (row_start + 1) as u16,
                row_end: (row_end + 1) as u16,
            }));
        }

        placements
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Grid {
    crate::impl_view_meta!("Grid");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 || self.items.is_empty() {
            return;
        }

        // Determine grid dimensions
        let col_count = if self.columns.is_empty() {
            // Auto-detect from placements
            self.items.iter()
                .map(|item| item.placement.col_end.max(item.placement.col_start + 1) as usize)
                .max()
                .unwrap_or(1)
        } else {
            self.columns.len()
        };

        let row_count = if self.rows.is_empty() {
            self.items.iter()
                .map(|item| item.placement.row_end.max(item.placement.row_start + 1) as usize)
                .max()
                .unwrap_or(1)
        } else {
            self.rows.len()
        };

        // Build track definitions
        let col_tracks: Vec<TrackSize> = if self.columns.is_empty() {
            vec![self.auto_cols; col_count]
        } else {
            self.columns.clone()
        };

        let row_tracks: Vec<TrackSize> = if self.rows.is_empty() {
            vec![self.auto_rows; row_count]
        } else {
            self.rows.clone()
        };

        // Calculate track sizes
        let col_sizes = self.calculate_tracks(area.width, &col_tracks, self.auto_cols, self.col_gap);
        let row_sizes = self.calculate_tracks(area.height, &row_tracks, self.auto_rows, self.row_gap);

        // Get track positions
        let col_positions = self.track_positions(&col_sizes, self.col_gap);
        let row_positions = self.track_positions(&row_sizes, self.row_gap);

        // Auto-place items
        let placements = self.auto_place_items(col_count, row_count);

        // Render each item
        for (idx, placement) in placements {
            if idx >= self.items.len() {
                continue;
            }

            let item = &self.items[idx];

            // Get cell bounds
            let col_start = (placement.col_start as usize).saturating_sub(1);
            let col_end = (placement.col_end as usize).saturating_sub(1);
            let row_start = (placement.row_start as usize).saturating_sub(1);
            let row_end = (placement.row_end as usize).saturating_sub(1);

            if col_start >= col_positions.len() || row_start >= row_positions.len() {
                continue;
            }

            let x = area.x + col_positions.get(col_start).copied().unwrap_or(0);
            let y = area.y + row_positions.get(row_start).copied().unwrap_or(0);

            let end_x = col_positions.get(col_end).copied().unwrap_or(area.width);
            let end_y = row_positions.get(row_end).copied().unwrap_or(area.height);

            let width = end_x.saturating_sub(col_positions.get(col_start).copied().unwrap_or(0));
            let height = end_y.saturating_sub(row_positions.get(row_start).copied().unwrap_or(0));

            if width == 0 || height == 0 {
                continue;
            }

            let cell_rect = Rect::new(x, y, width, height);

            let mut child_ctx = RenderContext::new(ctx.buffer, cell_rect);

            item.widget.render(&mut child_ctx);
        }
    }
}

impl_styled_view!(Grid);
impl_props_builders!(Grid);

/// Helper to create a grid
pub fn grid() -> Grid {
    Grid::new()
}

/// Helper to create a grid item
pub fn grid_item(widget: impl View + 'static) -> GridItem {
    GridItem::new(widget)
}

/// Create a simple NxM grid
pub fn grid_template(cols: usize, rows: usize) -> Grid {
    Grid::new()
        .cols(cols)
        .rows_count(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::widget::Text;

    #[test]
    fn test_grid_new() {
        let g = Grid::new();
        assert!(g.items.is_empty());
        assert!(g.columns.is_empty());
    }

    #[test]
    fn test_grid_columns() {
        let g = Grid::new().columns(vec![
            TrackSize::Fixed(10),
            TrackSize::Fr(1.0),
            TrackSize::Fr(2.0),
        ]);
        assert_eq!(g.columns.len(), 3);
    }

    #[test]
    fn test_grid_cols() {
        let g = Grid::new().cols(3);
        assert_eq!(g.columns.len(), 3);
        assert!(g.columns.iter().all(|t| matches!(t, TrackSize::Fr(1.0))));
    }

    #[test]
    fn test_grid_gap() {
        let g = Grid::new().gap(2);
        assert_eq!(g.col_gap, 2);
        assert_eq!(g.row_gap, 2);
    }

    #[test]
    fn test_grid_placement_cell() {
        let p = GridPlacement::cell(2, 3);
        assert_eq!(p.col_start, 2);
        assert_eq!(p.col_end, 3);
        assert_eq!(p.row_start, 3);
        assert_eq!(p.row_end, 4);
    }

    #[test]
    fn test_grid_placement_area() {
        let p = GridPlacement::area(1, 1, 2, 3);
        assert_eq!(p.col_start, 1);
        assert_eq!(p.col_end, 3);
        assert_eq!(p.row_start, 1);
        assert_eq!(p.row_end, 4);
    }

    #[test]
    fn test_grid_item_at() {
        let item = GridItem::new(Text::new("Test")).at(2, 3);
        assert_eq!(item.placement.col_start, 2);
        assert_eq!(item.placement.row_start, 3);
    }

    #[test]
    fn test_grid_item_span() {
        let item = GridItem::new(Text::new("Test"))
            .at(1, 1)
            .col_span(2)
            .row_span(3);
        assert_eq!(item.placement.col_end, 3);
        assert_eq!(item.placement.row_end, 4);
    }

    #[test]
    fn test_track_size_default() {
        let t = TrackSize::default();
        assert!(matches!(t, TrackSize::Fr(1.0)));
    }

    #[test]
    fn test_calculate_tracks_fixed() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Fixed(10), TrackSize::Fixed(20)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(sizes, vec![10, 20]);
    }

    #[test]
    fn test_calculate_tracks_fr() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Fr(1.0), TrackSize::Fr(1.0)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(sizes, vec![50, 50]);
    }

    #[test]
    fn test_calculate_tracks_mixed() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Fixed(20), TrackSize::Fr(1.0), TrackSize::Fr(2.0)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 0);
        // 20 fixed, remaining 80 split 1:2 = ~26, ~53
        assert_eq!(sizes[0], 20);
        assert!(sizes[1] > 0);
        assert!(sizes[2] > sizes[1]);
    }

    #[test]
    fn test_calculate_tracks_with_gap() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Fr(1.0), TrackSize::Fr(1.0)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 10);
        // 100 - 10 gap = 90, split evenly = 45, 45
        assert_eq!(sizes, vec![45, 45]);
    }

    #[test]
    fn test_track_positions() {
        let g = Grid::new();
        let sizes = vec![10, 20, 30];
        let positions = g.track_positions(&sizes, 0);
        assert_eq!(positions, vec![0, 10, 30, 60]);
    }

    #[test]
    fn test_track_positions_with_gap() {
        let g = Grid::new();
        let sizes = vec![10, 20];
        let positions = g.track_positions(&sizes, 5);
        assert_eq!(positions, vec![0, 15, 35]);
    }

    #[test]
    fn test_grid_render() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let g = Grid::new()
            .cols(2)
            .rows_count(2)
            .child(Text::new("A"))
            .child(Text::new("B"))
            .child(Text::new("C"))
            .child(Text::new("D"));

        g.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_grid_with_explicit_placement() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let g = Grid::new()
            .cols(3)
            .rows_count(3)
            .item(GridItem::new(Text::new("Header")).at(1, 1).col_span(3))
            .item(GridItem::new(Text::new("Sidebar")).at(1, 2).row_span(2))
            .item(GridItem::new(Text::new("Content")).at(2, 2).col_span(2));

        g.render(&mut ctx);
    }

    #[test]
    fn test_grid_helper() {
        let g = grid().cols(3);
        assert_eq!(g.columns.len(), 3);
    }

    #[test]
    fn test_grid_template() {
        let g = grid_template(3, 2);
        assert_eq!(g.columns.len(), 3);
        assert_eq!(g.rows.len(), 2);
    }

    #[test]
    fn test_grid_align() {
        let g = Grid::new()
            .justify_items(GridAlign::Center)
            .align_items(GridAlign::End);

        assert!(matches!(g.justify_items, GridAlign::Center));
        assert!(matches!(g.align_items, GridAlign::End));
    }

    #[test]
    fn test_grid_auto_flow() {
        let g1 = Grid::new().auto_flow_row();
        assert!(g1.auto_flow_row);

        let g2 = Grid::new().auto_flow_col();
        assert!(!g2.auto_flow_row);
    }

    #[test]
    fn test_grid_percent_tracks() {
        let g = Grid::new();
        let tracks = vec![TrackSize::Percent(25.0), TrackSize::Percent(75.0)];
        let sizes = g.calculate_tracks(100, &tracks, TrackSize::Fr(1.0), 0);
        assert_eq!(sizes, vec![25, 75]);
    }
}
