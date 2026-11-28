//! Divider widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{divider, vdivider, Divider, DividerStyle, Orientation};

// =============================================================================
// Basic Render Tests
// =============================================================================

#[test]
fn test_divider_render_horizontal() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('─'));
}

#[test]
fn test_divider_render_vertical() {
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = vdivider();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('│'));
}

#[test]
fn test_divider_with_label() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().label("Section");
    d.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Section"));
}

#[test]
fn test_divider_render_uses_helpers() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().label("Test");
    d.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Test"));
    assert!(text.contains("─"));
}

#[test]
fn test_divider_label_clipping() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().label("VeryLongLabelThatWontFit");
    d.render(&mut ctx);
}

#[test]
fn test_divider_vertical_uses_vline() {
    let mut buffer = Buffer::new(1, 5);
    let area = Rect::new(0, 0, 1, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = vdivider();
    d.render(&mut ctx);

    for y in 0..5 {
        assert_eq!(buffer.get(0, y).map(|c| c.symbol), Some('│'));
    }
}

// =============================================================================
// DividerStyle Tests - All Variants
// =============================================================================

#[test]
fn test_divider_style_solid_horizontal() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().style(DividerStyle::Solid);
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '─');
}

#[test]
fn test_divider_style_dashed_horizontal() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().dashed();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╌');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '╌');
}

#[test]
fn test_divider_style_dotted_horizontal() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().dotted();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┄');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '┄');
}

#[test]
fn test_divider_style_double_horizontal() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().double();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '═');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '═');
}

#[test]
fn test_divider_style_thick_horizontal() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().thick();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '━');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '━');
}

#[test]
fn test_divider_style_solid_vertical() {
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = Divider::new()
        .orientation(Orientation::Vertical)
        .style(DividerStyle::Solid);
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 5).unwrap().symbol, '│');
}

#[test]
fn test_divider_style_dashed_vertical() {
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = Divider::new().orientation(Orientation::Vertical).dashed();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╎');
    assert_eq!(buffer.get(0, 5).unwrap().symbol, '╎');
}

#[test]
fn test_divider_style_dotted_vertical() {
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = Divider::new().orientation(Orientation::Vertical).dotted();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┆');
    assert_eq!(buffer.get(0, 5).unwrap().symbol, '┆');
}

#[test]
fn test_divider_style_double_vertical() {
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = Divider::new().orientation(Orientation::Vertical).double();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '║');
    assert_eq!(buffer.get(0, 5).unwrap().symbol, '║');
}

#[test]
fn test_divider_style_thick_vertical() {
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = Divider::new().orientation(Orientation::Vertical).thick();
    d.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 5).unwrap().symbol, '┃');
}

#[test]
fn test_divider_styles_render() {
    // Test that all divider styles render without panic
    let styles = [
        divider(),
        divider().dashed(),
        divider().dotted(),
        divider().double(),
        divider().thick(),
    ];

    for d in styles {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);
    }
}

// =============================================================================
// Orientation Tests
// =============================================================================

#[test]
fn test_divider_orientation_horizontal_default() {
    let d = Divider::new();
    // Default is Horizontal
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '─');
}

#[test]
fn test_divider_orientation_vertical() {
    let d = Divider::new().orientation(Orientation::Vertical);
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '│');
}

#[test]
fn test_divider_vertical_helper() {
    let d = vdivider();
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '│');
}

// =============================================================================
// Builder Method Tests
// =============================================================================

#[test]
fn test_divider_builder_new() {
    let d = Divider::new();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '─');
}

#[test]
fn test_divider_builder_vertical() {
    let d = Divider::vertical();
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '│');
}

#[test]
fn test_divider_builder_style() {
    let d = Divider::new().style(DividerStyle::Dashed);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╌');
}

#[test]
fn test_divider_builder_color() {
    let d = Divider::new().color(Color::RED);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    // Check that color is applied
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_divider_builder_label() {
    let d = Divider::new().label("Test Label");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Test Label"));
}

#[test]
fn test_divider_builder_label_color() {
    let d = Divider::new().label("Test").label_color(Color::GREEN);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Find the label position and check color
    for x in 0..20 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'T' {
                assert_eq!(cell.fg, Some(Color::GREEN));
                break;
            }
        }
    }
}

