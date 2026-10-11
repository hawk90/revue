//! The `Inspector` settings change what it draws: the tree row and the
//! outline of the selected widget.

use crate::{corners, devtools_with, screen, with_button};
use revue::devtools::{DevToolsConfig, Inspector, InspectorConfig};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;

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
