use revue::widget::combobox;
#[test] fn test_combobox_new() { let c = combobox(); }
#[test] fn test_combobox_options() { let c = combobox().options(vec!["a".to_string()]); }