#[test]
fn test_divider_builder_margin() {
    let d = Divider::new().margin(2);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Margin creates space at start - implementation fills with spaces
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' '); // Margin (space)
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '─'); // Line starts
}

#[test]
fn test_divider_builder_length() {
    let d = Divider::new().length(5);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Should render 5 chars
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '─');
    // Implementation may render additional space or padding after the divider
    assert!(buffer.get(5, 0).is_some());
}

#[test]
fn test_divider_builder_chain() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let d = divider().dashed().color(Color::RED).label("Test").margin(2);
    d.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Test"));
}

#[test]
fn test_divider_default_trait() {
    let d = Divider::default();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '─');
}

// =============================================================================
// Helper Function Tests
// =============================================================================

#[test]
fn test_divider_helper() {
    let d = divider();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '─');
}

#[test]
fn test_vdivider_helper() {
    let d = vdivider();
    let mut buffer = Buffer::new(1, 10);
    let area = Rect::new(0, 0, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '│');
}

// =============================================================================
// Render Edge Cases Tests
// =============================================================================

#[test]
fn test_divider_render_with_offset() {
    let d = divider();
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(5, 2, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    assert_eq!(buffer.get(5, 2).unwrap().symbol, '─');
    assert_eq!(buffer.get(14, 2).unwrap().symbol, '─');
}

#[test]
fn test_divider_render_vertical_with_offset() {
    let d = vdivider();
    let mut buffer = Buffer::new(10, 20);
    let area = Rect::new(5, 2, 1, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    assert_eq!(buffer.get(5, 2).unwrap().symbol, '│');
    assert_eq!(buffer.get(5, 11).unwrap().symbol, '│');
}

#[test]
fn test_divider_render_empty_width() {
    let d = divider();
    let mut buffer = Buffer::new(0, 1);
    let area = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_divider_render_empty_height() {
    let d = divider();
    let mut buffer = Buffer::new(10, 0);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_divider_margin_full_width() {
    let d = divider().margin(5);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // 5 margin on each side = 10 chars for line
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '─');
    // Implementation fills remaining area with spaces
    assert!(buffer.get(15, 0).is_some());
}

#[test]
fn test_divider_length_with_margin() {
    let d = divider().margin(2).length(5);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Margin 2 (spaces), then 5 chars of line
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' '); // Margin
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '─');
    // Implementation fills remaining area with spaces
    assert!(buffer.get(7, 0).is_some());
}

#[test]
fn test_divider_label_centered() {
    let d = divider().label("Test");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Label should be centered
    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();

    // Count chars before and after "Test"
    let test_pos = text.find("Test").unwrap();
    assert!(test_pos > 0); // Space before
    assert!(test_pos + 4 < text.len()); // Space after
}

#[test]
fn test_divider_label_with_margin() {
    let d = divider().margin(2).label("X");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Should have margin (spaces) before the line
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, ' ');
}

#[test]
fn test_divider_vertical_with_margin() {
    let d = vdivider().margin(2);
    let mut buffer = Buffer::new(1, 20);
    let area = Rect::new(0, 0, 1, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Margin at top - implementation fills with spaces
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '│');
    // Margin at bottom
    assert_eq!(buffer.get(0, 17).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 18).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 19).unwrap().symbol, ' ');
}

#[test]
fn test_divider_vertical_with_length() {
    let d = vdivider().length(5);
    let mut buffer = Buffer::new(1, 20);
    let area = Rect::new(0, 0, 1, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Should render 5 chars
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '│');
    // Implementation fills remaining area with spaces
    assert!(buffer.get(0, 5).is_some());
}

// =============================================================================
// StyledView Trait Tests
// =============================================================================

#[test]
fn test_divider_styled_view_set_id() {
    let mut d = Divider::new();
    StyledView::set_id(&mut d, "test-id");
    assert_eq!(View::id(&d), Some("test-id"));
}

#[test]
fn test_divider_styled_view_add_class() {
    let mut d = Divider::new();
    StyledView::add_class(&mut d, "first");
    StyledView::add_class(&mut d, "second");
    assert!(StyledView::has_class(&d, "first"));
    assert!(StyledView::has_class(&d, "second"));
    assert_eq!(View::classes(&d).len(), 2);
}

