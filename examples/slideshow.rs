//! Markdown Slideshow Example
//!
//! This demonstrates the MarkdownPresentation widget for creating
//! Slidev-style terminal presentations from markdown files.
//!
//! Features:
//! - Parse markdown with `---` slide delimiters
//! - Navigate with arrow keys or vim keys
//! - Toggle between preview and slide mode
//! - Header sizing with Kitty Text Sizing Protocol (OSC 66)
//!
//! Run with: cargo run --example slideshow

use revue::prelude::*;

/// Sample presentation content
const SAMPLE_PRESENTATION: &str = r#"
# Welcome to Revue Slides

Terminal presentations with style!

---

## Slide Features

- **Markdown** support
- Syntax highlighting
- Speaker notes (hidden)

<!-- notes: This is a speaker note. It's not displayed. -->

---

## Code Example

```rust
fn main() {
    println!("Hello, Revue!");
}
```

---

## Navigation

| Key | Action |
|-----|--------|
| → / Space | Next slide |
| ← | Previous slide |
| Home | First slide |
| End | Last slide |
| s | Toggle mode |

---

## Text Sizing

In Kitty terminal, headers render at different sizes:

- H1: 100% (largest)
- H2: 83%
- H3: 75%
- H4: 67%
- H5: 60%
- H6: 33% (smallest)

Other terminals use Figlet ASCII art as fallback.

---

# Thank You!

Press `q` to exit
"#;

/// Slideshow widget wrapper
struct Slideshow {
    presentation: MarkdownPresentation,
}

impl Slideshow {
    fn new(source: &str) -> Self {
        let presentation = MarkdownPresentation::new(source)
            .bg(Color::rgb(20, 20, 40))
            .accent(Color::CYAN)
            .heading_fg(Color::rgb(200, 200, 255))
            .link_fg(Color::CYAN)
            .code_fg(Color::YELLOW);

        Self { presentation }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            // Navigation
            Key::Right | Key::Char(' ') | Key::Char('l') => {
                self.presentation.next_slide();
                true
            }
            Key::Left | Key::Char('h') => {
                self.presentation.prev_slide();
                true
            }
            Key::Home | Key::Char('g') => {
                self.presentation.first();
                true
            }
            Key::End | Key::Char('G') => {
                self.presentation.last();
                true
            }
            // Goto slide (1-9)
            Key::Char(c @ '1'..='9') => {
                let index = (*c as usize) - ('1' as usize);
                self.presentation.goto(index);
                true
            }
            // Mode toggle
            Key::Char('s') => {
                self.presentation.toggle_mode();
                true
            }
            // Scroll (preview mode only)
            Key::Char('j') | Key::Down => {
                if self.presentation.current_mode() == ViewMode::Preview {
                    self.presentation.scroll_down(1);
                } else {
                    self.presentation.next_slide();
                }
                true
            }
            Key::Char('k') | Key::Up => {
                if self.presentation.current_mode() == ViewMode::Preview {
                    self.presentation.scroll_up(1);
                } else {
                    self.presentation.prev_slide();
                }
                true
            }
            Key::PageDown => {
                if self.presentation.current_mode() == ViewMode::Preview {
                    self.presentation.scroll_down(10);
                } else {
                    self.presentation.next_slide();
                }
                true
            }
            Key::PageUp => {
                if self.presentation.current_mode() == ViewMode::Preview {
                    self.presentation.scroll_up(10);
                } else {
                    self.presentation.prev_slide();
                }
                true
            }
            _ => false,
        }
    }
}

impl View for Slideshow {
    fn render(&self, ctx: &mut RenderContext) {
        self.presentation.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("Slideshow")
    }
}

fn main() -> Result<()> {
    // Check for file argument
    let args: Vec<String> = std::env::args().collect();
    let source = if args.len() > 1 {
        std::fs::read_to_string(&args[1]).unwrap_or_else(|_| {
            eprintln!("Failed to read file: {}", args[1]);
            std::process::exit(1);
        })
    } else {
        SAMPLE_PRESENTATION.to_string()
    };

    let mut app = App::builder().build();
    let slideshow = Slideshow::new(&source);

    app.run(slideshow, |event, slideshow, _app| match event {
        Event::Key(key_event) => slideshow.handle_key(&key_event.key),
        _ => false,
    })
}
