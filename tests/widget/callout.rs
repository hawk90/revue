//! Callout widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{
    callout, danger, important, info_callout, note, tip, warning_callout, Callout, CalloutType,
    CalloutVariant,
};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_callout_new() {
    let c = Callout::new("Test content");
    assert!(!c.is_collapsible());
    assert!(c.is_expanded());
}

#[test]
fn test_callout_default() {
    let c = Callout::default();
    assert!(!c.is_collapsible());
    assert!(c.is_expanded());
}

#[test]
fn test_callout_helper() {
    let c = callout("Quick create");
    assert!(!c.is_collapsible());
    assert!(c.is_expanded());
}

// =============================================================================
// CalloutType Constructor Tests
// =============================================================================

#[test]
fn test_callout_note() {
    let c = Callout::note("Note content");
    assert!(c.is_expanded());
    // Verify by rendering - Note type has 📝 icon
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '📝');
}

#[test]
fn test_callout_tip() {
    let c = Callout::tip("Tip content");
    assert!(c.is_expanded());
    // Verify by rendering - Tip type has 💡 icon
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '💡');
}

#[test]
fn test_callout_important() {
    let c = Callout::important("Important content");
    assert!(c.is_expanded());
    // Verify by rendering - Important type has ❗ icon
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '❗');
}

#[test]
fn test_callout_warning() {
    let c = Callout::warning("Warning content");
    assert!(c.is_expanded());
    // Verify by rendering - Warning type has ⚠ icon
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '⚠');
}

#[test]
fn test_callout_danger() {
    let c = Callout::danger("Danger content");
    assert!(c.is_expanded());
    // Verify by rendering - Danger type has 🔴 icon
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '🔴');
}

#[test]
fn test_callout_info() {
    let c = Callout::info("Info content");
    assert!(c.is_expanded());
    // Verify by rendering - Info type has ℹ icon
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'ℹ');
}

// =============================================================================
// Helper Function Tests
// =============================================================================

#[test]
fn test_helper_callout() {
    let c = callout("Message");
    // Verify by rendering - default is Note type with 📝 icon
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '📝');
}

#[test]
fn test_helper_note() {
    let c = note("Note message");
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '📝');
}

#[test]
fn test_helper_tip() {
    let c = tip("Tip message");
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '💡');
}

#[test]
fn test_helper_important() {
    let c = important("Important message");
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '❗');
}

#[test]
fn test_helper_warning_callout() {
    let c = warning_callout("Warning message");
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '⚠');
}

#[test]
fn test_helper_danger() {
    let c = danger("Danger message");
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '🔴');
}

#[test]
fn test_helper_info_callout() {
    let c = info_callout("Info message");
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'ℹ');
}

// =============================================================================
// Builder Method Tests
// =============================================================================

#[test]
fn test_callout_builder_callout_type() {
    let c = Callout::new("Content").callout_type(CalloutType::Warning);
    // Verify by rendering - Warning has ⚠ icon
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '⚠');
}

#[test]
fn test_callout_builder_title() {
    let c = Callout::new("Content").title("Custom Title");
    // Verify by checking height - with title it's the same as without
    // The title replaces the default type title
    assert!(c.height() >= 2);
}

#[test]
fn test_callout_builder_variant_filled() {
    let c = Callout::new("Content").variant(CalloutVariant::Filled);
    // Filled variant has borders - height = 2 (borders) + 1 (title) + 1 (content) = 4
    assert_eq!(c.height(), 4);
}

#[test]
fn test_callout_builder_variant_left_border() {
    let c = Callout::new("Content").variant(CalloutVariant::LeftBorder);
    // LeftBorder variant has no top/bottom borders - height = 1 (title) + 1 (content) = 2
    assert_eq!(c.height(), 2);
}

#[test]
fn test_callout_builder_variant_minimal() {
    let c = Callout::new("Content").variant(CalloutVariant::Minimal);
    // Minimal variant has no borders - height = 1 (title) + 1 (content) = 2
    assert_eq!(c.height(), 2);
}

#[test]
fn test_callout_builder_icon_show() {
    let c = Callout::new("Content").icon(true);
    // Verify by rendering - icon should be visible
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '📝');
}

