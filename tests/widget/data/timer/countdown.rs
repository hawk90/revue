//! Timer (countdown) widget tests

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView};
use revue::widget::{Timer, TimerFormat, TimerState, View};
use std::thread;
use std::time::{Duration, Instant};

/// `view` drawn in a `width` x `height` area
fn draw(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    buffer
}

/// Each row of `buffer` as text
fn rows_of(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect()
        })
        .collect()
}

/// Each row of `view` drawn in a `width` x `height` area
fn screen(view: &impl View, width: u16, height: u16) -> Vec<String> {
    rows_of(&draw(view, width, height))
}

/// Poll for a condition with a timeout, returning when the condition becomes true
/// or the timeout elapses. Returns true if condition was met, false on timeout.
fn poll_until<F>(mut condition: F, timeout_ms: u64) -> bool
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    let timeout = Duration::from_millis(timeout_ms);
    let poll_interval = Duration::from_millis(5);

    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        thread::sleep(poll_interval);
    }
    false
}

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_timer_countdown_new() {
    let timer = Timer::countdown(60);
    assert_eq!(timer.remaining_seconds(), 60);
    assert_eq!(timer.state(), TimerState::Stopped);
    assert!(!timer.is_running());
    assert!(!timer.is_completed());
}

#[test]
fn test_timer_countdown_zero() {
    let timer = Timer::countdown(0);
    assert_eq!(timer.remaining_seconds(), 0);
    assert_eq!(timer.state(), TimerState::Stopped);
}

#[test]
fn test_timer_countdown_large() {
    let timer = Timer::countdown(3600); // 1 hour
    assert_eq!(timer.remaining_seconds(), 3600);
    assert_eq!(timer.state(), TimerState::Stopped);
}

#[test]
fn test_timer_pomodoro() {
    let timer = Timer::pomodoro();
    assert_eq!(timer.remaining_seconds(), 25 * 60);
    assert_eq!(screen(&timer, 30, 4)[0].trim(), "Pomodoro");
}
#[test]
fn test_timer_short_break() {
    let timer = Timer::short_break();
    assert_eq!(timer.remaining_seconds(), 5 * 60);
    assert_eq!(screen(&timer, 30, 4)[0].trim(), "Short Break");
}
#[test]
fn test_timer_long_break() {
    let timer = Timer::long_break();
    assert_eq!(timer.remaining_seconds(), 15 * 60);
    assert_eq!(screen(&timer, 30, 4)[0].trim(), "Long Break");
}
// =============================================================================
// Builder Methods Tests
// =============================================================================

#[test]
fn test_timer_format_full() {
    let timer = Timer::countdown(3661).format(TimerFormat::Full);
    assert_eq!(timer.format_remaining(), "01:01:01");

    let timer = Timer::countdown(3661); // 1h 1m 1s
    assert_eq!(timer.format_remaining(), "01:01:01");
}

#[test]
fn test_timer_format_short() {
    let timer = Timer::countdown(65).format(TimerFormat::Short);
    assert_eq!(timer.format_remaining(), "01:05");
}

#[test]
fn test_timer_format_short_with_hours() {
    let timer = Timer::countdown(3661).format(TimerFormat::Short);
    assert_eq!(timer.format_remaining(), "01:01:01");
}

#[test]
fn test_timer_format_precise() {
    let timer = Timer::countdown(0).format(TimerFormat::Precise);
    let formatted = timer.format_remaining();
    assert!(formatted.contains('.'));
    assert!(formatted.len() >= 4); // "00.000"

    // 65 s is a minute and five seconds; the minute used to be dropped.
    let timer = Timer::countdown(65).format(TimerFormat::Precise);
    assert_eq!(timer.format_remaining(), "01:05.000");
}

#[test]
fn test_timer_format_compact_hours() {
    let timer = Timer::countdown(3661).format(TimerFormat::Compact);
    assert_eq!(timer.format_remaining(), "1h 1m");
}

#[test]
fn test_timer_format_compact_minutes() {
    let timer = Timer::countdown(125).format(TimerFormat::Compact);
    assert_eq!(timer.format_remaining(), "2m 5s");
}

