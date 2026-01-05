//! Reactive Form example using FormState API
//!
//! Demonstrates:
//! - High-level FormState API with builder pattern
//! - Automatic reactive validation
//! - Password confirmation with matches()
//! - Focus management
//! - Form submission
//!
//! Run with: cargo run --example reactive_form

use revue::patterns::form::FormState;
use revue::prelude::*;

struct ReactiveForm {
    form: FormState,
    message: Signal<String>,
}

impl ReactiveForm {
    fn new() -> Self {
        let form = FormState::new()
            .field("name", |f| {
                f.label("Name")
                    .placeholder("Enter your name")
                    .required()
                    .min_length(2)
                    .max_length(50)
            })
            .field("email", |f| {
                f.email()
                    .label("Email")
                    .placeholder("user@example.com")
                    .required()
            })
            .field("password", |f| {
                f.password()
                    .label("Password")
                    .placeholder("Min 8 characters")
                    .required()
                    .min_length(8)
            })
            .field("confirm", |f| {
                f.password()
                    .label("Confirm Password")
                    .placeholder("Re-enter password")
                    .required()
                    .matches("password")
            })
            .build();

        // Focus the first field
        form.focus("name");

        Self {
            form,
            message: signal(String::new()),
        }
    }

    fn handle_input(&mut self, c: char) {
        if let Some(name) = self.form.focused() {
            let mut value = self.form.value(&name).unwrap_or_default();
            value.push(c);
            self.form.set_value(&name, &value);
        }
    }

    fn handle_backspace(&mut self) {
        if let Some(name) = self.form.focused() {
            let mut value = self.form.value(&name).unwrap_or_default();
            value.pop();
            self.form.set_value(&name, &value);
        }
    }

    fn submit(&mut self) {
        if self.form.submit() {
            let values = self.form.values();
            self.message.set(format!(
                "Submitted: {} ({})",
                values.get("name").unwrap_or(&String::new()),
                values.get("email").unwrap_or(&String::new())
            ));
            self.form.reset();
            self.form.focus("name");
        } else {
            self.message.set("Please fix validation errors".to_string());
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(c) if !c.is_control() => {
                self.handle_input(*c);
                true
            }
            Key::Backspace => {
                self.handle_backspace();
                true
            }
            Key::Tab => {
                self.form.focus_next();
                true
            }
            Key::BackTab => {
                self.form.focus_prev();
                true
            }
            Key::Enter => {
                self.submit();
                true
            }
            _ => false,
        }
    }
}

impl View for ReactiveForm {
    fn render(&self, ctx: &mut RenderContext) {
        let message = self.message.get();
        let form_valid = self.form.is_valid();
        let focused_name = self.form.focused();

        let mut main_view = vstack().gap(1);

        // Header
        main_view =
            main_view.child(
                Border::panel()
                    .title("Reactive Form (FormState API)")
                    .child(
                        vstack()
                            .child(hstack().gap(2).child(Text::new("Status:")).child(
                                if form_valid {
                                    Text::success("Valid - Ready to submit!")
                                } else {
                                    Text::error("Please complete all fields")
                                },
                            ))
                            .child(if !message.is_empty() {
                                Text::new(message).fg(Color::CYAN).bold()
                            } else {
                                Text::new("")
                            }),
                    ),
            );

        // Render each field
        for name in self.form.field_names() {
            if let Some(field) = self.form.get(name) {
                let is_focused = focused_name.as_deref() == Some(name);
                let value = field.value();
                let errors = field.errors();
                let is_valid = field.is_valid();
                let is_touched = field.is_touched();

                let border = if is_focused {
                    Border::double().fg(Color::CYAN)
                } else {
                    Border::single()
                };

                let status_icon = if value.is_empty() {
                    "○"
                } else if is_valid {
                    "✓"
                } else {
                    "✗"
                };

                let status_color = if value.is_empty() {
                    Color::rgb(100, 100, 100)
                } else if is_valid {
                    Color::GREEN
                } else {
                    Color::RED
                };

                // Mask password fields
                let display_value = if field.field_type
                    == revue::patterns::form::FieldType::Password
                    && !value.is_empty()
                {
                    "*".repeat(value.len())
                } else if value.is_empty() {
                    format!("({})", field.placeholder)
                } else {
                    value.clone()
                };

                let mut field_view = vstack()
                    .child(
                        hstack()
                            .gap(1)
                            .child(Text::new(status_icon).fg(status_color))
                            .child(Text::new(&field.label).bold()),
                    )
                    .child(Text::new(&display_value).fg(if is_focused {
                        Color::YELLOW
                    } else if value.is_empty() {
                        Color::rgb(100, 100, 100)
                    } else {
                        Color::WHITE
                    }));

                // Show errors only if touched or focused
                if (is_touched || is_focused) && !value.is_empty() {
                    for error in errors {
                        field_view =
                            field_view.child(Text::error(format!("  → {}", &error.message)));
                    }
                }

                main_view = main_view.child(border.child(field_view));
            }
        }

        // Controls
        main_view = main_view.child(
            Border::rounded().title("Controls").child(
                vstack()
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::muted("[Type]"))
                            .child(Text::new("Enter text")),
                    )
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::muted("[Tab]"))
                            .child(Text::new("Next field")),
                    )
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::muted("[Shift+Tab]"))
                            .child(Text::new("Previous field")),
                    )
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::muted("[Enter]"))
                            .child(Text::new("Submit form")),
                    )
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::muted("[q]"))
                            .child(Text::new("Quit")),
                    ),
            ),
        );

        // Feature highlights
        main_view = main_view.child(
            Border::success_box().title("FormState Features").child(
                vstack()
                    .child(Text::success(
                        "Builder pattern: FormState::new().field(...).build()",
                    ))
                    .child(Text::success(
                        "Auto validation: errors update on value change",
                    ))
                    .child(Text::success(
                        "matches() validator for password confirmation",
                    ))
                    .child(Text::info("→ Minimal boilerplate, maximum reactivity!")),
            ),
        );

        main_view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("ReactiveForm")
    }
}

fn main() -> Result<()> {
    println!("Reactive Form Example (FormState API)");
    println!("Demonstrates the high-level form API with automatic validation.\n");

    let mut app = App::builder().build();
    let form = ReactiveForm::new();

    app.run(form, |event, form, _app| match event {
        Event::Key(key_event) => form.handle_key(&key_event.key),
        _ => false,
    })
}
