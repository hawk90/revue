//! Tests for Toast widget
//!
//! Extracted from src/widget/feedback/toast.rs

use revue::prelude::*;

// =========================================================================
// ToastLevel enum tests
// =========================================================================

#[test]
fn test_toast_level_icon() {
    assert_eq!(ToastLevel::Info.icon(), 'ℹ');
    assert_eq!(ToastLevel::Success.icon(), '✓');
    assert_eq!(ToastLevel::Warning.icon(), '⚠');
    assert_eq!(ToastLevel::Error.icon(), '✗');
}

#[test]
fn test_toast_level_color() {
    assert_eq!(ToastLevel::Info.color(), Color::CYAN);
    assert_eq!(ToastLevel::Success.color(), Color::GREEN);
    assert_eq!(ToastLevel::Warning.color(), Color::YELLOW);
    assert_eq!(ToastLevel::Error.color(), Color::RED);
}

#[test]
fn test_toast_level_bg_color() {
    assert_eq!(ToastLevel::Info.bg_color(), Color::rgb(0, 40, 60));
    assert_eq!(ToastLevel::Success.bg_color(), Color::rgb(0, 40, 0));
    assert_eq!(ToastLevel::Warning.bg_color(), Color::rgb(60, 40, 0));
    assert_eq!(ToastLevel::Error.bg_color(), Color::rgb(60, 0, 0));
}

#[test]
fn test_toast_level_default() {
    assert_eq!(ToastLevel::default(), ToastLevel::Info);
}

#[test]
fn test_toast_all_icons_are_different() {
    let info_icon = ToastLevel::Info.icon();
    let success_icon = ToastLevel::Success.icon();
    let warning_icon = ToastLevel::Warning.icon();
    let error_icon = ToastLevel::Error.icon();

    assert_ne!(info_icon, success_icon);
    assert_ne!(info_icon, warning_icon);
    assert_ne!(info_icon, error_icon);
    assert_ne!(success_icon, warning_icon);
    assert_ne!(success_icon, error_icon);
    assert_ne!(warning_icon, error_icon);
}

#[test]
fn test_toast_all_colors_are_different() {
    let info_color = ToastLevel::Info.color();
    let success_color = ToastLevel::Success.color();
    let warning_color = ToastLevel::Warning.color();
    let error_color = ToastLevel::Error.color();

    assert_ne!(info_color, success_color);
    assert_ne!(info_color, warning_color);
    assert_ne!(info_color, error_color);
}

// =========================================================================
// ToastPosition enum tests
// =========================================================================

#[test]
fn test_toast_position_default() {
    assert_eq!(ToastPosition::default(), ToastPosition::TopRight);
}

#[test]
fn test_toast_all_positions_distinct() {
    let positions = [
        ToastPosition::TopLeft,
        ToastPosition::TopCenter,
        ToastPosition::TopRight,
        ToastPosition::BottomLeft,
        ToastPosition::BottomCenter,
        ToastPosition::BottomRight,
    ];

    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            assert_ne!(positions[i], positions[j]);
        }
    }
}

// =========================================================================
// Toast builder tests
// =========================================================================

#[test]
fn test_toast_new() {
    let t = Toast::new("Test message");
    assert_eq!(t.get_message(), "Test message");
    assert_eq!(t.get_level(), ToastLevel::Info);
}

#[test]
fn test_toast_new_with_string() {
    let msg = String::from("Owned message");
    let t = Toast::new(msg);
    assert_eq!(t.get_message(), "Owned message");
}

#[test]
fn test_toast_empty_message() {
    let t = Toast::new("");
    assert_eq!(t.get_message(), "");
}

#[test]
fn test_toast_long_message() {
    let long_msg = "A".repeat(1000);
    let t = Toast::new(long_msg.clone());
    assert_eq!(t.get_message().len(), 1000);
}

#[test]
fn test_toast_levels() {
    let info = Toast::info("Info");
    assert_eq!(info.get_level(), ToastLevel::Info);

    let success = Toast::success("Success");
    assert_eq!(success.get_level(), ToastLevel::Success);

    let warning = Toast::warning("Warning");
    assert_eq!(warning.get_level(), ToastLevel::Warning);

    let error = Toast::error("Error");
    assert_eq!(error.get_level(), ToastLevel::Error);
}

