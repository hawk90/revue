//! A ratchet: public types keep their fields private (REV-RUST-011).
//!
//! Splitting a type's methods into sibling files tempts opening its fields as
//! `pub(super)` or `pub(crate)` so the new files compile. That leaks the
//! type's invariants to the whole parent module or crate: any file there can
//! set a cursor past the text or a selection past the list. The fix is to make
//! the new files child modules of the module that defines the type, which see
//! its private fields, and to open only the methods they call.
//!
//! 30 public types were split that way before this check existed, most in
//! the large-file splits of January to March 2026 (#270, #271, #272, #512).
//! They are listed below with their count of open fields.
//!
//! # Working on this
//!
//! Close a type's fields and lower or delete its entry. The test fails when a
//! type not listed has an open field, when a listed type has more than its
//! count, and when it has fewer, so the list cannot quietly go stale.
//!
//! Only `pub struct`s are checked. A private or `pub(super)` struct whose
//! fields are open is a value passed between files (TimeSeries' `Plot`, the
//! tree's visible `Row`), which is the alternative this check points to.

use std::collections::BTreeMap;
use std::path::Path;

/// (file under src/, type, open fields). Shrinks; never grows.
const OPEN_FIELDS: &[(&str, &str, usize)] = &[
    ("core/app/mod.rs", "App", 1),
    (
        "devtools/time_travel/debugger/mod.rs",
        "TimeTravelDebugger",
        2,
    ),
    ("runtime/dom/renderer/collect.rs", "CollectSink", 1),
    ("runtime/dom/renderer/types.rs", "DomRenderer", 11),
    ("runtime/render/terminal/types.rs", "Terminal", 4),
    ("testing/visual/mod.rs", "VisualTest", 3),
    ("utils/selection/types.rs", "Selection", 2),
    ("widget/canvas/draw.rs", "DrawContext", 2),
    ("widget/data/chart/heatmap/mod.rs", "HeatMap", 5),
    ("widget/data/datagrid/core.rs", "DataGrid", 1),
    ("widget/data/filetree/mod.rs", "FileTree", 15),
    ("widget/debug_overlay/event_log.rs", "EventLog", 2),
    ("widget/debug_overlay/metrics.rs", "PerfMetrics", 5),
    ("widget/developer/code_editor.rs", "CodeEditor", 21),
    ("widget/developer/httpclient/client.rs", "HttpClient", 11),
    ("widget/developer/httpclient/request.rs", "HttpRequest", 1),
    ("widget/feedback/menu/context_menu.rs", "ContextMenu", 4),
    ("widget/feedback/menu/menu_bar.rs", "MenuBar", 6),
    ("widget/feedback/popover/core.rs", "Popover", 14),
    ("widget/filepicker/mod.rs", "FilePicker", 23),
    ("widget/form/rich_text_editor/core.rs", "RichTextEditor", 24),
    ("widget/input/input_widgets/combobox/mod.rs", "Combobox", 26),
    ("widget/input/input_widgets/select/mod.rs", "Select", 18),
    ("widget/multi_select/types.rs", "MultiSelect", 13),
    ("widget/range_picker/core.rs", "RangePicker", 20),
    ("widget/traits/render_context/mod.rs", "PaintNode", 2),
];

/// `pub(super)`, `pub(crate)` and `pub(in ..)` fields of every `pub struct`
/// in `src/`, keyed by (file, type).
fn open_fields() -> BTreeMap<(String, String), usize> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut out = BTreeMap::new();
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let rel = path
                    .strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                let src = std::fs::read_to_string(&path).unwrap();
                for (name, count) in pub_struct_open_fields(&src) {
                    out.insert((rel.clone(), name), count);
                }
            }
        }
    }
    out
}

/// For each `pub struct Name { .. }` in `src`, its number of restricted-public fields.
fn pub_struct_open_fields(src: &str) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let lines: Vec<&str> = src.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim_start();
        if let Some(rest) = line.strip_prefix("pub struct ") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            // a braced struct whose body starts on this line
            if line.ends_with('{') {
                let indent = lines[i].len() - line.len();
                let mut count = 0;
                let mut j = i + 1;
                while j < lines.len() {
                    let l = lines[j];
                    if l.trim() == "}" && l.len() - l.trim_start().len() == indent {
                        break;
                    }
                    let t = l.trim_start();
                    if (t.starts_with("pub(super)")
                        || t.starts_with("pub(crate)")
                        || t.starts_with("pub(in "))
                        && t.contains(':')
                    {
                        count += 1;
                    }
                    j += 1;
                }
                if count > 0 {
                    out.push((name, count));
                }
                i = j;
            }
        }
        i += 1;
    }
    out
}

#[test]
fn public_types_open_no_new_fields() {
    let found = open_fields();
    let listed: BTreeMap<(String, String), usize> = OPEN_FIELDS
        .iter()
        .map(|&(f, t, n)| ((f.to_string(), t.to_string()), n))
        .collect();
    let mut problems = Vec::new();
    for (key, &n) in &found {
        match listed.get(key) {
            None => problems.push(format!(
                "{}::{}: {n} open field(s); keep them private (REV-RUST-011)",
                key.0, key.1
            )),
            Some(&m) if n > m => problems.push(format!(
                "{}::{}: {n} open fields, {m} listed; keep new fields private",
                key.0, key.1
            )),
            Some(&m) if n < m => problems.push(format!(
                "{}::{}: {n} open fields, {m} listed; lower the entry",
                key.0, key.1
            )),
            _ => {}
        }
    }
    for (key, m) in &listed {
        if !found.contains_key(key) {
            problems.push(format!(
                "{}::{}: no open fields now ({m} listed); delete the entry",
                key.0, key.1
            ));
        }
    }
    assert!(problems.is_empty(), "\n{}", problems.join("\n"));
}
