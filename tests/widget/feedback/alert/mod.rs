//! Alert widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::render::Modifier;
use revue::style::Color;
use revue::style::{Style, VisualStyle};
use revue::widget::alert;
use revue::widget::theme::{DISABLED_FG, LIGHT_GRAY, SECONDARY_TEXT};
use revue::widget::traits::RenderContext;
use revue::widget::Alert;
use revue::widget::AlertLevel;
use revue::widget::AlertVariant;
use revue::widget::StyledView;
use revue::widget::View;
use revue::widget::{error_alert, info_alert, success_alert, warning_alert};

mod view;

/// Render `alert` into a fresh `w` x `h` buffer.
fn draw(alert: &Alert, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    alert.render(&mut ctx);
    buffer
}

/// Render `alert` with a stylesheet that sets `color` and `background`
/// (`Color::default()` leaves one unset).
fn draw_css(alert: &Alert, w: u16, h: u16, color: Color, background: Color) -> Buffer {
    let style = Style {
        visual: VisualStyle {
            color,
            background,
            ..VisualStyle::default()
        },
        ..Default::default()
    };
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::with_style(&mut buffer, Rect::new(0, 0, w, h), &style);
    alert.render(&mut ctx);
    buffer
}

/// Row `y` as text, continuation cells left out.
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .filter(|&c| c != '\0')
        .collect()
}

/// Every cell, row by row.
fn cells(buffer: &Buffer) -> Vec<revue::render::Cell> {
    (0..buffer.height())
        .flat_map(|y| (0..buffer.width()).map(move |x| *buffer.get(x, y).unwrap()))
        .collect()
}

/// Whether nothing was drawn.
fn blank(buffer: &Buffer) -> bool {
    cells(buffer)
        .iter()
        .all(|c| c.symbol == ' ' && c.fg.is_none() && c.bg.is_none())
}

// ==================== Constructor Tests ====================

