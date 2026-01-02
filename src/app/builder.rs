//! Application builder

use super::App;
use crate::plugin::{Plugin, PluginRegistry};
use crate::style::{parse_css, StyleSheet};
use std::fs;
use std::path::PathBuf;
use tracing::warn;

/// Builder for configuring and creating an App
pub struct AppBuilder {
    stylesheet: StyleSheet,
    // To keep track of file paths for hot reload
    style_paths: Vec<PathBuf>,
    hot_reload: bool,
    devtools: bool,
    mouse_capture: bool,
    plugins: PluginRegistry,
}

impl AppBuilder {
    /// Create a new application builder
    pub fn new() -> Self {
        Self {
            stylesheet: StyleSheet::new(),
            style_paths: Vec::new(),
            hot_reload: false,
            devtools: cfg!(feature = "devtools"),
            mouse_capture: true,
            plugins: PluginRegistry::new(),
        }
    }

    /// Register a plugin
    ///
    /// Plugins are initialized when the app is built and can hook into
    /// the application lifecycle.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use revue::plugin::LoggerPlugin;
    ///
    /// let app = App::builder()
    ///     .plugin(LoggerPlugin::new())
    ///     .build();
    /// ```
    pub fn plugin<P: Plugin + 'static>(mut self, plugin: P) -> Self {
        self.plugins.register(plugin);
        self
    }

    /// Add a CSS stylesheet from file
    pub fn style(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        self.style_paths.push(path.clone());
        match fs::read_to_string(&path) {
            Ok(content) => match parse_css(&content) {
                Ok(sheet) => self.stylesheet.merge(sheet),
                Err(e) => warn!("Failed to parse CSS from {:?}: {}", path, e),
            },
            Err(e) => warn!("Failed to read CSS file {:?}: {}", path, e),
        }
        self
    }

    /// Add inline CSS styles
    pub fn css(mut self, css: impl Into<String>) -> Self {
        let css = css.into();
        match parse_css(&css) {
            Ok(sheet) => self.stylesheet.merge(sheet),
            Err(e) => warn!("Failed to parse inline CSS: {}", e),
        }
        self
    }

    /// Enable hot reload for CSS files
    pub fn hot_reload(mut self, enabled: bool) -> Self {
        self.hot_reload = enabled;
        self
    }

    /// Enable devtools
    pub fn devtools(mut self, enabled: bool) -> Self {
        self.devtools = enabled;
        self
    }

    /// Enable/disable mouse capture
    pub fn mouse_capture(mut self, enabled: bool) -> Self {
        self.mouse_capture = enabled;
        self
    }

    /// Build the application
    pub fn build(mut self) -> App {
        let initial_size = crossterm::terminal::size().unwrap_or((80, 24));

        // Collect and merge plugin styles
        let plugin_css = self.plugins.collect_styles();
        if !plugin_css.is_empty() {
            if let Ok(sheet) = parse_css(&plugin_css) {
                self.stylesheet.merge(sheet);
            }
        }

        // Initialize plugins
        if let Err(e) = self.plugins.init() {
            warn!("Plugin initialization failed: {}", e);
        }

        App::new_with_plugins(
            initial_size,
            self.stylesheet,
            self.mouse_capture,
            self.plugins,
        )
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let builder = AppBuilder::new();
        assert!(builder.style_paths.is_empty());
        assert!(!builder.hot_reload);
    }

    #[test]
    fn test_builder_hot_reload() {
        let builder = AppBuilder::new().hot_reload(true);
        assert!(builder.hot_reload);
    }
}
