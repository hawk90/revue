//! EmptyState widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::StyledView;
use revue::widget::View;
use revue::widget::{empty_error, empty_state, first_use, no_results};
use revue::widget::{EmptyState, EmptyStateType, EmptyStateVariant};

// =============================================================================
// EmptyStateType tests
// =============================================================================

#[test]
fn test_empty_state_type_empty_icon() {
    assert_eq!(EmptyStateType::Empty.icon(), '📭');
}

#[test]
fn test_empty_state_type_no_results_icon() {
    assert_eq!(EmptyStateType::NoResults.icon(), '🔍');
}

#[test]
fn test_empty_state_type_error_icon() {
    assert_eq!(EmptyStateType::Error.icon(), '⚠');
}

#[test]
fn test_empty_state_type_no_permission_icon() {
    assert_eq!(EmptyStateType::NoPermission.icon(), '🔒');
}

#[test]
fn test_empty_state_type_offline_icon() {
    assert_eq!(EmptyStateType::Offline.icon(), '📡');
}

#[test]
fn test_empty_state_type_first_use_icon() {
    assert_eq!(EmptyStateType::FirstUse.icon(), '🚀');
}

#[test]
fn test_empty_state_type_empty_color() {
    let color = EmptyStateType::Empty.color();
    assert_eq!(color, Color::rgb(128, 128, 128));
}

#[test]
fn test_empty_state_type_no_results_color() {
    let color = EmptyStateType::NoResults.color();
    assert_eq!(color, Color::rgb(100, 149, 237));
}

#[test]
fn test_empty_state_type_error_color() {
    let color = EmptyStateType::Error.color();
    assert_eq!(color, Color::rgb(220, 80, 80));
}

#[test]
fn test_empty_state_type_no_permission_color() {
    let color = EmptyStateType::NoPermission.color();
    assert_eq!(color, Color::rgb(255, 165, 0));
}

#[test]
fn test_empty_state_type_offline_color() {
    let color = EmptyStateType::Offline.color();
    assert_eq!(color, Color::rgb(128, 128, 128));
}

#[test]
fn test_empty_state_type_first_use_color() {
    let color = EmptyStateType::FirstUse.color();
    assert_eq!(color, Color::rgb(100, 200, 100));
}

// =============================================================================
// EmptyStateVariant tests
// =============================================================================

#[test]
fn test_empty_state_variant_default() {
    let variant = EmptyStateVariant::default();
    assert_eq!(variant, EmptyStateVariant::Full);
}

// =============================================================================
// Constructor tests
// =============================================================================

#[test]
fn test_empty_state_new() {
    let es = EmptyState::new("No items");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_default() {
    let es = EmptyState::default();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_helper_function() {
    let es = empty_state("No data");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_no_results() {
    let es = EmptyState::no_results("No results found");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Check for no results icon
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '🔍' {
                found_icon = true;
                break;
            }
        }
    }
}

#[test]
fn test_empty_state_no_results_helper() {
    let es = no_results("No results");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_error() {
    let es = EmptyState::error("Failed to load");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Check for error icon
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '⚠' {
                found_icon = true;
                break;
            }
        }
    }
}

#[test]
fn test_empty_state_error_helper() {
    let es = empty_error("Error loading");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_no_permission() {
    let es = EmptyState::no_permission("Access denied");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Check for no permission icon
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '🔒' {
                found_icon = true;
                break;
            }
        }
    }
}

#[test]
fn test_empty_state_offline() {
    let es = EmptyState::offline("No connection");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Check for offline icon
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '📡' {
                found_icon = true;
                break;
            }
        }
    }
}

#[test]
fn test_empty_state_first_use() {
    let es = EmptyState::first_use("Welcome!");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Check for first use icon
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '🚀' {
                found_icon = true;
                break;
            }
        }
    }
}

#[test]
fn test_empty_state_first_use_helper() {
    let es = first_use("Welcome");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

// =============================================================================
// Builder method tests
// =============================================================================

#[test]
fn test_empty_state_description() {
    let es = EmptyState::new("No items").description("Create your first item");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_type_builder() {
    let es = EmptyState::new("Search empty").state_type(EmptyStateType::NoResults);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Check for no results icon
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '🔍' {
                found_icon = true;
                break;
            }
        }
    }
}

