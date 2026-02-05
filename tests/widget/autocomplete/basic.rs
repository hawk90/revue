use revue::widget::autocomplete;
#[test] fn test_autocomplete_new() { let a = autocomplete(); }
#[test] fn test_autocomplete_suggestions() { let a = autocomplete().suggestions(vec!["test".to_string()]); }
