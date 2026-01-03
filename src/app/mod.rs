//! Application lifecycle and coordination
//!
//! This module provides the main entry point for Revue applications.

mod builder;
#[cfg(feature = "hot-reload")]
mod hot_reload;
mod inspector;
pub mod profiler;
pub mod router;
pub mod screen;
pub mod snapshot;

pub use builder::AppBuilder;
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

use crate::constants::FRAME_DURATION_60FPS;
use crate::dom::DomRenderer;
use crate::event::{Event, KeyEvent};
use crate::layout::{LayoutEngine, Rect};
use crate::render::{Buffer, Terminal};
use crate::style::{StyleSheet, TransitionManager};
use crate::widget::View;
use std::io::stdout;
use std::time::{Duration, Instant};

/// Tick handler callback type
pub type TickHandler<V> = Box<dyn FnMut(&mut V, Duration) -> bool>;

/// Check if key is a quit key (Ctrl+C or 'q')
#[inline]
fn is_quit_key(key: &KeyEvent) -> bool {
    key.is_ctrl_c() || key.key == crate::event::Key::Char('q')
}

/// Main application struct
pub struct App {
    /// Manages all DOM nodes and style resolution
    dom: DomRenderer,
    /// Manages layout (taffy)
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
    mouse_capture: bool,
    /// Request full screen redraw (clears diff cache)
    needs_force_redraw: bool,
    /// Track if layout tree needs full rebuild
    needs_layout_rebuild: bool,
    /// Track if DOM tree needs rebuild (root node creation)
    needs_dom_rebuild: bool,
    /// Plugin registry
    plugins: crate::plugin::PluginRegistry,
}

impl App {
    /// Create a new application with plugins.
    pub(crate) fn new_with_plugins(
        initial_size: (u16, u16),
        stylesheet: StyleSheet,
        mouse_capture: bool,
        plugins: crate::plugin::PluginRegistry,
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
            plugins,
        }
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

    /// Run the application with a root view and event handler
    pub fn run<V, H>(&mut self, mut view: V, mut handler: H) -> crate::Result<()>
    where
        V: View,
        H: FnMut(&Event, &mut V, &mut Self) -> bool,
    {
        use crate::event::EventReader;

        let mut terminal = Terminal::new(stdout())?;
        terminal.init_with_mouse(self.mouse_capture)?;

        // Update plugin context with terminal size
        let (width, height) = terminal.size();
        self.plugins.update_terminal_size(width, height);

        // Mount plugins
        if let Err(e) = self.plugins.mount() {
            crate::log_warn!("Plugin mount failed: {}", e);
        }

        self.running = true;
        self.last_tick = Instant::now();

        self.dom.build(&view);
        self.draw(&view, &mut terminal, true)?;

        let reader = EventReader::new(FRAME_DURATION_60FPS);

        while self.running {
            let event = reader.read()?;
            let should_draw = self.handle_event(event, &mut view, &mut handler);

            if should_draw {
                self.draw(&view, &mut terminal, false)?;
            }
        }

        // Unmount plugins before exit
        if let Err(e) = self.plugins.unmount() {
            crate::log_warn!("Plugin unmount failed: {}", e);
        }

        terminal.restore()?;
        Ok(())
    }

    /// Run the application with a simplified key event handler
    ///
    /// This is a convenience method that wraps `run` with a simpler handler signature
    /// that only receives `KeyEvent` instead of all `Event` types.
    ///
    /// # Arguments
    /// * `view` - The root view component
    /// * `handler` - A function that handles key events and returns whether to redraw
    ///
    /// # Example
    /// ```ignore
    /// app.run_with_handler(my_view, |key_event, view| {
    ///     view.handle_key(&key_event.key)
    /// })
    /// ```
    pub fn run_with_handler<V, H>(&mut self, view: V, mut handler: H) -> crate::Result<()>
    where
        V: View,
        H: FnMut(&KeyEvent, &mut V) -> bool,
    {
        self.run(view, move |event, view, _app| match event {
            Event::Key(key_event) => handler(key_event, view),
            _ => false,
        })
    }

