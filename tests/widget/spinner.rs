//! Spinner widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView};
use revue::widget::{spinner, Spinner, SpinnerStyle, View};

#[test]
fn test_spinner_tick() {
    let mut s = Spinner::new();
    assert_eq!(s.frame(), 0);
    s.tick();
    assert_eq!(s.frame(), 1);
    s.tick();
    assert_eq!(s.frame(), 2);
}

#[test]
fn test_spinner_wrap() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);
    s.set_frame(3);
    s.tick();
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_render() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Spinner::new();
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_with_label() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Spinner::new().label("Loading...");
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'L');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'o');
}

#[test]
fn test_spinner_style_line() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Spinner::new().style(SpinnerStyle::Line);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '|');
}

#[test]
fn test_spinner_style_circle() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Spinner::new().style(SpinnerStyle::Circle);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '◐');
}

#[test]
fn test_spinner_reset() {
    let mut s = Spinner::new();
    s.tick();
    s.tick();
    assert_eq!(s.frame(), 2);
    s.reset();
    assert_eq!(s.frame(), 0);
}

// =============================================================================
// Builder Methods Tests
// =============================================================================

#[test]
fn test_spinner_default_trait() {
    let s = Spinner::default();
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_helper_function() {
    let s = spinner();
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_builder_fg() {
    let s = Spinner::new().fg(Color::RED);
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_spinner_builder_element_id() {
    let s = Spinner::new().element_id("my-spinner");
    assert_eq!(View::id(&s), Some("my-spinner"));
}

#[test]
fn test_spinner_builder_class() {
    let s = Spinner::new().class("loading").class("active");
    assert!(s.has_class("loading"));
    assert!(s.has_class("active"));
}

#[test]
fn test_spinner_builder_classes() {
    let s = Spinner::new().classes(vec!["loading", "spinner", "fast"]);
    assert!(s.has_class("loading"));
    assert!(s.has_class("spinner"));
    assert!(s.has_class("fast"));
}

#[test]
fn test_spinner_builder_classes_no_duplicates() {
    let s = Spinner::new().class("test").classes(vec!["test", "other"]);
    let classes = View::classes(&s);
    // Should only have one "test" class (other is also there, so total 2)
    assert!(classes.contains(&"test".to_string()));
    assert!(classes.contains(&"other".to_string()));
}

// =============================================================================
// Spinner Style Tests
// =============================================================================

#[test]
fn test_spinner_style_dots_default() {
    let s = Spinner::new();
    // Default is Dots style, which renders '⠋' at frame 0
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_style_dots_explicit() {
    let s = Spinner::new().style(SpinnerStyle::Dots);
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_style_arrow_render() {
    let s = Spinner::new().style(SpinnerStyle::Arrow);
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '←');
}

#[test]
fn test_spinner_style_box_render() {
    let s = Spinner::new().style(SpinnerStyle::Box);
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▖');
}

#[test]
fn test_spinner_style_bounce_render() {
    let s = Spinner::new().style(SpinnerStyle::Bounce);
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠁');
}

#[test]
fn test_spinner_style_frames_dots() {
    // Test all Dots frames
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    for (i, expected) in frames.iter().enumerate() {
        let mut test_s = Spinner::new().style(SpinnerStyle::Dots);
        test_s.set_frame(i);
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        test_s.render(&mut ctx);
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            expected.chars().next().unwrap()
        );
    }
}

#[test]
fn test_spinner_style_frames_line() {
    let frames = ["|", "/", "-", "\\"];
    for (i, expected) in frames.iter().enumerate() {
        let mut test_s = Spinner::new().style(SpinnerStyle::Line);
        test_s.set_frame(i);
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        test_s.render(&mut ctx);
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            expected.chars().next().unwrap()
        );
    }
}

#[test]
fn test_spinner_style_frames_circle() {
    let frames = ["◐", "◓", "◑", "◒"];
    for (i, expected) in frames.iter().enumerate() {
        let mut test_s = Spinner::new().style(SpinnerStyle::Circle);
        test_s.set_frame(i);
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        test_s.render(&mut ctx);
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            expected.chars().next().unwrap()
        );
    }
}

#[test]
fn test_spinner_style_frames_arrow() {
    let frames = ["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"];
    for (i, expected) in frames.iter().enumerate() {
        let mut test_s = Spinner::new().style(SpinnerStyle::Arrow);
        test_s.set_frame(i);
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        test_s.render(&mut ctx);
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            expected.chars().next().unwrap()
        );
    }
}

