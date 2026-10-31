//! Contracts for a [`Stack::content_sized`] stack under `css_layout`.
//!
//! The stack sizes each unsized child to its content, and the child's own CSS
//! box - `height`/`width`, `min-*`/`max-*`, margins along the stack's axis - is
//! folded into that size *before* the stack lays out. Without that, the box
//! properties would be applied to the content-sized area after the fact: a
//! `margin-top` on a one-row `Text` would inset its one row to nothing, and a
//! `height: 3` would grow it over the next sibling.
//!
//! The rule is written down in <https://github.com/hawk90/revue/blob/8f9dcb2e850e85e04f252a388e52ed037fe57179/docs/refactor/design-content-sized-stack.md>.

use revue::prelude::*;
use revue::testing::PipelineHarness;
use revue::widget::Fill;

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

/// An explicit `height` or `min-height` is put back by the box model, so
/// the slot keeps it when the column overflows; only measured siblings give
/// way. (With eight siblings a measured slot would shrink to four rows.)
#[test]
fn a_pinned_height_keeps_its_rows_when_the_column_overflows() {
    for css in ["#AAAA { height: 6; }", "#AAAA { min-height: 6; }"] {
        assert_eq!(
            draw(
                css,
                &Column::content_sized(&["AAAA", "B", "C", "D", "E", "F", "G", "H", "I"])
            ),
            "AAAA\n\n\n\n\n\nB\nC",
            "{css}"
        );
    }
}

// ---------------------------------------------------------------------------
// A CSS minimum on a child that fills (#1338)
// ---------------------------------------------------------------------------

/// Fills what it is given with `#`.
struct Fills;

impl View for Fills {
    fn render(&self, ctx: &mut RenderContext) {
        for y in 0..ctx.area.height {
            for x in 0..ctx.area.width {
                ctx.draw_char(x, y, '#', Color::WHITE);
            }
        }
    }

    fn widget_type(&self) -> &'static str {
        "Fills"
    }

    fn id(&self) -> Option<&str> {
        Some("w")
    }
}

/// `view` drawn in `w`×`h`, row by row.
fn screen(css: &str, view: &impl View, w: u16, h: u16) -> Vec<String> {
    let mut harness = PipelineHarness::with_css(css, w, h)
        .dom_from_render(true)
        .css_layout(true);
    harness.draw(view);
    harness
        .screen_text()
        .lines()
        .map(|line| format!("{line:<width$}", width = w as usize))
        .collect()
}

fn fills_then_end() -> Stack {
    vstack().child(Fills).child(Text::new("end"))
}

#[test]
fn a_minimum_on_a_filling_child_is_given_a_slot() {
    assert_eq!(
        screen("#w { min-height: 3; }", &fills_then_end(), 4, 6),
        ["####", "####", "####", "####", "####", "end "]
    );
}

#[test]
fn with_too_little_room_a_filling_child_keeps_its_minimum_and_the_rest_is_cut() {
    assert_eq!(
        screen("#w { min-height: 5; }", &fills_then_end(), 4, 4),
        ["####", "####", "####", "####"]
    );
    assert_eq!(
        screen("#w { min-height: 3; }", &fills_then_end(), 4, 4),
        ["####", "####", "####", "end "]
    );
}

#[test]
fn a_filling_child_with_a_minimum_keeps_it_above_a_measured_sibling() {
    let view = vstack().child(Text::new("1\n2\n3")).child(Fills);
    assert_eq!(
        screen("#w { min-height: 3; }", &view, 2, 4),
        ["1 ", "##", "##", "##"]
    );
}

#[test]
fn a_margin_is_part_of_the_slot_a_minimum_takes() {
    assert_eq!(
        screen(
            "#w { min-height: 2; margin-top: 1; margin-bottom: 1; }",
            &fills_then_end(),
            4,
            5
        ),
        ["    ", "####", "####", "    ", "end "]
    );
}

#[test]
fn with_too_little_room_the_margins_are_kept_with_the_minimum() {
    assert_eq!(
        screen(
            "#w { min-height: 2; margin-top: 1; margin-bottom: 1; }",
            &fills_then_end(),
            4,
            3
        ),
        ["    ", "####", "####"]
    );
}

/// Measures, but fills the stack's axis, so it takes a share.
struct FillsHeight;

impl View for FillsHeight {
    fn render(&self, ctx: &mut RenderContext) {
        Fills.render(ctx);
    }
    fn measure(&self, max_width: u16, _max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, 1))
    }
    fn fills(&self) -> Fill {
        Fill::HEIGHT
    }
    fn widget_type(&self) -> &'static str {
        "FillsHeight"
    }
    fn id(&self) -> Option<&str> {
        Some("w")
    }
}

#[test]
fn a_child_that_measures_and_fills_the_axis_keeps_its_minimum_too() {
    let view = vstack().child(FillsHeight).child(Text::new("end"));
    assert_eq!(
        screen("#w { min-height: 5; }", &view, 4, 4),
        ["####", "####", "####", "####"]
    );
}

#[test]
fn a_minimum_above_the_maximum_wins_on_either_axis_of_either_stack() {
    assert_eq!(
        screen(
            "#w { min-height: 3; max-height: 1; }",
            &fills_then_end(),
            4,
            4
        ),
        ["####", "####", "####", "end "]
    );
}

#[test]
fn a_minimum_above_the_maximum_does_not_panic_across_the_stack_either() {
    for css in [
        "#w { min-width: 3; max-width: 1; }",
        "#w { min-height: 3; max-height: 1; }",
    ] {
        let row = hstack().child(Fills).child(Text::new("e"));
        assert_eq!(screen(css, &row, 4, 3).len(), 3, "{css}");
        assert_eq!(screen(css, &fills_then_end(), 4, 4).len(), 4, "{css}");
    }
}

#[test]
fn a_minimum_works_along_a_row_too() {
    let view = hstack().child(Fills).child(Text::new("end"));
    assert_eq!(screen("#w { min-width: 4; }", &view, 6, 1), ["####en"]);
}

#[test]
fn a_maximum_caps_what_a_view_asks_for() {
    struct Asks;
    impl View for Asks {
        fn render(&self, ctx: &mut RenderContext) {
            Fills.render(ctx);
        }
        fn min_size(&self) -> (u16, u16) {
            (0, 5)
        }
        fn widget_type(&self) -> &'static str {
            "Asks"
        }
        fn id(&self) -> Option<&str> {
            Some("w")
        }
    }
    let view = vstack().child(Text::new("1\n2\n3")).child(Asks);
    // Two rows are all it can use, so the text gives up only one for it.
    assert_eq!(
        screen("#w { max-height: 2; }", &view, 2, 4),
        ["1 ", "2 ", "##", "##"]
    );
}

#[test]
fn the_issue_example_keeps_the_zone_and_cuts_the_text() {
    let view = vstack()
        .child(
            revue::widget::DropZone::new("Drop")
                .element_id("w")
                .min_height(0),
        )
        .child(Text::new("end"));
    assert_eq!(
        screen("#w { min-height: 5; }", &view, 6, 4),
        ["┌────┐", "│    │", "│ Dr │", "│    │"]
    );
}
