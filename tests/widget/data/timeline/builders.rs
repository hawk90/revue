//! Timeline builder tests: construction, each setting checked on the screen, events added and cleared, CSS ids and classes

use super::{blank, cut, draw, screen, sym, three};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::traits::StyledView;
use revue::widget::View;
use revue::widget::{timeline, Timeline, TimelineEvent, TimelineOrientation, TimelineStyle};

#[test]
fn test_timeline_new() {
    let tl = Timeline::new();
    assert!(tl.is_empty());
    assert_eq!(tl.len(), 0);
    assert!(tl.selected_event().is_none());
}

#[test]
fn test_timeline_default() {
    let tl = Timeline::default();
    assert!(tl.is_empty());
    assert_eq!(tl.len(), 0);
}

#[test]
fn test_timeline_helper() {
    let tl = timeline();
    assert!(tl.is_empty());
    assert_eq!(tl.len(), 0);
}

#[test]
fn test_timeline_event_single() {
    let tl = Timeline::new().event(TimelineEvent::new("Event 1"));
    assert_eq!(tl.len(), 1);
    assert!(!tl.is_empty());
}

#[test]
fn test_timeline_events_multiple() {
    let events = vec![
        TimelineEvent::new("Event 1"),
        TimelineEvent::new("Event 2"),
        TimelineEvent::new("Event 3"),
    ];
    let tl = Timeline::new().events(events);
    assert_eq!(tl.len(), 3);
}

#[test]
fn test_timeline_orientation_vertical() {
    let tl = three().orientation(TimelineOrientation::Vertical);
    // Icons stacked in column 12, right of the timestamps
    let rows = screen(&tl, 40, 10);
    let icons: String = rows.iter().map(|r| r.chars().nth(12).unwrap()).collect();
    assert_eq!(icons.trim_end(), "●││●│●");
}

#[test]
fn test_timeline_orientation_horizontal() {
    let tl = three().orientation(TimelineOrientation::Horizontal);
    assert_eq!(screen(&tl, 50, 4), screen(&three().horizontal(), 50, 4));
}

#[test]
fn test_timeline_vertical() {
    let tl = three().horizontal().vertical();
    assert_eq!(screen(&tl, 40, 10), screen(&three(), 40, 10));
}

#[test]
fn test_timeline_horizontal() {
    let tl = three().horizontal();
    // Titles above, the line with an icon every 15 columns, timestamps below
    let rows = screen(&tl, 50, 4);
    assert_eq!(
        rows[0].trim_end(),
        "    Event 1        Event 2        Event 3"
    );
    assert_eq!(
        rows[1],
        "───────●──────────────●──────────────●────────────"
    );
    assert_eq!(rows[2].trim_end(), "     10:00          11:00");
}

#[test]
fn test_timeline_style_line() {
    let tl = three().style(TimelineStyle::Line);
    let buffer = draw(&tl, 40, 10);
    assert_eq!(sym(&buffer, 13, 0), '─'); // connector
    assert_eq!(sym(&buffer, 12, 1), '│'); // line down to the next event
}

#[test]
fn test_timeline_style_boxed() {
    // Boxed draws the line and the connectors (#799: it draws the same as Line)
    let tl = three().style(TimelineStyle::Boxed);
    let buffer = draw(&tl, 40, 10);
    assert_eq!(sym(&buffer, 13, 0), '─');
    assert_eq!(sym(&buffer, 12, 1), '│');
}

#[test]
fn test_timeline_style_minimal() {
    let tl = three().style(TimelineStyle::Minimal);
    let rows = screen(&tl, 40, 10);
    assert!(
        rows.iter().all(|r| !r.contains('│') && !r.contains('─')),
        "{rows:#?}"
    );
    assert_eq!(rows[0].trim_end(), "10:00       ●  Event 1");
}

#[test]
fn test_timeline_style_alternating() {
    // Alternating draws the line and the connectors (#799: it draws the same as Line)
    let tl = three().style(TimelineStyle::Alternating);
    let buffer = draw(&tl, 40, 10);
    assert_eq!(sym(&buffer, 13, 0), '─');
    assert_eq!(sym(&buffer, 12, 1), '│');
}

#[test]
fn test_timeline_timestamps_show() {
    let tl = three().timestamps(true);
    let rows = screen(&tl, 40, 10);
    assert!(rows[0].starts_with("10:00       ●"), "{:?}", rows[0]);
    assert!(rows[3].starts_with("11:00       ●"), "{:?}", rows[3]);
}

#[test]
fn test_timeline_timestamps_hide() {
    let tl = three().timestamps(false);
    let rows = screen(&tl, 40, 10);
    // No timestamp column: the icons move to column 0
    assert_eq!(rows[0].trim_end(), "●─ Event 1");
    assert!(rows.iter().all(|r| !r.contains("10:00")));
}