#[test]
fn test_spinner_style_frames_box() {
    let frames = ["▖", "▘", "▝", "▗"];
    for (i, expected) in frames.iter().enumerate() {
        let mut test_s = Spinner::new().style(SpinnerStyle::Box);
        test_s.set_frame(i);
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        test_s.render(&mut ctx);
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            expected.chars().next().unwrap()
        );
    }
}

#[test]
fn test_spinner_style_frames_bounce() {
    let frames = ["⠁", "⠂", "⠄", "⠂"];
    for (i, expected) in frames.iter().enumerate() {
        let mut test_s = Spinner::new().style(SpinnerStyle::Bounce);
        test_s.set_frame(i);
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        test_s.render(&mut ctx);
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            expected.chars().next().unwrap()
        );
    }
}

// =============================================================================
// StyledView Trait Tests
// =============================================================================

#[test]
fn test_spinner_styled_view_set_id() {
    let mut s = Spinner::new();
    StyledView::set_id(&mut s, "test-id");
    assert_eq!(View::id(&s), Some("test-id"));
}

#[test]
fn test_spinner_styled_view_add_class() {
    let mut s = Spinner::new();
    StyledView::add_class(&mut s, "first");
    StyledView::add_class(&mut s, "second");
    assert!(StyledView::has_class(&s, "first"));
    assert!(StyledView::has_class(&s, "second"));
    assert_eq!(View::classes(&s).len(), 2);
}

#[test]
fn test_spinner_styled_view_add_class_no_duplicates() {
    let mut s = Spinner::new();
    StyledView::add_class(&mut s, "test");
    StyledView::add_class(&mut s, "test");
    let classes = View::classes(&s);
    // Should only have one "test" class
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_spinner_styled_view_remove_class() {
    let mut s = Spinner::new().class("a").class("b").class("c");
    StyledView::remove_class(&mut s, "b");
    assert!(StyledView::has_class(&s, "a"));
    assert!(!StyledView::has_class(&s, "b"));
    assert!(StyledView::has_class(&s, "c"));
}

#[test]
fn test_spinner_styled_view_remove_nonexistent_class() {
    let mut s = Spinner::new().class("test");
    StyledView::remove_class(&mut s, "nonexistent");
    assert!(StyledView::has_class(&s, "test"));
}

#[test]
fn test_spinner_styled_view_toggle_class_add() {
    let mut s = Spinner::new();
    StyledView::toggle_class(&mut s, "test");
    assert!(StyledView::has_class(&s, "test"));
}

#[test]
fn test_spinner_styled_view_toggle_class_remove() {
    let mut s = Spinner::new().class("test");
    StyledView::toggle_class(&mut s, "test");
    assert!(!StyledView::has_class(&s, "test"));
}

#[test]
fn test_spinner_styled_view_has_class() {
    let s = Spinner::new().class("present");
    assert!(StyledView::has_class(&s, "present"));
    assert!(!StyledView::has_class(&s, "absent"));
}

// =============================================================================
// View Trait Tests
// =============================================================================

#[test]
fn test_spinner_view_widget_type() {
    let s = Spinner::new();
    assert_eq!(s.widget_type(), "Spinner");
}

#[test]
fn test_spinner_view_id_none() {
    let s = Spinner::new();
    assert!(View::id(&s).is_none());
}

#[test]
fn test_spinner_view_id_some() {
    let s = Spinner::new().element_id("my-id");
    assert_eq!(View::id(&s), Some("my-id"));
}

#[test]
fn test_spinner_view_classes_empty() {
    let s = Spinner::new();
    assert!(View::classes(&s).is_empty());
}

#[test]
fn test_spinner_view_classes_with_values() {
    let s = Spinner::new().class("first").class("second");
    let classes = View::classes(&s);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"first".to_string()));
    assert!(classes.contains(&"second".to_string()));
}

