//! Application lifecycle and coordination
//!
//! This module provides the main entry point for Revue applications.
//!
//! # Application Lifecycle
//!
//! A Revue application follows this lifecycle:
//!
//! ```text
//! 1. INITIALIZATION (App::new, AppBuilder)
//!    ├─ Create DOM renderer with stylesheet
//!    ├─ Create layout engine
//!    ├─ Allocate double buffers
//!    ├─ Initialize plugin registry
//!    └─ Set up optional features (hot reload, devtools, etc.)
//!
//! 2. RUN LOOP (App::run)
//!    ├─ Initialize terminal
//!    ├─ Mount plugins
//!    ├─ Build initial DOM
//!    ├─ Enter event loop:
//!    │   ├─ SIGTERM/SIGHUP/SIGINT received (unix) → stop running
//!    │   ├─ Check hot reload (if enabled)
//!    │   ├─ Read next event
//!    │   ├─ Handle event → may trigger redraw
//!    │   └─ Draw frame (if needed)
//!    ├─ Unmount plugins
//!    └─ Restore terminal
//!
//! 3. EVENT HANDLING (handle_event)
//!    ├─ Quit keys (Ctrl+C, 'q') → stop running
//!    ├─ Resize events → update buffers, rebuild layout
//!    ├─ Tick events → update transitions, tick plugins
//!    └─ User handler → custom application logic
//!
//! 4. DRAW CYCLE (draw)
//!    ├─ Update DOM (if needed)
//!    ├─ Compute styles (always, with dirty checking)
//!    ├─ Update layout (if needed)
//!    ├─ Collect dirty regions
//!    ├─ Render to new buffer
//!    ├─ Diff buffers
//!    └─ Draw changes to terminal
//!
//! 5. CLEANUP
//!    ├─ Unmount plugins
//!    └─ Restore terminal state
//! ```
//!
//! # State Flags
//!
//! The `App` uses several boolean flags to track what needs to be updated:
//!
//! | Flag | Purpose | When Set |
//! |------|---------|----------|
//! | `running` | Controls the event loop | Set to `true` on run start, `false` on quit |
//! | `needs_force_redraw` | Full screen redraw | On resize, explicit request, or stylesheet reload |
//! | `needs_layout_rebuild` | Rebuild layout tree | On resize or structural DOM changes |
//! | `needs_dom_rebuild` | Rebuild DOM root | On first frame or explicit request |
//!
//! # Buffer Management
//!
//! Revue uses **double buffering** for efficient rendering:
//!
//! 1. Two buffers are allocated at the terminal size
//! 2. Each frame renders to the "new" buffer
//! 3. Buffers are diffed to find minimal changes
//! 4. Only changed cells are drawn to the terminal
//! 5. Buffers are swapped for the next frame
//!
//! # Threading Model
//!
//! The `App` is **single-threaded** by design:
//! - All UI updates happen on the main thread
//! - Event handling is synchronous
//! - Plugin operations run in sequence
//! - For async operations, use the worker pool module
//!
//! # Plugins
//!
//! Plugins can extend application functionality:
//! - Access terminal size via `update_terminal_size`
//! - Receive tick events via `tick`
//! - Lifecycle hooks: `mount`, `unmount`
//!
//! # Hot Reload
//!
//! With the `hot-reload` feature enabled and hot reload turned on
//! ([`AppBuilder::hot_reload`] or the `REVUE_HOT_RELOAD` environment variable):
//! - the directories of the files added with [`AppBuilder::style`] are watched
//! - on a change the stylesheet is rebuilt from its sources, so edited,
//!   added and deleted declarations all take effect
//! - invalid CSS logs a warning and keeps the last version that parsed

mod builder;
pub mod declarative_router;
mod draw;
mod event_loop;
#[cfg(feature = "hot-reload")]
mod hot_reload;
mod inspector;
pub mod profiler;
pub mod router;
pub mod screen;
mod signals;
pub mod snapshot;
#[cfg(feature = "hot-reload")]
mod style_sources;

