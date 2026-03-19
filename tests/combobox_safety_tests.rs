//! Combobox safety tests for edge cases
//!
//! Verifies bounds checking when options are empty or filtered to zero.

use revue::widget::combobox::Combobox;

#[test]
fn test_select_current_with_empty_options_no_panic() {
    let mut c = Combobox::new();
    let result = c.select_current();
    assert!(!result);
}

#[test]
fn test_select_current_with_empty_filtered_no_panic() {
    let mut c = Combobox::new().options(["Apple", "Banana"]);
    c.set_input("zzzzz");
    let result = c.select_current();
    assert!(!result);
}
