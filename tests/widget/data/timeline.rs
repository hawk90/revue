//! Public API tests for Timeline widget

use revue::style::Color;
use revue::widget::data::{
    timeline, timeline_event, EventType, TimelineOrientation, TimelineStyle,
};

#[test]
fn test_timeline_event() {
    let event = timeline_event("Test Event")
        .description("Description")
        .timestamp("10:30 AM")
        .success();

    assert_eq!(event.title, "Test Event");
    assert_eq!(event.description, Some("Description".to_string()));
    assert_eq!(event.event_type, EventType::Success);
}

#[test]
fn test_timeline() {
    let tl = timeline()
        .event(timeline_event("First"))
        .event(timeline_event("Second"))
        .event(timeline_event("Third"));

    assert_eq!(tl.len(), 3);
    assert!(!tl.is_empty());
}

#[test]
fn test_timeline_selection_navigation() {
    let mut tl = timeline()
        .event(timeline_event("A"))
        .event(timeline_event("B"))
        .event(timeline_event("C"));

    tl.select_next();
    let event = tl.selected_event();
    assert!(event.is_some());
    assert_eq!(event.unwrap().title, "A");

    tl.select_next();
    let event = tl.selected_event();
    assert!(event.is_some());
    assert_eq!(event.unwrap().title, "B");

    tl.select_prev();
    let event = tl.selected_event();
    assert!(event.is_some());
    assert_eq!(event.unwrap().title, "A");
}

#[test]
fn test_event_colors() {
    assert_eq!(EventType::Success.color(), Color::GREEN);
    assert_eq!(EventType::Error.color(), Color::RED);
    assert_eq!(EventType::Warning.color(), Color::YELLOW);
}

#[test]
fn test_timeline_event_warning() {
    let event = timeline_event("Warning").warning();
    assert_eq!(event.event_type, EventType::Warning);
}

#[test]
fn test_timeline_event_error() {
    let event = timeline_event("Error").error();
    assert_eq!(event.event_type, EventType::Error);
}

#[test]
fn test_timeline_event_color() {
    let event = timeline_event("Test").color(Color::MAGENTA);
    assert_eq!(event.color, Some(Color::MAGENTA));
}

#[test]
fn test_display_color_override() {
    let event = timeline_event("Test").success().color(Color::MAGENTA);
    assert_eq!(event.display_color(), Color::MAGENTA);
}

#[test]
fn test_display_color_default() {
    let event = timeline_event("Test").error();
    assert_eq!(event.display_color(), Color::RED);
}

#[test]
fn test_timeline_event_meta() {
    let event = timeline_event("Test")
        .meta("key1", "value1")
        .meta("key2", "value2");

    assert_eq!(event.metadata.len(), 2);
    assert_eq!(event.metadata[0].0, "key1");
    assert_eq!(event.metadata[0].1, "value1");
}

#[test]
fn test_timeline_event_meta_single() {
    let event = timeline_event("Test").meta("status", "pending");
    assert_eq!(event.metadata.len(), 1);
}

#[test]
fn test_timeline_events_multiple() {
    let events = vec![
        timeline_event("A"),
        timeline_event("B"),
        timeline_event("C"),
    ];
    let tl = timeline().events(events);
    assert_eq!(tl.len(), 3);
}

#[test]
fn test_timeline_events_empty() {
    let tl = timeline().events(vec![]);
    assert!(tl.is_empty());
}

#[test]
fn test_timeline_vertical() {
    let tl = timeline().vertical();
    // Can't access orientation field directly, but we can verify it was created
    assert_eq!(tl.len(), 0);
}

#[test]
fn test_timeline_horizontal() {
    let tl = timeline().horizontal();
    // Can't access orientation field directly, but we can verify it was created
    assert_eq!(tl.len(), 0);
}

