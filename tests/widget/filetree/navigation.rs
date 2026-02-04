use revue::widget::filetree;
#[test] fn test_filetree_collapse() { let mut t = filetree("."); t.expand("test"); t.collapse("test"); }
