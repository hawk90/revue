# Building Forms

Learn how to build forms with validation using Revue's reactive `FormState` API.

## Using FormState

Revue provides a high-level `FormState` API that handles reactive validation automatically:

```rust
use revue::prelude::*;
use revue::patterns::form::{FormState, Validators};

let form = FormState::new()
    .field("name", |f| f
        .label("Name")
        .placeholder("Enter your name")
        .required()
        .min_length(2)
        .max_length(50))
    .field("email", |f| f
        .email()
        .label("Email")
        .required())
    .field("age", |f| f
        .number()
        .label("Age")
        .min(1.0)
        .max(150.0))
    .build();

// Reactive validation - errors auto-update when values change
form.set_value("email", "invalid");
assert!(!form.is_valid());

form.set_value("email", "user@example.com");
// Validation automatically recalculates
```

## FormField Builder

Each field is configured using a builder pattern:

```rust
.field("username", |f| f
    .label("Username")           // Display label
    .placeholder("Enter name")   // Placeholder text
    .initial_value("default")    // Initial value
    .required()                  // Required validator
    .min_length(3)               // Minimum length
    .max_length(20)              // Maximum length
    .alphanumeric())             // Letters and numbers only
```

### Field Types

```rust
.field("password", |f| f.password())    // Password field
.field("email", |f| f.email())          // Email field
.field("age", |f| f.number())           // Number field
.field("bio", |f| f.textarea())         // Multi-line text
.field("count", |f| f.integer())        // Integer field
```

### Available Validators

```rust
.required()              // Field cannot be empty
.email()                 // Must be valid email
.min_length(n)           // Minimum character count
.max_length(n)           // Maximum character count
.min(n)                  // Minimum numeric value
.max(n)                  // Maximum numeric value
.numeric()               // Must contain only digits
.integer()               // Must be a valid integer
.alphanumeric()          // Letters and numbers only
.no_whitespace()         // No spaces allowed
.matches("password")     // Must match another field
.custom(|v| ...)         // Custom validator function
```

## Password Confirmation

Use the `matches()` validator for password confirmation:

```rust
let form = FormState::new()
    .field("password", |f| f
        .password()
        .label("Password")
        .required()
        .min_length(8))
    .field("confirm", |f| f
        .password()
        .label("Confirm Password")
        .required()
        .matches("password"))  // Must match password field
    .build();

form.set_value("password", "secret123");
form.set_value("confirm", "secret123");
assert!(form.is_valid());

form.set_value("confirm", "different");
assert!(!form.is_valid());  // Automatically invalid
```

## Accessing Form Data

```rust
// Get a field
if let Some(field) = form.get("email") {
    let value = field.value();           // Current value
    let errors = field.errors();         // Validation errors
    let is_valid = field.is_valid();     // Field validity
    let touched = field.is_touched();    // Has been edited
}

// Set values
form.set_value("email", "new@example.com");

// Get value directly
let email = form.value("email");  // Option<String>

// Get all values as HashMap
let values = form.values();

// Check overall validity
if form.is_valid() {
    // All fields pass validation
}

// Get all errors
let all_errors = form.errors();
```

## Focus Management

```rust
// Set focus
form.focus("email");

// Navigate fields
form.focus_next();  // Move to next field
form.focus_prev();  // Move to previous field

// Clear focus
form.blur();

// Get focused field
let focused = form.focused();  // Option<String>
```

## Form Submission

```rust
impl MyForm {
    fn submit(&mut self) {
        // submit() touches all fields and returns validity
        if self.form.submit() {
            let values = self.form.values();

            // Process form data
            println!("Name: {}", values.get("name").unwrap());
            println!("Email: {}", values.get("email").unwrap());

            // Reset form after successful submission
            self.form.reset();
        } else {
            // Show validation errors
            for (field, errors) in self.form.errors() {
                for error in errors {
                    println!("{}: {}", field, error.message);
                }
            }
        }
    }
}
```

## Rendering with FormState