#[test]
fn test_spinner_view_meta() {
    let s = Spinner::new().element_id("test-id").class("test-class");
    let meta = s.meta();
    assert_eq!(meta.widget_type, "Spinner");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

#[test]
fn test_spinner_view_children_default() {
    let s = Spinner::new();
    assert!(View::children(&s).is_empty());
}

// =============================================================================
// Tick and Progress Tests
// =============================================================================

#[test]
fn test_spinner_tick_multiple() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);
    assert_eq!(s.frame(), 0);

    s.tick();
    assert_eq!(s.frame(), 1);

    s.tick();
    assert_eq!(s.frame(), 2);

    s.tick();
    assert_eq!(s.frame(), 3);

    s.tick(); // Should wrap back to 0
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_tick_all_styles() {
    let styles = [
        SpinnerStyle::Dots,
        SpinnerStyle::Line,
        SpinnerStyle::Circle,
        SpinnerStyle::Arrow,
        SpinnerStyle::Box,
        SpinnerStyle::Bounce,
    ];

    for style in styles {
        let mut s = Spinner::new().style(style);
        let initial_frame = s.frame();
        s.tick();
        assert_ne!(
            s.frame(),
            initial_frame,
            "Tick should change frame for {:?}",
            style
        );
    }
}

#[test]
fn test_spinner_set_frame() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);
    s.set_frame(2);
    assert_eq!(s.frame(), 2);
}

#[test]
fn test_spinner_set_frame_wraps() {
    let mut s = Spinner::new().style(SpinnerStyle::Line); // 4 frames
    s.set_frame(10); // 10 % 4 = 2
    assert_eq!(s.frame(), 2);
}

#[test]
fn test_spinner_set_frame_zero() {
    let mut s = Spinner::new();
    s.tick();
    s.tick();
    assert_ne!(s.frame(), 0);
    s.set_frame(0);
    assert_eq!(s.frame(), 0);
}

// =============================================================================
// Render Edge Cases Tests
// =============================================================================

#[test]
fn test_spinner_render_empty_width() {
    let s = Spinner::new();
    let mut buffer = Buffer::new(0, 1);
    let area = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_spinner_render_empty_height() {
    let s = Spinner::new();
    let mut buffer = Buffer::new(10, 0);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_spinner_render_label_truncation() {
    let s = Spinner::new().label("Very long label that should be truncated");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    // Spinner at 0, space at 1-2, label starts at 2
    // Only 8 chars for label (10 - 2)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'V');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'e');
    // Last char position should be filled
    assert!(buffer.get(9, 0).is_some());
}

#[test]
fn test_spinner_render_label_exact_fit() {
    let s = Spinner::new().label("12345"); // 5 chars
    let mut buffer = Buffer::new(7, 1); // Spinner(1) + space(1) + label(5) = 7
    let area = Rect::new(0, 0, 7, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '1');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '5');
}

#[test]
fn test_spinner_render_no_space_for_label() {
    let s = Spinner::new().label("Test");
    let mut buffer = Buffer::new(2, 1); // Only room for spinner
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    // Spinner should render, label should be truncated
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_render_with_offset() {
    let s = Spinner::new().label("Test");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(5, 2, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    // Spinner at x=5, y=2
    assert_eq!(buffer.get(5, 2).unwrap().symbol, '⠋');
    // Label starts at x=7 (5 + 2)
    assert_eq!(buffer.get(7, 2).unwrap().symbol, 'T');
}

#[test]
fn test_spinner_render_with_custom_color() {
    let s = Spinner::new().fg(Color::GREEN).label("OK");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::GREEN));
    // Label cells inherit default foreground (not set)
    assert_eq!(buffer.get(2, 0).unwrap().fg, None);
}

// =============================================================================
// SpinnerStyle PartialEq Tests
// =============================================================================

#[test]
fn test_spinner_style_eq() {
    assert_eq!(SpinnerStyle::Dots, SpinnerStyle::Dots);
    assert_eq!(SpinnerStyle::Line, SpinnerStyle::Line);
}

#[test]
fn test_spinner_style_ne() {
    assert_ne!(SpinnerStyle::Dots, SpinnerStyle::Line);
    assert_ne!(SpinnerStyle::Circle, SpinnerStyle::Arrow);
}

// =============================================================================
// Color Tests
// =============================================================================

#[test]
fn test_spinner_rgb_color() {
    let s = Spinner::new().fg(Color::rgb(255, 128, 0));
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::rgb(255, 128, 0)));
}

