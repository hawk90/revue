//! Option list entries, option items and separator styles

/// Option list entry type
#[derive(Clone, Debug)]
pub enum OptionEntry {
    /// Regular option
    Option(OptionItem),
    /// Separator line
    Separator,
    /// Group header
    Group(String),
}

/// Single option item
#[derive(Clone, Debug)]
pub struct OptionItem {
    /// Display text
    pub text: String,
    /// Optional secondary text (right-aligned)
    pub hint: Option<String>,
    /// Optional value/id
    pub value: Option<String>,
    /// Whether option is disabled
    pub disabled: bool,
    /// Optional icon/prefix
    pub icon: Option<String>,
    /// Optional description
    pub description: Option<String>,
}

impl OptionItem {
    /// Create a new option
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            hint: None,
            value: None,
            disabled: false,
            icon: None,
            description: None,
        }
    }

    /// Set hint text
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
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

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Separator style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorStyle {
    /// Single line ─
    #[default]
    Line,
    /// Dashed ╌
    Dashed,
    /// Double ═
    Double,
    /// Blank line
    Blank,
}
