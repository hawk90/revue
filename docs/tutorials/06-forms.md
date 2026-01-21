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

## Advanced Patterns

### Multi-Step Forms

For complex forms, break them into steps:

```rust
use revue::prelude::*;
use revue::patterns::form::FormState;

struct MultiStepForm {
    step: usize,
    personal: FormState,
    address: FormState,
    confirmation: FormState,
}

impl MultiStepForm {
    fn new() -> Self {
        let personal = FormState::new()
            .field("name", |f| f.label("Name").required().min_length(2))
            .field("email", |f| f.email().label("Email").required())
            .build();

        let address = FormState::new()
            .field("street", |f| f.label("Street").required())
            .field("city", |f| f.label("City").required())
            .field("zip", |f| f.label("ZIP Code").required().integer())
            .build();

        let confirmation = FormState::new()
            .field("confirmed", |f| f.label("I confirm the information is correct"))
            .build();

        Self {
            step: 0,
            personal,
            address,
            confirmation,
        }
    }

    fn next_step(&mut self) -> bool {
        match self.step {
            0 if self.personal.is_valid() => {
                self.step = 1;
                true
            }
            1 if self.address.is_valid() => {
                self.step = 2;
                true
            }
            2 if self.confirmation.value("confirmed").as_deref() == Some("true") => {
                // Submit form
                true
            }
            _ => false,
        }
    }

    fn prev_step(&mut self) {
        if self.step > 0 {
            self.step -= 1;
        }
    }

    fn current_form(&mut self) -> &mut FormState {
        match self.step {
            0 => &mut self.personal,
            1 => &mut self.address,
            _ => &mut self.confirmation,
        }
    }

    fn all_values(&self) -> HashMap<String, String> {
        let mut values = HashMap::new();
        values.extend(self.personal.values());
        values.extend(self.address.values());
        values.extend(self.confirmation.values());
        values
    }
}
```

### Dynamic Fields

Add or remove fields at runtime:

```rust
use revue::prelude::*;
use revue::patterns::form::{FormState, FormFieldBuilder};

struct DynamicForm {
    form: FormState,
    field_count: usize,
}

impl DynamicForm {
    fn new() -> Self {
        let mut form = FormState::new()
            .field("title", |f| f.label("Title").required())
            .build();

        Self {
            form,
            field_count: 0,
        }
    }

    fn add_field(&mut self) {
        self.field_count += 1;
        let name = format!("item_{}", self.field_count);

        // Rebuild form with new field
        let mut builder = FormState::new();
        for (field_name, field) in self.form.iter() {
            builder = builder.field(field_name, |f| {
                let f = f.label(field.label)
                    .placeholder(&field.placeholder)
                    .initial_value(&field.value());
                if field.field_type == FieldType::Integer {
                    f.integer()
                } else {
                    f.text()
                }
            });
        }

        builder = builder.field(&name, |f| f
            .label(format!("Item {}", self.field_count))
            .text());

        self.form = builder.build();
    }

    fn remove_field(&mut self) {
        if self.field_count > 0 {
            let name = format!("item_{}", self.field_count);
            // Note: FormState doesn't support direct field removal
            // You would need to rebuild the form without that field
            self.field_count -= 1;
        }
    }
}
```

### Conditional Validation

Validate fields based on other field values:

```rust
use revue::patterns::form::{FormState, Validators, ValidationError};

let form = FormState::new()
    .field("has_shipping", |f| f.label("Require shipping?"))
    .field("address", |f| f
        .label("Shipping Address")
        .custom(|value| {
            // Only validate if has_shipping is "true"
            if let Ok(shipping) = std::env::var("has_shipping") {
                if shipping == "true" && value.is_empty() {
                    return Err(ValidationError::new("Address required when shipping is enabled"));
                }
            }
            Ok(())
        }))
    .build();

// In your handler, set the environment variable or use shared state
// when has_shipping changes
```

### Form with Hot Reload

Integrate forms with CSS hot reload for instant styling updates:

```rust
use revue::prelude::*;
use revue::patterns::form::FormState;

fn main() -> Result<()> {
    let mut app = App::builder()
        .style("styles.css")
        .hot_reload(true)  // Enable hot reload
        .devtools(true)    // Enable devtools for inspecting form
        .build();

    app.run(MyForm::new(), handler)?;
    Ok(())
}
```

```css
/* styles.css */
.form-field {
    border: rounded gray;
    padding: 1;
    margin: 1 0;
}

.form-field:focus {
    border: double cyan;
}

.form-error {
    color: red;
    margin: 0 0 0 2;
}

.form-valid {
    color: green;
}
```

Now when you edit `styles.css`, the form styling updates instantly without restarting!

### Form Sections

Group related fields together:

```rust
use revue::prelude::*;
use revue::patterns::form::FormState;

struct SectionedForm {
    personal: FormState,
    contact: FormState,
    preferences: FormState,
}

impl SectionedForm {
    fn new() -> Self {
        Self {
            personal: FormState::new()
                .field("first_name", |f| f.label("First Name").required())
                .field("last_name", |f| f.label("Last Name").required())
                .field("dob", |f| f.label("Date of Birth"))
                .build(),
            contact: FormState::new()
                .field("email", |f| f.email().label("Email").required())
                .field("phone", |f| f.label("Phone").numeric())
                .build(),
            preferences: FormState::new()
                .field("newsletter", |f| f.label("Subscribe to newsletter"))
                .field("notifications", |f| f.label("Enable notifications"))
                .build(),
        }
    }

    fn is_valid(&self) -> bool {
        self.personal.is_valid() && self.contact.is_valid()
    }

    fn all_values(&self) -> HashMap<String, String> {
        let mut values = HashMap::new();
        values.extend(self.personal.values());
        values.extend(self.contact.values());
        values.extend(self.preferences.values());
        values
    }
}
```

### Performance Tips for Large Forms

For forms with many fields:

```rust
// 1. Lazy validation - only validate visible fields
impl LargeForm {
    fn validate_current_step(&self) -> bool {
        match self.current_step {
            0 => self.step1.is_valid(),
            1 => self.step2.is_valid(),
            _ => true,
        }
    }
}

// 2. Batch updates to avoid multiple re-renders
impl LargeForm {
    fn update_multiple_fields(&mut self) {
        // Collect all changes
        let updates = vec![
            ("field1", "value1"),
            ("field2", "value2"),
            ("field3", "value3"),
        ];

        // Apply all at once
        for (field, value) in updates {
            self.form.set_value(field, value);
        }
        // Single re-render after all updates
    }
}

// 3. Use iterators for rendering many fields
impl View for LargeForm {
    fn render(&self, ctx: &mut RenderContext) {
        let form_view = vstack()
            .gap(1)
            .extend(self.form.field_names().iter().map(|name| {
                // Render each field
                self.render_field(name)
            }))
            .render(ctx);
    }
}
```

### Form with DevTools Integration

Use DevTools to inspect form state:

```rust
use revue::prelude::*;

impl MyForm {
    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::F12 => {
                // Toggle devtools to inspect form state
                // Requires devtools to be enabled
                true
            }
            Key::Char('d') if KeyModifiers::CONTROL.matches(key.modifiers) => {
                // Press Ctrl+D to open devtools
                true
            }
            _ => self.form_handle_key(key),
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