#[test]
fn test_callout_builder_icon_hide() {
    let c = Callout::new("Content").icon(false);
    // Verify by rendering - icon should not be visible at position 2
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_ne!(buffer.get(2, 0).unwrap().symbol, '📝');
}

#[test]
fn test_callout_builder_custom_icon() {
    let c = Callout::new("Content").custom_icon('★');
    // Verify by rendering - custom icon should be visible
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '★');
}

#[test]
fn test_callout_builder_collapsible() {
    let c = Callout::new("Content").collapsible(true);
    assert!(c.is_collapsible());
}

#[test]
fn test_callout_builder_collapsible_false() {
    let c = Callout::new("Content").collapsible(false);
    assert!(!c.is_collapsible());
}

#[test]
fn test_callout_builder_expanded() {
    let c = Callout::new("Content").expanded(true);
    assert!(c.is_expanded());
}

#[test]
fn test_callout_builder_collapsed() {
    let c = Callout::new("Content").expanded(false);
    assert!(!c.is_expanded());
}

#[test]
fn test_callout_builder_collapse_icons() {
    let c = Callout::new("Content")
        .collapsible(true)
        .collapse_icons('[', ']');
    // Verify by rendering when collapsed
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Should show expanded icon by default (▼ or custom])
    // Since we haven't collapsed it, it shows expanded icon
    assert_eq!(buffer.get(2, 0).unwrap().symbol, ']');
}

#[test]
fn test_callout_builder_chain() {
    let c = Callout::new("Content")
        .title("Custom")
        .callout_type(CalloutType::Tip)
        .variant(CalloutVariant::LeftBorder)
        .collapsible(true)
        .expanded(false)
        .custom_icon('💡')
        .collapse_icons('+', '-');

    assert!(c.is_collapsible());
    assert!(!c.is_expanded());

    // Verify by rendering - custom icon and tip type
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Should show custom icon
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '💡');
    // Should show collapsed icon
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '+');
}

// =============================================================================
// CalloutType Enum Tests
// =============================================================================

#[test]
fn test_callout_type_note_icon() {
    assert_eq!(CalloutType::Note.icon(), '📝');
}

#[test]
fn test_callout_type_tip_icon() {
    assert_eq!(CalloutType::Tip.icon(), '💡');
}

#[test]
fn test_callout_type_important_icon() {
    assert_eq!(CalloutType::Important.icon(), '❗');
}

#[test]
fn test_callout_type_warning_icon() {
    assert_eq!(CalloutType::Warning.icon(), '⚠');
}

#[test]
fn test_callout_type_danger_icon() {
    assert_eq!(CalloutType::Danger.icon(), '🔴');
}

#[test]
fn test_callout_type_info_icon() {
    assert_eq!(CalloutType::Info.icon(), 'ℹ');
}

#[test]
fn test_callout_type_accent_colors() {
    let note_accent = CalloutType::Note.accent_color();
    let tip_accent = CalloutType::Tip.accent_color();
    let important_accent = CalloutType::Important.accent_color();
    let warning_accent = CalloutType::Warning.accent_color();
    let danger_accent = CalloutType::Danger.accent_color();
    let info_accent = CalloutType::Info.accent_color();

    // Verify all accent colors are different
    assert_ne!(note_accent, tip_accent);
    assert_ne!(tip_accent, important_accent);
    assert_ne!(important_accent, warning_accent);
    assert_ne!(warning_accent, danger_accent);
    assert_ne!(danger_accent, info_accent);
}

#[test]
fn test_callout_type_bg_colors() {
    let note_bg = CalloutType::Note.bg_color();
    let tip_bg = CalloutType::Tip.bg_color();
    let important_bg = CalloutType::Important.bg_color();
    let warning_bg = CalloutType::Warning.bg_color();
    let danger_bg = CalloutType::Danger.bg_color();
    let info_bg = CalloutType::Info.bg_color();

    // Verify all bg colors are different
    assert_ne!(note_bg, tip_bg);
    assert_ne!(tip_bg, important_bg);
    assert_ne!(important_bg, warning_bg);
    assert_ne!(warning_bg, danger_bg);
    assert_ne!(danger_bg, info_bg);
}

