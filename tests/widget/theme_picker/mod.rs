//! ThemePicker widget tests

mod keys;
mod view;

use revue::event::Key;
use revue::event::KeyEvent;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::style::{get_theme, set_theme_by_id, use_theme};
use revue::widget::theme_picker;
use revue::widget::traits::RenderContext;
use revue::widget::Interactive;
use revue::widget::StyledView;
use revue::widget::ThemePicker;
use revue::widget::View;
use serial_test::serial;

// =========================================================================
// Constructor and builder tests
// =========================================================================

#[test]
fn test_theme_picker_new() {
    let picker = ThemePicker::new();
    // Should have themes from theme_ids()
    assert!(picker.selected_id().is_some());
    assert!(!picker.is_open());

    let _picker = ThemePicker::new();
    // Picker created successfully
}

#[test]
fn test_theme_picker_default() {
    let picker = ThemePicker::default();
    // Should have themes from theme_ids()
    assert!(picker.selected_id().is_some());
    assert!(!picker.is_open());

    let _picker = ThemePicker::default();
    // Default picker created
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
    let current = use_theme().get();
    let picker = ThemePicker::new().compact(true);
    let row = rendered_row(&picker, 20, 0);
    assert_eq!(row.trim(), "", "compact draws swatches, no text: {row:?}");
    assert_eq!(swatch_cells(&picker, 20, 0, &current), 4);

    let full = ThemePicker::new().compact(false);
    assert!(rendered_row(&full, 20, 0).starts_with("Theme: "));
}

fn render_buf(picker: &ThemePicker, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    picker.render(&mut ctx);
    buffer
}

fn rendered_row(picker: &ThemePicker, width: u16, y: u16) -> String {
    let mut buffer = Buffer::new(width, 6);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 6));
    picker.render(&mut ctx);
    (0..width)
        .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .collect()
}

/// Columns whose background is one of `theme`'s swatch colors.
fn swatch_cells(picker: &ThemePicker, width: u16, y: u16, theme: &revue::style::Theme) -> usize {
    let mut buffer = Buffer::new(width, 6);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 6));
    picker.render(&mut ctx);
    let swatch = [
        theme.colors.background,
        theme.palette.primary,
        theme.palette.success,
        theme.palette.error,
    ];
    (0..width)
        .filter(|&x| {
            buffer
                .get(x, y)
                .and_then(|c| c.bg)
                .is_some_and(|bg| swatch.contains(&bg))
        })
        .count()
}

#[test]
fn test_theme_picker_header_shows_the_swatch_by_default() {
    let current = revue::style::use_theme().get();
    let picker = ThemePicker::new();
    assert!(picker_shows_preview(&picker, &current));
}

#[test]
fn test_theme_picker_without_preview_leaves_the_swatch_out_of_the_header() {
    let current = revue::style::use_theme().get();
    let picker = ThemePicker::new().show_preview(false);
    assert!(!picker_shows_preview(&picker, &current));
    assert!(
        rendered_row(&picker, 40, 0).starts_with(&format!("Theme: {} ▼", current.name)),
        "the name should be followed by the dropdown arrow: {:?}",
        rendered_row(&picker, 40, 0)
    );
}

#[test]
fn test_theme_picker_without_preview_lists_names_only() {
    let nord = revue::style::get_theme("nord").unwrap();
    let mut picker = ThemePicker::new().themes(["nord"]).show_preview(false);
    picker.open();

    // Row 1 is the dropdown's top border, row 2 the first theme
    assert!(
        rendered_row(&picker, 30, 2).starts_with(&format!("│▶ {}", nord.name)),
        "{:?}",
        rendered_row(&picker, 30, 2)
    );
    assert_eq!(swatch_cells(&picker, 30, 2, &nord), 0);

    let mut with_preview = ThemePicker::new().themes(["nord"]);
    with_preview.open();
    assert_eq!(swatch_cells(&with_preview, 30, 2, &nord), 4);
}

fn picker_shows_preview(picker: &ThemePicker, theme: &revue::style::Theme) -> bool {
    swatch_cells(picker, 40, 0, theme) == 4
}

#[test]
fn test_theme_picker_width() {
    let surface = use_theme().get().colors.surface;

    let buffer = render_buf(&ThemePicker::new().width(50), 100, 5);
    assert_eq!(buffer.get(49, 0).unwrap().bg, Some(surface));
    assert_eq!(buffer.get(50, 0).unwrap().bg, None);

    // Without a width the header is 35 columns
    let buffer = render_buf(&ThemePicker::new(), 100, 5);
    assert_eq!(buffer.get(34, 0).unwrap().bg, Some(surface));
    assert_eq!(buffer.get(35, 0).unwrap().bg, None);
}

