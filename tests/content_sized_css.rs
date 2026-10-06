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

/// The reported bug: the margin used to inset a one-row area to nothing.
/// Cross-axis margins are left to the box model, as before.
/// The other half of the bug: the height used to grow a one-row area over the
/// next sibling instead of reserving rows for itself.
/// `height` then `max-height`, in that order - as the box model does it.
/// In a row the main axis is the width.
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
/// A percentage has no basis in a content-sized slot, so such a child fills
/// like an unmeasured one and the box model resolves the percentage against
/// its share - exactly as an equal-share stack does.
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
    let mut h = PipelineHarness::with_css("#AAAA { height: 3; margin-top: 2; }", 20, 8)
        .dom_from_render(true);
    h.draw(&Column::content_sized(&["AAAA", "BBBB"]));
    assert_eq!(h.screen_text(), "AAAA\nBBBB");
}
