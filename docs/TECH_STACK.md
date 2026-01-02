# Tech Stack

## Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Revue Framework                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                     Widget Layer                         │   │
│   │  pulldown-cmark │ syntect │ image │ base64              │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                     Style Layer                          │   │
│   │            cssparser │ selectors │ (custom)              │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Layout Layer                          │   │
│   │                        taffy                             │   │
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
| **crossterm** | 0.28 | Terminal I/O, events | Render, Event |
| **tokio** | 1.x | Async runtime | Runtime |

### CSS & Layout

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **cssparser** | 0.34 | CSS parsing | Mozilla/Firefox |
| **selectors** | 0.25 | CSS selector matching | Mozilla/Firefox |
| **taffy** | 0.5 | Flexbox layout | Dioxus/Bevy |

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
| **pulldown-cmark** | 0.12 | Markdown parsing | CommonMark |
| **syntect** | 5.2 | Syntax highlighting | Sublime syntax |

### Image

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **image** | 0.25 | Image decoding | PNG, JPEG, etc |
| **base64** | 0.22 | Base64 encoding | Kitty protocol |

### Utilities

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| **serde** | 1.0 | Serialization | Config, themes |
| **toml** | 0.8 | TOML parsing | Config files |
| **arboard** | 3.4 | Clipboard | Copy/paste |
| **fuzzy-matcher** | 0.3 | Fuzzy search | Command palette |
| **notify** | 6.1 | File watching | Hot reload |
| **tracing** | 0.1 | Logging | Debug |
| **tracing-subscriber** | 0.3 | Log output | Debug |

### Error Handling

| Crate | Version | Purpose |
|-------|---------|---------|
| **thiserror** | 1.0 | Error derive |
| **anyhow** | 1.0 | Error context |

### Dev Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| **insta** | 1.34 | Snapshot testing |
| **tokio-test** | 0.4 | Async testing |
| **pretty_assertions** | 1.4 | Better test diffs |

---

## Crate Details

### cssparser (Mozilla)

CSS tokenizer and parser used by Firefox's Servo engine.

```rust
use cssparser::{Parser, ParserInput};

let mut input = ParserInput::new(css_text);
let mut parser = Parser::new(&mut input);

// Parse declarations
while let Ok(decl) = parser.parse_declaration() {
    // Process property: value
}
```

**Why this crate?**
- Battle-tested in Firefox
- Full CSS3 support
- Fast and correct
- Active maintenance

### selectors (Mozilla)

CSS selector matching, also from Servo.

```rust
use selectors::parser::Selector;
use selectors::matching::matches_selector;

let selector = Selector::parse(".btn:focus")?;
let matches = matches_selector(&selector, &element);
```

**Supported selectors:**
- Element: `text`, `box`
- Class: `.container`
- ID: `#header`
- Pseudo-class: `:focus`, `:disabled`, `:first-child`
- Combinators: ` ` (descendant), `>` (child)

### taffy (Flexbox)

Layout engine used by Dioxus and Bevy.

```rust
use taffy::prelude::*;

let mut taffy = Taffy::new();

let child = taffy.new_leaf(Style {
    size: Size { width: Dimension::Points(100.0), height: auto() },
    ..Default::default()
})?;

let root = taffy.new_with_children(
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    },
    &[child],
)?;

taffy.compute_layout(root, available_space)?;
```

**Features:**
- Full Flexbox support
- Grid support (optional)
- Percentage sizing
- Min/max constraints

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

### syntect

Syntax highlighting using Sublime Text syntax definitions.

```rust
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};

let ps = SyntaxSet::load_defaults_newlines();
let ts = ThemeSet::load_defaults();

let syntax = ps.find_syntax_by_extension("rs").unwrap();
let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

for line in code.lines() {
    let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps)?;
    // Render styled ranges
}
```

**Languages:** 100+ supported via Sublime syntax files

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
default = ["devtools"]
devtools = []           # Enable devtools (adds ~100KB)
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
| CSS Parser | Custom | cssparser (Mozilla) |
| Layout | Custom | taffy (Dioxus/Bevy) |
| Markdown | Custom | pulldown-cmark |
| Syntax | Pygments | syntect (Sublime) |
| Runtime | asyncio | tokio |

### vs Ratatui (Rust)

| Aspect | Ratatui | Revue |
|--------|---------|-------|
| Styling | Code only | CSS files |
| Layout | Manual Rect | taffy Flexbox |
| Reactivity | None | Signal/Computed |
| Level | Low | High |

### vs Cursive (Rust)

| Aspect | Cursive | Revue |
|--------|---------|-------|
| Styling | Theme module | CSS |
| Backend | ncurses/crossterm | crossterm |
| Layout | BoxView/LinearLayout | Flexbox |
| State | Callbacks | Reactive |

---

## References

- [cssparser docs](https://docs.rs/cssparser)
- [selectors docs](https://docs.rs/selectors)
- [taffy docs](https://docs.rs/taffy)
- [crossterm docs](https://docs.rs/crossterm)
- [pulldown-cmark docs](https://docs.rs/pulldown-cmark)
- [syntect docs](https://docs.rs/syntect)
- [Kitty Graphics Protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
