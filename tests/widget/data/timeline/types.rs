//! TimelineEvent, EventType, TimelineStyle and TimelineOrientation tests: builders, icons, colors, equality, metadata and clones

use revue::style::Color;
use revue::widget::{
    timeline_event, EventType, Timeline, TimelineEvent, TimelineOrientation, TimelineStyle,
};

#[test]
fn test_timeline_event_new() {
    let event = TimelineEvent::new("Test Event");
    assert_eq!(event.title, "Test Event");
    assert_eq!(event.description, None);
    assert_eq!(event.timestamp, None);
    assert_eq!(event.event_type, EventType::Info);
    assert_eq!(event.color, None);
    assert!(event.metadata.is_empty());
}

#[test]
fn test_timeline_event_helper() {
    let event = timeline_event("Test Event");
    assert_eq!(event.title, "Test Event");
}

#[test]
fn test_timeline_event_builder_description() {
    let event = TimelineEvent::new("Event").description("This is a description");
    assert_eq!(event.description, Some("This is a description".to_string()));
}

#[test]
fn test_timeline_event_builder_timestamp() {
    let event = TimelineEvent::new("Event").timestamp("10:30 AM");
    assert_eq!(event.timestamp, Some("10:30 AM".to_string()));
}

#[test]
fn test_timeline_event_builder_event_type() {
    let event = TimelineEvent::new("Event").event_type(EventType::Success);
    assert_eq!(event.event_type, EventType::Success);
}

#[test]
fn test_timeline_event_builder_success() {
    let event = TimelineEvent::new("Event").success();
    assert_eq!(event.event_type, EventType::Success);
}

#[test]
fn test_timeline_event_builder_warning() {
    let event = TimelineEvent::new("Event").warning();
    assert_eq!(event.event_type, EventType::Warning);
}

#[test]
fn test_timeline_event_builder_error() {
    let event = TimelineEvent::new("Event").error();
    assert_eq!(event.event_type, EventType::Error);
}

#[test]
fn test_timeline_event_builder_color() {
    let event = TimelineEvent::new("Event").color(Color::MAGENTA);
    assert_eq!(event.color, Some(Color::MAGENTA));
}

#[test]
fn test_timeline_event_builder_meta() {
    let event = TimelineEvent::new("Event")
        .meta("key1", "value1")
        .meta("key2", "value2");
    assert_eq!(event.metadata.len(), 2);
    assert_eq!(
        event.metadata[0],
        ("key1".to_string(), "value1".to_string())
    );
    assert_eq!(
        event.metadata[1],
        ("key2".to_string(), "value2".to_string())
    );
}

#[test]
fn test_timeline_event_display_color_default() {
    let event = TimelineEvent::new("Event");
    assert_eq!(event.display_color(), EventType::Info.color());
}

#[test]
fn test_timeline_event_display_color_custom() {
    let event = TimelineEvent::new("Event").color(Color::MAGENTA);
    assert_eq!(event.display_color(), Color::MAGENTA);
}

