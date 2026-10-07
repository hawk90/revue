//! Modal button configuration and style presets

/// Button configuration for modal dialogs
///
/// This is distinct from the interactive `Button` widget.
/// `ModalButton` configures the appearance and label of buttons
/// shown at the bottom of modal dialogs.
#[derive(Clone)]
pub struct ModalButton {
    /// Button label
    pub label: String,
    /// Button style
    pub style: ModalButtonStyle,
}

/// Style preset for modal buttons
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ModalButtonStyle {
    /// Default neutral button
    #[default]
    Default,
    /// Primary action button (highlighted)
    Primary,
    /// Danger/destructive action button
    Danger,
}

impl ModalButton {
    /// Create a new button with default style
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Default,
        }
    }

    /// Create a primary action button
    pub fn primary(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Primary,
        }
    }

    /// Create a danger/destructive action button
    pub fn danger(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Danger,
        }
    }

    /// Set button style
    pub fn style(mut self, style: ModalButtonStyle) -> Self {
        self.style = style;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_button_styles() {
        let btn = ModalButton::new("Test");
        assert!(matches!(btn.style, ModalButtonStyle::Default));

        let btn = ModalButton::primary("OK");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));

        let btn = ModalButton::danger("Delete");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    // =========================================================================
    // ModalButtonStyle enum tests
    // =========================================================================

    #[test]
    fn test_modal_button_style_default() {
        let style = ModalButtonStyle::default();
        assert!(matches!(style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_style_clone() {
        let style1 = ModalButtonStyle::Primary;
        let style2 = style1;
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_modal_button_style_copy() {
        let style1 = ModalButtonStyle::Danger;
        let style2 = style1;
        assert_eq!(style2, ModalButtonStyle::Danger);
        // style1 is still valid because of Copy
        assert_eq!(style1, ModalButtonStyle::Danger);
    }

    #[test]
    fn test_modal_button_style_partial_eq() {
        assert_eq!(ModalButtonStyle::Default, ModalButtonStyle::Default);
        assert_eq!(ModalButtonStyle::Primary, ModalButtonStyle::Primary);
        assert_eq!(ModalButtonStyle::Danger, ModalButtonStyle::Danger);

        assert_ne!(ModalButtonStyle::Default, ModalButtonStyle::Primary);
        assert_ne!(ModalButtonStyle::Primary, ModalButtonStyle::Danger);
        assert_ne!(ModalButtonStyle::Danger, ModalButtonStyle::Default);
    }

    #[test]
    fn test_modal_button_style_all_variants() {
        let styles = [
            ModalButtonStyle::Default,
            ModalButtonStyle::Primary,
            ModalButtonStyle::Danger,
        ];

        for (i, style1) in styles.iter().enumerate() {
            for (j, style2) in styles.iter().enumerate() {
                if i == j {
                    assert_eq!(style1, style2);
                } else {
                    assert_ne!(style1, style2);
                }
            }
        }
    }

    // =========================================================================
    // ModalButton Clone trait tests
    // =========================================================================

    #[test]
    fn test_modal_button_clone() {
        let btn1 = ModalButton::new("Test").style(ModalButtonStyle::Primary);
        let btn2 = btn1.clone();

        assert_eq!(btn1.label, btn2.label);
        assert_eq!(btn1.style, btn2.style);
    }

    // =========================================================================
    // ModalButton builder method tests
    // =========================================================================

    #[test]
    fn test_modal_button_new_with_string() {
        let label = String::from("Owned Label");
        let btn = ModalButton::new(label);
        assert_eq!(btn.label, "Owned Label");
        assert!(matches!(btn.style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_new_with_str() {
        let btn = ModalButton::new("Test Label");
        assert_eq!(btn.label, "Test Label");
        assert!(matches!(btn.style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_empty_label() {
        let btn = ModalButton::new("");
        assert_eq!(btn.label, "");
    }

    #[test]
    fn test_modal_button_primary_with_string() {
        let label = String::from("Submit");
        let btn = ModalButton::primary(label);
        assert_eq!(btn.label, "Submit");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));
    }

    #[test]
    fn test_modal_button_primary_with_str() {
        let btn = ModalButton::primary("OK");
        assert_eq!(btn.label, "OK");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));
    }

    #[test]
    fn test_modal_button_danger_with_string() {
        let label = String::from("Delete");
        let btn = ModalButton::danger(label);
        assert_eq!(btn.label, "Delete");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    #[test]
    fn test_modal_button_danger_with_str() {
        let btn = ModalButton::danger("Cancel");
        assert_eq!(btn.label, "Cancel");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    #[test]
    fn test_modal_button_all_distinct() {
        let default_btn = ModalButton::new("Default");
        let primary_btn = ModalButton::primary("Primary");
        let danger_btn = ModalButton::danger("Danger");

        assert!(matches!(default_btn.style, ModalButtonStyle::Default));
        assert!(matches!(primary_btn.style, ModalButtonStyle::Primary));
        assert!(matches!(danger_btn.style, ModalButtonStyle::Danger));
    }
}
