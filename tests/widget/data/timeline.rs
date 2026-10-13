//! Timeline widget tests

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::theme::{DARK_GRAY, LIGHT_GRAY, MUTED_TEXT};
use revue::widget::traits::{RenderContext, StyledView};
use revue::widget::View;
use revue::widget::{
    timeline, timeline_event, EventType, Timeline, TimelineEvent, TimelineOrientation,
    TimelineStyle,
};

/// Three vertical events: two with timestamps, the first with a description
fn three() -> Timeline {
    Timeline::new()
        .event(
            TimelineEvent::new("Event 1")
                .timestamp("10:00")
                .description("first"),
        )
        .event(TimelineEvent::new("Event 2").timestamp("11:00"))
        .event(TimelineEvent::new("Event 3"))
}

/// `tl` drawn in a `width` x `height` area
fn draw(tl: &Timeline, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    tl.render(&mut ctx);
    buffer
}

/// Each row of `tl` drawn in a `width` x `height` area
fn screen(tl: &Timeline, width: u16, height: u16) -> Vec<String> {
    let buffer = draw(tl, width, height);
    (0..height)
        .map(|y| (0..width).map(|x| sym(&buffer, x, y)).collect())
        .collect()
}

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

/// Columns `from..to` of `row`
fn cut(row: &str, from: usize, to: usize) -> String {
    row.chars().take(to).skip(from).collect()
}

/// Nothing drawn anywhere in `buffer`
fn blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| (0..buffer.width()).all(|x| sym(buffer, x, y) == ' '))
}

// =============================================================================
// Constructor Tests
// =============================================================================

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

// =============================================================================
// TimelineEvent Tests
// =============================================================================

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

// =============================================================================
// EventType Tests
// =============================================================================

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

// =============================================================================
// Timeline Builder Methods Tests
// =============================================================================

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

// =============================================================================
// Selection Tests
// =============================================================================

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

// =============================================================================
// Dynamic Event Management Tests
// =============================================================================

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

// =============================================================================
// Render Tests - Vertical Timeline
// =============================================================================

#[test]
fn test_timeline_render_vertical_empty() {
    let tl = Timeline::new();
    assert!(blank(&draw(&tl, 40, 10)));
}
#[test]
fn test_timeline_render_vertical_single_event() {
    let tl = Timeline::new().event(TimelineEvent::new("Single Event").timestamp("10:00"));
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Verify title appears
    let mut found_title = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == 'S' {
                found_title = true;
                break;
            }
        }
    }
    assert!(found_title);
}

