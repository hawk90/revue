//! CSS Features Showcase
//!
//! Demonstrates all CSS features supported by revue:
//! - Named colors (50+), HSL, hex, rgb
//! - CSS variables with fallback
//! - text-align, font-weight, text-decoration
//! - overflow, flex-wrap, align-self, order
//! - Border shorthand, nth-child selectors
//!
//! Run: `cargo run --example css_features`

use revue::prelude::*;

const CSS: &str = r#"
:root {
    --primary: hsl(220, 90%, 56%);
    --success: #a6e3a1;
    --warning: orange;
    --error: crimson;
    --bg: #1e1e2e;
    --surface: #313244;
}

.title {
    text-align: center;
    font-weight: bold;
    color: var(--primary);
}

.card {
    border: rounded;
    border-color: var(--primary);
}

.success { color: var(--success); }
.warning { color: var(--warning); }
.error { color: var(--error); }

.muted {
    color: slategray;
    opacity: 0.8;
}

.underline { text-decoration: underline; }
.strikethrough { text-decoration: line-through; }
.bold { font-weight: bold; }

.right-align { text-align: right; }
.center { text-align: center; }
"#;

struct CssShowcase;

impl View for CssShowcase {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .gap(1)
            // Title
            .child(Text::new("CSS Features Showcase").class("title"))
            .child(Divider::new())
            // Colors section
            .child(
                Border::rounded().title("Named Colors & HSL").child(
                    vstack()
                        .child(Text::new("success: green").class("success"))
                        .child(Text::new("warning: orange").class("warning"))
                        .child(Text::new("error: crimson").class("error"))
                        .child(Text::new("muted: slategray").class("muted")),
                ),
            )
            // Text styling
            .child(
                Border::rounded().title("Text Styling").child(
                    vstack()
                        .child(Text::new("Bold text").class("bold"))
                        .child(Text::new("Underlined text").class("underline"))
                        .child(Text::new("Strikethrough text").class("strikethrough"))
                        .child(Text::new("Right-aligned").class("right-align"))
                        .child(Text::new("Centered").class("center")),
                ),
            )
            // Variables with fallback
            .child(
                Border::rounded().title("CSS Variables").child(
                    vstack()
                        .child(Text::new("var(--primary) = blue"))
                        .child(Text::new("var(--undefined, orange) = fallback")),
                ),
            )
            .child(Text::info("Press 'q' to quit").class("center"))
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("CssShowcase")
    }
}

fn main() -> Result<()> {
    App::builder()
        .css(CSS)
        .build()
        .run(CssShowcase, |event, _view, _app| {
            if let Event::Key(KeyEvent {
                key: Key::Char('q'),
                ..
            }) = event
            {
                std::process::exit(0);
            }
            false
        })
}
