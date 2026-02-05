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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_item_new() {
        let item = BreadcrumbItem::new("Home");
        assert_eq!(item.label, "Home");
        assert!(item.icon.is_none());
        assert!(item.clickable);
    }

    #[test]
    fn test_breadcrumb_item_builder() {
        let item = BreadcrumbItem::new("Folder").icon('📁').clickable(true);
        assert_eq!(item.label, "Folder");
        assert_eq!(item.icon, Some('📁'));
        assert!(item.clickable);
    }

    #[test]
    fn test_breadcrumb_item_not_clickable() {
        let item = BreadcrumbItem::new("Locked").clickable(false);
        assert!(!item.clickable);
    }

    #[test]
    fn test_breadcrumb_item_clone() {
        let item = BreadcrumbItem::new("Test");
        let cloned = item.clone();
        assert_eq!(item.label, cloned.label);
    }

    #[test]
    fn test_separator_style_default() {
        let style = SeparatorStyle::default();
        assert_eq!(style, SeparatorStyle::Slash);
    }

    #[test]
    fn test_separator_style_equality() {
        assert_eq!(SeparatorStyle::Arrow, SeparatorStyle::Arrow);
        assert_eq!(SeparatorStyle::Chevron, SeparatorStyle::Chevron);
        assert_ne!(SeparatorStyle::Slash, SeparatorStyle::Pipe);
    }

    #[test]
    fn test_separator_style_custom() {
        let style = SeparatorStyle::Custom('-');
        assert_eq!(style, SeparatorStyle::Custom('-'));
        assert_ne!(style, SeparatorStyle::Custom('>'));
    }
}
