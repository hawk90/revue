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
//! Every widget is checked, not just the first a file declares. Where one file
//! declares several, only each widget's own `impl` blocks count for it - one
//! of them reading CSS says nothing about its neighbour.
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
    // intentional: pure layout. These place children and paint no cell of their
    // own, so there is no color for a rule to reach. Their *children* are
    // selectable, which is what matters.
    "Layers",
    "Positioned",
    "ScreenStack",
    // The bar is a `Tabs` node and the body the tab's own widget; a rule for
    // `Tabs` reaches the bar (tests/tab_view.rs).
    "TabView",
    // Its areas are `SplitView`s and `TabView`s; a rule reaches each panel
    // (tests/dock.rs).
    "Dock",
    // intentional: per-token or per-role colors. A JSON key against its string
    // value, a log's timestamp against its source, a diff's additions against
    // its deletions, a syntax highlighter's keywords - the colors are how the
    // content is read, and one `color` cannot say all of them at once.
    "CodeEditor",
    "DiffViewer",
    "LogViewer",
    "Markdown",
    // intentional: a computed gradient. `GradientBox` derives every cell's color
    // from its position, and `Image` from the source pixels - there is no single
    // color for a rule to set.
    "GradientBox",
    "Image",
    // intentional: a palette or a scale, which one `color` cannot express. A
    // pie's slices, a scatter's series and a heat map's gradient carry the
    // data; flattening them to one color leaves a chart that shows nothing.
    // `CandleChart` is the same in miniature - bullish green against bearish
    // red is the reading.
    "BoxPlot",
    "CandleChart",
    "HeatMap",
    "PieChart",
    "ScatterChart",
    // intentional: per-line level colors (Info/Warning/Error) are the content,
    // and a line is not a node, so a rule on the log cannot restore them.
    "RichLog",
    // intentional: each event carries its own type color and an event is not a
    // node, so a rule on the timeline would flatten them with no way back.
    "Timeline",
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
    "background_or",
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

/// Every widget each `impl_view_meta!` declares: its DOM name, the Rust type
/// whose `View` impl the macro sits in, and the file it was declared in.
///
/// *Every* one, not the first per file. `Timer` and `Stopwatch` share a file,
/// as do `Form` and `FormField`; checking only the first let the second of
/// each pair ship reading no CSS while the ratchet stayed green.
fn declared_widgets() -> Vec<Widget> {
    let mut files = Vec::new();
    rust_files(&widget_root(), &mut files);
    files.sort();

    let mut found = Vec::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        let mut view_type = None;
        for line in text.lines() {
            // A doc example such as the macro's own `MyWidget` is not a widget.
            if line.trim_start().starts_with("//") {
                continue;
            }
            if let Some(ty) = view_impl_target(line) {
                view_type = Some(ty);
            }
            let Some(rest) = line.split_once("impl_view_meta!(\"") else {
                continue;
            };
            let Some((name, _)) = rest.1.split_once('"') else {
                continue;
            };
            if name.is_empty() {
                continue;
            }
            found.push(Widget {
                name: name.to_string(),
                rust_type: view_type.clone(),
                file: file.clone(),
            });
        }
    }
    found
}

struct Widget {
    name: String,
    /// The type the enclosing `impl View for …` names, when there is one.
    rust_type: Option<String>,
    file: PathBuf,
}

/// The type an `impl … for <Type>` / `impl <Type>` header implements on, with
/// generics dropped. `None` when the line is not an impl header.
fn impl_target(line: &str) -> Option<String> {
    let header = line.trim_start().strip_prefix("impl")?;
    if !(header.starts_with(' ') || header.starts_with('<')) {
        return None;
    }
    let header = header.split('{').next()?;
    let header = header.split(" where ").next()?;
    let target = match header.split_once(" for ") {
        Some((_, target)) => target,
        None => skip_generics(header),
    };
    let name: String = target
        .trim()
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

/// `<T: Foo> Bar<T>` -> ` Bar<T>`: the parameter list of `impl<…>`.
fn skip_generics(header: &str) -> &str {
    if !header.starts_with('<') {
        return header;
    }
    let mut depth = 0;
    for (i, c) in header.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => {
                depth -= 1;
                if depth == 0 {
                    return &header[i + 1..];
                }
            }
            _ => {}
        }
    }
    header
}

