/// Suggestion item with display text and optional value
#[derive(Clone, Debug)]
pub struct Suggestion {
    /// Display text shown in dropdown
    pub label: String,
    /// Value returned when selected (defaults to label)
    pub value: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional icon/prefix
    pub icon: Option<char>,
}

impl Suggestion {
    /// Create a new suggestion
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            description: None,
            icon: None,
        }
    }

    /// Create suggestion with separate value
    pub fn with_value(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            icon: None,
        }
    }

    /// Add description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl<S: Into<String>> From<S> for Suggestion {
    fn from(s: S) -> Self {
        Suggestion::new(s)
    }
}
