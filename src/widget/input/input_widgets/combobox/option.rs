//! ComboOption type for combobox items

/// Option item for combobox
#[derive(Clone, Debug)]
pub struct ComboOption {
    /// Display label
    pub label: String,
    /// Optional value (defaults to label)
    pub value: Option<String>,
    /// Whether this option is disabled
    pub disabled: bool,
    /// Optional group/category
    pub group: Option<String>,
}

impl ComboOption {
    /// Create a new option
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
            disabled: false,
            group: None,
        }
    }

    /// Set the value (different from label)
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Mark as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set group/category
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Get the value (label if no explicit value)
    pub fn get_value(&self) -> &str {
        self.value.as_deref().unwrap_or(&self.label)
    }
}

impl<T: Into<String>> From<T> for ComboOption {
    fn from(s: T) -> Self {
        ComboOption::new(s)
    }
}
