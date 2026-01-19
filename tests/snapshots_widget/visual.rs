//! Visual widget snapshot tests (Divider, Gauge, Sparkline, Spinner, Tag, Toast, Avatar, Breadcrumb, Rating)

#![allow(unused_imports)]

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};
use revue::widget::{Accordion, Breadcrumb, Gauge, Rating, Slider, Switch};

#[test]
fn test_divider_horizontal() {
    let view = vstack()
        .child(text("Above"))
        .child(Divider::new())
        .child(text("Below"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("divider_horizontal");
}

#[test]
fn test_divider_with_label() {
    let view = vstack()
        .child(text("Section 1"))
        .child(Divider::new().label("OR"))
        .child(text("Section 2"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("divider_with_label");
}

// =============================================================================
// Gauge Widget Tests
// =============================================================================

#[test]
fn test_gauge_basic() {
    let view = vstack()
        .gap(1)
        .child(Gauge::new().value(0.25).label("CPU"))
        .child(Gauge::new().value(0.75).label("Memory"))
        .child(Gauge::new().value(0.50).label("Disk"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("gauge_basic");
}

// =============================================================================
// Sparkline Widget Tests
// =============================================================================

#[test]
fn test_sparkline_basic() {
    let view = sparkline([1.0, 4.0, 2.0, 8.0, 3.0, 6.0, 5.0, 7.0]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("sparkline_basic");
}

// =============================================================================
// Spinner Widget Tests
// =============================================================================

#[test]
fn test_spinner_basic() {
    let view = hstack()
        .gap(1)
        .child(Spinner::new())
        .child(text("Loading..."));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("spinner_basic");
}

// =============================================================================
// Tag Widget Tests
// =============================================================================

#[test]
fn test_tag_basic() {
    let view = hstack()
        .gap(1)
        .child(Tag::new("Rust"))
        .child(Tag::new("TUI"))
        .child(Tag::new("CSS"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tag_basic");
}

#[test]
fn test_tag_colors() {
    let view = hstack()
        .gap(1)
        .child(Tag::new("Success").green())
        .child(Tag::new("Warning").yellow())
        .child(Tag::new("Error").red());

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tag_colors");
}

// =============================================================================
// Toast Widget Tests
// =============================================================================

#[test]
fn test_toast_variants() {
    let view = vstack()
        .gap(1)
        .child(Toast::success("Operation completed!"))
        .child(Toast::error("An error occurred"))
        .child(Toast::warning("Please check your input"))
        .child(Toast::info("New updates available"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("toast_variants");
}

// =============================================================================
// Avatar Widget Tests
// =============================================================================

#[test]
fn test_avatar_basic() {
    let view = hstack()
        .gap(2)
        .child(Avatar::new("JD"))
        .child(Avatar::new("AB"))
        .child(Avatar::new("XY"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("avatar_basic");
}

// =============================================================================
// Breadcrumb Widget Tests
// =============================================================================

#[test]
fn test_breadcrumb_basic() {
    let view = Breadcrumb::new()
        .push("Home")
        .push("Products")
        .push("Electronics")
        .push("Phones");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("breadcrumb_basic");
}

// =============================================================================
// Rating Widget Tests
// =============================================================================

#[test]
fn test_rating_basic() {
    let view = vstack()
        .gap(1)
        .child(Rating::new().max_value(5).value(3.0))
        .child(Rating::new().max_value(5).value(5.0))
        .child(Rating::new().max_value(5).value(0.0));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("rating_basic");
}
