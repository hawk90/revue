//! Doc examples marked `ignore` are never compiled, so they drift from the API
//! they show. This ratchet only lets their number go down (#800).
//!
//! - Adding an `ignore` example fails: write it as a running doctest, or
//!   `no_run` if it needs a real terminal.
//! - Converting some fails too, until [`CEILING`] is lowered to the new count,
//!   so the gain is kept.

use std::path::Path;

/// The number of `ignore` doc examples under `src/` that are allowed.
const CEILING: usize = 277;

/// Count the doc-comment lines that open an `ignore` code block.
fn count(dir: &Path, found: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).expect("read src") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            count(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = std::fs::read_to_string(&path).expect("read source");
            for (i, line) in text.lines().enumerate() {
                let line = line.trim_start();
                let Some(doc) = line
                    .strip_prefix("///")
                    .or_else(|| line.strip_prefix("//!"))
                else {
                    continue;
                };
                let doc = doc.trim_start();
                if let Some(attrs) = doc.strip_prefix("```") {
                    if attrs.split(',').any(|a| a.trim() == "ignore") {
                        found.push(format!("{}:{}", path.display(), i + 1));
                    }
                }
            }
        }
    }
}

#[test]
fn ignored_doc_examples_only_go_down() {
    let mut found = Vec::new();
    count(
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src")),
        &mut found,
    );
    let n = found.len();
    assert!(
        n <= CEILING,
        "{n} `ignore` doc examples, more than the {CEILING} allowed. Make the new \
         example a doctest (`no_run` if it needs a terminal). Newest candidates:\n{}",
        found
            .iter()
            .rev()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        n == CEILING,
        "only {n} `ignore` doc examples now - lower CEILING in {} from {CEILING} to {n}",
        file!()
    );
}
