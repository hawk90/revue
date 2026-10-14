//! One notification as drawn: its rows, bars and colors

use super::helpers::{box_width, corners, message_of, render, row};
use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::{Color, Style, VisualStyle};
use revue::widget::theme::SEPARATOR_COLOR;
use revue::widget::{
    notification_center, Notification, NotificationCenter, NotificationPosition, RenderContext,
    View,
};

#[test]
fn test_center_show_icons() {
    let mut with = notification_center().position(NotificationPosition::TopLeft);
    with.success("Done");
    assert_eq!(message_of(&row(&render(&with, 50, 10), 1)), "✓ Done");

    let mut without = notification_center()
        .position(NotificationPosition::TopLeft)
        .show_icons(false);
    without.success("Done");
    assert_eq!(message_of(&row(&render(&without, 50, 10), 1)), "Done");
}

#[test]
fn test_center_title_line_carries_the_icon() {
    let mut c = notification_center().position(NotificationPosition::TopLeft);
    c.push(Notification::warning("Body").title("Heads up"));
    let buffer = render(&c, 50, 10);
    assert_eq!(message_of(&row(&buffer, 1)), "⚠ Heads up");
    assert_eq!(message_of(&row(&buffer, 2)), "Body");
    assert_eq!(corners(&buffer), [(0, 0)]);
    assert!(row(&buffer, 3).starts_with('╰'));
}

#[test]
fn test_center_progress_bar() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .width(24);
    c.push(Notification::info("Uploading").progress(0.5));
    let bar = row(&render(&c, 50, 10), 2);
    // 20 cells wide, half filled.
    assert_eq!(bar.trim_end(), "│ ██████████░░░░░░░░░░ │");
}

#[test]
fn test_center_timer_bar_drains() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .width(24);
    c.push(Notification::info("Timed").duration(4));
    let full = row(&render(&c, 50, 10), 2);
    assert_eq!(full.matches('━').count(), 20);

    c.tick();
    c.tick();
    let half = row(&render(&c, 50, 10), 2);
    assert_eq!(half.matches('━').count(), 10);

    let c = c.show_timer(false);
    let off = row(&render(&c, 50, 10), 2);
    assert_eq!(off.matches('━').count(), 0);
    assert_eq!(off.trim_end(), format!("╰{}╯", "─".repeat(22)));
}

#[test]
fn test_center_action_line() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .show_timer(false);
    c.push(Notification::info("Failed").action("Retry"));
    let buffer = render(&c, 50, 10);
    let text: Vec<String> = (0..10).map(|y| row(&buffer, y)).collect();
    // Border, message, action, border: the action sits in its reserved row
    assert!(text[1].contains("Failed"), "{text:#?}");
    assert!(text[2].contains("[Retry]"), "{text:#?}");
    assert!(text[2].starts_with('│'), "{text:#?}");
    assert!(text[3].starts_with('╰'), "{text:#?}");
    assert!(!text[3].contains("Retry"), "{text:#?}");
    // Right-aligned with one blank before the border
    let width = box_width(&buffer, (0, 0)) as usize;
    let line: Vec<char> = text[2].chars().take(width).collect();
    assert_eq!(line[width - 1], '│');
    assert_eq!(line[width - 2], ' ');
    assert_eq!(line[width - 3], ']');
    // Drawn in the level color
    let r = (0..50)
        .find(|&x| buffer.get(x, 2).unwrap().symbol == 'R')
        .unwrap();
    assert_eq!(buffer.get(r, 2).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_center_action_line_clips_long_labels() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .width(20);
    c.push(Notification::info("x").action("A very long action label"));
    let buffer = render(&c, 30, 10);
    let line = row(&buffer, 2);
    let chars: Vec<char> = line.chars().collect();
    // The label is cut short of the right border, which survives
    assert_eq!(chars[19], '│', "{line:?}");
    assert_eq!(chars[18], ' ', "{line:?}");
    assert!(line.contains("[A very long"), "{line:?}");
    assert_eq!(chars[0], '│', "{line:?}");
}

// =========================================================================
// Colors and modifiers of each row
// =========================================================================

/// An error notification with every row, 24 wide at the top left
fn full() -> NotificationCenter {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .width(24);
    c.push(
        Notification::error("Body")
            .title("Head")
            .progress(0.5)
            .action("Go")
            .duration(4),
    );
    c
}

