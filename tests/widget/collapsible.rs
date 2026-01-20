//! Collapsible widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView};
use revue::widget::{collapsible, Collapsible, View};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_collapsible_new() {
    let c = Collapsible::new("Test Title");
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);
}

#[test]
fn test_collapsible_default() {
    let c = Collapsible::default();
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);
}

#[test]
fn test_collapsible_helper() {
    let c = collapsible("Quick Create");
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);
}

// =============================================================================
// Builder Methods Tests
// =============================================================================

#[test]
fn test_collapsible_content() {
    let c = Collapsible::new("Info")
        .content("Line 1\nLine 2\nLine 3")
        .expanded(true)
        .border(true);
    // 3 content lines + header + border = 5
    assert_eq!(c.height(), 5);
}

#[test]
fn test_collapsible_content_empty() {
    let c = Collapsible::new("Info")
        .content("")
        .expanded(true)
        .border(true);
    // No content, just header + bottom border = 2
    assert_eq!(c.height(), 2);
}

#[test]
fn test_collapsible_line() {
    let c = Collapsible::new("Info")
        .line("First")
        .line("Second")
        .line("Third")
        .expanded(true)
        .border(true);
    // 3 content lines + header + border = 5
    assert_eq!(c.height(), 5);
}

#[test]
fn test_collapsible_lines() {
    let c = Collapsible::new("Info")
        .lines(&["A", "B", "C", "D"])
        .expanded(true)
        .border(true);
    // 4 content lines + header + border = 6
    assert_eq!(c.height(), 6);
}

#[test]
fn test_collapsible_lines_empty() {
    let c = Collapsible::new("Info")
        .lines(&[])
        .expanded(true)
        .border(true);
    // No content, just header + bottom border = 2
    assert_eq!(c.height(), 2);
}

#[test]
fn test_collapsible_expanded() {
    let c = Collapsible::new("Info").expanded(true);
    assert!(c.is_expanded());

    let c2 = Collapsible::new("Info").expanded(false);
    assert!(!c2.is_expanded());
}

#[test]
fn test_collapsible_icons() {
    let c = Collapsible::new("Info").icons('[', ']');

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check custom collapsed icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '[');
}

#[test]
fn test_collapsible_icons_custom() {
    let c = Collapsible::new("Info").icons('+', '-');

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check custom collapsed icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '+');

    let c2 = Collapsible::new("Info").icons('•', '◦').expanded(true);

    let mut buffer2 = Buffer::new(20, 3);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);
    c2.render(&mut ctx2);

    // Check custom expanded icon
    assert_eq!(buffer2.get(0, 0).unwrap().symbol, '◦');
}

