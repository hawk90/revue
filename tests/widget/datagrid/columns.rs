use revue::widget::datagrid;
#[test]
fn test_datagrid_columns() { let g = datagrid().columns(5); }
#[test]
fn test_datagrid_add_column() { let mut g = datagrid(); g.add_column("col"); }
