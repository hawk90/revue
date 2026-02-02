//! Form widgets - Form and input validation components
//!
//! This module provides widgets and utilities for building forms with validation.
//!
//! # Widgets
//!
//! ## Form Widget
//!
//! A container for organizing input fields with a label and displaying validation errors.
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let form = form()
//!     .label("User Information")
//!     .child(input().placeholder("Name"))
//!     .child(input().placeholder("Email"))
//!     .child(button("Submit"));
//! ```
//!
//! ## Form Fields
//!
//! Individual input components with validation support:
//!
//! - [`form_field()`] - Labeled form field with error display
//! - [`password_input()`] - Password field with masking
//! - [`pin_input()`] - PIN/OTP code input
//! - [`masked_input()`] - Custom masked input (e.g., phone, SSN)
//! - [`credit_card_input()`] - Credit card number input with formatting
//!
//! ## Rich Text Editor
//!
//! Multi-line text editor with formatting support:
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let editor = rich_text_editor()
//!     .placeholder("Enter content...")
//!     .toolbar(true)
//!     .min_height(10);
//! ```
//!
//! # Validation
//!
//! Form fields support built-in and custom validators:
//!
//! ## Built-in Validators
//!
//! ```rust,ignore
//! use revue::widget::form_field;
//!
//! // Email validation
//! form_field("Email")
//!     .validator(Validators::email())
//!
//! // Required field
//! form_field("Name")
//!     .validator(Validators::required())
//!
//! // Length validation
//! form_field("Password")
//!     .validator(Validators::min_length(8))
//!
//! // Pattern matching
//! form_field("Phone")
//!     .validator(Validators::regex(r"^\d{3}-\d{3}-\d{4}$"))
//!
//! // Custom validator
//! form_field("Age")
//!     .validator(|value| {
//!         if value.parse::<u32>().ok().filter(|&a| a >= 18).is_some() {
//!             Ok(())
//!         } else {
//!             Err("Must be 18 or older".to_string())
//!         }
//!     })
//! ```
//!
//! ## Error Display
//!
//! Form fields can display errors in different styles:
//!
//! ```rust,ignore
//! use revue::widget::form_field;
//! use revue::widget::form::ErrorDisplayStyle;
//!
//! // Show errors inline below the field
//! form_field("Email")
//!     .validator(Validators::email())
//!     .error_style(ErrorDisplayStyle::Inline)
//!
//! // Show errors as tooltips
//! form_field("Password")
//!     .error_style(ErrorDisplayStyle::Tooltip)
//! ```
//!
//! # Masked Input
//!
//! Create inputs with automatic character masking for sensitive data:
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! // Password field (default masking)
//! password_input()
//!     .label("Password")
//!     .mask('*')
//!
//! // PIN input
//! pin_input()
//!     .length(6)
//!     .mask('●')
//!
//! // Custom masked input (e.g., phone number)
//! masked_input("(___) ___-____")
//!     .mask_char('_')
//!     .label("Phone Number")
//!
//! // Credit card with automatic formatting
//! credit_card_input()
//!     .label("Card Number")
//! ```
//!
//! # Input Types
//!
//! Form fields support various input types:
//!
//! ```rust,ignore
//! use revue::widget::form_field;
//! use revue::widget::form::InputType;
//!
//! form_field("Email")
//!     .input_type(InputType::Email)
//!
//! form_field("Password")
//!     .input_type(InputType::Password)
//!
//! form_field("Number")
//!     .input_type(InputType::Number)
//!
//! form_field("Phone")
//!     .input_type(InputType::Tel)
//! ```
//!
//! # Form State Management
//!
//! Combine form fields with the [`FormState`](crate::patterns::FormState) pattern
//! for complete form management:
//!
//! ```rust,ignore
//! use revue::patterns::{FormState, Validators};
//!
//! let mut form = FormState::new();
//!
//! form.add_field("name")
//!     .validator(Validators::required())
//!     .validator(Validators::min_length(2));
//!
//! form.add_field("email")
//!     .validator(Validators::email());
//!
//! // Validate all fields
//! if form.validate() {
//!     let data = form.get_values();
//!     // Submit form...
//! }
//! ```

#[allow(clippy::module_inception)]
pub mod form;
pub mod masked_input;
pub mod rich_text_editor;

// Re-exports for convenience
pub use form::{form, form_field, ErrorDisplayStyle, Form, FormField, FormFieldWidget, InputType};
pub use masked_input::{
    credit_card_input, masked_input, password_input, pin_input, MaskStyle, MaskedInput,
    ValidationState,
};
pub use rich_text_editor::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, ImageRef,
    Link as MarkdownLink, RichTextEditor, TextFormat, ToolbarAction,
};
