//! Grid column definition

use super::column_types::{Alignment, ColumnType};

/// Grid column definition
#[derive(Clone)]
pub struct GridColumn {
    /// Column key/id
    pub key: String,
    /// Display title
    pub title: String,
    /// Column type
    pub col_type: ColumnType,
    /// Width in cells (default 0 = auto)
    ///
    /// An auto-width column starts at `min_width` and shares the free space
    /// up to `max_width`; space a capped column cannot take goes to the
    /// other auto-width columns. A set width (> 0) gets no share of the free space
    /// and is clamped into `[min_width, max_width]`.
    pub width: u16,
    /// Minimum width in cells (default 5)
    ///
    /// When it is larger than a nonzero `max_width`, the minimum wins.
    pub min_width: u16,
    /// Maximum width in cells (default 50; 0 = no limit)
    ///
    /// It caps auto-width growth, a set `width` and resizing. A
    /// `min_width` larger than it wins.
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

    /// Set width (0 = auto)
    ///
    /// The drawn width is clamped into `[min_width, max_width]`: with the
    /// default maximum 50, `.width(80)` draws 50 unless `max_width` is
    /// raised or set to 0 (no limit).
    pub fn width(mut self, w: u16) -> Self {
        self.width = w;
        self
    }

    /// Set min width (default 5)
    ///
    /// A minimum larger than a nonzero `max_width` wins.
    pub fn min_width(mut self, w: u16) -> Self {
        self.min_width = w;
        self
    }

    /// Set max width (default 50; 0 = no limit)
    ///
    /// A `min_width` larger than the maximum wins.
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

    /// Set filterable (a non-filterable column's values never match the
    /// grid filter)
    pub fn filterable(mut self, f: bool) -> Self {
        self.filterable = f;
        self
    }

    /// Set frozen (stays visible during horizontal scroll)
    ///
    /// A frozen column is pinned to the left together with every column
    /// before it, like `DataGrid::freeze_columns_left`.
    pub fn frozen(mut self, f: bool) -> Self {
        self.frozen = f;
        self
    }

    /// Set visibility
    pub fn visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }

    /// Clamp `w` into `[min_width, max_width]`, as CSS does: `max_width`
    /// 0 is no limit, and when `min_width` is larger than the maximum the
    /// minimum wins
    pub(crate) fn clamp_width(&self, w: u16) -> u16 {
        let capped = if self.max_width > 0 {
            w.min(self.max_width)
        } else {
            w
        };
        capped.max(self.min_width)
    }

    /// The width a column starts at before free space is shared: its set
    /// width clamped into range, or `min_width` for an auto-width column
    /// (clamping 0 gives `min_width`)
    pub(crate) fn base_width(&self) -> u16 {
        self.clamp_width(self.width)
    }
}