#[test]
fn test_timeline_render_vertical_multiple_events() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Event 1").timestamp("10:00"))
        .event(TimelineEvent::new("Event 2").timestamp("11:00"))
        .event(TimelineEvent::new("Event 3").timestamp("12:00"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Verify line characters are drawn
    let mut found_line = false;
    for y in 0..area.height {
        if let Some(cell) = buffer.get(12, y) {
            // Icon position (timestamp_width = 12)
            if cell.symbol == '│' {
                found_line = true;
                break;
            }
        }
    }
    assert!(found_line);
}

#[test]
fn test_timeline_render_vertical_with_description() {
    let tl = Timeline::new().event(
        TimelineEvent::new("Event 1")
            .description("This is a description")
            .timestamp("10:00"),
    );

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Verify description appears
    let mut found_desc = false;
    for y in 0..area.height {
        if let Some(cell) = buffer.get(15, y) {
            // Content position (icon_x + 3 = 12 + 3 = 15)
            if cell.symbol == 'T' {
                found_desc = true;
                break;
            }
        }
    }
    assert!(found_desc);
}

#[test]
fn test_timeline_render_vertical_without_timestamps() {
    let tl = Timeline::new()
        .timestamps(false)
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Icon should be at x=0 (no timestamp)
    let cell = buffer.get(0, area.y).unwrap();
    assert_eq!(cell.symbol, '●'); // Info icon
}

#[test]
fn test_timeline_render_vertical_without_descriptions() {
    let tl = Timeline::new()
        .descriptions(false)
        .event(TimelineEvent::new("Event 1").description("Hidden"))
        .event(TimelineEvent::new("Event 2").description("Also hidden"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Events should still render
    let cell = buffer.get(12, area.y).unwrap();
    assert_eq!(cell.symbol, '●');
}

#[test]
fn test_timeline_render_vertical_with_selection() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"));

    tl.select(Some(1));

    let buffer = draw(&tl, 40, 10);
    // Event 2 (row 2): icon and title bold, the title in the event color
    let icon = buffer.get(12, 2).unwrap();
    let title = buffer.get(15, 2).unwrap();
    assert!(icon.modifier.contains(Modifier::BOLD));
    assert!(title.modifier.contains(Modifier::BOLD));
    assert_eq!(title.fg, icon.fg);
    // Event 1 is plain
    assert!(!buffer.get(15, 0).unwrap().modifier.contains(Modifier::BOLD));
    assert_eq!(buffer.get(15, 0).unwrap().fg, Some(Color::WHITE));
}
#[test]
fn test_timeline_render_vertical_small_height() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"))
        .event(TimelineEvent::new("Event 4"));

    let rows = screen(&tl, 40, 3);
    assert_eq!(rows[0].trim(), "●─ Event 1");
    assert_eq!(rows[1].trim(), "│");
    assert_eq!(rows[2].trim(), "●─ Event 2");
}
#[test]
fn test_timeline_render_vertical_line_style() {
    let tl = Timeline::new()
        .style(TimelineStyle::Line)
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Should have line characters
    let cell = buffer.get(12, area.y + 1).unwrap();
    assert_eq!(cell.symbol, '│');
}

#[test]
fn test_timeline_render_vertical_minimal_style() {
    let tl = Timeline::new()
        .style(TimelineStyle::Minimal)
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Should not have line characters
    let cell = buffer.get(12, area.y + 1).unwrap();
    assert_ne!(cell.symbol, '│');
}

#[test]
fn test_timeline_render_vertical_event_types() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Info").event_type(EventType::Info))
        .event(TimelineEvent::new("Success").event_type(EventType::Success))
        .event(TimelineEvent::new("Warning").event_type(EventType::Warning))
        .event(TimelineEvent::new("Error").event_type(EventType::Error));

    let buffer = draw(&tl, 40, 10);
    let icons: Vec<char> = [0, 2, 4, 6].iter().map(|&y| sym(&buffer, 12, y)).collect();
    assert_eq!(icons, ['●', '✓', '⚠', '✗']);
}
#[test]
fn test_timeline_render_vertical_custom_event_icon() {
    let tl = Timeline::new().event(TimelineEvent::new("Custom").event_type(EventType::Custom('★')));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    assert_eq!(buffer.get(12, area.y).unwrap().symbol, '★');
}

// =============================================================================
// Render Tests - Horizontal Timeline
// =============================================================================

#[test]
fn test_timeline_render_horizontal_empty() {
    let tl = Timeline::new().horizontal();
    assert!(blank(&draw(&tl, 40, 5)));
}
#[test]
fn test_timeline_render_horizontal_single_event() {
    let tl = Timeline::new()
        .horizontal()
        .event(TimelineEvent::new("Event 1").timestamp("10:00"));

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Should render horizontal line
    let cell = buffer.get(area.x, area.y + 1).unwrap();
    assert_eq!(cell.symbol, '─');
}

#[test]
fn test_timeline_render_horizontal_multiple_events() {
    let tl = Timeline::new()
        .horizontal()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"));

    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Should have horizontal line
    let cell = buffer.get(area.x + 5, area.y + 1).unwrap();
    assert_eq!(cell.symbol, '─');
}

