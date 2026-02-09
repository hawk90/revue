//! CalloutType implementations

use super::types::CalloutType;
use crate::style::Color;

impl CalloutType {
    /// Get the default icon for this callout type
    pub fn icon(&self) -> char {
        match self {
            CalloutType::Note => '📝',
            CalloutType::Tip => '💡',
            CalloutType::Important => '❗',
            CalloutType::Warning => '⚠',
            CalloutType::Danger => '🔴',
            CalloutType::Info => 'ℹ',
        }
    }

    /// Get the accent color for this callout type
    pub fn accent_color(&self) -> Color {
        match self {
            CalloutType::Note => Color::rgb(59, 130, 246), // Blue
            CalloutType::Tip => Color::rgb(34, 197, 94),   // Green
            CalloutType::Important => Color::rgb(168, 85, 247), // Purple
            CalloutType::Warning => Color::rgb(234, 179, 8), // Yellow
            CalloutType::Danger => Color::rgb(239, 68, 68), // Red
            CalloutType::Info => Color::rgb(6, 182, 212),  // Cyan
        }
    }

    /// Get the background color for this callout type
    pub fn bg_color(&self) -> Color {
        match self {
            CalloutType::Note => Color::rgb(23, 37, 53), // Dark blue
            CalloutType::Tip => Color::rgb(20, 40, 28),  // Dark green
            CalloutType::Important => Color::rgb(35, 25, 50), // Dark purple
            CalloutType::Warning => Color::rgb(45, 38, 15), // Dark yellow
            CalloutType::Danger => Color::rgb(50, 20, 20), // Dark red
            CalloutType::Info => Color::rgb(15, 40, 45), // Dark cyan
        }
    }

    /// Get the title text color for this callout type
    pub fn title_color(&self) -> Color {
        self.accent_color()
    }

