//! DevTools panel settings: position, configuration and tabs

use crate::style::Color;

/// DevTools panel position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DevToolsPosition {
    /// Right side panel
    #[default]
    Right,
    /// Bottom panel
    Bottom,
    /// Left side panel
    Left,
    /// Floating overlay
    Overlay,
}

/// DevTools configuration
#[derive(Debug, Clone)]
pub struct DevToolsConfig {
    /// Panel position
    pub position: DevToolsPosition,
    /// Panel size (width or height depending on position)
    pub size: u16,
    /// Is visible
    pub visible: bool,
    /// Active tab
    pub active_tab: DevToolsTab,
    /// Background color
    pub bg_color: Color,
    /// Text color
    pub fg_color: Color,
    /// Accent color
    pub accent_color: Color,
}

impl Default for DevToolsConfig {
    fn default() -> Self {
        Self {
            position: DevToolsPosition::Right,
            size: 50,
            visible: false,
            active_tab: DevToolsTab::Inspector,
            bg_color: Color::rgb(25, 25, 35),
            fg_color: Color::rgb(200, 200, 210),
            accent_color: Color::rgb(130, 180, 255),
        }
    }
}

/// DevTools tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DevToolsTab {
    /// Widget inspector
    #[default]
    Inspector,
    /// State debugger
    State,
    /// Style inspector
    Styles,
    /// Event logger
    Events,
    /// Performance profiler
    Profiler,
    /// Time-travel debugger
    TimeTravel,
}

impl DevToolsTab {
    /// Get tab label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Inspector => "Inspector",
            Self::State => "State",
            Self::Styles => "Styles",
            Self::Events => "Events",
            Self::Profiler => "Profiler",
            Self::TimeTravel => "Travel",
        }
    }

    /// Get all tabs
    pub fn all() -> &'static [DevToolsTab] {
        &[
            DevToolsTab::Inspector,
            DevToolsTab::State,
            DevToolsTab::Styles,
            DevToolsTab::Events,
            DevToolsTab::Profiler,
            DevToolsTab::TimeTravel,
        ]
    }

    /// Next tab
    pub fn next(&self) -> Self {
        match self {
            Self::Inspector => Self::State,
            Self::State => Self::Styles,
            Self::Styles => Self::Events,
            Self::Events => Self::Profiler,
            Self::Profiler => Self::TimeTravel,
            Self::TimeTravel => Self::Inspector,
        }
    }

    /// Previous tab
    pub fn prev(&self) -> Self {
        match self {
            Self::Inspector => Self::TimeTravel,
            Self::State => Self::Inspector,
            Self::Styles => Self::State,
            Self::Events => Self::Styles,
            Self::Profiler => Self::Events,
            Self::TimeTravel => Self::Profiler,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devtools_config_default() {
        let config = DevToolsConfig::default();
        assert!(!config.visible);
        assert_eq!(config.position, DevToolsPosition::Right);
        assert_eq!(config.active_tab, DevToolsTab::Inspector);
        assert_eq!(config.size, 50);
    }

    #[test]
    fn test_devtools_tab_cycle() {
        let tab = DevToolsTab::Inspector;
        assert_eq!(tab.next(), DevToolsTab::State);
        assert_eq!(tab.prev(), DevToolsTab::TimeTravel);
    }

    #[test]
    fn test_devtools_tab_label() {
        assert_eq!(DevToolsTab::Inspector.label(), "Inspector");
        assert_eq!(DevToolsTab::State.label(), "State");
        assert_eq!(DevToolsTab::Styles.label(), "Styles");
        assert_eq!(DevToolsTab::Events.label(), "Events");
        assert_eq!(DevToolsTab::Profiler.label(), "Profiler");
        assert_eq!(DevToolsTab::TimeTravel.label(), "Travel");
    }

    #[test]
    fn test_devtools_tab_all() {
        let all = DevToolsTab::all();
        assert_eq!(all.len(), 6);
        assert_eq!(all[0], DevToolsTab::Inspector);
        assert_eq!(all[5], DevToolsTab::TimeTravel);
    }

    #[test]
    fn test_devtools_tab_next_cycle() {
        assert_eq!(DevToolsTab::Inspector.next(), DevToolsTab::State);
        assert_eq!(DevToolsTab::State.next(), DevToolsTab::Styles);
        assert_eq!(DevToolsTab::Styles.next(), DevToolsTab::Events);
        assert_eq!(DevToolsTab::Events.next(), DevToolsTab::Profiler);
        assert_eq!(DevToolsTab::Profiler.next(), DevToolsTab::TimeTravel);
        assert_eq!(DevToolsTab::TimeTravel.next(), DevToolsTab::Inspector);
    }

    #[test]
    fn test_devtools_tab_prev_cycle() {
        assert_eq!(DevToolsTab::Inspector.prev(), DevToolsTab::TimeTravel);
        assert_eq!(DevToolsTab::TimeTravel.prev(), DevToolsTab::Profiler);
        assert_eq!(DevToolsTab::Profiler.prev(), DevToolsTab::Events);
        assert_eq!(DevToolsTab::Events.prev(), DevToolsTab::Styles);
        assert_eq!(DevToolsTab::Styles.prev(), DevToolsTab::State);
        assert_eq!(DevToolsTab::State.prev(), DevToolsTab::Inspector);
    }

    #[test]
    fn test_devtools_position_default() {
        assert_eq!(DevToolsPosition::default(), DevToolsPosition::Right);
    }

    #[test]
    fn test_devtools_tab_default() {
        assert_eq!(DevToolsTab::default(), DevToolsTab::Inspector);
    }
}