#[test]
fn test_alert_new() {
    // Filled, Info, with the icon: a rounded box around "ℹ message"
    let a = Alert::new("Test message");
    let b = draw(&a, 30, 3);
    assert_eq!(row(&b, 0), format!("╭{}╮", "─".repeat(28)));
    assert_eq!(row(&b, 1), format!("{:<29}│", "│ ℹ Test message"));
    assert_eq!(row(&b, 2), format!("╰{}╯", "─".repeat(28)));
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_default() {
    let _a = Alert::default();
    // Private field - just verify it compiles

    let a = Alert::default();
    assert!(!a.is_dismissed());
    assert_eq!(a.height(), 3); // "Alert" message with Filled variant, no title
}

#[test]
fn test_alert_helper() {
    assert_eq!(
        cells(&draw(&alert("Test message"), 30, 3)),
        cells(&draw(&Alert::new("Test message"), 30, 3))
    );
}

// ==================== Builder Tests ====================

#[test]
fn test_alert_title() {
    // The title takes the first row, bold and white; the message moves below
    let b = draw(&Alert::new("Message").title("Title"), 30, 4);
    assert_eq!(row(&b, 1), format!("{:<29}│", "│ ℹ Title"));
    assert_eq!(row(&b, 2), format!("{:<29}│", "│   Message"));
    let title = b.get(4, 1).unwrap();
    assert_eq!(title.fg, Some(Color::WHITE));
    assert!(title.modifier.contains(Modifier::BOLD));
    let message = b.get(4, 2).unwrap();
    assert_eq!(message.fg, Some(SECONDARY_TEXT));
    assert!(!message.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_alert_level() {
    let b = draw(&Alert::new("Message").level(AlertLevel::Warning), 30, 3);
    let icon = b.get(2, 1).unwrap();
    assert_eq!((icon.symbol, icon.fg), ('⚠', Some(Color::YELLOW)));
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::rgb(180, 120, 0)));
    assert_eq!(b.get(10, 1).unwrap().bg, Some(Color::rgb(50, 35, 0)));
}

#[test]
fn test_alert_variant() {
    // Minimal: no box, the icon and the message on one row
    let a = Alert::new("Message").variant(AlertVariant::Minimal);
    let b = draw(&a, 30, 3);
    assert_eq!(row(&b, 0).trim_end(), "ℹ Message");
    assert!(row(&b, 1).trim().is_empty());
    assert_eq!(a.height(), 1);
}

#[test]
fn test_alert_dismissible() {
    // The `×` sits two columns in from the right border, on the first row
    let mut a = Alert::new("Message").dismissible(true);
    let b = draw(&a, 30, 3);
    let x = b.get(27, 1).unwrap();
    assert_eq!((x.symbol, x.fg), ('×', Some(LIGHT_GRAY)));
    assert!(a.handle_key(&Key::Char('x')));
    assert!(blank(&draw(&a, 30, 3)));
}

#[test]
fn test_alert_not_dismissible() {
    let mut a = Alert::new("Message").dismissible(false);
    let b = draw(&a, 30, 3);
    assert!(!(0..3).any(|y| row(&b, y).contains('×')));
    assert!(!a.handle_key(&Key::Char('x')));
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_focused() {
    assert!(Alert::new("Test").focused(true).is_focused());
    assert!(!Alert::new("Test").is_focused());
}

#[test]
fn test_alert_not_focused() {
    assert!(!Alert::new("Test").focused(true).focused(false).is_focused());
}

#[test]
fn test_alert_disabled() {
    // A disabled outlined alert greys its message
    let a = Alert::new("Test")
        .variant(AlertVariant::Outlined)
        .disabled(true);
    assert!(a.is_disabled());
    let b = draw(&a, 20, 1);
    assert_eq!(b.get(4, 0).unwrap().symbol, 'T');
    assert_eq!(b.get(4, 0).unwrap().fg, Some(DISABLED_FG));
}

#[test]
fn test_alert_not_disabled() {
    let a = Alert::new("Test")
        .variant(AlertVariant::Outlined)
        .disabled(true)
        .disabled(false);
    assert!(!a.is_disabled());
    assert_eq!(draw(&a, 20, 1).get(4, 0).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_alert_icon() {
    let b = draw(&Alert::new("Message").icon(true), 30, 3);
    assert_eq!(row(&b, 1), format!("{:<29}│", "│ ℹ Message"));
    assert_eq!(b.get(2, 1).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_alert_no_icon() {
    // Without the icon the message starts where the icon was
    let b = draw(&Alert::new("Message").icon(false), 30, 3);
    assert_eq!(row(&b, 1), format!("{:<29}│", "│ Message"));
}

#[test]
fn test_alert_custom_icon() {
    let _a = Alert::new("Message").custom_icon('★');
    // Private field - just verify it compiles

    let a = Alert::new("Test").custom_icon('★');
    // The custom icon is used in rendering, but we can verify it doesn't break anything
    assert!(!a.is_dismissed());
    assert_eq!(a.height(), 3); // Filled variant with no title
}

#[test]
fn test_alert_fg() {
    // The outlined and minimal variants draw the message in `fg`
    // (the filled one keeps white: #799)
    for (variant, x) in [(AlertVariant::Outlined, 4), (AlertVariant::Minimal, 2)] {
        let b = draw(
            &Alert::new("Message").variant(variant).fg(Color::RED),
            20,
            1,
        );
        assert_eq!(b.get(x, 0).unwrap().symbol, 'M', "{variant:?}");
        assert_eq!(b.get(x, 0).unwrap().fg, Some(Color::RED), "{variant:?}");
    }
}

#[test]
fn test_alert_bg() {
    // The filled background is the level's, and a stylesheet background
    // replaces it. (`.bg()` itself changes nothing yet: #799.)
    let b = draw(&Alert::new("Message").level(AlertLevel::Error), 20, 3);
    assert!((0..20).all(|x| b.get(x, 1).unwrap().bg == Some(Color::rgb(50, 0, 0))));
    let css = draw_css(&Alert::new("Message"), 20, 3, Color::default(), Color::BLUE);
    assert!((0..20).all(|x| css.get(x, 1).unwrap().bg == Some(Color::BLUE)));
}

#[test]
fn test_alert_builder_chain() {
    let a = Alert::new("Message")
        .title("Title")
        .level(AlertLevel::Warning)
        .variant(AlertVariant::Filled)
        .dismissible(true)
        .focused(true)
        .disabled(false)
        .icon(true)
        .fg(Color::WHITE)
        .bg(Color::BLACK);
    let b = draw(&a, 30, 4);
    assert_eq!(row(&b, 1), format!("{:<27}× │", "│ ⚠ Title"));
    assert_eq!(row(&b, 2), format!("{:<29}│", "│   Message"));
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::rgb(180, 120, 0)));
    assert_eq!(a.height(), 4);
}

// ==================== Dismissal Tests ====================

#[test]
fn test_alert_dismiss() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(!a.is_dismissed());

    a.dismiss();
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_reset() {
    let mut a = Alert::new("Test").dismissible(true);
    a.dismiss();
    assert!(a.is_dismissed());

    a.reset();
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_dismiss_not_dismissible() {
    let mut a = Alert::new("Test").dismissible(false);
    assert!(!a.is_dismissed());

    // Can still manually dismiss
    a.dismiss();
    assert!(a.is_dismissed());
}

// ==================== Key Handling Tests ====================

#[test]
fn test_alert_handle_key_x() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_x_uppercase() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(a.handle_key(&Key::Char('X')));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_escape() {
    let mut a = Alert::new("Test").dismissible(true);
    assert!(a.handle_key(&Key::Escape));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_not_dismissible() {
    let mut a = Alert::new("Test").dismissible(false);
    assert!(!a.handle_key(&Key::Char('x')));
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_handle_key_already_dismissed() {
    let mut a = Alert::new("Test").dismissible(true);
    a.dismiss();

    assert!(!a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_handle_key_other_keys() {
    let mut a = Alert::new("Test").dismissible(true);

    assert!(!a.handle_key(&Key::Char('a')));
    assert!(!a.handle_key(&Key::Enter));
    assert!(!a.handle_key(&Key::Tab));
    assert!(!a.is_dismissed());
}

// ==================== AlertLevel Tests ====================

#[test]
fn test_alert_level_info() {
    let level = AlertLevel::Info;
    assert_eq!(level.color(), Color::CYAN);
    assert_eq!(level.icon(), 'ℹ');
}

#[test]
fn test_alert_level_success() {
    let level = AlertLevel::Success;
    assert_eq!(level.color(), Color::GREEN);
    assert_eq!(level.icon(), '✓');
}

#[test]
fn test_alert_level_warning() {
    let level = AlertLevel::Warning;
    assert_eq!(level.color(), Color::YELLOW);
    assert_eq!(level.icon(), '⚠');
}

#[test]
fn test_alert_level_error() {
    let level = AlertLevel::Error;
    assert_eq!(level.color(), Color::RED);
    assert_eq!(level.icon(), '✗');
}

#[test]
fn test_alert_level_default() {
    let level = AlertLevel::default();
    assert_eq!(level, AlertLevel::Info);
}

#[test]
fn test_alert_level_partial_eq() {
    assert_eq!(AlertLevel::Info, AlertLevel::Info);
    assert_eq!(AlertLevel::Success, AlertLevel::Success);
    assert_ne!(AlertLevel::Info, AlertLevel::Error);
}

#[test]
fn test_alert_level_colors() {
    assert_eq!(AlertLevel::Info.color(), Color::CYAN);
    assert_eq!(AlertLevel::Success.color(), Color::GREEN);
    assert_eq!(AlertLevel::Warning.color(), Color::YELLOW);
    assert_eq!(AlertLevel::Error.color(), Color::RED);

    // Test color access through public methods
    let info_color = AlertLevel::Info.color();
    let success_color = AlertLevel::Success.color();
    let warning_color = AlertLevel::Warning.color();
    let error_color = AlertLevel::Error.color();

    // Just verify they return colors (don't compare specific values unless needed)
    assert!(info_color != success_color);
    assert!(success_color != warning_color);
    assert!(warning_color != error_color);
}

#[test]
fn test_alert_level_icons() {
    assert_eq!(AlertLevel::Info.icon(), 'ℹ');
    assert_eq!(AlertLevel::Success.icon(), '✓');
    assert_eq!(AlertLevel::Warning.icon(), '⚠');
    assert_eq!(AlertLevel::Error.icon(), '✗');
}

#[test]
fn test_alert_level_bg_colors() {
    let info_bg = AlertLevel::Info.bg_color();
    let success_bg = AlertLevel::Success.bg_color();
    let warning_bg = AlertLevel::Warning.bg_color();
    let error_bg = AlertLevel::Error.bg_color();

    assert_ne!(info_bg, success_bg);
    assert_ne!(success_bg, warning_bg);
    assert_ne!(warning_bg, error_bg);
}

#[test]
fn test_alert_level_border_colors() {
    let info_border = AlertLevel::Info.border_color();
    let success_border = AlertLevel::Success.border_color();
    let warning_border = AlertLevel::Warning.border_color();
    let error_border = AlertLevel::Error.border_color();

    assert_ne!(info_border, success_border);
    assert_ne!(success_border, warning_border);
    assert_ne!(warning_border, error_border);
}

// ==================== AlertVariant Tests ====================

#[test]
fn test_alert_variant_default() {
    let variant = AlertVariant::default();
    assert_eq!(variant, AlertVariant::Filled);
}

#[test]
fn test_alert_variant_partial_eq() {
    assert_eq!(AlertVariant::Filled, AlertVariant::Filled);
    assert_eq!(AlertVariant::Outlined, AlertVariant::Outlined);
    assert_eq!(AlertVariant::Minimal, AlertVariant::Minimal);
    assert_ne!(AlertVariant::Filled, AlertVariant::Outlined);
}

#[test]
fn test_alert_variant_all_variants() {
    // Each variant's frame: a box, a bar down the left edge, or none
    let frame = |v| draw(&Alert::new("m").icon(false).variant(v), 10, 3);
    let filled = frame(AlertVariant::Filled);
    assert_eq!(
        (0..3)
            .map(|y| filled.get(0, y).unwrap().symbol)
            .collect::<String>(),
        "╭│╰"
    );
    let outlined = frame(AlertVariant::Outlined);
    assert_eq!(
        (0..3)
            .map(|y| outlined.get(0, y).unwrap().symbol)
            .collect::<String>(),
        "┃┃┃"
    );
    let minimal = frame(AlertVariant::Minimal);
    assert_eq!(row(&minimal, 0).trim_end(), "m");
    assert!(row(&minimal, 1).trim().is_empty() && row(&minimal, 2).trim().is_empty());
}

// ==================== Helper Functions Tests ====================

#[test]
fn test_alert_helper_function() {
    // The free helpers build the same alerts as the constructors
    let pairs = [
        (info_alert("m"), Alert::info("m")),
        (success_alert("m"), Alert::success("m")),
        (warning_alert("m"), Alert::warning("m")),
        (error_alert("m"), Alert::error("m")),
    ];
    for (helper, ctor) in pairs {
        assert_eq!(cells(&draw(&helper, 20, 3)), cells(&draw(&ctor, 20, 3)));
    }
}

#[test]
fn test_alert_info_helper() {
    let b = draw(&Alert::info("Info message"), 30, 3);
    let icon = b.get(2, 1).unwrap();
    assert_eq!((icon.symbol, icon.fg), ('ℹ', Some(Color::CYAN)));
    assert_eq!(icon.bg, Some(Color::rgb(0, 30, 50)));
    assert!(row(&b, 1).contains("Info message"));
}

#[test]
fn test_alert_success_helper() {
    let b = draw(&Alert::success("Success message"), 30, 3);
    let icon = b.get(2, 1).unwrap();
    assert_eq!((icon.symbol, icon.fg), ('✓', Some(Color::GREEN)));
    assert_eq!(icon.bg, Some(Color::rgb(0, 35, 0)));
    assert!(row(&b, 1).contains("Success message"));
}

#[test]
fn test_alert_warning_helper() {
    let b = draw(&Alert::warning("Warning message"), 30, 3);
    let icon = b.get(2, 1).unwrap();
    assert_eq!((icon.symbol, icon.fg), ('⚠', Some(Color::YELLOW)));
    assert_eq!(icon.bg, Some(Color::rgb(50, 35, 0)));
    assert!(row(&b, 1).contains("Warning message"));
}

#[test]
fn test_alert_error_helper() {
    let b = draw(&Alert::error("Error message"), 30, 3);
    let icon = b.get(2, 1).unwrap();
    assert_eq!((icon.symbol, icon.fg), ('✗', Some(Color::RED)));
    assert_eq!(icon.bg, Some(Color::rgb(50, 0, 0)));
    assert!(row(&b, 1).contains("Error message"));
}

// ==================== Height Tests ====================

#[test]
fn test_alert_height() {
    let minimal = Alert::new("msg").variant(AlertVariant::Minimal);
    assert_eq!(minimal.height(), 1);

    let minimal_title = Alert::new("msg")
        .title("Title")
        .variant(AlertVariant::Minimal);
    assert_eq!(minimal_title.height(), 2);

    let filled = Alert::new("msg").variant(AlertVariant::Filled);
    assert_eq!(filled.height(), 3);

    let filled_title = Alert::new("msg")
        .title("Title")
        .variant(AlertVariant::Filled);
    assert_eq!(filled_title.height(), 4);

    let mut dismissed = Alert::new("msg").dismissible(true);
    dismissed.dismiss();
    assert_eq!(dismissed.height(), 0);
}

#[test]
fn test_alert_height_outlined() {
    // Outlined is only the left accent bar - no rows above or below the text.
    let outlined = Alert::new("msg").variant(AlertVariant::Outlined);
    assert_eq!(outlined.height(), 1);

    let outlined_title = Alert::new("msg")
        .title("Title")
        .variant(AlertVariant::Outlined);
    assert_eq!(outlined_title.height(), 2);
}

#[test]
fn test_alert_height_dismissed() {
    let mut dismissed = Alert::new("msg").dismissible(true);
    dismissed.dismiss();
    assert_eq!(dismissed.height(), 0);
}

// ==================== Rendering Tests ====================

#[test]
fn test_alert_render_filled() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test message").variant(AlertVariant::Filled);
    a.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(39, 0).unwrap().symbol, '╮');
}

#[test]
fn test_alert_render_outlined() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test").variant(AlertVariant::Outlined);
    a.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '┃');
}

#[test]
fn test_alert_render_minimal() {
    let mut buffer = Buffer::new(40, 2);
    let area = Rect::new(0, 0, 40, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Alert::new("Test").variant(AlertVariant::Minimal);
    a.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
}

#[test]
fn test_alert_render_dismissed() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut a = Alert::new("Test");
    a.dismiss();
    a.render(&mut ctx);

    // Should not render anything
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_alert_render_with_title() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .title("Alert Title")
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    let title_cell = buffer.get(4, 1).unwrap();
    assert_eq!(title_cell.symbol, 'A');
    assert!(title_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_alert_render_without_icon() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .icon(false)
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_ne!(buffer.get(2, 1).unwrap().symbol, 'ℹ');
}

#[test]
fn test_alert_render_with_custom_icon() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test message")
        .custom_icon('★')
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(2, 1).unwrap().symbol, '★');
}

#[test]
fn test_alert_render_too_small_area() {
    // Under five columns nothing is drawn; at five the box starts
    for w in 0..5 {
        assert!(blank(&draw(&Alert::new("Test message"), w, 3)), "{w}");
    }
    assert_eq!(row(&draw(&Alert::new("Test message"), 5, 3), 0), "╭───╮");
}

#[test]
fn test_alert_render_message_truncation() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let long_message = "This is a very long message that should be truncated";
    let alert = Alert::new(long_message).variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(2, 1).unwrap().symbol, 'ℹ');
}

#[test]
fn test_alert_render_dismiss_button_filled() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(true)
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(37, 1).unwrap().symbol, '×');
}

