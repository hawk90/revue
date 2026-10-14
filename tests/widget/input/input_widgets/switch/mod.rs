//! Switch widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView};
use revue::widget::{switch, toggle, Switch, SwitchStyle, ToggleWidget, View};

mod view;

const GRAY: Color = Color::rgb(100, 100, 100);

fn screen(s: &Switch, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    s.render(&mut ctx);
    buffer
}

/// Row `y`, without wide glyphs' continuation cells
fn row_of(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .filter(|&c| c != '\0')
        .collect()
}

fn row(s: &Switch, w: u16) -> String {
    row_of(&screen(s, w, 1), 0)
}

fn fg(buffer: &Buffer, x: u16) -> Option<Color> {
    buffer.get(x, 0).unwrap().fg
}

// ==================== Constructor Tests ====================

#[test]
fn test_switch_new() {
    let s = Switch::new();
    assert!(!s.is_on());
}

#[test]
fn test_switch_default() {
    let s = Switch::default();
    assert!(!s.is_on());
}

#[test]
fn test_switch_helper() {
    let s = switch().on(true);
    assert!(s.is_on());
}

#[test]
fn test_toggle_helper() {
    let s = toggle("Enable");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Enable"));
}

// ==================== State Tests ====================

#[test]
fn test_switch_on() {
    let s = Switch::new().on(true);
    assert!(s.is_on());

    let s = Switch::new().on(false);
    assert!(!s.is_on());
}

#[test]
fn test_switch_checked_alias() {
    // Test checked() is an alias for on()
    let s = Switch::new().checked(true);
    assert!(s.is_on());
    assert!(s.is_checked());

    let s = Switch::new().checked(false);
    assert!(!s.is_on());
    assert!(!s.is_checked());
}

#[test]
fn test_switch_toggle() {
    let mut s = Switch::new();
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());

    s.toggle();
    assert!(!s.is_on());
}

#[test]
fn test_switch_set() {
    let mut s = Switch::new();

    s.set(true);
    assert!(s.is_on());

    s.set(false);
    assert!(!s.is_on());
}

#[test]
fn test_switch_disabled_toggle() {
    let mut s = Switch::new().disabled(true);

    s.toggle();
    assert!(!s.is_on()); // Should not change when disabled

    s.set(true);
    assert!(!s.is_on()); // set() also respects disabled
}

// ==================== Builder Tests ====================

#[test]
fn test_switch_focused() {
    // A focus bracket after the switch
    assert_eq!(row(&Switch::new(), 8), "[●━━━]  ");
    assert_eq!(row(&Switch::new().focused(true), 8), "[●━━━]] ");
}

#[test]
fn test_switch_disabled() {
    // Disabled: no focus brackets, even when focused
    let s = Switch::new().focused(true).disabled(true);
    assert_eq!(row(&s, 8), "[●━━━]  ");
}

#[test]
fn test_switch_label() {
    assert_eq!(
        row(&Switch::new().label("Dark Mode"), 18),
        "Dark Mode [●━━━]  "
    );
}

#[test]
fn test_switch_label_right() {
    // Without a label it changes nothing; with one, the label follows
    assert_eq!(row(&Switch::new().label_right(), 8), "[●━━━]  ");
    let s = Switch::new().label_right().label("ab");
    assert_eq!(row(&s, 10), "[●━━━] ab ");
}

#[test]
fn test_switch_width() {
    assert_eq!(row(&Switch::new().width(10), 12), "[●━━━━━━━]  ");
}

#[test]
fn test_switch_width_minimum() {
    // Width 2 is raised to 4
    assert_eq!(row(&Switch::new().width(2), 6), "[●━]  ");
}

#[test]
fn test_switch_on_color() {
    let buffer = screen(&Switch::new().on(true).on_color(Color::CYAN), 6, 1);
    // The knob and the brackets
    for x in [0, 4, 5] {
        assert_eq!(fg(&buffer, x), Some(Color::CYAN), "{x}");
    }
}

#[test]
fn test_switch_off_color() {
    let off = Color::rgb(10, 20, 30);
    let buffer = screen(&Switch::new().off_color(off), 6, 1);
    for x in [0, 1, 5] {
        assert_eq!(fg(&buffer, x), Some(off), "{x}");
    }
}

