//! Paint-level checks for the pickers and the file browsers.
//!
//! Each of these paints more than one meaningful color and only one of them is
//! CSS-addressable. A file tree's cyan directory against its magenta symlink is
//! how the tree is *read*, a drop zone's border says whether the drag would be
//! accepted, and a color picker's swatch is the value being picked. A blanket
//! `color` rule that flattened those would destroy a distinction a stylesheet
//! has no way to restore - so `color` reaches the base each of them departs
//! from: the plain file, the resting border, the chrome.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{
    ColorPicker, ColorPickerMode, DropZone, FileEntry, FileFilter, FilePicker, FileTree, FileType,
};

/// A color no widget here paints on its own.
///
/// `Color::RED` was the obvious sentinel and it was the wrong one: a color
/// picker's R channel slider *is* red, so the assertion passed with the wiring
/// reverted. A test color has to be one nothing in the widget can produce.
const INK: Color = Color {
    r: 17,
    g: 34,
    b: 51,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 60, 12).dom_from_render(true);
    h.draw(view);
    h
}

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

macro_rules! wrap {
    ($name:ident, $build:expr) => {
        struct $name;
        impl View for $name {
            fn render(&self, ctx: &mut RenderContext) {
                vstack().child($build).render(ctx);
            }
            fn widget_type(&self) -> &'static str {
                stringify!($name)
            }
            fn id(&self) -> Option<&str> {
                Some("root")
            }
        }
    };
}

wrap!(
    DropZoneView,
    DropZone::new("Drop files here").element_id("w")
);

wrap!(
    FileTreeView,
    FileTree::new()
        .root(vec![FileEntry::new(
            "notes.txt",
            "/tmp/notes.txt",
            FileType::File,
        )])
        .element_id("w")
);

wrap!(
    FilePickerView,
    // A directory of plain files and no subdirectories, so the rows that reach
    // the buffer are the ones `color` is meant to address. The first is
    // highlighted and keeps its own color; the rest are the base.
    FilePicker::new()
        .start_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/widget/dropzone"))
        .filter(FileFilter::Extensions(vec!["rs".into()]))
        .element_id("w")
);

wrap!(
    ColorPickerView,
    ColorPicker::new()
        .mode(ColorPickerMode::Rgb)
        .element_id("w")
);

#[test]
fn color_reaches_a_drop_zones_resting_border() {
    let h = draw("#w { color: #112233; }", &DropZoneView);
    assert!(any_fg(&h, INK), "`color` did not reach DropZone's border");
}

#[test]
fn color_reaches_a_file_trees_plain_file() {
    let h = draw("#w { color: #112233; }", &FileTreeView);
    assert!(any_fg(&h, INK), "`color` did not reach FileTree's file row");
}

#[test]
fn color_reaches_a_file_pickers_plain_file() {
    let h = draw("#w { color: #112233; }", &FilePickerView);
    assert!(
        any_fg(&h, INK),
        "`color` did not reach FilePicker's file row"
    );
}

#[test]
fn color_reaches_a_color_pickers_chrome() {
    let h = draw("#w { color: #112233; }", &ColorPickerView);
    assert!(
        any_fg(&h, INK),
        "`color` did not reach ColorPicker's chrome"
    );
}

/// The parts a rule cannot address keep their own colors. A directory stays
/// cyan under `color: red` because the reading is the difference between it and
/// the file next to it, not the color itself.
#[test]
fn a_directory_keeps_its_type_color() {
    let view = {
        struct V;
        impl View for V {
            fn render(&self, ctx: &mut RenderContext) {
                vstack()
                    .child(
                        FileTree::new()
                            .root(vec![FileEntry::new("src", "/tmp/src", FileType::Directory)])
                            .element_id("w"),
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
        V
    };
    let h = draw("#w { color: #112233; }", &view);
    assert!(
        any_fg(&h, Color::CYAN),
        "a directory lost its type color to a `color` rule"
    );
    assert!(
        !any_fg(&h, INK),
        "`color` flattened a directory row it should not address"
    );
}