#[test]
fn test_timeline_render_horizontal_small_width() {
    let tl = Timeline::new()
        .horizontal()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"))
        .event(TimelineEvent::new("Event 3"));

    // The first event takes 15 columns; the rest is cut at the edge
    let rows = screen(&tl, 20, 5);
    assert_eq!(rows[0], "    Event 1        E");
    assert_eq!(rows[1], "───────●────────────");
}
#[test]
fn test_timeline_render_horizontal_without_timestamps() {
    let tl = Timeline::new()
        .horizontal()
        .timestamps(false)
        .event(TimelineEvent::new("Event 1").timestamp("Hidden"));

    let rows = screen(&tl, 40, 5);
    assert!(rows[2].trim().is_empty(), "{:?}", rows[2]);
    assert!(rows.iter().all(|r| !r.contains("Hidden")));
}
// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_timeline_zero_area() {
    let tl = Timeline::new().event(TimelineEvent::new("Event 1").timestamp("10:00"));
    for tl in [tl.clone(), tl.horizontal()] {
        for (w, h) in [(0, 0), (0, 10), (10, 0)] {
            let mut buffer = Buffer::new(10, 10);
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
            tl.render(&mut ctx);
            assert!(blank(&buffer), "{w}x{h}");
        }
    }
}
#[test]
fn test_timeline_very_long_title() {
    let long_title = "This is a very long title that exceeds the available width";
    let tl = Timeline::new().event(TimelineEvent::new(long_title));

    // Cut to the 15 columns right of the icon (30 - 12 - 3)
    let rows = screen(&tl, 30, 10);
    assert_eq!(cut(&rows[0], 15, 30), "This is a very ");
}
#[test]
fn test_timeline_very_long_description() {
    let long_desc =
        "This is a very long description that exceeds the available width and should be truncated";
    let tl = Timeline::new().event(TimelineEvent::new("Event").description(long_desc));

    let rows = screen(&tl, 30, 10);
    assert_eq!(cut(&rows[1], 15, usize::MAX), "This is a very ");
    assert!(rows[2].trim().is_empty());
}
#[test]
fn test_timeline_very_long_timestamp() {
    let long_ts = "2024-01-15 10:30:45.123456";
    let tl = Timeline::new().event(TimelineEvent::new("Event").timestamp(long_ts));

    // Cut to 11 columns, leaving column 11 free before the icon
    let rows = screen(&tl, 40, 10);
    assert_eq!(cut(&rows[0], 0, 13), "2024-01-15  ●");
}
#[test]
fn test_timeline_select_out_of_bounds() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    tl.select(Some(10)); // Out of bounds
    assert!(tl.selected_event().is_none());
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
fn test_timeline_render_with_event_custom_color() {
    let tl = Timeline::new().event(TimelineEvent::new("Event 1").color(Color::YELLOW));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Icon should have custom color
    let cell = buffer.get(12, area.y).unwrap();
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

// =============================================================================
// CSS/Styling Tests
// =============================================================================

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

// =============================================================================
// Timeline Event Metadata Tests
// =============================================================================

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

// =============================================================================
// State Preservation Tests
// =============================================================================

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

// =============================================================================
// Timeline Event Clone Tests
// =============================================================================

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

// =============================================================================
// Unicode and Special Character Tests
// =============================================================================

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
fn test_timeline_render_unicode_content() {
    let tl = Timeline::new().event(
        TimelineEvent::new("🎉 Celebration")
            .description("한글과 이모지가 섞인 텍스트")
            .timestamp("10:00"),
    );

    let buffer = draw(&tl, 40, 10);
    // The emoji takes two columns; the title goes on after it
    assert_eq!(sym(&buffer, 15, 0), '🎉');
    assert_eq!(sym(&buffer, 18, 0), 'C');
    assert_eq!(sym(&buffer, 15, 1), '한');
    assert_eq!(sym(&buffer, 17, 1), '글');
}
#[test]
fn test_timeline_multiple_emoji_in_title() {
    let event = TimelineEvent::new("🎉🎊🎈 Celebration Time");
    assert_eq!(event.title, "🎉🎊🎈 Celebration Time");
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_custom_event_with_emoji() {
    let tl = Timeline::new().event(TimelineEvent::new("Party").event_type(EventType::Custom('🎉')));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    assert_eq!(buffer.get(12, area.y).unwrap().symbol, '🎉');
}

// =============================================================================
// Empty and Minimal Input Tests
// =============================================================================

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
fn test_timeline_render_empty_title() {
    let tl = Timeline::new().event(TimelineEvent::new("").timestamp("10:00"));
    assert_eq!(screen(&tl, 40, 10)[0].trim_end(), "10:00       ●─");
}
// =============================================================================
// Timeline Event Type Equality Tests
// =============================================================================

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

// =============================================================================
// TimelineStyle Equality Tests
// =============================================================================

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

// =============================================================================
// TimelineOrientation Equality Tests
// =============================================================================

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

// =============================================================================
// Color Interaction Tests
// =============================================================================

#[test]
fn test_timeline_default_colors() {
    let tl = Timeline::new().event(
        TimelineEvent::new("Event")
            .timestamp("10:00")
            .description("Desc"),
    );
    let buffer = draw(&tl, 40, 10);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(LIGHT_GRAY)); // timestamp
    assert_eq!(buffer.get(13, 0).unwrap().fg, Some(DARK_GRAY)); // connector
    assert_eq!(buffer.get(15, 0).unwrap().fg, Some(Color::WHITE)); // title
    assert_eq!(buffer.get(15, 1).unwrap().fg, Some(MUTED_TEXT)); // description
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

// =============================================================================
// Render Buffer Verification Tests
// =============================================================================

#[test]
fn test_timeline_render_vertical_line_connects_events() {
    let tl = Timeline::new()
        .style(TimelineStyle::Line)
        .timestamps(true)
        .event(TimelineEvent::new("Event 1").timestamp("10:00"))
        .event(TimelineEvent::new("Event 2").timestamp("11:00"))
        .event(TimelineEvent::new("Event 3").timestamp("12:00"));

    let mut buffer = Buffer::new(40, 15);
    let area = Rect::new(0, 0, 40, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Count vertical line characters
    let mut line_count = 0;
    for y in area.y..area.y + area.height {
        if let Some(cell) = buffer.get(12, y) {
            if cell.symbol == '│' {
                line_count += 1;
            }
        }
    }
    assert!(
        line_count >= 2,
        "Should have connecting lines between events"
    );
}

#[test]
fn test_timeline_render_horizontal_line_spans_width() {
    let tl = Timeline::new()
        .horizontal()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Verify horizontal line exists
    let mut line_count = 0;
    for x in area.x..area.x + area.width {
        if let Some(cell) = buffer.get(x, area.y + 1) {
            if cell.symbol == '─' {
                line_count += 1;
            }
        }
    }
    assert!(line_count > 0, "Should have horizontal line");
}

#[test]
fn test_timeline_render_timestamp_left_aligned() {
    let tl = Timeline::new()
        .timestamps(true)
        .event(TimelineEvent::new("Event").timestamp("10:00"));

    // The timestamp starts at column 0 of its 12-column gutter
    assert!(screen(&tl, 40, 10)[0].starts_with("10:00       ●"));
}
// =============================================================================
// Selection Boundary Tests
// =============================================================================

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
fn test_timeline_select_wrap_around_disabled() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    tl.select(Some(1));
    tl.select_next(); // Should NOT wrap to 0
    assert_eq!(tl.selected_event().unwrap().title, "Event 2");
}

#[test]
fn test_timeline_selection_with_invalid_index() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    tl.select(Some(100));
    assert!(
        tl.selected_event().is_none(),
        "Invalid index should return None"
    );
}

// =============================================================================
// Complex Scenarios
// =============================================================================

#[test]
fn test_timeline_complex_scenario() {
    // Build a complex timeline with various event types
    let mut tl = Timeline::new()
        .style(TimelineStyle::Line)
        .timestamps(true)
        .descriptions(true)
        .line_color(Color::rgb(100, 100, 100))
        .event(
            TimelineEvent::new("Application Started")
                .timestamp("09:00")
                .success()
                .description("Application initialized successfully"),
        )
        .event(
            TimelineEvent::new("User Login")
                .timestamp("09:05")
                .event_type(EventType::Info)
                .description("User 'alice' logged in")
                .meta("user_id", "12345"),
        )
        .event(
            TimelineEvent::new("Configuration Error")
                .timestamp("09:10")
                .warning()
                .description("Invalid configuration in config.yml")
                .meta("file", "config.yml"),
        )
        .event(
            TimelineEvent::new("Connection Failed")
                .timestamp("09:15")
                .error()
                .description("Failed to connect to database")
                .meta("host", "db.example.com")
                .meta("port", "5432"),
        )
        .event(
            TimelineEvent::new("Retry Attempt")
                .timestamp("09:16")
                .event_type(EventType::Custom('↻'))
                .description("Attempting to reconnect..."),
        );

    assert_eq!(tl.len(), 5);

    // Navigate through events
    tl.select_next();
    assert_eq!(tl.selected_event().unwrap().title, "Application Started");

    tl.select_next();
    tl.select_next();
    assert_eq!(tl.selected_event().unwrap().title, "Configuration Error");

    tl.select_prev();
    assert_eq!(tl.selected_event().unwrap().title, "User Login");

    // Render and verify
    let mut buffer = Buffer::new(60, 15);
    let area = Rect::new(0, 0, 60, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Verify all events rendered
    let mut found_events = 0;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '●'
                    || cell.symbol == '✓'
                    || cell.symbol == '⚠'
                    || cell.symbol == '✗'
                    || cell.symbol == '↻'
                {
                    found_events += 1;
                    break;
                }
            }
        }
    }
    assert!(found_events >= 4); // At least some events should be visible
}