#[test]
fn test_callout_type_title_colors() {
    let note_title = CalloutType::Note.title_color();
    let note_accent = CalloutType::Note.accent_color();
    assert_eq!(note_title, note_accent);
}

#[test]
fn test_callout_type_default_title_note() {
    assert_eq!(CalloutType::Note.default_title(), "Note");
}

#[test]
fn test_callout_type_default_title_tip() {
    assert_eq!(CalloutType::Tip.default_title(), "Tip");
}

#[test]
fn test_callout_type_default_title_important() {
    assert_eq!(CalloutType::Important.default_title(), "Important");
}

#[test]
fn test_callout_type_default_title_warning() {
    assert_eq!(CalloutType::Warning.default_title(), "Warning");
}

#[test]
fn test_callout_type_default_title_danger() {
    assert_eq!(CalloutType::Danger.default_title(), "Danger");
}

#[test]
fn test_callout_type_default_title_info() {
    assert_eq!(CalloutType::Info.default_title(), "Info");
}

#[test]
fn test_callout_type_default() {
    let default_type = CalloutType::default();
    assert_eq!(default_type, CalloutType::Note);
}

// =============================================================================
// Toggle Methods Tests
// =============================================================================

#[test]
fn test_callout_toggle() {
    let mut c = Callout::new("Content").collapsible(true);
    assert!(c.is_expanded());

    c.toggle();
    assert!(!c.is_expanded());

    c.toggle();
    assert!(c.is_expanded());
}

#[test]
fn test_callout_toggle_not_collapsible() {
    let mut c = Callout::new("Content").collapsible(false);
    assert!(c.is_expanded());

    c.toggle();
    // Should not change when not collapsible
    assert!(c.is_expanded());
}

#[test]
fn test_callout_toggle_multiple() {
    let mut c = Callout::new("Content").collapsible(true);

    for i in 0..10 {
        assert_eq!(c.is_expanded(), i % 2 == 0);
        c.toggle();
    }
    // After 10 toggles (even number), back to expanded
    assert!(c.is_expanded());
}

#[test]
fn test_callout_expand() {
    let mut c = Callout::new("Content").expanded(false);
    assert!(!c.is_expanded());

    c.expand();
    assert!(c.is_expanded());

    c.expand();
    assert!(c.is_expanded()); // Already expanded
}

#[test]
fn test_callout_collapse() {
    let mut c = Callout::new("Content");
    assert!(c.is_expanded());

    c.collapse();
    assert!(!c.is_expanded());

    c.collapse();
    assert!(!c.is_expanded()); // Already collapsed
}

#[test]
fn test_callout_is_expanded() {
    let c_expanded = Callout::new("Content").expanded(true);
    assert!(c_expanded.is_expanded());

    let c_collapsed = Callout::new("Content").expanded(false);
    assert!(!c_collapsed.is_expanded());
}

#[test]
fn test_callout_is_collapsible() {
    let c_collapsible = Callout::new("Content").collapsible(true);
    assert!(c_collapsible.is_collapsible());

    let c_not_collapsible = Callout::new("Content").collapsible(false);
    assert!(!c_not_collapsible.is_collapsible());
}

#[test]
fn test_callout_set_expanded() {
    let mut c = Callout::new("Content");
    assert!(c.is_expanded());

    c.set_expanded(false);
    assert!(!c.is_expanded());

    c.set_expanded(true);
    assert!(c.is_expanded());
}

#[test]
fn test_callout_set_expanded_same_value() {
    let mut c = Callout::new("Content").expanded(true);
    c.set_expanded(true);
    assert!(c.is_expanded());
}

// =============================================================================
// Height Query Tests
// =============================================================================

#[test]
fn test_callout_height_filled_single_line() {
    let c = Callout::new("Single line").variant(CalloutVariant::Filled);
    assert_eq!(c.height(), 4); // border + title + content + border
}

#[test]
fn test_callout_height_filled_multi_line() {
    let c = Callout::new("Line 1\nLine 2\nLine 3").variant(CalloutVariant::Filled);
    assert_eq!(c.height(), 6); // border + title + 3 content + border
}

#[test]
fn test_callout_height_left_border() {
    let c = Callout::new("Content").variant(CalloutVariant::LeftBorder);
    assert_eq!(c.height(), 2); // title + content
}

