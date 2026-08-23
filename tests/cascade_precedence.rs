//! Contracts for how a widget's own colors rank against the stylesheet.
//!
//! Three sources can decide a widget's color, and the order matters:
//!
//! | | | CSS analogue |
//! |---|---|---|
//! | `.fg(色)` on the builder | highest | inline `style=` attribute |
//! | the matched stylesheet rule | middle | author stylesheet |
//! | the widget's own default | lowest | user-agent stylesheet |
//!
//! A disabled widget's grey belongs in the bottom row. It used to sit above all
//! three, returned before `css_style` was so much as consulted - so
//! `:disabled { color: ... }` was computed by the cascade and thrown away.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::WidgetState;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
const GREEN: Color = Color {
    r: 0,
    g: 128,
    b: 0,
    a: 255,
};

fn css_with_color(color: Color) -> revue::style::Style {
    let mut style = revue::style::Style::default();
    style.visual.color = color;
    style
}

fn css_with_background(color: Color) -> revue::style::Style {
    let mut style = revue::style::Style::default();
    style.visual.background = color;
    style
}

// ---------------------------------------------------------------------------
// The order itself
// ---------------------------------------------------------------------------

#[test]
fn the_stylesheet_beats_the_widgets_default() {
    let state = WidgetState::new();
    let css = css_with_color(RED);

    assert_eq!(state.resolve_fg(Some(&css), GREEN), RED);
}

#[test]
fn an_inline_override_beats_the_stylesheet() {
    let mut state = WidgetState::new();
    state.set_fg(Some(BLUE));
    let css = css_with_color(RED);

    assert_eq!(state.resolve_fg(Some(&css), GREEN), BLUE);
}

#[test]
fn the_default_is_used_when_nothing_else_says_anything() {
    let state = WidgetState::new();

    assert_eq!(state.resolve_fg(None, GREEN), GREEN);
}

// ---------------------------------------------------------------------------
// Where `disabled` sits
// ---------------------------------------------------------------------------

/// The change: the grey is the bottom row, not the top one.
#[test]
fn the_stylesheet_beats_the_disabled_grey() {
    let mut state = WidgetState::new();
    state.set_disabled(true);
    let css = css_with_color(RED);

    assert_eq!(
        state.resolve_fg(Some(&css), GREEN),
        RED,
        "the disabled grey silenced the stylesheet"
    );
}

#[test]
fn the_disabled_grey_still_beats_the_widgets_own_default() {
    let mut state = WidgetState::new();
    state.set_disabled(true);

    assert_ne!(
        state.resolve_fg(None, GREEN),
        GREEN,
        "a disabled widget should not look like an enabled one"
    );
}

#[test]
fn the_same_holds_for_the_background() {
    let mut state = WidgetState::new();
    state.set_disabled(true);
    let css = css_with_background(RED);

    assert_eq!(state.resolve_bg(Some(&css), GREEN), RED);
    assert_ne!(state.resolve_bg(None, GREEN), GREEN);
}

#[test]
fn an_inline_override_beats_the_disabled_grey_too() {
    let mut state = WidgetState::new();
    state.set_disabled(true);
    state.set_fg(Some(BLUE));

    assert_eq!(state.resolve_fg(None, GREEN), BLUE);
}

// ---------------------------------------------------------------------------
// Interaction effects
// ---------------------------------------------------------------------------

/// A disabled widget does not react to the pointer, so it takes the resolved
/// colors without the hover/press tint. That part of the short-circuit was
/// right and is kept.
#[test]
fn a_disabled_widget_takes_no_interaction_tint() {
    let mut hovered = WidgetState::new();
    hovered.set_disabled(true);
    hovered.set_hovered(true);
    hovered.set_pressed(true);

    let mut idle = WidgetState::new();
    idle.set_disabled(true);

    assert_eq!(
        hovered.resolve_colors_interactive(None, GREEN, GREEN),
        idle.resolve_colors_interactive(None, GREEN, GREEN),
        "a disabled widget reacted to the pointer"
    );
}

/// And an enabled one still does.
#[test]
fn an_enabled_widget_still_takes_one() {
    let mut hovered = WidgetState::new();
    hovered.set_hovered(true);

    let idle = WidgetState::new();

    assert_ne!(
        hovered.resolve_colors_interactive(None, GREEN, GREEN).1,
        idle.resolve_colors_interactive(None, GREEN, GREEN).1,
    );
}

#[test]
fn a_disabled_widget_still_takes_the_stylesheets_colors() {
    let mut state = WidgetState::new();
    state.set_disabled(true);
    let css = css_with_color(RED);

    assert_eq!(
        state.resolve_colors_interactive(Some(&css), GREEN, GREEN).0,
        RED
    );
}

// ---------------------------------------------------------------------------
// Through the real pipeline
// ---------------------------------------------------------------------------

struct Form {
    disabled: bool,
}

impl View for Form {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(
                Button::new("Save")
                    .disabled(self.disabled)
                    .element_id("save"),
            )
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Form"
    }
    fn id(&self) -> Option<&str> {
        Some("form")
    }
}

/// The end-to-end shape: a stylesheet rule that only matches while the widget
/// is disabled, reaching the painted cell.
#[test]
fn a_disabled_rule_repaints_the_widget_when_it_is_disabled() {
    let mut h = PipelineHarness::with_css("#save:disabled { color: #ff0000; }", 20, 6)
        .dom_from_render(true);
    // The button pads its label, so the first glyph of "Save" is at x=2.
    h.draw(&Form { disabled: false });
    assert_ne!(h.buffer().get(2, 0).and_then(|c| c.fg), Some(RED));

    h.draw(&Form { disabled: true });
    assert_eq!(h.buffer().get(2, 0).and_then(|c| c.fg), Some(RED));

    h.draw(&Form { disabled: false });
    assert_ne!(
        h.buffer().get(2, 0).and_then(|c| c.fg),
        Some(RED),
        "the disabled styling outlived the disabled state"
    );
}