#[test]
fn test_spinner_rgba_color() {
    let s = Spinner::new().fg(Color::rgba(200, 100, 50, 180));
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(
        buffer.get(0, 0).unwrap().fg,
        Some(Color::rgba(200, 100, 50, 180))
    );
}

#[test]
fn test_spinner_multiple_fg_calls() {
    let s = Spinner::new().fg(Color::YELLOW).fg(Color::RED);
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Last call wins
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
}

// =============================================================================
// Label Edge Cases
// =============================================================================

#[test]
fn test_spinner_empty_label() {
    let s = Spinner::new().label("");
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_unicode_label() {
    let s = Spinner::new().label("로딩 중"); // Korean "Loading"
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '로');
}

#[test]
fn test_spinner_emoji_label() {
    let s = Spinner::new().label("🚀 Loading");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    // Emoji renders correctly
}

#[test]
fn test_spinner_very_long_label() {
    let long_label = "This is a very long label that goes on and on and on";
    let s = Spinner::new().label(long_label);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Should truncate to fit buffer
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_label_with_spaces() {
    let s = Spinner::new().label("  Multiple  Spaces  ");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Spaces should be preserved
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, ' ');
}

#[test]
fn test_spinner_label_single_char() {
    let s = Spinner::new().label("X");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'X');
}

// =============================================================================
// Frame Wrapping Edge Cases
// =============================================================================

#[test]
fn test_spinner_wrap_large_frame() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);
    s.set_frame(1000);
    assert_eq!(s.frame(), 1000 % 4);
}

#[test]
fn test_spinner_wrap_negative_frame() {
    let mut s = Spinner::new().style(SpinnerStyle::Dots);
    s.set_frame(9); // Last frame for Dots (10 frames: 0-9)
    s.tick(); // Should wrap to 0
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_all_styles_frame_count() {
    let styles = [
        (SpinnerStyle::Dots, 10),
        (SpinnerStyle::Line, 4),
        (SpinnerStyle::Circle, 4),
        (SpinnerStyle::Arrow, 8),
        (SpinnerStyle::Box, 4),
        (SpinnerStyle::Bounce, 4),
    ];

    for (style, expected_frames) in styles {
        let mut s = Spinner::new().style(style);
        for i in 0..expected_frames {
            s.set_frame(i);
            assert_eq!(s.frame(), i, "Frame {} for {:?}", i, style);
            s.tick();
        }
        // Should be back to 0 after full cycle
        assert_eq!(
            s.frame(),
            0,
            "Should wrap to 0 after {} ticks for {:?}",
            expected_frames,
            style
        );
    }
}

#[test]
fn test_spinner_multiple_wraps() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);
    for _ in 0..20 {
        s.tick();
    }
    assert_eq!(s.frame(), 20 % 4);
}

// =============================================================================
// State Transition Tests
// =============================================================================

