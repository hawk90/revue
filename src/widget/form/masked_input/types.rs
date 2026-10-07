//! Mask display styles and validation states

/// Mask display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaskStyle {
    /// Show all characters as mask (default)
    #[default]
    Full,
    /// Show last N characters
    ShowLast(usize),
    /// Show first N characters
    ShowFirst(usize),
    /// Show characters briefly then mask
    Peek,
    /// Show nothing (empty)
    Hidden,
}

/// Input validation result
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationState {
    /// No validation performed
    None,
    /// Input is valid
    Valid,
    /// Input is invalid with message
    Invalid(String),
    /// Validation in progress
    Validating,
}