#[test]
fn test_timeline_styles() {
    // Test that different styles can be applied
    let _tl1 = timeline().style(TimelineStyle::Boxed);
    let _tl2 = timeline().style(TimelineStyle::Minimal);
    let _tl3 = timeline().style(TimelineStyle::Alternating);
    let _tl4 = timeline().style(TimelineStyle::Line);
}

#[test]
fn test_timeline_timestamps() {
    let tl_hide = timeline().timestamps(false);
    let tl_show = timeline().timestamps(true);
    // Can't access show_timestamps field directly, but we can verify they were created
    assert_eq!(tl_hide.len(), 0);
    assert_eq!(tl_show.len(), 0);
}

#[test]
fn test_timeline_descriptions() {
    let tl_hide = timeline().descriptions(false);
    let tl_show = timeline().descriptions(true);
    // Can't access show_descriptions field directly, but we can verify they were created
    assert_eq!(tl_hide.len(), 0);
    assert_eq!(tl_show.len(), 0);
}

#[test]
fn test_timeline_line_color() {
    let tl = timeline().line_color(Color::MAGENTA);
    // Can't access line_color field directly, but we can verify it was created
    assert_eq!(tl.len(), 0);
}

#[test]
fn test_select_none() {
    let mut tl = timeline().event(timeline_event("A"));
    tl.select(Some(0));
    tl.select(None);
    let event = tl.selected_event();
    assert!(event.is_none());
}

#[test]
fn test_select_out_of_bounds() {
    let mut tl = timeline().event(timeline_event("A"));
    tl.select(Some(10));
    // selected_event should return None for out of bounds
    let event = tl.selected_event();
    assert!(event.is_none());
}

#[test]
fn test_selected_event() {
    let mut tl = timeline()
        .event(timeline_event("First"))
        .event(timeline_event("Second"));

    tl.select(Some(1));
    let event = tl.selected_event();
    assert!(event.is_some());
    assert_eq!(event.unwrap().title, "Second");
}

#[test]
fn test_selected_event_none_when_no_selection() {
    let tl = timeline().event(timeline_event("A"));
    let event = tl.selected_event();
    assert!(event.is_none());
}

#[test]
fn test_selected_event_empty_timeline() {
    let tl = timeline();
    let event = tl.selected_event();
    assert!(event.is_none());
}

