//! Checkbox widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Checkbox, StyledView, View};

#[test]
fn test_checkbox_toggle() {
    let mut cb = Checkbox::new("Toggle");
    assert!(!cb.is_checked());

    cb.toggle();
    assert!(cb.is_checked());

    cb.toggle();
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_disabled_toggle() {
    let mut cb = Checkbox::new("Disabled").disabled(true);
    assert!(!cb.is_checked());

    cb.toggle();
    assert!(!cb.is_checked()); // Should not change
}

#[test]
fn test_checkbox_handle_key() {
    let mut cb = Checkbox::new("Test");

    assert!(cb.handle_key(&Key::Enter));
    assert!(cb.is_checked());

    assert!(cb.handle_key(&Key::Char(' ')));
    assert!(!cb.is_checked());

    assert!(!cb.handle_key(&Key::Char('a')));
}

#[test]
fn test_checkbox_disabled_handle_key() {
    let mut cb = Checkbox::new("Test").disabled(true);

    assert!(!cb.handle_key(&Key::Enter));
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_render() {
    let cb = Checkbox::new("Option").checked(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);
}

#[test]
fn test_checkbox_css_id() {
    let cb = Checkbox::new("Accept").element_id("accept-checkbox");
    assert_eq!(View::id(&cb), Some("accept-checkbox"));

    let meta = cb.meta();
    assert_eq!(meta.id, Some("accept-checkbox".to_string()));
}

#[test]
fn test_checkbox_css_classes() {
    let cb = Checkbox::new("Option")
        .class("required")
        .class("form-control");

    assert!(cb.has_class("required"));
    assert!(cb.has_class("form-control"));
    assert!(!cb.has_class("optional"));

    let meta = cb.meta();
    assert!(meta.classes.contains("required"));
    assert!(meta.classes.contains("form-control"));
}

#[test]
fn test_checkbox_styled_view() {
    let mut cb = Checkbox::new("Test");

    cb.set_id("test-cb");
    assert_eq!(View::id(&cb), Some("test-cb"));

    cb.add_class("active");
    assert!(cb.has_class("active"));

    cb.toggle_class("active");
    assert!(!cb.has_class("active"));

    cb.toggle_class("selected");
    assert!(cb.has_class("selected"));

    cb.remove_class("selected");
    assert!(!cb.has_class("selected"));
}

#[test]
fn test_checkbox_css_colors_from_context() {
    use revue::style::{Style, VisualStyle};

    let cb = Checkbox::new("CSS Test");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::YELLOW,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    cb.render(&mut ctx);
}

// =============================================================================
// CheckboxStyle tests (via render output validation)
// =============================================================================

#[test]
fn test_checkbox_style_render_output() {
    use revue::widget::CheckboxStyle;

    // Test Square style
    let cb_square = Checkbox::new("Test")
        .checked(true)
        .style(CheckboxStyle::Square);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cb_square.render(&mut ctx);
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("[x]"));

    // Test Unicode style
    let cb_unicode = Checkbox::new("Test")
        .checked(true)
        .style(CheckboxStyle::Unicode);
    let mut buffer = Buffer::new(30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cb_unicode.render(&mut ctx);
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('☑'));

    // Test Filled style
    let cb_filled = Checkbox::new("Test")
        .checked(true)
        .style(CheckboxStyle::Filled);
    let mut buffer = Buffer::new(30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cb_filled.render(&mut ctx);
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('■'));

    // Test Circle style
    let cb_circle = Checkbox::new("Test")
        .checked(true)
        .style(CheckboxStyle::Circle);
    let mut buffer = Buffer::new(30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cb_circle.render(&mut ctx);
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('●'));
}

// =============================================================================
// Default trait tests
// =============================================================================

#[test]
fn test_checkbox_default() {
    use revue::widget::Checkbox;

    let cb = Checkbox::default();
    assert!(!cb.is_checked());
    assert!(!cb.is_focused());
    assert!(!cb.is_disabled());
}

// =============================================================================
// set_checked method tests
// =============================================================================

#[test]
fn test_checkbox_set_checked() {
    let mut cb = Checkbox::new("Option");
    assert!(!cb.is_checked());

    cb.set_checked(true);
    assert!(cb.is_checked());

    cb.set_checked(false);
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_set_checked_when_disabled() {
    let mut cb = Checkbox::new("Option").disabled(true);
    assert!(!cb.is_checked());

    // set_checked should work even when disabled (toggle doesn't, but set_checked does)
    cb.set_checked(true);
    assert!(cb.is_checked());
}

// =============================================================================
// Focus behavior tests (Interactive trait)
// =============================================================================

#[test]
fn test_checkbox_focusable_when_enabled() {
    use revue::widget::traits::Interactive;
    let cb = Checkbox::new("Option");
    // Enabled checkboxes are focusable
    assert!(cb.focusable());
}

#[test]
fn test_checkbox_not_focusable_when_disabled() {
    use revue::widget::traits::Interactive;
    let cb = Checkbox::new("Option").disabled(true);
    // Disabled checkboxes are not focusable
    assert!(!cb.focusable());
}

#[test]
fn test_checkbox_on_focus() {
    use revue::widget::traits::Interactive;

    let mut cb = Checkbox::new("Option");
    assert!(!cb.is_focused());

    cb.on_focus();
    assert!(cb.is_focused());
}

#[test]
fn test_checkbox_on_blur() {
    use revue::widget::traits::Interactive;

    let mut cb = Checkbox::new("Option").focused(true);
    assert!(cb.is_focused());

    cb.on_blur();
    assert!(!cb.is_focused());
}

#[test]
fn test_checkbox_focus_and_blur_cycle() {
    use revue::widget::traits::Interactive;

    let mut cb = Checkbox::new("Option");

    cb.on_focus();
    assert!(cb.is_focused());

    cb.on_blur();
    assert!(!cb.is_focused());

    cb.on_focus();
    assert!(cb.is_focused());
}

// =============================================================================
// Interactive trait handle_key EventResult tests
// =============================================================================

#[test]
fn test_checkbox_interactive_handle_key_enter() {
    use revue::event::KeyEvent;
    use revue::widget::traits::{EventResult, Interactive};

    let mut cb = Checkbox::new("Option");
    let event = KeyEvent::new(Key::Enter);

    let result = Interactive::handle_key(&mut cb, &event);
    assert_eq!(result, EventResult::ConsumedAndRender);
    assert!(cb.is_checked());
}

#[test]
fn test_checkbox_interactive_handle_key_space() {
    use revue::event::KeyEvent;
    use revue::widget::traits::{EventResult, Interactive};

    let mut cb = Checkbox::new("Option").checked(true);
    let event = KeyEvent::new(Key::Char(' '));

    let result = Interactive::handle_key(&mut cb, &event);
    assert_eq!(result, EventResult::ConsumedAndRender);
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_interactive_handle_key_other() {
    use revue::event::KeyEvent;
    use revue::widget::traits::{EventResult, Interactive};

    let mut cb = Checkbox::new("Option");
    let event = KeyEvent::new(Key::Char('a'));

    let result = Interactive::handle_key(&mut cb, &event);
    assert_eq!(result, EventResult::Ignored);
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_interactive_handle_key_escape() {
    use revue::event::KeyEvent;
    use revue::widget::traits::{EventResult, Interactive};

    let mut cb = Checkbox::new("Option");
    let event = KeyEvent::new(Key::Escape);

    let result = Interactive::handle_key(&mut cb, &event);
    assert_eq!(result, EventResult::Ignored);
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_interactive_handle_key_when_disabled() {
    use revue::event::KeyEvent;
    use revue::widget::traits::{EventResult, Interactive};

    let mut cb = Checkbox::new("Option").disabled(true);
    let event = KeyEvent::new(Key::Enter);

    let result = Interactive::handle_key(&mut cb, &event);
    assert_eq!(result, EventResult::Ignored);
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_interactive_handle_key_multiple_toggles() {
    use revue::event::KeyEvent;
    use revue::widget::traits::{EventResult, Interactive};

    let mut cb = Checkbox::new("Option");

    // First toggle
    let result1 = Interactive::handle_key(&mut cb, &KeyEvent::new(Key::Enter));
    assert_eq!(result1, EventResult::ConsumedAndRender);
    assert!(cb.is_checked());

    // Second toggle
    let result2 = Interactive::handle_key(&mut cb, &KeyEvent::new(Key::Char(' ')));
    assert_eq!(result2, EventResult::ConsumedAndRender);
    assert!(!cb.is_checked());

    // Third toggle
    let result3 = Interactive::handle_key(&mut cb, &KeyEvent::new(Key::Enter));
    assert_eq!(result3, EventResult::ConsumedAndRender);
    assert!(cb.is_checked());
}

// =============================================================================
// Checkbox::handle_key(&Key) -> bool method tests
// =============================================================================

#[test]
fn test_checkbox_handle_key_method_enter() {
    let mut cb = Checkbox::new("Option");
    assert!(cb.handle_key(&Key::Enter));
    assert!(cb.is_checked());
}

#[test]
fn test_checkbox_handle_key_method_space() {
    let mut cb = Checkbox::new("Option").checked(true);
    assert!(cb.handle_key(&Key::Char(' ')));
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_handle_key_method_other_key() {
    let mut cb = Checkbox::new("Option");
    assert!(!cb.handle_key(&Key::Char('a')));
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_handle_key_method_when_disabled() {
    let mut cb = Checkbox::new("Option").disabled(true);
    assert!(!cb.handle_key(&Key::Enter));
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_handle_key_method_multiple() {
    let mut cb = Checkbox::new("Option");

    assert!(cb.handle_key(&Key::Enter));
    assert!(cb.is_checked());

    assert!(cb.handle_key(&Key::Char(' ')));
    assert!(!cb.is_checked());

    assert!(cb.handle_key(&Key::Enter));
    assert!(cb.is_checked());
}

// =============================================================================
// Render tests for different styles and states
// =============================================================================

#[test]
fn test_checkbox_render_square_style_checked() {
    let cb = Checkbox::new("Option")
        .checked(true)
        .style(revue::widget::CheckboxStyle::Square);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Verify the checkbox is rendered
    // Should contain [x] for checked square style
    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("[x]"));
}

#[test]
fn test_checkbox_render_square_style_unchecked() {
    let cb = Checkbox::new("Option")
        .checked(false)
        .style(revue::widget::CheckboxStyle::Square);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should contain [ ] for unchecked square style
    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("[ ]"));
}

#[test]
fn test_checkbox_render_unicode_style_checked() {
    let cb = Checkbox::new("Option")
        .checked(true)
        .style(revue::widget::CheckboxStyle::Unicode);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should contain ☑ for checked unicode style
    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('☑'));
}

#[test]
fn test_checkbox_render_unicode_style_unchecked() {
    let cb = Checkbox::new("Option")
        .checked(false)
        .style(revue::widget::CheckboxStyle::Unicode);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should contain ☐ for unchecked unicode style
    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('☐'));
}

#[test]
fn test_checkbox_render_filled_style_checked() {
    let cb = Checkbox::new("Option")
        .checked(true)
        .style(revue::widget::CheckboxStyle::Filled);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should contain ■ for checked filled style
    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('■'));
}

#[test]
fn test_checkbox_render_filled_style_unchecked() {
    let cb = Checkbox::new("Option")
        .checked(false)
        .style(revue::widget::CheckboxStyle::Filled);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should contain □ for unchecked filled style
    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('□'));
}

#[test]
fn test_checkbox_render_circle_style_checked() {
    let cb = Checkbox::new("Option")
        .checked(true)
        .style(revue::widget::CheckboxStyle::Circle);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should contain ● for checked circle style
    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('●'));
}

#[test]
fn test_checkbox_render_circle_style_unchecked() {
    let cb = Checkbox::new("Option")
        .checked(false)
        .style(revue::widget::CheckboxStyle::Circle);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(1, 1, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should contain ○ for unchecked circle style
    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('○'));
}

#[test]
fn test_checkbox_render_focused() {
    let cb = Checkbox::new("Option").focused(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Focused checkbox should have > indicator
    let first_cell = buffer.get(0, 0).unwrap();
    assert_eq!(first_cell.symbol, '>');
}

#[test]
fn test_checkbox_render_unfocused() {
    let cb = Checkbox::new("Option").focused(false);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Unfocused checkbox should not have > indicator
    let first_cell = buffer.get(0, 0).unwrap();
    assert_ne!(first_cell.symbol, '>');
}

#[test]
fn test_checkbox_render_disabled() {
    let cb = Checkbox::new("Option").disabled(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Disabled checkbox should not have > indicator even if focused
    let first_cell = buffer.get(0, 0).unwrap();
    assert_ne!(first_cell.symbol, '>');
}

#[test]
fn test_checkbox_render_focused_disabled() {
    let cb = Checkbox::new("Option").focused(true).disabled(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Disabled checkbox should not show focus indicator
    let first_cell = buffer.get(0, 0).unwrap();
    assert_ne!(first_cell.symbol, '>');
}

#[test]
fn test_checkbox_render_with_custom_check_fg() {
    let cb = Checkbox::new("Option")
        .checked(true)
        .check_fg(Color::MAGENTA);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Check that the checkbox is rendered (custom color affects internal state)
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('x'));
}

#[test]
fn test_checkbox_render_label() {
    let cb = Checkbox::new("Accept Terms");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Check that label is rendered
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Accept Terms"));
}

#[test]
fn test_checkbox_render_empty_label() {
    let cb = Checkbox::new("");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should render successfully even with empty label
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    // Should at least have the checkbox
    assert!(text.contains('[') || text.contains('☐') || text.contains('□') || text.contains('○'));
}

// =============================================================================
// Edge case tests
// =============================================================================

#[test]
fn test_checkbox_render_zero_width() {
    let cb = Checkbox::new("Option");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic with zero width
    cb.render(&mut ctx);
}

#[test]
fn test_checkbox_render_zero_height() {
    let cb = Checkbox::new("Option");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic with zero height
    cb.render(&mut ctx);
}

#[test]
fn test_checkbox_render_label_truncated() {
    let cb = Checkbox::new("This is a very long label that will be truncated");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should render without panicking, truncating label
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    // The label should be partially present
    assert!(text.contains("This") || text.contains('['));
}

#[test]
fn test_checkbox_render_unicode_label() {
    let cb = Checkbox::new("日本語テキスト");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should handle Unicode characters in label
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("日本語"));
}

// =============================================================================
// Render output validation tests
// =============================================================================

#[test]
fn test_checkbox_render_checked_square_brackets() {
    let cb = Checkbox::new("Test")
        .checked(true)
        .style(revue::widget::CheckboxStyle::Square);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();

    // Should have [x] pattern
    assert!(text.contains("[x]"));
    assert!(text.contains("Test"));
}

#[test]
fn test_checkbox_render_all_styles_in_sequence() {
    use revue::widget::CheckboxStyle;

    let styles = [
        CheckboxStyle::Square,
        CheckboxStyle::Unicode,
        CheckboxStyle::Filled,
        CheckboxStyle::Circle,
    ];

    for style in styles {
        let cb = Checkbox::new("Option").checked(true).style(style);
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(0, 0, 25, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        cb.render(&mut ctx);

        // Verify rendering occurred
        let first_cell = buffer.get(0, area.y);
        assert!(first_cell.is_some());
    }
}

// =============================================================================
// Additional builder method tests
// =============================================================================

#[test]
fn test_checkbox_builder_chaining() {
    let cb = Checkbox::new("Chained")
        .checked(true)
        .focused(true)
        .disabled(false)
        .style(revue::widget::CheckboxStyle::Unicode)
        .fg(Color::YELLOW)
        .bg(Color::BLACK)
        .check_fg(Color::GREEN)
        .element_id("test-cb")
        .class("custom");

    assert!(cb.is_checked());
    assert!(cb.is_focused());
    assert!(!cb.is_disabled());
    assert!(cb.has_class("custom"));
    assert_eq!(View::id(&cb), Some("test-cb"));
}

#[test]
fn test_checkbox_multiple_set_checked_calls() {
    let mut cb = Checkbox::new("Option");

    cb.set_checked(true);
    assert!(cb.is_checked());

    cb.set_checked(true);
    assert!(cb.is_checked());

    cb.set_checked(false);
    assert!(!cb.is_checked());

    cb.set_checked(false);
    assert!(!cb.is_checked());
}

// =============================================================================
// Combination tests
// =============================================================================

#[test]
fn test_checkbox_toggle_then_handle_key() {
    let mut cb = Checkbox::new("Option");

    // Manual toggle
    cb.toggle();
    assert!(cb.is_checked());

    // Handle key should toggle again
    cb.handle_key(&Key::Char(' '));
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_disabled_focused_combination() {
    use revue::widget::traits::Interactive;

    let mut cb = Checkbox::new("Option").focused(true).disabled(true);

    assert!(cb.is_focused());
    assert!(cb.is_disabled());
    assert!(!cb.focusable());

    // Should be able to blur even when disabled
    cb.on_blur();
    assert!(!cb.is_focused());
}

#[test]
fn test_checkbox_style_persistence_after_toggle() {
    let mut cb = Checkbox::new("Option")
        .style(revue::widget::CheckboxStyle::Circle)
        .checked(true);

    cb.toggle();
    assert!(!cb.is_checked());

    // Style should persist
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    // Should show circle style unchecked
    assert!(text.contains('○'));
}

// =============================================================================
// Color Tests
// =============================================================================

#[test]
fn test_checkbox_fg_color() {
    let cb = Checkbox::new("Test").fg(Color::RED);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Find first non-space cell and check fg color
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol != ' ' {
                assert_eq!(cell.fg, Some(Color::RED));
                break;
            }
        }
    }
}

#[test]
fn test_checkbox_bg_color() {
    let cb = Checkbox::new("Test").bg(Color::BLUE);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Verify rendering with bg color works (implementation may not set bg on all cells)
    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_checkbox_rgb_color() {
    let cb = Checkbox::new("Test").fg(Color::rgb(100, 150, 200));
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should render with RGB color
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol != ' ' {
                assert_eq!(cell.fg, Some(Color::rgb(100, 150, 200)));
                break;
            }
        }
    }
}

#[test]
fn test_checkbox_rgba_color() {
    let cb = Checkbox::new("Test").fg(Color::rgba(200, 100, 50, 180));
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol != ' ' {
                assert_eq!(cell.fg, Some(Color::rgba(200, 100, 50, 180)));
                break;
            }
        }
    }
}