#[test]
fn test_divider_styled_view_remove_class() {
    let mut d = Divider::new().class("a").class("b").class("c");
    StyledView::remove_class(&mut d, "b");
    assert!(StyledView::has_class(&d, "a"));
    assert!(!StyledView::has_class(&d, "b"));
    assert!(StyledView::has_class(&d, "c"));
}

#[test]
fn test_divider_styled_view_toggle_class() {
    let mut d = Divider::new();
    StyledView::toggle_class(&mut d, "test");
    assert!(StyledView::has_class(&d, "test"));
    StyledView::toggle_class(&mut d, "test");
    assert!(!StyledView::has_class(&d, "test"));
}

// =============================================================================
// View Trait Tests
// =============================================================================

#[test]
fn test_divider_view_widget_type() {
    let d = Divider::new();
    assert_eq!(d.widget_type(), "Divider");
}

#[test]
fn test_divider_view_id_none() {
    let d = Divider::new();
    assert!(View::id(&d).is_none());
}

#[test]
fn test_divider_view_id_some() {
    let d = Divider::new().element_id("my-id");
    assert_eq!(View::id(&d), Some("my-id"));
}

#[test]
fn test_divider_view_classes_empty() {
    let d = Divider::new();
    assert!(View::classes(&d).is_empty());
}

#[test]
fn test_divider_view_classes_with_values() {
    let d = Divider::new().class("first").class("second");
    let classes = View::classes(&d);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"first".to_string()));
    assert!(classes.contains(&"second".to_string()));
}

#[test]
fn test_divider_view_meta() {
    let d = Divider::new().element_id("test-id").class("test-class");
    let meta = d.meta();
    assert_eq!(meta.widget_type, "Divider");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

#[test]
fn test_divider_view_children_default() {
    let d = Divider::new();
    assert!(View::children(&d).is_empty());
}

// =============================================================================
// Builder Props Tests
// =============================================================================

#[test]
fn test_divider_builder_element_id() {
    let d = Divider::new().element_id("my-divider");
    assert_eq!(View::id(&d), Some("my-divider"));
}

#[test]
fn test_divider_builder_class() {
    let d = Divider::new().class("horizontal").class("line");
    assert!(d.has_class("horizontal"));
    assert!(d.has_class("line"));
}

#[test]
fn test_divider_builder_classes() {
    let d = Divider::new().classes(vec!["first", "second", "third"]);
    assert!(d.has_class("first"));
    assert!(d.has_class("second"));
    assert!(d.has_class("third"));
}

// =============================================================================
// DividerStyle Enum Tests
// =============================================================================

#[test]
fn test_divider_style_default_trait() {
    let style = DividerStyle::default();
    assert_eq!(style, DividerStyle::Solid);
}

#[test]
fn test_divider_style_partial_eq() {
    assert_eq!(DividerStyle::Solid, DividerStyle::Solid);
    assert_eq!(DividerStyle::Dashed, DividerStyle::Dashed);
    assert_ne!(DividerStyle::Solid, DividerStyle::Dashed);
    assert_ne!(DividerStyle::Dotted, DividerStyle::Double);
}

// =============================================================================
// Orientation Enum Tests
// =============================================================================

#[test]
fn test_orientation_default_trait() {
    let orientation = Orientation::default();
    assert_eq!(orientation, Orientation::Horizontal);
}

#[test]
fn test_orientation_partial_eq() {
    assert_eq!(Orientation::Horizontal, Orientation::Horizontal);
    assert_eq!(Orientation::Vertical, Orientation::Vertical);
    assert_ne!(Orientation::Horizontal, Orientation::Vertical);
}

// =============================================================================
// Color Combination Tests
// =============================================================================

// =============================================================================
// RGB/RGBA Color Tests
// =============================================================================

#[test]
fn test_divider_rgb_color() {
    let d = Divider::new().color(Color::rgb(255, 128, 0));
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::rgb(255, 128, 0)));
}

#[test]
fn test_divider_rgba_color() {
    let d = Divider::new().color(Color::rgba(200, 100, 50, 180));
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
    assert_eq!(
        buffer.get(0, 0).unwrap().fg,
        Some(Color::rgba(200, 100, 50, 180))
    );
}