#[test]
fn test_callout_height_left_border_multi_line() {
    let c = Callout::new("Line 1\nLine 2").variant(CalloutVariant::LeftBorder);
    assert_eq!(c.height(), 3); // title + 2 content
}

#[test]
fn test_callout_height_minimal() {
    let c = Callout::new("Content").variant(CalloutVariant::Minimal);
    assert_eq!(c.height(), 2); // title + content
}

#[test]
fn test_callout_height_minimal_multi_line() {
    let c = Callout::new("Line 1\nLine 2\nLine 3").variant(CalloutVariant::Minimal);
    assert_eq!(c.height(), 4); // title + 3 content
}

#[test]
fn test_callout_height_collapsed() {
    let c = Callout::new("Content\nMore content")
        .collapsible(true)
        .expanded(false);
    assert_eq!(c.height(), 1); // Just header
}

#[test]
fn test_callout_height_collapsed_expanded() {
    let mut c = Callout::new("Line 1\nLine 2\nLine 3")
        .collapsible(true)
        .variant(CalloutVariant::Filled);

    c.collapse();
    assert_eq!(c.height(), 1);

    c.expand();
    assert_eq!(c.height(), 6); // border + title + 3 content + border
}

#[test]
fn test_callout_height_empty_content() {
    let c = Callout::new("").variant(CalloutVariant::Filled);
    // Empty content: lines().count() = 0, .max(1) = 1
    // height = 2 (borders) + 1 (title) + 1 (content min) = 4
    assert_eq!(c.height(), 4);
}

// =============================================================================
// Key Handling Tests
// =============================================================================

#[test]
fn test_callout_handle_key_enter() {
    let mut c = Callout::new("Content").collapsible(true);
    assert!(c.is_expanded());

    let handled = c.handle_key(&Key::Enter);
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_callout_handle_key_space() {
    let mut c = Callout::new("Content").collapsible(true);
    assert!(c.is_expanded());

    let handled = c.handle_key(&Key::Char(' '));
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_callout_handle_key_right() {
    let mut c = Callout::new("Content").collapsible(true).expanded(false);
    assert!(!c.is_expanded());

    let handled = c.handle_key(&Key::Right);
    assert!(handled);
    assert!(c.is_expanded());
}

#[test]
fn test_callout_handle_key_left() {
    let mut c = Callout::new("Content").collapsible(true);
    assert!(c.is_expanded());

    let handled = c.handle_key(&Key::Left);
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_callout_handle_key_char_l() {
    let mut c = Callout::new("Content").collapsible(true).expanded(false);
    assert!(!c.is_expanded());

    let handled = c.handle_key(&Key::Char('l'));
    assert!(handled);
    assert!(c.is_expanded());
}

#[test]
fn test_callout_handle_key_char_h() {
    let mut c = Callout::new("Content").collapsible(true);
    assert!(c.is_expanded());

    let handled = c.handle_key(&Key::Char('h'));
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_callout_handle_key_unhandled() {
    let mut c = Callout::new("Content").collapsible(true);
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Up));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Down));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Char('x')));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Tab));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Escape));
    assert!(c.is_expanded());
}

#[test]
fn test_callout_handle_key_not_collapsible() {
    let mut c = Callout::new("Content").collapsible(false);
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Enter));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Char(' ')));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Right));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Left));
    assert!(c.is_expanded());
}

#[test]
fn test_callout_handle_key_disabled() {
    let mut c = Callout::new("Content").collapsible(true).disabled(true);
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Enter));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Char(' ')));
    assert!(c.is_expanded());

    assert!(!c.handle_key(&Key::Right));
    assert!(c.is_expanded()); // Right key doesn't work when disabled

    assert!(!c.handle_key(&Key::Left));
    assert!(c.is_expanded());
}

#[test]
fn test_callout_handle_key_multiple_toggles() {
    let mut c = Callout::new("Content").collapsible(true);

    for i in 0..10 {
        assert!(c.handle_key(&Key::Enter));
        // After each toggle, state alternates (starting from expanded=true)
        // i=0: after toggle, expanded=false (0)
        // i=1: after toggle, expanded=true (1)
        assert_eq!(c.is_expanded(), i % 2 == 1);
    }
}