#[test]
fn test_timeline_event_builder_chain() {
    let event = TimelineEvent::new("Complex Event")
        .description("With description")
        .timestamp("2024-01-15 10:30")
        .success()
        .color(Color::CYAN)
        .meta("user", "alice")
        .meta("action", "login");

    assert_eq!(event.title, "Complex Event");
    assert_eq!(event.description, Some("With description".to_string()));
    assert_eq!(event.timestamp, Some("2024-01-15 10:30".to_string()));
    assert_eq!(event.event_type, EventType::Success);
    assert_eq!(event.color, Some(Color::CYAN));
    assert_eq!(event.metadata.len(), 2);
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
fn test_event_type_color_success() {
    assert_eq!(EventType::Success.color(), Color::GREEN);
}

#[test]
fn test_event_type_color_warning() {
    assert_eq!(EventType::Warning.color(), Color::YELLOW);
}

#[test]
fn test_event_type_color_error() {
    assert_eq!(EventType::Error.color(), Color::RED);
}

#[test]
fn test_event_type_color_custom() {
    assert_eq!(EventType::Custom('★').color(), Color::WHITE);
}

#[test]
fn test_timeline_event_metadata_empty() {
    let event = TimelineEvent::new("Event");
    assert!(event.metadata.is_empty());
}

#[test]
fn test_timeline_event_metadata_single_key() {
    let event = TimelineEvent::new("Event").meta("id", "123");
    assert_eq!(event.metadata.len(), 1);
    assert_eq!(event.metadata[0], ("id".to_string(), "123".to_string()));
}

#[test]
fn test_timeline_event_metadata_multiple_keys() {
    let event = TimelineEvent::new("Event")
        .meta("key1", "value1")
        .meta("key2", "value2")
        .meta("key3", "value3");
    assert_eq!(event.metadata.len(), 3);
}

#[test]
fn test_timeline_event_metadata_preserves_order() {
    let event = TimelineEvent::new("Event")
        .meta("first", "1")
        .meta("second", "2")
        .meta("third", "3");
    assert_eq!(event.metadata[0].0, "first");
    assert_eq!(event.metadata[1].0, "second");
    assert_eq!(event.metadata[2].0, "third");
}

#[test]
fn test_timeline_event_metadata_with_special_chars() {
    let event = TimelineEvent::new("Event")
        .meta("key with spaces", "value with spaces")
        .meta("key-with-dashes", "value-with-dashes")
        .meta("key_with_underscores", "value_with_underscores");
    assert_eq!(event.metadata.len(), 3);
}

#[test]
fn test_timeline_event_metadata_unicode_values() {
    let event = TimelineEvent::new("Event")
        .meta("emoji", "🎉")
        .meta("korean", "한글")
        .meta("japanese", "日本語");
    assert_eq!(event.metadata.len(), 3);
    assert_eq!(event.metadata[0].1, "🎉");
    assert_eq!(event.metadata[1].1, "한글");
    assert_eq!(event.metadata[2].1, "日本語");
}

#[test]
fn test_timeline_event_clone_preserves_all_fields() {
    let event = TimelineEvent::new("Title")
        .description("Description")
        .timestamp("10:30")
        .success()
        .color(Color::CYAN)
        .meta("key", "value");

    let cloned = event.clone();

    assert_eq!(cloned.title, event.title);
    assert_eq!(cloned.description, event.description);
    assert_eq!(cloned.timestamp, event.timestamp);
    assert_eq!(cloned.event_type, event.event_type);
    assert_eq!(cloned.color, event.color);
    assert_eq!(cloned.metadata.len(), event.metadata.len());
}

#[test]
fn test_timeline_event_clone_independent() {
    let event1 = TimelineEvent::new("Event").meta("key", "value1");
    let event2 = event1.clone().meta("key", "value2");

    // Cloning creates independent instances
    assert_eq!(event1.metadata[0].1, "value1");
    assert_eq!(event2.metadata[1].1, "value2");
}

#[test]
fn test_timeline_unicode_title() {
    let event = TimelineEvent::new("이벤트 제목 🎉");
    assert_eq!(event.title, "이벤트 제목 🎉");
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_unicode_description() {
    let event = TimelineEvent::new("Event").description("日本語の説明 🌸");
    assert_eq!(event.description, Some("日本語の説明 🌸".to_string()));
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_unicode_timestamp() {
    let event = TimelineEvent::new("Event").timestamp("2024년 1월 15일");
    assert_eq!(event.timestamp, Some("2024년 1월 15일".to_string()));
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_multiple_emoji_in_title() {
    let event = TimelineEvent::new("🎉🎊🎈 Celebration Time");
    assert_eq!(event.title, "🎉🎊🎈 Celebration Time");
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_empty_title() {
    let event = TimelineEvent::new("");
    assert_eq!(event.title, "");
}

#[test]
fn test_timeline_empty_description() {
    let event = TimelineEvent::new("Event").description("");
    assert_eq!(event.description, Some("".to_string()));
}

#[test]
fn test_timeline_empty_timestamp() {
    let event = TimelineEvent::new("Event").timestamp("");
    assert_eq!(event.timestamp, Some("".to_string()));
}

#[test]
fn test_timeline_empty_metadata_key() {
    let event = TimelineEvent::new("Event").meta("", "value");
    assert_eq!(event.metadata[0].0, "");
}

#[test]
fn test_timeline_empty_metadata_value() {
    let event = TimelineEvent::new("Event").meta("key", "");
    assert_eq!(event.metadata[0].1, "");
}

#[test]
fn test_event_type_info_equality() {
    assert_eq!(EventType::Info, EventType::Info);
}

#[test]
fn test_event_type_success_equality() {
    assert_eq!(EventType::Success, EventType::Success);
}

#[test]
fn test_event_type_warning_equality() {
    assert_eq!(EventType::Warning, EventType::Warning);
}

#[test]
fn test_event_type_error_equality() {
    assert_eq!(EventType::Error, EventType::Error);
}

#[test]
fn test_event_type_custom_equality() {
    assert_eq!(EventType::Custom('★'), EventType::Custom('★'));
}

#[test]
fn test_event_type_custom_inequality() {
    assert_ne!(EventType::Custom('★'), EventType::Custom('♦'));
}

#[test]
fn test_event_type_different_types_not_equal() {
    assert_ne!(EventType::Info, EventType::Success);
    assert_ne!(EventType::Warning, EventType::Error);
}

#[test]
fn test_timeline_style_line_equality() {
    assert_eq!(TimelineStyle::Line, TimelineStyle::Line);
}

#[test]
fn test_timeline_style_boxed_equality() {
    assert_eq!(TimelineStyle::Boxed, TimelineStyle::Boxed);
}

#[test]
fn test_timeline_style_minimal_equality() {
    assert_eq!(TimelineStyle::Minimal, TimelineStyle::Minimal);
}

#[test]
fn test_timeline_style_alternating_equality() {
    assert_eq!(TimelineStyle::Alternating, TimelineStyle::Alternating);
}

#[test]
fn test_timeline_style_different_not_equal() {
    assert_ne!(TimelineStyle::Line, TimelineStyle::Boxed);
    assert_ne!(TimelineStyle::Minimal, TimelineStyle::Alternating);
}

#[test]
fn test_timeline_orientation_vertical_equality() {
    assert_eq!(TimelineOrientation::Vertical, TimelineOrientation::Vertical);
}

#[test]
fn test_timeline_orientation_horizontal_equality() {
    assert_eq!(
        TimelineOrientation::Horizontal,
        TimelineOrientation::Horizontal
    );
}

#[test]
fn test_timeline_orientation_different_not_equal() {
    assert_ne!(
        TimelineOrientation::Vertical,
        TimelineOrientation::Horizontal
    );
}

#[test]
fn test_timeline_event_color_priority_over_type() {
    let event = TimelineEvent::new("Event")
        .error() // Red
        .color(Color::BLUE); // Override to blue

    assert_eq!(event.display_color(), Color::BLUE);
}

#[test]
fn test_timeline_event_default_color_from_type() {
    let event = TimelineEvent::new("Event").warning(); // Yellow
    assert_eq!(event.display_color(), Color::YELLOW);
}

#[test]
fn test_timeline_multiple_events_different_colors() {
    // Verify display_color on individual events before adding to timeline
    let event0 = TimelineEvent::new("Info").color(Color::CYAN);
    let event1 = TimelineEvent::new("Warning").color(Color::YELLOW);
    let event2 = TimelineEvent::new("Error").color(Color::RED);
    assert_eq!(event0.display_color(), Color::CYAN);
    assert_eq!(event1.display_color(), Color::YELLOW);
    assert_eq!(event2.display_color(), Color::RED);

    let tl = Timeline::new().event(event0).event(event1).event(event2);
    assert_eq!(tl.len(), 3);
}

#[test]
fn test_timeline_many_events() {
    let mut tl = Timeline::new();
    for i in 0..100 {
        tl = tl.event(TimelineEvent::new(format!("Event {}", i)));
    }
    assert_eq!(tl.len(), 100);
}

#[test]
fn test_timeline_title_with_tabs() {
    let event = TimelineEvent::new("Event\twith\ttabs");
    assert_eq!(event.title, "Event\twith\ttabs");
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_title_with_special_symbols() {
    let event = TimelineEvent::new("♠♥♦♣ ★☆☀☁");
    assert_eq!(event.title, "♠♥♦♣ ★☆☀☁");
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_custom_event_number() {
    let _tl = Timeline::new().event(TimelineEvent::new("Event").event_type(EventType::Custom('1')));
    assert_eq!(EventType::Custom('1').icon(), '1');
}