#[test]
fn test_switch_track_color() {
    let buffer = screen(&Switch::new().track_color(Color::BLUE), 6, 1);
    // The three track cells after the knob
    for x in 2..5 {
        assert_eq!(fg(&buffer, x), Some(Color::BLUE), "{x}");
    }
    assert_ne!(fg(&buffer, 1), Some(Color::BLUE));
}

#[test]
fn test_switch_text() {
    // Used by the Text style: the on text and the off text
    let s = Switch::new().style(SwitchStyle::Text).text("YES", "NO");
    assert_eq!(row(&s, 6), "[NO]  ");
    assert_eq!(row(&s.on(true), 6), "[YES] ");
}

#[test]
fn test_switch_builder_chain() {
    let s = Switch::new()
        .on(true)
        .focused(true)
        .disabled(false)
        .label("Toggle")
        .label_right()
        .width(8)
        .style(SwitchStyle::IOS)
        .on_color(Color::GREEN)
        .off_color(Color::rgb(100, 100, 100))
        .track_color(Color::BLUE)
        .text("ON", "OFF");
    let buffer = screen(&s, 20, 1);
    // An 8-wide IOS track, on: the knob at its right end on the on color;
    // the focus bracket, then the label
    let knob = buffer.get(6, 0).unwrap();
    assert_eq!((knob.symbol, knob.bg), ('●', Some(Color::GREEN)));
    assert_eq!(buffer.get(7, 0).unwrap().symbol, ')');
    assert_eq!(buffer.get(8, 0).unwrap().symbol, ']');
    assert!(row_of(&buffer, 0).ends_with("]Toggle     "));
}

// ==================== Style Tests ====================

#[test]
fn test_switch_style_default() {
    let style = SwitchStyle::default();
    assert_eq!(style, SwitchStyle::Default);
}

#[test]
fn test_switch_style_all_variants() {
    // Each style draws the off switch differently
    let styles = [
        SwitchStyle::Default,
        SwitchStyle::IOS,
        SwitchStyle::Material,
        SwitchStyle::Text,
        SwitchStyle::Emoji,
        SwitchStyle::Block,
    ];
    let rows: std::collections::HashSet<String> = styles
        .iter()
        .map(|&st| row(&Switch::new().style(st), 8))
        .collect();
    assert_eq!(rows.len(), styles.len());
}

#[test]
fn test_switch_style_partial_eq() {
    assert_eq!(SwitchStyle::Default, SwitchStyle::Default);
    assert_eq!(SwitchStyle::IOS, SwitchStyle::IOS);
    assert_ne!(SwitchStyle::Default, SwitchStyle::IOS);
}

// ==================== Key Handling Tests ====================

#[test]
fn test_switch_handle_key_enter() {
    let mut s = Switch::new().focused(true);
    assert!(!s.is_on());

    assert!(s.handle_key(&Key::Enter));
    assert!(s.is_on());
}

