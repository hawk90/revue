//! A selector list (`A, B { .. }`) styles every element any of its
//! selectors matches, not only the first one's (#868)

use revue::dom::{parse_selector, parse_selectors, DomTree, Query, WidgetMeta};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{vstack, Text};

fn three_texts() -> impl revue::widget::View {
    vstack()
        .child(Text::new("A").element_id("a").class("x"))
        .child(Text::new("B").element_id("b"))
        .child(Text::new("C").element_id("c"))
}

#[test]
fn every_selector_of_a_list_is_styled() {
    let mut h = PipelineHarness::with_css(".x, #b { color: red; }", 20, 3);
    h.draw(&three_texts());
    assert_eq!(h.computed_color("a"), Some(Color::RED));
    assert_eq!(
        h.computed_color("b"),
        Some(Color::RED),
        "the second selector"
    );
    assert_ne!(h.computed_color("c"), Some(Color::RED));
}

#[test]
fn each_selector_of_a_list_keeps_its_own_specificity() {
    // #b (an id) outweighs .y (a class) though .y comes later; .x, the
    // list's other selector, is weaker than .y
    let css = "#b, .x { color: red; } .y { color: blue; }";
    let mut h = PipelineHarness::with_css(css, 20, 3);
    h.draw(
        &vstack()
            .child(Text::new("A").element_id("a").class("x").class("y"))
            .child(Text::new("B").element_id("b").class("y")),
    );
    assert_eq!(h.computed_color("a"), Some(Color::BLUE));
    assert_eq!(h.computed_color("b"), Some(Color::RED));
}

#[test]
fn a_list_splits_on_top_level_commas_only() {
    let list = parse_selectors(r#"[title="a,b"], Button:not(:focus), .c"#).unwrap();
    assert_eq!(list.len(), 3);
    assert_eq!(
        format!("{:?}", list[0]),
        format!("{:?}", parse_selector(r#"[title="a,b"]"#).unwrap())
    );
}

#[test]
fn a_single_selector_parse_refuses_a_list() {
    assert!(parse_selector("Button, Input").is_err());
}

#[test]
fn query_finds_the_elements_of_every_selector() {
    let mut tree = DomTree::new();
    let root = tree.create_root(WidgetMeta::new("App"));
    tree.add_child(root, WidgetMeta::new("Button").id("ok"));
    tree.add_child(root, WidgetMeta::new("Input").id("name"));
    tree.add_child(root, WidgetMeta::new("Text").id("label"));

    let ids = |list: &str| -> Vec<String> {
        tree.query_all(list)
            .iter()
            .filter_map(|n| n.meta.id.clone())
            .collect()
    };
    // In document order, each element once
    assert_eq!(ids("Input, Button"), ["ok", "name"]);
    assert_eq!(ids("Button, #ok"), ["ok"]);
    assert_eq!(
        tree.query_one("Text, Input")
            .and_then(|n| n.meta.id.clone()),
        Some("name".to_string())
    );
}