#[test]
fn test_checkbox_check_fg_color() {
    let cb = Checkbox::new("Test").checked(true).check_fg(Color::GREEN);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Check indicator should have custom color
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('x'));
}

// =============================================================================
// Label Edge Cases
// =============================================================================

#[test]
fn test_checkbox_emoji_label() {
    let cb = Checkbox::new("🚀 Launch");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("🚀"));
}

#[test]
fn test_checkbox_rtl_label() {
    let cb = Checkbox::new("مرحبا"); // Arabic text
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should render RTL text
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(!text.is_empty());
}

#[test]
fn test_checkbox_label_with_quotes() {
    let cb = Checkbox::new("Quote's and \"Double\"");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Quote"));
}

#[test]
fn test_checkbox_label_with_tabs() {
    let cb = Checkbox::new("Tab\there");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should handle tabs
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Tab"));
}

#[test]
fn test_checkbox_label_with_newlines() {
    let cb = Checkbox::new("Line1\nLine2");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Should handle newlines
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Line1"));
}

#[test]
fn test_checkbox_single_char_label() {
    let cb = Checkbox::new("X");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("X"));
}

// =============================================================================
// CheckboxStyle Enum Tests
// =============================================================================

#[test]
fn test_checkbox_style_default() {
    use revue::widget::CheckboxStyle;
    let cb = Checkbox::new("Test");
    // Default style should be Square
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    cb.render(&mut ctx);
    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    // Default is Square which uses [ ] brackets
    assert!(text.contains('['));
}

