//! Badge widget tests

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::style::Style;
use revue::style::VisualStyle;
use revue::widget::traits::RenderContext;
use revue::widget::StyledView;
use revue::widget::View;
use revue::widget::{badge, dot_badge, Badge, BadgeShape, BadgeVariant};

// =============================================================================
// BadgeVariant tests
// =============================================================================

#[test]
fn test_variant_default_colors() {
    let (bg, fg) = BadgeVariant::Default.colors();
    assert_eq!(bg, Color::rgb(80, 80, 80));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_variant_primary_colors() {
    let (bg, fg) = BadgeVariant::Primary.colors();
    assert_eq!(bg, Color::rgb(50, 100, 200));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_variant_success_colors() {
    let (bg, fg) = BadgeVariant::Success.colors();
    assert_eq!(bg, Color::rgb(40, 160, 80));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_variant_warning_colors() {
    let (bg, fg) = BadgeVariant::Warning.colors();
    assert_eq!(bg, Color::rgb(200, 150, 40));
    assert_eq!(fg, Color::BLACK);
}

#[test]
fn test_variant_error_colors() {
    let (bg, fg) = BadgeVariant::Error.colors();
    assert_eq!(bg, Color::rgb(200, 60, 60));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_variant_info_colors() {
    let (bg, fg) = BadgeVariant::Info.colors();
    assert_eq!(bg, Color::rgb(60, 160, 180));
    assert_eq!(fg, Color::WHITE);
}

// =============================================================================
// Badge builder method tests
// =============================================================================

#[test]
fn test_badge_new() {
    let b = Badge::new("Test");
    // Verify it renders correctly
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_badge_variant_method() {
    let b = badge("Test").variant(BadgeVariant::Success);
    // Verify it renders with success colors
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(40, 160, 80)));
}

#[test]
fn test_badge_primary() {
    let b = badge("Test").primary();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(50, 100, 200)));
}

#[test]
fn test_badge_success() {
    let b = badge("OK").success();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(40, 160, 80)));
}

#[test]
fn test_badge_warning() {
    let b = badge("Warn").warning();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(200, 150, 40)));
    assert_eq!(cell.fg, Some(Color::BLACK));
}

#[test]
fn test_badge_error() {
    let b = badge("Error").error();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(200, 60, 60)));
}

#[test]
fn test_badge_info() {
    let b = badge("Info").info();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(60, 160, 180)));
}

#[test]
fn test_badge_shape_method() {
    let b = badge("Test").shape(BadgeShape::Pill);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    // Pill has 2 spaces padding
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
}

#[test]
fn test_badge_pill() {
    let b = badge("Tag").pill();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    // Pill has 2 spaces padding on each side
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
}

#[test]
fn test_badge_square() {
    let b = badge("Box").square();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    // Square has 1 space padding on each side
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'B');
}

#[test]
fn test_badge_bg() {
    let b = badge("Test").bg(Color::MAGENTA);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::MAGENTA));
}

#[test]
fn test_badge_fg() {
    let b = badge("Test").fg(Color::BLACK);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::BLACK));
}

#[test]
fn test_badge_colors() {
    let b = badge("Test").colors(Color::CYAN, Color::YELLOW);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::CYAN));
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

#[test]
fn test_badge_bold() {
    let b = badge("Test").bold();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_badge_max_width() {
    let b = badge("Test").max_width(10);
    // Verify it renders without panicking
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_badge_default() {
    let b = Badge::default();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

// =============================================================================
// Dot badge tests
// =============================================================================

#[test]
fn test_badge_dot() {
    let b = Badge::dot();
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
}

#[test]
fn test_badge_dot_with_variant() {
    let b = Badge::dot().success();
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::rgb(40, 160, 80)));
}

#[test]
fn test_badge_dot_with_custom_color() {
    let b = Badge::dot().bg(Color::MAGENTA);
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::MAGENTA));
}

// =============================================================================
// Helper function tests
// =============================================================================

#[test]
fn test_helper_badge() {
    let b = badge("Hi");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_helper_dot_badge() {
    let d = dot_badge();
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
}

// =============================================================================
// Render tests
// =============================================================================

#[test]
fn test_badge_render_default() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("Test");
    b.render(&mut ctx);

    // Check that badge renders with padding
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, ' '); // Padding
    assert_eq!(cell.bg, Some(Color::rgb(80, 80, 80))); // Default bg
    assert_eq!(cell.fg, Some(Color::WHITE)); // Default fg
}

