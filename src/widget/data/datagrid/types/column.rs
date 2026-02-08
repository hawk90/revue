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

    /// Set visibility
    pub fn visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::data::datagrid::types::column_types::{Alignment, ColumnType};

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_grid_column_new_with_str() {
        let col = GridColumn::new("id", "ID");
        assert_eq!(col.key, "id");
        assert_eq!(col.title, "ID");
    }

    #[test]
    fn test_grid_column_new_with_string() {
        let col = GridColumn::new(String::from("name"), String::from("Name"));
        assert_eq!(col.key, "name");
        assert_eq!(col.title, "Name");
    }

    #[test]
    fn test_grid_column_new_defaults() {
        let col = GridColumn::new("test", "Test");
        assert_eq!(col.col_type, ColumnType::Text);
        assert_eq!(col.width, 0);
        assert_eq!(col.min_width, 5);
        assert_eq!(col.max_width, 50);
        assert!(col.sortable);
        assert!(col.filterable);
        assert!(!col.editable);
        assert!(col.visible);
        assert_eq!(col.align, Alignment::Left);
        assert!(col.resizable);
        assert!(!col.frozen);
    }

    // =========================================================================
    // Builder method tests - col_type
    // =========================================================================

    #[test]
    fn test_grid_column_col_type_text() {
        let col = GridColumn::new("test", "Test").col_type(ColumnType::Text);
        assert_eq!(col.col_type, ColumnType::Text);
    }

    #[test]
    fn test_grid_column_col_type_number() {
        let col = GridColumn::new("test", "Test").col_type(ColumnType::Number);
        assert_eq!(col.col_type, ColumnType::Number);
    }

    #[test]
    fn test_grid_column_col_type_date() {
        let col = GridColumn::new("test", "Test").col_type(ColumnType::Date);
        assert_eq!(col.col_type, ColumnType::Date);
    }

    #[test]
    fn test_grid_column_col_type_boolean() {
        let col = GridColumn::new("test", "Test").col_type(ColumnType::Boolean);
        assert_eq!(col.col_type, ColumnType::Boolean);
    }

    #[test]
    fn test_grid_column_col_type_custom() {
        let col = GridColumn::new("test", "Test").col_type(ColumnType::Custom);
        assert_eq!(col.col_type, ColumnType::Custom);
    }

    // =========================================================================
    // Builder method tests - width
    // =========================================================================

    #[test]
    fn test_grid_column_width() {
        let col = GridColumn::new("test", "Test").width(20);
        assert_eq!(col.width, 20);
    }

    #[test]
    fn test_grid_column_width_zero() {
        let col = GridColumn::new("test", "Test").width(0);
        assert_eq!(col.width, 0);
    }

    #[test]
    fn test_grid_column_width_max() {
        let col = GridColumn::new("test", "Test").width(u16::MAX);
        assert_eq!(col.width, u16::MAX);
    }

    #[test]
    fn test_grid_column_min_width() {
        let col = GridColumn::new("test", "Test").min_width(10);
        assert_eq!(col.min_width, 10);
    }

    #[test]
    fn test_grid_column_min_width_zero() {
        let col = GridColumn::new("test", "Test").min_width(0);
        assert_eq!(col.min_width, 0);
    }

    #[test]
    fn test_grid_column_max_width() {
        let col = GridColumn::new("test", "Test").max_width(100);
        assert_eq!(col.max_width, 100);
    }

    #[test]
    fn test_grid_column_max_width_u16_max() {
        let col = GridColumn::new("test", "Test").max_width(u16::MAX);
        assert_eq!(col.max_width, u16::MAX);
    }

    #[test]
    fn test_grid_column_width_chain() {
        let col = GridColumn::new("test", "Test")
            .width(30)
            .min_width(10)
            .max_width(50);
        assert_eq!(col.width, 30);
        assert_eq!(col.min_width, 10);
        assert_eq!(col.max_width, 50);
    }

    // =========================================================================
    // Builder method tests - sortable
    // =========================================================================

    #[test]
    fn test_grid_column_sortable_true() {
        let col = GridColumn::new("test", "Test").sortable(true);
        assert!(col.sortable);
    }

    #[test]
    fn test_grid_column_sortable_false() {
        let col = GridColumn::new("test", "Test").sortable(false);
        assert!(!col.sortable);
    }

    #[test]
    fn test_grid_column_sortable_default_is_true() {
        let col = GridColumn::new("test", "Test");
        assert!(col.sortable);
    }

    // =========================================================================
    // Builder method tests - editable
    // =========================================================================

    #[test]
    fn test_grid_column_editable_true() {
        let col = GridColumn::new("test", "Test").editable(true);
        assert!(col.editable);
    }

    #[test]
    fn test_grid_column_editable_false() {
        let col = GridColumn::new("test", "Test").editable(false);
        assert!(!col.editable);
    }

    #[test]
    fn test_grid_column_editable_default_is_false() {
        let col = GridColumn::new("test", "Test");
        assert!(!col.editable);
    }

    // =========================================================================
    // Builder method tests - align
    // =========================================================================

    #[test]
    fn test_grid_column_align_left() {
        let col = GridColumn::new("test", "Test").align(Alignment::Left);
        assert_eq!(col.align, Alignment::Left);
    }

    #[test]
    fn test_grid_column_align_center() {
        let col = GridColumn::new("test", "Test").align(Alignment::Center);
        assert_eq!(col.align, Alignment::Center);
    }

    #[test]
    fn test_grid_column_align_right() {
        let col = GridColumn::new("test", "Test").align(Alignment::Right);
        assert_eq!(col.align, Alignment::Right);
    }

    #[test]
    fn test_grid_column_right_shortcut() {
        let col = GridColumn::new("test", "Test").right();
        assert_eq!(col.align, Alignment::Right);
    }

    #[test]
    fn test_grid_column_center_shortcut() {
        let col = GridColumn::new("test", "Test").center();
        assert_eq!(col.align, Alignment::Center);
    }

    #[test]
    fn test_grid_column_align_default_is_left() {
        let col = GridColumn::new("test", "Test");
        assert_eq!(col.align, Alignment::Left);
    }

    // =========================================================================
    // Builder method tests - resizable
    // =========================================================================

    #[test]
    fn test_grid_column_resizable_true() {
        let col = GridColumn::new("test", "Test").resizable(true);
        assert!(col.resizable);
    }

    #[test]
    fn test_grid_column_resizable_false() {
        let col = GridColumn::new("test", "Test").resizable(false);
        assert!(!col.resizable);
    }

    #[test]
    fn test_grid_column_resizable_default_is_true() {
        let col = GridColumn::new("test", "Test");
        assert!(col.resizable);
    }

    // =========================================================================
    // Builder method tests - frozen
    // =========================================================================

    #[test]
    fn test_grid_column_frozen_true() {
        let col = GridColumn::new("test", "Test").frozen(true);
        assert!(col.frozen);
    }

    #[test]
    fn test_grid_column_frozen_false() {
        let col = GridColumn::new("test", "Test").frozen(false);
        assert!(!col.frozen);
    }

    #[test]
    fn test_grid_column_frozen_default_is_false() {
        let col = GridColumn::new("test", "Test");
        assert!(!col.frozen);
    }

    // =========================================================================
    // Clone tests
    // =========================================================================

    #[test]
    fn test_grid_column_clone() {
        let col1 = GridColumn::new("id", "ID")
            .col_type(ColumnType::Number)
            .width(15)
            .sortable(true)
            .align(Alignment::Right);

        let col2 = col1.clone();

        assert_eq!(col1.key, col2.key);
        assert_eq!(col1.title, col2.title);
        assert_eq!(col1.col_type, col2.col_type);
        assert_eq!(col1.width, col2.width);
        assert_eq!(col1.sortable, col2.sortable);
        assert_eq!(col1.align, col2.align);
    }

    #[test]
    fn test_grid_column_clone_independence() {
        let col1 = GridColumn::new("test", "Test").width(10);
        let mut col2 = col1.clone();

        col2.width = 20;

        assert_eq!(col1.width, 10);
        assert_eq!(col2.width, 20);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_grid_column_full_builder_chain() {
        let col = GridColumn::new("price", "Price")
            .col_type(ColumnType::Number)
            .width(15)
            .min_width(8)
            .max_width(30)
            .sortable(true)
            .editable(false)
            .align(Alignment::Right)
            .resizable(true)
            .frozen(false);

        assert_eq!(col.key, "price");
        assert_eq!(col.title, "Price");
        assert_eq!(col.col_type, ColumnType::Number);
        assert_eq!(col.width, 15);
        assert_eq!(col.min_width, 8);
        assert_eq!(col.max_width, 30);
        assert!(col.sortable);
        assert!(!col.editable);
        assert_eq!(col.align, Alignment::Right);
        assert!(col.resizable);
        assert!(!col.frozen);
    }

    #[test]
    fn test_grid_column_frozen_resizable_column() {
        let col = GridColumn::new("id", "ID")
            .frozen(true)
            .resizable(false)
            .width(10);

        assert!(col.frozen);
        assert!(!col.resizable);
        assert_eq!(col.width, 10);
    }

    // =========================================================================
    // Public field access tests
    // =========================================================================

    #[test]
    fn test_grid_column_public_fields_accessible() {
        let mut col = GridColumn::new("key", "Title")
            .col_type(ColumnType::Boolean)
            .width(25)
            .min_width(5)
            .max_width(40)
            .sortable(false)
            .editable(true)
            .align(Alignment::Center)
            .resizable(false)
            .frozen(true);

        // Set remaining fields directly
        col.filterable = false;
        col.visible = false;

        assert_eq!(col.key, "key");
        assert_eq!(col.title, "Title");
        assert_eq!(col.col_type, ColumnType::Boolean);
        assert_eq!(col.width, 25);
        assert_eq!(col.min_width, 5);
        assert_eq!(col.max_width, 40);
        assert!(!col.sortable);
        assert!(!col.filterable);
        assert!(col.editable);
        assert!(!col.visible);
        assert_eq!(col.align, Alignment::Center);
        assert!(!col.resizable);
        assert!(col.frozen);
    }
}
