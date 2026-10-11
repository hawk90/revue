//! A content-sized stack inside another, under `css_layout`.
//!
//! The outer stack sizes the inner one before painting it, so whatever CSS
//! spacing lives *inside* the inner stack - its own `gap`, its children's
//! margins and heights, a deeper stack's - has to be in that size. When it was
//! not, the inner stack got its bare content size and lost its last rows: a
//! form whose fields had `margin-top: 1` dropped its submit button.
//!
//! The rule is written down in `docs/refactor/design-content-sized-stack.md`.

use revue::prelude::*;
use revue::testing::PipelineHarness;

/// `build()` as the whole screen.
struct Screen(fn() -> Stack);

impl View for Screen {
    fn render(&self, ctx: &mut RenderContext) {
        (self.0)().render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Screen"
    }
    fn id(&self) -> Option<&str> {
        Some("app")
    }
}

fn draw_at(css: &str, width: u16, height: u16, build: fn() -> Stack) -> String {
    let mut h = PipelineHarness::with_css(css, width, height)
        .dom_from_render(true)
        .css_layout(true);
    h.draw(&Screen(build));
    h.screen_text()
}

fn draw(css: &str, build: fn() -> Stack) -> String {
    draw_at(css, 20, 12, build)
}

/// The reported shape: a form column above a status line.
fn form() -> Stack {
    vstack()
        .child(
            vstack()
                .class("form")
                .child(Text::new("name"))
                .child(Text::new("mail"))
                .child(Text::new("send")),
        )
        .child(Text::new("status"))
}

// ---------------------------------------------------------------------------
// Spacing inside the inner stack is reserved by the outer one
// ---------------------------------------------------------------------------

#[test]
fn the_inner_stacks_css_gap_is_reserved() {
    assert_eq!(
        draw(".form { gap: 1; }", form),
        "name\n\nmail\n\nsend\nstatus"
    );
}

#[test]
fn the_inner_childrens_margins_are_reserved() {
    assert_eq!(
        draw(".form > * { margin-top: 1; }", form),
        "\nname\n\nmail\n\nsend\nstatus"
    );
}

#[test]
fn the_inner_childrens_heights_are_reserved() {
    assert_eq!(
        draw(".form > * { height: 2; }", form),
        "name\n\nmail\n\nsend\n\nstatus"
    );
}

#[test]
fn a_button_at_the_end_of_the_form_stays_on_screen() {
    fn with_button() -> Stack {
        vstack()
            .child(
                vstack()
                    .class("form")
                    .child(Text::new("name"))
                    .child(Text::new("mail"))
                    .child(Button::new("Send")),
            )
            .child(Text::new("status"))
    }
    let screen = draw(".form { gap: 1; }", with_button);
    let lines: Vec<&str> = screen.lines().collect();
    assert!(lines[4].contains("Send"), "button missing:\n{screen}");
    assert_eq!(lines[5], "status");
}

/// Two levels down: the middle stack has to count the innermost one's spacing
/// too.
#[test]
fn spacing_two_levels_down_is_reserved() {
    fn deep() -> Stack {
        vstack()
            .child(
                vstack().child(Text::new("head")).child(
                    vstack()
                        .class("inner")
                        .child(Text::new("a"))
                        .child(Text::new("b")),
                ),
            )
            .child(Text::new("tail"))
    }
    assert_eq!(
        draw(".inner { gap: 2; } .inner > * { margin-bottom: 1; }", deep),
        "head\na\n\n\n\nb\n\ntail"
    );
}

/// The inner stack's own box is folded in as before, on top of its content.
#[test]
fn the_inner_stacks_own_margin_adds_to_its_spacing() {
    assert_eq!(
        draw(".form { margin-top: 1; gap: 1; }", form),
        "\nname\n\nmail\n\nsend\nstatus"
    );
}

#[test]
fn a_hidden_inner_child_takes_no_space() {
    fn with_hidden() -> Stack {
        vstack()
            .child(
                vstack()
                    .class("form")
                    .child(Text::new("name"))
                    .child(Text::new("mail").class("gone"))
                    .child(Text::new("send")),
            )
            .child(Text::new("status"))
    }
    assert_eq!(
        draw(".form { gap: 1; } .gone { display: none; }", with_hidden),
        "name\n\nsend\nstatus"
    );
}

/// A row in a row: the inner row's gap widens it.
#[test]
fn a_nested_rows_gap_is_reserved() {
    fn rows() -> Stack {
        hstack()
            .child(
                hstack()
                    .class("pair")
                    .child(Text::new("ab"))
                    .child(Text::new("cd")),
            )
            .child(Text::new("|"))
    }
    assert_eq!(draw(".pair { gap: 2; }", rows), "ab  cd|");
}

/// The cross axis counts too: an outer row lays a column side by side with a
/// sibling, and the column's widest child includes its horizontal margin.
#[test]
fn a_nested_columns_cross_axis_margin_is_reserved() {
    fn side() -> Stack {
        hstack()
            .child(
                vstack()
                    .class("col")
                    .child(Text::new("ab"))
                    .child(Text::new("cd")),
            )
            .child(Text::new("|"))
    }
    assert_eq!(draw(".col > * { margin-left: 2; }", side), "  ab|\n  cd");
}

/// Without any CSS the nested layout is what it always was.
#[test]
fn plain_nesting_is_unchanged() {
    assert_eq!(draw("", form), "name\nmail\nsend\nstatus");
}

/// A wrapper that cannot measure styles (here `Border`) around a styled stack:
/// it may not know how much the spacing adds, so it fills rather than clip.
#[test]
fn a_wrapper_around_styled_spacing_fills_instead_of_clipping() {
    fn boxed() -> Stack {
        vstack()
            .child(
                Border::single().child(
                    vstack()
                        .class("form")
                        .child(Text::new("name"))
                        .child(Text::new("send")),
                ),
            )
            .child(Text::new("status"))
    }
    let screen = draw_at(".form { gap: 1; }", 20, 8, boxed);
    assert!(screen.contains("name"), "{screen}");
    assert!(
        screen.contains("send"),
        "the last row was clipped:\n{screen}"
    );
    assert!(screen.contains("status"), "{screen}");
}
