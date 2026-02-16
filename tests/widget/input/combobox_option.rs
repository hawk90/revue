//! Tests for combobox/option.rs
//!
//! Extracted from src/widget/input/input_widgets/combobox/option.rs

use revue::widget::input::input_widgets::combobox::option::ComboOption;

// =========================================================================
// ComboOption::new tests
// =========================================================================

#[test]
fn test_combo_option_new() {
    let opt = ComboOption::new("Test");
    assert_eq!(opt.label, "Test");
    assert!(opt.value.is_none());
    assert!(!opt.disabled);
    assert!(opt.group.is_none());
}

#[test]
fn test_combo_option_new_str() {
    let opt = ComboOption::new("Hello");
    assert_eq!(opt.label, "Hello");
    assert_eq!(opt.get_value(), "Hello");
}

#[test]
fn test_combo_option_new_string() {
    let opt = ComboOption::new(String::from("Owned"));
    assert_eq!(opt.label, "Owned");
}

#[test]
fn test_combo_option_new_empty() {
    let opt = ComboOption::new("");
    assert_eq!(opt.label, "");
}

// =========================================================================
// ComboOption::value tests
// =========================================================================

#[test]
fn test_combo_option_value() {
    let opt = ComboOption::new("Label").value("actual_value");
    assert_eq!(opt.label, "Label");
    assert_eq!(opt.value, Some("actual_value".to_string()));
}

#[test]
fn test_combo_option_get_value_with_explicit() {
    let opt = ComboOption::new("Label").value("value");
    assert_eq!(opt.get_value(), "value");
}

#[test]
fn test_combo_option_get_value_without_explicit() {
    let opt = ComboOption::new("Label");
    assert_eq!(opt.get_value(), "Label");
}

#[test]
fn test_combo_option_value_chain() {
    let opt = ComboOption::new("A").value("a").value("b");
    // Builder pattern should override
    assert_eq!(opt.value, Some("b".to_string()));
}

// =========================================================================
// ComboOption::disabled tests
// =========================================================================

#[test]
fn test_combo_option_disabled() {
    let opt = ComboOption::new("Test").disabled(true);
    assert!(opt.disabled);
}

#[test]
fn test_combo_option_disabled_false() {
    let opt = ComboOption::new("Test").disabled(false);
    assert!(!opt.disabled);
}

#[test]
fn test_combo_option_disabled_chain() {
    let opt = ComboOption::new("Test").disabled(true).disabled(false);
    // Builder pattern should override
    assert!(!opt.disabled);
}

// =========================================================================
// ComboOption::group tests
// =========================================================================

#[test]
fn test_combo_option_group() {
    let opt = ComboOption::new("Test").group("Group1");
    assert_eq!(opt.group, Some("Group1".to_string()));
}

#[test]
fn test_combo_option_group_chain() {
    let opt = ComboOption::new("Test").group("A").group("B");
    // Builder pattern should override
    assert_eq!(opt.group, Some("B".to_string()));
}

// =========================================================================
// ComboOption::get_value tests
// =========================================================================

#[test]
fn test_combo_option_get_value_implicit() {
    let opt = ComboOption::new("Label");
    assert_eq!(opt.get_value(), "Label");
}

#[test]
fn test_combo_option_get_value_explicit() {
    let opt = ComboOption::new("Label").value("Value");
    assert_eq!(opt.get_value(), "Value");
}

#[test]
fn test_combo_option_get_value_empty_string() {
    let opt = ComboOption::new("").value("x");
    assert_eq!(opt.get_value(), "x");
}

// =========================================================================
// ComboOption full chain tests
// =========================================================================

#[test]
fn test_combo_option_full_chain() {
    let opt = ComboOption::new("Display")
        .value("value")
        .disabled(true)
        .group("Category");
    assert_eq!(opt.label, "Display");
    assert_eq!(opt.value, Some("value".to_string()));
    assert!(opt.disabled);
    assert_eq!(opt.group, Some("Category".to_string()));
}