#[test]
fn test_badge_render_with_text() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("NEW").primary();
    b.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("NEW"));
}

#[test]
fn test_badge_render_primary() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("Test").primary();
    b.render(&mut ctx);

    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.symbol, 'T');
    assert_eq!(cell.bg, Some(Color::rgb(50, 100, 200))); // Primary bg
    assert_eq!(cell.fg, Some(Color::WHITE)); // Primary fg
}

#[test]
fn test_badge_render_success() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("OK").success();
    b.render(&mut ctx);

    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(40, 160, 80))); // Success bg
}

#[test]
fn test_badge_render_warning() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("WARN").warning();
    b.render(&mut ctx);

    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(200, 150, 40))); // Warning bg
    assert_eq!(cell.fg, Some(Color::BLACK)); // Warning fg (black for contrast)
}

#[test]
fn test_badge_render_error() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("ERR").error();
    b.render(&mut ctx);

    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(200, 60, 60))); // Error bg
}

#[test]
fn test_badge_render_info() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("INFO").info();
    b.render(&mut ctx);

    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(60, 160, 180))); // Info bg
}

#[test]
fn test_badge_render_dot() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = dot_badge().success();
    b.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
    // Dot uses bg color as fg color
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::rgb(40, 160, 80))); // Success color
}

#[test]
fn test_badge_render_pill() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("Tag").pill();
    b.render(&mut ctx);

    // Pill has 2 spaces padding on each side
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, ' '); // Left padding
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.symbol, ' '); // Left padding
    let cell = buffer.get(2, 0).unwrap();
    assert_eq!(cell.symbol, 'T'); // Text
}

#[test]
fn test_badge_render_square() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("Box").square();
    b.render(&mut ctx);

    // Square has 1 space padding on each side
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, ' '); // Left padding
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.symbol, 'B'); // Text
}

#[test]
fn test_badge_render_bold() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("Bold").bold();
    b.render(&mut ctx);

    let cell = buffer.get(1, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_badge_render_custom_colors() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("Custom").colors(Color::CYAN, Color::YELLOW);
    b.render(&mut ctx);

    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::CYAN));
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

#[test]
fn test_badge_render_max_width() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Text is 10 chars, but max_width is 8
    let b = badge("LongText!").max_width(8);
    b.render(&mut ctx);

    // Should be truncated to 8 width
    let mut found_text = false;
    for x in 0..8 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol != ' ' {
                found_text = true;
                break;
            }
        }
    }
    assert!(found_text);
}

#[test]
fn test_badge_render_area_constraint() {
    let mut buffer = Buffer::new(20, 1);
    // Area is only 5 wide
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("VeryLongText");
    b.render(&mut ctx);

    // Should only render within area bounds
    let cell = buffer.get(4, 0).unwrap();
    assert!(cell.bg.is_some()); // Should have background
}

#[test]
fn test_badge_render_empty_text() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("");
    b.render(&mut ctx);

    // Should render just the background with padding
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert!(cell.bg.is_some());
}

// =============================================================================
// StyledView tests
// =============================================================================

#[test]
fn test_badge_css_id() {
    let b = badge("Test").element_id("my-badge");
    assert_eq!(View::id(&b), Some("my-badge"));

    let meta = b.meta();
    assert_eq!(meta.id, Some("my-badge".to_string()));
}

#[test]
fn test_badge_css_classes() {
    let b = badge("Test").class("primary").class("large");

    assert!(b.has_class("primary"));
    assert!(b.has_class("large"));
    assert!(!b.has_class("small"));

    let meta = b.meta();
    assert!(meta.classes.contains("primary"));
    assert!(meta.classes.contains("large"));
}

#[test]
fn test_badge_styled_view() {
    let mut b = badge("Test");

    b.set_id("test-id");
    assert_eq!(View::id(&b), Some("test-id"));

    b.add_class("active");
    assert!(b.has_class("active"));

    b.remove_class("active");
    assert!(!b.has_class("active"));

    b.toggle_class("selected");
    assert!(b.has_class("selected"));

    b.toggle_class("selected");
    assert!(!b.has_class("selected"));
}

#[test]
fn test_badge_css_colors_from_context() {
    let b = badge("CSS");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::RED,
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    b.render(&mut ctx);
}

