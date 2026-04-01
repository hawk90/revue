//! QR Code Generator - Interactive URL to QR
//!
//! Type a URL and instantly see its QR code rendered in the terminal.
//! Switch between rendering styles with Tab.
//!
//! Run with: cargo run --example qrcode_generator --features qrcode

use revue::prelude::*;
use revue::widget::{qrcode, ErrorCorrection, QrStyle};

struct QrGenerator {
    url: Signal<String>,
    style_idx: Signal<usize>,
    cursor: Signal<usize>,
}

impl QrGenerator {
    fn new() -> Self {
        let default_url = String::from("https://example.com");
        let len = default_url.len();
        Self {
            url: signal(default_url),
            style_idx: signal(0),
            cursor: signal(len),
        }
    }

    fn styles() -> &'static [(QrStyle, &'static str)] {
        &[
            (QrStyle::HalfBlock, "Half Block"),
            (QrStyle::Braille, "Braille"),
            (QrStyle::FullBlock, "Full Block"),
            (QrStyle::Ascii, "ASCII"),
        ]
    }

    fn handle_key(&self, key_event: &KeyEvent) -> bool {
        match (&key_event.key, key_event.ctrl) {
            (Key::Char('u'), true) => {
                self.url.set(String::new());
                self.cursor.set(0);
                true
            }
            (Key::Char(c), false) if !c.is_control() => {
                let ch = *c;
                let cur = self.cursor.get();
                self.url.update(|s| s.insert(cur, ch));
                self.cursor.update(|c| *c += 1);
                true
            }
            (Key::Backspace, _) => {
                let cur = self.cursor.get();
                if cur > 0 {
                    self.url.update(|s| {
                        s.remove(cur - 1);
                    });
                    self.cursor.update(|c| *c -= 1);
                }
                true
            }
            (Key::Delete, _) => {
                let cur = self.cursor.get();
                let len = self.url.get().len();
                if cur < len {
                    self.url.update(|s| {
                        s.remove(cur);
                    });
                }
                true
            }
            (Key::Left, _) => {
                self.cursor.update(|c| {
                    if *c > 0 {
                        *c -= 1;
                    }
                });
                true
            }
            (Key::Right, _) => {
                let len = self.url.get().len();
                self.cursor.update(|c| {
                    if *c < len {
                        *c += 1;
                    }
                });
                true
            }
            (Key::Home, _) => {
                self.cursor.set(0);
                true
            }
            (Key::End, _) => {
                let len = self.url.get().len();
                self.cursor.set(len);
                true
            }
            (Key::Tab, _) => {
                let len = Self::styles().len();
                self.style_idx.update(|v| *v = (*v + 1) % len);
                true
            }
            (Key::BackTab, _) => {
                let len = Self::styles().len();
                self.style_idx.update(|v| *v = (*v + len - 1) % len);
                true
            }
            _ => false,
        }
    }
}

impl View for QrGenerator {
    fn render(&self, ctx: &mut RenderContext) {
        let url = self.url.get();
        let cursor = self.cursor.get();
        let idx = self.style_idx.get();
        let (style, style_name) = Self::styles()[idx];

        // Build input display with cursor indicator
        let input_display = if url.is_empty() {
            "Enter a URL...".to_string()
        } else {
            // Show cursor position with a block character
            let pos = cursor.min(url.len());
            let (before, after) = url.split_at(pos);
            format!("{}\u{2502}{}", before, after)
        };

        let input_color = if url.is_empty() {
            Color::hex(0x666666)
        } else {
            Color::WHITE
        };

        // Title (1 line, no border)
        let title = Text::new(format!("QR Code Generator [{}]", style_name))
            .bold()
            .fg(Color::CYAN)
            .align(Alignment::Center);

        // URL Input (border = 3 lines)
        let input = Border::rounded()
            .title("URL")
            .fg(Color::GREEN)
            .child(Text::new(input_display).fg(input_color));

        // QR Code (auto = fills remaining space)
        let qr_box = if !url.is_empty() {
            let qr = qrcode(&url)
                .style(style)
                .fg(Color::WHITE)
                .bg(Color::BLACK)
                .error_correction(ErrorCorrection::Medium)
                .quiet_zone(1);

            Border::rounded().title("QR").child(qr)
        } else {
            Border::rounded().title("QR").child(
                Text::new("Type a URL to generate QR code")
                    .fg(Color::hex(0x666666))
                    .align(Alignment::Center),
            )
        };

        // Controls (border = 3 lines)
        let controls = Border::single().title("Controls").child(
            hstack()
                .gap(3)
                .child(Text::muted("[Tab] Style"))
                .child(Text::muted("[Ctrl+U] Clear"))
                .child(Text::muted("[Esc] Quit")),
        );

        let main = vstack()
            .gap(1)
            .child_sized(title, 1)
            .child_sized(input, 3)
            .child(qr_box) // auto: takes remaining space
            .child_sized(controls, 3);

        main.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("QrGenerator")
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let generator = QrGenerator::new();

    app.run(generator, |event, gen, app| match event {
        Event::Key(key_event) => match key_event.key {
            Key::Escape => {
                app.quit();
                false
            }
            _ => gen.handle_key(key_event),
        },
        Event::Paste(text) => {
            let cur = gen.cursor.get();
            gen.url.update(|s| s.insert_str(cur, text));
            gen.cursor.update(|c| *c += text.len());
            true
        }
        _ => false,
    })
}