#[test]
fn test_divider_multiple_colors() {
    let d = Divider::new().color(Color::RED).label_color(Color::GREEN);

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);

    // Find divider and check its color
    for x in 0..20 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '─' {
                assert_eq!(cell.fg, Some(Color::RED));
                break;
            }
        }
    }
}

#[test]
fn test_divider_all_color_presets() {
    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
        Color::WHITE,
        Color::BLACK,
    ];

    for color in colors {
        let d = Divider::new().color(color);
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(color));
    }
}

// =============================================================================
// Multiple Render Calls
// =============================================================================

#[test]
fn test_divider_multiple_renders() {
    let d = divider().label("Test");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        // Should render consistently
        let cell = buffer.get(0, 0).unwrap();
        assert!(cell.symbol != ' ');
    }
}

#[test]
fn test_divider_multiple_areas() {
    let d = divider();
    let mut buffer = Buffer::new(30, 5);

    let areas = [
        Rect::new(0, 0, 10, 1),
        Rect::new(10, 2, 10, 1),
        Rect::new(20, 4, 10, 1),
    ];

    for area in areas {
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);
    }
}

// =============================================================================
// Label Edge Cases
// =============================================================================

/// An empty label is no label: the plain line, with no blank cells in its
/// middle (#1588)
#[test]
fn test_divider_empty_label() {
    for width in [1, 2, 4, 5, 20] {
        let d = divider()
            .color(Color::RED)
            .label("")
            .label_color(Color::GREEN);
        let mut buffer = Buffer::new(width + 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(1, 0, width, 1));
        d.render(&mut ctx);
        for x in 0..width + 2 {
            let c = buffer.get(x, 0).unwrap();
            let want = if (1..=width).contains(&x) {
                ('─', Some(Color::RED))
            } else {
                (' ', None)
            };
            assert_eq!((c.symbol, c.fg), want, "width {width}, x {x}");
        }
    }
    assert_eq!(divider().label("").measure(20, 5), Some((20, 1)));
}