#[test]
fn test_alert_render_no_dismiss_button() {
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(false)
        .variant(AlertVariant::Filled);
    alert.render(&mut ctx);

    assert_ne!(buffer.get(37, 1).unwrap().symbol, '×');
}

#[test]
fn test_alert_render_outlined_dismiss_button() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(true)
        .variant(AlertVariant::Outlined);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    assert_eq!(buffer.get(38, 0).unwrap().symbol, '×');
}

#[test]
fn test_alert_render_minimal_dismiss_button() {
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let alert = Alert::new("Test")
        .dismissible(true)
        .variant(AlertVariant::Minimal);
    alert.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'ℹ');
    assert_eq!(buffer.get(39, 0).unwrap().symbol, '×');
}

// ==================== CSS Integration Tests ====================

#[test]
fn test_alert_builder_id() {
    let alert = Alert::new("Test message").element_id("test-alert");
    assert_eq!(View::id(&alert), Some("test-alert"));
}

#[test]
fn test_alert_builder_classes() {
    let alert = Alert::new("Test").class("important").class("notification");
    assert!(alert.has_class("important"));
    assert!(alert.has_class("notification"));
    assert!(!alert.has_class("secondary"));
}

#[test]
fn test_alert_builder_styled_methods() {
    let mut alert = Alert::new("Test");

    alert.set_id("my-alert");
    assert_eq!(View::id(&alert), Some("my-alert"));

    alert.add_class("primary");
    assert!(alert.has_class("primary"));

    alert.remove_class("primary");
    assert!(!alert.has_class("primary"));

    alert.toggle_class("active");
    assert!(alert.has_class("active"));

    alert.toggle_class("active");
    assert!(!alert.has_class("active"));
}

