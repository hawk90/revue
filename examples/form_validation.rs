//! Form Validation example
//!
//! Demonstrates a complete signup form with various validation rules:
//! - Required fields
//! - Email format validation
//! - Password strength requirements
//! - Password confirmation (matches)
//! - Username format (alphanumeric)
//! - Age range validation
//! - Custom validation (terms acceptance)
//!
//! Run with: cargo run --example form_validation

use revue::patterns::form::{FormState, ValidationError, Validators};
use revue::prelude::*;

struct SignupForm {
    form: FormState,
    submitted_data: Signal<Option<UserData>>,
}

#[derive(Clone, Debug)]
struct UserData {
    username: String,
    email: String,
    age: String,
}

impl SignupForm {
    fn new() -> Self {
        let form = FormState::new()
            // Username: required, 3-20 chars, alphanumeric only
            .field("username", |f| {
                f.label("Username")
                    .placeholder("3-20 alphanumeric characters")
                    .required()
                    .min_length(3)
                    .max_length(20)
                    .validator(Validators::alphanumeric())
            })
            // Email: required, valid email format
            .field("email", |f| {
                f.email()
                    .label("Email Address")
                    .placeholder("you@example.com")
                    .required()
            })
            // Age: optional, must be 13-120 if provided
            .field("age", |f| {
                f.number()
                    .label("Age")
                    .placeholder("Must be 13 or older")
                    .min(13.0)
                    .max(120.0)
            })
            // Password: required, min 8 chars
            .field("password", |f| {
                f.password()
                    .label("Password")
                    .placeholder("Minimum 8 characters")
                    .required()
                    .min_length(8)
            })
            // Confirm password: must match password
            .field("confirm_password", |f| {
                f.password()
                    .label("Confirm Password")
                    .placeholder("Re-enter your password")
                    .required()
                    .matches("password")
            })
            // Terms: custom validator requiring "yes"
            .field("terms", |f| {
                f.label("Accept Terms")
                    .placeholder("Type 'yes' to accept")
                    .validator(Box::new(|v: &str| {
                        if v.to_lowercase() == "yes" {
                            Ok(())
                        } else {
                            Err(ValidationError::new("You must type 'yes' to accept terms"))
                        }
                    }))
            })
            .build();

        form.focus("username");

        Self {
            form,
            submitted_data: signal(None),
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
            let data = UserData {
                username: values.get("username").cloned().unwrap_or_default(),
                email: values.get("email").cloned().unwrap_or_default(),
                age: values.get("age").cloned().unwrap_or_default(),
            };
            self.submitted_data.set(Some(data));
            self.form.reset();
            self.form.focus("username");
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
            Key::Escape => {
                self.form.reset();
                self.submitted_data.set(None);
                self.form.focus("username");
                true
            }
            _ => false,
        }
    }
}

impl View for SignupForm {
    fn render(&self, ctx: &mut RenderContext) {
        let form_valid = self.form.is_valid();
        let focused_name = self.form.focused();
        let submitted = self.submitted_data.get();

        let mut main_view = vstack().gap(1);

        // Title
        main_view = main_view.child(
            Border::double().fg(Color::CYAN).child(
                Text::new(" Signup Form - Validation Demo ")
                    .bold()
                    .fg(Color::CYAN),
            ),
        );

        // Success message if submitted
        if let Some(data) = submitted {
            main_view = main_view.child(
                Border::success_box().title("Success!").child(
                    vstack()
                        .child(Text::success("Account created successfully!"))
                        .child(Text::new(format!("Username: {}", data.username)))
                        .child(Text::new(format!("Email: {}", data.email)))
                        .child(if !data.age.is_empty() {
                            Text::new(format!("Age: {}", data.age))
                        } else {
                            Text::new("Age: Not provided")
                        }),
                ),
            );
        }

        // Form fields
        for name in self.form.field_names() {
            if let Some(field) = self.form.get(name) {
                let is_focused = focused_name.as_deref() == Some(name);
                let value = field.value();
                let errors = field.errors();
                let is_valid = field.is_valid();
                let is_touched = field.is_touched();

                // Field container style
                let border = if is_focused {
                    Border::double().fg(Color::CYAN)
                } else if is_touched && !is_valid {
                    Border::single().fg(Color::RED)
                } else if is_touched && is_valid && !value.is_empty() {
                    Border::single().fg(Color::GREEN)
                } else {
                    Border::single()
                };

                // Status indicator
                let (status_icon, status_color) = if value.is_empty() && !is_touched {
                    ("○", Color::rgb(100, 100, 100))
                } else if is_valid {
                    ("✓", Color::GREEN)
                } else {
                    ("✗", Color::RED)
                };

                // Mask password fields
                let display_value = if field.field_type
                    == revue::patterns::form::FieldType::Password
                    && !value.is_empty()
                {
                    "•".repeat(value.len())
                } else if value.is_empty() {
                    field.placeholder.clone()
                } else {
                    value.clone()
                };

                let value_color = if is_focused {
                    Color::YELLOW
                } else if value.is_empty() {
                    Color::rgb(100, 100, 100)
                } else {
                    Color::WHITE
                };

                let mut field_view = vstack()
                    .child(
                        hstack()
                            .gap(1)
                            .child(Text::new(status_icon).fg(status_color))
                            .child(Text::new(&field.label).bold())
                            .child(if is_focused {
                                Text::new("(editing)").fg(Color::CYAN)
                            } else {
                                Text::new("")
                            }),
                    )
                    .child(Text::new(&display_value).fg(value_color));

                // Show errors if touched and has errors
                if is_touched && !errors.is_empty() {
                    for error in &errors {
                        field_view = field_view
                            .child(Text::new(format!("  ↳ {}", &error.message)).fg(Color::RED));
                    }
                }

                main_view = main_view.child(border.child(field_view));
            }
        }

        // Form status bar
        let status_text = if form_valid {
            Text::success("✓ All fields valid - Press Enter to submit")
        } else {
            let error_count = self.form.errors().len();
            Text::error(format!(
                "✗ {} validation error{} - Fix before submitting",
                error_count,
                if error_count == 1 { "" } else { "s" }
            ))
        };
        main_view = main_view.child(status_text);

        // Help section
        main_view = main_view.child(
            Border::rounded().title("Controls").child(
                hstack()
                    .gap(3)
                    .child(Text::muted("[Tab] Next"))
                    .child(Text::muted("[Shift+Tab] Prev"))
                    .child(Text::muted("[Enter] Submit"))
                    .child(Text::muted("[Esc] Reset"))
                    .child(Text::muted("[q] Quit")),
            ),
        );

        main_view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("SignupForm")
    }
}

fn main() -> Result<()> {
    println!("Form Validation Example");
    println!("Demonstrates various validation rules with the FormState API.\n");

    let mut app = App::builder().build();
    let form = SignupForm::new();

    app.run(form, |event, form, _app| match event {
        Event::Key(key_event) => form.handle_key(&key_event.key),
        _ => false,
    })
}
