use revue::widget::filetree;
#[test] fn test_filetree_new() { let t = filetree("."); }
#[test] fn test_filetree_expand() { let mut t = filetree("."); t.expand("test"); }