#[test]
fn test_timeline_descriptions_show() {
    let tl = three().descriptions(true);
    let rows = screen(&tl, 40, 10);
    assert_eq!(rows[1].trim_end(), "            │  first");
}

#[test]
fn test_timeline_descriptions_hide() {
    let tl = three().descriptions(false);
    let rows = screen(&tl, 40, 10);
    assert!(rows.iter().all(|r| !r.contains("first")), "{rows:#?}");
    // The next event moves up into the description's row
    assert!(rows[2].starts_with("11:00       ●"), "{rows:#?}");
}

#[test]
fn test_timeline_line_color() {
    let tl = three().line_color(Color::MAGENTA);
    let buffer = draw(&tl, 40, 10);
    assert_eq!(buffer.get(13, 0).unwrap().fg, Some(Color::MAGENTA)); // connector
    assert_eq!(buffer.get(12, 1).unwrap().fg, Some(Color::MAGENTA)); // line
}

#[test]
fn test_timeline_builder_chain() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .vertical()
        .style(TimelineStyle::Line)
        .timestamps(true)
        .descriptions(true)
        .line_color(Color::CYAN);

    assert_eq!(tl.len(), 2);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);
}

#[test]
fn test_timeline_push_empty() {
    let mut tl = Timeline::new();
    tl.push(TimelineEvent::new("Event 1"));
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_push_multiple() {
    let mut tl = Timeline::new();
    tl.push(TimelineEvent::new("Event 1"));
    tl.push(TimelineEvent::new("Event 2"));
    tl.push(TimelineEvent::new("Event 3"));
    assert_eq!(tl.len(), 3);
}

#[test]
fn test_timeline_clear() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));
    assert_eq!(tl.len(), 2);

    tl.clear();
    assert!(tl.is_empty());
    assert_eq!(tl.len(), 0);
    assert!(tl.selected_event().is_none());
}

#[test]
fn test_timeline_clear_with_selection() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));
    tl.select(Some(1));
    assert_eq!(tl.selected_event().unwrap().title, "Event 2");

    tl.clear();
    assert!(tl.is_empty());
    assert!(tl.selected_event().is_none());
}

#[test]
fn test_timeline_css_id() {
    let tl = timeline().element_id("my-timeline");
    assert_eq!(View::id(&tl), Some("my-timeline"));

    let meta = tl.meta();
    assert_eq!(meta.id, Some("my-timeline".to_string()));
}

#[test]
fn test_timeline_css_classes() {
    let tl = timeline().class("activity").class("recent");

    assert!(tl.has_class("activity"));
    assert!(tl.has_class("recent"));
    assert!(!tl.has_class("old"));

    let meta = tl.meta();
    assert!(meta.classes.contains("activity"));
    assert!(meta.classes.contains("recent"));
}

#[test]
fn test_timeline_styled_view_set_id() {
    let mut tl = timeline();
    tl.set_id("test-id");
    assert_eq!(View::id(&tl), Some("test-id"));
}

#[test]
fn test_timeline_styled_view_add_class() {
    let mut tl = timeline();
    tl.add_class("active");
    assert!(tl.has_class("active"));
}

#[test]
fn test_timeline_styled_view_remove_class() {
    let mut tl = timeline().class("active");
    tl.remove_class("active");
    assert!(!tl.has_class("active"));
}

#[test]
fn test_timeline_styled_view_toggle_class() {
    let mut tl = timeline();

    tl.toggle_class("selected");
    assert!(tl.has_class("selected"));

    tl.toggle_class("selected");
    assert!(!tl.has_class("selected"));
}

#[test]
fn test_timeline_classes_builder() {
    let tl = timeline().classes(vec!["class1", "class2", "class3"]);

    assert!(tl.has_class("class1"));
    assert!(tl.has_class("class2"));
    assert!(tl.has_class("class3"));
    assert_eq!(View::classes(&tl).len(), 3);
}

#[test]
fn test_timeline_duplicate_class_not_added() {
    let tl = timeline().class("test").class("test");

    let classes = View::classes(&tl);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_timeline_meta() {
    let tl = timeline()
        .element_id("test-timeline")
        .class("activity")
        .class("recent");

    let meta = tl.meta();
    assert_eq!(meta.widget_type, "Timeline");
    assert_eq!(meta.id, Some("test-timeline".to_string()));
    assert!(meta.classes.contains("activity"));
    assert!(meta.classes.contains("recent"));
}

#[test]
fn test_timeline_state_after_clear() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .line_color(Color::MAGENTA)
        .timestamps(false)
        .descriptions(false);

    tl.select(Some(1));
    assert_eq!(tl.selected_event().unwrap().title, "Event 2");

    tl.clear();

    assert!(tl.is_empty());
    assert!(tl.selected_event().is_none());
    // Configuration should be preserved - verify via render (no panic)
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);
}

