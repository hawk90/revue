//! Models of what browsing, selecting, typing and filtering should do
//!
//! The trees live under the target's scratch directory, built the same way by
//! every test process, and are entered by their canonical path so that
//! symlinks in the temp path play no part.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{file_picker, save_picker, FileFilter, FilePicker, PickerResult, View};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// `base/root` with `d1/{d3/z.rs, x.txt, .y}`, an empty `d2/`, a hidden
/// `.hd/h.txt`, and the files `a.txt`, `B.rs` and `.c`.
fn tree() -> PathBuf {
    static TREE: OnceLock<PathBuf> = OnceLock::new();
    TREE.get_or_init(|| {
        let base = Path::new(env!("CARGO_TARGET_TMPDIR")).join("filepicker-model");
        let root = base.join("root");
        for d in ["d1/d3", "d2", ".hd"] {
            fs::create_dir_all(root.join(d)).unwrap();
        }
        for f in [
            "d1/d3/z.rs",
            "d1/x.txt",
            "d1/.y",
            ".hd/h.txt",
            "a.txt",
            "B.rs",
            ".c",
        ] {
            fs::write(root.join(f), "x").unwrap();
        }
        base.canonicalize().unwrap()
    })
    .clone()
}

/// What each directory lists, directories first, then by name ignoring case.
/// A `/` marks a directory.
fn listing(rel: &[String], hidden: bool) -> Vec<&'static str> {
    let rel: Vec<&str> = rel.iter().map(String::as_str).collect();
    match (rel.as_slice(), hidden) {
        ([], _) => vec!["root/"],
        (["root"], false) => vec!["d1/", "d2/", "a.txt", "B.rs"],
        (["root"], true) => vec![".hd/", "d1/", "d2/", ".c", "a.txt", "B.rs"],
        (["root", "d1"], false) => vec!["d3/", "x.txt"],
        (["root", "d1"], true) => vec!["d3/", ".y", "x.txt"],
        (["root", "d1", "d3"], _) => vec!["z.rs"],
        (["root", "d2"], _) => vec![],
        (["root", ".hd"], _) => vec!["h.txt"],
        other => panic!("not in the tree: {other:?}"),
    }
}