    /// Handle a single event
    fn handle_event<V, H>(&mut self, event: Event, view: &mut V, handler: &mut H) -> bool
    where
        V: View,
        H: FnMut(&Event, &mut V, &mut Self) -> bool,
    {
        let mut should_draw = handler(&event, view, self);

        match event {
            Event::Key(key) if is_quit_key(&key) => {
                self.quit();
                return false;
            }
            Event::Resize(w, h) => {
                self.buffers[0].resize(w, h);
                self.buffers[1].resize(w, h);
                self.plugins.update_terminal_size(w, h);
                self.needs_force_redraw = true;
                self.needs_layout_rebuild = true; // Resize requires full layout rebuild
                should_draw = true;
            }
            Event::Tick => {
                let now = Instant::now();
                let delta = now.duration_since(self.last_tick);
                self.last_tick = now;
                // Update both legacy and node-aware transitions
                self.transitions.update(delta);
                self.transitions.update_nodes(delta);
                // Tick plugins
                if let Err(e) = self.plugins.tick(delta) {
                    crate::log_warn!("Plugin tick failed: {}", e);
                }
                if self.transitions.has_active() {
                    should_draw = true;
                }
            }
            _ => {}
        }

        should_draw || self.needs_force_redraw
    }

    /// Draw the UI to the terminal
    fn draw<V: View, W: std::io::Write>(
        &mut self,
        view: &V,
        terminal: &mut Terminal<W>,
        force_redraw: bool,
    ) -> crate::Result<()> {
        // Only rebuild DOM root if needed (first frame or explicit request)
        // This is a major optimization since build() clears the entire tree and style cache
        if self.needs_dom_rebuild {
            self.dom.build(view);
            self.needs_dom_rebuild = false;
            // DOM rebuild requires layout rebuild
            self.needs_layout_rebuild = true;
        }

        // Always compute styles (has internal dirty checking optimization)
        self.dom.compute_styles_with_inheritance();

        let root_dom_id = self
            .dom
            .tree()
            .root_id()
            .expect("Root DOM node must exist to draw.");

        // Only rebuild layout tree if needed (e.g., on resize or structural changes)
        // DOM build() now performs incremental updates (reuses nodes by ID/position)
        if self.needs_layout_rebuild {
            self.layout.clear();
            self.build_layout_tree(root_dom_id);
            self.needs_layout_rebuild = false;
        } else {
            // Incremental update: only update nodes that changed
            self.update_layout_tree(root_dom_id);
        }

        let (width, height) = terminal.size();
        let _ = self.layout.compute(root_dom_id, width, height);

        let dirty_dom_ids = self.dom.tree_mut().get_dirty_nodes();
        let mut dirty_rects = Vec::new();
        for dom_id in &dirty_dom_ids {
            if let Ok(rect) = self.layout.layout(*dom_id) {
                dirty_rects.push(rect);
            }
        }

        // Merge overlapping dirty rects to minimize update regions
        if !dirty_rects.is_empty() {
            dirty_rects = crate::layout::merge_rects(&dirty_rects);
        }

        // Only force full-screen redraw when explicitly requested or necessary
        // With proper dirty tracking, this should rarely trigger
        if dirty_rects.is_empty() {
            if force_redraw || self.needs_force_redraw {
                // Explicit full redraw request
                let full_screen_rect = Rect::new(0, 0, width, height);
                dirty_rects.push(full_screen_rect);
            } else if self.transitions.has_active() {
                // Active transitions need redraws - only redraw affected nodes
                // Collect rects for nodes with active transitions
                let transition_rects: Vec<Rect> = self
                    .transitions
                    .active_node_ids()
                    .filter_map(|element_id| {
                        // Look up DOM node by element ID and get its layout rect
                        self.dom
                            .get_by_id(element_id)
                            .map(|node| node.id)
                            .and_then(|dom_id| self.layout.layout(dom_id).ok())
                    })
                    .collect();

                if transition_rects.is_empty() {
                    // Fallback: if no node-aware transitions, use legacy behavior
                    // This handles global transitions that aren't tied to specific nodes
                    if self.transitions.active_properties().next().is_some() {
                        let full_screen_rect = Rect::new(0, 0, width, height);
                        dirty_rects.push(full_screen_rect);
                    }
                } else {
                    dirty_rects.extend(transition_rects);
                }
            }
            // Otherwise, if dirty_rects is empty, nothing changed - skip rendering
        }

        let new_buffer_idx = 1 - self.current_buffer;

        // Split buffers to get separate mutable and immutable references
        let (old_buffer, new_buffer) = if self.current_buffer == 0 {
            let (old, new) = self.buffers.split_at_mut(1);
            (&old[0], &mut new[0])
        } else {
            let (new, old) = self.buffers.split_at_mut(1);
            (&old[0], &mut new[0])
        };

        let area = Rect::new(0, 0, new_buffer.width(), new_buffer.height());

        new_buffer.clear();
        self.dom.render(view, new_buffer, area);

        if force_redraw || self.needs_force_redraw {
            terminal.force_redraw(new_buffer)?;
            self.needs_force_redraw = false;
        } else {
            let changes = crate::render::diff(old_buffer, new_buffer, &dirty_rects);
            terminal.draw_changes(changes, new_buffer)?;
        }

        self.current_buffer = new_buffer_idx;

        // Clear dirty flags after rendering
        self.dom.tree_mut().clear_dirty_flags();

        Ok(())
    }