pub use builder::AppBuilder;
pub use declarative_router::{
    declarative_router, is_active, link, use_param, use_params, use_path, use_route,
    DeclarativeRouter, Link, ReactiveRouteState, RouteContext, RouteRenderer,
};
#[cfg(feature = "hot-reload")]
pub use hot_reload::{hot_reload, HotReload, HotReloadBuilder, HotReloadConfig, HotReloadEvent};
pub use inspector::{inspector, Inspector, WidgetInfo};
pub use profiler::{
    fps_counter, profiler as new_profiler, FpsCounter, Metric, MetricType, Profiler, Sample, Stats,
};
pub use router::{
    router, routes, HistoryEntry, NavigationEvent, QueryParams, Route, RouteBuilder, RouteParams,
    Router,
};
pub use screen::{
    screen_manager, simple_screen, Screen, ScreenConfig, ScreenEvent, ScreenId, ScreenManager,
    ScreenMode, ScreenResult, SimpleScreen, Transition,
};
pub use snapshot::{snapshot, Snapshot, SnapshotConfig, SnapshotResult};

use crate::dom::DomRenderer;
use crate::event::{Key, KeyEvent};
use crate::layout::LayoutEngine;
use crate::render::Buffer;
use crate::style::{StyleSheet, TransitionManager};
use std::time::{Duration, Instant};

#[cfg(feature = "hot-reload")]
use style_sources::StyleSources;

/// Tick handler callback type
pub type TickHandler<V> = Box<dyn FnMut(&mut V, Duration) -> bool>;

/// Check if key is a quit key (Ctrl+C only)
#[inline]
fn is_quit_key(key: &KeyEvent) -> bool {
    key.is_ctrl_c()
}

/// Is this the key that moves focus?
///
/// Terminals disagree about Shift+Tab: most send `BackTab` as its own key,
/// some send `Tab` with the shift flag. Both count.
#[inline]
fn is_tab_key(key: &KeyEvent) -> bool {
    !key.ctrl && !key.alt && matches!(key.key, Key::Tab | Key::BackTab)
}

/// Main application struct
///
/// The `App` struct manages the entire application lifecycle including:
/// - DOM tree and style resolution
/// - Layout computation
/// - Double-buffered rendering
/// - Event loop and handling
/// - Plugin management
/// - Transition animations
///
/// # Creating an App
///
/// Use [`AppBuilder`] for configuration:
///
/// ```ignore
/// use revue::prelude::*;
///
/// let app = App::builder()
///     .stylesheet(StyleSheet::default())
///     .mouse_capture(true)
///     .build()
///     .unwrap();
/// ```
///
/// # Running the App
///
/// Use [`App::run()`] to start the event loop with a view and event handler:
///
/// ```ignore
/// app.run(my_view, |event, view, app| {
///     // Handle events, return true to trigger redraw
///     true
/// })?;
/// ```
///
/// # Requesting Updates
///
/// Use these methods to trigger updates:
/// - [`request_redraw()`][Self::request_redraw] - Force full screen redraw on next frame
/// - [`request_layout_rebuild()`][Self::request_layout_rebuild] - Rebuild layout tree on next frame
/// - [`request_dom_rebuild()`][Self::request_dom_rebuild] - Rebuild DOM root on next frame
pub struct App {
    /// Manages all DOM nodes and style resolution
    dom: DomRenderer,
    /// Manages layout computation
    layout: LayoutEngine,
    /// Double buffers for efficient diffing
    buffers: [Buffer; 2],
    /// Current buffer index (0 or 1)
    current_buffer: usize,

    /// Running state
    running: bool,
    /// Transition manager for animations
    transitions: TransitionManager,
    /// Last tick time for delta calculation
    last_tick: Instant,
    /// Whether to capture mouse events
    pub(crate) mouse_capture: bool,
    /// Request full screen redraw (clears diff cache)
    needs_force_redraw: bool,
    /// Track if layout tree needs full rebuild
    needs_layout_rebuild: bool,
    /// Track if DOM tree needs rebuild (root node creation)
    needs_dom_rebuild: bool,
    /// Reconcile the DOM against the view on every frame (opt-in, see
    /// [`AppBuilder::incremental_dom`](crate::core::app::AppBuilder::incremental_dom))
    incremental_dom: bool,
    /// Tab moves `:focus` between focusable nodes (see
    /// [`AppBuilder::tab_navigation`](crate::core::app::AppBuilder::tab_navigation))
    tab_navigation: bool,
    /// Run [`LayoutEngine`] every frame. Off unless something will read its
    /// output - and nothing in the render path does: containers compute their
    /// own geometry. See `docs/refactor/findings-layout.md`.
    layout_engine: bool,
    /// Plugin registry
    plugins: crate::plugin::PluginRegistry,
    /// Whether devtools are enabled for this app instance
    devtools_enabled: bool,
    /// Hot reload watcher
    #[cfg(feature = "hot-reload")]
    hot_reload: Option<HotReload>,
    /// What the stylesheet is built from, for hot reload
    #[cfg(feature = "hot-reload")]
    style_sources: StyleSources,
}

