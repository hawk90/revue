//! Timer widget tests not covered by tests/widget/timer.rs or tests/timer_tests.rs

use revue::widget::data::timer::{pomodoro, stopwatch, timer, Timer, TimerFormat, TimerState};

#[test]
fn test_timer_toggle() {
    let mut timer = Timer::countdown(60);
    assert_eq!(timer.state(), TimerState::Stopped);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Running);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Paused);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Running);
}

#[test]
fn test_helper_functions() {
    let t = timer(120);
    assert_eq!(t.remaining_seconds(), 120);

    let sw = stopwatch();
    assert_eq!(sw.elapsed_millis(), 0);

    let p = pomodoro();
    assert_eq!(p.remaining_seconds(), 25 * 60);
}

#[test]
fn test_timer_format_default() {
    assert_eq!(TimerFormat::default(), TimerFormat::Full);
}

#[test]
fn progress_width_sets_the_bar_width() {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};

    let timer = Timer::countdown(60).progress_width(12);
    let mut buffer = Buffer::new(40, 6);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 6));
    timer.render(&mut ctx);

    let bar_cells = |y: u16| {
        (0..40)
            .filter(|&x| matches!(buffer.get(x, y).map(|c| c.symbol), Some('█' | '░')))
            .count()
    };
    let widths: Vec<usize> = (0..6).map(bar_cells).filter(|&n| n > 0).collect();
    assert_eq!(widths, vec![12], "bar widths per row: {widths:?}");
}
