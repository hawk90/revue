//! Tests for StatusIndicator widget
//!
//! Extracted from src/widget/display/status_indicator.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{away_indicator, busy_indicator, offline, online, Status, StatusIndicator};

/// Foreground colors drawn on the first row.
fn drawn_colors(s: &StatusIndicator) -> Vec<Color> {
    let mut buffer = Buffer::new(12, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 12, 1));
    s.render(&mut ctx);
    (0..12)
        .filter_map(|x| {
            let cell = buffer.get(x, 0).unwrap();
            (cell.symbol != ' ').then_some(cell.fg).flatten()
        })
        .collect()
}

#[test]
fn test_status_indicator_new() {
    let s = StatusIndicator::new(Status::Online);
    assert_eq!(s.get_status(), Status::Online);
    let colors = drawn_colors(&s);
    assert!(!colors.is_empty());
    assert!(colors.iter().all(|&c| c == Status::Online.color()));
}

#[test]
fn test_status_indicator_set_status_changes_color() {
    let mut s = StatusIndicator::new(Status::Online);
    s.set_status(Status::Busy);
    assert_eq!(s.get_status(), Status::Busy);
    assert!(drawn_colors(&s).contains(&Status::Busy.color()));
    assert!(!drawn_colors(&s).contains(&Status::Online.color()));
}

#[test]
fn test_online_helper() {
    assert_eq!(online().get_status(), Status::Online);
    assert_eq!(
        drawn_colors(&online()),
        drawn_colors(&StatusIndicator::online())
    );
}

#[test]
fn test_offline_helper() {
    assert_eq!(offline().get_status(), Status::Offline);
    assert_eq!(
        drawn_colors(&offline()),
        drawn_colors(&StatusIndicator::offline())
    );
}

#[test]
fn test_away_indicator_helper() {
    assert_eq!(away_indicator().get_status(), Status::Away);
    assert_eq!(
        drawn_colors(&away_indicator()),
        drawn_colors(&StatusIndicator::away())
    );
}

#[test]
fn test_busy_indicator_helper() {
    assert_eq!(busy_indicator().get_status(), Status::Busy);
    assert_eq!(
        drawn_colors(&busy_indicator()),
        drawn_colors(&StatusIndicator::busy())
    );
}

#[test]
fn test_status_colors_are_distinct() {
    let all = [Status::Online, Status::Offline, Status::Busy, Status::Away];
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert_ne!(a.color(), b.color(), "{:?} vs {:?}", a, b);
        }
    }
}
