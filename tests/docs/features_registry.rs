//! `docs/specs/features.yaml` is the feature list and its evidence. Nothing
//! else reads it, so a file that does not parse, or evidence that points at a
//! file a split turned into a directory, passed every check (#1216). These
//! tests run the checks `docs/specs/README.md` describes.

use std::collections::HashMap;
use std::path::Path;

use yaml_rust2::{Yaml, YamlLoader};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// The registry's `items`. Panics with the parser's message if the file does
/// not parse, which fails every test below.
fn items() -> Vec<Yaml> {
    let path = Path::new(ROOT).join("docs/specs/features.yaml");
    let text = std::fs::read_to_string(&path).expect("read features.yaml");
    let docs = YamlLoader::load_from_str(&text)
        .unwrap_or_else(|e| panic!("docs/specs/features.yaml does not parse: {e}"));
    docs[0]["items"]
        .as_vec()
        .expect("features.yaml has an `items` list")
        .clone()
}

fn id(item: &Yaml) -> &str {
    item["id"].as_str().expect("every item has a string `id`")
}

/// An item's `evidence` entries; an item without the field has none.
fn evidence(item: &Yaml) -> Vec<&str> {
    match &item["evidence"] {
        Yaml::BadValue | Yaml::Null => Vec::new(),
        Yaml::Array(entries) => entries
            .iter()
            .map(|e| {
                e.as_str()
                    .unwrap_or_else(|| panic!("{}: evidence entry {e:?} is not a string", id(item)))
            })
            .collect(),
        other => panic!("{}: `evidence` is not a list: {other:?}", id(item)),
    }
}

#[test]
fn ids_are_unique() {
    let mut seen: HashMap<String, usize> = HashMap::new();
    for item in items() {
        *seen.entry(id(&item).to_string()).or_default() += 1;
    }
    let mut dup: Vec<_> = seen.into_iter().filter(|(_, n)| *n > 1).collect();
    dup.sort();
    assert!(dup.is_empty(), "duplicate ids: {dup:?}");
}

#[test]
fn done_and_partial_items_have_evidence() {
    let missing: Vec<String> = items()
        .iter()
        .filter(|item| matches!(item["status"].as_str(), Some("done" | "partial")))
        .filter(|item| evidence(item).is_empty())
        .map(|item| id(item).to_string())
        .collect();
    assert!(
        missing.is_empty(),
        "done/partial items without evidence: {missing:?}"
    );
}

/// Evidence is `path`, `path:line`, `path:start-end` or `path::name`. The path
/// must exist, and for `path::name` the file must contain `name` as a whole
/// word.
#[test]
fn evidence_paths_exist_and_name_what_they_cite() {
    let mut bad = Vec::new();
    for item in items() {
        for entry in evidence(&item) {
            let (path, name) = match entry.split_once("::") {
                Some((path, name)) => (path, Some(name)),
                None => (entry, None),
            };
            let path = strip_line_suffix(path);
            let full = Path::new(ROOT).join(path);
            if !full.exists() {
                bad.push(format!("{}: {entry} (no such path)", id(&item)));
                continue;
            }
            if let Some(name) = name {
                if full.is_file() {
                    let text = std::fs::read_to_string(&full).expect("read evidence file");
                    if !contains_word(&text, name) {
                        bad.push(format!("{}: {entry} (`{name}` not in {path})", id(&item)));
                    }
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "evidence that does not hold:\n{}",
        bad.join("\n")
    );
}

/// `src/a.rs:12` and `src/a.rs:12-30` to `src/a.rs`.
fn strip_line_suffix(path: &str) -> &str {
    match path.rsplit_once(':') {
        Some((head, lines))
            if !lines.is_empty() && lines.split('-').all(|n| n.parse::<u32>().is_ok()) =>
        {
            head
        }
        _ => path,
    }
}

/// `word` occurs in `text` with no identifier character on either side.
fn contains_word(text: &str, word: &str) -> bool {
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    text.match_indices(word).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + word.len()..].chars().next();
        !before.is_some_and(ident) && !after.is_some_and(ident)
    })
}

#[test]
fn strip_line_suffix_keeps_paths_without_one() {
    assert_eq!(strip_line_suffix("src/a.rs:12"), "src/a.rs");
    assert_eq!(strip_line_suffix("src/a.rs:12-30"), "src/a.rs");
    assert_eq!(strip_line_suffix("src/a.rs"), "src/a.rs");
    assert_eq!(strip_line_suffix("src/a.rs:"), "src/a.rs:");
}

#[test]
fn contains_word_needs_a_boundary_on_both_sides() {
    assert!(contains_word("fn style(", "style"));
    assert!(!contains_word("fn stylesheet(", "style"));
    assert!(!contains_word("fn my_style(", "style"));
    assert!(contains_word("style", "style"));
}
