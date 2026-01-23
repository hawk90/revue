//! Form field types

/// Form field type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldType {
    /// Text input
    #[default]
    Text,
    /// Password input (masked)
    Password,
    /// Email input
    Email,
    /// Number input
    Number,
    /// Integer input
    Integer,
    /// Multi-line text
    TextArea,
}