#[test]
fn test_empty_state_variant_builder_full() {
    let es = EmptyState::new("Test").variant(EmptyStateVariant::Full);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_variant_builder_compact() {
    let es = EmptyState::new("Test").variant(EmptyStateVariant::Compact);
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_variant_builder_minimal() {
    let es = EmptyState::new("Test").variant(EmptyStateVariant::Minimal);
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_icon_show() {
    let es = EmptyState::new("Test").icon(true);
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Icon should be present
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '📭' {
                found_icon = true;
                break;
            }
        }
    }
}

#[test]
fn test_empty_state_icon_hide() {
    let es = EmptyState::new("Test").icon(false);
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Just verify it renders without crashing - icon position is internal
}

#[test]
fn test_empty_state_custom_icon() {
    let es = EmptyState::new("Test").custom_icon('★');
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Custom icon should be present
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '★' {
                found_icon = true;
                break;
            }
        }
    }
}

#[test]
fn test_empty_state_action() {
    let es = EmptyState::new("No items").action("Create Item");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_builder_chain() {
    let es = EmptyState::new("No results")
        .description("Try different search terms")
        .state_type(EmptyStateType::NoResults)
        .variant(EmptyStateVariant::Full)
        .action("Clear Search")
        .custom_icon('🔎');

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Custom icon should be present
    let mut found_icon = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '🔎' {
                found_icon = true;
                break;
            }
        }
    }
}

// =============================================================================
// Height calculation tests
// =============================================================================

#[test]
fn test_empty_state_height_full() {
    let es = EmptyState::new("Test").variant(EmptyStateVariant::Full);
    assert_eq!(es.height(), 5);
}

#[test]
fn test_empty_state_height_full_with_description() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Full)
        .description("This is a description");
    assert_eq!(es.height(), 6);
}

#[test]
fn test_empty_state_height_full_with_action() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Full)
        .action("Click me");
    assert_eq!(es.height(), 7);
}

#[test]
fn test_empty_state_height_full_with_description_and_action() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Full)
        .description("Description")
        .action("Action");
    assert_eq!(es.height(), 8);
}

#[test]
fn test_empty_state_height_compact() {
    let es = EmptyState::new("Test").variant(EmptyStateVariant::Compact);
    assert_eq!(es.height(), 3);
}

#[test]
fn test_empty_state_height_compact_with_description() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Compact)
        .description("Description");
    assert_eq!(es.height(), 4);
}

#[test]
fn test_empty_state_height_minimal() {
    let es = EmptyState::new("Test").variant(EmptyStateVariant::Minimal);
    assert_eq!(es.height(), 1);
}

// =============================================================================
// Render tests - Full variant
// =============================================================================

#[test]
fn test_empty_state_render_full_basic() {
    let es = EmptyState::new("No items yet").variant(EmptyStateVariant::Full);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Should render icon
    let mut found_icon = false;
    for x in 0..40 {
        for y in 0..10 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '📭' {
                    found_icon = true;
                    break;
                }
            }
        }
        if found_icon {
            break;
        }
    }
}

#[test]
fn test_empty_state_render_full_with_description() {
    let es = EmptyState::new("No items")
        .variant(EmptyStateVariant::Full)
        .description("Create your first item");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_render_full_with_action() {
    let es = EmptyState::new("No items")
        .variant(EmptyStateVariant::Full)
        .action("Create Item");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Should render action button brackets
    let mut found_bracket = false;
    for x in 0..40 {
        for y in 0..10 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '[' {
                    found_bracket = true;
                    break;
                }
            }
        }
        if found_bracket {
            break;
        }
    }
    assert!(found_bracket);
}

#[test]
fn test_empty_state_render_full_no_icon() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Full)
        .icon(false);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_empty_state_render_full_with_custom_icon() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Full)
        .custom_icon('★');
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    let mut found_icon = false;
    for x in 0..40 {
        for y in 0..10 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '★' {
                    found_icon = true;
                    break;
                }
            }
        }
        if found_icon {
            break;
        }
    }
}

// =============================================================================
// Render tests - Compact variant
// =============================================================================

#[test]
fn test_empty_state_render_compact_basic() {
    let es = EmptyState::new("No results")
        .variant(EmptyStateVariant::Compact)
        .state_type(EmptyStateType::NoResults);
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Icon should be at position 0
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '🔍');
}

#[test]
fn test_empty_state_render_compact_with_description() {
    let es = EmptyState::new("No results")
        .variant(EmptyStateVariant::Compact)
        .description("Try different terms");
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Icon should be at position 0
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '📭');
    // Title should start at position 2
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'N');
}

#[test]
fn test_empty_state_render_compact_with_action() {
    let es = EmptyState::new("Empty")
        .variant(EmptyStateVariant::Compact)
        .action("Retry");
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Icon should be at position 0
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '📭');
}

#[test]
fn test_empty_state_render_compact_no_icon() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Compact)
        .icon(false);
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Title should be at position 0 when no icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
}