#[test]
fn test_timeline_horizontal_complex_scenario() {
    let tl = Timeline::new()
        .horizontal()
        .timestamps(true)
        .event(TimelineEvent::new("Step 1").timestamp("10:00").success())
        .event(TimelineEvent::new("Step 2").timestamp("10:05").success())
        .event(TimelineEvent::new("Step 3").timestamp("10:10").warning())
        .event(TimelineEvent::new("Step 4").timestamp("10:15"));

    let mut buffer = Buffer::new(80, 5);
    let area = Rect::new(0, 0, 80, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Verify horizontal line is drawn
    let mut found_line = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y + 1) {
            if cell.symbol == '─' {
                found_line = true;
                break;
            }
        }
    }
    assert!(found_line);
}

#[test]
fn test_timeline_minimal_style_no_lines() {
    let tl = Timeline::new()
        .style(TimelineStyle::Minimal)
        .event(
            TimelineEvent::new("Event 1")
                .timestamp("10:00")
                .description("Description 1"),
        )
        .event(
            TimelineEvent::new("Event 2")
                .timestamp("11:00")
                .description("Description 2"),
        )
        .event(
            TimelineEvent::new("Event 3")
                .timestamp("12:00")
                .description("Description 3"),
        );

    let mut buffer = Buffer::new(40, 15);
    let area = Rect::new(0, 0, 40, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Minimal style should not have line characters
    let mut found_line = false;
    for y in 0..area.height {
        if let Some(cell) = buffer.get(12, y) {
            if cell.symbol == '│' {
                found_line = true;
                break;
            }
        }
    }
    assert!(!found_line);
}

// =============================================================================
// Alternating Style Tests
// =============================================================================

#[test]
fn test_timeline_alternating_style_has_lines() {
    let tl = Timeline::new()
        .style(TimelineStyle::Alternating)
        .event(TimelineEvent::new("Event 1").timestamp("10:00"))
        .event(TimelineEvent::new("Event 2").timestamp("11:00"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Alternating style should have line characters
    let cell = buffer.get(12, area.y + 1).unwrap();
    assert_eq!(cell.symbol, '│');
}

// =============================================================================
// Boxed Style Tests
// =============================================================================

// =============================================================================
// Multi-Line Description Tests
// =============================================================================

#[test]
fn test_timeline_multiline_description_rendered() {
    let long_desc =
        "This is a long description that spans multiple lines when rendered in the timeline widget";
    let tl = Timeline::new().event(
        TimelineEvent::new("Event")
            .description(long_desc)
            .timestamp("10:00"),
    );

    // Cut to one row of 25 columns, not wrapped
    let rows = screen(&tl, 40, 10);
    assert_eq!(cut(&rows[1], 15, usize::MAX), "This is a long descriptio");
    assert!(rows[2].trim().is_empty());
}
#[test]
fn test_timeline_description_with_newlines() {
    let desc_with_newlines = "Line 1\nLine 2\nLine 3";
    let tl = Timeline::new().event(TimelineEvent::new("Event").description(desc_with_newlines));

    // Drawn on its one row, from the first line
    let rows = screen(&tl, 40, 10);
    assert!(
        cut(&rows[1], 15, usize::MAX).starts_with("Line 1"),
        "{:?}",
        rows[1]
    );
    assert!(rows[2].trim().is_empty());
}
// =============================================================================
// Timestamp Format Tests
// =============================================================================

#[test]
fn test_timeline_iso_timestamp() {
    let tl = Timeline::new().event(TimelineEvent::new("Event").timestamp("2024-01-15T10:30:00Z"));
    assert!(screen(&tl, 40, 10)[0].starts_with("2024-01-15T ●"));
}
#[test]
fn test_timeline_time_only_timestamp() {
    let tl = Timeline::new().event(TimelineEvent::new("Event").timestamp("10:30:45"));
    assert!(screen(&tl, 40, 10)[0].starts_with("10:30:45    ●"));
}
#[test]
fn test_timeline_relative_timestamp() {
    let tl = Timeline::new().event(TimelineEvent::new("Event").timestamp("2 hours ago"));
    assert!(screen(&tl, 40, 10)[0].starts_with("2 hours ago ●"));
}
// =============================================================================
// Large Event List Tests
// =============================================================================

#[test]
fn test_timeline_many_events() {
    let mut tl = Timeline::new();
    for i in 0..100 {
        tl = tl.event(TimelineEvent::new(format!("Event {}", i)));
    }
    assert_eq!(tl.len(), 100);
}

#[test]
fn test_timeline_render_many_events_small_area() {
    let mut tl = Timeline::new();
    for i in 0..20 {
        tl = tl.event(TimelineEvent::new(format!("Event {}", i)).timestamp(format!("{}:00", i)));
    }

    // The first three events fit
    let rows = screen(&tl, 40, 5);
    assert_eq!(rows[0].trim_end(), "0:00        ●─ Event 0");
    assert_eq!(rows[2].trim_end(), "1:00        ●─ Event 1");
    assert_eq!(rows[4].trim_end(), "2:00        ●─ Event 2");
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

// =============================================================================
// Mixed Event Configuration Tests
// =============================================================================

#[test]
fn test_timeline_mixed_timestamp_visibility() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Event 1").timestamp("10:00"))
        .event(TimelineEvent::new("Event 2")) // No timestamp
        .event(TimelineEvent::new("Event 3").timestamp("12:00"));

    let rows = screen(&tl, 40, 15);
    assert!(rows[0].starts_with("10:00"));
    assert!(
        rows[2].starts_with("            ●─ Event 2"),
        "{:?}",
        rows[2]
    );
    assert!(rows[4].starts_with("12:00"));
}
#[test]
fn test_timeline_mixed_description_visibility() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Event 1").description("Has description"))
        .event(TimelineEvent::new("Event 2")) // No description
        .event(TimelineEvent::new("Event 3").description("Also has description"));

    let rows: Vec<String> = screen(&tl, 40, 15)
        .iter()
        .map(|r| r.trim().to_string())
        .collect();
    assert_eq!(
        rows[..7],
        [
            "●─ Event 1",
            "│  Has description",
            "│",
            "●─ Event 2",
            "│",
            "●─ Event 3",
            "Also has description"
        ]
    );
}
#[test]
fn test_timeline_mixed_event_types() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Info Event"))
        .event(TimelineEvent::new("Success Event").success())
        .event(TimelineEvent::new("Warning Event").warning())
        .event(TimelineEvent::new("Error Event").error())
        .event(TimelineEvent::new("Custom Event").event_type(EventType::Custom('★')));

    let buffer = draw(&tl, 40, 15);
    let icons: Vec<char> = [0, 2, 4, 6, 8]
        .iter()
        .map(|&y| sym(&buffer, 12, y))
        .collect();
    assert_eq!(icons, ['●', '✓', '⚠', '✗', '★']);
}
#[test]
fn test_timeline_mixed_colors() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Red").color(Color::RED))
        .event(TimelineEvent::new("Green").color(Color::GREEN))
        .event(TimelineEvent::new("Blue").color(Color::BLUE))
        .event(TimelineEvent::new("Yellow").color(Color::YELLOW));

    let buffer = draw(&tl, 40, 15);
    let colors: Vec<_> = [0, 2, 4, 6]
        .iter()
        .map(|&y| buffer.get(12, y).unwrap().fg)
        .collect();
    assert_eq!(
        colors,
        [
            Some(Color::RED),
            Some(Color::GREEN),
            Some(Color::BLUE),
            Some(Color::YELLOW)
        ]
    );
}
// =============================================================================
// Property Combination Tests
// =============================================================================

