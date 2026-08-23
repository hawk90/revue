//! A ratchet: every widget that reports a DOM node should read its computed
//! style.
//!
//! Half the widget library paints from its own fields and hardcoded palettes,
//! so a rule that names a color reaches nothing - `F-5` from
//! `docs/refactor/findings-render-pipeline.md`. Wiring them is mechanical and
//! proceeds by category; this keeps the work visible and stops it going
//! backwards.
//!
//! The check is at the source level: a widget's module must mention at least
//! one way of reading a computed style. That cannot prove the value reaches a
//! cell - the per-category tests next to this one do that - but it does catch
//! the case this ratchet exists for, which is a *new* widget shipped with no
//! CSS support at all.
//!
//! **This test passing is not evidence a widget is styled.** That is not a
//! hypothetical: `Slider` was once wired by resolving its color in the `render`
//! that only dispatches to `render_horizontal` / `render_vertical`, so the value
//! was computed and dropped. This test was happy; `color_reaches_a_sliders_fill`
//! was not. Wire a widget *and* assert against the buffer.
//!
//! # Working on this
//!
//! Wire a widget, then delete its name from `NOT_YET_READING_CSS`. The test
//! fails if a listed widget turns out to read CSS after all, so the list cannot
//! quietly go stale.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Widgets that carry a DOM node and do not read a computed style yet.
///
/// Shrinks. Never grows: a new widget belongs on the wired side.
///
/// A few entries are here *deliberately*, marked `intentional:`. They color
/// sub-parts a stylesheet cannot address on their own - a `RichLog` line is
/// colored by its level, and lines are not nodes - so a blanket `color` rule
/// would flatten a distinction with no way to restore it. Same reasoning as a
/// selected table row keeping its highlight.
const NOT_YET_READING_CSS: &[&str] = &[
    // intentional: per-line level colors (Info/Warning/Error) are the content,
    // and a line is not a node, so a rule on the log cannot restore them.
    "RichLog",
    // intentional: each event carries its own type color and an event is not a
    // node, so a rule on the timeline would flatten them with no way back.
    "Timeline",
    "Accordion",
    "AiStream",
    "Autocomplete",
    "BarChart",
    "BoxPlot",
    "Calendar",
    "CandleChart",
    "Chart",
    "CodeEditor",
    "ColorPicker",
    "Combobox",
    "CommandPalette",
    "CsvViewer",
    "DataGrid",
    "DateTimePicker",
    "Diagram",
    "DiffViewer",
    "DropZone",
    "ErrorBoundary",
    "FilePicker",
    "FileTree",
    "Form",
    "Gauge",
    "GradientBox",
    "HeatMap",
    "Histogram",
    "HttpClient",
    "Image",
    "JsonViewer",
    "Layers",
    "LogViewer",
    "Markdown",
    "MarkdownPresentation",
    "MaskedInput",
    "Modal",
    "NotificationCenter",
    "PieChart",
    "Positioned",
    "Presentation",
    "ProcessMonitor",
    "QrCodeWidget",
    "Resizable",
    "RichText",
    "RichTextEditor",
    "ScatterChart",
    "ScreenStack",
    "ScrollView",
    "SearchBar",
    "Select",
    "SortableList",
    "Splitter",
    "StatusBar",
    "Stepper",
    "Streamline",
    "Terminal",
    "TextArea",
    "TimeSeries",
    "Timer",
    "ToastQueue",
    "Transition",
    "TransitionGroup",
    "Tree",
    "VirtualList",
    "Waveline",
    "ZenMode",
];

/// Every way a widget can reach its computed style.
const CSS_READERS: &[&str] = &[
    "ctx.style",
    "css_color",
    "css_background",
    "css_border",
    "css_visible",
    "css_opacity",
    "css_gap",
    "css_text_align",
    "css_bold",
    "css_underline",
    "css_line_through",
    "css_overflow_hidden",
    "css_flex_wrap",
    "css_padding",
    "css_margin",
    "css_width",
    "css_height",
    "color_or",
    "gap_or",
    "resolve_fg",
    "resolve_bg",
    "resolve_colors",
    "state_colors",
];

fn widget_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/widget")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The widget type name each `impl_view_meta!` declares, with the file it was
/// declared in.
fn declared_widgets() -> Vec<(String, PathBuf)> {
    let mut files = Vec::new();
    rust_files(&widget_root(), &mut files);
    files.sort();

    let mut found = Vec::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        for line in text.lines() {
            let Some(rest) = line.split_once("impl_view_meta!(\"") else {
                continue;
            };
            let Some((name, _)) = rest.1.split_once('"') else {
                continue;
            };
            if name.is_empty() {
                continue;
            }
            found.push((name.to_string(), file.clone()));
            break;
        }
    }
    found
}

fn file_reads_css(file: &Path) -> bool {
    std::fs::read_to_string(file)
        .map(|text| CSS_READERS.iter().any(|needle| text.contains(needle)))
        .unwrap_or(false)
}

/// How many widgets a directory declares, counting only its own files.
fn widgets_declared_directly_in(dir: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    entries
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "rs"))
        .filter(|e| {
            std::fs::read_to_string(e.path())
                .map(|t| t.contains("impl_view_meta!("))
                .unwrap_or(false)
        })
        .count()
}

/// Does this widget read a computed style?
///
/// Its own file first. A widget split across `core.rs` / `render.rs` reads it
/// somewhere else in the module, so siblings count too - but *only* when the
/// module declares this one widget. `src/widget/display/` holds sixteen
/// widgets in one directory, and crediting all of them because one reads CSS is
/// how a ratchet quietly stops ratcheting.
fn widget_reads_css(file: &Path) -> bool {
    if file_reads_css(file) {
        return true;
    }
    let Some(dir) = file.parent() else {
        return false;
    };
    if widgets_declared_directly_in(dir) > 1 {
        return false;
    }
    let mut files = Vec::new();
    rust_files(dir, &mut files);
    files.iter().any(|f| file_reads_css(f))
}

#[test]
fn every_widget_with_a_node_reads_its_computed_style() {
    let listed: BTreeSet<&str> = NOT_YET_READING_CSS.iter().copied().collect();

    let mut unwired_and_unlisted = Vec::new();
    let mut wired_but_listed = Vec::new();

    for (name, file) in declared_widgets() {
        let reads = widget_reads_css(&file);
        let is_listed = listed.contains(name.as_str());

        if !reads && !is_listed {
            unwired_and_unlisted.push(name);
        } else if reads && is_listed {
            wired_but_listed.push(name);
        }
    }

    unwired_and_unlisted.sort();
    unwired_and_unlisted.dedup();
    wired_but_listed.sort();
    wired_but_listed.dedup();

    assert!(
        unwired_and_unlisted.is_empty(),
        "these widgets report a DOM node and never read a computed style, so no \
         rule can reach them. Wire them - `ctx.css_color(default)` for a plain \
         field, `self.fg.or_else(|| ctx.css_color_if_set())` for an `Option` - \
         or, if that is genuinely not possible yet, add them to \
         NOT_YET_READING_CSS with a reason: {unwired_and_unlisted:?}"
    );

    assert!(
        wired_but_listed.is_empty(),
        "these widgets now read a computed style and are still listed in \
         NOT_YET_READING_CSS. Delete them from that list: {wired_but_listed:?}"
    );
}