#[test]
fn test_collapsible_header_colors() {
    let c = Collapsible::new("Info").header_colors(Color::CYAN, Some(Color::BLACK));

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check header color applied
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_collapsible_header_colors_no_bg() {
    let c = Collapsible::new("Info").header_colors(Color::WHITE, None);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check header foreground color
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_collapsible_content_colors() {
    let c = Collapsible::new("Info")
        .content_colors(Color::YELLOW, Some(Color::BLUE))
        .content("Test")
        .expanded(true)
        .border(true);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check content color applied at content line
    assert_eq!(buffer.get(2, 1).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_collapsible_content_colors_no_bg() {
    let c = Collapsible::new("Info")
        .content_colors(Color::rgb(128, 128, 128), None)
        .content("Test")
        .expanded(true)
        .border(true);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check content color
    assert_eq!(
        buffer.get(2, 1).unwrap().fg,
        Some(Color::rgb(128, 128, 128))
    );
}

#[test]
fn test_collapsible_border() {
    let c = Collapsible::new("Info")
        .border(true)
        .content("Test")
        .expanded(true);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check border present
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '│');
}

#[test]
fn test_collapsible_no_border() {
    let c = Collapsible::new("Info")
        .border(false)
        .content("Test")
        .expanded(true);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check no border
    assert_ne!(buffer.get(0, 1).unwrap().symbol, '│');
}

#[test]
fn test_collapsible_border_color() {
    let c = Collapsible::new("Info")
        .border_color(Color::RED)
        .content("Test")
        .expanded(true)
        .border(true);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check border color
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_collapsible_builder_chain() {
    let c = Collapsible::new("Settings")
        .content("Option 1\nOption 2")
        .expanded(true)
        .icons('[', ']')
        .header_colors(Color::CYAN, Some(Color::BLACK))
        .content_colors(Color::WHITE, None)
        .border(true)
        .border_color(Color::GREEN);

    assert!(c.is_expanded());
    assert_eq!(c.height(), 4); // 1 header + 2 content + 1 border (border shares line with header)

    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);

    // Check custom icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ']');
    // Check header color
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    // Check border color
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::GREEN));
}

// =============================================================================
// Toggle Methods Tests
// =============================================================================

#[test]
fn test_collapsible_toggle() {
    let mut c = Collapsible::new("Info");
    assert!(!c.is_expanded());

    c.toggle();
    assert!(c.is_expanded());

    c.toggle();
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_toggle_multiple() {
    let mut c = Collapsible::new("Info");

    for i in 0..10 {
        assert_eq!(c.is_expanded(), i % 2 == 1);
        c.toggle();
    }
    // After 10 toggles (even number), back to collapsed
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_expand() {
    let mut c = Collapsible::new("Info");
    assert!(!c.is_expanded());

    c.expand();
    assert!(c.is_expanded());

    c.expand();
    assert!(c.is_expanded());
}

#[test]
fn test_collapsible_collapse() {
    let mut c = Collapsible::new("Info").expanded(true);
    assert!(c.is_expanded());

    c.collapse();
    assert!(!c.is_expanded());

    c.collapse();
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_is_expanded() {
    let c = Collapsible::new("Info");
    assert!(!c.is_expanded());

    let c2 = Collapsible::new("Info").expanded(true);
    assert!(c2.is_expanded());
}

#[test]
fn test_collapsible_set_expanded() {
    let mut c = Collapsible::new("Info");
    assert!(!c.is_expanded());

    c.set_expanded(true);
    assert!(c.is_expanded());

    c.set_expanded(false);
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_set_expanded_same_value() {
    let mut c = Collapsible::new("Info").expanded(true);
    c.set_expanded(true);
    assert!(c.is_expanded());
}

// =============================================================================
// Height Query Tests
// =============================================================================

#[test]
fn test_collapsible_height_collapsed() {
    let c = Collapsible::new("Info").content("A\nB\nC\nD");
    assert_eq!(c.height(), 1);
}

#[test]
fn test_collapsible_height_expanded_with_border() {
    let c = Collapsible::new("Info")
        .content("A\nB\nC")
        .expanded(true)
        .border(true);
    // header (1) + content (3) + bottom border (1) = 5
    assert_eq!(c.height(), 5);
}

#[test]
fn test_collapsible_height_expanded_without_border() {
    let c = Collapsible::new("Info")
        .content("A\nB\nC")
        .expanded(true)
        .border(false);
    // header (1) + content (3) = 4
    assert_eq!(c.height(), 4);
}

#[test]
fn test_collapsible_height_no_content() {
    let c = Collapsible::new("Info").expanded(true).border(true);
    // header (1) + content (0) + bottom border (1) = 2
    assert_eq!(c.height(), 2);
}

#[test]
fn test_collapsible_height_single_line() {
    let c = Collapsible::new("Info")
        .content("Single")
        .expanded(true)
        .border(true);
    // header (1) + content (1) + bottom border (1) = 3
    assert_eq!(c.height(), 3);
}

#[test]
fn test_collapsible_height_many_lines() {
    let lines: Vec<&str> = (0..20).map(|_| "Line").collect();
    let c = Collapsible::new("Info")
        .lines(&lines)
        .expanded(true)
        .border(true);
    // header (1) + content (20) + bottom border (1) = 22
    assert_eq!(c.height(), 22);
}

// =============================================================================
// Key Handling Tests
// =============================================================================

#[test]
fn test_collapsible_handle_key_enter() {
    let mut c = Collapsible::new("Info");
    assert!(!c.is_expanded());

    let handled = c.handle_key(&Key::Enter);
    assert!(handled);
    assert!(c.is_expanded());
}

#[test]
fn test_collapsible_handle_key_space() {
    let mut c = Collapsible::new("Info");
    assert!(!c.is_expanded());

    let handled = c.handle_key(&Key::Char(' '));
    assert!(handled);
    assert!(c.is_expanded());
}

#[test]
fn test_collapsible_handle_key_right() {
    let mut c = Collapsible::new("Info");
    assert!(!c.is_expanded());

    let handled = c.handle_key(&Key::Right);
    assert!(handled);
    assert!(c.is_expanded());
}

#[test]
fn test_collapsible_handle_key_left() {
    let mut c = Collapsible::new("Info").expanded(true);
    assert!(c.is_expanded());

    let handled = c.handle_key(&Key::Left);
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_handle_key_char_l() {
    let mut c = Collapsible::new("Info");
    assert!(!c.is_expanded());

    let handled = c.handle_key(&Key::Char('l'));
    assert!(handled);
    assert!(c.is_expanded());
}

#[test]
fn test_collapsible_handle_key_char_h() {
    let mut c = Collapsible::new("Info").expanded(true);
    assert!(c.is_expanded());

    let handled = c.handle_key(&Key::Char('h'));
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_handle_key_unhandled() {
    let mut c = Collapsible::new("Info").expanded(true);

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
fn test_collapsible_handle_key_disabled() {
    let mut c = Collapsible::new("Info").disabled(true);
    assert!(!c.is_expanded());

    assert!(!c.handle_key(&Key::Enter));
    assert!(!c.is_expanded());

    assert!(!c.handle_key(&Key::Char(' ')));
    assert!(!c.is_expanded());

    assert!(!c.handle_key(&Key::Right));
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_handle_key_multiple_toggles() {
    let mut c = Collapsible::new("Info");

    for i in 0..10 {
        assert!(c.handle_key(&Key::Enter));
        // After each toggle, state alternates (starting from collapsed=false)
        // i=0: after toggle, expanded=true (1)
        // i=1: after toggle, expanded=false (0)
        assert_eq!(c.is_expanded(), (i + 1) % 2 == 1);
    }
}

// =============================================================================
// Render Tests
// =============================================================================

#[test]
fn test_collapsible_render_collapsed() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Click to expand");
    c.render(&mut ctx);

    // Check icon at position 0
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▶');
    // Check title starts at position 2
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'C');
}

#[test]
fn test_collapsible_render_expanded_with_border() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Details")
        .content("Hidden content")
        .expanded(true)
        .border(true);
    c.render(&mut ctx);

    // Check expanded icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
    // Check left border
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '│');
    // Check bottom corner
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '└');
}

#[test]
fn test_collapsible_render_expanded_without_border() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Details")
        .content("Content")
        .expanded(true)
        .border(false);
    c.render(&mut ctx);

    // Check expanded icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
    // No border at content start
    assert_ne!(buffer.get(0, 1).unwrap().symbol, '│');
}

#[test]
fn test_collapsible_render_multiple_lines() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Multi-line")
        .content("Line 1\nLine 2\nLine 3")
        .expanded(true)
        .border(true);
    c.render(&mut ctx);

    // Check icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
    // Check borders for each line
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 3).unwrap().symbol, '│');
    // Check bottom border
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
}