#[test]
fn test_switch_handle_key_space() {
    let mut s = Switch::new().focused(true).on(true);

    assert!(s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_handle_key_unfocused() {
    let mut s = Switch::new().focused(false);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_handle_key_disabled() {
    let mut s = Switch::new().focused(true).disabled(true);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_handle_key_invalid() {
    let mut s = Switch::new().focused(true);

    assert!(!s.handle_key(&Key::Tab));
    assert!(!s.handle_key(&Key::Up));
    assert!(!s.handle_key(&Key::Char('x')));
    assert!(!s.handle_key(&Key::Char('z')));
    assert!(!s.is_on());
}

// ==================== Rendering Tests ====================

#[test]
fn test_switch_render_default() {
    let s = Switch::new().on(true);
    let buffer = screen(&s, 20, 1);
    assert_eq!(row_of(&buffer, 0).trim_end(), "[━━━●]");
    assert_eq!(fg(&buffer, 4), Some(Color::GREEN));
}

#[test]
fn test_switch_render_off() {
    let s = Switch::new().on(false);
    let buffer = screen(&s, 20, 1);
    assert_eq!(row_of(&buffer, 0).trim_end(), "[●━━━]");
    // The off knob in the default off color
    assert_eq!(fg(&buffer, 1), Some(GRAY));
}

#[test]
fn test_switch_render_all_styles() {
    for (style, off, on) in [
        (SwitchStyle::Default, "[●━━━]", "[━━━●]"),
        (SwitchStyle::IOS, "(●   )", "(   ●)"),
        (SwitchStyle::Material, "●━━━━○", "○━━━━●"),
        (SwitchStyle::Text, "[OFF]", "[ON]"),
        (SwitchStyle::Emoji, "❌", "✅"),
        (SwitchStyle::Block, "▓▓▓░░░", "░░░▓▓▓"),
    ] {
        let s = Switch::new().style(style);
        assert_eq!(row(&s, 20).trim_end(), off, "{style:?}");
        assert_eq!(row(&s.on(true), 20).trim_end(), on, "{style:?}");
    }
}

#[test]
fn test_switch_render_with_label_left() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().label("Dark Mode");
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'D');
}

#[test]
fn test_switch_render_with_label_right() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().label_right().label("Dark Mode");
    s.render(&mut ctx);

    // Label should be on the right
    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Dark Mode"));
}

#[test]
fn test_switch_render_focused() {
    let s = Switch::new().focused(true);
    let buffer = screen(&s, 20, 1);
    // Both brackets turn cyan, and a focus bracket follows
    for x in [0, 5, 6] {
        assert_eq!(fg(&buffer, x), Some(Color::CYAN), "{x}");
    }
    // With a label on the left the focus brackets go around the switch's
    // own, which turn cyan too
    let buffer = screen(&s.label("ab"), 20, 1);
    assert_eq!(row_of(&buffer, 0).trim_end(), "ab[[●━━━]]");
    for x in [2, 3, 8, 9] {
        assert_eq!(fg(&buffer, x), Some(Color::CYAN), "{x}");
    }
}

#[test]
fn test_switch_render_disabled() {
    let buffer = screen(&Switch::new().disabled(true).label("Disabled"), 20, 1);
    assert_eq!(fg(&buffer, 0), Some(GRAY));
    let enabled = screen(&Switch::new().label("Disabled"), 20, 1);
    assert_eq!(fg(&enabled, 0), Some(Color::WHITE));
}

#[test]
fn test_switch_render_zero_area() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new();
    s.render(&mut ctx);
    assert!((0..20).all(|x| buffer.get(x, 0).unwrap().symbol == ' '));
}

#[test]
fn test_switch_render_custom_width() {
    // A 12-wide switch in 8 columns is clipped
    let s = Switch::new().width(12);
    assert_eq!(row(&s, 8), "[●━━━━━━");
    assert_eq!(row(&s, 30).trim_end(), "[●━━━━━━━━━]");
}

#[test]
fn test_switch_render_text_style_custom() {
    let s = Switch::new().style(SwitchStyle::Text).text("YES", "NO");
    let buffer = screen(&s, 30, 1);
    assert_eq!(row_of(&buffer, 0).trim_end(), "[NO]");
    // The text in the off color, the brackets white
    assert_eq!(fg(&buffer, 1), Some(GRAY));
    assert_eq!(fg(&buffer, 0), Some(Color::WHITE));
}

#[test]
fn test_switch_render_text_style_default_text() {
    let s = Switch::new().style(SwitchStyle::Text);
    assert_eq!(row(&s, 30).trim_end(), "[OFF]");
    let buffer = screen(&s.on(true), 30, 1);
    assert_eq!(row_of(&buffer, 0).trim_end(), "[ON]");
    // The on text is bold
    assert!(buffer.get(1, 0).unwrap().modifier.contains(Modifier::BOLD));
}

// ==================== Edge Cases ====================

#[test]
fn test_switch_empty_label() {
    // An empty label still leaves its one-column gap
    let s = Switch::new().label("");
    assert_eq!(row(&s, 8), " [●━━━] ");
}

#[test]
fn test_switch_unicode_label() {
    // Six wide glyphs: the switch starts after 12 columns and a space
    let s = Switch::new().label("ダークモード");
    let buffer = screen(&s, 30, 1);
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '[');
    assert!(row_of(&buffer, 0).starts_with("ダークモード [●━━━]"));
}

#[test]
fn test_switch_very_long_label() {
    let long_label = "This is a very long label that exceeds normal width";
    let s = Switch::new().label(long_label);
    // Clipped to the 40 columns, with no room left for the switch
    assert_eq!(row(&s, 40), long_label[..40]);
}

#[test]
fn test_switch_multiple_toggles() {
    let mut s = Switch::new();

    for _ in 0..10 {
        s.toggle();
    }

    // After 10 toggles (even number), should be off
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());
}