#[test]
fn test_alert_color_builders() {
    // `fg` outranks the stylesheet's color
    let a = Alert::new("Test").variant(AlertVariant::Outlined);
    let css = draw_css(&a, 20, 1, Color::MAGENTA, Color::default());
    assert_eq!(css.get(4, 0).unwrap().fg, Some(Color::MAGENTA));
    let both = draw_css(&a.fg(Color::RED), 20, 1, Color::MAGENTA, Color::default());
    assert_eq!(both.get(4, 0).unwrap().fg, Some(Color::RED));
}

// ==================== Edge Cases ====================

#[test]
fn test_alert_empty_message() {
    let b = draw(&Alert::new(""), 20, 3);
    assert_eq!(row(&b, 1), format!("{:<19}│", "│ ℹ"));
    let b = draw(&Alert::new("").variant(AlertVariant::Minimal), 20, 1);
    assert_eq!(row(&b, 0).trim_end(), "ℹ");
}

#[test]
fn test_alert_unicode_message() {
    // The filled alert draws wide glyphs with their continuation cells
    let b = draw(&Alert::new("Hello 世界 🌍"), 40, 3);
    let text = row(&b, 1);
    assert!(
        text.starts_with("│ ℹ Hello 世界 🌍 ") && text.ends_with(" │"),
        "{text}"
    );
    assert_eq!(b.get(10, 1).unwrap().symbol, '世');
    assert!(b.get(11, 1).unwrap().is_continuation());
    assert_eq!(b.get(12, 1).unwrap().symbol, '界');
    assert!(b.get(13, 1).unwrap().is_continuation());
}

