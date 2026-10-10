//! Screen-level snapshot tests: several widgets composed into one screen,
//! and whole screens at extreme sizes. Snapshot files are in tests/snapshots/.
//! The single-widget snapshots live with each widget's tests in tests/widget/.
//! Update snapshots: REVUE_UPDATE_SNAPSHOTS=1 cargo test

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};

#[test]
fn test_card_layout() {
    let view = Border::panel().title("Card Title").child(
        vstack()
            .gap(1)
            .child(Text::heading("Welcome"))
            .child(text("This is a card with multiple elements."))
            .child(
                hstack()
                    .child(text("[OK]"))
                    .child(text(" "))
                    .child(text("[Cancel]")),
            ),
    );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("card_layout");
}

#[test]
fn test_form_layout() {
    let view = Border::single().title("Login Form").child(
        vstack()
            .gap(1)
            .child(text("Username: "))
            .child(Border::single().child(text("admin")))
            .child(text("Password: "))
            .child(Border::single().child(text("****")))
            .child(text(""))
            .child(text("[Login]")),
    );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("form_layout");
}

#[test]
fn test_dashboard_layout() {
    let config = TestConfig::with_size(60, 20);
    // The panels are content-sized (bordered text), so they say they want the
    // rest: the row takes the height the header leaves, and the two panels
    // split its width.
    let view = vstack()
        .child(
            Border::double()
                .title("Dashboard")
                .child(text("Application Status: Running")),
        )
        .child_flex(
            hstack()
                .child_flex(
                    Border::single().title("Stats").child(
                        vstack()
                            .child(text("CPU: 45%"))
                            .child(text("Memory: 2.1GB"))
                            .child(text("Uptime: 2h 15m")),
                    ),
                    1.0,
                )
                .child_flex(
                    Border::single().title("Logs").child(
                        vstack()
                            .child(Text::info("[INFO] Server started"))
                            .child(Text::success("[OK] Connected"))
                            .child(Text::error("[ERR] Failed to load")),
                    ),
                    1.0,
                ),
            1.0,
        );

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("dashboard_layout");
}

#[test]
fn test_deeply_nested() {
    let view = vstack().child(vstack().child(vstack().child(vstack().child(text("Deep")))));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("deeply_nested");
}

#[test]
fn test_long_text() {
    let long_text = "This is a very long line of text that might wrap or get truncated depending on the terminal width.";
    let view = text(long_text);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("long_text");
}

#[test]
fn test_special_characters() {
    let view = vstack()
        .child(text("ASCII: ABC abc 123"))
        .child(text("Symbols: !@#$%^&*()"))
        .child(text("Unicode: ✓ ✗ → ← ↑ ↓"))
        .child(text("Box: ┌─┐ │ │ └─┘"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("special_characters");
}

#[test]
fn test_small_terminal() {
    let config = TestConfig::with_size(20, 10);
    let view = Border::single()
        .title("Small")
        .child(text("Fits in small space"));

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("small_terminal");
}

#[test]
fn test_large_terminal() {
    let config = TestConfig::with_size(120, 40);
    let view = Border::double().title("Large Terminal").child(
        vstack()
            .child(text("This is a large terminal with plenty of space."))
            .child(text("We can fit much more content here."))
            .child(text("Line 3"))
            .child(text("Line 4"))
            .child(text("Line 5")),
    );

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("large_terminal");
}
