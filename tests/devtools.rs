//! Tests for `revue::devtools`, laid out like `src/devtools/`

#[path = "devtools/inspector/mod.rs"]
mod inspector;
#[path = "devtools/render.rs"]
mod render;

use revue::devtools::{DevTools, Inspector};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;

/// An inspector with one `Button#save.primary` node, selected, at (2, 2) 10x4.
pub(crate) fn with_button(inspector: Inspector) -> Inspector {
    let mut inspector = inspector;
    let id = inspector.add_root("Button");
    let node = inspector.get_mut(id).unwrap();
    node.widget_id = Some("save".to_string());
    node.classes.push("primary".to_string());
    node.rect = Rect::new(2, 2, 10, 4);
    inspector.select(Some(id));
    inspector
}

/// Draw `devtools` over a 100x20 screen; the panel takes the right 50 columns.
pub(crate) fn screen(devtools: &DevTools) -> Buffer {
    let mut buffer = Buffer::new(100, 20);
    devtools.render(&mut buffer, Rect::new(0, 0, 100, 20));
    buffer
}

pub(crate) fn devtools_with(inspector: Inspector) -> DevTools {
    let mut devtools = DevTools::new();
    devtools.set_visible(true);
    *devtools.inspector_mut() = with_button(inspector);
    devtools
}

/// Background of the selected node's four corners.
pub(crate) fn corners(buffer: &Buffer) -> [Option<Color>; 4] {
    [(2, 2), (11, 2), (2, 5), (11, 5)].map(|(x, y)| buffer.get(x, y).unwrap().bg)
}
