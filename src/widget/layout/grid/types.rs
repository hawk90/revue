//! Grid widget types

use crate::widget::traits::View;

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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    ///
    /// Before a start column is set (0, auto) the span is kept as the end,
    /// which auto placement reads as the span, so a later start keeps it.
    pub fn span_cols(mut self, span: u16) -> Self {
        self.col_end = self.col_start.saturating_add(span);
        self
    }

    /// Span multiple rows
    ///
    /// Before a start row is set (0, auto) the span is kept as the end, as
    /// for [`span_cols`](Self::span_cols).
    pub fn span_rows(mut self, span: u16) -> Self {
        self.row_end = self.row_start.saturating_add(span);
        self
    }

    /// The columns spanned: at least one
    fn col_count(&self) -> u16 {
        self.col_end.saturating_sub(self.col_start).max(1)
    }

    /// The rows spanned: at least one
    fn row_count(&self) -> u16 {
        self.row_end.saturating_sub(self.row_start).max(1)
    }
}

/// Grid item with placement
pub struct GridItem {
    /// The widget
    pub widget: Box<dyn View>,
    /// Placement in grid
    pub placement: GridPlacement,
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
        // The span stays, set before or after (#1011)
        let span = self.placement.col_count();
        self.placement.col_start = col;
        self.placement.col_end = col.saturating_add(span);
        self
    }

    /// Set row
    pub fn row(mut self, row: u16) -> Self {
        let span = self.placement.row_count();
        self.placement.row_start = row;
        self.placement.row_end = row.saturating_add(span);
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
