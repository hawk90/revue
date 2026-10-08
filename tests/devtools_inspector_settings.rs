//! The devtools `Inspector` settings change what it draws.

use revue::devtools::{DevTools, DevToolsConfig, DevToolsTab, Inspector, InspectorConfig};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;

/// An inspector with one `Button#save.primary` node, selected, at (2, 2) 10x4.
fn with_button(inspector: Inspector) -> Inspector {
    let mut inspector = inspector;
    let id = inspector.add_root("Button");
    let node = inspector.get_mut(id).unwrap();
    node.widget_id = Some("save".to_string());
    node.classes.push("primary".to_string());
    node.rect = Rect::new(2, 2, 10, 4);
    inspector.select(Some(id));
    inspector
}

fn tree_row(inspector: &Inspector) -> String {
    let mut buffer = Buffer::new(40, 10);
    inspector.render_content(
        &mut buffer,
        Rect::new(0, 0, 40, 10),
        &DevToolsConfig::default(),
    );
    (0..40)
        .map(|x| buffer.get(x, 0).unwrap().symbol)
        .collect::<String>()
        .trim()
        .to_string()
}

#[test]
fn the_tree_shows_ids_and_classes_by_default() {
    assert_eq!(
        tree_row(&with_button(Inspector::new())),
        "Button#save.primary"
    );
}

#[test]
fn show_ids_false_leaves_the_id_out_of_the_tree() {
    assert_eq!(
        tree_row(&with_button(Inspector::new().show_ids(false))),
        "Button.primary"
    );
}

#[test]
fn show_classes_false_leaves_the_classes_out_of_the_tree() {
    assert_eq!(
        tree_row(&with_button(Inspector::new().show_classes(false))),
        "Button#save"
    );
}

/// Draw `devtools` over a 100x20 screen; the panel takes the right 50 columns.
fn screen(devtools: &DevTools) -> Buffer {
    let mut buffer = Buffer::new(100, 20);
    devtools.render(&mut buffer, Rect::new(0, 0, 100, 20));
    buffer
}

fn devtools_with(inspector: Inspector) -> DevTools {
    let mut devtools = DevTools::new();
    devtools.set_visible(true);
    *devtools.inspector_mut() = with_button(inspector);
    devtools
}

/// Background of the selected node's four corners.
fn corners(buffer: &Buffer) -> [Option<Color>; 4] {
    [(2, 2), (11, 2), (2, 5), (11, 5)].map(|(x, y)| buffer.get(x, y).unwrap().bg)
}

const DEFAULT_HIGHLIGHT: Color = Color::rgb(100, 200, 255);

#[test]
fn the_selected_widget_is_outlined_on_screen() {
    let buffer = screen(&devtools_with(Inspector::new()));
    assert_eq!(corners(&buffer), [Some(DEFAULT_HIGHLIGHT); 4]);
    assert_eq!(
        buffer.get(5, 3).unwrap().bg,
        None,
        "only the outline is drawn"
    );
}

#[test]
fn the_outline_takes_the_highlight_color() {
    let config = InspectorConfig {
        highlight_color: Color::MAGENTA,
        ..InspectorConfig::default()
    };
    let buffer = screen(&devtools_with(Inspector::new().config(config)));
    assert_eq!(corners(&buffer), [Some(Color::MAGENTA); 4]);
}

#[test]
fn show_bounds_false_draws_no_outline() {
    let buffer = screen(&devtools_with(Inspector::new().show_bounds(false)));
    assert_eq!(corners(&buffer), [None; 4]);
}

#[test]
fn no_outline_on_another_tab() {
    let mut devtools = devtools_with(Inspector::new());
    devtools.set_tab(DevToolsTab::Styles);
    assert_eq!(corners(&screen(&devtools)), [None; 4]);
}