    /// Recursively build the layout tree from the DOM tree
    fn build_layout_tree(&mut self, dom_id: crate::dom::DomId) {
        let children = if let Some(node) = self.dom.tree().get(dom_id) {
            node.children.clone()
        } else {
            return;
        };

        let style = self
            .dom
            .style_for_with_inheritance(dom_id)
            .expect("Style should exist");
        let _ = self
            .layout
            .create_node_with_children(dom_id, &style, &children);

        for child_dom_id in children {
            self.build_layout_tree(child_dom_id);
        }
    }

    /// Incrementally update layout tree (only update changed nodes)
    ///
    /// Works with the incremental DOM build to only update dirty nodes.
    fn update_layout_tree(&mut self, dom_id: crate::dom::DomId) {
        // Check if this node exists in layout
        let node_exists = self.layout.layout(dom_id).is_ok();

        if !node_exists {
            // Node doesn't exist, need full rebuild
            self.needs_layout_rebuild = true;
            return;
        }

        // Get node state to check if dirty
        let is_dirty = self
            .dom
            .tree()
            .get(dom_id)
            .map(|n| n.state.dirty)
            .unwrap_or(false);

        // Only update style if node is dirty
        if is_dirty {
            if let Some(style) = self.dom.style_for_with_inheritance(dom_id) {
                let _ = self.layout.update_style(dom_id, &style);
            }
        }

        // Recursively update children
        let children = self
            .dom
            .tree()
            .get(dom_id)
            .map(|n| n.children.clone())
            .unwrap_or_default();

        for child_id in children {
            self.update_layout_tree(child_id);
        }
    }

    /// Stop the application event loop
    pub fn quit(&mut self) {
        self.running = false;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Key;

    #[allow(dead_code)]
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
        )
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
        assert!(is_quit_key(&q_key));
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
        );
        assert!(!app.running);
        assert!(app.needs_force_redraw);
        assert!(app.needs_layout_rebuild);
        assert!(app.needs_dom_rebuild);
        assert!(app.mouse_capture);
    }

    #[test]
    fn test_buffer_initialization() {
        let app = App::new_with_plugins(
            (120, 40),
            StyleSheet::new(),
            false,
            crate::plugin::PluginRegistry::new(),
        );
        assert_eq!(app.buffers[0].width(), 120);
        assert_eq!(app.buffers[0].height(), 40);
        assert_eq!(app.buffers[1].width(), 120);
        assert_eq!(app.buffers[1].height(), 40);
        assert_eq!(app.current_buffer, 0);
    }

    #[test]
    fn test_handle_event_quit_q() {
        let mut app = create_test_app();
        app.running = true;
        let mut view = TestView;
        let mut handler = |_: &Event, _: &mut TestView, _: &mut App| false;

        let event = Event::Key(KeyEvent::new(Key::Char('q')));
        let _ = app.handle_event(event, &mut view, &mut handler);
        assert!(!app.is_running());
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