impl App {
    /// Create a new application with plugins.
    #[allow(dead_code)] // Used conditionally based on features
    pub(crate) fn new_with_plugins(
        initial_size: (u16, u16),
        stylesheet: StyleSheet,
        mouse_capture: bool,
        plugins: crate::plugin::PluginRegistry,
        devtools_enabled: bool,
    ) -> Self {
        let (width, height) = initial_size;
        Self {
            dom: DomRenderer::with_stylesheet(stylesheet),
            layout: LayoutEngine::new(),
            buffers: [Buffer::new(width, height), Buffer::new(width, height)],
            current_buffer: 0,
            running: false,
            transitions: TransitionManager::new(),
            last_tick: Instant::now(),
            mouse_capture,
            needs_force_redraw: true, // Initial render should be a full draw
            needs_layout_rebuild: true, // Initial render needs full layout build
            needs_dom_rebuild: true,  // Initial render needs DOM root creation
            incremental_dom: false,   // Opt-in until the benches say otherwise
            tab_navigation: false,    // Opt-in: Tab may already be the app's key
            layout_engine: false,     // Nothing reads it yet
            plugins,
            devtools_enabled,
            #[cfg(feature = "hot-reload")]
            hot_reload: None,
            #[cfg(feature = "hot-reload")]
            style_sources: StyleSources::default(),
        }
    }

    /// Create a new application with hot reload support.
    #[cfg(feature = "hot-reload")]
    pub(crate) fn new_with_hot_reload(
        initial_size: (u16, u16),
        stylesheet: StyleSheet,
        mouse_capture: bool,
        plugins: crate::plugin::PluginRegistry,
        devtools_enabled: bool,
        hot_reload: Option<HotReload>,
        style_sources: StyleSources,
    ) -> Self {
        let (width, height) = initial_size;
        Self {
            dom: DomRenderer::with_stylesheet(stylesheet),
            layout: LayoutEngine::new(),
            buffers: [Buffer::new(width, height), Buffer::new(width, height)],
            current_buffer: 0,
            running: false,
            transitions: TransitionManager::new(),
            last_tick: Instant::now(),
            mouse_capture,
            needs_force_redraw: true,
            needs_layout_rebuild: true,
            needs_dom_rebuild: true,
            incremental_dom: false,
            tab_navigation: false,
            layout_engine: false,
            plugins,
            devtools_enabled,
            hot_reload,
            style_sources,
        }
    }

    /// Enable per-frame DOM reconciliation.
    pub(crate) fn set_incremental_dom(&mut self, enabled: bool) {
        self.incremental_dom = enabled;
    }

    /// Let Tab and Shift+Tab move focus.
    pub(crate) fn set_tab_navigation(&mut self, enabled: bool) {
        self.tab_navigation = enabled;
    }

    /// Compute [`LayoutEngine`] output every frame.
    ///
    /// Crate-internal: the only reader is `testing::PipelineHarness`.
    pub(crate) fn set_layout_engine(&mut self, enabled: bool) {
        self.layout_engine = enabled;
    }

    /// Build the DOM from the render traversal instead of `View::children`.
    pub(crate) fn set_dom_from_render(&mut self, enabled: bool) {
        self.dom.set_dom_from_render(enabled);
    }

    /// Is the DOM built from the render traversal?
    pub fn dom_from_render(&self) -> bool {
        self.dom.dom_from_render()
    }

    /// Let CSS box properties override the geometry a container computed.
    pub(crate) fn set_css_layout(&mut self, enabled: bool) {
        self.dom.set_css_layout(enabled);
    }

    /// Do CSS box properties override container-computed geometry?
    pub fn css_layout(&self) -> bool {
        self.dom.css_layout()
    }

    /// Is per-frame DOM reconciliation enabled?
    pub fn incremental_dom(&self) -> bool {
        self.incremental_dom
    }

    /// Create a new application builder
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    /// Get access to the plugin registry
    pub fn plugins(&self) -> &crate::plugin::PluginRegistry {
        &self.plugins
    }

