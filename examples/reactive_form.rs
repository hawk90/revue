//! Reactive Form example
//!
//! Demonstrates:
//! - Multiple interconnected signals
//! - Computed validation
//! - Derived state from multiple sources
//! - Form submission with reactive state
//!
//! Run with: cargo run --example reactive_form

use revue::prelude::*;

#[derive(Clone, PartialEq)]
enum Field {
    Name,
    Email,
    Age,
}

struct ReactiveForm {
    // Input fields (reactive)
    name: Signal<String>,
    email: Signal<String>,
    age: Signal<String>,

    // UI state
    focused: Signal<Field>,
    submitted: Signal<bool>,
    message: Signal<String>,

    // Computed validations
    name_valid: Computed<bool>,
    email_valid: Computed<bool>,
    age_valid: Computed<bool>,
    form_valid: Computed<bool>,

    // Computed error messages
    name_error: Computed<Option<String>>,
    email_error: Computed<Option<String>>,
    age_error: Computed<Option<String>>,
}

impl ReactiveForm {
    fn new() -> Self {
        let name = signal(String::new());
        let email = signal(String::new());
        let age = signal(String::new());
        let focused = signal(Field::Name);
        let submitted = signal(false);
        let message = signal(String::new());

        // Validation: name (must be non-empty and < 50 chars)
        let name_clone = name.clone();
        let name_valid = computed(move || {
            let n = name_clone.get();
            !n.is_empty() && n.len() < 50
        });

        let name_clone2 = name.clone();
        let name_error = computed(move || {
            let n = name_clone2.get();
            if n.is_empty() {
                Some("Name is required".to_string())
            } else if n.len() >= 50 {
                Some("Name must be less than 50 characters".to_string())
            } else {
                None
            }
        });

        // Validation: email (must contain @)
        let email_clone = email.clone();
        let email_valid = computed(move || {
            let e = email_clone.get();
            e.contains('@') && e.len() > 3
        });

        let email_clone2 = email.clone();
        let email_error = computed(move || {
            let e = email_clone2.get();
            if e.is_empty() {
                Some("Email is required".to_string())
            } else if !e.contains('@') {
                Some("Email must contain @".to_string())
            } else if e.len() <= 3 {
                Some("Email is too short".to_string())
            } else {
                None
            }
        });

        // Validation: age (must be a number between 1-150)
        let age_clone = age.clone();
        let age_valid = computed(move || {
            age_clone.with(|a| a.parse::<u32>().map(|n| n > 0 && n <= 150).unwrap_or(false))
        });

        let age_clone2 = age.clone();
        let age_error = computed(move || {
            let a = age_clone2.get();
            if a.is_empty() {
                Some("Age is required".to_string())
            } else if let Ok(n) = a.parse::<u32>() {
                if n == 0 || n > 150 {
                    Some("Age must be between 1-150".to_string())
                } else {
                    None
                }
            } else {
                Some("Age must be a number".to_string())
            }
        });

        // Overall form validation (all fields must be valid)
        let name_valid_clone = name_valid.clone();
        let email_valid_clone = email_valid.clone();
        let age_valid_clone = age_valid.clone();
        let form_valid = computed(move || {
            name_valid_clone.get() && email_valid_clone.get() && age_valid_clone.get()
        });

        // Effect: log when form becomes valid/invalid
        let form_valid_clone = form_valid.clone();
        effect(move || {
            let valid = form_valid_clone.get();
            println!(
                "Form is now: {}",
                if valid { "VALID ✓" } else { "INVALID ✗" }
            );
        });

        Self {
            name,
            email,
            age,
            focused,
            submitted,
            message,
            name_valid,
            email_valid,
            age_valid,
            form_valid,
            name_error,
            email_error,
            age_error,
        }
    }

    fn handle_input(&mut self, c: char) {
        match self.focused.get() {
            Field::Name => self.name.update(|s| s.push(c)),
            Field::Email => self.email.update(|s| s.push(c)),
            Field::Age => {
                if c.is_ascii_digit() {
                    self.age.update(|s| s.push(c));
                }
            }
        }
    }

