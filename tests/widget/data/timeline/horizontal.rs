//! Horizontal timeline tests: the line, icons, titles and timestamps

use super::{blank, draw, screen};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{Timeline, TimelineEvent};

/// The events were drawn from the first, so a selected event past the width
/// was not drawn (#973)
#[test]
fn test_timeline_horizontal_selected_event_scrolls_into_view() {
    let mut tl = Timeline::new().horizontal().events(
        (0..10)
            .map(|i| TimelineEvent::new(format!("E{i}")))
            .collect(),
    );
    tl.select(Some(7));
    let rows = screen(&tl, 45, 3); // three events wide
    assert!(rows[0].contains("E7"), "{:?}", rows[0]); // was E0, E1, E2
    assert!(!rows[0].contains("E4"), "{:?}", rows[0]);
}

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

/// A title and a timestamp 65536 columns wide are cut to the event's 14
/// columns before they are centered, in an area at an offset (#1607)
#[test]
fn test_timeline_horizontal_title_wider_than_u16() {
    let tl = Timeline::new()
        .horizontal()
        .event(TimelineEvent::new("g".repeat(65_536)).timestamp("t".repeat(65_536)));
    let mut buffer = Buffer::new(24, 5);
    tl.render(&mut RenderContext::new(&mut buffer, Rect::new(2, 1, 20, 3)));
    let rows: Vec<String> = (0..5)
        .map(|y| (0..24).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect();
    assert_eq!(rows[0], " ".repeat(24));
    assert_eq!(rows[1], format!("  {}        ", "g".repeat(14)));
    assert_eq!(rows[2], "  ───────●────────────  ");
    assert_eq!(rows[3], format!("  {}        ", "t".repeat(14)));
    assert_eq!(rows[4], " ".repeat(24));
}
