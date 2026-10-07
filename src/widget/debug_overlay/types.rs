//! Debug panel position and configuration

use crate::style::Color;
use crate::widget::theme::{EDITOR_BG, SECONDARY_TEXT};

/// Position for debug panel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DebugPosition {
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    #[default]
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
}

/// Debug panel configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Show performance metrics
    pub show_metrics: bool,
    /// Show widget tree
    pub show_tree: bool,
    /// Show event log
    pub show_events: bool,
    /// Show style inspector
    pub show_styles: bool,
    /// Panel position
    pub position: DebugPosition,
    /// Panel width
    pub width: u16,
    /// Maximum height
    pub max_height: u16,
    /// Panel opacity (0-255)
    pub opacity: u8,
    /// Background color
    pub bg_color: Color,
    /// Text color
    pub fg_color: Color,
    /// Accent color
    pub accent_color: Color,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            show_metrics: true,
            show_tree: false,
            show_events: false,
            show_styles: false,
            position: DebugPosition::TopRight,
            width: 40,
            max_height: 20,
            opacity: 220,
            bg_color: EDITOR_BG,
            fg_color: SECONDARY_TEXT,
            accent_color: Color::rgb(100, 200, 255),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_config_default() {
        let config = DebugConfig::default();
        assert!(config.show_metrics);
        assert!(!config.show_tree);
        assert!(!config.show_events);
        assert_eq!(config.width, 40);
    }

    // =========================================================================
    // DebugPosition enum tests
    // =========================================================================

    #[test]
    fn test_debug_position_default() {
        let pos = DebugPosition::default();
        assert_eq!(pos, DebugPosition::TopRight);
    }

    #[test]
    fn test_debug_position_clone() {
        let pos = DebugPosition::BottomLeft;
        let cloned = pos;
        assert_eq!(pos, cloned);
    }

    #[test]
    fn test_debug_position_copy() {
        let pos1 = DebugPosition::TopLeft;
        let pos2 = pos1;
        assert_eq!(pos1, DebugPosition::TopLeft);
        assert_eq!(pos2, DebugPosition::TopLeft);
    }

    #[test]
    fn test_debug_position_partial_eq() {
        assert_eq!(DebugPosition::TopLeft, DebugPosition::TopLeft);
        assert_ne!(DebugPosition::TopLeft, DebugPosition::BottomLeft);
    }

    #[test]
    fn test_debug_position_debug() {
        let pos = DebugPosition::BottomRight;
        assert!(format!("{:?}", pos).contains("BottomRight"));
    }
}
