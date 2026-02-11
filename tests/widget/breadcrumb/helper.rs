//! Breadcrumb helper function tests

use revue::widget::breadcrumb::{breadcrumb, crumb, Breadcrumb, BreadcrumbItem};

// =============================================================================
// Helper Function Tests
// =============================================================================

#[test]
fn test_breadcrumb_function() {
    let bc = breadcrumb();
    assert!(bc.is_empty());
}

#[test]
fn test_crumb_function() {
    let item = crumb("Home");
    assert_eq!(item.label, "Home");
}

#[test]
fn test_crumb_function_with_string() {
    let item = crumb("Folder".to_string());
    assert_eq!(item.label, "Folder");
}

#[test]
fn test_crumb_function_chainable() {
    let item = crumb("File").icon('📄');
    assert_eq!(item.label, "File");
    assert_eq!(item.icon, Some('📄'));
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_crumb_empty_label() {
    let item = crumb("");
    assert_eq!(item.label, "");
}

#[test]
fn test_crumb_long_label() {
    let long_label = "A".repeat(100);
    let item = crumb(&long_label);
    assert_eq!(item.label.len(), 100);
}

#[test]
fn test_crumb_with_special_chars() {
    let item = crumb("Path/To/File");
    assert_eq!(item.label, "Path/To/File");
}

#[test]
fn test_crumb_with_unicode() {
    let item = crumb("🏠 Home");
    assert_eq!(item.label, "🏠 Home");
}

#[test]
fn test_breadcrumb_multiple_times() {
    let bc1 = breadcrumb();
    let bc2 = breadcrumb();
    assert!(bc1.is_empty());
    assert!(bc2.is_empty());
}

// =============================================================================
// Helper Combination Tests
// =============================================================================

#[test]
fn test_helper_combination() {
    let bc = breadcrumb()
        .item(crumb("Home"))
        .item(crumb("Docs"))
        .push("Work");

    assert_eq!(bc.len(), 3);
}

#[test]
fn test_mixed_builder_patterns() {
    let bc = breadcrumb()
        .push("Home")
        .item(crumb("Documents").icon('📁'))
        .push("Work");

    assert_eq!(bc.len(), 3);
    assert_eq!(bc.items()[1].label, "Documents");
    assert_eq!(bc.items()[1].icon, Some('📁'));
}