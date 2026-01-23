//! Form validation pattern with reactive state
//!
//! Provides reactive form field and validation state for input forms.
//! Values, errors, and validity are automatically computed using Signal/Computed.
//!
//! # Example
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
//!
//! form.set_value("username", "john");
//! form.set_value("email", "john@example.com");
//!
//! // Validity is automatically computed
//! if form.is_valid() {
//!     println!("Form is valid!");
//! }
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
