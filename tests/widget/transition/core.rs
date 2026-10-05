//! Transition widget tests: phase transitions and rendering
//!
//! Construction and plain show/hide/toggle visibility are covered by
//! tests/transition_tests.rs and tests/transition_widget_tests.rs; these
//! drive the animation phases and check what actually reaches the buffer.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Animation, AnimationTransition as Transition, TransitionPhase, View};
use std::thread;
use std::time::Duration;

/// An animation that will not finish during a test.
fn slow() -> Animation {
    Animation::fade().duration(60_000)
}

/// An animation that holds at progress 0 for the whole test.
fn held() -> Animation {
    Animation::fade().duration(10).delay(60_000)
}

/// An animation that finishes almost immediately.
fn quick() -> Animation {
    Animation::fade().duration(1)
}

fn wait_out_quick() {
    thread::sleep(Duration::from_millis(20));
}

fn render(t: &Transition, width: u16) -> Buffer {
    let mut buffer = Buffer::new(width, 2);
    let area = Rect::new(0, 0, width, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

fn untouched(buffer: &Buffer, width: u16) -> bool {
    (0..width).all(|x| {
        let cell = buffer.get(x, 0).unwrap();
        cell.symbol == ' ' && cell.bg.is_none() && cell.fg.is_none()
    })
}

// =========================================================================
// Phases without animations
// =========================================================================

#[test]
fn test_hide_without_leave_animation_goes_straight_to_hidden() {
    let mut t = Transition::new("test");
    t.hide();
    assert!(!t.is_visible());
    assert_eq!(t.phase(), TransitionPhase::Hidden);
}

#[test]
fn test_show_without_enter_animation_goes_straight_to_visible() {
    let mut t = Transition::new("test");
    t.hide();
    t.show();
    assert!(t.is_visible());
    assert_eq!(t.phase(), TransitionPhase::Visible);
}

#[test]
fn test_show_when_already_visible_keeps_phase() {
    let mut t = Transition::new("test").enter(slow());
    t.show();
    // Already visible: no enter animation is started
    assert_eq!(t.phase(), TransitionPhase::Visible);
}

#[test]
fn test_update_without_animation_changes_nothing() {
    let mut t = Transition::new("test");
    t.update();
    assert_eq!(t.phase(), TransitionPhase::Visible);
    assert!(t.is_visible());
}

// =========================================================================
// Phases with animations
// =========================================================================

#[test]
fn test_hide_with_leave_animation_stays_visible_while_leaving() {
    let mut t = Transition::new("test").leave(slow());
    t.hide();
    assert_eq!(t.phase(), TransitionPhase::Leaving);
    assert!(t.is_visible());

    t.update();
    assert_eq!(t.phase(), TransitionPhase::Leaving);
    assert!(t.is_visible());
}

#[test]
fn test_leave_animation_completes_to_hidden() {
    let mut t = Transition::new("test").leave(quick());
    t.hide();
    wait_out_quick();
    t.update();
    assert_eq!(t.phase(), TransitionPhase::Hidden);
    assert!(!t.is_visible());
}

#[test]
fn test_show_with_enter_animation_is_entering() {
    let mut t = Transition::new("test").enter(slow());
    t.hide();
    t.show();
    assert_eq!(t.phase(), TransitionPhase::Entering);
    assert!(t.is_visible());

    t.update();
    assert_eq!(t.phase(), TransitionPhase::Entering);
}

#[test]
fn test_enter_animation_completes_to_visible() {
    let mut t = Transition::new("test").enter(quick());
    t.hide();
    t.show();
    wait_out_quick();
    t.update();
    assert_eq!(t.phase(), TransitionPhase::Visible);
    assert!(t.is_visible());
}

#[test]
fn test_animations_sets_both_directions() {
    let mut t = Transition::new("test").animations(quick(), quick());

    t.hide();
    assert_eq!(t.phase(), TransitionPhase::Leaving);
    wait_out_quick();
    t.update();
    assert_eq!(t.phase(), TransitionPhase::Hidden);

    t.show();
    assert_eq!(t.phase(), TransitionPhase::Entering);
    wait_out_quick();
    t.update();
    assert_eq!(t.phase(), TransitionPhase::Visible);
}

#[test]
fn test_show_during_leave_reverses_it() {
    let mut t = Transition::new("test").animations(slow(), quick());
    t.hide();
    assert_eq!(t.phase(), TransitionPhase::Leaving);

    t.show();
    assert_eq!(t.phase(), TransitionPhase::Entering);

    // The abandoned leave animation must not hide the content later
    wait_out_quick();
    t.update();
    assert!(t.is_visible());
    assert_eq!(t.phase(), TransitionPhase::Entering);
}

#[test]
fn test_toggle_during_leave_reverses_it() {
    let mut t = Transition::new("test").animations(slow(), slow());
    t.hide();
    assert_eq!(t.phase(), TransitionPhase::Leaving);

    t.toggle();
    assert_eq!(t.phase(), TransitionPhase::Entering);
    assert!(t.is_visible());
}

// =========================================================================
// Rendering
// =========================================================================

#[test]
fn test_render_visible_draws_content() {
    let t = Transition::new("Hello");
    let buffer = render(&t, 10);
    assert_eq!(row(&buffer, 0, 10), "Hello     ");

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::WHITE));
    assert_eq!(cell.bg, Some(Color::BLACK));
    assert!(!cell.modifier.contains(Modifier::DIM));
    // Single-line content: the second row is left alone
    assert_eq!(row(&buffer, 1, 10), "          ");
}

#[test]
fn test_render_clips_to_width() {
    let t = Transition::new("Hello World");
    let buffer = render(&t, 5);
    assert_eq!(row(&buffer, 0, 5), "Hello");
}

#[test]
fn test_render_wide_chars() {
    let t = Transition::new("한글");
    let buffer = render(&t, 6);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '글');
}

#[test]
fn test_render_wide_char_that_does_not_fit_is_dropped() {
    let t = Transition::new("A한");
    let buffer = render(&t, 2);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, ' ');
}

#[test]
fn test_render_hidden_draws_nothing() {
    let mut t = Transition::new("Hello");
    t.hide();
    let buffer = render(&t, 10);
    assert!(untouched(&buffer, 10));
}

#[test]
fn test_render_after_leave_completes_draws_nothing() {
    let mut t = Transition::new("Hello").leave(quick());
    t.hide();
    wait_out_quick();
    t.update();
    let buffer = render(&t, 10);
    assert!(untouched(&buffer, 10));
}

#[test]
fn test_render_entering_at_start_reveals_nothing() {
    let mut t = Transition::new("Hello").enter(held());
    t.hide();
    t.show();
    assert_eq!(t.phase(), TransitionPhase::Entering);

    let buffer = render(&t, 10);
    // Progress 0: every character is still a blank placeholder
    assert_eq!(row(&buffer, 0, 10), "          ");
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_render_leaving_at_start_shows_everything() {
    let mut t = Transition::new("Hello").leave(held());
    t.hide();
    assert_eq!(t.phase(), TransitionPhase::Leaving);

    let buffer = render(&t, 10);
    assert_eq!(row(&buffer, 0, 10), "Hello     ");
    assert!(!buffer.get(0, 0).unwrap().modifier.contains(Modifier::DIM));
}

#[test]
fn test_render_mid_animation_is_dimmed() {
    let mut t = Transition::new("Hello").leave(slow());
    t.hide();
    thread::sleep(Duration::from_millis(5));

    let buffer = render(&t, 10);
    // Barely started leaving: all characters still shown, but dimmed
    assert_eq!(row(&buffer, 0, 10), "Hello     ");
    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::DIM));
}
