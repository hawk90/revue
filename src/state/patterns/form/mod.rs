//! Form validation pattern with reactive state
//!
//! Provides reactive form field and validation state for input forms.
//! Values, errors, and validity are automatically computed using Signal/Computed.
//!
//! # Features
//!
//! | Feature | Description |
//!|---------|-------------|
//! | **Reactive State** | Auto-updating values and errors |
//! | **Field Validation** | Built-in validators for common patterns |
//! | **Custom Validators** | Add your own validation logic |
//! | **Form State** | Track overall validity and errors |
//! | **Type Safety** | Strongly typed field values |
//!
//! # Quick Start
//!
//! ## Create a Form
//!
//! ```
//! use revue::prelude::*;
//!
//! let form = FormState::new()
//!     .field("username", |f| f
//!         .label("Username")
//!         .required()
//!         .min_length(3)
//!         .max_length(20))
//!     .field("email", |f| f.email().required())
//!     .field("age", |f| f.number().min(0.0).max(150.0))
//!     .build();
//! ```
//!
//! ## Set Values, Check Validity, Get Errors
//!
//! ```
//! use revue::patterns::FormState;
//!
//! let form = FormState::new()
//!     .field("username", |f| f.label("Username").required().min_length(3))
//!     .field("email", |f| f.email().required())
//!     .field("age", |f| f.number().min(0.0).max(150.0))
//!     .build();
//!
//! // Setting a value validates it
//! form.set_value("username", "jo");
//! form.set_value("email", "john@example.com");
//! form.set_value("age", "25");
//!
//! // Check an individual field
//! let username = form.get("username").unwrap();
//! assert!(!username.is_valid());
//! assert!(username.first_error().is_some());
//!
//! // Every field's first error, as (field, message)
//! assert_eq!(form.errors().len(), 1);
//!
//! // Check the entire form
//! form.set_value("username", "john");
//! if form.is_valid() {
//!     let data = form.values();
//!     assert_eq!(data["username"], "john");
//!     // Submit form...
//! }
//! ```
//!
//! # Built-in Validators
//!
//! | Builder method | Description | Parameters |
//!|----------------|-------------|------------|
//! | `required()` | Value must be present | - |
//! | `min_length()` | Minimum string length | `usize` |
//! | `max_length()` | Maximum string length | `usize` |
//! | `min()` | Minimum numeric value | `f64` |
//! | `max()` | Maximum numeric value | `f64` |
//! | `email()` | Email field, validates the format | - |
//! | `number()` / `integer()` | Numeric field, validates the number | - |
//! | `matches()` | Must equal another field (password confirmation) | field name |
//! | `validator()` | Any [`ValidatorFn`], e.g. from [`Validators`] | `ValidatorFn` |
//!
//! # Custom Validators
//!
//! ```
//! use revue::patterns::form::{FormState, ValidationError, Validators};
//!
//! let password_validator = Validators::custom(|value| {
//!     if value.len() < 8 {
//!         return Err(ValidationError::new("Password must be at least 8 characters"));
//!     }
//!     Ok(())
//! });
//!
//! let form = FormState::new()
//!     .field("password", |f| f
//!         .label("Password")
//!         .required()
//!         .validator(password_validator))
//!     .build();
//!
//! form.set_value("password", "short");
//! assert!(!form.is_valid());
//! ```

mod field;
mod state;
mod types;
mod validators;

pub use field::{FormField, FormFieldBuilder};
pub use state::{FormState, FormStateBuilder};
pub use types::FieldType;
pub use validators::{ValidationError, ValidatorFn, Validators};

#[cfg(test)]
mod tests;
