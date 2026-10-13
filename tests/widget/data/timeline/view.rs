//! Timeline View tests: zero areas, every style in both layouts, classes and placement

use super::{blank, cut, screen, three};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::traits::StyledView;
use revue::widget::View;
use revue::widget::{timeline, Timeline, TimelineEvent, TimelineOrientation, TimelineStyle};

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

#[test]
fn test_timeline_draws_the_same_when_offset() {
    let mut tl = three().line_color(Color::YELLOW);
    tl.select(Some(1));
    super::super::chart::assert_same_when_offset(tl.clone(), 40, 10);
    super::super::chart::assert_same_when_offset(tl.horizontal(), 40, 4);
}