fn draw(picker: &FilePicker, width: u16, height: u16) -> Vec<String> {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    picker.render(&mut ctx);
    (0..height)
        .map(|y| {
            (0..width)
                .filter_map(|x| buffer.get(x, y))
                .filter(|c| !c.is_continuation())
                .map(|c| c.symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

/// The names of the entry rows on screen, in order.
fn shown_names(screen: &[String]) -> Vec<String> {
    screen
        .iter()
        .filter(|r| r.starts_with("  📁 ") || r.starts_with("  📄 ") || r.starts_with("✓ "))
        .map(|r| r.chars().skip(4).collect::<String>())
        .map(|r| r.split_whitespace().next().unwrap_or("").to_string())
        .filter(|n| n != "..")
        .collect()
}

#[derive(Clone, Debug)]
enum Nav {
    Next,
    Prev,
    Enter,
    Up,
    Back,
    Forward,
    Hidden,
}

fn nav() -> impl Strategy<Value = Nav> {
    prop_oneof![
        3 => Just(Nav::Next),
        2 => Just(Nav::Prev),
        3 => Just(Nav::Enter),
        1 => Just(Nav::Up),
        2 => Just(Nav::Back),
        2 => Just(Nav::Forward),
        1 => Just(Nav::Hidden),
    ]
}

proptest! {
    /// Browsing follows a plain model of the tree: the directory shown, the
    /// back/forward history, the highlighted entry, the scroll window drawn
    /// on screen, and what Enter returns on a file.
    #[test]
    fn prop_filepicker_browsing_follows_the_model(
        start in prop::sample::select(vec![vec![], vec!["root"], vec!["root", "d1"]]),
        max_visible in 1usize..5,
        ops in prop::collection::vec(nav(), 0..40),
    ) {
        let base = tree();
        let mut cur: Vec<String> = start.iter().map(|s| s.to_string()).collect();
        let at = |rel: &[String]| rel.iter().fold(base.clone(), |p, s| p.join(s));
        let mut picker = file_picker().start_dir(at(&cur)).max_visible(max_visible);
        let mut history = vec![cur.clone()];
        let mut idx = 0usize;
        let mut hidden = false;
        let (mut hl, mut offset) = (0usize, 0usize);

        for op in &ops {
            let list = listing(&cur, hidden);
            match op {
                Nav::Next => {
                    picker.highlight_next();
                    if hl + 1 < list.len() { hl += 1; }
                }
                Nav::Prev => {
                    picker.highlight_previous();
                    hl = hl.saturating_sub(1);
                }
                Nav::Enter => {
                    let got = picker.enter();
                    match list.get(hl) {
                        Some(name) if name.ends_with('/') => {
                            prop_assert!(got.is_none());
                            history.truncate(idx + 1);
                            cur.push(name.trim_end_matches('/').to_string());
                            history.push(cur.clone());
                            idx = history.len() - 1;
                            hl = 0;
                        }
                        Some(name) => {
                            let want = at(&cur).join(name);
                            prop_assert!(
                                matches!(&got, Some(PickerResult::Selected(p)) if *p == want),
                                "{:?}", got
                            );
                        }
                        None => prop_assert!(got.is_none()),
                    }
                }
                Nav::Up => {
                    // Above the base the listing is the machine's.
                    if cur.is_empty() { continue; }
                    picker.go_up();
                    history.truncate(idx + 1);
                    cur.pop();
                    history.push(cur.clone());
                    idx = history.len() - 1;
                    hl = 0;
                }
                Nav::Back => {
                    picker.go_back();
                    if idx > 0 { idx -= 1; cur = history[idx].clone(); hl = 0; }
                }
                Nav::Forward => {
                    picker.go_forward();
                    if idx + 1 < history.len() { idx += 1; cur = history[idx].clone(); hl = 0; }
                }
                Nav::Hidden => {
                    picker.toggle_hidden();
                    hidden = !hidden;
                    hl = 0;
                }
            }
            if hl == 0 { offset = 0; }
            if hl < offset { offset = hl; }
            if hl >= offset + max_visible { offset = hl + 1 - max_visible; }

            let list = listing(&cur, hidden);
            let want_dir = at(&cur);
            prop_assert_eq!(picker.current_dir(), want_dir.as_path(), "after {:?}", op);
            prop_assert_eq!(
                picker.highlighted_entry().map(|e| e.name.clone()),
                list.get(hl).map(|n| n.trim_end_matches('/').to_string()),
                "after {:?}", op
            );
            let screen = draw(&picker, 300, 20);
            let window: Vec<String> = list
                .iter()
                .skip(offset)
                .take(max_visible)
                .map(|n| n.trim_end_matches('/').to_string())
                .collect();
            prop_assert_eq!(shown_names(&screen), window, "after {:?}", op);
            prop_assert_eq!(screen.iter().any(|r| r == "  ↑ more..."), offset > 0);
            prop_assert_eq!(
                screen.iter().any(|r| r == "  ↓ more..."),
                offset + max_visible < list.len()
            );
        }
    }
}

#[derive(Clone, Debug)]
enum Pick {
    Next,
    Prev,
    Toggle,
    Enter,
}

fn pick() -> impl Strategy<Value = Pick> {
    prop_oneof![
        Just(Pick::Next),
        Just(Pick::Prev),
        Just(Pick::Toggle),
        Just(Pick::Enter),
    ]
}

proptest! {
    /// Multi-select keeps the files toggled on, in the order chosen, marks
    /// their rows and counts them; `confirm` returns their paths. A directory
    /// of files only: entering a directory refreshes the listing, which
    /// drops the marks (#894).
    #[test]
    fn prop_filepicker_multi_select_follows_the_model(ops in prop::collection::vec(pick(), 0..40)) {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("filepicker-multi");
        fs::create_dir_all(&dir).unwrap();
        let names = ["p0.txt", "p1.txt", "p2.txt", "p3.txt", "p4.txt"];
        for n in names {
            fs::write(dir.join(n), "x").unwrap();
        }
        let dir = dir.canonicalize().unwrap();
        let mut picker = FilePicker::multi_select().start_dir(&dir);
        let mut hl = 0usize;
        let mut chosen: Vec<&str> = Vec::new();

        for op in &ops {
            match op {
                Pick::Next => { picker.highlight_next(); hl = (hl + 1).min(names.len() - 1); }
                Pick::Prev => { picker.highlight_previous(); hl = hl.saturating_sub(1); }
                Pick::Toggle | Pick::Enter => {
                    if matches!(op, Pick::Toggle) {
                        picker.toggle_selection();
                    } else {
                        prop_assert!(picker.enter().is_none());
                    }
                    if let Some(i) = chosen.iter().position(|n| *n == names[hl]) {
                        chosen.remove(i);
                    } else {
                        chosen.push(names[hl]);
                    }
                }
            }
            match picker.confirm() {
                PickerResult::None => prop_assert!(chosen.is_empty()),
                PickerResult::Multiple(paths) => {
                    let want: Vec<PathBuf> = chosen.iter().map(|n| dir.join(n)).collect();
                    prop_assert_eq!(paths, want);
                }
                other => prop_assert!(false, "{:?}", other),
            }
            let screen = draw(&picker, 90, 14);
            for n in names {
                let marked = screen.iter().any(|r| r.starts_with("✓ 📄 ") && r.contains(n));
                prop_assert_eq!(marked, chosen.contains(&n), "{} after {:?}", n, op);
            }
            let count = format!("Selected: {} files", chosen.len());
            prop_assert_eq!(screen.contains(&count), !chosen.is_empty());
        }
    }

    /// In save mode `confirm` returns the current directory joined with the
    /// typed name when it is one file name, and nothing when it is empty,
    /// `.`, `..` or has a separator (#1110); the footer shows the name.
    /// Other modes ignore typing.
    #[test]
    fn prop_filepicker_save_name_follows_the_typing(
        keys in prop::collection::vec(
            prop_oneof![
                4 => prop::sample::select(vec!['a', 'Z', '_', '한', '😀', ' ', '-', '.', '.', '/', '\\']).prop_map(Some),
                1 => Just(None),
            ],
            0..20,
        ),
    ) {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("filepicker-save");
        fs::create_dir_all(&dir).unwrap();
        let dir = dir.canonicalize().unwrap();
        let mut save = save_picker().start_dir(&dir);
        let mut open = file_picker().start_dir(&dir);
        let mut name = String::new();
        for key in &keys {
            match key {
                Some(c) => { save.input_char(*c); open.input_char(*c); name.push(*c); }
                None => { save.input_backspace(); open.input_backspace(); name.pop(); }
            }
        }
        // One file name: "." and ".." name the directory itself or its
        // parent, and a separator names another directory
        let file_name = !name.is_empty()
            && name != "."
            && name != ".."
            && !name.chars().any(std::path::is_separator);
        match save.confirm() {
            PickerResult::None => prop_assert!(!file_name, "{:?}", name),
            PickerResult::Selected(p) => {
                prop_assert!(file_name, "{:?}", name);
                prop_assert_eq!(p, dir.join(&name));
            }
            other => prop_assert!(false, "{:?}", other),
        }
        let footer = format!("Filename: {name}_");
        prop_assert!(draw(&save, 120, 10).iter().any(|r| *r == footer.trim_end()));
        prop_assert!(matches!(open.confirm(), PickerResult::None));
        prop_assert!(!draw(&open, 120, 10).iter().any(|r| r.starts_with("Filename:")));
    }
}

/// `mix/` holds a directory, hidden files, files without an extension and
/// with two, in mixed case.
const MIX: [&str; 10] = [
    "a.rs",
    "b.RS",
    "c.txt",
    "d.md",
    "e.toml",
    "f",
    "rs",
    "h.tar.gz",
    ".g.rs",
    "test_x.md",
];

fn mix() -> PathBuf {
    static MIX_DIR: OnceLock<PathBuf> = OnceLock::new();
    MIX_DIR
        .get_or_init(|| {
            let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("filepicker-mix");
            fs::create_dir_all(dir.join("sub")).unwrap();
            for n in MIX {
                fs::write(dir.join(n), "x").unwrap();
            }
            dir.canonicalize().unwrap()
        })
        .clone()
}

fn listed(picker: &mut FilePicker) -> Vec<String> {
    let mut names = Vec::new();
    while let Some(e) = picker.highlighted_entry() {
        if names.last() == Some(&e.name) {
            break;
        }
        names.push(e.name.clone());
        picker.highlight_next();
    }
    names
}

/// The extension as `Path::extension` reads it: after the last dot, unless
/// that dot starts the name.
fn extension(name: &str) -> Option<&str> {
    let dot = name.rfind('.')?;
    (dot > 0).then(|| &name[dot + 1..])
}

fn sorted(mut files: Vec<&str>) -> Vec<String> {
    files.sort_by_key(|n| n.to_lowercase());
    std::iter::once("sub")
        .chain(files)
        .map(str::to_string)
        .collect()
}

proptest! {
    /// An extension filter lists the directory and the files whose extension
    /// is one of the given ones, ignoring case; hidden files only when shown.
    #[test]
    fn prop_filepicker_extension_filter_follows_the_model(
        exts in prop::sample::subsequence(vec!["rs", "TXT", "md", "toml", "gz", "tar"], 0..=6),
        hidden in any::<bool>(),
    ) {
        let dir = mix();
        let mut picker = file_picker()
            .start_dir(&dir)
            .show_hidden(hidden)
            .filter(FileFilter::extensions(&exts));
        let want = sorted(
            MIX.iter()
                .copied()
                .filter(|n| hidden || !n.starts_with('.'))
                .filter(|n| extension(n).is_some_and(|e| exts.iter().any(|x| x.eq_ignore_ascii_case(e))))
                .collect(),
        );
        prop_assert_eq!(listed(&mut picker), want);
    }

    /// A pattern filter: `*x` keeps names ending in `x`, `x*` names starting
    /// with it, anything else the exact name. Directories always stay.
    #[test]
    fn prop_filepicker_pattern_filter_follows_the_model(
        pattern in prop::sample::select(vec![
            "*.rs", "*.RS", "*s", "*", "test*", "a*", "*.md", "rs", "f", "c.txt", "nothing", "*.gz",
        ]),
    ) {
        let dir = mix();
        let mut picker = file_picker()
            .start_dir(&dir)
            .filter(FileFilter::pattern(pattern));
        let matches = |n: &str| {
            if let Some(suffix) = pattern.strip_prefix('*') {
                n.ends_with(suffix)
            } else if let Some(prefix) = pattern.strip_suffix('*') {
                n.starts_with(prefix)
            } else {
                n == pattern
            }
        };
        let want = sorted(MIX.iter().copied().filter(|n| !n.starts_with('.') && matches(n)).collect());
        prop_assert_eq!(listed(&mut picker), want);
    }
}
