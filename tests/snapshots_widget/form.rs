//! Form widget snapshot tests (Interactive State, Button, Badge, Checkbox, Radio, Switch, Input, Slider, Select)

#![allow(unused_imports)]

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};
use revue::widget::{Button, Slider, Switch};

#[test]
fn test_focused_state() {
    // Placeholder for future focused state testing
    let view = Border::single().child(text("[Focused Element]"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("focused_state");
}

#[test]
fn test_disabled_state() {
    // Placeholder for future disabled state testing
    let view = vstack()
        .child(text("[Enabled Button]"))
        .child(Text::muted("[Disabled Button]"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("disabled_state");
}

// =============================================================================
// Button Widget Tests
// =============================================================================

#[test]
fn test_button_basic() {
    let view = button("Click Me");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("button_basic");
}

#[test]
fn test_button_variants() {
    use revue::widget::Button;
    let view = vstack()
        .gap(1)
        .child(Button::primary("Primary"))
        .child(Button::ghost("Ghost"))
        .child(Button::success("Success"))
        .child(Button::danger("Danger"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("button_variants");
}

// =============================================================================
// Badge Widget Tests
// =============================================================================

#[test]
fn test_badge_basic() {
    let view = hstack()
        .gap(1)
        .child(badge("New"))
        .child(badge("5"))
        .child(badge("Beta"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("badge_basic");
}

#[test]
fn test_badge_variants() {
    let view = vstack()
        .gap(1)
        .child(badge("Success").success())
        .child(badge("Error").error())
        .child(badge("Warning").warning())
        .child(badge("Info").info());

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("badge_variants");
}

// =============================================================================
// Checkbox Widget Tests
// =============================================================================

#[test]
fn test_checkbox_basic() {
    let view = vstack()
        .child(checkbox("Option 1"))
        .child(checkbox("Option 2"))
        .child(checkbox("Option 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("checkbox_basic");
}

#[test]
fn test_checkbox_checked() {
    let view = vstack()
        .child(checkbox("Unchecked"))
        .child(Checkbox::new("Checked").checked(true))
        .child(Checkbox::new("Disabled").disabled(true));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("checkbox_checked");
}

// =============================================================================
// Radio Widget Tests
// =============================================================================

#[test]
fn test_radio_group() {
    let view = RadioGroup::new(["Option A", "Option B", "Option C"]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("radio_group");
}

// =============================================================================
// Switch Widget Tests
// =============================================================================

#[test]
fn test_switch_basic() {
    let view = vstack()
        .gap(1)
        .child(hstack().child(Switch::new()).child(text(" Enable feature")))
        .child(
            hstack()
                .child(Switch::new().on(true))
                .child(text(" Dark mode")),
        );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("switch_basic");
}

// =============================================================================
// Input Widget Tests
// =============================================================================

#[test]
fn test_input_basic() {
    let view = vstack()
        .gap(1)
        .child(Input::new().placeholder("Enter text..."))
        .child(Input::new().value("Hello World"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("input_basic");
}

#[test]
fn test_input_with_label() {
    let view = vstack()
        .gap(1)
        .child(text("Username:"))
        .child(Input::new().placeholder("Enter username"))
        .child(text("Password:"))
        .child(Input::new().placeholder("Enter password"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("input_with_label");
}

// =============================================================================
// Slider Widget Tests
// =============================================================================

#[test]
fn test_slider_basic() {
    let view = vstack()
        .gap(1)
        .child(Slider::new().value(50.0))
        .child(Slider::new().value(25.0))
        .child(Slider::new().value(75.0));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("slider_basic");
}

// =============================================================================
// Select Widget Tests
// =============================================================================

#[test]
fn test_select_basic() {
    let view = Select::new()
        .options(vec!["Option 1", "Option 2", "Option 3"])
        .selected(0);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("select_basic");
}