#[test]
fn test_collapsible_render_custom_icons() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Custom Icons")
        .icons('[', ']')
        .expanded(true);
    c.render(&mut ctx);

    // Check custom expanded icon
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ']');
}

#[test]
fn test_collapsible_render_with_colors() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Colored")
        .header_colors(Color::CYAN, Some(Color::BLACK))
        .content_colors(Color::YELLOW, Some(Color::BLUE))
        .expanded(true)
        .border(true)
        .border_color(Color::GREEN);
    c.render(&mut ctx);

    // Check icon color
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    // Check border color
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_collapsible_render_focused() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Focused").focused(true);
    c.render(&mut ctx);

    // Check that focused icon has BOLD modifier
    let icon_cell = buffer.get(0, 0).unwrap();
    assert!(icon_cell.modifier.contains(revue::render::Modifier::BOLD));
}

#[test]
fn test_collapsible_render_disabled() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Disabled").disabled(true);
    c.render(&mut ctx);

    // Check disabled color (gray)
    assert_eq!(
        buffer.get(0, 0).unwrap().fg,
        Some(Color::rgb(100, 100, 100))
    );
}

#[test]
fn test_collapsible_render_small_area() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Small").content("Test").expanded(true);
    c.render(&mut ctx);

    // Should not panic and should render something
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
}

