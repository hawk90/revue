use revue::widget::datagrid;
#[test]
fn test_datagrid_new() { let g = datagrid(); }
#[test]
fn test_datagrid_rows() { let g = datagrid().rows(10); }