#[test]
fn test_badge_inline_override_css() {
    let b = badge("Override").fg(Color::GREEN).bg(Color::YELLOW);

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::RED,
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    b.render(&mut ctx);

    // Inline styles should override CSS
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::GREEN));
    assert_eq!(cell.bg, Some(Color::YELLOW));
}

// =============================================================================
// BadgeShape tests
// =============================================================================

#[test]
fn test_badge_shape_default_is_rounded() {
    let b = Badge::new("Test");
    // Default shape is Rounded, which has 1 space padding
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    // Rounded has 1 space padding
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'T');
}

#[test]
fn test_all_shape_variants_render() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    // Test Rounded (default)
    let b = badge("Test");
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());

    // Clear buffer
    buffer = Buffer::new(20, 1);

    // Test Square
    let b = badge("Test").square();
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());

    // Clear buffer
    buffer = Buffer::new(20, 1);

    // Test Pill
    let b = badge("Test").pill();
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert!(buffer.get(0, 0).is_some());

    // Clear buffer
    buffer = Buffer::new(20, 1);

    // Test Dot
    let b = badge("Test").shape(BadgeShape::Dot);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
}

// =============================================================================
// Multi-byte character tests
// =============================================================================

#[test]
fn test_badge_render_unicode() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("日本語");
    b.render(&mut ctx);

    // Should render without panicking
    let cell = buffer.get(1, 0).unwrap();
    assert!(cell.bg.is_some());
}

#[test]
fn test_badge_render_emoji() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("🎉");
    b.render(&mut ctx);

    // Should render without panicking
    let cell = buffer.get(1, 0).unwrap();
    assert!(cell.bg.is_some());
}

// =============================================================================
// Chaining builder methods
// =============================================================================

#[test]
fn test_badge_builder_chain() {
    let b = badge("Chain").primary().pill().bold().max_width(15);

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);

    // Verify all chained properties applied
    let cell = buffer.get(2, 0).unwrap(); // Pill has 2 padding
    assert_eq!(cell.symbol, 'C');
    assert_eq!(cell.bg, Some(Color::rgb(50, 100, 200))); // Primary
    assert!(cell.modifier.contains(Modifier::BOLD)); // Bold
}

// =============================================================================
// BadgeShape Dot with all variants
// =============================================================================

#[test]
fn test_dot_badge_all_variants() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);

    // Test default dot
    let b = dot_badge();
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));

    // Test primary dot
    buffer = Buffer::new(5, 1);
    let b = dot_badge().primary();
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::rgb(50, 100, 200)));

    // Test error dot
    buffer = Buffer::new(5, 1);
    let b = dot_badge().error();
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::rgb(200, 60, 60)));
}

// =============================================================================

// =============================================================================
// Additional Edge Cases
// =============================================================================