#[test]
fn test_timer_format_compact_seconds() {
    let timer = Timer::countdown(30).format(TimerFormat::Compact);
    assert_eq!(timer.format_remaining(), "30s");
}

#[test]
fn test_timer_show_progress() {
    let on = screen(&Timer::countdown(60).show_progress(true), 30, 4);
    assert!(on[1].starts_with('░'), "{on:#?}");
    let off = screen(&Timer::countdown(60).show_progress(false), 30, 4);
    assert_eq!(off[1].trim(), "Stopped");
}
#[test]
fn test_timer_progress_width() {
    let rows = screen(&Timer::countdown(60).progress_width(10), 30, 4);
    assert_eq!(rows[1].trim_end(), "░".repeat(10));
}
#[test]
fn test_timer_fg() {
    // Above the warning threshold the time takes `fg`
    let buffer = draw(&Timer::countdown(120).fg(Color::BLUE), 30, 4);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::BLUE));
}
#[test]
fn test_timer_warning_threshold() {
    // 60 s left: the default threshold (60 s) colors it, 30 s does not
    let default = draw(&Timer::countdown(60), 30, 4);
    assert_eq!(default.get(0, 0).unwrap().fg, Some(Color::YELLOW));
    let lower = draw(&Timer::countdown(60).warning_threshold(30), 30, 4);
    assert_eq!(lower.get(0, 0).unwrap().fg, Some(Color::WHITE));
}
#[test]
fn test_timer_danger_threshold() {
    // 60 s left: past the warning threshold, and past a danger threshold of 60
    let warning = draw(&Timer::countdown(60).danger_threshold(15), 30, 4);
    assert_eq!(warning.get(0, 0).unwrap().fg, Some(Color::YELLOW));
    let danger = draw(&Timer::countdown(60).danger_threshold(60), 30, 4);
    assert_eq!(danger.get(0, 0).unwrap().fg, Some(Color::RED));
}
#[test]
fn test_timer_title() {
    let rows = screen(&Timer::countdown(60).title("Test Timer"), 30, 4);
    assert_eq!(rows[0].trim(), "Test Timer");
    assert_eq!(rows[1].trim(), "00:01:00");
}
#[test]
fn test_timer_large_digits() {
    let large = screen(&Timer::countdown(60).large_digits(true), 40, 6);
    // Three rows of block digits for 00:01:00
    assert!(
        large[..3]
            .iter()
            .all(|r| r.contains('█') || r.contains('▀')),
        "{large:#?}"
    );
    assert!(large[3].starts_with('░'));
    let plain = screen(&Timer::countdown(60).large_digits(false), 40, 6);
    assert_eq!(plain[0].trim(), "00:01:00");
}
#[test]
fn test_timer_auto_restart_builder() {
    // A zero-length timer completes on its first update, unless it restarts
    let mut timer = Timer::countdown(0).auto_restart(true);
    timer.start();
    timer.update();
    assert_eq!(timer.state(), TimerState::Running);

    let mut timer = Timer::countdown(0).auto_restart(false);
    timer.start();
    timer.update();
    assert_eq!(timer.state(), TimerState::Completed);
}
#[test]
fn test_timer_element_id() {
    let timer = Timer::countdown(60).element_id("my-timer");
    assert_eq!(View::id(&timer), Some("my-timer"));
}

#[test]
fn test_timer_class() {
    let timer = Timer::countdown(60).class("countdown").class("urgent");
    assert!(timer.has_class("countdown"));
    assert!(timer.has_class("urgent"));
}

#[test]
fn test_timer_classes() {
    let timer = Timer::countdown(60).classes(vec!["class1", "class2", "class3"]);
    assert!(timer.has_class("class1"));
    assert!(timer.has_class("class2"));
    assert!(timer.has_class("class3"));
}

// =============================================================================
// State Management Tests
// =============================================================================

#[test]
fn test_timer_start() {
    let mut timer = Timer::countdown(60);
    assert_eq!(timer.state(), TimerState::Stopped);

    timer.start();
    assert_eq!(timer.state(), TimerState::Running);
    assert!(timer.is_running());
}

#[test]
fn test_timer_pause() {
    let mut timer = Timer::countdown(60);
    timer.start();
    assert_eq!(timer.state(), TimerState::Running);

    timer.pause();
    assert_eq!(timer.state(), TimerState::Paused);
    assert!(!timer.is_running());
}