// =============================================================================
// Render Tests - Filled Variant
// =============================================================================

#[test]
fn test_callout_render_filled() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Test content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Check left accent border
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '┃');
}

#[test]
fn test_callout_render_filled_with_title() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::tip("Content")
        .title("Custom Tip")
        .variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Check title is rendered
    let title_cell = buffer.get(4, 0).unwrap();
    assert_eq!(title_cell.symbol, 'C');
    assert!(title_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_callout_render_filled_with_icon() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::warning("Content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Check warning icon
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '⚠');
}

#[test]
fn test_callout_render_filled_without_icon() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Content")
        .icon(false)
        .variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Icon should not be rendered at position 2
    assert_ne!(buffer.get(2, 0).unwrap().symbol, '📝');
}

#[test]
fn test_callout_render_filled_custom_icon() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::info("Content")
        .custom_icon('★')
        .variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Custom icon should be rendered
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '★');
}

#[test]
fn test_callout_render_filled_multi_line_content() {
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::danger("Line 1\nLine 2\nLine 3").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Check border on multiple lines
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 3).unwrap().symbol, '┃');
}

#[test]
fn test_callout_render_filled_colors() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Check background color is set
    let bg_cell = buffer.get(1, 0).unwrap();
    assert!(bg_cell.bg.is_some());

    // Check accent border color
    let border_cell = buffer.get(0, 0).unwrap();
    assert!(border_cell.fg.is_some());
}

// =============================================================================
// Render Tests - LeftBorder Variant
// =============================================================================

#[test]
fn test_callout_render_left_border() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::tip("Test content").variant(CalloutVariant::LeftBorder);
    c.render(&mut ctx);

    // Check left accent border
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '┃');
}

#[test]
fn test_callout_render_left_border_with_icon() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::important("Content").variant(CalloutVariant::LeftBorder);
    c.render(&mut ctx);

    // Check important icon
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '❗');
}

#[test]
fn test_callout_render_left_border_multi_line() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Line 1\nLine 2").variant(CalloutVariant::LeftBorder);
    c.render(&mut ctx);

    // Check border on all lines
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '┃');
}

#[test]
fn test_callout_render_left_border_no_background() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::warning("Content").variant(CalloutVariant::LeftBorder);
    c.render(&mut ctx);

    // LeftBorder variant should not have background
    let content_cell = buffer.get(2, 1).unwrap();
    assert_eq!(content_cell.bg, None);
}

// =============================================================================
// Render Tests - Minimal Variant
// =============================================================================

#[test]
fn test_callout_render_minimal() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::info("Content").variant(CalloutVariant::Minimal);
    c.render(&mut ctx);

    // Check icon at position 0
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
}

#[test]
fn test_callout_render_minimal_with_title() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Content")
        .title("Custom Note")
        .variant(CalloutVariant::Minimal);
    c.render(&mut ctx);

    // Check icon and title
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '📝');
    let title_cell = buffer.get(2, 0).unwrap();
    assert_eq!(title_cell.symbol, 'C');
    assert!(title_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_callout_render_minimal_without_icon() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::tip("Content")
        .icon(false)
        .variant(CalloutVariant::Minimal);
    c.render(&mut ctx);

    // Content should start at position 0 when no icon
    let content_cell = buffer.get(0, 1).unwrap();
    assert_eq!(content_cell.symbol, 'C');
}

#[test]
fn test_callout_render_minimal_multi_line() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::warning("Line 1\nLine 2\nLine 3").variant(CalloutVariant::Minimal);
    c.render(&mut ctx);

    // Check icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⚠');
}

// =============================================================================
// Render Tests - Collapsible Behavior
// =============================================================================

#[test]
fn test_callout_render_collapsed() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Hidden content")
        .collapsible(true)
        .expanded(false)
        .variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Only header should be rendered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    // Check collapse icon
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '▶');
}

#[test]
fn test_callout_render_collapsed_expanded() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::tip("Visible content")
        .collapsible(true)
        .expanded(true)
        .variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Content should be visible
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    // Check expand icon
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '▼');
}