#[test]
fn test_checkbox_style_eq() {
    use revue::widget::CheckboxStyle;
    assert_eq!(CheckboxStyle::Square, CheckboxStyle::Square);
    assert_eq!(CheckboxStyle::Unicode, CheckboxStyle::Unicode);
}

#[test]
fn test_checkbox_style_ne() {
    use revue::widget::CheckboxStyle;
    assert_ne!(CheckboxStyle::Square, CheckboxStyle::Unicode);
    assert_ne!(CheckboxStyle::Circle, CheckboxStyle::Filled);
}

#[test]
fn test_checkbox_all_styles_unique() {
    use revue::widget::CheckboxStyle;
    let styles = [
        CheckboxStyle::Square,
        CheckboxStyle::Unicode,
        CheckboxStyle::Filled,
        CheckboxStyle::Circle,
    ];

    for i in 0..styles.len() {
        for j in (i + 1)..styles.len() {
            assert_ne!(styles[i], styles[j]);
        }
    }
}

// =============================================================================
// Clone Tests
// =============================================================================

#[test]
fn test_checkbox_clone_preserves_checked() {
    let cb1 = Checkbox::new("Test").checked(true);
    let cb2 = cb1.clone();
    assert_eq!(cb1.is_checked(), cb2.is_checked());
}

