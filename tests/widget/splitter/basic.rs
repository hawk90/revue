use revue::widget::splitter;
#[test]
fn test_splitter_new() { let s = splitter(); }
#[test]
fn test_splitter_ratio() { let s = splitter().ratio(0.5); }
