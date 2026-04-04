//! QR Code Showcase - Google Drive Link
//!
//! Demonstrates the QrCodeWidget with different rendering styles.
//!
//! Run with: cargo run --example qrcode_showcase --features qrcode

use revue::prelude::*;
use revue::widget::{qrcode, ErrorCorrection, QrStyle};

const DRIVE_URL: &str =
    "https://drive.google.com/drive/folders/1iUx5K9AjN-LVa8X0A8RxV1RiscXHjgnC?usp=drive_link";

struct QrShowcase {
    current_style: Signal<usize>,
}

impl QrShowcase {
    fn new() -> Self {
        Self {
            current_style: signal(0),
        }
    }

    fn styles() -> &'static [(QrStyle, &'static str)] {
        &[
            (QrStyle::HalfBlock, "Half Block"),
            (QrStyle::FullBlock, "Full Block"),
            (QrStyle::Braille, "Braille"),
            (QrStyle::Ascii, "ASCII"),
        ]
    }

    fn next_style(&self) {
        let len = Self::styles().len();
        self.current_style.update(|v| *v = (*v + 1) % len);
    }

    fn prev_style(&self) {
        let len = Self::styles().len();
        self.current_style.update(|v| *v = (*v + len - 1) % len);
    }

    fn handle_key(&self, key: &Key) -> bool {
        match key {
            Key::Right | Key::Char('l') | Key::Tab => {
                self.next_style();
                true
            }
            Key::Left | Key::Char('h') => {
                self.prev_style();
                true
            }
            _ => false,
        }
    }
}

impl View for QrShowcase {
    fn render(&self, ctx: &mut RenderContext) {
        let idx = self.current_style.get();
        let (style, style_name) = Self::styles()[idx];

        let qr = qrcode(DRIVE_URL)
            .style(style)
            .fg(Color::WHITE)
            .bg(Color::BLACK)
            .error_correction(ErrorCorrection::Medium)
            .quiet_zone(2);

        // Header (1 line)
        let title = Text::new(format!("QR Code Showcase [{}]", style_name))
            .bold()
            .fg(Color::CYAN)
            .align(Alignment::Center);

        // URL display (1 line)
        let url_text = Text::new(DRIVE_URL)
            .fg(Color::hex(0x888888))
            .align(Alignment::Center);

        // QR Code (auto = fills remaining space)
        let qr_box = Border::rounded().title("QR").child(qr);

        // Controls (border = 3 lines)
        let controls = Border::single().title("Controls").child(
            hstack()
                .gap(3)
                .child(Text::muted("[<-/->] Style"))
                .child(Text::muted("[q/Esc] Quit")),
        );

        let view = vstack()
            .gap(1)
            .child_sized(title, 1)
            .child_sized(url_text, 1)
            .child(qr_box) // auto: takes remaining space
            .child_sized(controls, 3);

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("QrShowcase")
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let showcase = QrShowcase::new();

    app.run(showcase, |event, showcase, _app| match event {
        Event::Key(key_event) => showcase.handle_key(&key_event.key),
        _ => false,
    })
}
