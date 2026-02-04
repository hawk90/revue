use revue::widget::textarea;
#[test] fn test_textarea_insert() { let mut t = textarea(); t.insert("test"); }
#[test] fn test_textarea_backspace() { let mut t = textarea().text("test"); t.backspace(); }
