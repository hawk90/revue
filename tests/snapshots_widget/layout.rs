//! Layout widget snapshot tests (Stack, Border, Combined, Edge Cases, Size Variation)

#![allow(unused_imports)]

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};

#[test]
fn test_vstack_basic() {
    let view = vstack()
        .child(text("Item 1"))
        .child(text("Item 2"))
        .child(text("Item 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("vstack_basic");
}

#[test]
fn test_hstack_basic() {
    let view = hstack().child(text("A")).child(text("B")).child(text("C"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("hstack_basic");
}

#[test]
fn test_vstack_with_gap() {
    let view = vstack()
        .gap(1)
        .child(text("Item 1"))
        .child(text("Item 2"))
        .child(text("Item 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("vstack_with_gap");
}

#[test]
fn test_nested_stacks() {
    let view = vstack()
        .child(text("Header"))
        .child(
            hstack()
                .child(text("Left"))
                .child(text(" | "))
                .child(text("Right")),
        )
        .child(text("Footer"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("nested_stacks");
}

// =============================================================================
// Border Widget Tests
// =============================================================================

#[test]
fn test_border_single() {
    let view = Border::single().child(text("Bordered content"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_single");
}

#[test]
fn test_border_double() {
    let view = Border::double().child(text("Double border"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_double");
}

#[test]
fn test_border_rounded() {
    let view = Border::rounded().child(text("Rounded corners"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_rounded");
}

#[test]
fn test_border_with_title() {
    let view = Border::single()
        .title("My Title")
        .child(text("Content with title"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_with_title");
}

#[test]
fn test_border_panel() {
    let view = Border::panel()
        .title("Panel")
        .child(vstack().child(text("Line 1")).child(text("Line 2")));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_panel");
}

// =============================================================================
// Progress Widget Tests
// =============================================================================

#[test]
fn test_progress_0_percent() {
    let view = progress(0.0);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("progress_0_percent");
}

#[test]
fn test_progress_50_percent() {
    let view = progress(0.5);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("progress_50_percent");
}

#[test]
fn test_progress_100_percent() {
    let view = progress(1.0);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("progress_100_percent");
}

#[test]
fn test_progress_with_label() {
    let view = vstack()
        .child(text("Download Progress"))
        .child(progress(0.75));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("progress_with_label");
}

// =============================================================================
// Combined Widget Tests
// =============================================================================

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
    let view = vstack()
        .child(
            Border::double()
                .title("Dashboard")
                .child(text("Application Status: Running")),
        )
        .child(
            hstack()
                .child(
                    Border::single().title("Stats").child(
                        vstack()
                            .child(text("CPU: 45%"))
                            .child(text("Memory: 2.1GB"))
                            .child(text("Uptime: 2h 15m")),
                    ),
                )
                .child(
                    Border::single().title("Logs").child(
                        vstack()
                            .child(Text::info("[INFO] Server started"))
                            .child(Text::success("[OK] Connected"))
                            .child(Text::error("[ERR] Failed to load")),
                    ),
                ),
        );

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("dashboard_layout");
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_empty_vstack() {
    let view = vstack();

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("empty_vstack");
}

#[test]
fn test_empty_border() {
    let view = Border::single();

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("empty_border");
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

// =============================================================================
// Size Variation Tests
// =============================================================================

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
