use revue::widget::checkbox;
#[test] fn test_checkbox_new() { let c = checkbox(); }
#[test] fn test_checkbox_checked() { let c = checkbox().checked(true); }