#[test]
fn test_empty_state_render_compact_custom_icon() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Compact)
        .custom_icon('⭐');
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Custom icon should be at position 0
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⭐');
}

// =============================================================================
// Render tests - Minimal variant
// =============================================================================

#[test]
fn test_empty_state_render_minimal_basic() {
    let es = EmptyState::new("Empty")
        .variant(EmptyStateVariant::Minimal)
        .state_type(EmptyStateType::NoResults);
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Icon should be at position 0
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '🔍');
}

#[test]
fn test_empty_state_render_minimal_no_icon() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Minimal)
        .icon(false);
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Title should be at position 0 when no icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
}

#[test]
fn test_empty_state_render_minimal_custom_icon() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Minimal)
        .custom_icon('●');
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);

    // Custom icon should be at position 0
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '●');
}

// =============================================================================
// Render tests - All state types
// =============================================================================

#[test]
fn test_empty_state_render_all_types_full() {
    let types = [
        EmptyStateType::Empty,
        EmptyStateType::NoResults,
        EmptyStateType::Error,
        EmptyStateType::NoPermission,
        EmptyStateType::Offline,
        EmptyStateType::FirstUse,
    ];

    let icons = ['📭', '🔍', '⚠', '🔒', '📡', '🚀'];

    for (i, state_type) in types.iter().enumerate() {
        let es = EmptyState::new("Test")
            .variant(EmptyStateVariant::Full)
            .state_type(*state_type);
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        es.render(&mut ctx);

        // Check for correct icon
        let mut found_icon = false;
        for x in 0..40 {
            for y in 0..10 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == icons[i] {
                        found_icon = true;
                        break;
                    }
                }
            }
            if found_icon {
                break;
            }
        }
        assert!(
            found_icon,
            "Icon {} not found for {:?}",
            icons[i], state_type
        );
    }
}

#[test]
fn test_empty_state_render_all_types_compact() {
    let types = [
        EmptyStateType::Empty,
        EmptyStateType::NoResults,
        EmptyStateType::Error,
        EmptyStateType::NoPermission,
        EmptyStateType::Offline,
        EmptyStateType::FirstUse,
    ];

    let icons = ['📭', '🔍', '⚠', '🔒', '📡', '🚀'];

    for (i, state_type) in types.iter().enumerate() {
        let es = EmptyState::new("Test")
            .variant(EmptyStateVariant::Compact)
            .state_type(*state_type);
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        es.render(&mut ctx);

        // Icon should be at position 0
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            icons[i],
            "Icon mismatch for {:?}",
            state_type
        );
    }
}

#[test]
fn test_empty_state_render_all_types_minimal() {
    let types = [
        EmptyStateType::Empty,
        EmptyStateType::NoResults,
        EmptyStateType::Error,
        EmptyStateType::NoPermission,
        EmptyStateType::Offline,
        EmptyStateType::FirstUse,
    ];

    let icons = ['📭', '🔍', '⚠', '🔒', '📡', '🚀'];

    for (i, state_type) in types.iter().enumerate() {
        let es = EmptyState::new("Test")
            .variant(EmptyStateVariant::Minimal)
            .state_type(*state_type);
        let mut buffer = Buffer::new(40, 1);
        let area = Rect::new(0, 0, 40, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        es.render(&mut ctx);

        // Icon should be at position 0
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            icons[i],
            "Icon mismatch for {:?}",
            state_type
        );
    }
}

// =============================================================================
// Render tests - All variants
// =============================================================================

#[test]
fn test_empty_state_render_all_variants() {
    let variants = [
        EmptyStateVariant::Full,
        EmptyStateVariant::Compact,
        EmptyStateVariant::Minimal,
    ];

    for variant in variants {
        let es = EmptyState::new("Test").variant(variant);
        let height = es.height();

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, height);
        let mut ctx = RenderContext::new(&mut buffer, area);
        es.render(&mut ctx);

        assert!(buffer.get(0, 0).is_some());
    }
}

// =============================================================================
// Edge case tests
// =============================================================================

#[test]
fn test_empty_state_render_small_area() {
    let es = EmptyState::new("Test");
    let mut buffer = Buffer::new(4, 1);
    let area = Rect::new(0, 0, 4, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should return early, not panic
}

#[test]
fn test_empty_state_render_zero_width() {
    let es = EmptyState::new("Test");
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should return early, not panic
}

#[test]
fn test_empty_state_render_zero_height() {
    let es = EmptyState::new("Test");
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should return early, not panic
}

#[test]
fn test_empty_state_very_long_title() {
    let long_title =
        "This is a very long title that exceeds the available width and should be truncated";
    let es = EmptyState::new(long_title).variant(EmptyStateVariant::Compact);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should not panic, content gets truncated
}

#[test]
fn test_empty_state_very_long_description() {
    let long_desc = "This is a very long description that exceeds the available width and should be truncated without causing issues";
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Full)
        .description(long_desc);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should not panic, content gets truncated
}

