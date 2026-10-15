//! One option of the multi-select widget

/// An option in the multi-select widget
#[derive(Debug, Clone)]
pub struct MultiSelectOption {
    /// Display label
    pub label: String,
    /// Value (can be same as label)
    pub value: String,
    /// Whether this option is disabled
    pub disabled: bool,
}

impl MultiSelectOption {
    /// Create a new option
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Create an option where label equals value
    pub fn simple(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            disabled: false,
        }
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
