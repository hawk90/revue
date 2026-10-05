//! NotificationCenter tests
//!
//! The center's fields are private, so its state is read through count(),
//! handle_key() and what it renders. A selected notification draws its
//! border in white instead of its level color.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::{
    notification_center, Notification, NotificationCenter, NotificationPosition, RenderContext,
    View,
};

fn render(center: &NotificationCenter, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    center.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .filter(|&c| c != '\0')
        .collect()
}

/// (x, y) of every notification's top-left corner, top to bottom.
fn corners(buffer: &Buffer) -> Vec<(u16, u16)> {
    let mut found = Vec::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if buffer.get(x, y).map(|c| c.symbol) == Some('╭') {
                found.push((x, y));
            }
        }
    }
    found
}

/// Width of the notification whose corner is at (x, y).
fn box_width(buffer: &Buffer, (x, y): (u16, u16)) -> u16 {
    (x..buffer.width())
        .find(|&cx| buffer.get(cx, y).map(|c| c.symbol) == Some('╮'))
        .map(|right| right - x + 1)
        .unwrap()
}

/// Messages of the notifications drawn with a white (selected) border.
/// Expects a top-left center without icons or titles.
fn selected_messages(center: &NotificationCenter) -> Vec<String> {
    let buffer = render(center, 50, 30);
    corners(&buffer)
        .into_iter()
        .filter(|&(x, y)| buffer.get(x, y).unwrap().fg == Some(Color::WHITE))
        .map(|(_, y)| message_of(&row(&buffer, y + 1)))
        .collect()
}

/// A focused top-left center holding info notifications "1", "2", ...
fn center_with(n: usize) -> NotificationCenter {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .show_icons(false)
        .focused(true);
    for i in 1..=n {
        c.info(i.to_string());
    }
    c
}

/// The message text as drawn: everything between the side borders.
fn message_of(text: &str) -> String {
    text.trim_matches(|c| c == '│' || c == ' ').to_string()
}

// =========================================================================
// Contents
// =========================================================================

#[test]
fn test_center_new() {
    let c = notification_center();
    assert!(c.is_empty());
    assert_eq!(c.count(), 0);
    assert!(corners(&render(&c, 50, 20)).is_empty());

    let d = NotificationCenter::default();
    assert!(d.is_empty());
}

#[test]
fn test_center_push_and_shortcuts() {
    let mut c = notification_center();
    c.push(Notification::info("Pushed"));
    c.info("Info");
    c.success("Success");
    c.warning("Warning");
    c.error("Error");
    assert_eq!(c.count(), 5);
    assert!(!c.is_empty());
}

#[test]
fn test_center_dismiss_by_id() {
    let mut c = notification_center();
    let keep = Notification::info("keep");
    let drop = Notification::info("drop");
    let drop_id = drop.id;
    c.push(keep);
    c.push(drop);

    c.dismiss(drop_id);
    assert_eq!(c.count(), 1);
    c.dismiss(drop_id);
    assert_eq!(c.count(), 1);
}

#[test]
fn test_center_clear() {
    let mut c = center_with(3);
    c.select_next();
    c.clear();
    assert!(c.is_empty());
    // Nothing is left to select.
    c.info("new");
    assert!(selected_messages(&c).is_empty());
}

#[test]
fn test_center_tick_expires_notifications() {
    let mut c = notification_center();
    c.tick();
    assert!(c.is_empty());

    c.push(Notification::info("short").duration(2));
    c.push(Notification::info("forever").duration(0));
    c.tick();
    assert_eq!(c.count(), 2);
    c.tick();
    assert_eq!(c.count(), 1);
    for _ in 0..100 {
        c.tick();
    }
    assert_eq!(c.count(), 1);
}

// =========================================================================
// Selection
// =========================================================================

#[test]
fn test_center_selection() {
    let mut c = center_with(3);
    assert!(selected_messages(&c).is_empty());
    c.select_next();
    assert_eq!(selected_messages(&c), ["1"]);
    c.select_next();
    assert_eq!(selected_messages(&c), ["2"]);
    c.select_prev();
    assert_eq!(selected_messages(&c), ["1"]);
}

#[test]
fn test_center_selection_wraps() {
    let mut c = center_with(2);
    c.select_prev();
    assert_eq!(selected_messages(&c), ["2"]);
    c.select_next();
    assert_eq!(selected_messages(&c), ["1"]);
    c.select_prev();
    assert_eq!(selected_messages(&c), ["2"]);
}

#[test]
fn test_center_select_on_empty_does_nothing() {
    let mut c = notification_center();
    c.select_next();
    c.select_prev();
    c.dismiss_selected();
    assert!(c.is_empty());
}

#[test]
fn test_center_dismiss_selected() {
    let mut c = center_with(2);
    c.dismiss_selected(); // nothing selected
    assert_eq!(c.count(), 2);

    c.select_next(); // "1"
    c.dismiss_selected();
    assert_eq!(c.count(), 1);
    let buffer = render(&c, 50, 20);
    assert_eq!(message_of(&row(&buffer, 1)), "2");
}