```rust
impl View for MyForm {
    fn render(&self, ctx: &mut RenderContext) {
        let mut children = vstack().gap(1);

        // Render each field
        for name in self.form.field_names() {
            if let Some(field) = self.form.get(name) {
                let is_focused = self.form.focused().as_deref() == Some(name);
                let value = field.value();
                let errors = field.errors();
                let is_valid = field.is_valid();

                let border = if is_focused {
                    Border::double().fg(Color::CYAN)
                } else {
                    Border::single()
                };

                let status = if value.is_empty() {
                    Text::new("○").fg(Color::GRAY)
                } else if is_valid {
                    Text::new("✓").fg(Color::GREEN)
                } else {
                    Text::new("✗").fg(Color::RED)
                };

                let mut field_view = vstack()
                    .child(hstack().gap(1)
                        .child(status)
                        .child(Text::new(field.label).bold()))
                    .child(Text::new(if value.is_empty() {
                        &field.placeholder
                    } else {
                        &value
                    }));

                // Show errors if touched
                if field.is_touched() {
                    for error in errors {
                        field_view = field_view.child(
                            Text::error(format!("  → {}", error.message))
                        );
                    }
                }

                children = children.child(border.child(field_view));
            }
        }

        // Submit button
        children = children.child(
            if self.form.is_valid() {
                Text::success("[Enter] Submit")
            } else {
                Text::muted("[Enter] Submit (fix errors first)")
            }
        );

        children.render(ctx);
    }
}
```

## Custom Validators

```rust
use revue::patterns::form::ValidationError;

let form = FormState::new()
    .field("username", |f| f
        .required()
        .custom(|value| {
            if value.starts_with("admin") {
                Err(ValidationError::new("Username cannot start with 'admin'"))
            } else {
                Ok(())
            }
        }))
    .build();
```

## Complete Example

```rust
use revue::prelude::*;
use revue::patterns::form::FormState;

struct RegistrationForm {
    form: FormState,
}

impl RegistrationForm {
    fn new() -> Self {
        let form = FormState::new()
            .field("username", |f| f
                .label("Username")
                .required()
                .min_length(3)
                .max_length(20)
                .alphanumeric())
            .field("email", |f| f
                .email()
                .label("Email")
                .required())
            .field("password", |f| f
                .password()
                .label("Password")
                .required()
                .min_length(8))
            .field("confirm", |f| f
                .password()
                .label("Confirm Password")
                .required()
                .matches("password"))
            .build();

        Self { form }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(c) if !c.is_control() => {
                if let Some(name) = self.form.focused() {
                    let mut value = self.form.value(&name).unwrap_or_default();
                    value.push(*c);
                    self.form.set_value(&name, &value);
                }
                true
            }
            Key::Backspace => {
                if let Some(name) = self.form.focused() {
                    let mut value = self.form.value(&name).unwrap_or_default();
                    value.pop();
                    self.form.set_value(&name, &value);
                }
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
                if self.form.submit() {
                    println!("Form submitted: {:?}", self.form.values());
                    self.form.reset();
                }
                true
            }
            _ => false,
        }
    }
}
```

## Best Practices

### 1. Use appropriate field types

```rust
// Good - using semantic field types
.field("email", |f| f.email().required())
.field("password", |f| f.password().min_length(8))

// Avoid - generic validation for specialized fields
.field("email", |f| f.validator(Validators::contains("@", "Must contain @")))
```

### 2. Provide clear labels and placeholders

```rust
.field("phone", |f| f
    .label("Phone Number")
    .placeholder("(555) 123-4567")
    .numeric())
```

### 3. Use matches() for confirmation fields

```rust
.field("password", |f| f.password().required())
.field("confirm", |f| f.password().matches("password"))
```

### 4. Show errors only after interaction

```rust
if field.is_touched() && !field.is_valid() {
    // Show errors
}
```

## Next Steps

- [Reactive Form Example](../../examples/reactive_form.rs) - Complete form with new API
- [Reactive Tutorial](./04-reactive.md) - Deep dive into signals
- [Accessibility Guide](../guides/accessibility.md) - Make forms accessible