#[test]
fn test_callout_render_custom_collapse_icons() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::important("Content")
        .collapsible(true)
        .expanded(false)
        .collapse_icons('[', ']')
        .variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Check custom collapse icon
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '[');
}

#[test]
fn test_callout_render_custom_expand_icons() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::danger("Content")
        .collapsible(true)
        .expanded(true)
        .collapse_icons('[', ']')
        .variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Check custom expand icon
    assert_eq!(buffer.get(2, 0).unwrap().symbol, ']');
}

// =============================================================================
// Render Tests - Edge Cases
// =============================================================================

#[test]
fn test_callout_render_too_small_width() {
    let mut buffer = Buffer::new(3, 5);
    let area = Rect::new(0, 0, 3, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Content");
    c.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_callout_render_too_small_height() {
    let mut buffer = Buffer::new(50, 0);
    let area = Rect::new(0, 0, 50, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Content");
    c.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_callout_render_long_title() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Content")
        .title("This is a very long title that should be truncated")
        .variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Should render without panicking
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
}

#[test]
fn test_callout_render_long_content() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let long_line = "This is a very long content line that should be truncated to fit";
    let c = Callout::note(long_line).variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Should render without panicking
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
}

#[test]
fn test_callout_render_content_clipped() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Line 1\nLine 2\nLine 3\nLine 4\nLine 5").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Should clip content to available area
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
}

#[test]
fn test_callout_render_empty_content() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    // Should render header and borders
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
}

// =============================================================================
// Render Tests - All Callout Types
// =============================================================================

#[test]
fn test_callout_render_note() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::note("Note content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    assert_eq!(buffer.get(2, 0).unwrap().symbol, '📝');
}

#[test]
fn test_callout_render_tip() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::tip("Tip content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    assert_eq!(buffer.get(2, 0).unwrap().symbol, '💡');
}

#[test]
fn test_callout_render_important() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::important("Important content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    assert_eq!(buffer.get(2, 0).unwrap().symbol, '❗');
}

#[test]
fn test_callout_render_warning() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::warning("Warning content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    assert_eq!(buffer.get(2, 0).unwrap().symbol, '⚠');
}

#[test]
fn test_callout_render_danger() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::danger("Danger content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    assert_eq!(buffer.get(2, 0).unwrap().symbol, '🔴');
}

#[test]
fn test_callout_render_info() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::info("Info content").variant(CalloutVariant::Filled);
    c.render(&mut ctx);

    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'ℹ');
}

// =============================================================================
// CSS/Styling Tests
// =============================================================================

#[test]
fn test_callout_css_id() {
    let c = Callout::new("Content").element_id("my-callout");
    assert_eq!(View::id(&c), Some("my-callout"));
}

#[test]
fn test_callout_css_classes() {
    let c = Callout::new("Content").class("primary").class("highlight");

    assert!(c.has_class("primary"));
    assert!(c.has_class("highlight"));
    assert!(!c.has_class("secondary"));
}

#[test]
fn test_callout_styled_view_set_id() {
    let mut c = Callout::new("Content");
    c.set_id("test-id");
    assert_eq!(View::id(&c), Some("test-id"));
}

#[test]
fn test_callout_styled_view_add_class() {
    let mut c = Callout::new("Content");
    c.add_class("active");
    assert!(c.has_class("active"));
}

#[test]
fn test_callout_styled_view_remove_class() {
    let mut c = Callout::new("Content").class("active");
    c.remove_class("active");
    assert!(!c.has_class("active"));
}

#[test]
fn test_callout_styled_view_toggle_class() {
    let mut c = Callout::new("Content");

    c.toggle_class("selected");
    assert!(c.has_class("selected"));

    c.toggle_class("selected");
    assert!(!c.has_class("selected"));
}

#[test]
fn test_callout_classes_builder() {
    let c = Callout::new("Content").classes(vec!["class1", "class2", "class3"]);

    assert!(c.has_class("class1"));
    assert!(c.has_class("class2"));
    assert!(c.has_class("class3"));
    assert_eq!(View::classes(&c).len(), 3);
}

#[test]
fn test_callout_focused() {
    let c = Callout::new("Content").focused(true);
    assert!(c.is_focused());
}