#[test]
fn test_theme_picker_fg() {
    let buffer = render_buf(&ThemePicker::new().fg(Color::RED), 20, 5);
    for x in 0..7 {
        assert_eq!(buffer.get(x, 0).unwrap().fg, Some(Color::RED), "col {x}");
    }

    let text = use_theme().get().colors.text;
    let buffer = render_buf(&ThemePicker::new(), 20, 5);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(text));
}

#[test]
fn test_theme_picker_bg() {
    let buffer = render_buf(&ThemePicker::new().bg(Color::BLUE), 20, 5);
    for x in [0, 6, 19] {
        assert_eq!(buffer.get(x, 0).unwrap().bg, Some(Color::BLUE), "col {x}");
    }

    let surface = use_theme().get().colors.surface;
    let buffer = render_buf(&ThemePicker::new(), 20, 5);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(surface));
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

    let picker = ThemePicker::new();
    assert!(!picker.is_open());

    let mut picker = picker;
    picker.open();
    assert!(picker.is_open());
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

    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.select_next();
    // Select next works
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

    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.select_next(); // Move to second item
    picker.select_next(); // Move to third item
    picker.select_prev(); // Move back to second item
                          // Select prev works
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

    let picker = ThemePicker::new().themes(["dracula", "nord"]);
    assert_eq!(picker.selected_id(), Some("dracula"));
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

    let picker = ThemePicker::new();
    let _theme = picker.selected_theme();
    // Can get selected theme
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
#[serial]
fn test_theme_picker_apply_selected() {
    let mut picker = ThemePicker::new().themes(["light", "dracula", "nord"]);
    for id in ["light", "dracula", "nord"] {
        picker.apply_selected();
        assert_eq!(use_theme().get().name, get_theme(id).unwrap().name);
        picker.select_next();
    }
    set_theme_by_id("dark");
}

#[test]
#[serial]
fn test_theme_picker_apply_selected_empty() {
    set_theme_by_id("nord");
    let before = use_theme().get().name;

    ThemePicker::new()
        .themes::<Vec<&str>, _>(vec![])
        .apply_selected();
    assert_eq!(use_theme().get().name, before);

    // An unknown id applies nothing either
    ThemePicker::new().themes(["nonexistent"]).apply_selected();
    assert_eq!(use_theme().get().name, before);
    set_theme_by_id("dark");
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

    let mut picker = ThemePicker::new().themes(["dark", "light"]);
    let event = KeyEvent::new(Key::Tab);
    let _result = Interactive::handle_key(&mut picker, &event);
    // Tab cycles through themes
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
    for width in [5, 9] {
        let buffer = render_buf(&ThemePicker::new(), width, 1);
        for x in 0..width {
            let cell = buffer.get(x, 0).unwrap();
            assert_eq!((cell.symbol, cell.bg), (' ', None), "{width}: col {x}");
        }
    }
    assert!(rendered_row(&ThemePicker::new(), 10, 0).starts_with("Theme:"));
}

#[test]
fn test_theme_picker_render_too_small_height() {
    let picker = ThemePicker::new();
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    for y in 0..2 {
        for x in 0..20 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, ' ', "({x}, {y})");
        }
    }
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

    // The id does not change what is drawn. `View::id` does not report it
    // (ThemePicker's View has no view meta, draft-pk-1), so it is not checked here.
    for y in 0..2 {
        assert_eq!(
            rendered_row(&picker, 35, y),
            rendered_row(&ThemePicker::new(), 35, y)
        );
    }
}

#[test]
fn test_theme_picker_css_classes() {
    let picker = ThemePicker::new().class("dropdown").class("themed");

    // Checked through StyledView; `View::classes` reports none (draft-pk-1)
    assert!(picker.has_class("dropdown"));
    assert!(picker.has_class("themed"));
    assert!(!picker.has_class("other"));
}

#[test]
fn test_theme_picker_styled_view() {
    let mut picker = ThemePicker::new();

    picker.set_id("test-picker");
    picker.add_class("active");
    picker.remove_class("active");
    picker.toggle_class("selected");
    picker.toggle_class("selected");

    assert!(!picker.has_class("active"));
    assert!(!picker.has_class("selected"));

    picker.toggle_class("selected");
    assert!(picker.has_class("selected"));
}

#[test]
fn test_theme_picker_inline_color_override() {
    let current = use_theme().get();
    let picker = ThemePicker::new().fg(Color::RED).bg(Color::BLUE);
    let buffer = render_buf(&picker, 35, 5);

    let label = buffer.get(0, 0).unwrap();
    assert_eq!((label.fg, label.bg), (Some(Color::RED), Some(Color::BLUE)));
    assert_eq!(buffer.get(34, 0).unwrap().bg, Some(Color::BLUE));

    // The swatch keeps the theme's colors: "Theme: Dark" and a space, then 4 cells
    let swatch = 8 + current.name.chars().count() as u16;
    assert_eq!(
        buffer.get(swatch, 0).unwrap().bg,
        Some(current.colors.background)
    );
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
    picker.open();

    // One row in the box, marked, then the bottom border
    assert!(rendered_row(&picker, 30, 2).starts_with("│▶"));
    assert!(rendered_row(&picker, 30, 3).starts_with('└'));
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
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    for y in 0..10 {
        assert_eq!(buffer.get(0, y).unwrap().symbol, ' ', "row {y}");
    }
}