#[test]
fn test_center_dismiss_moves_selection_to_last() {
    let mut c = center_with(3);
    c.select_prev(); // "3"
    c.dismiss_selected();
    assert_eq!(selected_messages(&c), ["2"]);

    let mut single = center_with(1);
    single.select_next();
    single.dismiss_selected();
    assert!(single.is_empty());
    single.info("new");
    assert!(selected_messages(&single).is_empty());
}

#[test]
fn test_center_tick_adjusts_selection() {
    let mut c = notification_center()
        .position(NotificationPosition::TopLeft)
        .show_icons(false);
    c.push(Notification::info("stays").duration(0));
    c.push(Notification::info("expires").duration(1));
    c.select_prev(); // "expires"
    c.tick();
    assert_eq!(c.count(), 1);
    assert_eq!(selected_messages(&c), ["stays"]);
}

// =========================================================================
// Keys
// =========================================================================

#[test]
fn test_center_handle_key_needs_focus_and_content() {
    let mut unfocused = notification_center();
    unfocused.info("Test");
    assert!(!unfocused.handle_key(&Key::Down));

    let mut empty = notification_center().focused(true);
    assert!(!empty.handle_key(&Key::Down));

    let mut c = center_with(1);
    assert!(!c.handle_key(&Key::Char('x')));
    assert!(c.handle_key(&Key::Down));
}

#[test]
fn test_center_handle_key_navigation() {
    let mut c = center_with(2);
    assert!(c.handle_key(&Key::Up));
    assert_eq!(selected_messages(&c), ["2"]);
    assert!(c.handle_key(&Key::Down));
    assert_eq!(selected_messages(&c), ["1"]);
    assert!(c.handle_key(&Key::Char('k')));
    assert_eq!(selected_messages(&c), ["2"]);
    assert!(c.handle_key(&Key::Char('j')));
    assert_eq!(selected_messages(&c), ["1"]);
}

#[test]
fn test_center_handle_key_dismiss() {
    for key in [Key::Char('d'), Key::Delete] {
        let mut c = center_with(1);
        c.select_next();
        assert!(c.handle_key(&key));
        assert!(c.is_empty(), "{key:?}");
    }
}

#[test]
fn test_center_handle_key_clear() {
    let mut c = center_with(3);
    assert!(c.handle_key(&Key::Char('c')));
    assert!(c.is_empty());
}

// =========================================================================
// Builders, observed through rendering
// =========================================================================

#[test]
fn test_center_positions() {
    // 40 wide, 3 tall (message line plus borders).
    let cases = [
        (NotificationPosition::TopRight, (40, 0)),
        (NotificationPosition::TopLeft, (0, 0)),
        (NotificationPosition::TopCenter, (20, 0)),
        (NotificationPosition::BottomRight, (40, 21)),
        (NotificationPosition::BottomLeft, (0, 21)),
        (NotificationPosition::BottomCenter, (20, 21)),
    ];
    for (position, corner) in cases {
        let mut c = notification_center().position(position);
        c.info("Test");
        assert_eq!(corners(&render(&c, 80, 24)), [corner], "{position:?}");
    }
}

#[test]
fn test_center_newest_first_and_spacing() {
    let mut c = center_with(2).spacing(3);
    let buffer = render(&c, 50, 20);
    assert_eq!(corners(&buffer), [(0, 0), (0, 6)]);
    assert_eq!(message_of(&row(&buffer, 1)), "2");
    assert_eq!(message_of(&row(&buffer, 7)), "1");

    c.clear();
    c.info("only");
    assert_eq!(corners(&render(&c, 50, 20)), [(0, 0)]);
}

#[test]
fn test_center_bottom_stacks_upward() {
    let mut c = notification_center()
        .position(NotificationPosition::BottomLeft)
        .show_icons(false);
    c.info("old");
    c.info("new");
    let buffer = render(&c, 50, 20);
    assert_eq!(corners(&buffer), [(0, 13), (0, 17)]);
    assert_eq!(message_of(&row(&buffer, 18)), "new");
    assert_eq!(message_of(&row(&buffer, 14)), "old");
}

#[test]
fn test_center_max_visible() {
    let mut c = center_with(3).max_visible(2);
    let buffer = render(&c, 50, 30);
    assert_eq!(corners(&buffer).len(), 2);
    assert_eq!(c.count(), 3);

    c = c.max_visible(0);
    assert_eq!(corners(&render(&c, 50, 30)).len(), 1, "minimum is 1");
}

#[test]
fn test_center_width() {
    let mut c = notification_center().position(NotificationPosition::TopLeft);
    c.info("Test");
    assert_eq!(box_width(&render(&c, 80, 10), (0, 0)), 40);

    let c = c.width(60);
    assert_eq!(box_width(&render(&c, 80, 10), (0, 0)), 60);

    let c = c.width(10);
    assert_eq!(box_width(&render(&c, 80, 10), (0, 0)), 20, "minimum is 20");
}

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
