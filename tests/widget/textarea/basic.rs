use revue::widget::textarea;
#[test] fn test_textarea_new() { let t = textarea(); }
#[test] fn test_textarea_placeholder() { let t = textarea().placeholder("test"); }
