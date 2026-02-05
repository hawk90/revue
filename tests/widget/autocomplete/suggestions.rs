use revue::widget::autocomplete;
#[test] fn test_autocomplete_update() { let mut a = autocomplete(); a.update(vec!["new".to_string()]); }