fn cell(buffer: &Buffer, x: u16, y: u16) -> (char, Option<Color>, Option<Color>, Modifier) {
    let c = buffer.get(x, y).unwrap();
    (c.symbol, c.fg, c.bg, c.modifier)
}

#[test]
fn test_center_row_colors() {
    let red = Some(Color::RED);
    let bg = Some(Color::rgb(60, 20, 20));
    let plain = Modifier::empty();
    let buffer = render(&full(), 30, 8);
    // Top border, in the level color
    assert_eq!(cell(&buffer, 0, 0), ('╭', red, None, plain));
    assert_eq!(cell(&buffer, 5, 0), ('─', red, None, plain));
    assert_eq!(cell(&buffer, 23, 0), ('╮', red, None, plain));
    // Title row: the icon in the level color, the title white and bold, the
    // rest of the row on the level background
    assert_eq!(cell(&buffer, 0, 1), ('│', red, None, plain));
    assert_eq!(cell(&buffer, 1, 1), ('✗', red, bg, plain));
    assert_eq!(cell(&buffer, 2, 1), (' ', None, bg, plain));
    assert_eq!(
        cell(&buffer, 3, 1),
        ('H', Some(Color::WHITE), bg, Modifier::BOLD)
    );
    assert_eq!(cell(&buffer, 22, 1), (' ', None, bg, plain));
    assert_eq!(cell(&buffer, 23, 1), ('│', red, None, plain));
    // Message row: white, not bold, from the first inner column (the
    // title carries the icon)
    assert_eq!(cell(&buffer, 1, 2), ('B', Some(Color::WHITE), bg, plain));
    // Progress row: 20 cells from column 2, half in the level color
    assert_eq!(cell(&buffer, 1, 3), (' ', None, bg, plain));
    assert_eq!(cell(&buffer, 2, 3), ('█', red, bg, plain));
    assert_eq!(cell(&buffer, 11, 3), ('█', red, bg, plain));
    assert_eq!(
        cell(&buffer, 12, 3),
        ('░', Some(SEPARATOR_COLOR), bg, plain)
    );
    assert_eq!(
        cell(&buffer, 21, 3),
        ('░', Some(SEPARATOR_COLOR), bg, plain)
    );
    // Action row: bold, in the level color, ending one column before the
    // border
    assert_eq!(cell(&buffer, 18, 4), ('[', red, bg, Modifier::BOLD));
    assert_eq!(cell(&buffer, 21, 4), (']', red, bg, Modifier::BOLD));
    assert_eq!(cell(&buffer, 22, 4), (' ', None, bg, plain));
    // Bottom border: the timer in the level color
    assert_eq!(cell(&buffer, 0, 5), ('╰', red, None, plain));
    assert_eq!(cell(&buffer, 1, 5), ('━', red, None, plain));
    assert_eq!(cell(&buffer, 23, 5), ('╯', red, None, plain));
    assert_eq!(row(&buffer, 6).trim(), "");
}

#[test]
fn test_center_selected_border_is_white() {
    let mut c = full();
    c.select_next();
    let buffer = render(&c, 30, 8);
    let white = Some(Color::WHITE);
    for (x, y) in [(0, 0), (5, 0), (23, 0), (0, 1), (23, 3), (0, 5), (23, 5)] {
        assert_eq!(buffer.get(x, y).unwrap().fg, white, "({x}, {y})");
    }
    // The icon, the bars and the action keep the level color, and so does
    // the drained timer's filled part; its drained part takes the border's
    assert_eq!(buffer.get(1, 1).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(2, 3).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(18, 4).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(1, 5).unwrap().fg, Some(Color::RED));
    assert_eq!(cell(&buffer, 22, 5), ('─', white, None, Modifier::empty()));
}

#[test]
fn test_center_stylesheet_color_replaces_the_level_color() {
    let style = Style {
        visual: VisualStyle {
            color: Color::rgb(9, 8, 7),
            ..VisualStyle::default()
        },
        ..Default::default()
    };
    let mut buffer = Buffer::new(30, 8);
    let mut ctx = RenderContext::with_style(&mut buffer, Rect::new(0, 0, 30, 8), &style);
    full().render(&mut ctx);
    let author = Some(Color::rgb(9, 8, 7));
    // Borders, icon, bar, action and timer
    for (x, y) in [(0, 0), (1, 1), (2, 3), (18, 4), (1, 5), (23, 5)] {
        assert_eq!(buffer.get(x, y).unwrap().fg, author, "({x}, {y})");
    }
    // The text stays white on the level background
    assert_eq!(cell(&buffer, 1, 2).1, Some(Color::WHITE));
    assert_eq!(cell(&buffer, 1, 2).2, Some(Color::rgb(60, 20, 20)));
}

