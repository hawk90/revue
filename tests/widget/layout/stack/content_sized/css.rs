//! Contracts for a [`Stack::content_sized`] stack under `css_layout`.
//!
//! The stack sizes each unsized child to its content, and the child's own CSS
//! box - `height`/`width`, `min-*`/`max-*`, margins along the stack's axis - is
//! folded into that size *before* the stack lays out. Without that, the box
//! properties would be applied to the content-sized area after the fact: a
//! `margin-top` on a one-row `Text` would inset its one row to nothing, and a
//! `height: 3` would grow it over the next sibling.
//!
//! The rule is written down in `docs/refactor/design-content-sized-stack.md`.

use revue::prelude::*;
use revue::testing::PipelineHarness;

/// A column of `Text` rows, each with its text as its id.
struct Column {
    rows: Vec<&'static str>,
    content_sized: bool,
    gap: u16,
}

impl Column {
    fn content_sized(rows: &[&'static str]) -> Self {
        Self {
            rows: rows.to_vec(),
            content_sized: true,
            gap: 0,
        }
    }

    fn equal_share(rows: &[&'static str]) -> Self {
        Self {
            content_sized: false,
            ..Self::content_sized(rows)
        }
    }

    fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }
}

impl View for Column {
    fn render(&self, ctx: &mut RenderContext) {
        let mut stack = vstack().content_sized(self.content_sized).gap(self.gap);
        for row in &self.rows {
            stack = stack.child(Text::new(*row).element_id(*row));
        }
        stack.render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Column"
    }
    fn id(&self) -> Option<&str> {
        Some("app")
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 8)
        .dom_from_render(true)
        .css_layout(true)
}

fn draw(css: &str, view: &impl View) -> String {
    let mut h = harness(css);
    h.draw(view);
    h.screen_text()
}

// ---------------------------------------------------------------------------
// The box is folded into the content size
// ---------------------------------------------------------------------------

/// The reported bug: the margin used to inset a one-row area to nothing.
#[test]
fn a_margin_top_pushes_the_text_down_and_keeps_it_visible() {
    assert_eq!(
        draw(
            "#AAAA { margin-top: 2; }",
            &Column::content_sized(&["AAAA", "BBBB"])
        ),
        "\n\nAAAA\nBBBB"
    );
}

#[test]
fn a_margin_bottom_pushes_the_next_sibling_down() {
    assert_eq!(
        draw(
            "#AAAA { margin-bottom: 1; }",
            &Column::content_sized(&["AAAA", "BBBB"])
        ),
        "AAAA\n\nBBBB"
    );
}

/// Cross-axis margins are left to the box model, as before.
#[test]
fn a_cross_axis_margin_still_insets_the_box() {
    assert_eq!(
        draw(
            "#AAAA { margin-left: 2; }",
            &Column::content_sized(&["AAAA", "BBBB"])
        ),
        "  AAAA\nBBBB"
    );
}

/// The other half of the bug: the height used to grow a one-row area over the
/// next sibling instead of reserving rows for itself.
#[test]
fn a_height_reserves_its_rows_and_the_next_sibling_starts_after_them() {
    assert_eq!(
        draw(
            "#AAAA { height: 3; }",
            &Column::content_sized(&["AAAA", "BBBB"])
        ),
        "AAAA\n\n\nBBBB"
    );
}

#[test]
fn a_height_and_margins_add_up() {
    assert_eq!(
        draw(
            "#AAAA { height: 2; margin-top: 1; margin-bottom: 1; }",
            &Column::content_sized(&["AAAA", "BBBB"])
        ),
        "\nAAAA\n\n\nBBBB"
    );
}

#[test]
fn a_min_height_floors_the_content_size() {
    assert_eq!(
        draw(
            "#AAAA { min-height: 2; }",
            &Column::content_sized(&["AAAA", "BBBB"])
        ),
        "AAAA\n\nBBBB"
    );
}

#[test]
fn a_max_height_caps_the_content_size() {
    /// A three-row stack, nested.
    struct Nested;
    impl View for Nested {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .content_sized(true)
                .child(
                    vstack()
                        .content_sized(true)
                        .element_id("group")
                        .child(Text::new("X"))
                        .child(Text::new("Y"))
                        .child(Text::new("Z")),
                )
                .child(Text::new("BBBB"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Nested"
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }

    assert_eq!(draw("", &Nested), "X\nY\nZ\nBBBB");
    assert_eq!(draw("#group { max-height: 2; }", &Nested), "X\nY\nBBBB");
}

/// `height` then `max-height`, in that order - as the box model does it.
#[test]
fn a_max_height_caps_an_explicit_height() {
    assert_eq!(
        draw(
            "#AAAA { height: 5; max-height: 2; }",
            &Column::content_sized(&["AAAA", "BBBB"])
        ),
        "AAAA\n\nBBBB"
    );
}

/// In a row the main axis is the width.
#[test]
fn a_row_folds_width_and_horizontal_margins() {
    struct Row;
    impl View for Row {
        fn render(&self, ctx: &mut RenderContext) {
            hstack()
                .content_sized(true)
                .child(Text::new("AA").element_id("a"))
                .child(Text::new("BB").element_id("b"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Row"
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }

    assert_eq!(draw("", &Row), "AABB");
    assert_eq!(draw("#a { width: 4; }", &Row), "AA  BB");
    assert_eq!(
        draw("#a { margin-left: 1; margin-right: 2; }", &Row),
        " AA  BB"
    );
}

// ---------------------------------------------------------------------------
// display: none
// ---------------------------------------------------------------------------

#[test]
fn a_hidden_child_takes_no_space() {
    assert_eq!(
        draw(
            "#AAAA { display: none; }",
            &Column::content_sized(&["AAAA", "BBBB"])
        ),
        "BBBB"
    );
}

/// Nor a gap: hiding the middle child must not leave two gaps behind.
#[test]
fn a_hidden_child_takes_no_gap() {
    assert_eq!(
        draw("", &Column::content_sized(&["AAAA", "BBBB", "CCCC"]).gap(1)),
        "AAAA\n\nBBBB\n\nCCCC"
    );
    assert_eq!(
        draw(
            "#BBBB { display: none; } #CCCC { color: rgb(0, 255, 0); }",
            &Column::content_sized(&["AAAA", "BBBB", "CCCC"]).gap(1)
        ),
        "AAAA\n\nCCCC"
    );
}

/// The sibling after a hidden one still gets its own style - the peek and the
/// paint cursor agree.
#[test]
fn the_sibling_after_a_hidden_child_keeps_its_own_style() {
    let mut h = harness("#BBBB { display: none; } #CCCC { color: rgb(0, 255, 0); }");
    h.draw(&Column::content_sized(&["AAAA", "BBBB", "CCCC"]));
    assert_eq!(h.screen_text(), "AAAA\nCCCC");
    assert_eq!(
        h.buffer().get(0, 1).and_then(|c| c.fg),
        Some(Color::rgb(0, 255, 0))
    );
}

// ---------------------------------------------------------------------------
// Precedence and guards
// ---------------------------------------------------------------------------

/// Builder over stylesheet: `child_sized` decides the slot, CSS `height` does
/// not move the next sibling.
#[test]
fn a_builder_child_sized_beats_a_css_height() {
    struct Sized;
    impl View for Sized {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .content_sized(true)
                .child_sized(Text::new("AAAA").element_id("AAAA"), 2)
                .child(Text::new("BBBB").element_id("BBBB"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Sized"
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }

    assert_eq!(draw("#AAAA { height: 5; }", &Sized), "AAAA\n\nBBBB");
    // A margin does not grow the builder's slot either; the box model insets
    // inside it, as it does in an equal-share stack.
    assert_eq!(draw("#AAAA { margin-top: 1; }", &Sized), "\nAAAA\nBBBB");
}

/// A percentage has no basis in a content-sized slot, so such a child fills
/// like an unmeasured one and the box model resolves the percentage against
/// its share - exactly as an equal-share stack does.
#[test]
fn a_percentage_height_makes_the_child_fill() {
    // 8 rows: BBBB measures 1, AAAA fills the other 7 and keeps 50% of them.
    let mut h = harness("#AAAA { height: 50%; }");
    h.draw(&Column::content_sized(&["AAAA", "BBBB"]));
    assert_eq!(h.screen_text(), "AAAA\n\n\n\n\n\n\nBBBB");
}

/// Equal-share stacks are untouched: the box model still adjusts the share.
#[test]
fn an_equal_share_stack_behaves_as_before() {
    // 8 rows, two children: 4 rows each; the margin insets the first share.
    assert_eq!(
        draw(
            "#AAAA { margin-bottom: 1; }",
            &Column::equal_share(&["AAAA", "BBBB"])
        ),
        "AAAA\n\n\n\nBBBB"
    );
    assert_eq!(
        draw(
            "#AAAA { margin-top: 2; }",
            &Column::equal_share(&["AAAA", "BBBB"])
        ),
        "\n\nAAAA\n\nBBBB"
    );
    // And a hidden child still holds its share.
    assert_eq!(
        draw(
            "#AAAA { display: none; }",
            &Column::equal_share(&["AAAA", "BBBB"])
        ),
        "\n\n\n\nBBBB"
    );
}

/// Without `css_layout` the stylesheet's box properties do nothing, content
/// sized or not.
#[test]
fn it_is_inert_without_css_layout() {
    let mut h =
        PipelineHarness::with_css("#AAAA { height: 3; margin-top: 2; }", 20, 8).css_layout(false);
    h.draw(&Column::content_sized(&["AAAA", "BBBB"]));
    assert_eq!(h.screen_text(), "AAAA\nBBBB");
}

/// An explicit main-axis size outranks a child that fills that axis.
#[test]
fn an_explicit_width_beats_a_child_that_fills_the_row() {
    struct Row;
    impl View for Row {
        fn render(&self, ctx: &mut RenderContext) {
            hstack()
                .content_sized(true)
                .child(Text::new("["))
                .child(Progress::new(1.0).element_id("bar"))
                .child(Text::new("]"))
                .render(ctx);
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }
    assert_eq!(draw("#bar { width: 5; }", &Row), "[█████]");
}
