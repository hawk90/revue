//! Developer tools for Revue applications
//!
//! Provides debugging and inspection tools for development:
//!
//! | Tool | Description |
//! |------|-------------|
//! | [`Inspector`] | Widget tree inspector |
//! | [`StateDebugger`] | Reactive state viewer |
//! | [`StyleInspector`] | CSS style inspector |
//! | [`EventLogger`] | Event stream logger |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::devtools::{DevTools, Inspector};
//!
//! // Enable dev tools with F12
//! let app = App::builder()
//!     .with_devtools(true)
//!     .build();
//! ```
//!
//! # Widget Inspector
//!
//! ```rust,ignore
//! use revue::devtools::Inspector;
//!
//! let inspector = Inspector::new()
//!     .show_bounds(true)
//!     .show_classes(true);
//! ```

mod events;
mod helpers;
mod inspector;
mod profiler;
mod render;
mod state;
mod style;
mod time_travel;
mod types;

pub use events::{EventFilter, EventLogger, EventType, LoggedEvent};
pub use inspector::{ComponentPicker, Inspector, InspectorConfig, PickerMode, WidgetNode};
pub use profiler::{ComponentStats, Frame, Profiler, ProfilerView, RenderEvent, RenderReason};
pub use state::{StateDebugger, StateEntry, StateValue};
pub use style::{ComputedProperty, PropertySource, StyleCategory, StyleInspector};
pub use time_travel::{
    Action, SnapshotValue, StateDiff, StateSnapshot, TimeTravelConfig, TimeTravelDebugger,
    TimeTravelView,
};
pub use types::{DevToolsConfig, DevToolsPosition, DevToolsTab};

use crate::layout::Rect;

// =============================================================================
// DevTools
// =============================================================================

/// Main DevTools panel
pub struct DevTools {
    /// Configuration
    config: DevToolsConfig,
    /// Widget inspector
    inspector: Inspector,
    /// State debugger
    state: StateDebugger,
    /// Style inspector
    styles: StyleInspector,
    /// Event logger
    events: EventLogger,
    /// Performance profiler
    profiler: Profiler,
    /// Time-travel debugger
    time_travel: TimeTravelDebugger,
}

impl DevTools {
    /// Create new DevTools
    pub fn new() -> Self {
        Self {
            config: DevToolsConfig::default(),
            inspector: Inspector::new(),
            state: StateDebugger::new(),
            styles: StyleInspector::new(),
            events: EventLogger::new(),
            profiler: Profiler::new(),
            time_travel: TimeTravelDebugger::new(),
        }
    }

    /// Set configuration
    pub fn config(mut self, config: DevToolsConfig) -> Self {
        self.config = config;
        self
    }

    /// Set position
    pub fn position(mut self, position: DevToolsPosition) -> Self {
        self.config.position = position;
        self
    }

    /// Set size
    pub fn size(mut self, size: u16) -> Self {
        self.config.size = size;
        self
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.config.visible = !self.config.visible;
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.config.visible = visible;
    }

    /// Is visible
    pub fn is_visible(&self) -> bool {
        self.config.visible
    }

    /// Set active tab
    pub fn set_tab(&mut self, tab: DevToolsTab) {
        self.config.active_tab = tab;
    }

    /// Next tab
    pub fn next_tab(&mut self) {
        self.config.active_tab = self.config.active_tab.next();
    }

    /// Previous tab
    pub fn prev_tab(&mut self) {
        self.config.active_tab = self.config.active_tab.prev();
    }

    /// Get inspector
    pub fn inspector(&self) -> &Inspector {
        &self.inspector
    }

    /// Get mutable inspector
    pub fn inspector_mut(&mut self) -> &mut Inspector {
        &mut self.inspector
    }

    /// Get state debugger
    pub fn state(&self) -> &StateDebugger {
        &self.state
    }

    /// Get mutable state debugger
    pub fn state_mut(&mut self) -> &mut StateDebugger {
        &mut self.state
    }

    /// Get style inspector
    pub fn styles(&self) -> &StyleInspector {
        &self.styles
    }