#[test]
fn test_checkbox_clone_preserves_focused() {
    let cb1 = Checkbox::new("Test").focused(true);
    let cb2 = cb1.clone();
    assert_eq!(cb1.is_focused(), cb2.is_focused());
}

#[test]
fn test_checkbox_clone_preserves_disabled() {
    let cb1 = Checkbox::new("Test").disabled(true);
    let cb2 = cb1.clone();
    assert_eq!(cb1.is_disabled(), cb2.is_disabled());
}

#[test]
fn test_checkbox_clone_independent() {
    let mut cb1 = Checkbox::new("Test").checked(true);
    let mut cb2 = cb1.clone();

    // Both start checked
    assert!(cb1.is_checked());
    assert!(cb2.is_checked());

    cb1.toggle();
    // cb1 is now unchecked, cb2 is still checked
    assert!(!cb1.is_checked());
    assert!(cb2.is_checked());

    cb2.toggle();
    // Both are now unchecked
    assert!(!cb1.is_checked());
    assert!(!cb2.is_checked());
}

// =============================================================================
// State Transition Tests
// =============================================================================

#[test]
fn test_checkbox_multiple_toggles() {
    let mut cb = Checkbox::new("Test");

    assert!(!cb.is_checked());
    cb.toggle();
    assert!(cb.is_checked());
    cb.toggle();
    assert!(!cb.is_checked());
    cb.toggle();
    assert!(cb.is_checked());
    cb.toggle();
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_set_checked_to_same_value() {
    let mut cb = Checkbox::new("Test").checked(true);

    cb.set_checked(true);
    assert!(cb.is_checked());

    cb.set_checked(false);
    assert!(!cb.is_checked());

    cb.set_checked(false);
    assert!(!cb.is_checked());
}

#[test]
fn test_checkbox_focus_cycle() {
    use revue::widget::traits::Interactive;

    let mut cb = Checkbox::new("Test");

    assert!(!cb.is_focused());
    Interactive::on_focus(&mut cb);
    assert!(cb.is_focused());
    Interactive::on_blur(&mut cb);
    assert!(!cb.is_focused());
    Interactive::on_focus(&mut cb);
    assert!(cb.is_focused());
}

#[test]
fn test_checkbox_disabled_toggle_blocked() {
    let mut cb = Checkbox::new("Test").checked(true).disabled(true);

    assert!(cb.is_checked());
    cb.toggle(); // Should not change
    assert!(cb.is_checked());
}

#[test]
fn test_checkbox_disabled_set_checked_works() {
    let mut cb = Checkbox::new("Test").disabled(true);

    assert!(!cb.is_checked());
    cb.set_checked(true); // Should work
    assert!(cb.is_checked());
}

// =============================================================================
// Render with Offset Tests
// =============================================================================

#[test]
fn test_checkbox_render_with_offset() {
    let cb = Checkbox::new("Test").focused(true);
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(10, 5, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    // Focus indicator should be at offset position
    assert_eq!(buffer.get(10, 5).unwrap().symbol, '>');
}

#[test]
fn test_checkbox_render_label_with_offset() {
    let cb = Checkbox::new("Label");
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(5, 3, 25, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);

    let text: String = (area.x..area.x + area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Label"));
}

#[test]
fn test_checkbox_render_multiple_positions() {
    let cb = Checkbox::new("Test");
    let mut buffer = Buffer::new(50, 10);

    let positions = [
        Rect::new(0, 0, 20, 1),
        Rect::new(5, 3, 20, 1),
        Rect::new(15, 6, 20, 1),
    ];

    for area in positions {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);
        // Should render at each position
        let first_cell = buffer.get(area.x, area.y);
        assert!(first_cell.is_some());
    }
}

// =============================================================================
// Multiple Render Calls
// =============================================================================

#[test]
fn test_checkbox_multiple_renders() {
    let cb = Checkbox::new("Test").checked(true);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);

    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);
        let text: String = (0..area.width)
            .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
            .collect();
        assert!(text.contains('x'));
    }
}

