//! Widget snapshot tests part A (Accordion, Calendar, BarChart, Notification, Modal, Pagination)

#![allow(unused_imports)]

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};
use revue::widget::{Accordion, Breadcrumb, Calendar, Gauge, Grid, Rating, Slider, Switch};

#[test]
fn test_accordion_basic() {
    use revue::widget::AccordionSection;

    let view = Accordion::new()
        .section(AccordionSection::new("Section 1").content("Content for section 1"))
        .section(AccordionSection::new("Section 2").content("Content for section 2"))
        .section(AccordionSection::new("Section 3").content("Content for section 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("accordion_basic");
}

// =============================================================================
// Calendar Widget Tests
// =============================================================================

#[test]
fn test_calendar_basic() {
    let view = Calendar::new(2024, 6); // June 2024

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("calendar_basic");
}

// =============================================================================
// BarChart Widget Tests
// =============================================================================

#[test]
fn test_barchart_basic() {
    let view = BarChart::new()
        .bar("Mon", 10.0)
        .bar("Tue", 20.0)
        .bar("Wed", 15.0)
        .bar("Thu", 25.0)
        .bar("Fri", 18.0);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("barchart_basic");
}

// =============================================================================
// Notification Widget Tests
// =============================================================================

#[test]
fn test_notification_basic() {
    use revue::widget::NotificationCenter;

    // NotificationCenter is the widget, Notification is a data struct
    let view = NotificationCenter::new();

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("notification_basic");
}

// =============================================================================
// Modal Widget Tests
// =============================================================================

#[test]
fn test_modal_basic() {
    let mut modal = Modal::new()
        .title("Confirm Action")
        .content("Are you sure you want to proceed?")
        .ok();
    modal.show();

    let mut app = TestApp::new(modal);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("modal_basic");
}

// =============================================================================
// Pagination Widget Tests
// =============================================================================

#[test]
fn test_pagination_basic() {
    let view = Pagination::new(10).current(3);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("pagination_basic");
}
