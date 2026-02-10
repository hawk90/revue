//! Tests for EmptyState widget
//!
//! Extracted from src/widget/display/empty_state.rs

use revue::prelude::*;

// =========================================================================
// EmptyStateType enum tests
// =========================================================================

#[test]
fn test_empty_state_type_icons() {
    assert_eq!(EmptyStateType::Empty.icon(), '📭');
    assert_eq!(EmptyStateType::NoResults.icon(), '🔍');
    assert_eq!(EmptyStateType::Error.icon(), '⚠');
    assert_eq!(EmptyStateType::NoPermission.icon(), '🔒');
    assert_eq!(EmptyStateType::Offline.icon(), '📡');
    assert_eq!(EmptyStateType::FirstUse.icon(), '🚀');
}

// =========================================================================
// EmptyState builder tests
// =========================================================================

#[test]
fn test_empty_state_type_helpers() {
    let _es = EmptyState::no_results("msg");
    let _es = EmptyState::error("msg");
    let _es = EmptyState::no_permission("msg");
    let _es = EmptyState::offline("msg");
    let _es = EmptyState::first_use("msg");
}

#[test]
fn test_empty_state_height() {
    let minimal = EmptyState::new("msg").variant(EmptyStateVariant::Minimal);
    assert_eq!(minimal.height(), 1);

    let compact = EmptyState::new("msg").variant(EmptyStateVariant::Compact);
    assert_eq!(compact.height(), 3);

    let compact_desc = EmptyState::new("msg")
        .variant(EmptyStateVariant::Compact)
        .description("desc");
    assert_eq!(compact_desc.height(), 4);

    let full = EmptyState::new("msg").variant(EmptyStateVariant::Full);
    assert_eq!(full.height(), 5);

    let full_with_action = EmptyState::new("msg")
        .variant(EmptyStateVariant::Full)
        .action("Click");
    assert_eq!(full_with_action.height(), 7);
}

#[test]
fn test_empty_state_custom_icon() {
    let es = EmptyState::new("Test").custom_icon('★');
    assert_eq!(es.get_icon(), '★');
}

#[test]
fn test_empty_state_helpers() {
    let es = empty_state("msg");
    // Can't access private title field
    // Just verify helper works

    let nr = no_results("search");
    // Can't access private state_type field
    // Just verify helper works

    let err = empty_error("error");
    // Can't access private state_type field
    // Just verify helper works

    let fu = first_use("welcome");
    // Can't access private state_type field
    // Just verify helper works
}

#[test]
fn test_empty_state_default() {
    let es = EmptyState::default();
    // Can't access private title field
    // Just verify Default implementation works
}

#[test]
fn test_empty_state_type_colors() {
    // Test all state type colors
    let _ = EmptyStateType::Empty.color();
    let _ = EmptyStateType::NoResults.color();
    let _ = EmptyStateType::Error.color();
    let _ = EmptyStateType::NoPermission.color();
    let _ = EmptyStateType::Offline.color();
    let _ = EmptyStateType::FirstUse.color();
}
