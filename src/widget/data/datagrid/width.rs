//! DataGrid width calculation

use super::core::DataGrid;

impl DataGrid {
    /// Get display widths, using user-set widths if available
    pub(super) fn get_display_widths(&self, available: u16) -> Vec<u16> {
        if !self.column_widths.is_empty() {
            self.column_widths.clone()
        } else {
            self.calculate_widths(available)
        }
    }

    /// Calculate column widths
    pub(crate) fn calculate_widths(&self, available: u16) -> Vec<u16> {
        let visible_cols: Vec<_> = self.columns.iter().filter(|c| c.visible).collect();

        if visible_cols.is_empty() {
            return vec![];
        }

        let row_num_width = if self.options.show_row_numbers { 5 } else { 0 };
        let borders = visible_cols.len() as u16 + 1;
        let available = available.saturating_sub(row_num_width + borders);

        // Start with fixed or min widths
        let mut widths: Vec<u16> = visible_cols
            .iter()
            .map(|c| if c.width > 0 { c.width } else { c.min_width })
            .collect();

        let total: u16 = widths.iter().sum();

        if total < available {
            // Distribute extra space only to auto-width columns (width = 0)
            let extra = available - total;
            let auto_cols: Vec<_> = visible_cols
                .iter()
                .enumerate()
                .filter(|(_, c)| c.width == 0)
                .collect();

            if !auto_cols.is_empty() {
                let per_col = extra / auto_cols.len() as u16;
                for &(i, col) in &auto_cols {
                    let new_width = widths[i] + per_col;
                    widths[i] = new_width.min(col.max_width);
                }
            }
        }

        widths
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::data::datagrid::types::GridColumn;

    // =========================================================================
    // calculate_widths tests
    // =========================================================================

    #[test]
    fn test_calculate_widths_empty_columns() {
        let grid = DataGrid::new();
        let widths = grid.calculate_widths(100);
        assert!(widths.is_empty());
    }

    #[test]
    fn test_calculate_widths_single_column() {
        let grid = DataGrid::new().column(GridColumn::new("a", "A"));
        let widths = grid.calculate_widths(50);
        assert_eq!(widths.len(), 1);
        assert!(widths[0] >= 5); // At least min_width
    }

    #[test]
    fn test_calculate_widths_multiple_columns() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .column(GridColumn::new("b", "B"));
        let widths = grid.calculate_widths(50);
        assert_eq!(widths.len(), 2);
    }