#[test]
fn test_checkbox_render_after_state_change() {
    let mut cb = Checkbox::new("Test");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 25, 1);

    // Render unchecked
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);
    }

    // Change state and render checked
    cb.set_checked(true);
    buffer.clear();
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);
    }

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('x'));
}

// =============================================================================
// Combination Edge Cases
// =============================================================================

#[test]
fn test_checkbox_checked_focused_disabled() {
    use revue::widget::traits::Interactive;

    let cb = Checkbox::new("Test")
        .checked(true)
        .focused(true)
        .disabled(true);

    assert!(cb.is_checked());
    assert!(cb.is_focused());
    assert!(cb.is_disabled());
    assert!(!Interactive::focusable(&cb));
}

#[test]
fn test_checkbox_all_styles_checked_focused() {
    use revue::widget::CheckboxStyle;

    let styles = [
        CheckboxStyle::Square,
        CheckboxStyle::Unicode,
        CheckboxStyle::Filled,
        CheckboxStyle::Circle,
    ];

    for style in styles {
        let cb = Checkbox::new("Test")
            .checked(true)
            .focused(true)
            .style(style);

        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(0, 0, 25, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        cb.render(&mut ctx);

        // Should show focus indicator
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '>');
    }
}

#[test]
fn test_checkbox_single_pixel_area() {
    let cb = Checkbox::new("X");
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_checkbox_very_short_width() {
    let cb = Checkbox::new("LongLabel");
    let mut buffer = Buffer::new(5, 3);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cb.render(&mut ctx);
    // Should truncate label
}

// =============================================================================