#[test]
fn test_switch_set_while_disabled() {
    let mut s = Switch::new().disabled(true);

    s.set(true);
    assert!(!s.is_on());

    s.set(false);
    assert!(!s.is_on());
}

#[test]
fn test_switch_toggle_on_off() {
    let mut s = Switch::new().on(true);

    s.toggle();
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());
}

#[test]
fn test_switch_state_query_methods() {
    let s_on = Switch::new().on(true);
    assert!(s_on.is_on());
    assert!(s_on.is_checked());

    let s_off = Switch::new().on(false);
    assert!(!s_off.is_on());
    assert!(!s_off.is_checked());
}

// ==================== CSS Integration Tests ====================

#[test]
fn test_switch_element_id() {
    let s = Switch::new().element_id("dark-mode-toggle");
    assert_eq!(View::id(&s), Some("dark-mode-toggle"));
}

#[test]
fn test_switch_classes() {
    let s = Switch::new().class("toggle").class("interactive");
    assert!(s.has_class("toggle"));
    assert!(s.has_class("interactive"));
    assert!(!s.has_class("hidden"));
}

#[test]
fn test_switch_styled_view_methods() {
    let mut s = Switch::new();

    s.set_id("my-switch");
    assert_eq!(View::id(&s), Some("my-switch"));

    s.add_class("active");
    assert!(s.has_class("active"));

    s.remove_class("active");
    assert!(!s.has_class("active"));

    s.toggle_class("visible");
    assert!(s.has_class("visible"));

    s.toggle_class("visible");
    assert!(!s.has_class("visible"));
}

#[test]
fn test_switch_meta() {
    let s = Switch::new()
        .element_id("test")
        .class("class1")
        .class("class2");

    let meta = s.meta();
    assert_eq!(meta.id, Some("test".to_string()));
    assert_eq!(meta.classes.len(), 2);
}

// ==================== Color Tests ====================

#[test]
fn test_switch_render_with_custom_colors() {
    let s = Switch::new()
        .style(SwitchStyle::Material)
        .on(true)
        .on_color(Color::GREEN)
        .off_color(Color::RED)
        .track_color(Color::BLUE);
    let buffer = screen(&s, 30, 1);
    // On: the empty end in the track color, the line and the knob in the on color
    assert_eq!(fg(&buffer, 0), Some(Color::BLUE));
    assert_eq!(fg(&buffer, 2), Some(Color::GREEN));
    assert_eq!(fg(&buffer, 5), Some(Color::GREEN));
}

#[test]
fn test_switch_render_same_on_off_colors() {
    // With one color for both, only the knob's place tells them apart
    let s = Switch::new().on_color(Color::CYAN).off_color(Color::CYAN);
    let off = screen(&s, 6, 1);
    let on = screen(&s.on(true), 6, 1);
    assert_eq!(
        (fg(&off, 1), fg(&on, 4)),
        (Some(Color::CYAN), Some(Color::CYAN))
    );
    assert_ne!(row_of(&off, 0), row_of(&on, 0));
}

// ==================== Width Edge Cases ====================

#[test]
fn test_switch_very_small_width() {
    // Width 1 is raised to the minimum, 4
    assert_eq!(
        row(&Switch::new().width(1), 8),
        row(&Switch::new().width(4), 8)
    );
}

#[test]
fn test_switch_very_large_width() {
    // Measured as wide as it is, up to the room given; drawn clipped
    let s = Switch::new().width(1000);
    assert_eq!(s.measure(50, 1), Some((50, 1)));
    assert_eq!(row(&s, 20), format!("[●{}", "━".repeat(18)));
}

#[test]
fn test_switch_zero_width() {
    assert_eq!(Switch::new().width(0).measure(100, 1), Some((4, 1)));
}

// ==================== Label Edge Cases ====================

#[test]
fn test_switch_label_with_special_chars() {
    let s = Switch::new().label("Toggle & Switch <Test>");
    assert!(row(&s, 40).starts_with("Toggle & Switch <Test> [●━━━]"));
}

#[test]
fn test_switch_label_with_tabs() {
    // A tab has no width: it is dropped, and the switch follows the rest
    let s = Switch::new().label("Toggle\tTest");
    assert!(row(&s, 30).starts_with("ToggleTest [●━━━]"));
    assert_eq!(s.measure(100, 1), Some((17, 1)));
}

