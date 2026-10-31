//! StatusIndicator widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::away_indicator;
use revue::widget::busy_indicator;
use revue::widget::offline;
use revue::widget::online;
use revue::widget::traits::RenderContext;
use revue::widget::traits::View;
use revue::widget::Status;
use revue::widget::StatusIndicator;

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

/// The character a dot-style indicator draws at the origin.
fn rendered_dot(indicator: &StatusIndicator) -> char {
    let mut buffer = Buffer::new(4, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 4, 1));
    indicator.render(&mut ctx);
    buffer.get(0, 0).map(|c| c.symbol).unwrap_or(' ')
}

#[test]
fn a_stored_indicator_blinks_as_it_is_ticked() {
    let mut indicator = StatusIndicator::online().pulsing(true);
    assert_eq!(rendered_dot(&indicator), '●');
    for _ in 0..6 {
        indicator.tick();
    }
    // Frames 6 and 7 of each 8-frame cycle are the "off" phase.
    assert_eq!(rendered_dot(&indicator), ' ');
}

#[test]
fn a_rebuilt_indicator_takes_its_frame_from_the_app() {
    let on = StatusIndicator::online().pulsing(true).frame(5);
    let off = StatusIndicator::online().pulsing(true).frame(6);
    assert_eq!(rendered_dot(&on), '●');
    assert_eq!(rendered_dot(&off), ' ');
    // The cycle wraps.
    assert_eq!(
        rendered_dot(&StatusIndicator::online().pulsing(true).frame(8)),
        '●'
    );
}

#[test]
fn the_frame_does_nothing_unless_pulsing() {
    assert_eq!(rendered_dot(&StatusIndicator::online().frame(6)), '●');
}

// =========================================================================
// #1204: the Large dot's pulse, the label and the badge's size
// =========================================================================

mod sizes {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::Color;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{StatusIndicator, StatusSize, StatusStyle};

    fn draw(view: &StatusIndicator, width: u16) -> Buffer {
        let mut buffer = Buffer::new(width, 1);
        view.render(&mut RenderContext::new(
            &mut buffer,
            Rect::new(0, 0, width, 1),
        ));
        buffer
    }

    /// The pulse shows the dot at frame 0 and hides it at frame 6.
    fn at_frame(view: StatusIndicator, frame: usize) -> StatusIndicator {
        view.pulsing(true).frame(frame)
    }

    #[test]
    fn the_whole_large_dot_blinks() {
        let large = || {
            StatusIndicator::online()
                .size(StatusSize::Large)
                .indicator_style(StatusStyle::Dot)
        };
        let on = draw(&at_frame(large(), 0), 4);
        assert_ne!(on.get(0, 0).unwrap().symbol, ' ');
        assert!(on.get(1, 0).unwrap().bg.is_some());
        let off = draw(&at_frame(large(), 6), 4);
        assert_eq!(off.get(0, 0).unwrap().symbol, ' ');
        assert_eq!(off.get(1, 0).unwrap().bg, None, "column 1 blinks too");
        // Without a pulse it never goes off.
        let still = draw(&large(), 4);
        assert!(still.get(1, 0).unwrap().bg.is_some());
    }

    #[test]
    fn the_label_form_draws_the_same_dot() {
        let large = || {
            StatusIndicator::online()
                .size(StatusSize::Large)
                .indicator_style(StatusStyle::DotWithLabel)
        };
        let on = draw(&large(), 12);
        assert!(on.get(1, 0).unwrap().bg.is_some());
        let off = draw(&at_frame(large(), 6), 12);
        assert_eq!(off.get(0, 0).unwrap().symbol, ' ');
        assert_eq!(off.get(1, 0).unwrap().bg, None);
        // Smaller sizes have no second column.
        let medium = StatusIndicator::online().indicator_style(StatusStyle::DotWithLabel);
        assert_eq!(draw(&medium, 12).get(1, 0).unwrap().bg, None);
        // A narrow area keeps it in bounds.
        assert_eq!(
            draw(&large(), 1).get(0, 0).unwrap().symbol,
            StatusSize::Large.dot()
        );
    }

    #[test]
    fn the_badge_draws_the_dot_of_its_size() {
        for size in [StatusSize::Small, StatusSize::Medium, StatusSize::Large] {
            let badge = StatusIndicator::online()
                .size(size)
                .indicator_style(StatusStyle::Badge);
            let buffer = draw(&badge, 14);
            assert_eq!(buffer.get(1, 0).unwrap().symbol, size.dot(), "{size:?}");
            assert_ne!(buffer.get(1, 0).unwrap().fg, Some(Color::BLACK));
        }
    }
}
