//! Timeline selection tests: select, next and previous, the ends and out-of-range indexes

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{Timeline, TimelineEvent};

#[test]
fn test_timeline_select_none() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));
    tl.select(None);
    assert!(tl.selected_event().is_none());
}

#[test]
fn test_timeline_select_first() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));
    tl.select(Some(0));
    assert_eq!(tl.selected_event().unwrap().title, "Event 1");
}

#[test]
fn test_timeline_select_last() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"));
    tl.select(Some(2));
    assert_eq!(tl.selected_event().unwrap().title, "Event 3");
}

#[test]
fn test_timeline_select_next_from_none() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));
    tl.select_next();
    assert!(tl.selected_event().is_some());
    assert_eq!(tl.selected_event().unwrap().title, "Event 1");
}

#[test]
fn test_timeline_select_next_middle() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"));
    tl.select(Some(1));
    tl.select_next();
    assert_eq!(tl.selected_event().unwrap().title, "Event 3");
}

#[test]
fn test_timeline_select_next_at_end() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));
    tl.select(Some(1));
    tl.select_next();
    // Should stay at last index
    assert_eq!(tl.selected_event().unwrap().title, "Event 2");
}

#[test]
fn test_timeline_select_prev_from_start() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));
    tl.select(Some(0));
    tl.select_prev();
    // Should stay at first index
    assert_eq!(tl.selected_event().unwrap().title, "Event 1");
}

#[test]
fn test_timeline_select_prev_middle() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"));
    tl.select(Some(2));
    tl.select_prev();
    assert_eq!(tl.selected_event().unwrap().title, "Event 2");
}

#[test]
fn test_timeline_select_prev_from_none() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));
    tl.select_prev();
    // Should remain None
    assert!(tl.selected_event().is_none());
}

#[test]
fn test_timeline_navigate_forward_backward() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"))
        .event(TimelineEvent::new("Event 4"));

    tl.select_next();
    assert_eq!(tl.selected_event().unwrap().title, "Event 1");

    tl.select_next();
    assert_eq!(tl.selected_event().unwrap().title, "Event 2");

    tl.select_next();
    assert_eq!(tl.selected_event().unwrap().title, "Event 3");

    tl.select_prev();
    assert_eq!(tl.selected_event().unwrap().title, "Event 2");

    tl.select_prev();
    assert_eq!(tl.selected_event().unwrap().title, "Event 1");
}

#[test]
fn test_timeline_select_out_of_bounds() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    // Past the end: the nearest event, the last (#975)
    tl.select(Some(10));
    assert_eq!(
        tl.selected_event().map(|e| e.title.as_str()),
        Some("Event 2")
    );
}

#[test]
fn test_timeline_empty_select_operations() {
    let mut tl = Timeline::new();

    tl.select(Some(0));
    assert!(tl.selected_event().is_none());

    // Note: select_next() and select_prev() on empty timeline cause overflow in implementation
    // This is an edge case - implementation doesn't handle empty timeline properly
    // For now, just verify the widget handles empty state without crashing on render
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);
    // Should not panic on render even with empty timeline
}

#[test]
fn test_timeline_select_first_index_boundary() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    tl.select(Some(0));
    tl.select_prev();
    assert_eq!(
        tl.selected_event().unwrap().title,
        "Event 1",
        "Should stay at first index"
    );
}

#[test]
fn test_timeline_select_last_index_boundary() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    tl.select(Some(1));
    tl.select_next();
    assert_eq!(
        tl.selected_event().unwrap().title,
        "Event 2",
        "Should stay at last index"
    );
}

#[test]
fn test_timeline_selection_with_invalid_index() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    tl.select(Some(100));
    assert_eq!(
        tl.selected_event().map(|e| e.title.as_str()),
        Some("Event 2"),
        "an index past the end selects the last event"
    );
}

/// An index past the end was kept, and select_next on an empty timeline
/// with an index set underflowed `len() - 1` (#975).
#[test]
fn timeline_selection_stays_on_an_event() {
    let mut empty = Timeline::new();
    empty.select(Some(0));
    empty.select_next(); // panicked: `self.events.len() - 1` underflows
    assert!(empty.selected_event().is_none());

    let mut tl = Timeline::new()
        .event(TimelineEvent::new("a"))
        .event(TimelineEvent::new("b"));
    tl.select(Some(5));
    assert_eq!(tl.selected_event().map(|e| e.title.as_str()), Some("b"));
    tl.select_prev();
    assert_eq!(tl.selected_event().map(|e| e.title.as_str()), Some("a"));
}

#[test]
fn test_timeline_navigation_through_many_events() {
    let mut tl = Timeline::new();
    for i in 0..50 {
        tl = tl.event(TimelineEvent::new(format!("Event {}", i)));
    }

    // Navigate to various positions
    tl.select(Some(25));
    assert_eq!(tl.selected_event().unwrap().title, "Event 25");

    tl.select(Some(49));
    assert_eq!(tl.selected_event().unwrap().title, "Event 49");

    tl.select(Some(0));
    assert_eq!(tl.selected_event().unwrap().title, "Event 0");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Any sequence of select_next / select_prev on a non-empty timeline
    /// keeps the selection on an event: None until the first next, then
    /// moving by one and stopping at both ends.
    #[test]
    fn test_timeline_selection_moves_and_stops(
        len in 1usize..20,
        moves in prop::collection::vec(any::<bool>(), 0..60),
    ) {
        let mut tl = Timeline::new().events((0..len).map(|i| TimelineEvent::new(format!("E{i}"))).collect());
        let mut model: Option<usize> = None;
        for next in moves {
            if next {
                tl.select_next();
                model = Some(model.map_or(0, |i| (i + 1).min(len - 1)));
            } else {
                tl.select_prev();
                model = model.map(|i| i.saturating_sub(1));
            }
            let selected = tl.selected_event().map(|e| e.title.clone());
            prop_assert_eq!(selected, model.map(|i| format!("E{i}")));
        }
    }
}