#[test]
fn test_switch_label_with_newlines() {
    // Likewise a newline: the label stays on one row
    let s = Switch::new().label("Line1\nLine2");
    assert!(row(&s, 30).starts_with("Line1Line2 [●━━━]"));
}

// ==================== Style-Specific Tests ====================

#[test]
fn test_switch_render_emoji_style_on() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(true).style(SwitchStyle::Emoji);
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    // Emoji style should render with emoji characters
    assert!(text.contains('✓') || text.contains('✗') || text.contains('✅') || text.contains('❌'));
}

#[test]
fn test_switch_render_emoji_style_off() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(false).style(SwitchStyle::Emoji);
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    // Should show off state emoji
    assert!(text.contains('✓') || text.contains('✗') || text.contains('✅') || text.contains('❌'));
}

#[test]
fn test_switch_render_block_style_on() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(true).style(SwitchStyle::Block);
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    // Block style should use block characters
    assert!(text.contains('█') || text.contains('▓') || text.contains('▒'));
}

#[test]
fn test_switch_render_block_style_off() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().on(false).style(SwitchStyle::Block);
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    // Block style should show empty block
    assert!(text.contains('░') || text.contains(' ') || text.contains('○'));
}

#[test]
fn test_switch_render_material_style() {
    let buffer = screen(&Switch::new().style(SwitchStyle::Material), 20, 1);
    // Off: the knob on the left in the off color, the rest in the track color
    assert_eq!(fg(&buffer, 0), Some(GRAY));
    assert_ne!(fg(&buffer, 2), Some(GRAY));
    assert_eq!(fg(&buffer, 2), fg(&buffer, 5));
}

#[test]
fn test_switch_render_ios_style() {
    let buffer = screen(&Switch::new().style(SwitchStyle::IOS), 20, 1);
    // A white knob on the track's background, between the parentheses
    let knob = buffer.get(1, 0).unwrap();
    assert_eq!((knob.symbol, knob.fg), ('●', Some(Color::WHITE)));
    assert_eq!(buffer.get(2, 0).unwrap().bg, knob.bg);
    assert!(knob.bg.is_some());
}

// ==================== Text Style Tests ====================

#[test]
fn test_switch_render_text_style_on() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .on(true)
        .style(SwitchStyle::Text)
        .text("ON", "OFF");
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("ON"));
}

#[test]
fn test_switch_render_text_style_off() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .on(false)
        .style(SwitchStyle::Text)
        .text("ON", "OFF");
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("OFF"));
}

#[test]
fn test_switch_render_text_style_custom_text() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .on(true)
        .style(SwitchStyle::Text)
        .text("YES", "NO");
    s.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("YES"));
}

#[test]
fn test_switch_render_text_style_empty_text() {
    let s = Switch::new().style(SwitchStyle::Text).text("", "");
    assert_eq!(row(&s, 20).trim_end(), "[]");
    assert_eq!(s.measure(20, 1), Some((2, 1)));
}

#[test]
fn test_switch_render_text_style_unicode_text() {
    // A wide glyph: the closing bracket after its two columns
    let s = Switch::new()
        .on(true)
        .style(SwitchStyle::Text)
        .text("開", "關");
    let buffer = screen(&s, 20, 1);
    assert_eq!(row_of(&buffer, 0).trim_end(), "[開]");
    assert_eq!(buffer.get(3, 0).unwrap().symbol, ']');
}

// ==================== State Transition Tests ====================

#[test]
fn test_switch_multiple_state_changes() {
    let mut s = Switch::new();
    // Initial state is off
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());

    s.toggle();
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());

    s.set(false);
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());
}

#[test]
fn test_switch_set_same_state() {
    let mut s = Switch::new().on(true);

    s.set(true);
    assert!(s.is_on());

    s.set(false);
    assert!(!s.is_on());
}

#[test]
fn test_switch_toggle_chain() {
    let mut s = Switch::new();
    for _ in 0..100 {
        s.toggle();
    }
    // After 100 toggles (even number), should be off
    assert!(!s.is_on());
}

#[test]
fn test_switch_focus_with_toggle() {
    let mut s = Switch::new().focused(true);

    assert!(s.handle_key(&Key::Enter));
    assert!(s.is_on());
    assert!(s.handle_key(&Key::Enter));
    assert!(!s.is_on());
}

