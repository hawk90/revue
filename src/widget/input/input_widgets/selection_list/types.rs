//! List items and selection display styles

/// Selection list item
#[derive(Clone, Debug)]
pub struct SelectionItem {
    /// Display text
    pub text: String,
    /// Optional value (for forms)
    pub value: Option<String>,
    /// Whether item is disabled
    pub disabled: bool,
    /// Optional description
    pub description: Option<String>,
    /// Optional icon/prefix
    pub icon: Option<String>,
}

impl SelectionItem {
    /// Create a new item
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            value: None,
            disabled: false,
            description: None,
            icon: None,
        }
    }

    /// Set value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl<S: Into<String>> From<S> for SelectionItem {
    fn from(s: S) -> Self {
        Self::new(s)
    }
}

/// Selection display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionStyle {
    /// Checkboxes \[x\] / \[ \]
    #[default]
    Checkbox,
    /// Bullets ● / ○
    Bullet,
    /// Highlight only
    Highlight,
    /// Brackets \[item\] / item
    Bracket,
}