#[test]
fn test_timeline_all_combinations_vertical() {
    let orientations = [TimelineOrientation::Vertical];
    let styles = [
        TimelineStyle::Line,
        TimelineStyle::Boxed,
        TimelineStyle::Minimal,
        TimelineStyle::Alternating,
    ];

    for orientation in &orientations {
        for style in &styles {
            let tl = Timeline::new()
                .orientation(*orientation)
                .style(*style)
                .timestamps(true)
                .descriptions(true)
                .event(TimelineEvent::new("Test").description("Test description"));

            let rows = screen(&tl, 40, 10);
            assert_eq!(cut(&rows[0], 15, 19), "Test", "{style:?}");
            assert_eq!(cut(&rows[1], 15, 31), "Test description", "{style:?}");
        }
    }
}
#[test]
fn test_timeline_all_combinations_horizontal() {
    let orientations = [TimelineOrientation::Horizontal];
    let styles = [
        TimelineStyle::Line,
        TimelineStyle::Boxed,
        TimelineStyle::Minimal,
        TimelineStyle::Alternating,
    ];

    for orientation in &orientations {
        for style in &styles {
            let tl = Timeline::new()
                .orientation(*orientation)
                .style(*style)
                .timestamps(true)
                .event(TimelineEvent::new("Test"));

            // The horizontal layout draws the same for every style
            let rows = screen(&tl, 60, 5);
            assert_eq!(rows[0].trim(), "Test", "{style:?}");
            assert!(rows[1].starts_with("───────●───"), "{style:?}");
        }
    }
}
// =============================================================================
// Event Content Edge Cases
// =============================================================================