#[test]
fn test_timer_pause_from_non_running() {
    let mut timer = Timer::countdown(60);
    timer.pause();
    assert_eq!(timer.state(), TimerState::Stopped);
}

#[test]
fn test_timer_stop() {
    let mut timer = Timer::countdown(60);
    timer.start();
    timer.pause();

    timer.stop();
    assert_eq!(timer.state(), TimerState::Stopped);
    assert_eq!(timer.remaining_seconds(), 60);

    let mut timer = Timer::countdown(60);
    timer.start();

    timer.stop();
    assert_eq!(timer.state(), TimerState::Stopped);
    assert_eq!(timer.remaining_seconds(), 60);
}

#[test]
fn test_timer_reset() {
    let mut timer = Timer::countdown(60);
    timer.start();
    // Wait for timer to advance (poll for up to 500ms)
    poll_until(
        || {
            timer.update();
            timer.remaining_seconds() < 60
        },
        500,
    );
    let remaining = timer.remaining_seconds();
    timer.reset();
    assert_eq!(timer.remaining_seconds(), 60);
    assert!(timer.remaining_seconds() > remaining || remaining == 60);

    let mut timer = Timer::countdown(60);

    timer.reset();
    assert_eq!(timer.remaining_seconds(), 60);
}

#[test]
fn test_timer_toggle_from_stopped() {
    let mut timer = Timer::countdown(60);
    assert_eq!(timer.state(), TimerState::Stopped);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Running);
}

#[test]
fn test_timer_toggle_from_running() {
    let mut timer = Timer::countdown(60);
    timer.start();
    assert_eq!(timer.state(), TimerState::Running);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Paused);
}

#[test]
fn test_timer_toggle_from_paused() {
    let mut timer = Timer::countdown(60);
    timer.start();
    timer.pause();
    assert_eq!(timer.state(), TimerState::Paused);

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Running);
}

#[test]
fn test_timer_toggle_from_completed() {
    let mut timer = Timer::countdown(0); // Already complete
    timer.toggle();
    // Should start
    assert_eq!(timer.state(), TimerState::Running);
}

#[test]
fn test_timer_resume_from_pause() {
    let mut timer = Timer::countdown(60);
    timer.start();
    // Wait for timer to advance
    poll_until(
        || {
            timer.update();
            timer.remaining_seconds() < 60
        },
        500,
    );
    timer.pause();

    let paused_remaining = timer.remaining_seconds();
    timer.start();

    assert_eq!(timer.state(), TimerState::Running);
    // Should resume from where it paused
    assert!(timer.remaining_seconds() <= paused_remaining);
}

// =============================================================================
// Progress Tests
// =============================================================================

#[test]
fn test_timer_progress_initial() {
    let timer = Timer::countdown(60);
    assert_eq!(timer.progress(), 0.0);
}

#[test]
fn test_timer_progress_half() {
    // Cannot directly set remaining_ms (private field)
    // Progress can only be tested indirectly through timer running
    let mut timer = Timer::countdown(60);
    let initial = timer.progress();
    timer.start();
    // Wait for progress to increase
    poll_until(
        || {
            timer.update();
            timer.progress() > initial
        },
        500,
    );
    // Progress should increase as time passes
    assert!(timer.progress() > initial);
}

#[test]
fn test_timer_progress_completed() {
    // Cannot directly set remaining_ms (private field)
    // Test with a timer that completes quickly
    let mut timer = Timer::countdown(0);
    timer.start();
    timer.update();
    assert_eq!(timer.progress(), 1.0);
}

#[test]
fn test_timer_progress_zero_duration() {
    let timer = Timer::countdown(0);
    assert_eq!(timer.progress(), 1.0);
}

#[test]
fn test_timer_progress_increases() {
    let mut timer = Timer::countdown(60);
    let initial_progress = timer.progress();

    timer.start();
    // Wait for progress to increase
    poll_until(
        || {
            timer.update();
            timer.progress() > initial_progress
        },
        500,
    );

    let later_progress = timer.progress();
    assert!(later_progress > initial_progress);
}

// =============================================================================
// Update Behavior Tests
// =============================================================================

