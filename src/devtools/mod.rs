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
mod inspector;
mod profiler;
mod state;
mod style;

pub use events::{EventFilter, EventLogger, EventType, LoggedEvent};
pub use inspector::{ComponentPicker, Inspector, InspectorConfig, PickerMode, WidgetNode};
pub use profiler::{ComponentStats, Frame, Profiler, ProfilerView, RenderEvent, RenderReason};
pub use state::{StateDebugger, StateEntry, StateValue};
pub use style::{ComputedProperty, PropertySource, StyleCategory, StyleInspector};

use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::sync::atomic::{AtomicBool, Ordering};

// =============================================================================
// DevTools Panel
// =============================================================================

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
        ]
    }

    /// Next tab
    pub fn next(&self) -> Self {
        match self {
            Self::Inspector => Self::State,
            Self::State => Self::Styles,
            Self::Styles => Self::Events,
            Self::Events => Self::Profiler,
            Self::Profiler => Self::Inspector,
        }
    }

    /// Previous tab
    pub fn prev(&self) -> Self {
        match self {
            Self::Inspector => Self::Profiler,
            Self::State => Self::Inspector,
            Self::Styles => Self::State,
            Self::Events => Self::Styles,
            Self::Profiler => Self::Events,
        }
    }
}

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
                let width = (area.width * 2 / 3).min(80);
                let height = (area.height * 2 / 3).min(30);
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

    /// Render devtools panel
    pub fn render(&self, buffer: &mut Buffer, area: Rect) {
        if let Some(panel) = self.panel_rect(area) {
            self.render_panel(buffer, panel);
        }
    }

    fn render_panel(&self, buffer: &mut Buffer, area: Rect) {
        // Fill background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buffer.get_mut(x, y) {
                    cell.symbol = ' ';
                    cell.bg = Some(self.config.bg_color);
                    cell.fg = Some(self.config.fg_color);
                }
            }
        }

        // Draw border
        self.draw_border(buffer, area);

        // Tab bar
        let tab_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, 1);
        self.render_tabs(buffer, tab_area);

        // Content area
        let content_area = Rect::new(
            area.x + 1,
            area.y + 3,
            area.width - 2,
            area.height.saturating_sub(4),
        );

        match self.config.active_tab {
            DevToolsTab::Inspector => {
                self.inspector
                    .render_content(buffer, content_area, &self.config)
            }
            DevToolsTab::State => self
                .state
                .render_content(buffer, content_area, &self.config),
            DevToolsTab::Styles => self
                .styles
                .render_content(buffer, content_area, &self.config),
            DevToolsTab::Events => self
                .events
                .render_content(buffer, content_area, &self.config),
            DevToolsTab::Profiler => {
                self.profiler
                    .render_content(buffer, content_area, &self.config)
            }
        }
    }

    fn render_tabs(&self, buffer: &mut Buffer, area: Rect) {
        let mut x = area.x;

        for tab in DevToolsTab::all() {
            let label = format!(" {} ", tab.label());
            let is_active = *tab == self.config.active_tab;

            let (fg, bg) = if is_active {
                (self.config.bg_color, self.config.accent_color)
            } else {
                (self.config.fg_color, self.config.bg_color)
            };

            for ch in label.chars() {
                if x < area.x + area.width {
                    if let Some(cell) = buffer.get_mut(x, area.y) {
                        cell.symbol = ch;
                        cell.fg = Some(fg);
                        cell.bg = Some(bg);
                    }
                    x += 1;
                }
            }

            x += 1; // Gap between tabs
        }
    }

    fn draw_border(&self, buffer: &mut Buffer, area: Rect) {
        let color = self.config.accent_color;

        // Corners and edges
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get_mut(x, area.y) {
                cell.symbol = if x == area.x {
                    '┌'
                } else if x == area.x + area.width - 1 {
                    '┐'
                } else {
                    '─'
                };
                cell.fg = Some(color);
            }
            if let Some(cell) = buffer.get_mut(x, area.y + area.height - 1) {
                cell.symbol = if x == area.x {
                    '└'
                } else if x == area.x + area.width - 1 {
                    '┘'
                } else {
                    '─'
                };
                cell.fg = Some(color);
            }
        }

        for y in area.y + 1..area.y + area.height - 1 {
            if let Some(cell) = buffer.get_mut(area.x, y) {
                cell.symbol = '│';
                cell.fg = Some(color);
            }
            if let Some(cell) = buffer.get_mut(area.x + area.width - 1, y) {
                cell.symbol = '│';
                cell.fg = Some(color);
            }
        }

        // Separator after tabs
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get_mut(x, area.y + 2) {
                cell.symbol = if x == area.x {
                    '├'
                } else if x == area.x + area.width - 1 {
                    '┤'
                } else {
                    '─'
                };
                cell.fg = Some(color);
            }
        }
    }
}

impl Default for DevTools {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Global DevTools State (Deprecated)
// =============================================================================
//
// These global functions are deprecated. Use `App::is_devtools_enabled()`,
// `App::enable_devtools()`, `App::disable_devtools()`, and `App::toggle_devtools()`
// instead for proper test isolation and cleaner architecture.

static DEVTOOLS_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable global devtools
///
/// # Deprecated
/// Use `App::enable_devtools()` instead for proper test isolation.
#[deprecated(since = "2.1.0", note = "Use App::enable_devtools() instead")]
pub fn enable_devtools() {
    DEVTOOLS_ENABLED.store(true, Ordering::Relaxed);
}

/// Disable global devtools
///
/// # Deprecated
/// Use `App::disable_devtools()` instead for proper test isolation.
#[deprecated(since = "2.1.0", note = "Use App::disable_devtools() instead")]
pub fn disable_devtools() {
    DEVTOOLS_ENABLED.store(false, Ordering::Relaxed);
}

/// Check if devtools are enabled
///
/// # Deprecated
/// Use `App::is_devtools_enabled()` instead for proper test isolation.
#[deprecated(since = "2.1.0", note = "Use App::is_devtools_enabled() instead")]
pub fn is_devtools_enabled() -> bool {
    DEVTOOLS_ENABLED.load(Ordering::Relaxed)
}

/// Toggle devtools
///
/// # Deprecated
/// Use `App::toggle_devtools()` instead for proper test isolation.
#[deprecated(since = "2.1.0", note = "Use App::toggle_devtools() instead")]
pub fn toggle_devtools() -> bool {
    let was = DEVTOOLS_ENABLED.fetch_xor(true, Ordering::Relaxed);
    !was
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devtools_config_default() {
        let config = DevToolsConfig::default();
        assert!(!config.visible);
        assert_eq!(config.position, DevToolsPosition::Right);
        assert_eq!(config.active_tab, DevToolsTab::Inspector);
    }

    #[test]
    fn test_devtools_tab_cycle() {
        let tab = DevToolsTab::Inspector;
        assert_eq!(tab.next(), DevToolsTab::State);
        assert_eq!(tab.prev(), DevToolsTab::Profiler);
    }

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
    }

    #[test]
    fn test_content_rect() {
        let mut devtools = DevTools::new().size(30);
        devtools.set_visible(true);

        let area = Rect::new(0, 0, 100, 50);
        let content = devtools.content_rect(area);

        assert_eq!(content.width, 70);
    }
}
