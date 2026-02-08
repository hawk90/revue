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

    // =========================================================================
    // SeparatorStyle enum tests
    // =========================================================================

    #[test]
    fn test_separator_style_clone() {
        let style = SeparatorStyle::Arrow;
        let cloned = style.clone();
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_separator_style_copy() {
        let style1 = SeparatorStyle::Chevron;
        let style2 = style1;
        assert_eq!(style1, SeparatorStyle::Chevron);
        assert_eq!(style2, SeparatorStyle::Chevron);
    }

    #[test]
    fn test_separator_style_debug() {
        let style = SeparatorStyle::Dot;
        assert!(format!("{:?}", style).contains("Dot"));
    }

    // =========================================================================
    // SeparatorStyle::char() tests
    // =========================================================================

    #[test]
    fn test_separator_style_char_slash() {
        assert_eq!(SeparatorStyle::Slash.char(), '/');
    }

    #[test]
    fn test_separator_style_char_arrow() {
        assert_eq!(SeparatorStyle::Arrow.char(), '>');
    }

    #[test]
    fn test_separator_style_char_chevron() {
        assert_eq!(SeparatorStyle::Chevron.char(), '›');
    }

    #[test]
    fn test_separator_style_char_double_arrow() {
        assert_eq!(SeparatorStyle::DoubleArrow.char(), '»');
    }

    #[test]
    fn test_separator_style_char_dot() {
        assert_eq!(SeparatorStyle::Dot.char(), '•');
    }

    #[test]
    fn test_separator_style_char_pipe() {
        assert_eq!(SeparatorStyle::Pipe.char(), '|');
    }

    #[test]
    fn test_separator_style_char_custom() {
        assert_eq!(SeparatorStyle::Custom('*').char(), '*');
    }

    // =========================================================================
    // BreadcrumbItem edge cases
    // =========================================================================

    #[test]
    fn test_breadcrumb_item_empty_label() {
        let item = BreadcrumbItem::new("");
        assert_eq!(item.label, "");
        assert!(item.clickable);
    }

    #[test]
    fn test_breadcrumb_item_icon() {
        let item = BreadcrumbItem::new("Home").icon('H');
        assert_eq!(item.icon, Some('H'));
    }

    #[test]
    fn test_breadcrumb_item_clickable_false() {
        let item = BreadcrumbItem::new("Current").clickable(false);
        assert!(!item.clickable);
    }

    #[test]
    fn test_breadcrumb_item_builder_chain() {
        let item = BreadcrumbItem::new("Chain").icon('C').clickable(false);
        assert_eq!(item.label, "Chain");
        assert_eq!(item.icon, Some('C'));
        assert!(!item.clickable);
    }
}