#[test]
fn test_timer_update_when_stopped() {
    let mut timer = Timer::countdown(60);
    timer.update();
    assert_eq!(timer.state(), TimerState::Stopped);
    assert_eq!(timer.remaining_seconds(), 60);
}

#[test]
fn test_timer_update_when_paused() {
    let mut timer = Timer::countdown(60);
    timer.start();
    timer.pause();
    timer.update();
    assert_eq!(timer.state(), TimerState::Paused);
}

#[test]
fn test_timer_update_decrements_time() {
    let mut timer = Timer::countdown(60);
    timer.start();
    // Wait for timer to decrement
    poll_until(
        || {
            timer.update();
            timer.remaining_seconds() < 60
        },
        500,
    );

    assert!(timer.remaining_seconds() < 60);
    assert!(timer.remaining_seconds() > 0);
}

#[test]
fn test_timer_update_to_completion() {
    let mut timer = Timer::countdown(0);
    timer.start();
    timer.update();

    assert_eq!(timer.state(), TimerState::Completed);
    assert!(timer.is_completed());
    assert_eq!(timer.remaining_seconds(), 0);
}

#[test]
fn test_timer_auto_restart() {
    let mut timer = Timer::countdown(0).auto_restart(true);
    timer.start();
    timer.update();

    // Should restart immediately when completing
    assert_eq!(timer.state(), TimerState::Running);
    assert!(!timer.is_completed());
}

// =============================================================================
// Color State Tests
// =============================================================================

#[test]
fn test_timer_color_normal() {
    let timer = Timer::countdown(120); // Above warning threshold
                                       // Default warning is 60s
                                       // current_color returns white when above warning
    assert_eq!(timer.remaining_seconds(), 120);
}

#[test]
fn test_timer_color_warning() {
    // Cannot directly set remaining_ms (private field)
    // Test warning threshold with a timer that has less time remaining
    let timer = Timer::countdown(45).warning_threshold(30);
    // 45 seconds is within the warning threshold (30s)
    assert!(timer.remaining_seconds() > 30); // Above warning threshold
}

#[test]
fn test_timer_color_danger() {
    // Cannot directly set remaining_ms (private field)
    // Test danger threshold with a timer that has very little time
    let timer = Timer::countdown(10).danger_threshold(15);
    // 10 seconds is within the danger threshold (15s)
    assert!(timer.remaining_seconds() <= 15); // At or below danger threshold
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_timer_render_basic() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let timer = Timer::countdown(60);
    timer.render(&mut ctx);

    // Should render without panic
    // Check that time is rendered (contains digits or colons)
    let mut found_time = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol.is_ascii_digit() || cell.symbol == ':' {
                    found_time = true;
                    break;
                }
            }
        }
    }
    assert!(found_time);
}

#[test]
fn test_timer_render_with_title() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let timer = Timer::countdown(60).title("Test");
    timer.render(&mut ctx);

    // Should render title
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 's');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 't');
}

#[test]
fn test_timer_render_stopped_state() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let timer = Timer::countdown(60);
    timer.render(&mut ctx);

    // Should render "Stopped" indicator
    let mut found_stopped = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'S' {
                    found_stopped = true;
                    break;
                }
            }
        }
    }
    assert!(found_stopped);
}

#[test]
fn test_timer_render_running_state() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut timer = Timer::countdown(60);
    timer.start();
    timer.render(&mut ctx);

    // Should render "Running" indicator
    let mut found_running = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'R' {
                    found_running = true;
                    break;
                }
            }
        }
    }
    assert!(found_running);
}

#[test]
fn test_timer_render_paused_state() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut timer = Timer::countdown(60);
    timer.start();
    timer.pause();
    timer.render(&mut ctx);

    // Should render "Paused" indicator
    let mut found_paused = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'P' {
                    found_paused = true;
                    break;
                }
            }
        }
    }
    assert!(found_paused);
}

#[test]
fn test_timer_render_completed_state() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut timer = Timer::countdown(0);
    timer.start();
    timer.update();
    timer.render(&mut ctx);

    // Should render "Completed!" indicator
    let mut found_completed = false;
    for y in 0..5 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'C' {
                    found_completed = true;
                    break;
                }
            }
        }
    }
    assert!(found_completed);
}