    /// Get mutable access to the plugin registry
    pub fn plugins_mut(&mut self) -> &mut crate::plugin::PluginRegistry {
        &mut self.plugins
    }

    /// Stop the application event loop
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Buffer that was most recently presented to the terminal.
    ///
    /// Crate-internal: used by `testing::PipelineHarness` to assert on the
    /// output of the real draw pipeline.
    pub(crate) fn presented_buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer]
    }

    /// Read-only access to the DOM built by the last draw.
    pub(crate) fn dom(&self) -> &DomRenderer {
        &self.dom
    }

    /// Computed layout rect for a node, as `LayoutEngine` produced it.
    ///
    /// Crate-internal: used by `testing::PipelineHarness`. Nothing in the
    /// render path reads this yet - see `docs/refactor/findings-layout.md`.
    /// Always `None` unless [`set_layout_engine`](Self::set_layout_engine)
    /// turned the engine on.
    pub(crate) fn layout_rect(&self, dom_id: crate::dom::DomId) -> Option<crate::layout::Rect> {
        self.layout.try_layout(dom_id)
    }

    /// Children of a node in the layout tree, which must mirror the DOM tree.
    pub(crate) fn layout_children(&self, dom_id: crate::dom::DomId) -> Vec<crate::dom::DomId> {
        self.layout.children(dom_id)
    }

    /// Request a full screen redraw on the next frame
    pub fn request_redraw(&mut self) {
        self.needs_force_redraw = true;
    }

    /// Request a full layout rebuild on next draw
    pub fn request_layout_rebuild(&mut self) {
        self.needs_layout_rebuild = true;
    }

    /// Request a full DOM rebuild on next draw
    /// This should rarely be needed - the framework handles this automatically
    pub fn request_dom_rebuild(&mut self) {
        self.needs_dom_rebuild = true;
        self.needs_layout_rebuild = true; // DOM rebuild implies layout rebuild
    }

    /// Check if the application is still running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Check if devtools are enabled for this app instance
    pub fn is_devtools_enabled(&self) -> bool {
        self.devtools_enabled
    }

    /// Enable devtools for this app instance
    pub fn enable_devtools(&mut self) {
        self.devtools_enabled = true;
    }

    /// Disable devtools for this app instance
    pub fn disable_devtools(&mut self) {
        self.devtools_enabled = false;
    }

    /// Toggle devtools for this app instance
    pub fn toggle_devtools(&mut self) -> bool {
        self.devtools_enabled = !self.devtools_enabled;
        self.devtools_enabled
    }

    /// Get mutable access to the DOM renderer
    pub fn dom_renderer(&mut self) -> &mut DomRenderer {
        &mut self.dom
    }

    /// Get immutable access to the transition manager
    pub fn transitions(&self) -> &TransitionManager {
        &self.transitions
    }

    /// Get mutable access to the transition manager
    pub fn transitions_mut(&mut self) -> &mut TransitionManager {
        &mut self.transitions
    }

    /// Start a transition animation for a property
    pub fn start_transition(
        &mut self,
        property: &str,
        from: f32,
        to: f32,
        transition: &crate::style::Transition,
    ) {
        self.transitions.start(property, from, to, transition);
    }

    /// Get the current value of a transitioning property
    pub fn transition_value(&self, property: &str) -> Option<f32> {
        self.transitions.get(property)
    }

    /// Check if there are any active transitions
    pub fn has_active_transitions(&self) -> bool {
        self.transitions.has_active()
    }
}