    /// Get the default title for this callout type
    pub fn default_title(&self) -> &'static str {
        match self {
            CalloutType::Note => "Note",
            CalloutType::Tip => "Tip",
            CalloutType::Important => "Important",
            CalloutType::Warning => "Warning",
            CalloutType::Danger => "Danger",
            CalloutType::Info => "Info",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // CalloutType::icon() tests
    // =========================================================================

    #[test]
    fn test_callout_type_icon_note() {
        assert_eq!(CalloutType::Note.icon(), '📝');
    }

    #[test]
    fn test_callout_type_icon_tip() {
        assert_eq!(CalloutType::Tip.icon(), '💡');
    }

    #[test]
    fn test_callout_type_icon_important() {
        assert_eq!(CalloutType::Important.icon(), '❗');
    }

    #[test]
    fn test_callout_type_icon_warning() {
        assert_eq!(CalloutType::Warning.icon(), '⚠');
    }

    #[test]
    fn test_callout_type_icon_danger() {
        assert_eq!(CalloutType::Danger.icon(), '🔴');
    }

    #[test]
    fn test_callout_type_icon_info() {
        assert_eq!(CalloutType::Info.icon(), 'ℹ');
    }

    // =========================================================================
    // CalloutType::accent_color() tests
    // =========================================================================

    #[test]
    fn test_callout_type_accent_color_note() {
        assert_eq!(CalloutType::Note.accent_color(), Color::rgb(59, 130, 246));
    }

    #[test]
    fn test_callout_type_accent_color_tip() {
        assert_eq!(CalloutType::Tip.accent_color(), Color::rgb(34, 197, 94));
    }

    #[test]
    fn test_callout_type_accent_color_important() {
        assert_eq!(
            CalloutType::Important.accent_color(),
            Color::rgb(168, 85, 247)
        );
    }

    #[test]
    fn test_callout_type_accent_color_warning() {
        assert_eq!(CalloutType::Warning.accent_color(), Color::rgb(234, 179, 8));
    }

    #[test]
    fn test_callout_type_accent_color_danger() {
        assert_eq!(CalloutType::Danger.accent_color(), Color::rgb(239, 68, 68));
    }

    #[test]
    fn test_callout_type_accent_color_info() {
        assert_eq!(CalloutType::Info.accent_color(), Color::rgb(6, 182, 212));
    }

    // =========================================================================
    // CalloutType::bg_color() tests
    // =========================================================================

    #[test]
    fn test_callout_type_bg_color_note() {
        assert_eq!(CalloutType::Note.bg_color(), Color::rgb(23, 37, 53));
    }

    #[test]
    fn test_callout_type_bg_color_tip() {
        assert_eq!(CalloutType::Tip.bg_color(), Color::rgb(20, 40, 28));
    }

    #[test]
    fn test_callout_type_bg_color_important() {
        assert_eq!(CalloutType::Important.bg_color(), Color::rgb(35, 25, 50));
    }

    #[test]
    fn test_callout_type_bg_color_warning() {
        assert_eq!(CalloutType::Warning.bg_color(), Color::rgb(45, 38, 15));
    }

    #[test]
    fn test_callout_type_bg_color_danger() {
        assert_eq!(CalloutType::Danger.bg_color(), Color::rgb(50, 20, 20));
    }

    #[test]
    fn test_callout_type_bg_color_info() {
        assert_eq!(CalloutType::Info.bg_color(), Color::rgb(15, 40, 45));
    }

    // =========================================================================
    // CalloutType::title_color() tests
    // =========================================================================

    #[test]
    fn test_callout_type_title_color_note() {
        assert_eq!(
            CalloutType::Note.title_color(),
            CalloutType::Note.accent_color()
        );
    }

    #[test]
    fn test_callout_type_title_color_tip() {
        assert_eq!(
            CalloutType::Tip.title_color(),
            CalloutType::Tip.accent_color()
        );
    }

    #[test]
    fn test_callout_type_title_color_danger() {
        assert_eq!(
            CalloutType::Danger.title_color(),
            CalloutType::Danger.accent_color()
        );
    }

    // =========================================================================
    // CalloutType::default_title() tests
    // =========================================================================

    #[test]
    fn test_callout_type_default_title_note() {
        assert_eq!(CalloutType::Note.default_title(), "Note");
    }

    #[test]
    fn test_callout_type_default_title_tip() {
        assert_eq!(CalloutType::Tip.default_title(), "Tip");
    }

    #[test]
    fn test_callout_type_default_title_important() {
        assert_eq!(CalloutType::Important.default_title(), "Important");
    }

    #[test]
    fn test_callout_type_default_title_warning() {
        assert_eq!(CalloutType::Warning.default_title(), "Warning");
    }

    #[test]
    fn test_callout_type_default_title_danger() {
        assert_eq!(CalloutType::Danger.default_title(), "Danger");
    }

    #[test]
    fn test_callout_type_default_title_info() {
        assert_eq!(CalloutType::Info.default_title(), "Info");
    }

    // =========================================================================
    // Integration tests
    // =========================================================================

    #[test]
    fn test_callout_type_consistency() {
        // All types should have icons, colors, and titles
        for ct in [
            CalloutType::Note,
            CalloutType::Tip,
            CalloutType::Important,
            CalloutType::Warning,
            CalloutType::Danger,
            CalloutType::Info,
        ] {
            let _ = ct.icon();
            let _ = ct.accent_color();
            let _ = ct.bg_color();
            let _ = ct.title_color();
            let _ = ct.default_title();
        }
    }

    #[test]
    fn test_callout_type_different_icons() {
        let icons = [
            CalloutType::Note.icon(),
            CalloutType::Tip.icon(),
            CalloutType::Important.icon(),
            CalloutType::Warning.icon(),
            CalloutType::Danger.icon(),
            CalloutType::Info.icon(),
        ];
        // All icons should be unique
        assert_eq!(icons.len(), 6);
        let unique: std::collections::HashSet<_> = icons.iter().collect();
        assert_eq!(unique.len(), 6);
    }

    #[test]
    fn test_callout_type_different_titles() {
        let titles = [
            CalloutType::Note.default_title(),
            CalloutType::Tip.default_title(),
            CalloutType::Important.default_title(),
            CalloutType::Warning.default_title(),
            CalloutType::Danger.default_title(),
            CalloutType::Info.default_title(),
        ];
        // All titles should be unique
        assert_eq!(titles.len(), 6);
        let unique: std::collections::HashSet<_> = titles.iter().collect();
        assert_eq!(unique.len(), 6);
    }
}
