//! Theme picker widget tests

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{theme_picker, Interactive, StyledView, ThemePicker, View};

// =========================================================================
// Constructor and builder tests
// =========================================================================

#[test]
fn test_theme_picker_new() {
    let picker = ThemePicker::new();
    // Should have themes from theme_ids()
    assert!(picker.selected_id().is_some());
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_default() {
    let picker = ThemePicker::default();
    // Should have themes from theme_ids()
    assert!(picker.selected_id().is_some());
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_helper() {
    let picker = theme_picker();
    // Should have themes from theme_ids()
    assert!(picker.selected_id().is_some());
}

#[test]
fn test_theme_picker_themes_builder() {
    let picker = ThemePicker::new().themes(["dark", "light", "dracula"]);

    assert_eq!(picker.selected_id(), Some("dark"));
    assert!(picker.selected_theme().is_some());
}

#[test]
fn test_theme_picker_themes_builder_vec() {
    let themes = vec!["nord", "github"];
    let picker = ThemePicker::new().themes(themes);

    assert_eq!(picker.selected_id(), Some("nord"));
    assert!(picker.selected_theme().is_some());
}

#[test]
fn test_theme_picker_compact() {
    // Compact mode affects rendering, test that it can be set
    let picker = ThemePicker::new().compact(true);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should render without panic in compact mode
    picker.render(&mut ctx);

    let picker = ThemePicker::new().compact(false);
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_show_preview() {
    // show_preview affects rendering, test that it can be set
    let picker = ThemePicker::new().show_preview(true);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    let picker = ThemePicker::new().show_preview(false);
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_width() {
    let picker = ThemePicker::new().width(50);
    // Width affects rendering, verify it renders without panic
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    let picker = ThemePicker::new().width(100);
    let mut buffer = Buffer::new(100, 5);
    let area = Rect::new(0, 0, 100, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_fg() {
    let picker = ThemePicker::new().fg(Color::RED);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should render with custom foreground
    picker.render(&mut ctx);

    let picker = ThemePicker::new().fg(Color::rgb(255, 128, 0));
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_bg() {
    let picker = ThemePicker::new().bg(Color::BLUE);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should render with custom background
    picker.render(&mut ctx);

    let picker = ThemePicker::new().bg(Color::rgb(0, 128, 255));
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_chained_builders() {
    let picker = ThemePicker::new()
        .themes(["dracula", "nord"])
        .compact(true)
        .show_preview(false)
        .width(40)
        .fg(Color::GREEN)
        .bg(Color::BLACK);

    // Verify the picker works as expected
    assert_eq!(picker.selected_id(), Some("dracula"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should render with all builder settings applied
    picker.render(&mut ctx);
}

// =========================================================================
// State management tests
// =========================================================================

#[test]
fn test_theme_picker_toggle() {
    let mut picker = ThemePicker::new();
    assert!(!picker.is_open());

    picker.toggle();
    assert!(picker.is_open());

    picker.toggle();
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_open_close() {
    let mut picker = ThemePicker::new();
    assert!(!picker.is_open());

    picker.open();
    assert!(picker.is_open());

    picker.close();
    assert!(!picker.is_open());

    // Close when already closed should be safe
    picker.close();
    assert!(!picker.is_open());

    // Open when already open should be safe
    picker.open();
    picker.open();
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_is_open() {
    let mut picker = ThemePicker::new();
    assert!(!picker.is_open());

    picker.open();
    assert!(picker.is_open());

    picker.close();
    assert!(!picker.is_open());
}

// =========================================================================
// Selection navigation tests
// =========================================================================

#[test]
fn test_theme_picker_select_next() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);

    assert_eq!(picker.selected_id(), Some("dark"));

    picker.select_next();
    assert_eq!(picker.selected_id(), Some("light"));

    picker.select_next();
    assert_eq!(picker.selected_id(), Some("dracula"));

    // Should not go past end
    picker.select_next();
    assert_eq!(picker.selected_id(), Some("dracula"));

    picker.select_next();
    picker.select_next();
    assert_eq!(picker.selected_id(), Some("dracula"));
}

#[test]
fn test_theme_picker_select_prev() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.select_next();
    picker.select_next();

    assert_eq!(picker.selected_id(), Some("dracula"));

    picker.select_prev();
    assert_eq!(picker.selected_id(), Some("light"));

    picker.select_prev();
    assert_eq!(picker.selected_id(), Some("dark"));

    // Should not go below 0
    picker.select_prev();
    assert_eq!(picker.selected_id(), Some("dark"));

    picker.select_prev();
    picker.select_prev();
    assert_eq!(picker.selected_id(), Some("dark"));
}

#[test]
fn test_theme_picker_select_empty_themes() {
    let mut picker = ThemePicker::new().themes::<Vec<&str>, _>(vec![]);

    picker.select_next();
    assert_eq!(picker.selected_id(), None);

    picker.select_prev();
    assert_eq!(picker.selected_id(), None);
}

#[test]
fn test_theme_picker_select_single_theme() {
    let mut picker = ThemePicker::new().themes(["dark"]);

    assert_eq!(picker.selected_id(), Some("dark"));

    picker.select_next();
    assert_eq!(picker.selected_id(), Some("dark"));

    picker.select_prev();
    assert_eq!(picker.selected_id(), Some("dark"));
}

// =========================================================================
// Theme selection and application tests
// =========================================================================

#[test]
fn test_theme_picker_selected_id() {
    let picker = ThemePicker::new().themes(["dracula", "nord", "github"]);

    assert_eq!(picker.selected_id(), Some("dracula"));

    let picker = ThemePicker::new().themes::<Vec<&str>, _>(vec![]);
    assert_eq!(picker.selected_id(), None);
}

#[test]
fn test_theme_picker_selected_id_after_navigation() {
    let mut picker = ThemePicker::new().themes(["dracula", "nord", "github"]);

    assert_eq!(picker.selected_id(), Some("dracula"));

    picker.select_next();
    assert_eq!(picker.selected_id(), Some("nord"));

    picker.select_next();
    assert_eq!(picker.selected_id(), Some("github"));

    picker.select_prev();
    assert_eq!(picker.selected_id(), Some("nord"));
}

#[test]
fn test_theme_picker_selected_theme() {
    let picker = ThemePicker::new().themes(["dark", "light"]);

    let theme = picker.selected_theme();
    assert!(theme.is_some());
    let theme = theme.unwrap();
    // Theme name may be capitalized (e.g., "Dark")
    assert!(theme.name.to_lowercase() == "dark");
}

#[test]
fn test_theme_picker_selected_theme_invalid_id() {
    let picker = ThemePicker::new().themes(["nonexistent_theme_id"]);

    let theme = picker.selected_theme();
    // Should return None for invalid theme ID
    assert!(theme.is_none());
}

#[test]
fn test_theme_picker_selected_theme_empty() {
    let picker = ThemePicker::new().themes::<Vec<&str>, _>(vec![]);

    let theme = picker.selected_theme();
    assert!(theme.is_none());
}

#[test]
fn test_theme_picker_apply_selected() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);

    // Should not panic when applying
    picker.apply_selected();

    picker.select_next();
    picker.apply_selected();

    picker.select_next();
    picker.apply_selected();
}

#[test]
fn test_theme_picker_apply_selected_empty() {
    let picker = ThemePicker::new().themes::<Vec<&str>, _>(vec![]);

    // Should not panic when applying with no themes
    picker.apply_selected();
}

// =========================================================================
// Keyboard event handling tests
// =========================================================================

#[test]
fn test_theme_picker_handle_key_enter_to_open() {
    let mut picker = ThemePicker::new();

    let event = KeyEvent::new(Key::Enter);
    let result = picker.handle_key(&event);

    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_space_to_open() {
    let mut picker = ThemePicker::new();

    let event = KeyEvent::new(Key::Char(' '));
    let result = picker.handle_key(&event);

    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_enter_to_close_and_apply() {
    let mut picker = ThemePicker::new().themes(["dark", "light"]);
    picker.open();

    assert!(picker.is_open());

    let event = KeyEvent::new(Key::Enter);
    let result = picker.handle_key(&event);

    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert!(!picker.is_open());
    // Theme should be applied
}

#[test]
fn test_theme_picker_handle_key_escape_to_close() {
    let mut picker = ThemePicker::new();
    picker.open();

    assert!(picker.is_open());

    let event = KeyEvent::new(Key::Escape);
    let result = picker.handle_key(&event);

    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_escape_when_closed() {
    let mut picker = ThemePicker::new();

    assert!(!picker.is_open());

    let event = KeyEvent::new(Key::Escape);
    let result = picker.handle_key(&event);

    // Escape is ignored when dropdown is not open
    assert_eq!(result, revue::widget::traits::EventResult::Ignored);
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_down_when_open() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();

    assert_eq!(picker.selected_id(), Some("dark"));

    let event = KeyEvent::new(Key::Down);
    let result = picker.handle_key(&event);

    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert_eq!(picker.selected_id(), Some("light"));
}

#[test]
fn test_theme_picker_handle_key_up_when_open() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();
    picker.select_next();

    assert_eq!(picker.selected_id(), Some("light"));

    let event = KeyEvent::new(Key::Up);
    let result = picker.handle_key(&event);

    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert_eq!(picker.selected_id(), Some("dark"));
}

#[test]
fn test_theme_picker_handle_key_j_vim_down() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();

    let event = KeyEvent::new(Key::Char('j'));
    let result = picker.handle_key(&event);

    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert_eq!(picker.selected_id(), Some("light"));
}

#[test]
fn test_theme_picker_handle_key_k_vim_up() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();
    picker.select_next();

    let event = KeyEvent::new(Key::Char('k'));
    let result = picker.handle_key(&event);

    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert_eq!(picker.selected_id(), Some("dark"));
}

#[test]
fn test_theme_picker_handle_key_navigation_when_closed() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);

    // Navigation keys should be ignored when closed
    let result = picker.handle_key(&KeyEvent::new(Key::Down));
    assert_eq!(result, revue::widget::traits::EventResult::Ignored);

    let result = picker.handle_key(&KeyEvent::new(Key::Up));
    assert_eq!(result, revue::widget::traits::EventResult::Ignored);

    let result = picker.handle_key(&KeyEvent::new(Key::Char('j')));
    assert_eq!(result, revue::widget::traits::EventResult::Ignored);

    let result = picker.handle_key(&KeyEvent::new(Key::Char('k')));
    assert_eq!(result, revue::widget::traits::EventResult::Ignored);
}

#[test]
fn test_theme_picker_handle_key_tab() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);

    assert_eq!(picker.selected_id(), Some("dark"));

    let result = picker.handle_key(&KeyEvent::new(Key::Tab));
    assert_eq!(result, revue::widget::traits::EventResult::Consumed);
    assert_eq!(picker.selected_id(), Some("light"));

    picker.handle_key(&KeyEvent::new(Key::Tab));
    assert_eq!(picker.selected_id(), Some("dracula"));

    // Note: Tab doesn't wrap around - stays at last theme
    picker.handle_key(&KeyEvent::new(Key::Tab));
    assert_eq!(picker.selected_id(), Some("dracula"));
}

#[test]
fn test_theme_picker_handle_key_unknown() {
    let mut picker = ThemePicker::new();

    let result = picker.handle_key(&KeyEvent::new(Key::Char('x')));
    assert_eq!(result, revue::widget::traits::EventResult::Ignored);

    let result = picker.handle_key(&KeyEvent::new(Key::Char('a')));
    assert_eq!(result, revue::widget::traits::EventResult::Ignored);
}

// =========================================================================
// Rendering tests
// =========================================================================

#[test]
fn test_theme_picker_render_too_small_width() {
    let picker = ThemePicker::new();
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic with too small width
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_render_too_small_height() {
    let picker = ThemePicker::new();
    let mut buffer = Buffer::new(20, 0);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic with zero height
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_render_basic() {
    let picker = ThemePicker::new();
    let mut buffer = Buffer::new(35, 10);
    let area = Rect::new(0, 0, 35, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    // Should render "Theme: " label
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'T');
}

#[test]
fn test_theme_picker_render_closed() {
    let picker = ThemePicker::new();
    let mut buffer = Buffer::new(35, 10);
    let area = Rect::new(0, 0, 35, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    // Should render "Theme: " label at start
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'T');

    // Should show dropdown indicator somewhere on the line
    let mut found_arrow = false;
    for x in 0..35 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '▼' {
                found_arrow = true;
                break;
            }
        }
    }
    assert!(found_arrow, "Should find dropdown arrow indicator");
}

#[test]
fn test_theme_picker_render_open() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();

    let mut buffer = Buffer::new(35, 10);
    let area = Rect::new(0, 0, 35, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    // Should show dropdown indicator for open state (up arrow) somewhere
    let mut found_arrow = false;
    for x in 0..35 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == '▲' {
                found_arrow = true;
                break;
            }
        }
    }
    assert!(found_arrow, "Should find up arrow when open");

    // Should show top border of dropdown
    let cell = buffer.get(0, 1).unwrap();
    assert_eq!(cell.symbol, '┌');
}

#[test]
fn test_theme_picker_render_compact_closed() {
    let picker = ThemePicker::new().compact(true);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    // In compact mode, should show color swatch at position 0
    // The swatch has 4 colors
    for i in 0..4 {
        let cell = buffer.get(i, 0).unwrap();
        assert_eq!(cell.symbol, ' ');
        // Should have background color
        assert!(cell.bg.is_some());
    }
}

#[test]
fn test_theme_picker_render_compact_open() {
    let mut picker = ThemePicker::new().themes(["dark", "light"]).compact(true);
    picker.open();

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    // First theme should have selection indicator '>'
    let cell = buffer.get(0, 1).unwrap();
    assert_eq!(cell.symbol, '>');
}

#[test]
fn test_theme_picker_render_with_custom_width() {
    let picker = ThemePicker::new().width(50);
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    // Should render up to custom width
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'T');
}