#[test]
fn test_collapsible_render_zero_width() {
    let mut buffer = Buffer::new(0, 5);
    let area = Rect::new(0, 0, 0, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Test");
    c.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_collapsible_render_zero_height() {
    let mut buffer = Buffer::new(30, 0);
    let area = Rect::new(0, 0, 30, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Test");
    c.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_collapsible_render_too_small_area() {
    let mut buffer = Buffer::new(3, 1);
    let area = Rect::new(0, 0, 3, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Test");
    c.render(&mut ctx);
    // Should not panic (width < 4 check)
}

#[test]
fn test_collapsible_render_long_title() {
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("This is a very long title that exceeds the area");
    c.render(&mut ctx);

    // Should truncate title
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▶');
}

#[test]
fn test_collapsible_render_long_content() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let long_line = "This is a very long content line that should be truncated";
    let c = Collapsible::new("Info")
        .content(long_line)
        .expanded(true)
        .border(true);
    c.render(&mut ctx);

    // Should render without panic
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
}

#[test]
fn test_collapsible_render_content_clipped() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Clipped")
        .content("Line 1\nLine 2\nLine 3\nLine 4\nLine 5")
        .expanded(true)
        .border(true);
    c.render(&mut ctx);

    // Should clip content to available area
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
}

#[test]
fn test_collapsible_render_empty_content() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Empty")
        .content("")
        .expanded(true)
        .border(true);
    c.render(&mut ctx);

    // Should render header and border only
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
}

// =============================================================================
// CSS/Styling Tests
// =============================================================================

#[test]
fn test_collapsible_css_id() {
    let c = Collapsible::new("Info").element_id("my-collapsible");
    assert_eq!(View::id(&c), Some("my-collapsible"));

    let meta = c.meta();
    assert_eq!(meta.id, Some("my-collapsible".to_string()));
}

#[test]
fn test_collapsible_css_classes() {
    let c = Collapsible::new("Info")
        .class("primary")
        .class("expandable");

    assert!(c.has_class("primary"));
    assert!(c.has_class("expandable"));
    assert!(!c.has_class("collapsed"));

    let meta = c.meta();
    assert!(meta.classes.contains("primary"));
    assert!(meta.classes.contains("expandable"));
}

#[test]
fn test_collapsible_css_classes_from_view_trait() {
    let c = Collapsible::new("Info").class("widget").class("info");

    let classes = View::classes(&c);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"widget".to_string()));
    assert!(classes.contains(&"info".to_string()));
}

#[test]
fn test_collapsible_styled_view_set_id() {
    let mut c = Collapsible::new("Info");
    c.set_id("test-id");
    assert_eq!(View::id(&c), Some("test-id"));
}

#[test]
fn test_collapsible_styled_view_add_class() {
    let mut c = Collapsible::new("Info");
    c.add_class("active");
    assert!(c.has_class("active"));
}

#[test]
fn test_collapsible_styled_view_remove_class() {
    let mut c = Collapsible::new("Info").class("active");
    c.remove_class("active");
    assert!(!c.has_class("active"));
}

#[test]
fn test_collapsible_styled_view_toggle_class() {
    let mut c = Collapsible::new("Info");

    c.toggle_class("selected");
    assert!(c.has_class("selected"));

    c.toggle_class("selected");
    assert!(!c.has_class("selected"));
}

#[test]
fn test_collapsible_classes_builder() {
    let c = Collapsible::new("Info").classes(vec!["class1", "class2", "class3"]);

    assert!(c.has_class("class1"));
    assert!(c.has_class("class2"));
    assert!(c.has_class("class3"));
    assert_eq!(View::classes(&c).len(), 3);
}

