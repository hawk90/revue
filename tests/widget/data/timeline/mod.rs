//! Timeline widget tests

mod builders;
mod horizontal;
mod selection;
mod types;
mod vertical;
mod view;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{Timeline, TimelineEvent};

/// Three vertical events: two with timestamps, the first with a description
pub(super) fn three() -> Timeline {
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
pub(super) fn draw(tl: &Timeline, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    tl.render(&mut ctx);
    buffer
}

/// Each row of `tl` drawn in a `width` x `height` area
pub(super) fn screen(tl: &Timeline, width: u16, height: u16) -> Vec<String> {
    let buffer = draw(tl, width, height);
    (0..height)
        .map(|y| (0..width).map(|x| sym(&buffer, x, y)).collect())
        .collect()
}

pub(super) fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

/// Columns `from..to` of `row`
pub(super) fn cut(row: &str, from: usize, to: usize) -> String {
    row.chars().take(to).skip(from).collect()
}

/// Nothing drawn anywhere in `buffer`
pub(super) fn blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| (0..buffer.width()).all(|x| sym(buffer, x, y) == ' '))
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