/// The type of an `impl View for <Type>` header.
fn view_impl_target(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("impl") || !trimmed.contains(" View for ") {
        return None;
    }
    impl_target(line)
}

/// The bodies of every `impl` block in `text` whose target is `rust_type`.
///
/// Brace matching skips string and char literals and `//` comments, which is
/// enough for this crate's source - a `"{:02}"` format string or a `'{'` would
/// otherwise end a block early.
fn impl_blocks_of(text: &str, rust_type: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let start = offset;
        offset += line.len();
        if impl_target(line).as_deref() != Some(rust_type) {
            continue;
        }
        let Some(open) = text[start..].find('{').map(|i| start + i) else {
            continue;
        };
        if let Some(close) = matching_brace(text, open) {
            blocks.push(text[open..=close].to_string());
        }
    }
    blocks
}

fn matching_brace(text: &str, open: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut depth = 0usize;
    let mut i = open;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            b'"' => {
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    if bytes[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
            }
            // A char literal, not a lifetime: `'x'` or `'\x'`.
            b'\'' if bytes.get(i + 2) == Some(&b'\'') => i += 2,
            b'\'' if bytes.get(i + 1) == Some(&b'\\') => {
                i += 2;
                while i < bytes.len() && bytes[i] != b'\'' {
                    i += 1;
                }
            }
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// How many widgets one file declares, outside comments.
fn widgets_declared_in(file: &Path) -> usize {
    std::fs::read_to_string(file)
        .map(|t| {
            t.lines()
                .filter(|l| !l.trim_start().starts_with("//"))
                .filter(|l| l.contains("impl_view_meta!(\""))
                .count()
        })
        .unwrap_or(0)
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
///
/// A file that declares several widgets cannot credit them by the file either:
/// `Timer` reading CSS said nothing about `Stopwatch` beside it. There only the
/// widget's own `impl` blocks count - wherever in its module they live.
fn widget_reads_css(widget: &Widget) -> bool {
    let file = &widget.file;
    if widgets_declared_in(file) > 1 {
        return own_impls_read_css(widget);
    }
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

/// Does any `impl` block for this widget's type, anywhere in its module, read
/// a computed style?
fn own_impls_read_css(widget: &Widget) -> bool {
    let Some(rust_type) = &widget.rust_type else {
        return false;
    };
    let Some(dir) = widget.file.parent() else {
        return false;
    };
    let mut files = Vec::new();
    rust_files(dir, &mut files);
    files.iter().any(|f| {
        let text = std::fs::read_to_string(f).unwrap_or_default();
        impl_blocks_of(&text, rust_type)
            .iter()
            .any(|block| CSS_READERS.iter().any(|needle| block.contains(needle)))
    })
}

#[test]
fn every_widget_with_a_node_reads_its_computed_style() {
    let listed: BTreeSet<&str> = NOT_YET_READING_CSS.iter().copied().collect();

    let mut unwired_and_unlisted = Vec::new();
    let mut wired_but_listed = Vec::new();

    for widget in declared_widgets() {
        let reads = widget_reads_css(&widget);
        let name = widget.name;
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

/// The scan itself: a file declaring two widgets yields both, each tied to its
/// own type. Reading only the first per file is how `Stopwatch` and
/// `FormField` went unchecked.
#[test]
fn the_scan_sees_every_widget_in_a_file() {
    let widgets = declared_widgets();
    for (name, rust_type) in [
        ("Timer", "Timer"),
        ("Stopwatch", "Stopwatch"),
        ("Form", "Form"),
        ("FormField", "FormFieldWidget"),
    ] {
        let found = widgets.iter().find(|w| w.name == name);
        assert!(found.is_some(), "the scan missed {name}");
        assert_eq!(
            found.and_then(|w| w.rust_type.as_deref()),
            Some(rust_type),
            "{name} was tied to the wrong type"
        );
    }
    assert!(
        !widgets.iter().any(|w| w.name == "MyWidget"),
        "a doc example was counted as a widget"
    );
}
