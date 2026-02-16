//! Breadcrumb navigation widget - public types

/// Breadcrumb item
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    /// Item label
    pub label: String,
    /// Optional icon
    pub icon: Option<char>,
    /// Is item clickable
    pub clickable: bool,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            clickable: true,
        }
    }

    /// Set icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set clickable state
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }
}

/// Separator style
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SeparatorStyle {
    /// Slash /
    #[default]
    Slash,
    /// Arrow >
    Arrow,
    /// Chevron ›
    Chevron,
    /// Double arrow »
    DoubleArrow,
    /// Dot •
    Dot,
    /// Pipe |
    Pipe,
    /// Custom character
    Custom(char),
}

impl SeparatorStyle {
    pub(crate) fn char(&self) -> char {
        match self {
            SeparatorStyle::Slash => '/',
            SeparatorStyle::Arrow => '>',
            SeparatorStyle::Chevron => '›',
            SeparatorStyle::DoubleArrow => '»',
            SeparatorStyle::Dot => '•',
            SeparatorStyle::Pipe => '|',
            SeparatorStyle::Custom(c) => *c,
        }
    }
}
