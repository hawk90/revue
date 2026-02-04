use revue::widget::input;
#[test] fn test_input_new() { let i = input(); }
#[test] fn test_input_placeholder() { let i = input().placeholder("test"); }