#[test]
fn test_toast_position() {
    let t = Toast::new("Test").position(ToastPosition::BottomLeft);
    assert_eq!(t.get_position(), ToastPosition::BottomLeft);
}

#[test]
fn test_toast_width() {
    let t = Toast::new("Test").width(50);
    assert_eq!(t.get_width(), Some(50));
}

#[test]
fn test_toast_width_zero() {
    let t = Toast::new("Test").width(0);
    assert_eq!(t.get_width(), Some(0));
}

#[test]
fn test_toast_show_icon_true() {
    let t = Toast::new("Test").show_icon(true);
    assert!(t.get_show_icon());
}

#[test]
fn test_toast_show_icon_false() {
    let t = Toast::new("Test").show_icon(false);
    assert!(!t.get_show_icon());
}

#[test]
fn test_toast_show_border_true() {
    let t = Toast::new("Test").show_border(true);
    assert!(t.get_show_border());
}

#[test]
fn test_toast_show_border_false() {
    let t = Toast::new("Test").show_border(false);
    assert!(!t.get_show_border());
}

#[test]
fn test_toast_builder_chain() {
    let t = Toast::new("Builder test")
        .level(ToastLevel::Warning)
        .position(ToastPosition::BottomCenter)
        .width(60)
        .show_icon(false)
        .show_border(true);

    assert_eq!(t.get_message(), "Builder test");
    assert_eq!(t.get_level(), ToastLevel::Warning);
    assert_eq!(t.get_position(), ToastPosition::BottomCenter);
    assert_eq!(t.get_width(), Some(60));
    assert!(!t.get_show_icon());
    assert!(t.get_show_border());
}

#[test]
fn test_toast_info_method() {
    let t = Toast::info("Info message");
    assert_eq!(t.get_level(), ToastLevel::Info);
    assert_eq!(t.get_message(), "Info message");
}

#[test]
fn test_toast_success_method() {
    let t = Toast::success("Success message");
    assert_eq!(t.get_level(), ToastLevel::Success);
    assert_eq!(t.get_message(), "Success message");
}

#[test]
fn test_toast_warning_method() {
    let t = Toast::warning("Warning message");
    assert_eq!(t.get_level(), ToastLevel::Warning);
    assert_eq!(t.get_message(), "Warning message");
}

#[test]
fn test_toast_error_method() {
    let t = Toast::error("Error message");
    assert_eq!(t.get_level(), ToastLevel::Error);
    assert_eq!(t.get_message(), "Error message");
}

#[test]
fn test_toast_helper() {
    let t = toast("Quick toast");
    assert_eq!(t.get_message(), "Quick toast");
}

#[test]
fn test_toast_unicode_message() {
    let t = Toast::new("🎉 Success! 你好 🎊");
    assert_eq!(t.get_message(), "🎉 Success! 你好 🎊");
}

#[test]
fn test_toast_multiline_message() {
    let t = Toast::new("Line 1\nLine 2\nLine 3");
    assert_eq!(t.get_message(), "Line 1\nLine 2\nLine 3");
}

#[test]
fn test_toast_with_all_levels() {
    let msg = "Test";
    let _ = Toast::info(msg);
    let _ = Toast::success(msg);
    let _ = Toast::warning(msg);
    let _ = Toast::error(msg);
}

// =========================================================================
// Toast rendering tests
// =========================================================================

#[test]
fn test_toast_render() {
    let t = Toast::new("Hello World")
        .level(ToastLevel::Success)
        .position(ToastPosition::TopRight);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    t.render(&mut ctx);
}

#[test]
fn test_toast_no_border() {
    let t = Toast::new("No border").show_border(false);

    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    t.render(&mut ctx);
}

#[test]
fn test_toast_no_icon() {
    let t = Toast::new("No icon").show_icon(false);

    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    t.render(&mut ctx);
}

#[test]
fn test_toast_all_positions() {
    let positions = [
        ToastPosition::TopLeft,
        ToastPosition::TopCenter,
        ToastPosition::TopRight,
        ToastPosition::BottomLeft,
        ToastPosition::BottomCenter,
        ToastPosition::BottomRight,
    ];

    for pos in positions {
        let t = Toast::new("Test").position(pos);
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);
        t.render(&mut ctx);
    }
}
