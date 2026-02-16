//! Breadcrumb type tests

use revue::widget::breadcrumb::{BreadcrumbItem, SeparatorStyle};

// =============================================================================
// BreadcrumbItem Constructor Tests
// =============================================================================

#[test]
fn test_breadcrumb_item_new() {
    let item = BreadcrumbItem::new("Home");
    assert_eq!(item.label, "Home");
    assert_eq!(item.icon, None);
    assert!(item.clickable);
}

#[test]
fn test_breadcrumb_item_from_string() {
    let item = BreadcrumbItem::new(String::from("Documents"));
    assert_eq!(item.label, "Documents");
}

// =============================================================================
// BreadcrumbItem Builder Methods Tests
// =============================================================================

#[test]
fn test_breadcrumb_item_icon() {
    let item = BreadcrumbItem::new("Home").icon('🏠');
    assert_eq!(item.icon, Some('🏠'));
    assert_eq!(item.label, "Home");
}

#[test]
fn test_breadcrumb_item_clickable() {
    let item = BreadcrumbItem::new("Disabled").clickable(false);
    assert!(!item.clickable);
}

#[test]
fn test_breadcrumb_item_builder_chain() {
    let item = BreadcrumbItem::new("Work").icon('💼').clickable(true);

    assert_eq!(item.label, "Work");
    assert_eq!(item.icon, Some('💼'));
    assert!(item.clickable);
}

// =============================================================================
// SeparatorStyle Tests
// =============================================================================

#[test]
fn test_separator_style_default() {
    let style = SeparatorStyle::default();
    assert_eq!(style, SeparatorStyle::Slash);
}

#[test]
fn test_separator_style_equality() {
    assert_eq!(SeparatorStyle::Slash, SeparatorStyle::Slash);
    assert_ne!(SeparatorStyle::Slash, SeparatorStyle::Arrow);
    assert_ne!(SeparatorStyle::Chevron, SeparatorStyle::DoubleArrow);
}

#[test]
fn test_separator_style_clone() {
    let style1 = SeparatorStyle::Chevron;
    let style2 = style1.clone();
    assert_eq!(style1, style2);
}

#[test]
fn test_separator_style_debug() {
    let style = SeparatorStyle::Chevron;
    let debug_str = format!("{:?}", style);
    assert!(debug_str.contains("Chevron"));
}

#[test]
fn test_separator_style_all_variants() {
    let styles = [
        SeparatorStyle::Slash,
        SeparatorStyle::Arrow,
        SeparatorStyle::Chevron,
        SeparatorStyle::DoubleArrow,
        SeparatorStyle::Dot,
        SeparatorStyle::Pipe,
        SeparatorStyle::Custom('→'),
    ];

    // Test that all styles can be created and used
    for style in styles {
        let _bc = Breadcrumb::new().separator(style).push("Test");
    }
}

// =============================================================================
// SeparatorStyle::char() tests
// =============================================================================

#[test]
fn test_separator_style_char_slash() {
    assert_eq!(SeparatorStyle::Slash.char(), '/');
}

#[test]
fn test_separator_style_char_arrow() {
    assert_eq!(SeparatorStyle::Arrow.char(), '>');
}

#[test]
fn test_separator_style_char_chevron() {
    assert_eq!(SeparatorStyle::Chevron.char(), '›');
}

#[test]
fn test_separator_style_char_double_arrow() {
    assert_eq!(SeparatorStyle::DoubleArrow.char(), '»');
}

#[test]
fn test_separator_style_char_dot() {
    assert_eq!(SeparatorStyle::Dot.char(), '•');
}

#[test]
fn test_separator_style_char_pipe() {
    assert_eq!(SeparatorStyle::Pipe.char(), '|');
}

#[test]
fn test_separator_style_char_custom() {
    assert_eq!(SeparatorStyle::Custom('*').char(), '*');
}

// =============================================================================
// BreadcrumbItem edge cases
// =============================================================================

#[test]
fn test_breadcrumb_item_empty_label() {
    let item = BreadcrumbItem::new("");
    assert_eq!(item.label, "");
    assert!(item.clickable);
}

#[test]
fn test_breadcrumb_item_icon() {
    let item = BreadcrumbItem::new("Home").icon('H');
    assert_eq!(item.icon, Some('H'));
}

#[test]
fn test_breadcrumb_item_clickable_false() {
    let item = BreadcrumbItem::new("Current").clickable(false);
    assert!(!item.clickable);
}

#[test]
fn test_breadcrumb_item_builder_chain() {
    let item = BreadcrumbItem::new("Chain").icon('C').clickable(false);
    assert_eq!(item.label, "Chain");
    assert_eq!(item.icon, Some('C'));
    assert!(!item.clickable);
}

// =============================================================================
// BreadcrumbItem Clone and Debug Tests
// =============================================================================

#[test]
fn test_breadcrumb_item_clone() {
    let item1 = BreadcrumbItem::new("Test").icon('📁').clickable(true);

    let item2 = item1.clone();

    assert_eq!(item1.label, item2.label);
    assert_eq!(item1.icon, item2.icon);
    assert_eq!(item1.clickable, item2.clickable);
}

#[test]
fn test_breadcrumb_item_debug() {
    let item = BreadcrumbItem::new("Test").icon('📁');
    let debug_str = format!("{:?}", item);
    assert!(debug_str.contains("Test"));
}