    /// Get mutable style inspector
    pub fn styles_mut(&mut self) -> &mut StyleInspector {
        &mut self.styles
    }

    /// Get event logger
    pub fn events(&self) -> &EventLogger {
        &self.events
    }

    /// Get mutable event logger
    pub fn events_mut(&mut self) -> &mut EventLogger {
        &mut self.events
    }

    /// Get profiler
    pub fn profiler(&self) -> &Profiler {
        &self.profiler
    }

    /// Get mutable profiler
    pub fn profiler_mut(&mut self) -> &mut Profiler {
        &mut self.profiler
    }

    /// Get time-travel debugger
    pub fn time_travel(&self) -> &TimeTravelDebugger {
        &self.time_travel
    }

    /// Get mutable time-travel debugger
    pub fn time_travel_mut(&mut self) -> &mut TimeTravelDebugger {
        &mut self.time_travel
    }

    /// Calculate panel rect based on position
    pub fn panel_rect(&self, area: Rect) -> Option<Rect> {
        if !self.config.visible {
            return None;
        }

        let size = self.config.size;

        Some(match self.config.position {
            DevToolsPosition::Right => Rect::new(
                area.x + area.width.saturating_sub(size),
                area.y,
                size.min(area.width),
                area.height,
            ),
            DevToolsPosition::Left => Rect::new(area.x, area.y, size.min(area.width), area.height),
            DevToolsPosition::Bottom => Rect::new(
                area.x,
                area.y + area.height.saturating_sub(size),
                area.width,
                size.min(area.height),
            ),
            DevToolsPosition::Overlay => {
                // In u32: `area.width * 2` overflows u16 above 32767
                let width = (u32::from(area.width) * 2 / 3).min(80) as u16;
                let height = (u32::from(area.height) * 2 / 3).min(30) as u16;
                Rect::new(
                    area.x + (area.width - width) / 2,
                    area.y + (area.height - height) / 2,
                    width,
                    height,
                )
            }
        })
    }

    /// Calculate content area (excluding devtools)
    pub fn content_rect(&self, area: Rect) -> Rect {
        if !self.config.visible {
            return area;
        }

        let size = self.config.size;

        match self.config.position {
            DevToolsPosition::Right => {
                Rect::new(area.x, area.y, area.width.saturating_sub(size), area.height)
            }
            DevToolsPosition::Left => Rect::new(
                area.x + size.min(area.width),
                area.y,
                area.width.saturating_sub(size),
                area.height,
            ),
            DevToolsPosition::Bottom => {
                Rect::new(area.x, area.y, area.width, area.height.saturating_sub(size))
            }
            DevToolsPosition::Overlay => area,
        }
    }
}

impl Default for DevTools {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_devtools_toggle() {
        let mut devtools = DevTools::new();
        assert!(!devtools.is_visible());

        devtools.toggle();
        assert!(devtools.is_visible());

        devtools.toggle();
        assert!(!devtools.is_visible());
    }

    #[test]
    fn test_panel_rect_right() {
        let devtools = DevTools::new().size(30);
        let mut dt = devtools;
        dt.set_visible(true);

        let area = Rect::new(0, 0, 100, 50);
        let panel = dt.panel_rect(area).unwrap();

        assert_eq!(panel.x, 70);
        assert_eq!(panel.width, 30);
        assert_eq!(panel.height, 50);
    }

    #[test]
    fn test_panel_rect_left() {
        let mut devtools = DevTools::new().size(25);
        devtools.set_visible(true);
        devtools.config.position = DevToolsPosition::Left;

        let area = Rect::new(10, 5, 100, 50);
        let panel = devtools.panel_rect(area).unwrap();

        assert_eq!(panel.x, 10);
        assert_eq!(panel.y, 5);
        assert_eq!(panel.width, 25);
        assert_eq!(panel.height, 50);
    }

