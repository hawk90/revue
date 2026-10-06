use revue::prelude::*;

/// Form app demonstrating input widgets and validation.
struct FormApp {
    name: Signal<String>,
    email: Signal<String>,
    active_field: Signal<usize>,
    submitted: Signal<bool>,
    message: Signal<String>,
}

impl FormApp {
    fn new() -> Self {
        Self {
            name: signal(String::new()),
            email: signal(String::new()),
            active_field: signal(0),
            submitted: signal(false),
            message: signal(String::new()),
        }
    }

    fn handle_input(&self, ch: char) {
        match self.active_field.get() {
            0 => self.name.update(|v| v.push(ch)),
            1 => self.email.update(|v| v.push(ch)),
            _ => {}
        }
    }

    fn handle_backspace(&self) {
        match self.active_field.get() {
            0 => self.name.update(|v| {
                v.pop();
            }),
            1 => self.email.update(|v| {
                v.pop();
            }),
            _ => {}
        }
    }

    fn next_field(&self) {
        self.active_field.update(|v| *v = (*v + 1) % 2);
    }

    fn submit(&self) {
        let name = self.name.get();
        let email = self.email.get();
        if name.is_empty() {
            self.message.set("Name is required".into());
        } else if !email.contains('@') {
            self.message.set("Invalid email address".into());
        } else {
            self.submitted.set(true);
            self.message.set(format!("Welcome, {}!", name));
        }
    }
}

impl View for FormApp {
    fn render(&self, ctx: &mut RenderContext) {
        let active = self.active_field.get();
        let msg = self.message.get();

        Border::new()
            .title("Registration Form")
            .child(
                vstack()
                    .gap(1)
                    .child(
                        hstack()
                            .gap(1)
                            .child(Text::new(if active == 0 { "> Name:" } else { "  Name:" }))
                            .child(Text::new(format!("[{}]", self.name.get()))),
                    )
                    .child(
                        hstack()
                            .gap(1)
                            .child(Text::new(if active == 1 { "> Email:" } else { "  Email:" }))
                            .child(Text::new(format!("[{}]", self.email.get()))),
                    )
                    .child(Divider::new())
                    .child(if !msg.is_empty() {
                        if self.submitted.get() {
                            Text::new(msg).class("status-ok")
                        } else {
                            Text::new(msg).class("status-error")
                        }
                    } else {
                        Text::muted("Fill in the form and press Enter to submit")
                    })
                    .child(Text::info("[Tab] next field  [Enter] submit  [Esc] quit")),
            )
            .render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("FormApp")
    }
}

fn main() -> Result<()> {
    let form = FormApp::new();

    // The Border is this view's own body, so it is styled through the view's
    // type; the status line is a child with its own node, so its class works.
    App::builder()
        .css(
            r#"
            FormApp { border: rounded cyan; }
            .status-ok { color: #a6e3a1; }
            .status-error { color: #f38ba8; }
        "#,
        )
        .build()
        .run(form, |event, view, app| {
            let Event::Key(KeyEvent { key, .. }) = event else {
                return false;
            };
            match key {
                // Every printable key is text for the fields, so quit on Esc.
                // `app.quit()` stops the loop and `run` restores the terminal.
                Key::Escape => app.quit(),
                Key::Tab => view.next_field(),
                Key::Enter => view.submit(),
                Key::Backspace => view.handle_backspace(),
                Key::Char(ch) => view.handle_input(*ch),
                _ => return false,
            }
            // Returning `true` asks for a redraw.
            true
        })
}
