//! The parser entry points the fault layer drives, with each format's
//! tokens and the corpus of nasty inputs. Shared with the `fuzz` crate
//! (`fuzz/src/lib.rs` includes this file), so it uses only `revue` and
//! `std`.

use std::collections::HashMap;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::{RenderContext, View};

/// Render a view into a small buffer.
fn paint(view: &dyn View) {
    let mut buffer = Buffer::new(40, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 10));
    view.render(&mut ctx);
}

// ─── Targets ────────────────────────────────────────────────────────────────

fn css(s: &str) {
    use revue::style::{parse_css, Style};
    if let Ok(sheet) = parse_css(s) {
        let base = Style::default();
        for rule in &sheet.rules {
            let _ = sheet.apply(&rule.selector, &base);
            let _ = sheet.animation(&rule.selector);
            let _ = revue::dom::parse_selectors(&rule.selector);
        }
        for name in sheet.keyframes.keys() {
            let _ = sheet.keyframes_definition(name);
        }
    }
}

fn selector(s: &str) {
    let _ = revue::dom::parse_selector(s);
    let _ = revue::dom::parse_selectors(s);
}

const PROPERTIES: &[&str] = &[
    "color",
    "background",
    "border-color",
    "width",
    "height",
    "padding",
    "margin",
    "gap",
    "flex",
    "grid-template-columns",
    "grid-column",
    "animation",
    "animation-duration",
    "opacity",
    "z-index",
];

fn declaration(s: &str) {
    use revue::style::{apply_declaration, Style};
    let mut vars = HashMap::new();
    vars.insert("--a".to_string(), s.to_string());
    vars.insert("--self".to_string(), "var(--self)".to_string());
    vars.insert("--b".to_string(), "var(--a)".to_string());
    let mut style = Style::default();
    for property in PROPERTIES {
        apply_declaration(&mut style, property, s, &vars);
    }
}

fn color(s: &str) {
    use revue::style::{apply_declaration, Style};
    let vars = HashMap::new();
    let mut style = Style::default();
    for property in ["color", "background", "background-color", "border-color"] {
        apply_declaration(&mut style, property, s, &vars);
    }
}

fn transition(s: &str) {
    use revue::style::{Easing, Transition, Transitions};
    let _ = Easing::parse(s);
    let _ = Transition::parse(s);
    let all = Transitions::parse(s);
    let _ = all.has("opacity");
}

#[cfg(feature = "markdown")]
fn markdown(s: &str) {
    let md = revue::widget::Markdown::new(s);
    let _ = md.toc();
    let _ = md.line_count();
    paint(&md);
    let _ = revue::widget::parse_slides(s);
}

fn rich_text(s: &str) {
    let text = revue::widget::RichText::markup(s);
    let _ = text.width();
    paint(&text);
}

fn keymap(s: &str) {
    if let Some(binding) = revue::utils::parse_key_binding(s) {
        let _ = revue::utils::parse_key_binding(&revue::utils::format_key_binding(&binding));
    }
    let _ = revue::utils::KeyChord::parse(s);
}

fn mermaid(s: &str) {
    let mut diagram = revue::widget::Diagram::new().parse(s);
    diagram.compute_layout(40, 10);
    paint(&diagram);
}

fn log_viewer(s: &str) {
    let mut viewer = revue::widget::LogViewer::new().show_line_numbers(true);
    viewer.load(s);
    viewer.push(s);
    paint(&viewer);
    let parser = revue::widget::LogParser::new().json_parsing(true);
    for (i, line) in s.lines().take(64).enumerate() {
        let _ = parser.parse(line, i);
    }
}

struct Item;

impl revue::query::Queryable for Item {
    fn field_value(&self, field: &str) -> Option<revue::query::QueryValue> {
        use revue::query::QueryValue;
        match field {
            "name" => Some(QueryValue::String("Alice 한글".into())),
            "age" => Some(QueryValue::Int(30)),
            "score" => Some(QueryValue::Float(1.5)),
            "active" => Some(QueryValue::Bool(true)),
            "date" | "created" => Some(QueryValue::Date("2024-01-01".into())),
            _ => None,
        }
    }