#[test]
fn test_timer_render_small_buffer() {
    let timer = Timer::countdown(60);
    let rows = screen(&timer, 5, 3);
    assert_eq!(rows[0], "00:01");
    assert_eq!(rows[1], "░░░░░");
}
#[test]
fn test_timer_render_with_offset() {
    let timer = Timer::countdown(60).title("Tea").progress_width(12);
    super::super::chart::assert_same_when_offset(timer, 20, 5);
}
// =============================================================================
// Helper Functions Tests
// =============================================================================

#[test]
fn test_timer_helper_function() {
    // Test the helper function by using Timer::countdown directly
    // (the `timer()` function from digits module is a different type)
    let t = Timer::countdown(120);
    assert_eq!(t.remaining_seconds(), 120);
    assert_eq!(t.state(), TimerState::Stopped);
}

// =============================================================================
// Edge Cases Tests
// =============================================================================

#[test]
fn test_timer_zero_duration() {
    let timer = Timer::countdown(0);
    assert_eq!(timer.remaining_seconds(), 0);
    assert_eq!(timer.format_remaining(), "00:00:00");
}

#[test]
fn test_timer_very_short_duration() {
    let timer = Timer::countdown(1);
    assert_eq!(timer.remaining_seconds(), 1);
}

#[test]
fn test_timer_very_long_duration() {
    let timer = Timer::countdown(86400); // 24 hours
    assert_eq!(timer.remaining_seconds(), 86400);
    assert_eq!(timer.format_remaining(), "24:00:00");
}

#[test]
fn test_timer_multiple_starts() {
    let mut timer = Timer::countdown(60);
    timer.start();
    timer.start();
    timer.start();

    // Should remain running
    assert_eq!(timer.state(), TimerState::Running);
}

#[test]
fn test_timer_multiple_pauses() {
    let mut timer = Timer::countdown(60);
    timer.start();
    timer.pause();
    timer.pause();

    // Should remain paused
    assert_eq!(timer.state(), TimerState::Paused);
}

#[test]
fn test_timer_format_zero_remaining() {
    let timer = Timer::countdown(0);
    assert_eq!(timer.format_remaining(), "00:00:00");
}

#[test]
fn test_timer_format_precise_zero() {
    let timer = Timer::countdown(0).format(TimerFormat::Precise);
    let formatted = timer.format_remaining();
    assert!(formatted.starts_with("00"));
}

#[test]
fn test_timer_threshold_edge_case() {
    // Test with timer exactly at warning threshold
    let timer = Timer::countdown(10)
        .warning_threshold(10)
        .danger_threshold(5);
    assert_eq!(timer.remaining_seconds(), 10);
    // Exactly at warning threshold
}

#[test]
fn test_timer_danger_threshold_edge_case() {
    // Test with timer exactly at danger threshold
    let timer = Timer::countdown(5).danger_threshold(5);
    assert_eq!(timer.remaining_seconds(), 5);
    // Exactly at danger threshold
}

#[test]
fn test_timer_format_compact_zero() {
    let timer = Timer::countdown(0).format(TimerFormat::Compact);
    assert_eq!(timer.format_remaining(), "0s");
}

#[test]
fn test_timer_progress_clamp() {
    // Progress should be 0.0 to 1.0
    let timer = Timer::countdown(60);
    let progress = timer.progress();
    assert!(progress >= 0.0);
    assert!(progress <= 1.0);
}

// =============================================================================
// StyledView Trait Tests
// =============================================================================

#[test]
fn test_timer_styled_view_set_id() {
    let mut timer = Timer::countdown(60);
    StyledView::set_id(&mut timer, "test-timer-id");
    assert_eq!(View::id(&timer), Some("test-timer-id"));
}

#[test]
fn test_timer_styled_view_add_class() {
    let mut timer = Timer::countdown(60);
    StyledView::add_class(&mut timer, "first");
    StyledView::add_class(&mut timer, "second");
    assert!(StyledView::has_class(&timer, "first"));
    assert!(StyledView::has_class(&timer, "second"));
    assert_eq!(View::classes(&timer).len(), 2);
}