#[test]
fn test_alert_special_characters_message() {
    let b = draw(&Alert::new("!@#$%^&*()"), 40, 3);
    assert_eq!(row(&b, 1), format!("{:<39}│", "│ ℹ !@#$%^&*()"));
}

#[test]
fn test_alert_very_long_message() {
    // The message is cut at the right padding
    let long_msg = "This is an extremely long alert message that contains a lot of text and would normally wrap or truncate in most UI scenarios but the widget should handle it gracefully without panicking or crashing.";
    let b = draw(&Alert::new(long_msg).icon(false), 20, 3);
    assert_eq!(row(&b, 1), "│ This is an extre │");
}

#[test]
fn test_alert_zero_area() {
    for (w, h) in [(0, 0), (0, 5), (10, 0)] {
        let mut buffer = Buffer::new(10, 5);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        Alert::new("Test").render(&mut ctx);
        assert!(blank(&buffer), "{w}x{h}");
    }
}

#[test]
fn test_alert_newline_in_message() {
    // The first line is drawn where the message starts. (What follows it
    // is drawn on the same row, run together: see the alert issue from the
    // #930 audit.)
    let b = draw(&Alert::new("Line 1\nLine 2\nLine 3"), 40, 3);
    assert!(row(&b, 1).starts_with("│ ℹ Line 1"), "{}", row(&b, 1));
    assert_eq!(Alert::new("Line 1\nLine 2\nLine 3").height(), 3);
}