    fn full_text(&self) -> String {
        "Alice 한글 30".into()
    }
}

fn query(s: &str) {
    if let Ok(q) = revue::query::Query::parse(s) {
        let _ = q.matches(&Item);
    }
}

fn terminal_ansi(s: &str) {
    let mut term = revue::widget::Terminal::new(40, 10);
    term.write(s);
    paint(&term);
}

fn ansi(s: &str) {
    let _ = revue::utils::parse_ansi(s);
    let _ = revue::utils::strip_ansi(s);
    let _ = revue::utils::ansi_len(s);
}

fn json(s: &str) {
    let viewer = revue::widget::JsonViewer::from_content(s);
    let _ = viewer.visible_count();
    paint(&viewer);
}

fn time_travel(s: &str) {
    let mut debugger = revue::devtools::TimeTravelDebugger::new();
    // Bare JSON values, and the same text as a session's state.
    if debugger.import_json(s).is_ok() {
        let _ = debugger.export();
    }
    let session = format!(r#"{{"snapshots": [{{"id": 0, "state": {s}}}]}}"#);
    if debugger.import_json(&session).is_ok() {
        let _ = debugger.export();
    }
}

fn csv(s: &str) {
    let viewer = revue::widget::CsvViewer::from_content(s);
    let _ = (viewer.row_count(), viewer.column_count());
    paint(&viewer);
}

#[cfg(feature = "syntax-highlighting")]
fn tree_sitter(s: &str) {
    // One grammar, on inputs up to 100 KB: tree-sitter's own C parser is
    // slow in a debug build (half a megabyte of markup takes it over a
    // second), and what is revue's here - spreading the spans over the
    // lines - is the same for every language and shows at this size
    if s.len() > 100_000 {
        return;
    }
    // Nor brackets nested thousands deep: tree-sitter takes time quadratic
    // in the depth there (10 000 took Windows CI's debug build over 3 s),
    // inside its own code, before any span reaches revue's (#1319)
    let mut depth: usize = 0;
    let mut deepest = 0;
    for c in s.chars() {
        match c {
            '[' | '{' => depth += 1,
            ']' | '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
        deepest = deepest.max(depth);
    }
    if deepest > 1000 {
        return;
    }
    let mut h = revue::widget::TreeSitterHighlighter::new(revue::widget::Language::Json);
    let _ = h.highlight_code(s);
}

fn syntax(s: &str) {
    for lang in ["rust", "python", "js", "json", "sh", "unknown"] {
        let _ = revue::utils::highlight(s, lang);
    }
}

// ─── Token soups ────────────────────────────────────────────────────────────

const CSS_TOKENS: &[&str] = &[
    "a",
    ".x",
    "#id",
    "*",
    ">",
    "+",
    "~",
    " ",
    ":hover",
    ":nth-child(2n+1)",
    ":not(",
    "::before",
    "[data=x]",
    "{",
    "}",
    ";",
    ":",
    "color",
    "background",
    "width",
    "red",
    "#fff",
    "#12345678",
    "rgb(1,2,3)",
    "rgba(",
    "hsl(",
    ")",
    ",",
    "var(--a)",
    "var(--a, red)",
    "var(",
    "--a",
    "--a: var(--a);",
    "@keyframes k",
    "from",
    "to",
    "50%",
    "@media",
    "@import",
    "animation: k 1s",
    "transition: color 0.3s ease",
    "1s",
    "200ms",
    "-1ms",
    "cubic-bezier(0.1,",
    "calc(100% - 2px)",
    "/*",
    "*/",
    "\"",
    "'",
    "\\",
    "!important",
    "\n",
    "é",
    "漢",
    "0",
    "-",
    "%",
    "fr",
    "auto",
];

#[cfg(feature = "markdown")]
const MARKDOWN_TOKENS: &[&str] = &[
    "#",
    "##",
    "###### ",
    "*",
    "**",
    "_",
    "`",
    "```",
    "```rust\n",
    "\n",
    "\n\n",
    "- ",
    "1. ",
    "> ",
    "[",
    "]",
    "(",
    ")",
    "![",
    "|",
    "---",
    "|---|",
    "<div>",
    "</div>",
    "\\",
    "~~",
    "[^1]",
    "[^1]: ",
    "- [ ] ",
    "- [x] ",
    "    ",
    "\t",
    "&amp;",
    "<!--",
    "-->",
    "http://x",
    "\u{301}",
    "漢",
    " ",
    "text",
    "===",
];

const MARKUP_TOKENS: &[&str] = &[
    "[",
    "]",
    "[/]",
    "[bold]",
    "[red]",
    "[on blue]",
    "[link=",
    "http://x]",
    "[/bold]",
    "[b i u]",
    "[#ff0000]",
    "text",
    "\\[",
    "[[",
    "]]",
    "[/",
    "=",
    "\n",
    "漢",
    "😀",
    " ",
    "[dim italic]",
];

const KEY_TOKENS: &[&str] = &[
    "ctrl", "alt", "shift", "meta", "super", "+", "-", "a", "A", "F1", "F24", "F0", "F99", "F256",
    "f", "esc", "enter", "space", "tab", "backtab", "up", "pgdn", "home", " ", "++", "+-", "ctrl+",
    "é", "漢", "\0", "C-", "M-", "<", ">",
];

const MERMAID_TOKENS: &[&str] = &[
    "graph TD",
    "flowchart LR",
    "\n",
    "A",
    "B",
    "C",
    "-->",
    "---",
    "-.->",
    "==>",
    "|",
    "label",
    "[",
    "]",
    "(",
    ")",
    "{",
    "}",
    "((",
    "))",
    "%%",
    " ",
    ";",
    "A[",
    "-->|x|",
    "subgraph",
    "end",
    "漢",
    "A-->A",
];

const LOG_TOKENS: &[&str] = &[
    "2024-01-01T00:00:00Z",
    "[INFO]",
    "ERROR",
    "WARN",
    "debug",
    " ",
    "{",
    "}",
    "\"level\"",
    ":",
    "\"msg\"",
    "\"x\"",
    ",",
    "\n",
    "\r\n",
    "\t",
    "[",
    "]",
    "12:00:00",
    "\u{1b}[31m",
    "漢",
    "\"timestamp\"",
    "null",
    "1e999",
];

const QUERY_TOKENS: &[&str] = &[
    "name",
    ":",
    "~",
    ":!",
    ">",
    "<",
    ">=",
    "<=",
    "=",
    "age",
    "18",
    "\"",
    "quoted",
    "before:",
    "after:",
    "2024-01-01",
    " ",
    "-",
    "AND",
    "OR",
    "(",
    ")",
    "*",
    "é",
    "\\",
    "sort:",
    "score",
    "1.5",
    "active",
    "true",
];

const ANSI_TOKENS: &[&str] = &[
    "\x1b",
    "[",
    "]",
    "\x1b[",
    "\x1b]",
    "0;title",
    "\x07",
    "\x1b\\",
    "31",
    "38;5;",
    "38;2;1;2;3",
    "m",
    "H",
    "J",
    "K",
    "A",
    "B",
    "C",
    "D",
    "?25l",
    "?1049h",
    ";",
    "9999",
    "99999999999",
    "\r",
    "\n",
    "\t",
    "\x08",
    "x",
    "漢",
    "😀",
    "\u{301}",
    "8;;http://x",
    "\x1bc",
    "@",
    "L",
    "M",
    "P",
    "r",
];

const JSON_TOKENS: &[&str] = &[
    "{", "}", "[", "]", "\"", "a", "\\", "\\u", "D800", "\\uDC00", ":", ",", "1", "-", "1e999",
    "0.", "true", "false", "null", " ", "\n", "漢", "\"k\":",
];

const CSV_TOKENS: &[&str] = &[
    ",", ";", "\t", "\"", "\"\"", "a", "1", "\n", "\r\n", "\r", " ", "漢", "|",
];

const COLOR_TOKENS: &[&str] = &[
    "#",
    "f",
    "0",
    "ff",
    "rgb(",
    "rgba(",
    "hsl(",
    "hsla(",
    ")",
    ",",
    "%",
    "-",
    "1",
    "255",
    "256",
    "999999999999",
    "0.5",
    "red",
    "transparent",
    "var(--c)",
    " ",
    ".",
    "e",
    "deg",
    "/",
];

const TRANSITION_TOKENS: &[&str] = &[
    "opacity",
    "color",
    "all",
    " ",
    "0.3s",
    "200ms",
    "-1s",
    "1e9s",
    "1e400s",
    "s",
    "ms",
    "ease",
    "ease-in-out",
    "linear",
    "cubic-bezier(",
    "0.1",
    ",",
    ")",
    "steps(4)",
    "NaN",
    "inf",
    "-",
    ".",
    "0",
];

const SYNTAX_TOKENS: &[&str] = &[
    "fn", "let", "\"", "'", "/*", "*/", "//", "#", "\n", " ", "{", "}", "r#\"", "\"#", "\\", "`",
    "${", "0x", "1e", "漢", "<<EOF", "'''",
];

// ─── Corpus ─────────────────────────────────────────────────────────────────

/// Bytes from a fixed xorshift stream, decoded lossily.
fn lossy(seed: u64, len: usize) -> String {
    let mut x = seed;
    let bytes: Vec<u8> = (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8
        })
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// The nasty inputs every target is run on, by name
pub fn corpus() -> Vec<(&'static str, String)> {
    let deep = 10_000;
    let mut c: Vec<(&'static str, String)> = vec![
        ("empty", String::new()),
        ("space", " ".into()),
        ("newline", "\n".into()),
        ("huge line", "x".repeat(200_000)),
        // Many tokens on one line, as minified code: a highlighter that
        // measures each token from the line's start is quadratic in it
        ("tokens on one line", format!("[{}0]", "1,".repeat(40_000))),
        ("many lines", "line\n".repeat(20_000)),
        ("[ x10000", "[".repeat(deep)),
        ("] x10000", "]".repeat(deep)),
        (
            "[] nested",
            format!("{}{}", "[".repeat(deep), "]".repeat(deep)),
        ),
        (
            "( nested",
            format!("{}{}", "(".repeat(deep), ")".repeat(deep)),
        ),
        (
            "{ nested",
            format!("{}{}", "{".repeat(deep), "}".repeat(deep)),
        ),
        (
            "json nested",
            format!("{}1{}", "{\"a\":".repeat(deep), "}".repeat(deep)),
        ),
        (
            "json array nested",
            format!("{}{}", "[".repeat(deep), "]".repeat(deep)),
        ),
        ("quote nested", "> ".repeat(deep)),
        (
            "list nested",
            (0..500)
                .map(|i| format!("{}- x\n", "  ".repeat(i)))
                .collect(),
        ),
        ("markup nested", "[bold]".repeat(deep)),
        // Tags closed under newer ones, and many distinct colors open: at
        // 40k a parser quadratic in the open tags takes far past `SLOW`
        // (RichText's did), while the linear parsers given the same input
        // (Markdown, the syntax highlighter) stay well inside it in a debug
        // build on CI
        (
            "markup interleaved",
            format!("{}{}", "[b][i]x".repeat(40_000), "[/b]y".repeat(40_000)),
        ),
        (
            "markup many colors",
            (0..40_000)
                .map(|i| format!("[#{i:06x}]x"))
                .collect::<String>(),
        ),
        // Many colors closed oldest first, each under all the newer ones
        (
            "markup colors closed in order",
            (0..40_000)
                .map(|i| format!("[#{i:06x}]x"))
                .chain((0..40_000).map(|i| format!("[/#{i:06x}]y")))
                .collect::<String>(),
        ),
        (
            "var chain",
            (0..deep)
                .map(|i| format!("--v{i}: var(--v{});", i + 1))
                .collect::<String>(),
        ),
        (
            "var recursion",
            ":root { --a: var(--b); --b: var(--a); } a { color: var(--a); }".into(),
        ),
        ("css unterminated", "a { color: red".into()),
        ("comment unterminated", "/* a { color: red; }".into()),
        (
            "keyframes unterminated",
            "@keyframes k { from { opacity: 0 }".into(),
        ),
        ("string unterminated", "\"abc".into()),
        ("fence unterminated", "```rust\nfn main() {".into()),
        ("markup unterminated", "[bold red".into()),
        ("link unterminated", "[link=http://x".into()),
        ("escape unterminated", "\x1b[".into()),
        ("osc unterminated", "\x1b]8;;http://x".into()),
        ("csi params huge", format!("\x1b[{}m", "1;".repeat(deep))),
        ("csi number huge", "\x1b[99999999999999999999H".into()),
        ("rgb unterminated", "rgb(1, 2".into()),
        ("var unterminated", "var(--a".into()),
        ("nul", "\0".into()),
        ("nuls", "a\0b\0\0c".into()),
        ("controls", (0u8..0x20).map(char::from).collect()),
        ("crlf", "a\r\nb\r\n\r\n".into()),
        ("cr only", "\r\r\ra\rb".into()),
        ("combining only", "\u{301}\u{302}\u{303}".repeat(100)),
        ("zwj emoji", "👨‍👩‍👧‍👦".repeat(50)),
        (
            "rtl ltr",
            "abc \u{202e}עברית\u{202c} def العربية 123 \u{2067}x\u{2069}".into(),
        ),
        ("max char", "\u{10FFFF}\u{FFFD}\u{FEFF}".into()),
        ("wide", "漢字".repeat(5_000)),
        ("tabs", "\t".repeat(10_000)),
    ];
    for (i, seed) in [1u64, 0xdead_beef, 0x1234_5678_9abc, u64::MAX / 3]
        .iter()
        .enumerate()
    {
        let name: &'static str = [
            "lossy bytes 1",
            "lossy bytes 2",
            "lossy bytes 3",
            "lossy bytes 4",
        ][i];
        c.push((name, lossy(*seed, 512)));
    }
    c
}

// ─── Targets by name ─────────────────────────────────────────────────────────

/// A parser entry point: its name in case keys and fuzz target names, the
/// function, and the tokens its soups are made of
pub type Target = (&'static str, fn(&str), &'static [&'static str]);

/// Every target this build has (some need a feature)
pub fn targets() -> Vec<Target> {
    let t = |name, run, tokens| -> Target { (name, run, tokens) };
    #[allow(unused_mut)]
    let mut targets = vec![
        t("css", css as fn(&str), CSS_TOKENS),
        t("selector", selector, CSS_TOKENS),
        t("declaration", declaration, CSS_TOKENS),
        t("color", color, COLOR_TOKENS),
        t("transition", transition, TRANSITION_TOKENS),
        t("richtext", rich_text, MARKUP_TOKENS),
        t("keymap", keymap, KEY_TOKENS),
        t("mermaid", mermaid, MERMAID_TOKENS),
        t("logviewer", log_viewer, LOG_TOKENS),
        t("query", query, QUERY_TOKENS),
        t("terminal", terminal_ansi, ANSI_TOKENS),
        t("ansi", ansi, ANSI_TOKENS),
        t("json", json, JSON_TOKENS),
        t("timetravel", time_travel, JSON_TOKENS),
        t("csv", csv, CSV_TOKENS),
        t("syntax", syntax, SYNTAX_TOKENS),
    ];
    #[cfg(feature = "markdown")]
    targets.push(t("markdown", markdown, MARKDOWN_TOKENS));
    #[cfg(feature = "syntax-highlighting")]
    targets.push(t("tree-sitter", tree_sitter, SYNTAX_TOKENS));
    targets
}