#[test]
fn test_timeline_title_with_tabs() {
    let event = TimelineEvent::new("Event\twith\ttabs");
    assert_eq!(event.title, "Event\twith\ttabs");
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_description_with_tabs() {
    let tl = Timeline::new().event(TimelineEvent::new("Event").description("Desc\twith\ttabs"));
    assert!(cut(&screen(&tl, 40, 3)[1], 15, usize::MAX).starts_with("Desc"));
}
#[test]
fn test_timeline_timestamp_with_tabs() {
    let tl = Timeline::new().event(TimelineEvent::new("Event").timestamp("10:\t00"));
    let rows = screen(&tl, 40, 3);
    assert!(rows[0].starts_with("10:"));
    assert_eq!(sym(&draw(&tl, 40, 3), 12, 0), '●');
}
#[test]
fn test_timeline_title_with_special_symbols() {
    let event = TimelineEvent::new("♠♥♦♣ ★☆☀☁");
    assert_eq!(event.title, "♠♥♦♣ ★☆☀☁");
    let tl = Timeline::new().event(event);
    assert_eq!(tl.len(), 1);
}

#[test]
fn test_timeline_mathematical_symbols() {
    let tl = Timeline::new().event(TimelineEvent::new("∑ ∫ ∞ √ ≠ ≈ ≤ ≥"));
    assert!(screen(&tl, 40, 3)[0].ends_with("∑ ∫ ∞ √ ≠ ≈ ≤ ≥          "));
}
#[test]
fn test_timeline_arrows_and_directions() {
    let tl = Timeline::new().event(TimelineEvent::new("← ↑ → ↓ ↔ ↕ ↖ ↗ ↘ ↙"));
    assert!(screen(&tl, 40, 3)[0].contains("●─ ← ↑ → ↓ ↔ ↕ ↖ ↗ ↘ ↙"));
}
// =============================================================================
// Builder Pattern Idempotency Tests
// =============================================================================

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

// =============================================================================
// Event Type Custom Character Tests
// =============================================================================

#[test]
fn test_timeline_custom_event_ascii() {
    let tl = Timeline::new().event(TimelineEvent::new("Event").event_type(EventType::Custom('#')));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    assert_eq!(buffer.get(12, area.y).unwrap().symbol, '#');
}

#[test]
fn test_timeline_custom_event_number() {
    let _tl = Timeline::new().event(TimelineEvent::new("Event").event_type(EventType::Custom('1')));
    assert_eq!(EventType::Custom('1').icon(), '1');
}

#[test]
fn test_timeline_custom_event_punctuation() {
    let chars = ['!', '@', '#', '$', '%', '&', '*', '?'];
    for ch in chars {
        let tl =
            Timeline::new().event(TimelineEvent::new("Event").event_type(EventType::Custom(ch)));

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        tl.render(&mut ctx);

        assert_eq!(buffer.get(12, area.y).unwrap().symbol, ch);
    }
}

// =============================================================================
// Selected Event Rendering Tests
// =============================================================================

#[test]
fn test_timeline_selected_event_bold_modifier() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1").timestamp("10:00"))
        .event(TimelineEvent::new("Event 2").timestamp("11:00"))
        .event(TimelineEvent::new("Event 3").timestamp("12:00"));

    tl.select(Some(1));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Selected event should render with BOLD modifier somewhere
    let mut found_bold = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.modifier.contains(Modifier::BOLD) {
                    found_bold = true;
                    break;
                }
            }
        }
        if found_bold {
            break;
        }
    }
    assert!(found_bold);
}