#[test]
fn test_alert_render_all_levels() {
    // icon, accent, background, border
    let cases = [
        (
            AlertLevel::Info,
            'ℹ',
            Color::CYAN,
            Color::rgb(0, 30, 50),
            Color::rgb(0, 100, 150),
        ),
        (
            AlertLevel::Success,
            '✓',
            Color::GREEN,
            Color::rgb(0, 35, 0),
            Color::rgb(0, 120, 0),
        ),
        (
            AlertLevel::Warning,
            '⚠',
            Color::YELLOW,
            Color::rgb(50, 35, 0),
            Color::rgb(180, 120, 0),
        ),
        (
            AlertLevel::Error,
            '✗',
            Color::RED,
            Color::rgb(50, 0, 0),
            Color::rgb(150, 0, 0),
        ),
    ];
    for (level, icon, accent, bg, border) in cases {
        let b = draw(&Alert::new("Test").level(level), 40, 3);
        let i = b.get(2, 1).unwrap();
        assert_eq!(
            (i.symbol, i.fg, i.bg),
            (icon, Some(accent), Some(bg)),
            "{level:?}"
        );
        let corner = b.get(0, 0).unwrap();
        assert_eq!(
            (corner.fg, corner.bg),
            (Some(border), Some(bg)),
            "{level:?}"
        );
    }
}