#[test]
fn test_center_text_stops_before_the_last_inner_column() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .width(20)
        .show_icons(false);
    c.push(Notification::info("x".repeat(40)).title("y".repeat(40)));
    let buffer = render(&c, 20, 6);
    // Columns 1..=17 hold text, 18 stays blank, 19 is the border
    for y in [1, 2] {
        let line = row(&buffer, y);
        assert_eq!(
            line.chars().filter(|c| c.is_alphabetic()).count(),
            17,
            "{line}"
        );
        assert!(line.ends_with(" │"), "{line}");
    }
}

#[test]
fn test_center_wide_text_takes_two_columns() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .show_icons(false);
    c.info("한글ab");
    let buffer = render(&c, 50, 4);
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '한');
    assert_eq!(buffer.get(3, 1).unwrap().symbol, '글');
    assert_eq!(buffer.get(5, 1).unwrap().symbol, 'a');
    assert_eq!(buffer.get(6, 1).unwrap().symbol, 'b');
}

#[test]
fn test_center_icons_per_level() {
    for (n, icon) in [
        (Notification::info("m"), 'ℹ'),
        (Notification::success("m"), '✓'),
        (Notification::warning("m"), '⚠'),
        (Notification::error("m"), '✗'),
        (Notification::debug("m"), '⚙'),
    ] {
        let mut c = notification_center().position(NotificationPosition::TopLeft);
        c.push(n);
        let buffer = render(&c, 50, 4);
        assert_eq!(buffer.get(1, 1).unwrap().symbol, icon);
        assert_eq!(buffer.get(2, 1).unwrap().symbol, ' ');
        assert_eq!(buffer.get(3, 1).unwrap().symbol, 'm');
    }
}

#[test]
fn test_center_permanent_notification_has_no_timer() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .width(20);
    c.push(Notification::info("m").duration(0));
    let bottom = row(&render(&c, 20, 4), 2);
    assert_eq!(bottom, format!("╰{}╯", "─".repeat(18)));
}

proptest! {
    /// The progress bar fills round(progress × (width − 4)) of its
    /// width − 4 cells, from the left.
    #[test]
    fn prop_center_progress_bar(width in 20u16..60, progress in 0.0f64..=1.0) {
        let mut c = notification_center().position(NotificationPosition::TopLeft).width(width);
        c.push(Notification::info("m").progress(progress));
        let buffer = render(&c, width, 5);
        let bar: String = (2..width - 2).map(|x| buffer.get(x, 2).unwrap().symbol).collect();
        let filled = (progress * (width - 4) as f64).round() as usize;
        let expected = format!("{}{}", "█".repeat(filled), "░".repeat(width as usize - 4 - filled));
        prop_assert_eq!(bar, expected);
        prop_assert_eq!(buffer.get(1, 2).unwrap().symbol, ' ');
    }

    /// The timer drains from the right as the notification ticks: its
    /// filled cells are a run from the left that never grows, and it stays
    /// within three cells of the remaining share of the border. (A fresh
    /// timer is not drawn full: #1048.)
    #[test]
    fn prop_center_timer_drains(width in 20u16..60, duration in 1u32..30) {
        let mut c = notification_center().position(NotificationPosition::TopLeft).width(width);
        c.push(Notification::info("m").duration(duration));
        let mut last = usize::MAX;
        for tick in 0..duration {
            let line = row(&render(&c, width, 4), 2);
            let inner: Vec<char> = line.chars().skip(1).take(width as usize - 2).collect();
            let filled = inner.iter().take_while(|&&ch| ch == '━').count();
            prop_assert!(inner[filled..].iter().all(|&ch| ch == '─'), "{}", line);
            prop_assert!(filled <= last, "tick {}: {}", tick, line);
            let share = (duration - tick) as f64 / duration as f64 * (width - 2) as f64;
            prop_assert!((filled as f64 - share).abs() <= 3.0, "tick {}: {}", tick, line);
            last = filled;
            c.tick();
        }
        prop_assert!(c.is_empty());
    }
}