#[test]
fn test_theme_picker_render_dropdown_content_clipped() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula", "nord", "github"]);
    picker.open();

    let mut buffer = Buffer::new(35, 3);
    let area = Rect::new(0, 0, 35, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    // Should clip content that doesn't fit
    // Should show bottom border
    let cell = buffer.get(0, 2).unwrap();
    assert_eq!(cell.symbol, '└');

    let cell = buffer.get(34, 2).unwrap();
    assert_eq!(cell.symbol, '┘');
}

#[test]
fn test_theme_picker_render_theme_name_truncation() {
    let picker = ThemePicker::new().width(15);
    let mut buffer = Buffer::new(15, 5);
    let area = Rect::new(0, 0, 15, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    // Should render without panic even with narrow width
    // Theme name should be truncated if needed
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'T');
}

// =========================================================================
// CSS integration tests
// =========================================================================

#[test]
fn test_theme_picker_css_id() {
    let picker = ThemePicker::new().element_id("theme-selector");

    // Verify the picker can be created with element_id
    // and renders without issues
    let mut buffer = Buffer::new(35, 10);
    let area = Rect::new(0, 0, 35, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_css_classes() {
    let picker = ThemePicker::new().class("dropdown").class("themed");

    // Verify the picker can be created with classes
    // and renders without issues
    let mut buffer = Buffer::new(35, 10);
    let area = Rect::new(0, 0, 35, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_styled_view() {
    let mut picker = ThemePicker::new();

    // Test CSS methods work without panicking
    picker.set_id("test-picker");
    picker.add_class("active");
    picker.remove_class("active");
    picker.toggle_class("selected");
    picker.toggle_class("selected");

    // Should render successfully
    let mut buffer = Buffer::new(35, 10);
    let area = Rect::new(0, 0, 35, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_inline_color_override() {
    let picker = ThemePicker::new().fg(Color::RED).bg(Color::BLUE);

    let mut buffer = Buffer::new(35, 5);
    let area = Rect::new(0, 0, 35, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should render with custom colors
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_combined_with_classes() {
    let picker = ThemePicker::new()
        .themes(["dark", "light"])
        .element_id("my-picker")
        .class("custom")
        .class("picker")
        .compact(true)
        .width(30)
        .fg(Color::GREEN)
        .bg(Color::BLACK);

    assert_eq!(picker.selected_id(), Some("dark"));

    // Should render with all settings
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_theme_picker_with_empty_themes_list() {
    let mut picker = ThemePicker::new().themes::<Vec<&str>, _>(vec![]);

    assert_eq!(picker.selected_id(), None);
    assert!(picker.selected_theme().is_none());

    // Should not panic on operations
    picker.select_next();
    picker.select_prev();
    picker.apply_selected();
}

#[test]
fn test_theme_picker_with_single_theme() {
    let mut picker = ThemePicker::new().themes(["dark"]);

    assert_eq!(picker.selected_id(), Some("dark"));

    picker.select_next();
    assert_eq!(picker.selected_id(), Some("dark"));

    picker.select_prev();
    assert_eq!(picker.selected_id(), Some("dark"));
}

#[test]
fn test_theme_picker_multiple_open_close() {
    let mut picker = ThemePicker::new();

    for _ in 0..10 {
        picker.open();
        assert!(picker.is_open());
        picker.close();
        assert!(!picker.is_open());
        picker.toggle();
        assert!(picker.is_open());
        picker.toggle();
        assert!(!picker.is_open());
    }
}

#[test]
fn test_theme_picker_render_with_zero_width() {
    let picker = ThemePicker::new();
    let mut buffer = Buffer::new(0, 10);
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic with zero width
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_render_with_minimal_area() {
    let picker = ThemePicker::new();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should render with minimal valid area
    picker.render(&mut ctx);
}

#[test]
fn test_theme_picker_navigation_bounds() {
    let mut picker = ThemePicker::new().themes(["a", "b", "c", "d", "e"]);

    // Navigate to end
    for _ in 0..10 {
        picker.select_next();
    }
    assert_eq!(picker.selected_id(), Some("e"));

    // Navigate to start
    for _ in 0..10 {
        picker.select_prev();
    }
    assert_eq!(picker.selected_id(), Some("a"));
}

#[test]
fn test_theme_picker_clone() {
    let picker1 = ThemePicker::new()
        .themes(["dark", "light"])
        .compact(true)
        .width(25)
        .fg(Color::RED)
        .bg(Color::BLUE);

    let picker2 = picker1.clone();

    // Both should have same selected ID
    assert_eq!(picker1.selected_id(), picker2.selected_id());
    assert!(picker1.is_open() == picker2.is_open());

    let mut buffer = Buffer::new(25, 10);
    let area = Rect::new(0, 0, 25, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Both should render the same
    picker1.render(&mut ctx);
    picker2.render(&mut ctx);
}

#[test]
fn test_theme_picker_debug_format() {
    let picker = ThemePicker::new().themes(["dark", "light"]).compact(true);

    let debug_str = format!("{:?}", picker);
    assert!(debug_str.contains("ThemePicker"));
}