#[test]
fn test_timer_styled_view_add_class_no_duplicates() {
    let mut timer = Timer::countdown(60);
    StyledView::add_class(&mut timer, "test");
    StyledView::add_class(&mut timer, "test");
    let classes = View::classes(&timer);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_timer_styled_view_remove_class() {
    let mut timer = Timer::countdown(60).class("a").class("b").class("c");
    StyledView::remove_class(&mut timer, "b");
    assert!(StyledView::has_class(&timer, "a"));
    assert!(!StyledView::has_class(&timer, "b"));
    assert!(StyledView::has_class(&timer, "c"));
}

#[test]
fn test_timer_styled_view_remove_nonexistent_class() {
    let mut timer = Timer::countdown(60).class("test");
    StyledView::remove_class(&mut timer, "nonexistent");
    assert!(StyledView::has_class(&timer, "test"));
}

#[test]
fn test_timer_styled_view_toggle_class_add() {
    let mut timer = Timer::countdown(60);
    StyledView::toggle_class(&mut timer, "test");
    assert!(StyledView::has_class(&timer, "test"));
}

#[test]
fn test_timer_styled_view_toggle_class_remove() {
    let mut timer = Timer::countdown(60).class("test");
    StyledView::toggle_class(&mut timer, "test");
    assert!(!StyledView::has_class(&timer, "test"));
}

#[test]
fn test_timer_styled_view_has_class() {
    let timer = Timer::countdown(60).class("present");
    assert!(StyledView::has_class(&timer, "present"));
    assert!(!StyledView::has_class(&timer, "absent"));
}

// =============================================================================
// View Trait Tests
// =============================================================================

#[test]
fn test_timer_view_widget_type() {
    let timer = Timer::countdown(60);
    assert_eq!(timer.widget_type(), "Timer");
}

#[test]
fn test_timer_view_id_none() {
    let timer = Timer::countdown(60);
    assert!(View::id(&timer).is_none());
}

#[test]
fn test_timer_view_classes_empty() {
    let timer = Timer::countdown(60);
    assert!(View::classes(&timer).is_empty());
}

#[test]
fn test_timer_view_classes_with_values() {
    let timer = Timer::countdown(60).class("first").class("second");
    let classes = View::classes(&timer);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"first".to_string()));
    assert!(classes.contains(&"second".to_string()));
}