    fn handle_backspace(&mut self) {
        match self.focused.get() {
            Field::Name => self.name.update(|s| {
                s.pop();
            }),
            Field::Email => self.email.update(|s| {
                s.pop();
            }),
            Field::Age => self.age.update(|s| {
                s.pop();
            }),
        }
    }

    fn next_field(&mut self) {
        self.focused.update(|f| {
            *f = match f {
                Field::Name => Field::Email,
                Field::Email => Field::Age,
                Field::Age => Field::Name,
            };
        });
    }

    fn prev_field(&mut self) {
        self.focused.update(|f| {
            *f = match f {
                Field::Name => Field::Age,
                Field::Email => Field::Name,
                Field::Age => Field::Email,
            };
        });
    }

    fn submit(&mut self) {
        if self.form_valid.get() {
            let name = self.name.get();
            let email = self.email.get();
            let age = self.age.get();

            self.message.set(format!(
                "✓ Submitted: {} ({}) - {} years old",
                name, email, age
            ));
            self.submitted.set(true);

            // Clear form
            self.name.set(String::new());
            self.email.set(String::new());
            self.age.set(String::new());
        } else {
            self.message
                .set("✗ Please fix validation errors".to_string());
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
                self.next_field();
                true
            }
            Key::BackTab => {
                self.prev_field();
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
        // Get all reactive state (cached!)
        let name = self.name.get();
        let email = self.email.get();
        let age = self.age.get();
        let focused = self.focused.get();
        let message = self.message.get();
        let form_valid = self.form_valid.get();

        let name_valid = self.name_valid.get();
        let email_valid = self.email_valid.get();
        let age_valid = self.age_valid.get();

        let name_error = self.name_error.get();
        let email_error = self.email_error.get();
        let age_error = self.age_error.get();

        // Helper to render field
        let render_field =
            |label: &str, value: &str, is_focused: bool, is_valid: bool, error: Option<String>| {
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

                let mut field_view = vstack()
                    .child(
                        hstack()
                            .gap(1)
                            .child(Text::new(status_icon).fg(status_color))
                            .child(Text::new(label).bold()),
                    )
                    .child(
                        Text::new(if value.is_empty() { "(empty)" } else { value }).fg(
                            if is_focused {
                                Color::YELLOW
                            } else {
                                Color::WHITE
                            },
                        ),
                    );

                if let Some(err) = error {
                    if !value.is_empty() || is_focused {
                        field_view = field_view.child(Text::error(format!("  → {}", err)));
                    }
                }

                border.child(field_view)
            };

        let view =
            vstack()
                .gap(1)
                .child(
                    Border::panel().title("📋 Reactive Form").child(
                        vstack()
                            .child(hstack().gap(2).child(Text::new("Status:")).child(
                                if form_valid {
                                    Text::success("✓ Valid - Ready to submit!")
                                } else {
                                    Text::error("✗ Please complete all fields")
                                },
                            ))
                            .child(if !message.is_empty() {
                                Text::new(message).fg(Color::CYAN).bold()
                            } else {
                                Text::new("")
                            }),
                    ),
                )
                .child(render_field(
                    "Name",
                    &name,
                    focused == Field::Name,
                    name_valid,
                    name_error,
                ))
                .child(render_field(
                    "Email",
                    &email,
                    focused == Field::Email,
                    email_valid,
                    email_error,
                ))
                .child(render_field(
                    "Age",
                    &age,
                    focused == Field::Age,
                    age_valid,
                    age_error,
                ))
                .child(
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
                )
                .child(
                    Border::success_box().title("✨ Reactive Validation").child(
                        vstack()
                            .child(Text::success("✓ Real-time validation with Computed"))
                            .child(Text::success("✓ Error messages auto-update"))
                            .child(Text::success("✓ Form validity is derived from all fields"))
                            .child(Text::info("→ No manual validation logic in render!")),
                    ),
                );

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("ReactiveForm")
    }
}

fn main() -> Result<()> {
    println!("📋 Reactive Form Example");
    println!("Demonstrates interconnected signals and computed validations.\n");

    let mut app = App::builder().build();
    let form = ReactiveForm::new();

    app.run(form, |event, form, _app| match event {
        Event::Key(key_event) => form.handle_key(&key_event.key),
        _ => false,
    })
}
