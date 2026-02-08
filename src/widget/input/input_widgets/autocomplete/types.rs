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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Suggestion::new tests
    // =========================================================================

    #[test]
    fn test_suggestion_new() {
        let sugg = Suggestion::new("Test");
        assert_eq!(sugg.label, "Test");
        assert_eq!(sugg.value, "Test");
        assert!(sugg.description.is_none());
        assert!(sugg.icon.is_none());
    }

    #[test]
    fn test_suggestion_new_str() {
        let sugg = Suggestion::new("Hello World");
        assert_eq!(sugg.label, "Hello World");
        assert_eq!(sugg.value, "Hello World");
    }

    #[test]
    fn test_suggestion_new_string() {
        let sugg = Suggestion::new(String::from("Owned String"));
        assert_eq!(sugg.label, "Owned String");
        assert_eq!(sugg.value, "Owned String");
    }

    #[test]
    fn test_suggestion_new_empty() {
        let sugg = Suggestion::new("");
        assert_eq!(sugg.label, "");
        assert_eq!(sugg.value, "");
    }

    // =========================================================================
    // Suggestion::with_value tests
    // =========================================================================

    #[test]
    fn test_suggestion_with_value() {
        let sugg = Suggestion::with_value("Display", "actual_value");
        assert_eq!(sugg.label, "Display");
        assert_eq!(sugg.value, "actual_value");
        assert!(sugg.description.is_none());
        assert!(sugg.icon.is_none());
    }

    #[test]
    fn test_suggestion_with_value_same() {
        let sugg = Suggestion::with_value("Same", "Same");
        assert_eq!(sugg.label, "Same");
        assert_eq!(sugg.value, "Same");
    }

    #[test]
    fn test_suggestion_with_value_different() {
        let sugg = Suggestion::with_value("Label", "value123");
        assert_eq!(sugg.label, "Label");
        assert_eq!(sugg.value, "value123");
    }

    // =========================================================================
    // Suggestion::description tests
    // =========================================================================

    #[test]
    fn test_suggestion_description() {
        let sugg = Suggestion::new("Test").description("This is a description");
        assert_eq!(sugg.description, Some("This is a description".to_string()));
        assert_eq!(sugg.label, "Test");
    }

    #[test]
    fn test_suggestion_description_str() {
        let sugg = Suggestion::with_value("A", "B").description("Desc");
        assert_eq!(sugg.description, Some("Desc".to_string()));
    }

    #[test]
    fn test_suggestion_description_string() {
        let sugg = Suggestion::new("Test").description(String::from("Owned"));
        assert_eq!(sugg.description, Some("Owned".to_string()));
    }

    #[test]
    fn test_suggestion_description_chain() {
        let sugg = Suggestion::new("Test")
            .description("First")
            .description("Second");
        // Builder pattern should override
        assert_eq!(sugg.description, Some("Second".to_string()));
    }

    // =========================================================================
    // Suggestion::icon tests
    // =========================================================================

    #[test]
    fn test_suggestion_icon() {
        let sugg = Suggestion::new("Test").icon('★');
        assert_eq!(sugg.icon, Some('★'));
    }

    #[test]
    fn test_suggestion_icon_multiple() {
        let sugg = Suggestion::new("Test").icon('A').icon('B');
        // Builder pattern should override
        assert_eq!(sugg.icon, Some('B'));
    }

    #[test]
    fn test_suggestion_icon_unicode() {
        let sugg = Suggestion::new("Test").icon('你');
        assert_eq!(sugg.icon, Some('你'));
    }

    #[test]
    fn test_suggestion_icon_emoji() {
        let sugg = Suggestion::new("Test").icon('🔍');
        assert_eq!(sugg.icon, Some('🔍'));
    }

    // =========================================================================
    // Suggestion chain tests
    // =========================================================================

    #[test]
    fn test_suggestion_full_chain() {
        let sugg = Suggestion::new("Test").description("Description").icon('★');
        assert_eq!(sugg.label, "Test");
        assert_eq!(sugg.value, "Test");
        assert_eq!(sugg.description, Some("Description".to_string()));
        assert_eq!(sugg.icon, Some('★'));
    }

    #[test]
    fn test_suggestion_with_value_chain() {
        let sugg = Suggestion::with_value("Label", "value")
            .description("Desc")
            .icon('#');
        assert_eq!(sugg.label, "Label");
        assert_eq!(sugg.value, "value");
        assert_eq!(sugg.description, Some("Desc".to_string()));
        assert_eq!(sugg.icon, Some('#'));
    }

    #[test]
    fn test_suggestion_reverse_chain() {
        let sugg = Suggestion::new("Test").icon('A').description("B");
        assert_eq!(sugg.icon, Some('A'));
        assert_eq!(sugg.description, Some("B".to_string()));
    }

    // =========================================================================
    // Suggestion field access tests
    // =========================================================================

    #[test]
    fn test_suggestion_public_fields() {
        let sugg = Suggestion {
            label: "Label".to_string(),
            value: "Value".to_string(),
            description: Some("Desc".to_string()),
            icon: Some('★'),
        };
        assert_eq!(sugg.label, "Label");
        assert_eq!(sugg.value, "Value");
        assert_eq!(sugg.description, Some("Desc".to_string()));
        assert_eq!(sugg.icon, Some('★'));
    }

    #[test]
    fn test_suggestion_public_fields_none() {
        let sugg = Suggestion {
            label: "Label".to_string(),
            value: "Value".to_string(),
            description: None,
            icon: None,
        };
        assert!(sugg.description.is_none());
        assert!(sugg.icon.is_none());
    }

    // =========================================================================
    // Suggestion Clone tests
    // =========================================================================

    #[test]
    fn test_suggestion_clone() {
        let sugg1 = Suggestion::new("Test").description("Desc").icon('★');
        let sugg2 = sugg1.clone();
        assert_eq!(sugg1.label, sugg2.label);
        assert_eq!(sugg1.value, sugg2.value);
        assert_eq!(sugg1.description, sugg2.description);
        assert_eq!(sugg1.icon, sugg2.icon);
    }

    // =========================================================================
    // From impl tests
    // =========================================================================

    #[test]
    fn test_suggestion_from_str() {
        let sugg: Suggestion = "Test".into();
        assert_eq!(sugg.label, "Test");
        assert_eq!(sugg.value, "Test");
    }

    #[test]
    fn test_suggestion_from_string() {
        let sugg: Suggestion = String::from("Owned").into();
        assert_eq!(sugg.label, "Owned");
        assert_eq!(sugg.value, "Owned");
    }

    #[test]
    fn test_suggestion_from_empty() {
        let sugg: Suggestion = "".into();
        assert_eq!(sugg.label, "");
        assert_eq!(sugg.value, "");
    }

    #[test]
    fn test_suggestion_from_vs_new() {
        let s1 = Suggestion::new("Test");
        let s2: Suggestion = "Test".into();
        assert_eq!(s1.label, s2.label);
        assert_eq!(s1.value, s2.value);
    }

    // =========================================================================
    // Suggestion Debug tests
    // =========================================================================

    #[test]
    fn test_suggestion_debug() {
        let sugg = Suggestion::new("Test");
        let debug_str = format!("{:?}", sugg);
        assert!(debug_str.contains("Suggestion"));
        assert!(debug_str.contains("Test"));
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_suggestion_unicode_label() {
        let sugg = Suggestion::new("你好世界");
        assert_eq!(sugg.label, "你好世界");
        assert_eq!(sugg.value, "你好世界");
    }

    #[test]
    fn test_suggestion_unicode_value() {
        let sugg = Suggestion::with_value("Display", "値");
        assert_eq!(sugg.value, "値");
    }

    #[test]
    fn test_suggestion_unicode_description() {
        let sugg = Suggestion::new("Test").description("説明");
        assert_eq!(sugg.description, Some("説明".to_string()));
    }

    #[test]
    fn test_suggestion_long_label() {
        let long_label = "A".repeat(1000);
        let sugg = Suggestion::new(long_label.clone());
        assert_eq!(sugg.label, long_label);
        assert_eq!(sugg.value, long_label);
    }

    #[test]
    fn test_suggestion_long_value() {
        let long_value = "x".repeat(1000);
        let sugg = Suggestion::with_value("Label", long_value.clone());
        assert_eq!(sugg.value, long_value);
    }

    #[test]
    fn test_suggestion_newline_in_label() {
        let sugg = Suggestion::new("Line1\nLine2");
        assert_eq!(sugg.label, "Line1\nLine2");
    }

    #[test]
    fn test_suggestion_tab_in_label() {
        let sugg = Suggestion::new("Tab\tSeparated");
        assert_eq!(sugg.label, "Tab\tSeparated");
    }
}