    #[test]
    fn test_calculate_widths_with_fixed_widths() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A").width(20))
            .column(GridColumn::new("b", "B").width(30));
        let widths = grid.calculate_widths(100);
        assert_eq!(widths[0], 20);
        assert_eq!(widths[1], 30);
    }

    #[test]
    fn test_calculate_widths_with_min_widths() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A").min_width(10))
            .column(GridColumn::new("b", "B").min_width(15));
        let widths = grid.calculate_widths(50);
        assert!(widths[0] >= 10);
        assert!(widths[1] >= 15);
    }

    #[test]
    fn test_calculate_widths_respects_max_width() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A").max_width(20))
            .column(GridColumn::new("b", "B").max_width(20));
        let widths = grid.calculate_widths(200);
        assert!(widths[0] <= 20);
        assert!(widths[1] <= 20);
    }

    #[test]
    fn test_calculate_widths_distributes_extra_space() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A").width(10))
            .column(GridColumn::new("b", "B").width(10));
        // 10+10=20, available=50 after subtracting borders, extra=30 to distribute
        let widths = grid.calculate_widths(80);
        assert_eq!(widths[0], 10);
        assert_eq!(widths[1], 10);
    }

    #[test]
    fn test_calculate_widths_with_row_numbers() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .row_numbers(true);
        let widths = grid.calculate_widths(50);
        // Should account for row number column (5 chars)
        assert_eq!(widths.len(), 1);
    }

    #[test]
    fn test_calculate_widths_without_row_numbers() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .row_numbers(false);
        let widths = grid.calculate_widths(50);
        assert_eq!(widths.len(), 1);
    }

    #[test]
    fn test_calculate_widths_ignores_hidden_columns() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .column(GridColumn::new("b", "B").visible(false));
        let widths = grid.calculate_widths(50);
        assert_eq!(widths.len(), 1);
    }

    #[test]
    fn test_calculate_widths_all_columns_hidden() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A").visible(false))
            .column(GridColumn::new("b", "B").visible(false));
        let widths = grid.calculate_widths(50);
        assert!(widths.is_empty());
    }

    #[test]
    fn test_calculate_widths_saturating_available() {
        let grid = DataGrid::new().column(GridColumn::new("a", "A"));
        // Available width less than borders
        let widths = grid.calculate_widths(1);
        assert!(widths.len() == 1);
        assert!(widths[0] >= 5); // Still at least min_width
    }

    #[test]
    fn test_calculate_widths_u16_max() {
        let grid = DataGrid::new().column(GridColumn::new("a", "A"));
        let widths = grid.calculate_widths(u16::MAX);
        assert!(widths[0] > 0);
    }

    #[test]
    fn test_calculate_widths_zero_min_width() {
        let grid = DataGrid::new().column(GridColumn::new("a", "A").min_width(0));
        let widths = grid.calculate_widths(20);
        // Should still allocate some space
        assert!(widths[0] >= 5); // At least min_width
    }

    #[test]
    fn test_calculate_widths_mixed_fixed_and_auto() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A").width(20))
            .column(GridColumn::new("b", "B")) // Auto width
            .column(GridColumn::new("c", "C").width(15));
        let widths = grid.calculate_widths(100);
        assert_eq!(widths[0], 20);
        assert!(widths[1] >= 5); // At least min_width
        assert_eq!(widths[2], 15);
    }

    #[test]
    fn test_calculate_widths_equal_distribution() {
        let grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .column(GridColumn::new("b", "B"))
            .column(GridColumn::new("c", "C"));
        let widths = grid.calculate_widths(60);
        // All columns should get similar widths (borders subtracted)
        assert_eq!(widths.len(), 3);
        // After subtracting 4 for borders (len+1), and 0 for row numbers
        // 60 - 4 = 56, divided by 3 ≈ 18-19 per column
        for width in &widths {
            assert!(*width >= 16 && *width <= 20);
        }
    }

    // =========================================================================
    // get_display_widths tests
    // =========================================================================

    #[test]
    fn test_get_display_widths_no_user_widths() {
        let grid = DataGrid::new().column(GridColumn::new("a", "A"));
        let widths = grid.get_display_widths(50);
        assert!(!widths.is_empty());
    }

    #[test]
    fn test_get_display_widths_with_user_widths() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .column(GridColumn::new("b", "B"));
        grid.column_widths = vec![20, 30];
        let widths = grid.get_display_widths(100);
        assert_eq!(widths, vec![20, 30]);
    }

    #[test]
    fn test_get_display_widths_empty_user_widths() {
        let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));
        grid.column_widths = vec![];
        let widths = grid.get_display_widths(50);
        // Should fall back to calculate_widths
        assert!(!widths.is_empty());
    }

    #[test]
    fn test_get_display_widths_user_widths_cloned() {
        let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));
        grid.column_widths = vec![25];
        let widths1 = grid.get_display_widths(100);
        let widths2 = grid.get_display_widths(100);
        assert_eq!(widths1, widths2);
        assert_eq!(widths1, vec![25]);
    }

    #[test]
    fn test_get_display_widths_mismatched_column_count() {
        let mut grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .column(GridColumn::new("b", "B"));
        // User set only one width but we have two columns
        grid.column_widths = vec![20];
        let widths = grid.get_display_widths(100);
        // Returns user widths as-is (caller's responsibility to match)
        assert_eq!(widths, vec![20]);
    }
}