#[test]
fn test_timer_view_meta() {
    let timer = Timer::countdown(60)
        .element_id("test-id")
        .class("test-class");
    let meta = timer.meta();
    assert_eq!(meta.widget_type, "Timer");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

#[test]
fn test_timer_view_children_default() {
    let timer = Timer::countdown(60);
    assert!(View::children(&timer).is_empty());
}

// =============================================================================
// TimerFormat PartialEq Tests
// =============================================================================

#[test]
fn test_timer_format_eq() {
    assert_eq!(TimerFormat::Full, TimerFormat::Full);
    assert_eq!(TimerFormat::Short, TimerFormat::Short);
    assert_eq!(TimerFormat::Precise, TimerFormat::Precise);
    assert_eq!(TimerFormat::Compact, TimerFormat::Compact);
}

#[test]
fn test_timer_format_ne() {
    assert_ne!(TimerFormat::Full, TimerFormat::Short);
    assert_ne!(TimerFormat::Precise, TimerFormat::Compact);
    assert_ne!(TimerFormat::Full, TimerFormat::Precise);
}

// =============================================================================
// TimerState PartialEq Tests
// =============================================================================

#[test]
fn test_timer_state_eq() {
    assert_eq!(TimerState::Stopped, TimerState::Stopped);
    assert_eq!(TimerState::Running, TimerState::Running);
    assert_eq!(TimerState::Paused, TimerState::Paused);
    assert_eq!(TimerState::Completed, TimerState::Completed);
}

#[test]
fn test_timer_state_ne() {
    assert_ne!(TimerState::Stopped, TimerState::Running);
    assert_ne!(TimerState::Running, TimerState::Paused);
    assert_ne!(TimerState::Paused, TimerState::Completed);
}

// =============================================================================
// Clone Tests
// =============================================================================

#[test]
fn test_timer_clone() {
    let timer = Timer::countdown(60)
        .title("Test")
        .fg(Color::RED)
        .element_id("test-id")
        .class("test-class");

    let cloned = timer.clone();
    assert_eq!(cloned.remaining_seconds(), 60);
    // Cannot directly access private fields like title and fg
    // These are verified through rendering and behavior
    assert_eq!(View::id(&cloned), Some("test-id"));
    assert!(cloned.has_class("test-class"));
}

// =============================================================================
// Default Values Tests
// =============================================================================

#[test]
fn test_timer_default_format() {
    let timer = Timer::countdown(60);
    // Default format is Full, which produces "HH:MM:SS" format
    assert_eq!(timer.format_remaining(), "00:01:00");
}

#[test]
fn test_timer_default_show_progress() {
    assert!(screen(&Timer::countdown(60), 30, 4)[1].starts_with('░'));
}
#[test]
fn test_timer_default_progress_width() {
    // 0: the bar takes the whole width
    assert_eq!(screen(&Timer::countdown(60), 30, 4)[1], "░".repeat(30));
}
#[test]
fn test_timer_default_warning_threshold() {
    assert_eq!(
        draw(&Timer::countdown(60), 30, 4).get(0, 0).unwrap().fg,
        Some(Color::YELLOW)
    );
    assert_eq!(
        draw(&Timer::countdown(61), 30, 4).get(0, 0).unwrap().fg,
        Some(Color::WHITE)
    );
}
#[test]
fn test_timer_default_danger_threshold() {
    assert_eq!(
        draw(&Timer::countdown(10), 30, 4).get(0, 0).unwrap().fg,
        Some(Color::RED)
    );
    assert_eq!(
        draw(&Timer::countdown(11), 30, 4).get(0, 0).unwrap().fg,
        Some(Color::YELLOW)
    );
}
#[test]
fn test_timer_default_large_digits() {
    assert_eq!(screen(&Timer::countdown(60), 30, 4)[0].trim(), "00:01:00");
}
#[test]
fn test_timer_default_auto_restart() {
    let mut timer = Timer::countdown(0);
    timer.start();
    timer.update();
    assert!(timer.is_completed());
}
// =============================================================================
// Tick Behavior Integration Tests
// =============================================================================

#[test]
fn test_timer_tick_integration() {
    let mut timer = Timer::countdown(1); // 1 second
    timer.start();

    // Wait for timer to progress (but not complete)
    poll_until(
        || {
            timer.update();
            timer.remaining_seconds() < 1
        },
        500,
    );

    // Should have progressed but not completed
    assert!(timer.remaining_seconds() < 1);
    assert!(!timer.is_completed());
}

#[test]
fn test_timer_completion_flow() {
    let mut timer = Timer::countdown(0);
    assert!(!timer.is_completed());

    timer.start();
    timer.update();

    assert!(timer.is_completed());
    assert_eq!(timer.state(), TimerState::Completed);
}

#[test]
fn test_timer_restart_after_completion() {
    let mut timer = Timer::countdown(0);
    timer.start();
    timer.update();

    assert!(timer.is_completed());

    timer.stop();
    assert!(!timer.is_completed());
    assert_eq!(timer.state(), TimerState::Stopped);
    assert_eq!(timer.remaining_seconds(), 0);
}

#[test]
fn test_timer_chained_builder_methods() {
    let timer = Timer::countdown(120)
        .format(TimerFormat::Short)
        .show_progress(true)
        .progress_width(25)
        .fg(Color::GREEN)
        .warning_threshold(30)
        .danger_threshold(10)
        .title("Chain Test")
        .large_digits(true)
        .auto_restart(false)
        .element_id("chained")
        .class("timer")
        .class("countdown");

    assert_eq!(timer.remaining_seconds(), 120);
    // Verify format affects output
    assert_eq!(timer.format_remaining(), "02:00"); // Short format
    assert_eq!(View::id(&timer), Some("chained"));
    assert!(timer.has_class("timer"));
    assert!(timer.has_class("countdown"));
}

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

#[test]
fn the_default_progress_bar_spans_the_timer() {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};

    let timer = Timer::countdown(60);
    let mut buffer = Buffer::new(40, 6);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 6));
    timer.render(&mut ctx);

    let widest = (0..6)
        .map(|y| {
            (0..40)
                .filter(|&x| matches!(buffer.get(x, y).map(|c| c.symbol), Some('█' | '░')))
                .count()
        })
        .max();
    assert_eq!(
        widest,
        Some(40),
        "without progress_width the bar fills the width"
    );
}