    #[test]
    fn test_panel_rect_bottom() {
        let mut devtools = DevTools::new().size(15);
        devtools.set_visible(true);
        devtools.config.position = DevToolsPosition::Bottom;

        let area = Rect::new(0, 0, 100, 50);
        let panel = devtools.panel_rect(area).unwrap();

        assert_eq!(panel.x, 0);
        assert_eq!(panel.y, 35);
        assert_eq!(panel.width, 100);
        assert_eq!(panel.height, 15);
    }

    #[test]
    fn test_panel_rect_overlay() {
        let mut devtools = DevTools::new();
        devtools.set_visible(true);
        devtools.config.position = DevToolsPosition::Overlay;

        let area = Rect::new(0, 0, 100, 50);
        let panel = devtools.panel_rect(area).unwrap();

        // Overlay should be centered: 2/3 of width/height, capped at 80/30
        assert_eq!(panel.width, 66); // 100 * 2 / 3 = 66
        assert_eq!(panel.height, 30); // 50 * 2 / 3 = 33, capped at 30
    }

    #[test]
    fn test_panel_rect_overlay_in_wide_area() {
        let mut devtools = DevTools::new();
        devtools.set_visible(true);
        devtools.config.position = DevToolsPosition::Overlay;

        // area.width * 2 used to overflow u16 above 32767
        let area = Rect::new(0, 0, 60_000, 60_000);
        let panel = devtools.panel_rect(area).unwrap();
        assert_eq!((panel.width, panel.height), (80, 30));
        assert_eq!((panel.x, panel.y), (29_960, 29_985));

        let area = Rect::new(0, 0, u16::MAX, u16::MAX);
        let panel = devtools.panel_rect(area).unwrap();
        assert_eq!((panel.width, panel.height), (80, 30));
    }

    #[test]
    fn test_panel_rect_invisible() {
        let mut devtools = DevTools::new();
        devtools.set_visible(false);

        let area = Rect::new(0, 0, 100, 50);
        assert!(devtools.panel_rect(area).is_none());
    }

    #[test]
    fn test_content_rect_right() {
        let mut devtools = DevTools::new().size(30);
        devtools.set_visible(true);

        let area = Rect::new(0, 0, 100, 50);
        let content = devtools.content_rect(area);

        assert_eq!(content.x, 0);
        assert_eq!(content.y, 0);
        assert_eq!(content.width, 70);
        assert_eq!(content.height, 50);
    }

    #[test]
    fn test_content_rect_left() {
        let mut devtools = DevTools::new().size(30);
        devtools.set_visible(true);
        devtools.config.position = DevToolsPosition::Left;

        let area = Rect::new(0, 0, 100, 50);
        let content = devtools.content_rect(area);

        assert_eq!(content.x, 30);
        assert_eq!(content.width, 70);
    }

    #[test]
    fn test_content_rect_bottom() {
        let mut devtools = DevTools::new().size(20);
        devtools.set_visible(true);
        devtools.config.position = DevToolsPosition::Bottom;

        let area = Rect::new(0, 0, 100, 50);
        let content = devtools.content_rect(area);

        assert_eq!(content.y, 0);
        assert_eq!(content.height, 30);
    }

    #[test]
    fn test_content_rect_overlay() {
        let mut devtools = DevTools::new();
        devtools.set_visible(true);
        devtools.config.position = DevToolsPosition::Overlay;

        let area = Rect::new(0, 0, 100, 50);
        let content = devtools.content_rect(area);

        // Overlay doesn't reduce content area
        assert_eq!(content, area);
    }

    #[test]
    fn test_content_rect_invisible() {
        let mut devtools = DevTools::new();
        devtools.set_visible(false);

        let area = Rect::new(0, 0, 100, 50);
        let content = devtools.content_rect(area);

        assert_eq!(content, area);
    }

    #[test]
    fn test_devtools_new() {
        let devtools = DevTools::new();
        assert!(!devtools.is_visible());
        assert_eq!(devtools.config.active_tab, DevToolsTab::Inspector);
    }

    #[test]
    fn test_devtools_default() {
        let devtools = DevTools::default();
        assert!(!devtools.is_visible());
        assert_eq!(devtools.config.active_tab, DevToolsTab::Inspector);
    }

