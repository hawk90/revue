//! Every type that implements `View` in `src/` has a catalog entry, so a new
//! widget cannot skip the matrix.
//!
//! The source is scanned for `impl ... View for <Type>` lines; a type counts
//! as covered when a catalog entry carries its name. The module is built only
//! with every feature that gates a catalog entry (CI runs with
//! `--all-features`; see `mod.rs`).

use std::path::Path;

use super::catalog::catalog;

/// `View` implementors that are not public widgets, with the reason.
const NOT_WIDGETS: &[(&str, &str)] = &[
    ("Box", "the blanket impl for `Box<dyn View>`"),
    ("&V", "the blanket impl for a borrowed view"),
    ("HelloView", "a test view in `src/testing/test_app.rs`"),
    ("SimpleView", "a test view in `src/testing/pilot/mod.rs`"),
    ("TestView", "a test view in `src/core/app/mod.rs`"),
];

/// Catalog names for types that share a name with another `View`: the
/// catalog name, the type name and a fragment of the path that defines it.
const RENAMED: &[(&str, &str, &str)] =
    &[("RouterLink", "Link", "src/core/app/declarative_router/")];

/// The catalog name a type found at `at` is expected under.
fn catalog_name<'a>(name: &'a str, at: &str) -> &'a str {
    RENAMED
        .iter()
        .find(|(_, ty, path)| *ty == name && at.contains(path))
        .map_or(name, |(renamed, _, _)| renamed)
}

/// The type after `View for` on an impl line, if the line is one.
fn implementor(line: &str) -> Option<&str> {
    let line = line.trim_start();
    if !line.starts_with("impl") {
        return None;
    }
    let rest = &line[line.find(" View for ")? + " View for ".len()..];
    // A name, or `&` and a name for an impl on a borrowed type.
    let end = rest
        .char_indices()
        .find(|&(i, c)| !(c.is_alphanumeric() || c == '_' || (i == 0 && c == '&')))
        .map_or(rest.len(), |(i, _)| i);
    Some(&rest[..end])
}

fn scan(dir: &Path, found: &mut Vec<(String, String)>) {
    for entry in std::fs::read_dir(dir).expect("src is readable") {
        let path = entry.expect("a directory entry").path();
        if path.is_dir() {
            scan(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let source = std::fs::read_to_string(&path).expect("a source file");
            for (n, line) in source.lines().enumerate() {
                if let Some(name) = implementor(line) {
                    found.push((name.to_string(), format!("{}:{}", path.display(), n + 1)));
                }
            }
        }
    }
}

#[test]
fn every_view_in_src_is_in_the_catalog() {
    let mut found = Vec::new();
    scan(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut found,
    );
    assert!(found.len() > 100, "the scan found {} impls", found.len());

    let names: Vec<&str> = catalog().iter().map(|e| e.name).collect();
    let missing: Vec<String> = found
        .iter()
        .filter(|(name, at)| {
            !names.contains(&catalog_name(name, at)) && !NOT_WIDGETS.iter().any(|(n, _)| n == name)
        })
        .map(|(name, at)| format!("{name} ({at})"))
        .collect();
    assert!(
        missing.is_empty(),
        "View implementors without a catalog entry - add one to \
         tests/widget/catalog/matrix/catalog.rs, or list them in NOT_WIDGETS \
         with the reason:\n  {}",
        missing.join("\n  ")
    );

    let stale: Vec<&str> = NOT_WIDGETS
        .iter()
        .map(|(n, _)| *n)
        .filter(|n| !found.iter().any(|(name, _)| name == n))
        .collect();
    assert!(
        stale.is_empty(),
        "NOT_WIDGETS names no View implementor: {stale:?}"
    );
}