#[test]
fn test_theme_picker_render_with_minimal_area() {
    let picker = ThemePicker::new();
    let mut buffer = Buffer::new(12, 2);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);

    let row: String = (0..10).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
    assert!(row.starts_with("Theme:"), "{row:?}");
    // Nothing outside the area
    for (x, y) in [(10, 0), (11, 0), (0, 1), (9, 1)] {
        let cell = buffer.get(x, y).unwrap();
        assert_eq!((cell.symbol, cell.bg), (' ', None), "({x}, {y})");
    }
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

#[test]
fn test_theme_picker_themes() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();

    for (y, id) in [(2, "dark"), (3, "light"), (4, "dracula")] {
        let name = get_theme(id).unwrap().name;
        assert!(rendered_row(&picker, 30, y).contains(&name), "row {y}");
    }
}

#[test]
fn test_theme_picker_show_preview() {
    // Compact mode is made of swatches, so it ignores show_preview(false)
    let current = use_theme().get();
    let picker = ThemePicker::new().compact(true).show_preview(false);
    assert_eq!(swatch_cells(&picker, 20, 0, &current), 4);
}

#[test]
fn test_theme_picker_open() {
    let mut picker = ThemePicker::new();
    picker.open();
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_close() {
    let mut picker = ThemePicker::new();
    picker.open();
    picker.close();
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_open() {
    let mut picker = ThemePicker::new();
    let event = KeyEvent::new(Key::Enter);
    let _result = Interactive::handle_key(&mut picker, &event);
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_space() {
    let mut picker = ThemePicker::new();
    let event = KeyEvent::new(Key::Char(' '));
    let _result = Interactive::handle_key(&mut picker, &event);
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_close() {
    let mut picker = ThemePicker::new();
    picker.open();

    let event = KeyEvent::new(Key::Escape);
    let _result = Interactive::handle_key(&mut picker, &event);
    assert!(!picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_down() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();

    for expected in ["light", "dracula", "dracula"] {
        let result = Interactive::handle_key(&mut picker, &KeyEvent::new(Key::Down));
        assert_eq!(result, revue::widget::traits::EventResult::Consumed);
        assert_eq!(picker.selected_id(), Some(expected));
    }
}

#[test]
fn test_theme_picker_handle_key_up() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();
    picker.select_next();
    picker.select_next(); // Move to third item

    for expected in ["light", "dark", "dark"] {
        let result = Interactive::handle_key(&mut picker, &KeyEvent::new(Key::Up));
        assert_eq!(result, revue::widget::traits::EventResult::Consumed);
        assert_eq!(picker.selected_id(), Some(expected));
    }
}

#[test]
fn test_theme_picker_handle_key_j() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();

    for _ in 0..3 {
        Interactive::handle_key(&mut picker, &KeyEvent::new(Key::Char('j')));
    }
    assert_eq!(picker.selected_id(), Some("dracula"));
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_handle_key_k() {
    let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
    picker.open();
    picker.select_next();
    picker.select_next(); // Move to third item

    for _ in 0..3 {
        Interactive::handle_key(&mut picker, &KeyEvent::new(Key::Char('k')));
    }
    assert_eq!(picker.selected_id(), Some("dark"));
    assert!(picker.is_open());
}

#[test]
fn test_theme_picker_builder_pattern() {
    let current = use_theme().get();
    let picker = theme_picker()
        .themes(["dark", "light"])
        .compact(true)
        .width(40)
        .fg(Color::CYAN)
        .bg(Color::BLUE);

    assert_eq!(picker.selected_id(), Some("dark"));
    // Compact: the swatch and nothing after it
    let buffer = render_buf(&picker, 40, 3);
    assert_eq!(swatch_cells(&picker, 40, 0, &current), 4);
    for x in 4..40 {
        assert_eq!(buffer.get(x, 0).unwrap().bg, None, "col {x}");
    }
}

#[test]
fn test_theme_picker_single_theme() {
    let picker = ThemePicker::new().themes(["dark"]);
    assert_eq!(picker.selected_id(), Some("dark"));
}

mod snapshots {

    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_theme_picker_basic() {
        use revue::widget::ThemePicker;

        let view = ThemePicker::new();

        let config = TestConfig::with_size(40, 8);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("theme_picker_basic");
    }

    #[test]
    fn test_theme_picker_compact() {
        use revue::widget::ThemePicker;

        let view = ThemePicker::new().compact(true);

        let config = TestConfig::with_size(30, 5);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("theme_picker_compact");
    }
}
