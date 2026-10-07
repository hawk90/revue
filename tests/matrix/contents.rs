//! Layer 1b - contents only.
//!
//! Every widget at one ordinary size (40×10), unfocused, on the direct path,
//! over the content edge set. Targets display-width and char-boundary bugs.

use super::{run_layer, Case, Content, Path, Placement};

pub fn contents() -> Vec<Content> {
    vec![
        Content::text("empty", ""),
        Content::text("hangul", "한글 텍스트"),
        Content::text("emoji", "👍🏽 emoji 👨‍👩‍👧"),
        Content::text("combining", "e\u{301} combining e\u{301}"),
        Content::text("rtl", "שלום עולם"),
        Content::text("zero-width", "a\u{200b}b\u{200b}c"),
        Content::text("control", "tab\tand\nnewline \u{1}\u{7f}"),
        Content::text("long", "abc ".repeat(250)),
        Content::items("items0", 0),
        Content::items("items1", 1),
        Content::items("items200", 200),
    ]
}

#[test]
fn every_widget_survives_edge_contents() {
    let contents = contents();
    let cases: Vec<Case> = contents
        .iter()
        .map(|content| Case {
            placement: Placement::sized("40x10", 40, 10),
            content,
            focused: false,
            path: Path::Direct,
        })
        .collect();
    run_layer("contents", &cases);
}