#[test]
fn test_collapsible_duplicate_class_not_added() {
    let c = Collapsible::new("Info").class("test").class("test");

    let classes = View::classes(&c);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

// =============================================================================
// Meta and Debug Tests
// =============================================================================

#[test]
fn test_collapsible_meta() {
    let c = Collapsible::new("Test")
        .element_id("test-collapsible")
        .class("widget")
        .class("info");

    let meta = c.meta();
    assert_eq!(meta.widget_type, "Collapsible");
    assert_eq!(meta.id, Some("test-collapsible".to_string()));
    assert!(meta.classes.contains("widget"));
    assert!(meta.classes.contains("info"));
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_collapsible_full_expand_collapse_cycle() {
    let mut c = Collapsible::new("Cycle").content("Test content");

    // Initial state
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);

    // Expand
    c.expand();
    assert!(c.is_expanded());
    assert_eq!(c.height(), 3); // 1 + 1 + 1 with border

    // Collapse
    c.collapse();
    assert!(!c.is_expanded());
    assert_eq!(c.height(), 1);
}

#[test]
fn test_collapsible_keyboard_navigation() {
    let mut c = Collapsible::new("Navigation").content("Content");

    // Expand with Enter
    assert!(c.handle_key(&Key::Enter));
    assert!(c.is_expanded());

    // Collapse with Left
    assert!(c.handle_key(&Key::Left));
    assert!(!c.is_expanded());

    // Expand with Right
    assert!(c.handle_key(&Key::Right));
    assert!(c.is_expanded());

    // Toggle with Space
    assert!(c.handle_key(&Key::Char(' ')));
    assert!(!c.is_expanded());
}

#[test]
fn test_collapsible_builder_with_all_options() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Collapsible::new("Complete")
        .content("Line 1\nLine 2\nLine 3")
        .expanded(true)
        .icons('[', ']')
        .header_colors(Color::CYAN, Some(Color::BLACK))
        .content_colors(Color::WHITE, Some(Color::BLUE))
        .border(true)
        .border_color(Color::GREEN)
        .element_id("complete")
        .class("test-widget");

    c.render(&mut ctx);

    // Verify rendering
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ']');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '│');

    // Verify state
    assert!(c.is_expanded());
    assert_eq!(c.height(), 5);
    assert_eq!(View::id(&c), Some("complete"));
    assert!(c.has_class("test-widget"));
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_collapsible_empty_title() {
    let c = Collapsible::new("");
    assert!(!c.is_expanded());

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_collapsible_unicode_title() {
    let c = Collapsible::new("한글 제목");

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_collapsible_unicode_content() {
    let c = Collapsible::new("Info")
        .content("Line 1\n한글 줄\nEmoji 🎉")
        .expanded(true)
        .border(true);

    assert_eq!(c.height(), 5);
}

#[test]
fn test_collapsible_special_characters_in_content() {
    let c = Collapsible::new("Special").content("Tab:\tTab\nNewline:\nNewline");

    // Collapsed by default, so only header height (1)
    assert_eq!(c.height(), 1);
}

#[test]
fn test_collapsible_very_long_single_line() {
    let long_line = "A".repeat(1000);
    let c = Collapsible::new("Long").content(&long_line).expanded(true);

    assert!(c.height() >= 2);
}

#[test]
fn test_collapsible_many_empty_lines() {
    let c = Collapsible::new("Empty")
        .content("\n\n\n\n\n")
        .expanded(true)
        .border(true);

    // Splits by newlines, creating empty strings
    assert!(c.height() > 2);
}

#[test]
fn test_collapsible_toggle_then_check_height() {
    let mut c = Collapsible::new("Dynamic").content("A\nB\nC").border(true);

    assert_eq!(c.height(), 1);

    c.expand();
    assert_eq!(c.height(), 5);

    c.collapse();
    assert_eq!(c.height(), 1);
}

#[test]
fn test_collapsible_disabled_toggle_methods() {
    let mut c = Collapsible::new("Disabled").disabled(true);

    // Disabled state doesn't prevent toggle methods (only handle_key)
    c.toggle();
    assert!(c.is_expanded());

    c.collapse();
    assert!(!c.is_expanded());

    c.expand();
    assert!(c.is_expanded());
}

#[test]
fn test_collapsible_render_expanded_icon() {
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Collapsed shows collapsed icon
    let c1 = Collapsible::new("Test").icons('+', '-');
    c1.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '+');

    let mut buffer2 = Buffer::new(20, 3);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);

    // Expanded shows expanded icon
    let c2 = Collapsible::new("Test").icons('+', '-').expanded(true);
    c2.render(&mut ctx2);
    assert_eq!(buffer2.get(0, 0).unwrap().symbol, '-');
}

// =============================================================================