#[test]
fn test_alert_render_all_variants() {
    // Only the filled variant paints a background, over its whole area
    let bg = Color::rgb(0, 30, 50);
    for (variant, painted) in [
        (AlertVariant::Filled, true),
        (AlertVariant::Outlined, false),
        (AlertVariant::Minimal, false),
    ] {
        let b = draw(&Alert::new("Test").variant(variant), 40, 5);
        let all = (0..5).all(|y| (0..40).all(|x| b.get(x, y).unwrap().bg == Some(bg)));
        let none = (0..5).all(|y| (0..40).all(|x| b.get(x, y).unwrap().bg.is_none()));
        assert_eq!((all, none), (painted, !painted), "{variant:?}");
    }
}

#[test]
fn test_alert_focused_rendering() {
    // Focus has no look of its own without a stylesheet: a `:focus` rule
    // reaches the alert through the style it is drawn with
    for variant in [
        AlertVariant::Filled,
        AlertVariant::Outlined,
        AlertVariant::Minimal,
    ] {
        let plain = Alert::new("Test").variant(variant);
        let focused = plain.clone().focused(true);
        assert_eq!(cells(&draw(&focused, 40, 3)), cells(&draw(&plain, 40, 3)));
    }
}

#[test]
fn test_alert_disabled_rendering() {
    // The outlined and minimal variants grey the message; the icon keeps its
    // accent
    for (variant, x) in [(AlertVariant::Outlined, 4), (AlertVariant::Minimal, 2)] {
        let b = draw(&Alert::new("Test").variant(variant).disabled(true), 40, 1);
        assert_eq!(b.get(x, 0).unwrap().symbol, 'T', "{variant:?}");
        assert_eq!(b.get(x, 0).unwrap().fg, Some(DISABLED_FG), "{variant:?}");
        assert_eq!(
            b.get(x - 2, 0).unwrap().fg,
            Some(Color::CYAN),
            "{variant:?}"
        );
    }
}

