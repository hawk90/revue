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

    // Margin should leave space at start and end
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), None); // Margin
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
    assert_eq!(buffer.get(5, 0).map(|c| c.symbol), None);
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
    assert_eq!(buffer.get(15, 0).map(|c| c.symbol), None);
}

#[test]
fn test_divider_length_with_margin() {
    let d = divider().margin(2).length(5);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Margin 2, then 5 chars of line
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), None);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(7, 0).map(|c| c.symbol), None);
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

    // Should have margin before the line
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), None);
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), None);
}

#[test]
fn test_divider_vertical_with_margin() {
    let d = vdivider().margin(2);
    let mut buffer = Buffer::new(1, 20);
    let area = Rect::new(0, 0, 1, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    d.render(&mut ctx);

    // Margin at top
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), None);
    assert_eq!(buffer.get(0, 1).map(|c| c.symbol), None);
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '│');
    // Margin at bottom
    assert_eq!(buffer.get(0, 17).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 18).map(|c| c.symbol), None);
    assert_eq!(buffer.get(0, 19).map(|c| c.symbol), None);
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
    assert_eq!(buffer.get(0, 5).map(|c| c.symbol), None);
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
