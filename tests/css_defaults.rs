//! 3.0 defaults: the stylesheet reaches every widget without opting in.
//!
//! In 2.x `dom_from_render` and `css_layout` were off, so a rule written for a
//! widget below the root matched nothing and was silently ignored. In 3.0 both
//! are on, and turning them off restores 2.x.
//!
//! See `docs/migration/v3.0.0.md` section 1.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

/// A title below the root, the shape every application has.
struct Page;

impl View for Page {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new("Title").class("title"))
            .child(Text::new("Body"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Page"
    }
}

const CSS: &str = ".title { text-align: center; background: #ff0000; margin-left: 2; }";

fn row(h: &PipelineHarness, y: u16) -> String {
    h.screen_text()
        .lines()
        .nth(y as usize)
        .unwrap_or("")
        .to_string()
}

#[test]
fn both_are_on_by_default() {
    let app = App::builder().size(20, 4).build();
    assert!(app.dom_from_render(), "dom_from_render is off by default");
    assert!(app.css_layout(), "css_layout is off by default");
}

#[test]
fn turning_them_off_restores_2x() {
    let app = App::builder()
        .size(20, 4)
        .dom_from_render(false)
        .css_layout(false)
        .build();
    assert!(!app.dom_from_render());
    assert!(!app.css_layout());
}

#[test]
fn the_harness_follows_the_app_defaults() {
    let h = PipelineHarness::new(20, 4);
    assert!(h.app().dom_from_render());
    assert!(h.app().css_layout());
}

/// The rule names a widget below the root, and nothing opted in.
#[test]
fn a_rule_on_a_nested_widget_applies_by_default() {
    let mut h = PipelineHarness::with_css(CSS, 20, 4);
    h.draw(&Page);

    // margin-left: 2 leaves 18 columns; "Title" centered in them starts at 2 + 6.
    let title = row(&h, 0);
    assert_eq!(title.find("Title"), Some(8), "title row: {title:?}");

    // The background fills the title's box, margin excluded.
    assert_eq!(
        h.buffer().get(1, 0).and_then(|c| c.bg),
        None,
        "margin painted"
    );
    assert_eq!(h.buffer().get(2, 0).and_then(|c| c.bg), Some(RED));
    assert_eq!(h.buffer().get(19, 0).and_then(|c| c.bg), Some(RED));

    // The sibling has no rule and is untouched.
    assert!(row(&h, 1).starts_with("Body"), "body row: {:?}", row(&h, 1));
    assert_eq!(h.buffer().get(0, 1).and_then(|c| c.bg), None);
}

#[test]
fn opting_out_ignores_the_rule_as_2x_did() {
    let mut h = PipelineHarness::with_css(CSS, 20, 4)
        .dom_from_render(false)
        .css_layout(false);
    h.draw(&Page);

    assert!(row(&h, 0).starts_with("Title"), "row 0: {:?}", row(&h, 0));
    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.bg), None);
}

/// Paint without box: `css_layout(false)` alone keeps the 2.x geometry but
/// still lets paint properties through.
#[test]
fn css_layout_can_be_turned_off_on_its_own() {
    let mut h = PipelineHarness::with_css(CSS, 20, 4).css_layout(false);
    h.draw(&Page);

    let title = row(&h, 0);
    assert_eq!(
        title.find("Title"),
        Some(7),
        "no margin, centered in 20: {title:?}"
    );
    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.bg), Some(RED));
}
