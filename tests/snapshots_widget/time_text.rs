//! Time and text widget snapshot tests (Timeline, ThemePicker, Timer, BigText)

#![allow(unused_imports)]

use revue::prelude::*;
use revue::style::Color;
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

// =============================================================================
// ThemePicker Widget Tests
// =============================================================================

#[test]
fn test_theme_picker_basic() {
    use revue::widget::ThemePicker;

    let view = ThemePicker::new();

    let config = TestConfig::with_size(40, 8);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("theme_picker_basic");
}

#[test]
fn test_theme_picker_compact() {
    use revue::widget::ThemePicker;

    let view = ThemePicker::new().compact(true);

    let config = TestConfig::with_size(30, 5);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("theme_picker_compact");
}

// =============================================================================
// Timer Widget Tests
// =============================================================================

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

#[test]
fn test_stopwatch_basic() {
    use revue::widget::Stopwatch;

    let view = Stopwatch::new();

    let config = TestConfig::with_size(30, 5);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("stopwatch_basic");
}

// =============================================================================
// BigText Widget Tests (OSC 66 / Figlet)
// =============================================================================

#[test]
fn test_bigtext_h1() {
    use revue::widget::BigText;

    let view = BigText::h1("Hello").force_figlet(true);

    let config = TestConfig::with_size(80, 10);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("bigtext_h1");
}

#[test]
fn test_bigtext_h2() {
    use revue::widget::BigText;

    let view = BigText::h2("World").force_figlet(true);

    let config = TestConfig::with_size(80, 10);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("bigtext_h2");
}

#[test]
fn test_bigtext_h3() {
    use revue::widget::BigText;

    let view = BigText::h3("Test").force_figlet(true);

    let config = TestConfig::with_size(60, 6);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("bigtext_h3");
}

#[test]
fn test_bigtext_tiers() {
    use revue::widget::BigText;

    let view = vstack()
        .child(BigText::h1("H1").force_figlet(true))
        .child(BigText::h2("H2").force_figlet(true))
        .child(BigText::h3("H3").force_figlet(true));

    let config = TestConfig::with_size(80, 24);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("bigtext_tiers");
}

#[test]
fn test_bigtext_with_color() {
    use revue::widget::BigText;

    let view = BigText::h1("Color").force_figlet(true).fg(Color::CYAN);

    let config = TestConfig::with_size(80, 10);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("bigtext_color");
}

#[test]
fn test_bigtext_small_tiers() {
    use revue::widget::BigText;

    // H4-H6 use Mini font
    let view = vstack()
        .child(BigText::h4("H4").force_figlet(true))
        .child(BigText::h5("H5").force_figlet(true))
        .child(BigText::h6("H6").force_figlet(true));

    let config = TestConfig::with_size(40, 12);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("bigtext_small_tiers");
}