#[test]
fn test_badge_very_long_text() {
    let long_text = "This is a very long badge text that exceeds normal width";
    let b = badge(long_text);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_badge_render_with_offset_area() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(10, 2, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let b = badge("Offset");
    b.render(&mut ctx);
}

#[test]
fn test_badge_render_very_narrow_area() {
    let mut buffer = Buffer::new(3, 1);
    let area = Rect::new(0, 0, 3, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let b = badge("Test");
    b.render(&mut ctx);
}

#[test]
fn test_badge_combine_variant_and_custom_colors() {
    let b = badge("Custom")
        .success()
        .colors(Color::MAGENTA, Color::YELLOW);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::MAGENTA));
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

#[test]
fn test_badge_empty_text_with_max_width() {
    let b = badge("").max_width(10);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_badge_unicode_max_width() {
    let b = badge("日本語テスト").max_width(10);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_badge_emoji_max_width() {
    let b = badge("🎉🎊🎁").max_width(5);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_badge_mixed_content() {
    let b = badge("Text123!@#");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_badge_multiple_renders() {
    let b = badge("Test");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        b.render(&mut ctx);
    }
}

#[test]
fn test_badge_variant_partial_eq() {
    assert_eq!(BadgeVariant::Default, BadgeVariant::Default);
    assert_eq!(BadgeVariant::Primary, BadgeVariant::Primary);
    assert_ne!(BadgeVariant::Default, BadgeVariant::Primary);
}

#[test]
fn test_badge_shape_partial_eq() {
    assert_eq!(BadgeShape::Rounded, BadgeShape::Rounded);
    assert_eq!(BadgeShape::Square, BadgeShape::Square);
    assert_eq!(BadgeShape::Pill, BadgeShape::Pill);
    assert_eq!(BadgeShape::Dot, BadgeShape::Dot);
    assert_ne!(BadgeShape::Rounded, BadgeShape::Square);
}

#[test]
fn test_badge_colors_override_variant() {
    let b = badge("Test").success().colors(Color::BLUE, Color::RED);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
    assert_eq!(cell.fg, Some(Color::RED));
}

#[test]
fn test_badge_variant_then_bg() {
    let b = badge("Test").success().bg(Color::CYAN);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::CYAN));
}

#[test]
fn test_badge_bg_then_variant() {
    let b = badge("Test").bg(Color::CYAN).success();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    // bg() before success() - CYAN is kept
    assert_eq!(cell.bg, Some(Color::CYAN));
}

#[test]
fn test_badge_zero_area() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let b = badge("Test");
    b.render(&mut ctx);
}

#[test]
fn test_badge_spaces_in_text() {
    let b = badge("  Spaces  ");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_badge_variant_enum_all_values() {
    let _ = BadgeVariant::Default;
    let _ = BadgeVariant::Primary;
    let _ = BadgeVariant::Success;
    let _ = BadgeVariant::Warning;
    let _ = BadgeVariant::Error;
    let _ = BadgeVariant::Info;
}

#[test]
fn test_badge_shape_enum_all_values() {
    let _ = BadgeShape::Rounded;
    let _ = BadgeShape::Square;
    let _ = BadgeShape::Pill;
    let _ = BadgeShape::Dot;
}

#[test]
fn test_badge_default_variant() {
    let (bg, fg) = BadgeVariant::Default.colors();
    assert_eq!(bg, Color::rgb(80, 80, 80));
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_badge_warning_has_black_fg() {
    let (bg, fg) = BadgeVariant::Warning.colors();
    assert_eq!(fg, Color::BLACK);
    assert!(bg.r > 150);
}

#[test]
fn test_badge_success_with_dark_bg() {
    let (bg, fg) = BadgeVariant::Success.colors();
    assert!(bg.g > 100 && bg.b < 100);
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_badge_error_with_red_bg() {
    let (bg, fg) = BadgeVariant::Error.colors();
    assert!(bg.r > 150 && bg.g < 100 && bg.b < 100);
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_badge_info_with_cyan_bg() {
    let (bg, fg) = BadgeVariant::Info.colors();
    assert!(bg.b > 100);
    assert_eq!(fg, Color::WHITE);
}

#[test]
fn test_badge_render_output_exists() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let b = badge("Test");
    b.render(&mut ctx);
    let mut has_content = false;
    for x in 0..20 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol != ' ' {
                has_content = true;
                break;
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_badge_text_content_preserved() {
    let text = "MyBadge";
    let b = badge(text);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    let rendered: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(rendered.contains("MyBadge"));
}

#[test]
fn test_badge_all_variants_render() {
    let variants = [
        BadgeVariant::Default,
        BadgeVariant::Primary,
        BadgeVariant::Success,
        BadgeVariant::Warning,
        BadgeVariant::Error,
        BadgeVariant::Info,
    ];

    for variant in variants {
        let b = match variant {
            BadgeVariant::Default => badge("Test"),
            BadgeVariant::Primary => badge("Test").primary(),
            BadgeVariant::Success => badge("Test").success(),
            BadgeVariant::Warning => badge("Test").warning(),
            BadgeVariant::Error => badge("Test").error(),
            BadgeVariant::Info => badge("Test").info(),
        };

        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        b.render(&mut ctx);
    }
}

#[test]
fn test_badge_all_shapes_with_padding() {
    let shapes = [BadgeShape::Rounded, BadgeShape::Square, BadgeShape::Pill];

    for shape in shapes {
        let b = badge("Test").shape(shape);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        b.render(&mut ctx);
    }
}

#[test]
fn test_badge_dot_with_all_variants() {
    let b = dot_badge().primary();
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
}

#[test]
fn test_badge_right_to_left_text() {
    let b = badge("مرحبا");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}

#[test]
fn test_badge_newline_in_text() {
    let b = badge("Line1\nLine2");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    b.render(&mut ctx);
}