// ==================== Disabled State Tests ====================

#[test]
fn test_switch_disabled_with_focus() {
    let mut s = Switch::new().focused(true).disabled(true);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.is_on());

    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.is_on());
}

#[test]
fn test_switch_toggle_while_disabled() {
    let mut s = Switch::new().on(true).disabled(true);

    s.toggle();
    assert!(s.is_on()); // Should not change

    s.toggle();
    assert!(s.is_on()); // Still should not change
}

#[test]
fn test_switch_render_disabled_with_colors() {
    // The label grays out whatever the colors (the switch itself does not
    // yet, #1018)
    let s = Switch::new()
        .disabled(true)
        .label("L")
        .on_color(Color::GREEN)
        .off_color(Color::RED);
    assert_eq!(fg(&screen(&s, 30, 1), 0), Some(GRAY));
}

// ==================== Layout Tests ====================

#[test]
fn test_switch_render_narrow_buffer() {
    assert_eq!(row(&Switch::new(), 10), "[●━━━]    ");
    // Narrower than the switch: clipped
    assert_eq!(row(&Switch::new(), 3), "[●━");
}

#[test]
fn test_switch_render_wide_buffer() {
    let buffer = screen(&Switch::new().label("Wide Label"), 100, 1);
    assert!(row_of(&buffer, 0).starts_with("Wide Label [●━━━]"));
    assert!((17..100).all(|x| buffer.get(x, 0).unwrap().symbol == ' '));
}

#[test]
fn test_switch_render_offset_area() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(10, 2, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new().label("Offset");
    s.render(&mut ctx);
    // Drawn from (10, 2), and nowhere else
    assert!(row_of(&buffer, 2)[10..].starts_with("Offset [●━━━]"));
    for y in [0, 1, 3, 4] {
        assert_eq!(row_of(&buffer, y).trim(), "", "{y}");
    }
}

// ==================== Builder Pattern Tests ====================

#[test]
fn test_switch_builder_preserves_state() {
    let s = Switch::new()
        .on(true)
        .focused(true)
        .disabled(false)
        .label("Test");

    assert!(s.is_on());
}

#[test]
fn test_switch_builder_override() {
    let s = Switch::new()
        .on(false)
        .on(true)
        .label("First")
        .label("Second");

    assert!(s.is_on());
}

#[test]
fn test_switch_multiple_builders() {
    let s1 = Switch::new().on(true).label("Switch 1");
    let s2 = Switch::new().on(false).label("Switch 2");

    assert!(s1.is_on());
    assert!(!s2.is_on());
}

// ==================== Helper Function Tests ====================

#[test]
fn test_toggle_helper_with_state() {
    let s_on = toggle("Feature").on(true);
    let s_off = toggle("Feature").on(false);

    assert!(s_on.is_on());
    assert!(!s_off.is_on());
}

#[test]
fn test_toggle_helper_with_label() {
    let s = toggle("Enable Notifications");

    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..40)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Enable Notifications"));
}

#[test]
fn test_switch_helper_variants() {
    let s1 = switch().on(true);
    let s2 = switch().on(false).label("Toggle");

    assert!(s1.is_on());
    assert!(!s2.is_on());
}

// ==================== Clone and Copy Tests ====================

#[test]
fn test_switch_clone_not_implemented() {
    // Switch doesn't implement Clone - verify it compiles without clone
    let s1 = Switch::new().on(true).focused(true).disabled(false);
    let s2 = Switch::new().on(true).focused(true).disabled(false);

    assert_eq!(s1.is_on(), s2.is_on());
}

// ==================== Interaction Edge Cases ====================

#[test]
fn test_switch_rapid_toggle() {
    let mut s = Switch::new();

    for _ in 0..1000 {
        s.toggle();
    }

    assert!(!s.is_on()); // Even number of toggles
}

#[test]
fn test_switch_set_after_toggle() {
    let mut s = Switch::new();

    s.toggle();
    assert!(s.is_on());

    s.set(false);
    assert!(!s.is_on());

    s.toggle();
    assert!(s.is_on());
}

#[test]
fn test_switch_all_keys_unfocused() {
    let mut s = Switch::new().focused(false);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.handle_key(&Key::Tab));
    assert!(!s.handle_key(&Key::Up));
    assert!(!s.handle_key(&Key::Down));
    assert!(!s.is_on());
}

