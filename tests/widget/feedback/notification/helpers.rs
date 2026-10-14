//! Helpers shared by the NotificationCenter tests
//!
//! The center's fields are private, so its state is read through count(),
//! handle_key() and what it renders. A selected notification draws its
//! border in white instead of its level color.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::{
    notification_center, NotificationCenter, NotificationPosition, RenderContext, View,
};

pub(super) fn render(center: &NotificationCenter, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    center.render(&mut ctx);
    buffer
}

pub(super) fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .filter(|&c| c != '\0')
        .collect()
}

/// (x, y) of every notification's top-left corner, top to bottom.
pub(super) fn corners(buffer: &Buffer) -> Vec<(u16, u16)> {
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
pub(super) fn box_width(buffer: &Buffer, (x, y): (u16, u16)) -> u16 {
    (x..buffer.width())
        .find(|&cx| buffer.get(cx, y).map(|c| c.symbol) == Some('╮'))
        .map(|right| right - x + 1)
        .unwrap()
}

/// Messages of the notifications drawn with a white (selected) border.
/// Expects a top-left center without icons or titles.
pub(super) fn selected_messages(center: &NotificationCenter) -> Vec<String> {
    let buffer = render(center, 50, 30);
    corners(&buffer)
        .into_iter()
        .filter(|&(x, y)| buffer.get(x, y).unwrap().fg == Some(Color::WHITE))
        .map(|(_, y)| message_of(&row(&buffer, y + 1)))
        .collect()
}

/// A focused top-left center holding info notifications "1", "2", ...
pub(super) fn center_with(n: usize) -> NotificationCenter {
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
pub(super) fn message_of(text: &str) -> String {
    text.trim_matches(|c| c == '│' || c == ' ').to_string()
}
