//! Toast widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::{toast, RenderContext, Toast, ToastLevel, ToastPosition, View};

fn render(t: &Toast, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    t.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .filter(|&c| c != '\0')
        .collect::<String>()
}

/// Top-left corner of the drawn toast.
fn corner(buffer: &Buffer) -> Option<(u16, u16)> {
    (0..buffer.height()).find_map(|y| {
        (0..buffer.width())
            .find(|&x| buffer.get(x, y).map(|c| c.symbol) == Some('╭'))
            .map(|x| (x, y))
    })
}

// =========================================================================
// ToastLevel / ToastPosition
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
fn test_toast_enum_defaults() {
    assert_eq!(ToastLevel::default(), ToastLevel::Info);
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
// Builder
// =========================================================================

#[test]
fn test_toast_new_defaults() {
    let t = Toast::new("Test message");
    assert_eq!(t.get_message(), "Test message");
    assert_eq!(t.get_level(), ToastLevel::Info);
    assert_eq!(t.get_position(), ToastPosition::TopRight);
    assert_eq!(t.get_width(), None);
    assert!(t.get_show_icon());
    assert!(t.get_show_border());
}

#[test]
fn test_toast_message_kept_verbatim() {
    for msg in [
        String::new(),
        "A".repeat(1000),
        "🎉 Success! 你好 🎊".to_string(),
        "Line 1\nLine 2\nLine 3".to_string(),
    ] {
        assert_eq!(Toast::new(msg.clone()).get_message(), msg);
    }
}

#[test]
fn test_toast_level_constructors() {
    let cases = [
        (Toast::info("m"), ToastLevel::Info),
        (Toast::success("m"), ToastLevel::Success),
        (Toast::warning("m"), ToastLevel::Warning),
        (Toast::error("m"), ToastLevel::Error),
    ];
    for (t, level) in cases {
        assert_eq!(t.get_level(), level);
        assert_eq!(t.get_message(), "m");
    }
}

#[test]
fn test_toast_helper() {
    let t = toast("Quick toast");
    assert_eq!(t.get_message(), "Quick toast");
    assert_eq!(t.get_level(), ToastLevel::Info);
}

#[test]
fn test_toast_builder_chain() {
    let t = Toast::new("Builder test")
        .level(ToastLevel::Warning)
        .position(ToastPosition::BottomCenter)
        .width(60)
        .show_icon(false)
        .show_border(false);
    assert_eq!(t.get_level(), ToastLevel::Warning);
    assert_eq!(t.get_position(), ToastPosition::BottomCenter);
    assert_eq!(t.get_width(), Some(60));
    assert!(!t.get_show_icon());
    assert!(!t.get_show_border());

    assert_eq!(Toast::new("t").width(0).get_width(), Some(0));
    let t = t.show_icon(true).show_border(true);
    assert!(t.get_show_icon());
    assert!(t.get_show_border());
}

// =========================================================================
// Rendering
// =========================================================================

#[test]
fn test_toast_render_content() {
    let t = Toast::success("Hello World");
    let buffer = render(&t, 40, 10);
    let (x, y) = corner(&buffer).expect("border drawn");
    // Border, then the icon, then the message.
    assert!(
        row(&buffer, y + 1).contains("│ ✓ Hello World │"),
        "{}",
        row(&buffer, y + 1)
    );
    let corner_cell = buffer.get(x, y).unwrap();
    assert_eq!(corner_cell.fg, Some(Color::GREEN));
    assert_eq!(corner_cell.bg, Some(Color::rgb(0, 40, 0)));
}

#[test]
fn test_toast_render_without_border_or_icon() {
    let t = Toast::new("Plain")
        .show_border(false)
        .show_icon(false)
        .position(ToastPosition::TopLeft);
    let buffer = render(&t, 30, 5);
    assert!(corner(&buffer).is_none());
    assert_eq!(row(&buffer, 1).trim(), "Plain");
}

#[test]
fn test_toast_render_positions() {
    // "Test" with icon and border is 10 wide and 3 tall; 1 cell margin.
    let cases = [
        (ToastPosition::TopLeft, (1, 1)),
        (ToastPosition::TopCenter, (15, 1)),
        (ToastPosition::TopRight, (29, 1)),
        (ToastPosition::BottomLeft, (1, 16)),
        (ToastPosition::BottomCenter, (15, 16)),
        (ToastPosition::BottomRight, (29, 16)),
    ];
    for (position, expected) in cases {
        let t = Toast::new("Test").position(position);
        assert_eq!(corner(&render(&t, 40, 20)), Some(expected), "{position:?}");
    }
}

#[test]
fn test_toast_render_inline_stays_in_its_area() {
    // Without an overlay layer the toast is placed inside its own area.
    let t = Toast::new("Test").position(ToastPosition::BottomRight);
    let mut buffer = Buffer::new(60, 30);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(10, 5, 30, 10));
    t.render(&mut ctx);
    // 10 x 3 toast, 1 cell margin, in the 30 x 10 area at (10, 5).
    assert_eq!(corner(&buffer), Some((29, 11)));
}

#[test]
fn test_toast_render_too_small_draws_nothing() {
    let buffer = render(&Toast::new("Test"), 4, 10);
    assert!((0..10).all(|y| row(&buffer, y).trim().is_empty()));
}

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_toast_variants() {
        let view = vstack()
            .gap(1)
            .child(Toast::success("Operation completed!"))
            .child(Toast::error("An error occurred"))
            .child(Toast::warning("Please check your input"))
            .child(Toast::info("New updates available"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("toast_variants");
    }
}
