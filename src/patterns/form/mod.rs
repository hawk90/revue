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
//! ```rust,ignore
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
//! ## Set Values
//!
//! ```rust,ignore
//! form.set_value("username", "john");
//! form.set_value("email", "john@example.com");
//! form.set_value("age", "25");
//! ```
//!
//! ## Check Validity
//!
//! ```rust,ignore
//! // Check individual field
//! if form.is_field_valid("username") {
//!     println!("Username is valid");
//! }
//!
//! // Check entire form
//! if form.is_valid() {
//!     println!("Form is valid!");
//!     let data = form.get_values();
//!     // Submit form...
//! }
//! ```
//!
//! ## Get Errors
//!
//! ```rust,ignore
//! // Get all errors
//! let errors = form.get_errors();
//!
//! // Get field-specific errors
//! if let Some(field_errors) = form.get_field_errors("email") {
//!     for error in field_errors {
//!         println!("{}", error.message);
//!     }
//! }
//! ```
//!
//! # Built-in Validators
//!
//! | Validator | Description | Parameters |
//!|-----------|-------------|------------|
//! | `required()` | Value must be present | - |
//! | `min_length()` | Minimum string length | `usize` |
//! | `max_length()` | Maximum string length | `usize` |
//! | `min()` | Minimum numeric value | `f64` |
//! | `max()` | Maximum numeric value | `f64` |
//! | `pattern()` | Regex pattern match | `&str` |
//! | `email()` | Email format validation | - |
//! | `url()` | URL format validation | - |
//! | `custom()` | Custom validator | `ValidatorFn` |
//!
//! # Custom Validators
//!
//! ```rust,ignore
//! use revue::patterns::form::{ValidatorFn, ValidationError};
//!
//! let password_validator: ValidatorFn = |value| {
//!     let s = value.as_string().unwrap_or_default();
//!     if s.len() < 8 {
//!         return Err(vec![ValidationError {
//!             message: "Password must be at least 8 characters".to_string(),
//!             code: "min_length".to_string(),
//!         }]);
//!     }
//!     Ok(())
//! };
//!
//! let form = FormState::new()
//!     .field("password", |f| f
//!         .label("Password")
//!         .required()
//!         .custom(password_validator))
//!     .build();
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