#[test]
fn test_alert_builder_pattern() {
    // Test that we can create and configure an alert using only public methods
    let a = Alert::new("Test message")
        .title("My Title")
        .level(AlertLevel::Error)
        .variant(AlertVariant::Outlined)
        .dismissible(true)
        .icon(false);

    // Test public methods
    assert!(!a.is_dismissed());
    assert_eq!(a.height(), 2); // Title + message; outlined has no top/bottom border
}

#[test]
fn test_alert_level_builder_methods() {
    // Test the builder methods for AlertLevel
    let info = Alert::info("Info message");
    assert!(!info.is_dismissed());

    let success = Alert::success("Success message");
    assert!(!success.is_dismissed());

    let warning = Alert::warning("Warning message");
    assert!(!warning.is_dismissed());

    let error = Alert::error("Error message");
    assert!(!error.is_dismissed());
}

#[test]
fn test_alert_state_methods() {
    let mut a = Alert::new("Test");

    // Test initial state
    assert!(!a.is_dismissed());

    // Test dismiss
    a.dismiss();
    assert!(a.is_dismissed());

    // Test reset
    a.reset();
    assert!(!a.is_dismissed());
}

#[test]
fn test_alert_height_calculations() {
    // Test height calculations using only public methods
    let a1 = Alert::new("Message").variant(AlertVariant::Filled);
    assert_eq!(a1.height(), 3);

    let a2 = Alert::new("Message")
        .title("Title")
        .variant(AlertVariant::Filled);
    assert_eq!(a2.height(), 4);

    let a3 = Alert::new("Message").variant(AlertVariant::Minimal);
    assert_eq!(a3.height(), 1);

    let a4 = Alert::new("Message")
        .title("Title")
        .variant(AlertVariant::Minimal);
    assert_eq!(a4.height(), 2);
}

#[test]
fn test_alert_key_handling() {
    let mut a = Alert::new("Test").dismissible(true);

    // Test that non-dismiss keys don't work
    assert!(!a.handle_key(&Key::Char('a')));
    assert!(!a.is_dismissed());

    // Test dismiss keys
    assert!(a.handle_key(&Key::Char('x')));
    assert!(a.is_dismissed());

    // Reset for next test
    a.reset();
    assert!(!a.is_dismissed());

    assert!(a.handle_key(&Key::Char('X')));
    assert!(a.is_dismissed());

    // Reset for next test
    a.reset();
    assert!(!a.is_dismissed());

    assert!(a.handle_key(&Key::Escape));
    assert!(a.is_dismissed());
}

#[test]
fn test_alert_helper_functions() {
    // Test the public helper functions
    let a = alert("Helper message");
    assert!(!a.is_dismissed());
    assert_eq!(a.height(), 3); // Default is Filled variant with no title

    let i = Alert::info("Info");
    assert!(!i.is_dismissed());

    let s = Alert::success("Success");
    assert!(!s.is_dismissed());

    let w = Alert::warning("Warning");
    assert!(!w.is_dismissed());

    let e = Alert::error("Error");
    assert!(!e.is_dismissed());
}
