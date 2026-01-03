# Building Forms

Learn how to build forms with validation using Revue's reactive system.

## Basic Input

```rust
use revue::prelude::*;

struct Form {
    name: Signal<String>,
}

impl Form {
    fn new() -> Self {
        Self { name: signal(String::new()) }
    }

    fn handle_input(&mut self, c: char) {
        self.name.update(|s| s.push(c));
    }

    fn handle_backspace(&mut self) {
        self.name.update(|s| { s.pop(); });
    }
}

impl View for Form {
    fn render(&self, ctx: &mut RenderContext) {
        let name = self.name.get();

        vstack()
            .child(Text::new("Name:"))
            .child(
                Border::single().child(
                    Text::new(if name.is_empty() { "(empty)" } else { &name })
                )
            )
            .render(ctx);
    }
}
```

## Multiple Fields

```rust
#[derive(Clone, PartialEq)]
enum Field {
    Name,
    Email,
    Age,
}

struct Form {
    name: Signal<String>,
    email: Signal<String>,
    age: Signal<String>,
    focused: Signal<Field>,
}

impl Form {
    fn new() -> Self {
        Self {
            name: signal(String::new()),
            email: signal(String::new()),
            age: signal(String::new()),
            focused: signal(Field::Name),
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

    fn next_field(&mut self) {
        self.focused.update(|f| {
            *f = match f {
                Field::Name => Field::Email,
                Field::Email => Field::Age,
                Field::Age => Field::Name,
            };
        });
    }
}
```

## Reactive Validation

Use `Computed` for real-time validation:

```rust
struct Form {
    name: Signal<String>,
    email: Signal<String>,

    // Computed validations
    name_valid: Computed<bool>,
    email_valid: Computed<bool>,
    form_valid: Computed<bool>,

    // Computed error messages
    name_error: Computed<Option<String>>,
    email_error: Computed<Option<String>>,
}

impl Form {
    fn new() -> Self {
        let name = signal(String::new());
        let email = signal(String::new());

        // Name validation
        let name_clone = name.clone();
        let name_valid = computed(move || {
            let n = name_clone.get();
            !n.is_empty() && n.len() < 50
        });

        let name_clone = name.clone();
        let name_error = computed(move || {
            let n = name_clone.get();
            if n.is_empty() {
                Some("Name is required".into())
            } else if n.len() >= 50 {
                Some("Name too long".into())
            } else {
                None
            }
        });

        // Email validation
        let email_clone = email.clone();
        let email_valid = computed(move || {
            let e = email_clone.get();
            e.contains('@') && e.len() > 3
        });

        let email_clone = email.clone();
        let email_error = computed(move || {
            let e = email_clone.get();
            if e.is_empty() {
                Some("Email is required".into())
            } else if !e.contains('@') {
                Some("Invalid email".into())
            } else {
                None
            }
        });

        // Form-level validation
        let name_valid_clone = name_valid.clone();
        let email_valid_clone = email_valid.clone();
        let form_valid = computed(move || {
            name_valid_clone.get() && email_valid_clone.get()
        });

        Self {
            name, email,
            name_valid, email_valid, form_valid,
            name_error, email_error,
        }
    }
}
```

## Rendering Fields with Validation

```rust
impl View for Form {
    fn render(&self, ctx: &mut RenderContext) {
        let name = self.name.get();
        let name_valid = self.name_valid.get();
        let name_error = self.name_error.get();

        let email = self.email.get();
        let email_valid = self.email_valid.get();
        let email_error = self.email_error.get();

        let form_valid = self.form_valid.get();

        vstack().gap(1)
            // Name field
            .child(
                vstack()
                    .child(
                        hstack().gap(1)
                            .child(Text::new(if name_valid { "✓" } else { "○" })
                                .fg(if name_valid { Color::GREEN } else { Color::GRAY }))
                            .child(Text::new("Name").bold())
                    )
                    .child(Border::single().child(Text::new(&name)))
                    .child(if let Some(err) = name_error {
                        Text::error(err)
                    } else {
                        Text::new("")
                    })
            )
            // Email field
            .child(
                vstack()
                    .child(
                        hstack().gap(1)
                            .child(Text::new(if email_valid { "✓" } else { "○" })
                                .fg(if email_valid { Color::GREEN } else { Color::GRAY }))
                            .child(Text::new("Email").bold())
                    )
                    .child(Border::single().child(Text::new(&email)))
                    .child(if let Some(err) = email_error {
                        Text::error(err)
                    } else {
                        Text::new("")
                    })
            )
            // Submit status
            .child(
                if form_valid {
                    Text::success("✓ Ready to submit")
                } else {
                    Text::error("✗ Please fix errors")
                }
            )
            .render(ctx);
    }
}
```

## Form Submission

```rust
impl Form {
    fn submit(&mut self) {
        if self.form_valid.get() {
            let name = self.name.get();
            let email = self.email.get();

            // Process form data
            println!("Submitted: {} ({})", name, email);

            // Clear form
            self.name.set(String::new());
            self.email.set(String::new());

            announce_success("Form submitted");
        } else {
            announce_error("Please fix validation errors");
        }
    }
}
```

## Event Handling

```rust
fn main() -> Result<()> {
    let mut app = App::builder().build();
    let form = Form::new();

    app.run(form, |event, form, _app| {
        match event {
            Event::Key(key_event) => match key_event.key {
                Key::Char(c) if !c.is_control() => {
                    form.handle_input(c);
                    true
                }
                Key::Backspace => {
                    form.handle_backspace();
                    true
                }
                Key::Tab => {
                    form.next_field();
                    true
                }
                Key::Enter => {
                    form.submit();
                    true
                }
                Key::Char('q') | Key::Esc => false,
                _ => true,
            },
            _ => true,
        }
    })
}
```

## Best Practices

### 1. Validate on blur, not every keystroke

For complex validation (API calls), consider validating only when focus leaves the field.

### 2. Show errors contextually

Only show errors after the user has interacted with the field:

```rust
let show_error = !value.is_empty() || has_been_focused;
```

### 3. Provide clear feedback

```rust
// Status indicators
let icon = if valid { "✓" } else if touched { "✗" } else { "○" };

// Color coding
let color = if valid { Color::GREEN } else if touched { Color::RED } else { Color::GRAY };
```

### 4. Announce for accessibility

```rust
if form_valid {
    announce_success("Form is ready to submit");
}
```

## Next Steps

- [Reactive Form Example](../../examples/reactive_form.rs) - Complete form with validation
- [Reactive Tutorial](./04-reactive.md) - Deep dive into signals
- [Accessibility Guide](../guides/accessibility.md) - Make forms accessible