#[test]
fn test_callout_disabled() {
    let c = Callout::new("Content").disabled(true);
    assert!(c.is_disabled());
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_callout_full_expand_collapse_cycle() {
    let mut c = Callout::note("Test content")
        .collapsible(true)
        .variant(CalloutVariant::Filled);

    // Initial state
    assert!(c.is_expanded());
    assert_eq!(c.height(), 4);

    // Collapse
    c.collapse();
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);

    // Expand
    c.expand();
    assert!(c.is_expanded());
    assert_eq!(c.height(), 4);
}

#[test]
fn test_callout_keyboard_navigation() {
    let mut c = Callout::tip("Content")
        .collapsible(true)
        .variant(CalloutVariant::Filled);

    // Collapse with Enter
    assert!(c.handle_key(&Key::Enter));
    assert!(!c.is_expanded());

    // Expand with Right
    assert!(c.handle_key(&Key::Right));
    assert!(c.is_expanded());

    // Collapse with Left
    assert!(c.handle_key(&Key::Left));
    assert!(!c.is_expanded());

    // Toggle with Space
    assert!(c.handle_key(&Key::Char(' ')));
    assert!(c.is_expanded());
}

#[test]
fn test_callout_builder_with_all_options() {
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Callout::warning("Line 1\nLine 2\nLine 3")
        .title("Custom Warning")
        .variant(CalloutVariant::Filled)
        .collapsible(true)
        .expanded(true)
        .custom_icon('⚡')
        .collapse_icons('[', ']')
        .element_id("warning-callout")
        .class("important");

    c.render(&mut ctx);

    // Verify rendering
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');

    // Verify state
    assert!(c.is_expanded());
    assert!(c.is_collapsible());
    // Verify custom icon and custom expand icon
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '⚡');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, ']');
    assert_eq!(View::id(&c), Some("warning-callout"));
    assert!(c.has_class("important"));
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_callout_empty_title() {
    let c = Callout::note("Content").title("");
    // Empty title is valid - should use empty string instead of default
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_callout_unicode_title() {
    let c = Callout::note("Content").title("한글 제목");
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Should render unicode without crashing
}

#[test]
fn test_callout_unicode_content() {
    let c = Callout::note("한글 내용\nEmoji 🎉");
    // Verify height calculation handles unicode
    assert!(c.height() >= 3);
}

#[test]
fn test_callout_special_characters_in_content() {
    let c = Callout::tip("Tab:\tTab\nNewline:\nNewline");
    // Lines: "Tab:\tTab", "Newline:", "Newline" = 3 lines
    // height = 2 (borders) + 1 (title) + 3 (content) = 6 for Filled variant (default)
    assert_eq!(c.height(), 6);
}

#[test]
fn test_callout_very_long_single_line() {
    let long_line = "A".repeat(1000);
    let c = Callout::note(&long_line).variant(CalloutVariant::Filled);
    assert!(c.height() >= 3);
}

#[test]
fn test_callout_many_empty_lines() {
    let c = Callout::important("\n\n\n\n").variant(CalloutVariant::Filled);
    // 5 empty strings from split
    assert!(c.height() >= 3);
}

#[test]
fn test_callout_toggle_then_check_height() {
    let mut c = Callout::danger("Line 1\nLine 2\nLine 3")
        .collapsible(true)
        .variant(CalloutVariant::Filled);

    assert_eq!(c.height(), 6);

    c.collapse();
    assert_eq!(c.height(), 1);

    c.expand();
    assert_eq!(c.height(), 6);
}

#[test]
fn test_callout_disabled_toggle_methods() {
    let mut c = Callout::warning("Content").collapsible(true).disabled(true);

    // Disabled state doesn't prevent toggle methods (only handle_key)
    c.toggle();
    assert!(!c.is_expanded());

    c.expand();
    assert!(c.is_expanded());

    c.collapse();
    assert!(!c.is_expanded());
}

#[test]
fn test_callout_not_collapsible_toggle_methods() {
    let mut c = Callout::info("Content").collapsible(false);

    // Not collapsible - toggle should have no effect
    c.toggle();
    assert!(c.is_expanded());

    c.collapse();
    assert!(!c.is_expanded()); // collapse() still works directly

    c.expand();
    assert!(c.is_expanded());
}
