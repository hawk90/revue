//! MultiSelectOption tests

use revue::widget::MultiSelectOption;

#[test]
fn test_multiselectoption_new() {
    let option = MultiSelectOption::new("Apple", "apple");

    assert_eq!(option.label, "Apple");
    assert_eq!(option.value, "apple");
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_new_with_string_types() {
    let option = MultiSelectOption::new(String::from("Label"), String::from("value"));

    assert_eq!(option.label, "Label");
    assert_eq!(option.value, "value");
}

#[test]
fn test_multiselectoption_simple() {
    let option = MultiSelectOption::simple("Apple");

    assert_eq!(option.label, "Apple");
    assert_eq!(option.value, "Apple");
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_disabled_true() {
    let option = MultiSelectOption::simple("Apple").disabled(true);
    assert!(option.disabled);
}

#[test]
fn test_multiselectoption_disabled_false() {
    let option = MultiSelectOption::simple("Apple").disabled(false);
    assert!(!option.disabled);
}

#[test]
fn test_multiselectoption_clone() {
    let option1 = MultiSelectOption::new("Apple", "apple").disabled(true);

    let option2 = option1.clone();

    assert_eq!(option1.label, option2.label);
    assert_eq!(option1.value, option2.value);
    assert_eq!(option1.disabled, option2.disabled);
}
