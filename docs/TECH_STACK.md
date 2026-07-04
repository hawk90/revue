# Tech Stack

## Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Revue Framework                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                     Widget Layer                         │   │
│   │  pulldown-cmark │ tree-sitter │ image │ base64          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                     Style Layer                          │   │
│   │              custom CSS parser (in-tree)                 │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Layout Layer                          │   │
│   │            custom layout engine (in-tree)               │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                     Text Layer                           │   │
│   │  unicode-width │ unicode-segmentation │ unic-emoji-char │   │
│   │                     textwrap                             │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Render Layer                          │   │
│   │                      crossterm                           │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                      Runtime                             │   │
│   │                       tokio                              │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Dependencies

### Core

| Crate | Version | Purpose | Used By |
|-------|---------|---------|---------|
| **crossterm** | 0.29 | Terminal I/O, events | Render, Event |
| **tokio** | 1.x | Async runtime | Runtime |

### CSS & Layout

CSS parsing, selector matching, and layout are **custom, in-tree
implementations** — Revue has no third-party CSS or layout engine dependency.
See [Custom Implementations](#custom-implementations) below.

### Text & Unicode

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **unicode-width** | 0.2 | Character width | Base width calculation |
| **unicode-segmentation** | 1.10 | Grapheme clusters | Emoji, CJK |
| **unic-emoji-char** | 0.9 | Emoji detection | Emoji width |
| **textwrap** | 0.16 | Text wrapping | Line breaks |

### Markdown & Syntax

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **pulldown-cmark** | 0.13 | Markdown parsing | CommonMark (optional, `markdown` feature) |
| **tree-sitter-highlight** | 0.26 | Syntax highlighting | Incremental parsing; per-language grammars (`tree-sitter-rust`, `-python`, `-javascript`, `-json`, `-go`, `-bash`, `-html`, `-css`, `-toml-ng`, `-yaml`, `-sequel`, `-md`) via `syntax-highlighting` feature |

### Image

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **image** | 0.25 | Image decoding | PNG, JPEG, etc |
| **base64** | 0.22 | Base64 encoding | Kitty protocol |

### Utilities

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **bitflags** | 2.6 | Bit flags | Modifiers, state flags |
| **serde** | 1.0 | Serialization | Config, themes (`config` feature) |
| **toml** | 1.0 | TOML parsing | Config files (`config` feature) |
| **dirs** | 6.0 | Standard directories | Config paths (`config` feature) |
| **arboard** | 3.4 | Clipboard | Copy/paste (`clipboard` feature) |
| **notify** | 8.2 | File watching | Hot reload (`hot-reload` feature) |
| **sysinfo** | 0.39 | System/process info | Process monitor (`sysinfo` feature) |
| **reqwest** | 0.13 | HTTP client | HTTP client widget (`http` feature) |
| **similar** | 3.0 | Text diffing | Diff viewer (`diff` feature) |
| **qrcode** | 0.14 | QR code generation | QR widget (`qrcode` feature) |
| **tracing** | 0.1 | Logging | Debug (`tracing` feature) |
| **tracing-subscriber** | 0.3 | Log output | Debug (`tracing` feature) |

### Error Handling

| Crate | Version | Purpose |
|-------|---------|---------|
| **thiserror** | 2.0 | Error derive |
| **anyhow** | 1.0 | Error context |
| **revue-macros** | (workspace) | Internal proc macros |

### Dev Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| **insta** | 1.34 | Snapshot testing |
| **tokio-test** | 0.4 | Async testing |
| **pretty_assertions** | 1.4 | Better test diffs |

---

## Crate Details

### Custom CSS Parser (in-tree)

Revue ships its own CSS tokenizer, parser, and selector matcher under
`src/runtime/style/parser/` (scanner → parse → apply) with selector matching in
`src/runtime/dom/`. There is **no** dependency on `cssparser` or the `selectors`
crate.

```rust
use revue::style::StyleSheet;

// Parse a CSS string into a StyleSheet
let sheet = StyleSheet::parse(css_text)?;

// Selectors are matched against the widget DOM during the cascade
```

**Why in-tree?**
- Terminal-focused subset of CSS — no need for a full browser engine
- Zero third-party CSS dependencies (smaller tree, full control)
- Tailored diagnostics via `style::ParseError`

**Supported selectors:**
- Element: `text`, `button`
- Class: `.container`
- ID: `#header`
- Pseudo-class: `:focus`, `:hover`, `:disabled`, `:not()`, `:nth-child(An+B)`
- Combinators: ` ` (descendant), `>` (child)

### Custom Layout Engine (in-tree)

Flexbox, grid, and block layout are implemented directly under
`src/runtime/layout/` (`flex.rs`, `grid.rs`, `block.rs`, `node.rs`,
`compute.rs`). There is **no** dependency on `taffy`.

```rust
// Styles resolved from CSS drive the layout node tree, then compute() runs
// the flex/grid/block algorithms to produce a position map.
layout::compute(&node_tree, available_space);
```

**Features:**
- Flexbox layout (`flex.rs`)
- Grid layout (`grid.rs`)
- Block layout (`block.rs`)
- Percentage sizing, min/max constraints, responsive breakpoints

### crossterm

Cross-platform terminal manipulation.

```rust
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    style::{Color, SetForegroundColor, Print},
    event::{read, Event, KeyCode},
};

// Output
execute!(
    stdout,
    Clear(ClearType::All),
    MoveTo(0, 0),
    SetForegroundColor(Color::Cyan),
    Print("Hello"),
)?;

// Input
match read()? {
    Event::Key(event) => {
        match event.code {
            KeyCode::Char('q') => break,
            KeyCode::Enter => submit(),
            _ => {}
        }
    }
    _ => {}
}
```

**Features:**
- Windows, macOS, Linux
- Raw mode
- Alternate screen
- Mouse events
- Bracketed paste

### pulldown-cmark

CommonMark-compliant markdown parser.

```rust
use pulldown_cmark::{Parser, Event, Tag};

let parser = Parser::new(markdown_text);

for event in parser {
    match event {
        Event::Start(Tag::Heading(level, _, _)) => {
            // Start heading
        }
        Event::Text(text) => {
            // Text content
        }
        Event::End(Tag::Heading(..)) => {
            // End heading
        }
        _ => {}
    }
}
```

**Supported elements:**
- Headings, paragraphs
- Bold, italic, strikethrough
- Links, images
- Code blocks (fenced)
- Lists (ordered, unordered)
- Blockquotes
- Tables

### tree-sitter

Accurate syntax highlighting using tree-sitter incremental parsing.

```rust
use revue::widget::{TreeSitterHighlighter, Language, SyntaxTheme};

// Create highlighter for Rust code
let mut highlighter = TreeSitterHighlighter::with_theme(
    Language::Rust,
    SyntaxTheme::dark(),
);

// Highlight a line of code
let spans = highlighter.highlight_line("fn main() { println!(\"hello\"); }");

// Or highlight multiple lines for better accuracy
let code = "fn main() {\n    println!(\"hello\");\n}";
let line_spans = highlighter.highlight_code(code);
```

**Supported Languages:**
- Rust, Python, JavaScript, Go
- JSON, TOML, YAML
- HTML, CSS, SQL
- Bash/Shell, Markdown

### image

Image decoding library.

```rust
use image::io::Reader as ImageReader;

let img = ImageReader::open("image.png")?.decode()?;
let rgba = img.to_rgba8();
let (width, height) = rgba.dimensions();
```

**Formats:** PNG, JPEG, GIF, BMP, TIFF, WebP, etc.

---

## Custom Implementations

### Reactive System

Not using external crate to keep dependencies minimal and have full control.

```rust
// Signal implementation sketch
pub struct Signal<T> {
    id: SignalId,
    value: Rc<RefCell<T>>,
    runtime: Rc<RefCell<Runtime>>,
}

impl<T: Clone + 'static> Signal<T> {
    pub fn get(&self) -> T {
        // Track read dependency
        self.runtime.borrow_mut().track(self.id);
        self.value.borrow().clone()
    }

    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        // Notify runtime of change
        self.runtime.borrow_mut().notify(self.id);
    }
}
```

### Double Buffer

Custom implementation for terminal rendering.

```rust
pub struct Buffer {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
}

pub fn diff(old: &Buffer, new: &Buffer) -> Vec<Change> {
    let mut changes = Vec::new();
    for (i, (old_cell, new_cell)) in old.cells.iter().zip(&new.cells).enumerate() {
        if old_cell != new_cell {
            let x = (i % width) as u16;
            let y = (i / width) as u16;
            changes.push(Change { x, y, cell: new_cell.clone() });
        }
    }
    changes
}
```

### Kitty Image Protocol

Direct implementation without external crate.

```rust
pub fn render_kitty_image(data: &[u8], width: u32, height: u32) -> String {
    let b64 = base64::encode(data);

    // Kitty graphics protocol escape sequence
    // f=100 (PNG), a=T (transmit), t=d (direct)
    format!("\x1b_Gf=100,s={},v={},a=T;{}\x1b\\", width, height, b64)
}
```

### Character Width Table

Custom implementation with terminal detection.

```rust
pub struct CharWidthTable {
    cjk: u8,
    emoji: u8,
    nerd_font: u8,
    overrides: HashMap<char, u8>,
}

impl CharWidthTable {
    pub fn detect() -> Self {
        // Query terminal for actual character widths
        let cjk = detect_width('한');
        let emoji = detect_width('😀');
        let nerd = detect_width('');

        Self { cjk, emoji, nerd_font: nerd, overrides: HashMap::new() }
    }

    pub fn width(&self, ch: char) -> u8 {
        if let Some(&w) = self.overrides.get(&ch) {
            return w;
        }

        if unic_emoji_char::is_emoji(ch) {
            return self.emoji;
        }

        // Use unicode-width as base
        unicode_width::UnicodeWidthChar::width(ch)
            .map(|w| w as u8)
            .unwrap_or(1)
    }
}
```

---

## Build Configuration

### Features

```toml
[features]
default = ["async", "config"]  # tokio runtime + config (serde/toml/dirs)
devtools = []                   # Opt-in devtools (widget inspector, profiler)
```

### Profile

```toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
strip = true            # Strip symbols
panic = "abort"         # Smaller binary

[profile.dev]
opt-level = 1           # Faster dev builds
```

### Expected Binary Size

| Profile | Size |
|---------|------|
| Debug | ~50MB |
| Release | ~5-15MB |
| Release (stripped) | ~3-10MB |

---

## Comparison with Other Frameworks

### vs Textual (Python)

| Aspect | Textual | Revue |
|--------|---------|-------|
| CSS Parser | Custom | Custom (in-tree) |
| Layout | Custom | Custom (in-tree) |
| Markdown | Custom | pulldown-cmark |
| Syntax | Pygments | tree-sitter |
| Runtime | asyncio | tokio |

### vs ratatui (Rust)

| Aspect | ratatui | Revue |
|--------|---------|-------|
| Type | Library | Framework |
| Styling | Code only | CSS files |
| Layout | Constraint (Cassowary) | Custom flex/grid engine |
| Reactivity | Manual | Signal/Computed |
| Level | Low | High |

> **Note**: ratatui is a low-level immediate-mode library (like React's DOM),
> whereas Revue is a retained-mode framework. Among Rust framework peers, the
> credible modern comparisons are r3bl_tui, tui-realm, and iocraft — see
> [FRAMEWORK_COMPARISON.md](FRAMEWORK_COMPARISON.md). (`reratui`, an
> immediate-mode wrapper over ratatui with negligible adoption, is not a
> meaningful peer.)

### vs Cursive (Rust)

| Aspect | Cursive | Revue |
|--------|---------|-------|
| Styling | Theme module | CSS |
| Backend | ncurses/crossterm | crossterm |
| Layout | BoxView/LinearLayout | Flexbox |
| State | Callbacks | Reactive |

---

## References

- [crossterm docs](https://docs.rs/crossterm)
- [pulldown-cmark docs](https://docs.rs/pulldown-cmark)
- [tree-sitter-highlight docs](https://docs.rs/tree-sitter-highlight)
- [tokio docs](https://docs.rs/tokio)
- [Kitty Graphics Protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
