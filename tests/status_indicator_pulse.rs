//! A pulsing `StatusIndicator` only blinks when something advances its frame.
//!
//! The widget owns no clock: `tick()` steps an internal counter, and views
//! usually rebuild the indicator on every render, which resets that counter.
//! So the app keeps the frame, advances it on `Event::Tick`, and hands it in
//! with `.frame(n)`.

use revue::prelude::*;

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