// =========================================================================
// ComboOption public fields tests
// =========================================================================

#[test]
fn test_combo_option_public_fields() {
    let opt = ComboOption {
        label: "Label".to_string(),
        value: Some("Value".to_string()),
        disabled: true,
        group: Some("Group".to_string()),
    };
    assert_eq!(opt.label, "Label");
    assert_eq!(opt.value, Some("Value".to_string()));
    assert!(opt.disabled);
    assert_eq!(opt.group, Some("Group".to_string()));
}

#[test]
fn test_combo_option_public_fields_none() {
    let opt = ComboOption {
        label: "Label".to_string(),
        value: None,
        disabled: false,
        group: None,
    };
    assert!(opt.value.is_none());
    assert!(!opt.disabled);
    assert!(opt.group.is_none());
}

// =========================================================================
// ComboOption Clone tests
// =========================================================================

#[test]
fn test_combo_option_clone() {
    let opt1 = ComboOption::new("Test").value("val").group("grp");
    let opt2 = opt1.clone();
    assert_eq!(opt1.label, opt2.label);
    assert_eq!(opt1.value, opt2.value);
    assert_eq!(opt1.disabled, opt2.disabled);
    assert_eq!(opt1.group, opt2.group);
}

// =========================================================================
// From impl tests
// =========================================================================

#[test]
fn test_combo_option_from_str() {
    let opt: ComboOption = "Test".into();
    assert_eq!(opt.label, "Test");
    assert!(opt.value.is_none());
}

#[test]
fn test_combo_option_from_string() {
    let opt: ComboOption = String::from("Owned").into();
    assert_eq!(opt.label, "Owned");
    assert!(opt.value.is_none());
}

#[test]
fn test_combo_option_from_vs_new() {
    let opt1 = ComboOption::new("Test");
    let opt2: ComboOption = "Test".into();
    assert_eq!(opt1.label, opt2.label);
    assert_eq!(opt1.disabled, opt2.disabled);
}

// =========================================================================
// ComboOption Debug tests
// =========================================================================

#[test]
fn test_combo_option_debug() {
    let opt = ComboOption::new("Test");
    let debug_str = format!("{:?}", opt);
    assert!(debug_str.contains("ComboOption"));
    assert!(debug_str.contains("Test"));
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_combo_option_unicode_label() {
    let opt = ComboOption::new("选项");
    assert_eq!(opt.label, "选项");
    assert_eq!(opt.get_value(), "选项");
}

#[test]
fn test_combo_option_unicode_value() {
    let opt = ComboOption::new("Display").value("値");
    assert_eq!(opt.get_value(), "値");
}

#[test]
fn test_combo_option_unicode_group() {
    let opt = ComboOption::new("Test").group("グループ");
    assert_eq!(opt.group, Some("グループ".to_string()));
}

#[test]
fn test_combo_option_long_label() {
    let long_label = "A".repeat(1000);
    let opt = ComboOption::new(long_label.clone());
    assert_eq!(opt.label, long_label);
}

#[test]
fn test_combo_option_long_value() {
    let long_value = "x".repeat(1000);
    let opt = ComboOption::new("Display").value(long_value.clone());
    assert_eq!(opt.get_value(), long_value);
}

#[test]
fn test_combo_option_newline_in_label() {
    let opt = ComboOption::new("Line1\nLine2");
    assert_eq!(opt.label, "Line1\nLine2");
}

#[test]
fn test_combo_option_tab_in_label() {
    let opt = ComboOption::new("Tab\tSeparated");
    assert_eq!(opt.label, "Tab\tSeparated");
}

#[test]
fn test_combo_option_special_chars_label() {
    let opt = ComboOption::new("Test\"quotes\"");
    assert_eq!(opt.label, "Test\"quotes\"");
}

#[test]
fn test_combo_option_empty_label_with_value() {
    let opt = ComboOption::new("").value("value");
    assert_eq!(opt.label, "");
    assert_eq!(opt.get_value(), "value");
}