    #[test]
    fn test_devtools_config_builder() {
        let config = DevToolsConfig {
            position: DevToolsPosition::Left,
            size: 40,
            visible: true,
            active_tab: DevToolsTab::Profiler,
            bg_color: Color::rgb(10, 10, 10),
            fg_color: Color::rgb(255, 255, 255),
            accent_color: Color::rgb(100, 100, 255),
        };

        let devtools = DevTools::new().config(config);
        assert!(devtools.is_visible());
        assert_eq!(devtools.config.position, DevToolsPosition::Left);
        assert_eq!(devtools.config.size, 40);
        assert_eq!(devtools.config.active_tab, DevToolsTab::Profiler);
    }

    #[test]
    fn test_devtools_position() {
        let devtools = DevTools::new().position(DevToolsPosition::Bottom);
        assert_eq!(devtools.config.position, DevToolsPosition::Bottom);
    }

    #[test]
    fn test_devtools_size() {
        let devtools = DevTools::new().size(60);
        assert_eq!(devtools.config.size, 60);
    }

    #[test]
    fn test_devtools_set_visible() {
        let mut devtools = DevTools::new();
        assert!(!devtools.is_visible());

        devtools.set_visible(true);
        assert!(devtools.is_visible());

        devtools.set_visible(false);
        assert!(!devtools.is_visible());
    }

    #[test]
    fn test_devtools_set_tab() {
        let mut devtools = DevTools::new();
        assert_eq!(devtools.config.active_tab, DevToolsTab::Inspector);

        devtools.set_tab(DevToolsTab::Profiler);
        assert_eq!(devtools.config.active_tab, DevToolsTab::Profiler);
    }

    #[test]
    fn test_devtools_next_tab() {
        let mut devtools = DevTools::new();
        assert_eq!(devtools.config.active_tab, DevToolsTab::Inspector);

        devtools.next_tab();
        assert_eq!(devtools.config.active_tab, DevToolsTab::State);

        devtools.next_tab();
        assert_eq!(devtools.config.active_tab, DevToolsTab::Styles);
    }

    #[test]
    fn test_devtools_prev_tab() {
        let mut devtools = DevTools::new();
        assert_eq!(devtools.config.active_tab, DevToolsTab::Inspector);

        devtools.prev_tab();
        assert_eq!(devtools.config.active_tab, DevToolsTab::TimeTravel);

        devtools.prev_tab();
        assert_eq!(devtools.config.active_tab, DevToolsTab::Profiler);
    }

    #[test]
    fn test_devtools_getters() {
        let devtools = DevTools::new();

        // Test inspector getter - just check it returns a reference
        let _inspector = devtools.inspector();
        let _state = devtools.state();
        let _styles = devtools.styles();
        let _events = devtools.events();
        let _profiler = devtools.profiler();
        let _time_travel = devtools.time_travel();
    }

    #[test]
    fn test_devtools_getters_mut() {
        let mut devtools = DevTools::new();

        // Test mutable getters
        devtools.inspector_mut();
        devtools.state_mut();
        devtools.styles_mut();
        devtools.events_mut();
        devtools.profiler_mut();
        devtools.time_travel_mut();
    }

    #[test]
    fn test_panel_rect_saturation() {
        let mut devtools = DevTools::new().size(200);
        devtools.set_visible(true);

        let area = Rect::new(0, 0, 100, 50);
        let panel = devtools.panel_rect(area).unwrap();

        // Size should be capped at area size
        assert_eq!(panel.width, 100);
        assert_eq!(panel.height, 50);
    }

    #[test]
    fn test_content_rect_saturation() {
        let mut devtools = DevTools::new().size(200);
        devtools.set_visible(true);

        let area = Rect::new(0, 0, 100, 50);
        let content = devtools.content_rect(area);

        // Content should handle large size gracefully
        assert_eq!(content.width, 0); // 100 - 100 = 0
        assert_eq!(content.height, 50);
    }
}
