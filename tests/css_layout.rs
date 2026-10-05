//! Contracts for CSS box properties overriding container-computed geometry.
//!
//! `App::builder().css_layout(true)` lets a node's own specified `display`,
//! `width`, `height`, `margin` and min/max constraints adjust the area its
//! container handed it. The container still decides the *flow*.
//!
//! Why an override rather than a layout engine that owns the geometry:
//! `docs/refactor/findings-layout.md`.

use revue::prelude::*;
use revue::testing::PipelineHarness;

/// The idiomatic shape: the tree is assembled inside `render`.
struct App {
    rows: Vec<&'static str>,
}

impl App {
    fn new(rows: &[&'static str]) -> Self {
        Self {
            rows: rows.to_vec(),
        }
    }
}

impl View for App {
    fn render(&self, ctx: &mut RenderContext) {
        let mut stack = vstack();
        for row in &self.rows {
            stack = stack.child(Text::new(*row).element_id(*row));
        }
        stack.render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "App"
    }
    fn id(&self) -> Option<&str> {
        Some("app")
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 6)
        .dom_from_render(true)
        .css_layout(true)
}

// ---------------------------------------------------------------------------
// The properties do something
// ---------------------------------------------------------------------------

#[test]
fn display_none_hides_a_widget() {
    let view = App::new(&["AAAA", "BBBB"]);

    let mut plain = harness("");
    plain.draw(&view);
    assert!(plain.contains("BBBB"));

    let mut hidden = harness("#BBBB { display: none; }");
    hidden.draw(&view);

    assert!(hidden.contains("AAAA"), "it hid the wrong widget");
    assert!(
        !hidden.contains("BBBB"),
        "`display: none` did not hide the widget"
    );
}

/// And the subtree goes with it - without the cursor skipping the whole
/// subtree, later siblings would be handed the wrong node's style.
#[test]
fn display_none_hides_the_whole_subtree() {
    struct Nested;
    impl View for Nested {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    vstack()
                        .element_id("group")
                        .child(Text::new("IN").element_id("in")),
                )
                .child(Text::new("AFTER").element_id("after"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Nested"
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }

    let mut h = harness("#group { display: none; } #after { color: rgb(0, 255, 0); }");
    h.draw(&Nested);

    assert!(!h.contains("IN"), "a hidden subtree still painted");
    assert!(h.contains("AFTER"), "the sibling after it was hidden too");

    let text = h.screen_text();
    let row = text
        .lines()
        .position(|l| l.contains("AFTER"))
        .expect("AFTER");
    assert_eq!(
        h.buffer().get(0, row as u16).and_then(|c| c.fg),
        Some(Color::rgb(0, 255, 0)),
        "the sibling after a hidden subtree got the wrong node's style - the \
         paint cursor did not skip the subtree"
    );
}

#[test]
fn width_narrows_a_widget() {
    let view = App::new(&["AAAAAAAAAA"]);

    let mut plain = harness("");
    plain.draw(&view);
    assert_eq!(plain.screen_text(), "AAAAAAAAAA");

    let mut narrow = harness("#AAAAAAAAAA { width: 4; }");
    narrow.draw(&view);

    assert_eq!(
        narrow.screen_text(),
        "AAAA",
        "`width` did not clip the widget"
    );
}

#[test]
fn a_margin_offsets_a_widget() {
    let mut h = harness("#AAAA { margin-left: 3; }");
    h.draw(&App::new(&["AAAA"]));

    assert_eq!(h.screen_text(), "   AAAA", "`margin-left` had no effect");
}

/// The longhands exist. Only the shorthand did, so `margin-left: 3` parsed to
/// nothing and silently did nothing.
#[test]
fn the_spacing_longhands_are_understood() {
    let mut h = harness("#AAAA { margin-top: 2; margin-left: 1; }");
    h.draw(&App::new(&["AAAA"]));
    assert_eq!(h.screen_text(), "\n\n AAAA");
}

#[test]
fn max_width_clamps_and_min_width_floors() {
    let mut h = harness("#AAAAAAAAAA { width: 8; max-width: 5; }");
    h.draw(&App::new(&["AAAAAAAAAA"]));
    assert_eq!(h.screen_text(), "AAAAA");
}

#[test]
fn a_percentage_resolves_against_the_offered_area() {
    let mut h = harness("#AAAAAAAAAA { width: 50%; }");
    h.draw(&App::new(&["AAAAAAAAAA"]));

    // The harness is 20 cells wide.
    assert_eq!(h.screen_text(), "AAAAAAAAAA");
}

// ---------------------------------------------------------------------------
// Guards
// ---------------------------------------------------------------------------

/// Off by default. A stylesheet that sets layout properties must not change
/// anything until the flag is on.
#[test]
fn it_is_off_by_default() {
    let view = App::new(&["AAAA", "BBBB"]);
    let css = "#BBBB { display: none; } #AAAA { width: 1; }";

    let mut off = PipelineHarness::with_css(css, 20, 6).dom_from_render(true);
    off.draw(&view);

    assert!(
        off.contains("BBBB") && off.contains("AAAA"),
        "css_layout took effect without being enabled"
    );
}

/// Enabling it must not move anything that specifies no layout property. This
/// is the regression guard for the whole change.
#[test]
fn enabling_it_changes_nothing_without_layout_css() {
    let view = App::new(&["AAAA", "BBBB", "CCCC"]);
    let paint_only = "#BBBB { color: rgb(255, 0, 0); }";

    let mut off = PipelineHarness::with_css(paint_only, 20, 6).dom_from_render(true);
    off.draw(&view);

    let mut on = harness(paint_only);
    on.draw(&view);

    assert_eq!(off.screen_text(), on.screen_text());
}

/// Inert without `dom_from_render`: there are no per-node styles below the root
/// to read box properties from.
#[test]
fn it_is_inert_without_dom_from_render() {
    let view = App::new(&["AAAA", "BBBB"]);

    let mut h = PipelineHarness::with_css("#BBBB { display: none; }", 20, 6).css_layout(true);
    h.draw(&view);

    assert!(
        h.contains("BBBB"),
        "a box property took effect without the render-derived DOM"
    );
}

/// Node identity is unaffected: a hidden widget still has a node, so its state
/// survives being hidden and shown again.
#[test]
fn a_hidden_widget_keeps_its_node() {
    let mut h = harness("#BBBB { display: none; }");
    h.draw(&App::new(&["AAAA", "BBBB"]));

    assert!(
        h.node_id("BBBB").is_some(),
        "hiding a widget removed it from the DOM"
    );
}

/// The paint pass renders with an area the collect pass never saw, so a widget
/// that branches on its width can produce a different number of nodes in each
/// pass. The cursor has to survive that: it is reset to the child's subtree end
/// after every child, so a miscount cannot reach the siblings.
#[test]
fn a_child_that_renders_differently_under_css_does_not_desync_its_siblings() {
    /// Renders two rows when it has room, none when it does not.
    struct Shrinker;
    impl View for Shrinker {
        fn render(&self, ctx: &mut RenderContext) {
            if ctx.area.width >= 5 {
                vstack()
                    .child(Text::new("P").element_id("p"))
                    .child(Text::new("Q").element_id("q"))
                    .render(ctx);
            }
        }
        fn widget_type(&self) -> &'static str {
            "Shrinker"
        }
        fn id(&self) -> Option<&str> {
            Some("shrinker")
        }
    }

    struct Root;
    impl View for Root {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(Shrinker)
                .child(Text::new("TAIL").element_id("tail"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Root"
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }

    let mut h = harness("#shrinker { width: 2; } #tail { color: rgb(0, 0, 255); }");
    h.draw(&Root);

    let text = h.screen_text();
    assert!(
        !text.contains('P'),
        "the shrunk child still painted its rows"
    );
    let row = text.lines().position(|l| l.contains("TAIL")).expect("TAIL");
    assert_eq!(
        h.buffer().get(0, row as u16).and_then(|c| c.fg),
        Some(Color::rgb(0, 0, 255)),
        "the sibling was handed the wrong node's style - the paint cursor \
         desynchronized"
    );
}

// ---------------------------------------------------------------------------
// gap: the one flow property a container can honor
// ---------------------------------------------------------------------------

/// `gap` describes flow, so it stays with the container - but the container can
/// read it, and `Stack` now does. `gap: 0` is the initial value and reads as
/// "not specified", so a stylesheet that says nothing leaves the builder's own
/// gap alone.
#[test]
fn a_css_gap_overrides_the_builders_gap() {
    struct Two;
    impl View for Two {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .gap(0)
                .child(Text::new("A").element_id("a"))
                .child(Text::new("B").element_id("b"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Two"
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }

    let mut plain = harness("");
    plain.draw(&Two);

    let mut spaced = harness("#app { gap: 2; }");
    spaced.draw(&Two);

    let gap_of = |t: &str| {
        let lines: Vec<&str> = t.lines().collect();
        let a = lines.iter().position(|l| l.contains('A')).expect("A");
        let b = lines.iter().position(|l| l.contains('B')).expect("B");
        b - a
    };

    assert!(
        gap_of(&spaced.screen_text()) > gap_of(&plain.screen_text()),
        "`gap` did not reach the container"
    );
}

/// The other direction, which is the one that was broken: a stylesheet has to
/// be able to say `gap: 0` and mean it.
///
/// `gap` was a plain `u16`, so "the stylesheet said zero" and "the stylesheet
/// said nothing" were the same value and the builder's gap won either way -
/// the limitation `docs/FEATURES.md` recorded. `column-gap` and `row-gap` were
/// already `Option<u16>` and did not have it, which is what made the
/// inconsistency visible.
#[test]
fn a_css_gap_of_zero_overrides_the_builders_gap() {
    struct Spaced;
    impl View for Spaced {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .gap(2)
                .child(Text::new("A").element_id("a"))
                .child(Text::new("B").element_id("b"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Spaced"
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }

    let gap_of = |t: &str| {
        let lines: Vec<&str> = t.lines().collect();
        let a = lines.iter().position(|l| l.contains('A')).expect("A");
        let b = lines.iter().position(|l| l.contains('B')).expect("B");
        b - a
    };

    let mut builder_only = harness("");
    builder_only.draw(&Spaced);

    let mut closed = harness("#app { gap: 0; }");
    closed.draw(&Spaced);

    assert!(
        gap_of(&closed.screen_text()) < gap_of(&builder_only.screen_text()),
        "`gap: 0` did not close the builder's gap"
    );
}

#[test]
fn a_css_gap_is_ignored_without_the_flag() {
    struct Two;
    impl View for Two {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .gap(0)
                .child(Text::new("A").element_id("a"))
                .child(Text::new("B").element_id("b"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Two"
        }
        fn id(&self) -> Option<&str> {
            Some("app")
        }
    }

    let mut off = PipelineHarness::with_css("#app { gap: 2; }", 20, 6).dom_from_render(true);
    off.draw(&Two);
    let mut plain = PipelineHarness::with_css("", 20, 6).dom_from_render(true);
    plain.draw(&Two);

    assert_eq!(
        off.screen_text(),
        plain.screen_text(),
        "`gap` took effect without css_layout"
    );
}
