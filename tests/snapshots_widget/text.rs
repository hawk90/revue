//! Text widget snapshot tests

#![allow(unused_imports)]

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};

#[test]
fn test_text_simple() {
    let view = text("Hello, World!");
    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_simple");
}

#[test]
fn test_text_multiline() {
    let view = vstack()
        .child(text("Line 1"))
        .child(text("Line 2"))
        .child(text("Line 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_multiline");
}

#[test]
fn test_text_formatting() {
    let view = vstack()
        .child(Text::new("Normal text"))
        .child(Text::heading("Heading"))
        .child(Text::muted("Muted text"))
        .child(Text::success("Success!"))
        .child(Text::error("Error!"))
        .child(Text::info("Info"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_formatting");
}

#[test]
fn test_text_alignment() {
    let config = TestConfig::with_size(40, 10);
    let view = vstack()
        .child(Text::new("Left aligned").align(Alignment::Left))
        .child(Text::new("Centered").align(Alignment::Center))
        .child(Text::new("Right aligned").align(Alignment::Right));

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_alignment");
}

#[test]
fn test_text_reverse() {
    let view = vstack()
        .child(Text::new("Normal text"))
        .child(Text::new("Reversed text").reverse())
        .child(Text::new("Bold + Reversed").bold().reverse());

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_reverse");
}

#[test]
fn test_text_justify() {
    let config = TestConfig::with_size(30, 5);
    let view = vstack()
        .child(Text::new("Hello World Test").align(Alignment::Justify))
        .child(Text::new("A B C D E").align(Alignment::Justify))
        .child(Text::new("SingleWord").align(Alignment::Justify));

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_justify");
}
