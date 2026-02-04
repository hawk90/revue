use revue::widget::option_list;
#[test]
fn test_option_list_new() { let list = option_list(); }
#[test]
fn test_option_list_options() { let list = option_list().options(vec!["a".to_string()]); }
