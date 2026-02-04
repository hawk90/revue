use revue::widget::callout;
#[test] fn test_callout_new() { let c = callout("test"); }
#[test] fn test_callout_with_content() { let c = callout("test").content("content"); }