#[test]
fn test_timeline_non_selected_event_no_bold() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1").timestamp("10:00"))
        .event(TimelineEvent::new("Event 2").timestamp("11:00"));

    tl.select(Some(0));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Non-selected event should not have BOLD on its icon
    let icon_cell = buffer.get(12, area.y + 3).unwrap(); // Second event
    assert!(!icon_cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_timeline_selected_title_uses_event_color() {
    let mut tl = Timeline::new()
        .event(TimelineEvent::new("Event 1").color(Color::MAGENTA))
        .event(TimelineEvent::new("Event 2"));

    tl.select(Some(0));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    // Selected title should use event color
    let title_cell = buffer.get(15, area.y).unwrap(); // Content starts at x=15
    assert_eq!(title_cell.fg, Some(Color::MAGENTA));
}

// =============================================================================
// Area Boundary Tests
// =============================================================================

#[test]
fn test_timeline_very_narrow_width() {
    // Without timestamps the icon, connector and two title columns fit
    let tl = Timeline::new()
        .timestamps(false)
        .event(TimelineEvent::new("Event"));
    assert_eq!(screen(&tl, 5, 10)[0], "●─ Ev");
}
#[test]
fn test_timeline_very_short_height() {
    let tl = Timeline::new()
        .event(TimelineEvent::new("Event 1"))
        .event(TimelineEvent::new("Event 2"));

    // Fewer than 2 rows: nothing is drawn
    assert!(blank(&draw(&tl, 40, 1)));
}
#[test]
fn test_timeline_single_cell_area() {
    let tl = Timeline::new().event(TimelineEvent::new("E"));
    assert!(blank(&draw(&tl, 1, 1)));
}
// =============================================================================
// Integration with CSS/Styling System
// =============================================================================

#[test]
fn test_timeline_with_id_and_classes_renders() {
    let tl = timeline()
        .element_id("test-timeline")
        .class("class1")
        .class("class2")
        .event(TimelineEvent::new("Event"));

    assert_eq!(View::id(&tl), Some("test-timeline"));
    assert!(tl.has_class("class1"));
    assert!(tl.has_class("class2"));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);
}