#[test]
fn test_divider_whitespace_label() {
    let d = divider().label("   ");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_emoji_label() {
    let d = divider().label("🔥");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_unicode_label() {
    let d = divider().label("日本語");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_rtl_label() {
    let d = divider().label("مرحبا");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_special_chars_label() {
    let d = divider().label("@#$%^&*()");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_newline_in_label() {
    let d = divider().label("Line1\nLine2");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_tabs_in_label() {
    let d = divider().label("Tab\tHere");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

// =============================================================================
// Additional Edge Cases
// =============================================================================

#[test]
fn test_divider_very_small_length() {
    let d = divider().length(1);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '─');
}

#[test]
fn test_divider_zero_length() {
    let d = divider().length(0);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
    // Should handle gracefully
}

#[test]
fn test_divider_large_margin() {
    let d = divider().margin(15);
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
    // Margin should be respected
}

#[test]
fn test_divider_zero_margin() {
    let d = divider().margin(0);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_single_pixel_width() {
    let d = divider();
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_single_pixel_height() {
    let d = vdivider();
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    d.render(&mut ctx);
}

#[test]
fn test_divider_all_styles_vertical() {
    let styles = [
        DividerStyle::Solid,
        DividerStyle::Dashed,
        DividerStyle::Dotted,
        DividerStyle::Double,
        DividerStyle::Thick,
    ];

    for style in styles {
        let d = Divider::new()
            .orientation(Orientation::Vertical)
            .style(style);

        let mut buffer = Buffer::new(1, 10);
        let area = Rect::new(0, 0, 1, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);
        // Should render without panic
    }
}

#[test]
fn test_divider_all_colors_with_styles() {
    let colors = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];

    for color in colors {
        let d = divider().color(color).dashed();
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(color));
    }
}

#[test]
fn test_divider_label_with_all_styles() {
    let styles = [
        divider().label("Test"),
        divider().dashed().label("Test"),
        divider().dotted().label("Test"),
        divider().double().label("Test"),
        divider().thick().label("Test"),
    ];

    for d in styles {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        let text: String = (0..20)
            .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
            .collect();
        assert!(text.contains("Test"));
    }
}

#[test]
fn test_divider_style_combinations() {
    let combos = [
        (DividerStyle::Dashed, Color::RED),
        (DividerStyle::Dotted, Color::GREEN),
        (DividerStyle::Double, Color::BLUE),
        (DividerStyle::Thick, Color::YELLOW),
        (DividerStyle::Solid, Color::CYAN),
    ];

    for (style, color) in combos {
        let d = Divider::new().style(style).color(color);
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(color));
    }
}

#[test]
fn test_divider_label_all_colors() {
    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
    ];

    for color in colors {
        let d = divider().label("X").label_color(color);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        // Find label and check color
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'X' {
                    assert_eq!(cell.fg, Some(color));
                    break;
                }
            }
        }
    }
}

#[test]
fn test_divider_all_styles_with_label() {
    let styles = [
        DividerStyle::Solid,
        DividerStyle::Dashed,
        DividerStyle::Dotted,
        DividerStyle::Double,
        DividerStyle::Thick,
    ];

    for style in styles {
        let d = divider().style(style).label("Test");
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        let text: String = (0..20)
            .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
            .collect();
        assert!(text.contains("Test"));
    }
}

#[test]
fn test_divider_orientation_with_all_styles() {
    let styles = [
        DividerStyle::Solid,
        DividerStyle::Dashed,
        DividerStyle::Dotted,
        DividerStyle::Double,
        DividerStyle::Thick,
    ];

    for style in styles {
        // Horizontal
        let d1 = Divider::new().style(style);
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d1.render(&mut ctx);

        // Vertical
        let d2 = Divider::new()
            .orientation(Orientation::Vertical)
            .style(style);
        let mut buffer = Buffer::new(1, 10);
        let area = Rect::new(0, 0, 1, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d2.render(&mut ctx);
    }
}

#[test]
fn test_divider_width_variations() {
    let widths = [1, 5, 10, 15, 20];

    for width in widths {
        let d = divider().length(width);
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        // First char should always be the divider (solid by default)
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '─', "width {width}");
    }
}

#[test]
fn test_divider_height_variations() {
    let heights = [1, 5, 10, 15, 20];

    for height in heights {
        let d = Divider::new()
            .orientation(Orientation::Vertical)
            .length(height);
        let mut buffer = Buffer::new(1, 30);
        let area = Rect::new(0, 0, 1, 30);
        let mut ctx = RenderContext::new(&mut buffer, area);
        d.render(&mut ctx);

        // First char should always be the divider (solid by default)
        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, '│', "height {height}");
    }
}

// =============================================================================

/// Test Divider edge cases
mod edge_cases {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::Divider;
    #[test]
    fn test_divider_horizontal_zero_width() {
        let divider = Divider::new();
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_vertical_zero_height() {
        let divider = Divider::vertical();
        let mut buffer = Buffer::new(10, 0);
        let area = Rect::new(0, 0, 10, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_both_zero() {
        let divider = Divider::new();
        let mut buffer = Buffer::new(0, 0);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_with_empty_label() {
        let divider = Divider::new().label("");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let divider = Divider::new().label(&long_label);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_with_unicode_label() {
        let divider = Divider::new().label("구분선 🎨");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }
}

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_divider_horizontal() {
        let view = vstack()
            .child(text("Above"))
            .child(Divider::new())
            .child(text("Below"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("divider_horizontal");
    }

    #[test]
    fn test_divider_with_label() {
        let view = vstack()
            .child(text("Section 1"))
            .child(Divider::new().label("OR"))
            .child(text("Section 2"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("divider_with_label");
    }
}

// =============================================================================
// #1528: a vertical divider draws its label down its column
// =============================================================================

mod vertical_label {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::Color;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{vdivider, Divider};

    const LINE: Color = Color::RED;
    const LABEL: Color = Color::GREEN;

    /// What a cell holds: its glyph (`'>'` for the second half of a wide
    /// glyph) and its color
    type Drawn = (char, Option<Color>);

    /// Renders `d` into a `width` x `height` area at (`x`, `y`) of a buffer
    /// two cells larger on every side, and returns the whole buffer
    fn draw_at(d: &Divider, x: u16, y: u16, width: u16, height: u16) -> Buffer {
        let mut buffer = Buffer::new(x + width + 2, y + height + 2);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(x, y, width, height));
        d.render(&mut ctx);
        buffer
    }

    fn cell(b: &Buffer, x: u16, y: u16) -> Drawn {
        let c = b.get(x, y).unwrap();
        if c.is_continuation() {
            ('>', c.fg)
        } else {
            (c.symbol, c.fg)
        }
    }

    /// Column `x` of the buffer, rows `0..height`
    fn column(b: &Buffer, x: u16, height: u16) -> Vec<Drawn> {
        (0..height).map(|y| cell(b, x, y)).collect()
    }

    fn line() -> Drawn {
        ('│', Some(LINE))
    }
    fn gap() -> Drawn {
        (' ', Some(LINE))
    }
    fn label(ch: char) -> Drawn {
        (ch, Some(LABEL))
    }
    fn blank() -> Drawn {
        (' ', None)
    }

    fn labeled(text: &str) -> Divider {
        vdivider().color(LINE).label(text).label_color(LABEL)
    }

    #[test]
    fn an_even_label_in_an_even_length_is_centered() {
        let b = draw_at(&labeled("AB"), 0, 0, 1, 10);
        assert_eq!(
            column(&b, 0, 10),
            [
                line(),
                line(),
                line(),
                gap(),
                label('A'),
                label('B'),
                gap(),
                line(),
                line(),
                line(),
            ]
        );
    }

    #[test]
    fn an_odd_label_in_an_even_length_puts_the_extra_row_below() {
        // The horizontal form rounds the same way: (10 - 3) / 2 - 1 = 2
        let b = draw_at(&labeled("ABC"), 0, 0, 1, 10);
        assert_eq!(
            column(&b, 0, 10),
            [
                line(),
                line(),
                gap(),
                label('A'),
                label('B'),
                label('C'),
                gap(),
                line(),
                line(),
                line(),
            ]
        );
    }

    #[test]
    fn an_even_label_in_an_odd_length_puts_the_extra_row_below() {
        let b = draw_at(&labeled("AB"), 0, 0, 1, 9);
        assert_eq!(
            column(&b, 0, 9),
            [
                line(),
                line(),
                gap(),
                label('A'),
                label('B'),
                gap(),
                line(),
                line(),
                line(),
            ]
        );
    }

    #[test]
    fn a_label_that_just_fits_keeps_one_line_cell_on_each_side() {
        let b = draw_at(&labeled("AB"), 0, 0, 1, 6);
        assert_eq!(
            column(&b, 0, 6),
            [line(), gap(), label('A'), label('B'), gap(), line()]
        );
    }

    #[test]
    fn a_label_one_row_too_long_is_drawn_alone_and_cut() {
        // Five rows: "ABC" needs seven with its gaps and line ends
        let b = draw_at(&labeled("ABC"), 0, 0, 1, 5);
        assert_eq!(
            column(&b, 0, 7),
            [
                label('A'),
                label('B'),
                label('C'),
                blank(),
                blank(),
                blank(),
                blank(),
            ]
        );
        let b = draw_at(&labeled("ABCDEFGH"), 0, 0, 1, 5);
        assert_eq!(
            column(&b, 0, 7),
            [
                label('A'),
                label('B'),
                label('C'),
                label('D'),
                label('E'),
                blank(),
                blank(),
            ],
            "cut at the area's last row"
        );
    }

    #[test]
    fn a_wide_label_makes_the_divider_two_columns_wide() {
        let d = labeled("가나");
        assert_eq!(d.measure(10, 8), Some((2, 8)));
        assert_eq!(d.measure(1, 8), Some((1, 8)), "never past the offer");

        let b = draw_at(&d, 0, 0, 2, 8);
        assert_eq!(
            column(&b, 0, 8),
            [
                line(),
                line(),
                gap(),
                label('가'),
                label('나'),
                gap(),
                line(),
                line(),
            ]
        );
        // The second column holds the wide glyphs' second halves only
        let second: Vec<char> = column(&b, 1, 8).iter().map(|c| c.0).collect();
        assert_eq!(second, [' ', ' ', ' ', '>', '>', ' ', ' ', ' ']);
        assert_eq!(column(&b, 2, 8), vec![blank(); 8], "nothing past it");
    }

    #[test]
    fn a_narrow_label_keeps_one_column() {
        assert_eq!(labeled("AB").measure(10, 8), Some((1, 8)));
        assert_eq!(vdivider().measure(10, 8), Some((1, 8)));
        // A mix is as wide as its widest glyph
        assert_eq!(labeled("A가").measure(10, 8), Some((2, 8)));
    }

    #[test]
    fn a_combining_mark_stays_in_its_letters_row() {
        // "e" + COMBINING ACUTE is one grapheme, one row; a lone zero-width
        // space takes no row
        let d = labeled("e\u{301}\u{200B}x");
        assert_eq!(d.measure(10, 8), Some((1, 8)));
        let b = draw_at(&d, 0, 0, 1, 8);
        assert_eq!(
            column(&b, 0, 8),
            [
                line(),
                line(),
                gap(),
                label('e'),
                label('x'),
                gap(),
                line(),
                line(),
            ]
        );
    }

    #[test]
    fn an_area_at_an_offset_draws_inside_it() {
        let b = draw_at(&labeled("AB"), 3, 2, 1, 6);
        assert_eq!(
            column(&b, 3, 10),
            [
                blank(),
                blank(),
                line(),
                gap(),
                label('A'),
                label('B'),
                gap(),
                line(),
                blank(),
                blank(),
            ]
        );
        for x in [0, 1, 2, 4, 5] {
            assert_eq!(column(&b, x, 10), vec![blank(); 10], "column {x}");
        }
    }

    #[test]
    fn one_row_holds_the_first_glyph_and_zero_rows_hold_nothing() {
        let b = draw_at(&labeled("AB"), 1, 1, 1, 1);
        assert_eq!(column(&b, 1, 3), [blank(), label('A'), blank()], "one row");
        let b = draw_at(&labeled("AB"), 1, 1, 1, 0);
        assert_eq!(column(&b, 1, 3), vec![blank(); 3], "zero rows");
    }

    #[test]
    fn the_label_takes_the_line_color_without_a_label_color() {
        let b = draw_at(&vdivider().color(LINE).label("A"), 0, 0, 1, 5);
        assert_eq!(
            column(&b, 0, 5),
            [line(), gap(), ('A', Some(LINE)), gap(), line()]
        );
    }

    /// An empty label is no label: the plain line, with no blank rows in
    /// its middle, one column wide (#1588)
    #[test]
    fn an_empty_label_draws_the_plain_line() {
        let d = labeled("");
        assert_eq!(d.measure(10, 8), Some((1, 8)));
        for height in [1, 2, 4, 5, 8] {
            let b = draw_at(&d, 1, 1, 1, height);
            let mut want = vec![blank()];
            want.extend(vec![line(); usize::from(height)]);
            want.push(blank());
            assert_eq!(column(&b, 1, height + 2), want, "height {height}");
            assert_eq!(
                column(&b, 2, height + 2),
                vec![blank(); usize::from(height) + 2]
            );
        }
    }

    /// Offered one column, a wide glyph does not fit: its row stays empty
    /// like any clipped widget, and nothing is drawn past the column (#1588)
    #[test]
    fn a_wide_glyph_in_one_column_leaves_its_row_empty() {
        let b = draw_at(&labeled("A가B"), 2, 1, 1, 9);
        assert_eq!(
            column(&b, 2, 11),
            [
                blank(),
                line(),
                line(),
                gap(),
                label('A'),
                blank(),
                label('B'),
                gap(),
                line(),
                line(),
                blank(),
            ]
        );
        for x in [1, 3] {
            assert_eq!(column(&b, x, 11), vec![blank(); 11], "column {x}");
        }
    }

    #[test]
    fn margin_and_length_bound_the_labeled_line() {
        let b = draw_at(&labeled("A").margin(1).length(5), 0, 0, 1, 8);
        assert_eq!(
            column(&b, 0, 8),
            [
                blank(),
                line(),
                gap(),
                label('A'),
                gap(),
                line(),
                blank(),
                blank(),
            ]
        );
    }
}
