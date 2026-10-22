//! Vertical timeline tests: timestamps, icons, lines, titles and descriptions, cut to the area

use super::{blank, cut, draw, screen, sym};
use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::theme::{DARK_GRAY, LIGHT_GRAY, MUTED_TEXT};
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{EventType, Timeline, TimelineEvent, TimelineStyle};

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
fn test_timeline_custom_event_with_emoji() {
    let tl = Timeline::new().event(TimelineEvent::new("Party").event_type(EventType::Custom('🎉')));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    tl.render(&mut ctx);

    assert_eq!(buffer.get(12, area.y).unwrap().symbol, '🎉');
}

#[test]
fn test_timeline_render_empty_title() {
    let tl = Timeline::new().event(TimelineEvent::new("").timestamp("10:00"));
    assert_eq!(screen(&tl, 40, 10)[0].trim_end(), "10:00       ●─");
}

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
fn test_timeline_render_timestamp_left_aligned() {
    let tl = Timeline::new()
        .timestamps(true)
        .event(TimelineEvent::new("Event").timestamp("10:00"));

    // The timestamp starts at column 0 of its 12-column gutter
    assert!(screen(&tl, 40, 10)[0].starts_with("10:00       ●"));
}

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
fn test_timeline_mathematical_symbols() {
    let tl = Timeline::new().event(TimelineEvent::new("∑ ∫ ∞ √ ≠ ≈ ≤ ≥"));
    assert!(screen(&tl, 40, 3)[0].ends_with("∑ ∫ ∞ √ ≠ ≈ ≤ ≥          "));
}

#[test]
fn test_timeline_arrows_and_directions() {
    let tl = Timeline::new().event(TimelineEvent::new("← ↑ → ↓ ↔ ↕ ↖ ↗ ↘ ↙"));
    assert!(screen(&tl, 40, 3)[0].contains("●─ ← ↑ → ↓ ↔ ↕ ↖ ↗ ↘ ↙"));
}

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

/// Events E0..E{n-1}, without timestamps, each with a description d{i}
fn described(n: usize) -> Timeline {
    Timeline::new().timestamps(false).events(
        (0..n)
            .map(|i| TimelineEvent::new(format!("E{i}")).description(format!("d{i}")))
            .collect(),
    )
}

/// The view never scrolled, so a selected event below the area was not
/// drawn (#973). It scrolls in rows, so the event above can be cut.
#[test]
fn test_timeline_vertical_selected_event_scrolls_into_view() {
    let mut tl = described(10);
    tl.select(Some(9));
    let rows: Vec<String> = screen(&tl, 20, 4)
        .iter()
        .map(|r| r.trim_end().to_string())
        .collect();
    // E8 is cut to its description and the line below it
    assert_eq!(rows, ["│  d8", "│", "●─ E9", "   d9"]); // was E0, d0, │, E1
}

#[test]
fn test_timeline_vertical_view_holds_while_the_selection_moves_inside_it() {
    let mut tl = described(10);
    tl.select(Some(9));
    let before = screen(&tl, 20, 6);
    assert!(before[1].starts_with("●─ E8"), "{before:?}"); // followed it down
    tl.select_prev(); // E8, on screen
    assert_eq!(screen(&tl, 20, 6), before);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Without timestamps and descriptions, event i is drawn on row 2i with
    /// its title, for every event that fits the height.
    #[test]
    fn test_timeline_vertical_rows(n in 1usize..15, h in 2u16..30, minimal in any::<bool>()) {
        let style = if minimal { TimelineStyle::Minimal } else { TimelineStyle::Line };
        let tl = Timeline::new()
            .timestamps(false)
            .style(style)
            .events((0..n).map(|i| TimelineEvent::new(format!("E{i}"))).collect());
        let rows = screen(&tl, 20, h);
        for i in 0..n {
            let y = 2 * i;
            if y < h as usize {
                let want = if minimal { format!("●  E{i}") } else { format!("●─ E{i}") };
                prop_assert_eq!(rows[y].trim_end(), want);
            }
        }
    }
}
