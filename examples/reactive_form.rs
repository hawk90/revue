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
use revue::utils::unicode::display_width;

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

        // Unsized stack children share space equally, so fixed-height pieces
        // use `child_sized` (a Border needs content + 2 rows) and only the
        // field list absorbs the leftover rows. Everything has to fit in 30.
        let mut main_view = vstack();

        // Header: status and the last message share one row
        let status = if form_valid {
            "Valid - Ready to submit!"
        } else {
            "Please complete all fields"
        };
        main_view = main_view.child_sized(
            Border::panel()
                .title("Reactive Form (FormState API)")
                .child(
                    hstack()
                        .gap(2)
                        .child_sized(Text::new("Status:"), 7)
                        .child_sized(
                            if form_valid {
                                Text::success(status)
                            } else {
                                Text::error(status)
                            },
                            display_width(status) as u16,
                        )
                        .child(if !message.is_empty() {
                            Text::new(message).fg(Color::CYAN).bold()
                        } else {
                            Text::new("")
                        }),
                ),
            3,
        );

        let mut fields_view = vstack();
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
                    .child_sized(
                        hstack()
                            .gap(1)
                            .child_sized(Text::new(status_icon).fg(status_color), 1)
                            .child(Text::new(&field.label).bold()),
                        1,
                    )
                    .child_sized(
                        Text::new(&display_value).fg(if is_focused {
                            Color::YELLOW
                        } else if value.is_empty() {
                            Color::rgb(100, 100, 100)
                        } else {
                            Color::WHITE
                        }),
                        1,
                    );
                let mut rows = 2;

                // Show errors only if touched or focused
                if (is_touched || is_focused) && !value.is_empty() {
                    for error in errors {
                        field_view = field_view
                            .child_sized(Text::error(format!("  → {}", error.message)), 1);
                        rows += 1;
                    }
                }

                fields_view = fields_view.child_sized(border.child(field_view), rows + 2);
            }
        }

        main_view = main_view.child(fields_view);

        // Controls and feature highlights side by side
        let mut controls = vstack();
        for (key, action) in [
            ("[Type]", "Enter text"),
            ("[Tab]", "Next field"),
            ("[Shift+Tab]", "Previous field"),
            ("[Enter]", "Submit form"),
            ("[q]", "Quit"),
        ] {
            controls = controls.child_sized(
                hstack()
                    .gap(2)
                    .child_sized(Text::muted(key), 11)
                    .child(Text::new(action)),
                1,
            );
        }

        main_view = main_view.child_sized(
            hstack()
                .gap(1)
                .child_sized(Border::rounded().title("Controls").child(controls), 32)
                .child(
                    Border::success_box().title("FormState Features").child(
                        vstack()
                            .child_sized(
                                Text::success(
                                    "Builder pattern: FormState::new().field(...).build()",
                                ),
                                1,
                            )
                            .child_sized(
                                Text::success("Auto validation: errors update on value change"),
                                1,
                            )
                            .child_sized(
                                Text::success("matches() validator for password confirmation"),
                                1,
                            )
                            .child_sized(
                                Text::info("→ Minimal boilerplate, maximum reactivity!"),
                                1,
                            ),
                    ),
                ),
            7,
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