#[test]
fn test_switch_all_keys_disabled() {
    let mut s = Switch::new().focused(true).disabled(true);

    assert!(!s.handle_key(&Key::Enter));
    assert!(!s.handle_key(&Key::Char(' ')));
    assert!(!s.handle_key(&Key::Tab));
    assert!(!s.handle_key(&Key::Up));
    assert!(!s.handle_key(&Key::Down));
    assert!(!s.is_on());
}

// ==================== Rendering Validation ====================

#[test]
fn test_switch_render_output_exists() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new();
    s.render(&mut ctx);

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
fn test_switch_render_off_then_on() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    // Render off state
    let s_off = Switch::new().on(false);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s_off.render(&mut ctx);

    buffer.clear();

    // Render on state
    let s_on = Switch::new().on(true);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s_on.render(&mut ctx);

    // Both should render successfully
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
fn test_switch_render_with_label_and_style() {
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new()
        .label("Dark Mode")
        .style(SwitchStyle::IOS)
        .on(true);

    s.render(&mut ctx);

    let text: String = (0..40)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Dark Mode"));
}

#[test]
fn test_switch_render_very_narrow() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Switch::new();
    s.render(&mut ctx);
    assert_eq!(row_of(&buffer, 0), "[●━━━");
}

#[test]
fn test_switch_render_all_styles_with_colors() {
    // On, every style uses the on color for its knob or text
    for (style, x) in [
        (SwitchStyle::Default, 4),
        (SwitchStyle::Material, 5),
        (SwitchStyle::Text, 1),
        (SwitchStyle::Emoji, 0),
        (SwitchStyle::Block, 5),
    ] {
        let s = Switch::new()
            .style(style)
            .on(true)
            .on_color(Color::GREEN)
            .off_color(Color::RED)
            .track_color(Color::BLUE);
        assert_eq!(fg(&screen(&s, 30, 1), x), Some(Color::GREEN), "{style:?}");
    }
    // IOS fills its track with it
    let s = Switch::new()
        .style(SwitchStyle::IOS)
        .on(true)
        .on_color(Color::GREEN);
    assert_eq!(screen(&s, 30, 1).get(2, 0).unwrap().bg, Some(Color::GREEN));
}

/// Test Switch edge cases
mod edge_cases {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{Switch, ToggleWidget};
    #[test]
    fn test_switch_no_label() {
        let switch = Switch::new();
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
        // One row only
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '[');
        assert!((1..10).all(|y| (0..10).all(|x| buffer.get(x, y).unwrap().symbol == ' ')));
    }

    #[test]
    fn test_switch_with_empty_label() {
        let switch = Switch::new().label("");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
        // Measured with the label's gap, as drawn
        assert_eq!(switch.measure(10, 10), Some((7, 1)));
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '[');
    }

    #[test]
    fn test_switch_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let switch = Switch::new().label(&long_label);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
        assert!((0..10).all(|x| buffer.get(x, 0).unwrap().symbol == 'A'));
        assert_eq!(switch.measure(10, 10), Some((10, 1)));
    }

    #[test]
    fn test_switch_toggle_multiple_times() {
        let mut switch = Switch::new();

        // Toggle many times
        for i in 0..100 {
            switch = switch.on(i % 2 == 1);
        }

        // 99 is odd, so on(true)
        assert!(switch.is_on());
    }

    #[test]
    fn test_switch_with_unicode() {
        let switch = Switch::new().label("전환 ⚡");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
        // "전환 ⚡" is 7 columns wide: the switch starts at 8
        assert_eq!(buffer.get(8, 0).unwrap().symbol, '[');
        assert_eq!(switch.measure(20, 10), Some((14, 1)));
    }

    #[test]
    fn test_switch_disabled_on() {
        let switch = Switch::new().on(true).disabled(true);
        assert!(switch.is_on());
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
    }

    #[test]
    fn test_switch_with_zero_width_buffer() {
        let switch = Switch::new();
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
        assert_eq!(switch.measure(0, 10), Some((0, 1)));
    }
}

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};
    use revue::widget::Switch;

    #[test]
    fn test_switch_basic() {
        let view = vstack()
            .gap(1)
            .child(hstack().child(Switch::new()).child(text(" Enable feature")))
            .child(
                hstack()
                    .child(Switch::new().on(true))
                    .child(text(" Dark mode")),
            );

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("switch_basic");
    }
}