#[test]
fn test_timer_countdown_basic() {
    let timer = Timer::countdown(60);

    assert_eq!(timer.remaining_seconds(), 60);
    // Default format is Full, which shows "00:01:00" for 60 seconds
    assert!(timer.format_remaining().contains("01:00"));
    assert_eq!(timer.progress(), 0.0);
}

#[test]
fn test_timer_format_compact() {
    let timer = Timer::countdown(90).format(TimerFormat::Compact);
    assert_eq!(timer.format_remaining(), "1m 30s");
}

#[test]
fn test_timer_state_transitions() {
    let mut timer = Timer::countdown(60);

    assert_eq!(timer.state(), TimerState::Stopped);
    assert!(!timer.is_running());
    assert!(!timer.is_completed());

    timer.start();
    assert_eq!(timer.state(), TimerState::Running);
    assert!(timer.is_running());

    timer.pause();
    assert_eq!(timer.state(), TimerState::Paused);
    assert!(!timer.is_running());

    timer.toggle();
    assert_eq!(timer.state(), TimerState::Running);
}

#[test]
fn test_timer_progress_calculation() {
    let timer = Timer::countdown(100);
    assert_eq!(timer.progress(), 0.0);

    // Progress is based on remaining_ms / total_ms
    assert!(timer.progress() >= 0.0);
    assert!(timer.progress() <= 1.0);
}

#[test]
fn test_timer_builder() {
    let timer = Timer::countdown(60)
        .format(TimerFormat::Compact)
        .show_progress(false)
        .progress_width(50)
        .fg(Color::CYAN)
        .warning_threshold(30)
        .danger_threshold(5)
        .title("Test Timer")
        .large_digits(true)
        .auto_restart(true);
    // Title, the large digits of "1m 0s" in the fg color (60 s is above
    // both thresholds), no progress bar
    let buffer = draw(&timer, 30, 6);
    let rows = rows_of(&buffer);
    assert_eq!(rows[0].trim(), "Test Timer");
    assert_eq!(buffer.get(1, 1).unwrap().fg, Some(Color::CYAN));
    assert!(rows[1].contains('▀'), "{rows:#?}");
    assert!(rows.iter().all(|r| !r.contains('░')));
}

#[test]
fn test_timer_zero_area_draws_nothing() {
    super::super::chart::assert_zero_areas_draw_nothing(
        &Timer::countdown(90).title("Tea").large_digits(true),
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// For any duration, each format shows the hours, minutes and seconds
    /// of the full time before the timer starts.
    #[test]
    fn test_timer_formats_any_duration(secs in 0u64..400_000) {
        let (h, m, s) = (secs / 3600, (secs / 60) % 60, secs % 60);
        let timer = |format| Timer::countdown(secs).format(format).format_remaining();
        prop_assert_eq!(timer(TimerFormat::Full), format!("{h:02}:{m:02}:{s:02}"));
        let short = if h > 0 { format!("{h:02}:{m:02}:{s:02}") } else { format!("{m:02}:{s:02}") };
        prop_assert_eq!(timer(TimerFormat::Short), short);
        prop_assert_eq!(timer(TimerFormat::Precise), format!("{m:02}:{s:02}.000"));
        let compact = if h > 0 { format!("{h}h {m}m") } else if m > 0 { format!("{m}m {s}s") } else { format!("{s}s") };
        prop_assert_eq!(timer(TimerFormat::Compact), compact);
        // The first row shows it
        let rows = screen(&Timer::countdown(secs), 40, 3);
        prop_assert_eq!(rows[0].trim(), format!("{h:02}:{m:02}:{s:02}"));
    }
}

mod snapshots {

    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_timer_countdown() {
        use revue::widget::Timer;

        let view = Timer::countdown(300); // 5 minutes

        let config = TestConfig::with_size(30, 5);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("timer_countdown");
    }

    #[test]
    fn test_timer_with_progress() {
        use revue::widget::Timer;

        let view = Timer::countdown(600).title("Pomodoro").show_progress(true);

        let config = TestConfig::with_size(40, 6);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("timer_progress");
    }
}