#[test]
fn test_timeline_orientation_preserved_after_clear() {
    let mut tl = Timeline::new()
        .horizontal()
        .event(TimelineEvent::new("Event"));

    tl.clear();
    assert!(blank(&draw(&tl, 40, 5)));
    tl.push(TimelineEvent::new("Again"));
    assert!(screen(&tl, 40, 5)[1].starts_with("───────●"));
}

#[test]
fn test_timeline_style_preserved_after_clear() {
    let mut tl = Timeline::new()
        .style(TimelineStyle::Minimal)
        .event(TimelineEvent::new("Event"));

    tl.clear();
    tl.push(TimelineEvent::new("Again"));
    tl.push(TimelineEvent::new("More"));
    let rows = screen(&tl, 40, 10);
    assert_eq!(rows[0].trim(), "●  Again");
    assert!(rows.iter().all(|r| !r.contains('│')));
}

#[test]
fn test_timeline_selection_cleared_on_clear() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"));

    tl.select(Some(1));
    assert!(tl.selected_event().is_some());

    tl.clear();
    assert!(tl.selected_event().is_none());
}

#[test]
fn test_timeline_multiple_same_orientation_calls() {
    // Last orientation call wins - verify by rendering
    let tl = Timeline::new()
        .vertical()
        .horizontal()
        .vertical()
        .event(TimelineEvent::new("Event 1").timestamp("10:00"))
        .event(TimelineEvent::new("Event 2").timestamp("11:00"));

    // Vertical orientation renders line characters between events
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);
    let cell = buffer.get(12, area.y + 1).unwrap();
    assert_eq!(cell.symbol, '│');
}

#[test]
fn test_timeline_multiple_same_style_calls() {
    // Last style call wins - verify Minimal has no line characters at icon column
    // timestamps(false) so icon_x = 0, then line would be at (0, 1) for non-Minimal styles
    let tl = Timeline::new()
        .timestamps(false)
        .style(TimelineStyle::Line)
        .style(TimelineStyle::Boxed)
        .style(TimelineStyle::Minimal)
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);
    // Minimal style should not render line characters between events
    let cell = buffer.get(0, area.y + 1).unwrap();
    assert_ne!(cell.symbol, '│');
}

#[test]
fn test_timeline_multiple_timestamps_calls() {
    // Last timestamps call wins - verify timestamps(true) shows timestamp in render
    let tl = Timeline::new()
        .timestamps(true)
        .timestamps(false)
        .timestamps(true)
        .event(TimelineEvent::new("Event").timestamp("10:00"));

    // timestamps(true) means icon is at x=12 (after timestamp width)
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);
    let icon_cell = buffer.get(12, area.y).unwrap();
    assert_eq!(icon_cell.symbol, '●');
}

#[test]
fn test_timeline_multiple_descriptions_calls() {
    // Last descriptions call wins
    let tl = Timeline::new()
        .descriptions(true)
        .descriptions(false)
        .descriptions(true)
        .event(
            TimelineEvent::new("Event")
                .description("Visible desc")
                .timestamp("10:00"),
        );

    assert_eq!(cut(&screen(&tl, 40, 10)[1], 15, 27), "Visible desc");
}

#[test]
fn test_timeline_multiple_color_calls() {
    // Last line_color call wins - verify via render (no panic with the final color)
    let tl = Timeline::new()
        .line_color(Color::RED)
        .line_color(Color::GREEN)
        .line_color(Color::BLUE)
        .event(TimelineEvent::new("Event 1").timestamp("10:00"))
        .event(TimelineEvent::new("Event 2").timestamp("11:00"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);
    // Line character should have blue color (the last set)
    let line_cell = buffer.get(12, area.y + 1).unwrap();
    assert_eq!(line_cell.fg, Some(Color::BLUE));
}

#[test]
fn test_timeline_meta_widget_type() {
    let tl = timeline();
    let meta = tl.meta();
    assert_eq!(meta.widget_type, "Timeline");
}

#[test]
fn test_timeline_view_id_none() {
    let tl = timeline();
    assert_eq!(View::id(&tl), None);
}

#[test]
fn test_timeline_view_id_some() {
    let tl = timeline().element_id("my-timeline");
    assert_eq!(View::id(&tl), Some("my-timeline"));
}

#[test]
fn test_timeline_view_classes_empty() {
    let tl = timeline();
    assert!(View::classes(&tl).is_empty());
}

#[test]
fn test_timeline_view_classes_multiple() {
    let tl = timeline().classes(vec!["a", "b"]);
    let classes = View::classes(&tl);
    assert_eq!(classes.len(), 2);
}