impl Default for App {
    fn default() -> Self {
        App::builder().build()
    }
}
// KEEP HERE - Private implementation tests (accesses private fields)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, Key};
    use crate::render::Terminal;
    use crate::widget::View;

    struct TestView;
    impl View for TestView {
        fn render(&self, _ctx: &mut crate::widget::RenderContext) {}
        fn meta(&self) -> crate::dom::WidgetMeta {
            crate::dom::WidgetMeta::new("TestView")
        }
    }

    fn create_test_app() -> App {
        App::new_with_plugins(
            (80, 24),
            StyleSheet::new(),
            false,
            crate::plugin::PluginRegistry::new(),
            false, // devtools_enabled
        )
    }

    /// Draw `view` twice - enough for both DOM paths to have a tree.
    fn draw_twice(app: &mut App, view: &crate::widget::Text) {
        let (w, h) = app.get_buffer_size();
        let mut terminal = Terminal::with_size(Vec::new(), w, h);
        for _ in 0..2 {
            app.draw(view, &mut terminal, true).unwrap();
        }
    }

    /// Nothing in the render path reads `LayoutEngine`'s output, so an app that
    /// did not ask for it must not pay for computing it.
    #[test]
    fn test_draw_skips_layout_nobody_reads() {
        for dom_from_render in [false, true] {
            let mut app = create_test_app();
            app.set_dom_from_render(dom_from_render);
            draw_twice(&mut app, &crate::widget::Text::new("x"));

            let root = app.dom.tree().root_id().unwrap();
            assert_eq!(
                app.layout_rect(root),
                None,
                "dom_from_render={dom_from_render}: computed a layout nothing reads"
            );
        }
    }

    #[test]
    fn test_draw_computes_layout_when_asked() {
        for dom_from_render in [false, true] {
            let mut app = create_test_app();
            app.set_dom_from_render(dom_from_render);
            app.set_layout_engine(true);
            draw_twice(&mut app, &crate::widget::Text::new("x"));

            let root = app.dom.tree().root_id().unwrap();
            assert!(
                app.layout_rect(root).is_some(),
                "dom_from_render={dom_from_render}: asked for layout, got none"
            );
        }
    }

    #[test]
    fn test_app_builder_and_new() {
        let app = App::builder().css(".test { color: red; }").build();
        assert!(!app.is_running());
    }

    #[test]
    fn test_app_default() {
        let app = App::default();
        assert!(!app.is_running());
    }

    #[test]
    fn test_app_quit() {
        let mut app = create_test_app();
        app.running = true;
        assert!(app.is_running());
        app.quit();
        assert!(!app.is_running());
    }

    #[test]
    fn test_is_quit_key() {
        let q_key = KeyEvent::new(Key::Char('q'));
        let ctrl_c = KeyEvent::ctrl(Key::Char('c'));
        let other_key = KeyEvent::new(Key::Char('a'));
        assert!(!is_quit_key(&q_key)); // 'q' alone is not a quit key
        assert!(is_quit_key(&ctrl_c));
        assert!(!is_quit_key(&other_key));
    }

    #[test]
    fn test_is_quit_key_other_keys() {
        let escape = KeyEvent::new(Key::Escape);
        let enter = KeyEvent::new(Key::Enter);
        let ctrl_d = KeyEvent::ctrl(Key::Char('d'));
        assert!(!is_quit_key(&escape));
        assert!(!is_quit_key(&enter));
        assert!(!is_quit_key(&ctrl_d));
    }

    #[test]
    fn test_request_redraw() {
        let mut app = create_test_app();
        app.needs_force_redraw = false;
        app.request_redraw();
        assert!(app.needs_force_redraw);
    }

    #[test]
    fn test_request_layout_rebuild() {
        let mut app = create_test_app();
        app.needs_layout_rebuild = false;
        app.request_layout_rebuild();
        assert!(app.needs_layout_rebuild);
    }

    #[test]
    fn test_request_dom_rebuild() {
        let mut app = create_test_app();
        app.needs_dom_rebuild = false;
        app.needs_layout_rebuild = false;
        app.request_dom_rebuild();
        assert!(app.needs_dom_rebuild);
        assert!(app.needs_layout_rebuild); // DOM rebuild implies layout rebuild
    }

    #[test]
    fn test_plugins_access() {
        let mut app = create_test_app();
        let _ = app.plugins();
        let _ = app.plugins_mut();
    }

    #[test]
    fn test_dom_renderer_access() {
        let mut app = create_test_app();
        let _ = app.dom_renderer();
    }

    #[test]
    fn test_transitions_access() {
        let mut app = create_test_app();
        assert!(!app.has_active_transitions());
        let _ = app.transitions();
        let _ = app.transitions_mut();
    }

    #[test]
    fn test_transition_value_none() {
        let app = create_test_app();
        assert!(app.transition_value("opacity").is_none());
    }

    #[test]
    fn test_start_transition() {
        let mut app = create_test_app();
        let transition = crate::style::Transition {
            property: "opacity".to_string(),
            duration: Duration::from_millis(300),
            delay: Duration::ZERO,
            easing: crate::style::Easing::Linear,
        };
        app.start_transition("opacity", 0.0, 1.0, &transition);
        assert!(app.has_active_transitions());
        // Initial value should be close to 0 (start value)
        let value = app.transition_value("opacity");
        assert!(value.is_some());
    }

    #[test]
    fn test_new_with_plugins_initial_state() {
        let app = App::new_with_plugins(
            (100, 50),
            StyleSheet::new(),
            true,
            crate::plugin::PluginRegistry::new(),
            false, // devtools_enabled
        );
        assert!(!app.running);
        assert!(app.needs_force_redraw);
        assert!(app.needs_layout_rebuild);
        assert!(app.needs_dom_rebuild);
        assert!(app.mouse_capture);
        assert!(!app.devtools_enabled);
    }

    #[test]
    fn test_buffer_initialization() {
        let app = App::new_with_plugins(
            (120, 40),
            StyleSheet::new(),
            false,
            crate::plugin::PluginRegistry::new(),
            false, // devtools_enabled
        );
        assert_eq!(app.buffers[0].width(), 120);
        assert_eq!(app.buffers[0].height(), 40);
        assert_eq!(app.buffers[1].width(), 120);
        assert_eq!(app.buffers[1].height(), 40);
        assert_eq!(app.current_buffer, 0);
    }

    #[test]
    fn test_devtools_methods() {
        let mut app = create_test_app();
        assert!(!app.is_devtools_enabled());

        app.enable_devtools();
        assert!(app.is_devtools_enabled());

        app.disable_devtools();
        assert!(!app.is_devtools_enabled());

        let result = app.toggle_devtools();
        assert!(result);
        assert!(app.is_devtools_enabled());

        let result = app.toggle_devtools();
        assert!(!result);
        assert!(!app.is_devtools_enabled());
    }

    #[test]
    fn test_handle_event_quit_q() {
        // 'q' alone should NOT quit (only Ctrl+C quits)
        let mut app = create_test_app();
        app.running = true;
        let mut view = TestView;
        let mut handler = |_: &Event, _: &mut TestView, _: &mut App| false;

        let event = Event::Key(KeyEvent::new(Key::Char('q')));
        let _ = app.handle_event(event, &mut view, &mut handler);
        assert!(app.is_running());
    }

    #[test]
    fn test_handle_event_quit_ctrl_c() {
        let mut app = create_test_app();
        app.running = true;
        let mut view = TestView;
        let mut handler = |_: &Event, _: &mut TestView, _: &mut App| false;

        let event = Event::Key(KeyEvent::ctrl(Key::Char('c')));
        let _ = app.handle_event(event, &mut view, &mut handler);
        assert!(!app.is_running());
    }

    #[test]
    fn test_handle_event_resize() {
        let mut app = create_test_app();
        app.needs_force_redraw = false;
        app.needs_layout_rebuild = false;
        let mut view = TestView;
        let mut handler = |_: &Event, _: &mut TestView, _: &mut App| false;

        let event = Event::Resize(100, 50);
        let should_draw = app.handle_event(event, &mut view, &mut handler);

        assert!(should_draw);
        assert!(app.needs_force_redraw);
        assert!(app.needs_layout_rebuild);
        assert_eq!(app.buffers[0].width(), 100);
        assert_eq!(app.buffers[0].height(), 50);
    }

    #[test]
    fn test_handle_event_tick() {
        let mut app = create_test_app();
        let mut view = TestView;
        let mut handler = |_: &Event, _: &mut TestView, _: &mut App| false;

        let event = Event::Tick;
        let _ = app.handle_event(event, &mut view, &mut handler);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_handle_event_handler_returns_true() {
        let mut app = create_test_app();
        app.needs_force_redraw = false;
        let mut view = TestView;
        let mut handler = |_: &Event, _: &mut TestView, _: &mut App| true;

        let event = Event::Key(KeyEvent::new(Key::Char('a')));
        let should_draw = app.handle_event(event, &mut view, &mut handler);
        assert!(should_draw);
    }

    #[test]
    fn test_handle_event_handler_returns_false() {
        let mut app = create_test_app();
        app.needs_force_redraw = false;
        let mut view = TestView;
        let mut handler = |_: &Event, _: &mut TestView, _: &mut App| false;

        let event = Event::Key(KeyEvent::new(Key::Char('a')));
        let should_draw = app.handle_event(event, &mut view, &mut handler);
        assert!(!should_draw);
    }
}