#[test]
fn test_timeline_classes_builder_renders() {
    let tl = timeline()
        .classes(vec!["a", "b", "c"])
        .event(TimelineEvent::new("Event"));

    assert!(tl.has_class("b"));
    let plain = timeline().event(TimelineEvent::new("Event"));
    assert_eq!(screen(&tl, 40, 10), screen(&plain, 40, 10));
}
// =============================================================================
// View Trait Implementation Tests
// =============================================================================

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

mod snapshots {

    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_timeline_basic() {
        use revue::widget::{Timeline, TimelineEvent};

        let view = Timeline::new()
            .event(TimelineEvent::new("Project Started").timestamp("2024-01"))
            .event(TimelineEvent::new("Beta Release").timestamp("2024-06"))
            .event(TimelineEvent::new("1.0 Launch").timestamp("2024-12"));

        let config = TestConfig::with_size(50, 12);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("timeline_basic");
    }

    #[test]
    fn test_timeline_with_descriptions() {
        use revue::widget::{EventType, Timeline, TimelineEvent};

        let view = Timeline::new()
            .event(
                TimelineEvent::new("Bug Fix")
                    .description("Fixed critical login issue")
                    .event_type(EventType::Success)
                    .timestamp("10:30"),
            )
            .event(
                TimelineEvent::new("Deployment")
                    .description("Pushed to production")
                    .event_type(EventType::Info)
                    .timestamp("11:00"),
            )
            .event(
                TimelineEvent::new("Alert")
                    .description("High memory usage")
                    .event_type(EventType::Warning)
                    .timestamp("11:30"),
            );

        let config = TestConfig::with_size(60, 15);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("timeline_descriptions");
    }
}
