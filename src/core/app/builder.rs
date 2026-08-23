//! Application builder

use super::App;
use crate::constants::MAX_CSS_FILE_SIZE;
use crate::plugin::{Plugin, PluginRegistry};
use crate::style::{parse_css, StyleSheet};
use std::fs;
use std::path::PathBuf;

#[cfg(feature = "hot-reload")]
use super::HotReload;

/// Builder for configuring and creating an App
pub struct AppBuilder {
    stylesheet: StyleSheet,
    // To keep track of file paths for hot reload
    style_paths: Vec<PathBuf>,
    hot_reload: bool,
    devtools: bool,
    mouse_capture: bool,
    plugins: PluginRegistry,
    /// Explicit initial size; when None the size is queried from the terminal
    size: Option<(u16, u16)>,
    /// Reconcile the DOM against the view on every frame
    incremental_dom: bool,
    /// Build the DOM from the render traversal instead of `View::children`
    dom_from_render: bool,
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
            size: None,
            incremental_dom: false,
            dom_from_render: false,
        }
    }

    /// Build the DOM from the render traversal instead of `View::children`.
    ///
    /// **Off by default.** With it off the DOM only contains widgets exposed
    /// through [`View::children`](crate::widget::View::children), which almost
    /// nothing implements - a widget assembled inside `render` is invisible to
    /// it. A real application therefore has a DOM of one node, and CSS matching,
    /// `:focus`/`:hover` and devtools all work against a tree that does not
    /// describe it.
    ///
    /// With it on, the frame renders twice - once to discover the tree, once to
    /// paint it - and every widget rendered through
    /// [`RenderContext::render_child`](crate::widget::RenderContext::render_child)
    /// gets a DOM node and its own computed style. That is what makes CSS reach
    /// widgets below the root.
    ///
    /// Implies per-frame reconciliation, so `incremental_dom` has no additional
    /// effect when this is on.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let app = App::builder()
    ///     .dom_from_render(true)
    ///     .build();
    /// ```
    pub fn dom_from_render(mut self, enabled: bool) -> Self {
        self.dom_from_render = enabled;
        self
    }

    /// Reconcile the DOM against the view on every frame.
    ///
    /// **Off by default.** Without it the DOM is built once and then stops
    /// following the view, so a widget added after the first frame is invisible
    /// to CSS matching, to layout and to devtools. With it on, every frame
    /// reconciles: nodes that still match keep their `DomId`, their state
    /// (focus, hover, selection) and their cached style, and only the parts
    /// that actually changed are marked dirty.
    ///
    /// Widgets in a dynamic collection should implement
    /// [`View::key`](crate::widget::View::key) so they are matched by identity
    /// rather than by position.
    ///
    /// This is opt-in while the performance characteristics are being measured
    /// against the Phase 0 benchmark baseline; it will become the default.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let app = App::builder()
    ///     .incremental_dom(true)
    ///     .build();
    /// ```
    pub fn incremental_dom(mut self, enabled: bool) -> Self {
        self.incremental_dom = enabled;
        self
    }

    /// Set an explicit initial size instead of querying the terminal.
    ///
    /// Required for headless use (tests, snapshots, CI) where
    /// `crossterm::terminal::size()` is unavailable or non-deterministic.
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.size = Some((width.max(1), height.max(1)));
        self
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

        // Check file size to prevent DoS
        match fs::metadata(&path) {
            Ok(metadata) => {
                if metadata.len() > MAX_CSS_FILE_SIZE {
                    log_warn!(
                        "CSS file too large ({} bytes, max {}): {:?}",
                        metadata.len(),
                        MAX_CSS_FILE_SIZE,
                        path
                    );
                    return self;
                }
            }
            Err(e) => {
                log_warn!("Failed to read CSS file metadata {:?}: {}", path, e);
                return self;
            }
        }

        // Read and parse CSS file
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                log_warn!("Failed to read CSS file {:?}: {}", path, e);
                return self;
            }
        };

        match parse_css(&content) {
            Ok(sheet) => self.stylesheet.merge(sheet),
            Err(e) => log_warn!("Failed to parse CSS from {:?}: {}", path, e),
        }

        self
    }

    /// Add inline CSS styles
    pub fn css(mut self, css: impl Into<String>) -> Self {
        let css = css.into();
        match parse_css(&css) {
            Ok(sheet) => self.stylesheet.merge(sheet),
            Err(e) => log_warn!("Failed to parse inline CSS: {}", e),
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
        let initial_size = match self.size {
            Some(size) => size,
            None => {
                let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
                // Clamp to a sane minimum to avoid 0x0 buffers on some environments
                (w.max(1), h.max(1))
            }
        };

        // Collect and merge plugin styles
        let plugin_css = self.plugins.collect_styles();
        if !plugin_css.is_empty() {
            if let Ok(sheet) = parse_css(&plugin_css) {
                self.stylesheet.merge(sheet);
            }
        }

        // Initialize plugins
        if let Err(e) = self.plugins.init() {
            log_warn!("Plugin initialization failed: {}", e);
        }

        // Set up hot reload if enabled and there are style paths
        #[cfg(feature = "hot-reload")]
        let hot_reload = if self.hot_reload && !self.style_paths.is_empty() {
            match HotReload::new() {
                Ok(mut hr) => {
                    for path in &self.style_paths {
                        if let Err(e) = hr.watch(path) {
                            log_warn!("Failed to watch {:?} for hot reload: {}", path, e);
                        }
                    }
                    Some(hr)
                }
                Err(e) => {
                    log_warn!("Failed to initialize hot reload: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let incremental_dom = self.incremental_dom;
        let dom_from_render = self.dom_from_render;

        #[cfg(feature = "hot-reload")]
        let mut app = App::new_with_hot_reload(
            initial_size,
            self.stylesheet,
            self.mouse_capture,
            self.plugins,
            self.devtools,
            hot_reload,
            self.style_paths,
        );

        #[cfg(not(feature = "hot-reload"))]
        let mut app = App::new_with_plugins(
            initial_size,
            self.stylesheet,
            self.mouse_capture,
            self.plugins,
            self.devtools,
        );

        app.set_incremental_dom(incremental_dom);
        app.set_dom_from_render(dom_from_render);
        app
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// KEEP HERE - Private implementation tests (accesses private fields: style_paths, hot_reload, etc.)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new() {
        let builder = AppBuilder::new();
        assert!(builder.style_paths.is_empty());
        assert!(!builder.hot_reload);
        assert!(builder.mouse_capture);
    }

    #[test]
    fn test_builder_default_trait() {
        let builder = AppBuilder::default();
        assert!(builder.style_paths.is_empty());
        assert!(!builder.hot_reload);
        assert!(builder.mouse_capture);
    }

    #[test]
    fn test_builder_hot_reload_enabled() {
        let builder = AppBuilder::new().hot_reload(true);
        assert!(builder.hot_reload);
    }

    #[test]
    fn test_builder_hot_reload_disabled() {
        let builder = AppBuilder::new().hot_reload(false);
        assert!(!builder.hot_reload);
    }

    #[test]
    fn test_builder_devtools_enabled() {
        let builder = AppBuilder::new().devtools(true);
        assert!(builder.devtools);
    }

    #[test]
    fn test_builder_devtools_disabled() {
        let builder = AppBuilder::new().devtools(false);
        assert!(!builder.devtools);
    }

    #[test]
    fn test_builder_mouse_capture_enabled() {
        let builder = AppBuilder::new().mouse_capture(true);
        assert!(builder.mouse_capture);
    }

    #[test]
    fn test_builder_mouse_capture_disabled() {
        let builder = AppBuilder::new().mouse_capture(false);
        assert!(!builder.mouse_capture);
    }

    #[test]
    fn test_builder_css_valid() {
        let builder = AppBuilder::new().css("div { color: red; }");
        // Should parse without error; stylesheet gets updated
        assert!(!builder.stylesheet.rules.is_empty());
    }

    #[test]
    fn test_builder_css_empty() {
        let builder = AppBuilder::new().css("");
        // Empty CSS is valid, stylesheet remains empty
        assert!(builder.stylesheet.rules.is_empty());
    }

    #[test]
    fn test_builder_css_invalid() {
        // Invalid CSS should log warning but not panic
        let builder = AppBuilder::new().css("not { valid {{{ css");
        // Should still return a builder (with warning logged)
        assert!(builder.style_paths.is_empty());
    }

    #[test]
    fn test_builder_multiple_css() {
        let builder = AppBuilder::new()
            .css("div { color: red; }")
            .css("span { color: blue; }");
        // Both should be merged
        assert!(!builder.stylesheet.rules.is_empty());
    }

    #[test]
    fn test_builder_chaining() {
        let builder = AppBuilder::new()
            .hot_reload(true)
            .devtools(true)
            .mouse_capture(false)
            .css("div { display: flex; }");

        assert!(builder.hot_reload);
        assert!(builder.devtools);
        assert!(!builder.mouse_capture);
        assert!(!builder.stylesheet.rules.is_empty());
    }

    #[test]
    fn test_builder_style_nonexistent_file() {
        // Should handle missing file gracefully with warning
        let builder = AppBuilder::new().style("/nonexistent/path/style.css");
        assert_eq!(builder.style_paths.len(), 1);
        // File doesn't exist but path is tracked
    }

    #[test]
    fn test_builder_build() {
        let app = AppBuilder::new()
            .mouse_capture(false)
            .css("div { color: red; }")
            .build();
        assert!(!app.is_running());
        assert!(!app.mouse_capture);
    }

    #[test]
    fn test_builder_build_with_defaults() {
        let app = AppBuilder::new().build();
        assert!(!app.is_running());
        assert!(app.mouse_capture); // Default is true
    }

    #[test]
    #[ignore = "flaky: crossterm::terminal::size() returns (0,0) in parallel test environment"]
    fn test_builder_build_initializes_buffers() {
        let app = AppBuilder::new().build();
        // Should have initialized buffers
        assert!(app.buffers[0].width() > 0 || app.buffers[0].height() > 0);
    }

    #[test]
    fn test_builder_devtools_actually_enables() {
        // Build with devtools enabled
        let app = AppBuilder::new().devtools(true).build();

        // Verify devtools was enabled by build()
        assert!(
            app.is_devtools_enabled(),
            "devtools should be enabled after build() with devtools(true)"
        );
    }

    #[test]
    fn test_builder_devtools_disabled_by_default_when_feature_off() {
        // Build with devtools explicitly disabled
        let app = AppBuilder::new().devtools(false).build();

        // Verify devtools is disabled
        assert!(
            !app.is_devtools_enabled(),
            "devtools should be disabled when devtools(false)"
        );
    }

    #[test]
    #[cfg(feature = "hot-reload")]
    #[ignore = "HotReload::new() blocks for extended time on Windows CI (24+ minutes)"]
    fn test_builder_hot_reload_with_style_path() {
        use std::io::Write;

        // Create a temporary CSS file
        let temp_dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(_) => return, // Skip test if tempdir creation fails
        };
        let css_path = temp_dir.path().join("test.css");
        let mut file = match std::fs::File::create(&css_path) {
            Ok(f) => f,
            Err(_) => return, // Skip test if file creation fails
        };
        let _ = writeln!(file, "div {{ color: red; }}");

        // Build with hot reload enabled
        let app = AppBuilder::new().hot_reload(true).style(&css_path).build();

        // Verify hot reload is set up (app has hot_reload field)
        assert!(app.hot_reload.is_some(), "hot_reload should be initialized");
    }

    #[test]
    #[cfg(feature = "hot-reload")]
    fn test_builder_hot_reload_disabled_no_watcher() {
        // Build with hot reload disabled
        let app = AppBuilder::new().hot_reload(false).build();

        // Verify hot reload is not set up
        assert!(
            app.hot_reload.is_none(),
            "hot_reload should be None when disabled"
        );
    }

    #[test]
    #[cfg(feature = "hot-reload")]
    fn test_builder_hot_reload_no_style_paths() {
        // Build with hot reload enabled but no style paths
        let app = AppBuilder::new().hot_reload(true).build();

        // hot_reload should be None because there are no style paths to watch
        assert!(
            app.hot_reload.is_none(),
            "hot_reload should be None when no style paths"
        );
    }
}
