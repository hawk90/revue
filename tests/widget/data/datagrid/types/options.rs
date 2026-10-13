//! DataGrid GridOptions tests

use revue::widget::data::datagrid::GridOptions;

#[test]
fn test_grid_options_new() {
    let options = GridOptions::new();
    assert!(options.show_header);
    assert!(!options.show_row_numbers);
    assert!(options.zebra);
}

#[test]
fn test_grid_options_debug_clone() {
    let options = GridOptions::default();
    let cloned = options.clone();
    assert_eq!(options.show_header, cloned.show_header);
    let _ = format!("{:?}", options);
}

#[test]
fn test_grid_options_default() {
    let opts = GridOptions::default();
    assert!(opts.show_header);
    assert!(!opts.show_row_numbers);
    assert!(!opts.multi_select);
    assert!(opts.zebra);
    assert!(opts.use_natural_sort);
    assert!(opts.virtual_scroll);
    assert_eq!(opts.row_height, 1);
    assert_eq!(opts.overscan, 5);
}