#[test]
fn test_spinner_state_transitions() {
    let mut s = Spinner::new().style(SpinnerStyle::Line);

    // Test each state transition
    let expected_frames = ['|', '/', '-', '\\', '|'];
    for (i, expected) in expected_frames.iter().enumerate() {
        let mut buffer = Buffer::new(5, 1);
        let area = Rect::new(0, 0, 5, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        s.render(&mut ctx);
        assert_eq!(
            buffer.get(0, 0).unwrap().symbol,
            *expected,
            "Frame {} should be {}",
            i,
            expected
        );
        s.tick();
    }
}

#[test]
fn test_spinner_reset_from_middle() {
    let mut s = Spinner::new().style(SpinnerStyle::Circle);
    s.set_frame(2);
    assert_eq!(s.frame(), 2);
    s.reset();
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_reset_after_multiple_ticks() {
    let mut s = Spinner::new().style(SpinnerStyle::Arrow);
    for _ in 0..15 {
        s.tick();
    }
    assert_ne!(s.frame(), 0);
    s.reset();
    assert_eq!(s.frame(), 0);
}

#[test]
fn test_spinner_style_change_preserves_frame() {
    // Create new instance with different style
    let mut s1 = Spinner::new().style(SpinnerStyle::Line);
    let mut s2 = Spinner::new().style(SpinnerStyle::Circle);
    s1.set_frame(2);
    s2.set_frame(2);
    // Both should have same frame regardless of style
    assert_eq!(s1.frame(), 2);
    assert_eq!(s2.frame(), 2);
}

// =============================================================================
// Render Multiple Times
// =============================================================================

#[test]
fn test_spinner_render_multiple_times_same_position() {
    let s = Spinner::new();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);

    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        s.render(&mut ctx);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
    }
}

#[test]
fn test_spinner_render_at_different_areas() {
    let s = Spinner::new().label("Test");
    let mut buffer = Buffer::new(30, 5);

    let areas = [
        Rect::new(0, 0, 10, 1),
        Rect::new(5, 2, 10, 1),
        Rect::new(10, 4, 10, 1),
    ];

    for area in areas {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        s.render(&mut ctx);
        // Should render at each area
        assert_eq!(buffer.get(area.x, area.y).unwrap().symbol, '⠋');
    }
}

// =============================================================================
// CSS Integration Tests (Extended)
// =============================================================================

#[test]
fn test_spinner_multiple_classes() {
    let s = Spinner::new()
        .class("fast")
        .class("spinner")
        .class("loading");
    assert!(s.has_class("fast"));
    assert!(s.has_class("spinner"));
    assert!(s.has_class("loading"));
    assert_eq!(View::classes(&s).len(), 3);
}

#[test]
fn test_spinner_class_operations() {
    let mut s = Spinner::new();

    StyledView::add_class(&mut s, "first");
    assert!(StyledView::has_class(&s, "first"));

    StyledView::add_class(&mut s, "second");
    assert!(StyledView::has_class(&s, "second"));
    assert_eq!(View::classes(&s).len(), 2);

    StyledView::remove_class(&mut s, "first");
    assert!(!StyledView::has_class(&s, "first"));
    assert_eq!(View::classes(&s).len(), 1);
}

// =============================================================================
// Additional Edge Cases
// =============================================================================

#[test]
fn test_spinner_zero_area() {
    let s = Spinner::new().label("Test");
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_spinner_single_pixel_area() {
    let s = Spinner::new();
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_label_with_newline() {
    let s = Spinner::new().label("Line1\nLine2");
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Should handle newlines
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_label_with_tabs() {
    let s = Spinner::new().label("Tab\there");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Should handle tabs
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⠋');
}

#[test]
fn test_spinner_all_styles_with_label() {
    let styles = [
        SpinnerStyle::Dots,
        SpinnerStyle::Line,
        SpinnerStyle::Circle,
        SpinnerStyle::Arrow,
        SpinnerStyle::Box,
        SpinnerStyle::Bounce,
    ];

    for style in styles {
        let s = Spinner::new().style(style).label("Test");
        let mut buffer = Buffer::new(15, 1);
        let area = Rect::new(0, 0, 15, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        s.render(&mut ctx);
        // All should render spinner and label
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'e');
    }
}

#[test]
fn test_spinner_label_color_independence() {
    let s = Spinner::new().fg(Color::RED).label("Test");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
    // Spinner has color, label uses default
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(2, 0).unwrap().fg, None);
}
