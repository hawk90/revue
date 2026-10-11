//! Every widget that reports a `WidgetMeta` must have a way to fill it in.
//!
//! A widget that takes part in the DOM but offers no `element_id` or `class`
//! builder cannot be named by any rule. It is in the tree, and unreachable -
//! which is worse than being absent, because `query()` finds it and a
//! stylesheet cannot.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::DropZone;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

#[test]
fn a_list_can_be_given_an_element_id() {
    struct V;
    impl View for V {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(List::new(vec!["a".to_string()]).element_id("lst"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "V"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = PipelineHarness::with_css("#lst { color: #ff0000; }", 20, 5).dom_from_render(true);
    h.draw(&V);

    assert!(h.node_id("lst").is_some(), "List could not be given an id");
    assert_eq!(
        h.computed_color("lst"),
        Some(RED),
        "no rule could select the List"
    );
}

#[test]
fn a_list_can_be_given_a_class() {
    struct V;
    impl View for V {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    List::new(vec!["a".to_string()])
                        .class("hot")
                        .element_id("lst"),
                )
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "V"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = PipelineHarness::with_css(".hot { color: #ff0000; }", 20, 5).dom_from_render(true);
    h.draw(&V);

    assert_eq!(h.computed_color("lst"), Some(RED));
}

#[test]
fn a_dropzone_can_be_given_an_element_id() {
    struct V;
    impl View for V {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(DropZone::new("drop here").element_id("dz"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "V"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let mut h = PipelineHarness::with_css("#dz { color: #ff0000; }", 20, 5).dom_from_render(true);
    h.draw(&V);

    assert!(
        h.node_id("dz").is_some(),
        "DropZone could not be given an id"
    );
    assert_eq!(h.computed_color("dz"), Some(RED));
}