#[test]
fn test_push() {
    let mut tl = timeline();
    assert!(tl.is_empty());

    tl.push(timeline_event("Dynamic"));
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_push_multiple() {
    let mut tl = timeline();
    tl.push(timeline_event("1"));
    tl.push(timeline_event("2"));
    tl.push(timeline_event("3"));
    assert_eq!(tl.len(), 3);
}

#[test]
fn test_event_type_icon_info() {
    assert_eq!(EventType::Info.icon(), '●');
}

#[test]
fn test_event_type_icon_success() {
    assert_eq!(EventType::Success.icon(), '✓');
}

#[test]
fn test_event_type_icon_warning() {
    assert_eq!(EventType::Warning.icon(), '⚠');
}

#[test]
fn test_event_type_icon_error() {
    assert_eq!(EventType::Error.icon(), '✗');
}

#[test]
fn test_event_type_icon_custom() {
    assert_eq!(EventType::Custom('★').icon(), '★');
}

#[test]
fn test_event_type_color_info() {
    assert_eq!(EventType::Info.color(), Color::CYAN);
}

#[test]
fn test_event_type_color_custom() {
    assert_eq!(EventType::Custom('X').color(), Color::WHITE);
}

#[test]
fn test_timeline_orientation_default() {
    assert_eq!(
        TimelineOrientation::default(),
        TimelineOrientation::Vertical
    );
}

#[test]
fn test_timeline_orientation_clone() {
    let orient = TimelineOrientation::Horizontal;
    let cloned = orient;
    assert_eq!(orient, cloned);
}

#[test]
fn test_timeline_orientation_copy() {
    let o1 = TimelineOrientation::Horizontal;
    let o2 = o1;
    assert_eq!(o1, TimelineOrientation::Horizontal);
    assert_eq!(o2, TimelineOrientation::Horizontal);
}

#[test]
fn test_timeline_style_default() {
    assert_eq!(TimelineStyle::default(), TimelineStyle::Line);
}

#[test]
fn test_timeline_style_clone() {
    let style = TimelineStyle::Boxed;
    let cloned = style;
    assert_eq!(style, cloned);
}

#[test]
fn test_timeline_style_copy() {
    let s1 = TimelineStyle::Minimal;
    let s2 = s1;
    assert_eq!(s1, TimelineStyle::Minimal);
    assert_eq!(s2, TimelineStyle::Minimal);
}

#[test]
fn test_timeline_helper() {
    let tl = timeline();
    assert!(tl.is_empty());
}

#[test]
fn test_timeline_event_helper() {
    let event = timeline_event("Test Event");
    assert_eq!(event.title, "Test Event");
}

#[test]
fn test_timeline_event_helper_with_string() {
    let event = timeline_event("Event".to_string());
    assert_eq!(event.title, "Event");
}

#[test]
fn test_timeline_default() {
    let tl = timeline();
    assert!(tl.is_empty());
}

#[test]
fn test_timeline_event_no_default() {
    // TimelineEvent doesn't implement Default
    // Just verify we can create one with new()
    let event = timeline_event("Test");
    assert_eq!(event.title, "Test");
}

#[test]
fn test_select_next_empty() {
    let mut tl = timeline();
    tl.select_next(); // Should do nothing
    let event = tl.selected_event();
    assert!(event.is_none());
}

#[test]
fn test_select_next_at_end() {
    let mut tl = timeline()
        .event(timeline_event("A"))
        .event(timeline_event("B"));
    tl.select(Some(1));
    tl.select_next(); // Should stay at end
    let event = tl.selected_event();
    assert!(event.is_some());
    assert_eq!(event.unwrap().title, "B");
}

#[test]
fn test_select_prev_from_start() {
    let mut tl = timeline()
        .event(timeline_event("A"))
        .event(timeline_event("B"));
    tl.select(Some(0));
    tl.select_prev(); // Should stay at start
    let event = tl.selected_event();
    assert!(event.is_some());
    assert_eq!(event.unwrap().title, "A");
}

#[test]
fn test_select_prev_none() {
    let mut tl = timeline().event(timeline_event("A"));
    tl.select_prev(); // Should do nothing
    let event = tl.selected_event();
    assert!(event.is_none());
}

#[test]
fn test_timeline_builder_chain() {
    let tl = timeline()
        .vertical()
        .style(TimelineStyle::Alternating)
        .timestamps(true)
        .descriptions(false)
        .line_color(Color::CYAN);

    // Verify timeline was created successfully
    assert_eq!(tl.len(), 0);
}

#[test]
fn test_event_builder_chain() {
    let event = timeline_event("Title")
        .description("Description")
        .timestamp("10:30 AM")
        .warning()
        .color(Color::YELLOW);

    assert_eq!(event.title, "Title");
    assert_eq!(event.description, Some("Description".to_string()));
    assert_eq!(event.timestamp, Some("10:30 AM".to_string()));
    assert_eq!(event.event_type, EventType::Warning);
    assert_eq!(event.color, Some(Color::YELLOW));
}

#[test]
fn test_event_type_clone() {
    let et = EventType::Warning;
    let cloned = et;
    assert_eq!(et, cloned);
}

#[test]
fn test_timeline_event_clone() {
    let event = timeline_event("Test").description("Desc").timestamp("Now");

    let cloned = event.clone();
    assert_eq!(cloned.title, "Test");
    assert_eq!(cloned.description, event.description);
}

#[test]
fn test_event_type_debug() {
    let debug_str = format!("{:?}", EventType::Success);
    assert!(debug_str.contains("Success"));
}

#[test]
fn test_event_type_custom_debug() {
    let debug_str = format!("{:?}", EventType::Custom('★'));
    assert!(debug_str.contains("Custom"));
}
