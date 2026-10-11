//! A ratchet: a type named in the docs should exist.
//!
//! Three separate documentation examples were found describing APIs that do not
//! exist — `focus_order()` and a `focus_manager` the `App` has never heard of,
//! a `ButtonStyle` where the real type is `ButtonVariant`, a `FilePickerMode`
//! where it is `PickerMode`, and a `style_registry.register_property` call for
//! custom CSS properties that are declared in the stylesheet. Each read as
//! real, working code.
//!
//! `cargo build --examples` covers `examples/`, and doctests cover `///`
//! comments. Nothing covered `docs/`, so this does the cheap half: every
//! `Type::` path in a Rust code fence names a type that is either defined in
//! this crate, or listed below with a reason.
//!
//! It cannot prove an example *runs*. It does catch the failure that actually
//! happened: a type invented while writing prose.
//!
//! # Working on this
//!
//! A new name has to go in one of the lists, and the lists say why. If it is a
//! revue type, the fix is the doc, not the list.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Types from the standard library and from dependencies.
///
/// Docs legitimately name these; this crate does not define them.
const EXTERNAL: &[&str] = &[
    // std
    "Arc",
    "Box",
    "Default",
    "Duration",
    "Err",
    "HashMap",
    "Instant",
    "None",
    "Ok",
    "Option",
    "Ordering",
    "PathBuf",
    "Rc",
    "RefCell",
    "Result",
    "Self",
    "Some",
    "String",
    "Vec",
    "f64",
    // dependencies named in TECH_STACK.md and the performance guide
    "ClearType",
    "ImageReader",
    "KeyCode",
    "Parser",
    "UnicodeWidthChar",
];

/// Types an example defines for itself.
///
/// A tutorial builds an application, and that application has its own types.
/// They are not API and are not expected to exist in `src/`.
const EXAMPLE_LOCAL: &[&str] = &[
    "AnotherPlugin",
    "Counter",
    "CounterStore",
    "MyApp",
    "MyConfig",
    "MyForm",
    "MyPlugin",
    "MyWidget",
    "Todo",
    "TodoApp",
    "TodoFilter",
    // guides/testing.md builds a small application per section
    "AnimatedWidget",
    "Dashboard",
    "DataLoader",
    "LoginForm",
    "MyAsyncView",
    // the plugin guides and the CLI guide each write their own plugin
    "GitPlugin",
    "GitStatusPlugin",
    "MyFeaturePlugin",
    // architecture-review.md discusses a TEA-style design revue does not use,
    // so its `Msg` is the shape being argued about rather than an API
    "Msg",
];

/// Placeholders that stand for "any widget" in a pattern description.
///
/// `constructor-patterns.md` shows the *shape* a constructor should have, so
/// its template names a type that deliberately does not exist.
const PLACEHOLDER: &[&str] = &["Widget"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn markdown_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            markdown_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
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

/// Every `struct`, `enum` and `trait` this crate defines.
fn defined_types() -> BTreeSet<String> {
    let mut files = Vec::new();
    rust_files(&repo_root().join("src"), &mut files);

    let mut out = BTreeSet::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        for line in text.lines() {
            let line = line.trim_start();
            for keyword in [
                "pub struct ",
                "pub enum ",
                "pub trait ",
                "struct ",
                "enum ",
                "trait ",
                "pub type ",
                "type ",
            ] {
                if let Some(rest) = line.strip_prefix(keyword) {
                    let name: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !name.is_empty() {
                        out.insert(name);
                    }
                    break;
                }
            }
        }
    }
    out
}

/// The Rust code fences in one markdown file.
fn rust_fences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(block) = current.as_mut() {
            if trimmed.starts_with("```") {
                out.push(std::mem::take(block));
                current = None;
            } else {
                block.push_str(line);
                block.push('\n');
            }
        } else if trimmed == "```rust" || trimmed == "```rs" || trimmed == "```" {
            // A bare fence in these docs is Rust often enough to be worth
            // reading; a false positive here shows up as an unknown name and
            // gets listed, which is cheap. A missed fence is a defect that
            // stays hidden, which is not.
            if trimmed != "```" {
                current = Some(String::new());
            }
        }
    }
    out
}

/// Every `Something::` path in a code fence.
fn type_paths(block: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let bytes: Vec<char> = block.chars().collect();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == ':' && bytes[i + 1] == ':' {
            // Walk backwards over the identifier.
            let mut start = i;
            while start > 0 && (bytes[start - 1].is_alphanumeric() || bytes[start - 1] == '_') {
                start -= 1;
            }
            if start < i && bytes[start].is_uppercase() {
                out.insert(bytes[start..i].iter().collect());
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

#[test]
fn every_type_named_in_the_docs_exists() {
    let defined = defined_types();
    let known: BTreeSet<&str> = EXTERNAL
        .iter()
        .chain(EXAMPLE_LOCAL)
        .chain(PLACEHOLDER)
        .copied()
        .collect();

    let mut files = Vec::new();
    markdown_files(&repo_root().join("docs"), &mut files);
    files.sort();

    let mut unknown: Vec<String> = Vec::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        let shown = file
            .strip_prefix(repo_root())
            .unwrap_or(&file)
            .display()
            .to_string();
        for block in rust_fences(&text) {
            for name in type_paths(&block) {
                if !defined.contains(&name) && !known.contains(name.as_str()) {
                    unknown.push(format!("{shown}: {name}::"));
                }
            }
        }
    }
    unknown.sort();
    unknown.dedup();

    assert!(
        unknown.is_empty(),
        "these types are named in the docs and defined nowhere in `src/`.\n\
         If it is a revue type the doc is wrong - find the real name and fix the \
         doc. If it belongs to std or a dependency, add it to EXTERNAL; if an \
         example defines it, add it to EXAMPLE_LOCAL.\n{unknown:#?}"
    );
}

/// The lists cannot go stale: a name that turns out to exist after all has to
/// come off them, or the next reader trusts a stale exemption.
#[test]
fn the_exemption_lists_hold_nothing_that_exists() {
    let defined = defined_types();
    let stale: Vec<&str> = EXAMPLE_LOCAL
        .iter()
        .chain(PLACEHOLDER)
        .copied()
        .filter(|name| defined.contains(*name))
        .collect();

    assert!(
        stale.is_empty(),
        "these are listed as not existing, but `src/` defines them - drop them \
         from the list: {stale:?}"
    );
}