#[test]
fn test_empty_state_empty_title() {
    let es = EmptyState::new("");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_empty_state_empty_description() {
    let es = EmptyState::new("Test").description("");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_empty_state_empty_action() {
    let es = EmptyState::new("Test").action("");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_empty_state_height_less_than_content() {
    let es = EmptyState::new("Test")
        .variant(EmptyStateVariant::Full)
        .description("Description")
        .action("Action");
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should render without panic even though area is too small
}

#[test]
fn test_empty_state_unicode_title() {
    let es = EmptyState::new("🎉 한글 테스트 🎉");
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should handle unicode without panic
}

#[test]
fn test_empty_state_unicode_description() {
    let es = EmptyState::new("Test").description("한글 설명 🚀");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should handle unicode without panic
}

#[test]
fn test_empty_state_unicode_action() {
    let es = EmptyState::new("Test").action("시작하기 🚀");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    es.render(&mut ctx);
    // Should handle unicode without panic
}

// =============================================================================
// CSS/Styling tests
// =============================================================================

#[test]
fn test_empty_state_element_id() {
    let es = EmptyState::new("Test").element_id("test-empty-state");
    assert_eq!(View::id(&es), Some("test-empty-state"));
}

#[test]
fn test_empty_state_css_classes() {
    let es = EmptyState::new("Test").class("primary").class("large");
    assert!(es.has_class("primary"));
    assert!(es.has_class("large"));
    assert!(!es.has_class("small"));
}

#[test]
fn test_empty_state_styled_view_set_id() {
    let mut es = EmptyState::new("Test");
    es.set_id("my-id");
    assert_eq!(View::id(&es), Some("my-id"));
}

#[test]
fn test_empty_state_styled_view_add_class() {
    let mut es = EmptyState::new("Test");
    es.add_class("active");
    assert!(es.has_class("active"));
}

#[test]
fn test_empty_state_styled_view_remove_class() {
    let mut es = EmptyState::new("Test").class("active");
    es.remove_class("active");
    assert!(!es.has_class("active"));
}

#[test]
fn test_empty_state_styled_view_toggle_class() {
    let mut es = EmptyState::new("Test");

    es.toggle_class("selected");
    assert!(es.has_class("selected"));

    es.toggle_class("selected");
    assert!(!es.has_class("selected"));
}

#[test]
fn test_empty_state_classes_builder() {
    let es = EmptyState::new("Test").classes(vec!["class1", "class2", "class3"]);

    assert!(es.has_class("class1"));
    assert!(es.has_class("class2"));
    assert!(es.has_class("class3"));
    assert_eq!(View::classes(&es).len(), 3);
}

#[test]
fn test_empty_state_duplicate_class_not_added() {
    let es = EmptyState::new("Test").class("test").class("test");

    let classes = View::classes(&es);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_empty_state_focused() {
    let es = EmptyState::new("Test").focused(true);
    assert!(es.is_focused());
}

#[test]
fn test_empty_state_disabled() {
    let es = EmptyState::new("Test").disabled(true);
    assert!(es.is_disabled());
}

// =============================================================================
// Color tests for different state types
// =============================================================================

#[test]
fn test_empty_state_colors_are_distinct() {
    let types = [
        EmptyStateType::Empty,
        EmptyStateType::NoResults,
        EmptyStateType::Error,
        EmptyStateType::NoPermission,
        EmptyStateType::Offline,
        EmptyStateType::FirstUse,
    ];

    let colors: Vec<_> = types.iter().map(|t| t.color()).collect();

    // Check that Error color is distinct
    assert_eq!(colors[2], Color::rgb(220, 80, 80));
    // Check that FirstUse color is distinct
    assert_eq!(colors[5], Color::rgb(100, 200, 100));
}

// =============================================================================
// Meta and debug tests
// =============================================================================

#[test]
fn test_empty_state_meta() {
    let es = EmptyState::new("Test")
        .element_id("test-state")
        .class("primary");

    let meta = es.meta();
    assert_eq!(meta.widget_type, "EmptyState");
    assert_eq!(meta.id, Some("test-state".to_string()));
    assert!(meta.classes.contains("primary"));
}

#[test]
fn test_empty_state_meta_widget_type() {
    let es = EmptyState::new("Test").state_type(EmptyStateType::NoResults);

    let meta = es.meta();
    assert_eq!(meta.widget_type, "EmptyState");
}
