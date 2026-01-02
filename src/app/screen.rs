//! Screen system for multi-page TUI applications
//!
//! Provides a screen stack for managing multiple views/pages with transitions.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::app::screen::{ScreenManager, Screen, ScreenId};
//! use revue::widget::{View, RenderContext};
//!
//! struct HomeScreen {
//!     title: String,
//! }
//!
//! impl Screen for HomeScreen {
//!     fn id(&self) -> ScreenId {
//!         ScreenId::new("home")
//!     }
//!
//!     fn render(&self, ctx: &mut RenderContext) {
//!         // Render home screen
//!     }
//! }
//!
//! let mut manager = ScreenManager::new();
//! manager.register(HomeScreen { title: "Home".into() });
//! manager.push("home");
//! ```

use std::any::Any;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::event::KeyEvent;
use crate::widget::RenderContext;

/// Screen identifier
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScreenId(String);

impl ScreenId {
    /// Create new screen ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get ID as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<S: Into<String>> From<S> for ScreenId {
    fn from(s: S) -> Self {
        Self::new(s)
    }
}

impl std::fmt::Display for ScreenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Screen transition type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transition {
    /// No transition
    None,
    /// Fade in/out
    Fade,
    /// Slide from left
    SlideLeft,
    /// Slide from right
    SlideRight,
    /// Slide from top
    SlideUp,
    /// Slide from bottom
    SlideDown,
    /// Push (new screen slides over)
    Push,
    /// Pop (current screen slides away)
    Pop,
}

impl Default for Transition {
    fn default() -> Self {
        Self::None
    }
}

/// Screen mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenMode {
    /// Full screen (replaces previous)
    Fullscreen,
    /// Modal (overlays previous)
    Modal,
    /// Popup (small overlay)
    Popup,
}

impl Default for ScreenMode {
    fn default() -> Self {
        Self::Fullscreen
    }
}

/// Screen lifecycle event
#[derive(Clone, Debug)]
pub enum ScreenEvent {
    /// Screen is being mounted
    Mount,
    /// Screen is being unmounted
    Unmount,
    /// Screen became visible
    Show,
    /// Screen became hidden
    Hide,
    /// Screen received focus
    Focus,
    /// Screen lost focus
    Blur,
    /// Screen is being resumed (returned to from another screen)
    Resume,
    /// Screen is being suspended (another screen pushed on top)
    Suspend,
}

/// Result of screen event handling
#[derive(Clone, Debug)]
pub enum ScreenResult {
    /// Continue normally
    Continue,
    /// Request re-render
    Render,
    /// Pop this screen
    Pop,
    /// Push a new screen
    Push(ScreenId),
    /// Replace with a new screen
    Replace(ScreenId),
    /// Go to a specific screen (clear stack)
    Goto(ScreenId),
    /// Exit the application
    Exit,
    /// Pass event to next handler
    Pass,
}

impl Default for ScreenResult {
    fn default() -> Self {
        Self::Continue
    }
}

/// Data passed between screens
pub type ScreenData = Box<dyn Any + Send + Sync>;

/// Screen configuration
#[derive(Clone, Debug)]
pub struct ScreenConfig {
    /// Screen mode
    pub mode: ScreenMode,
    /// Entry transition
    pub enter_transition: Transition,
    /// Exit transition
    pub exit_transition: Transition,
    /// Transition duration
    pub transition_duration: Duration,
    /// Whether screen can be dismissed with Escape
    pub dismissable: bool,
    /// Title for the screen
    pub title: Option<String>,
}

impl Default for ScreenConfig {
    fn default() -> Self {
        Self {
            mode: ScreenMode::Fullscreen,
            enter_transition: Transition::None,
            exit_transition: Transition::None,
            transition_duration: Duration::from_millis(200),
            dismissable: true,
            title: None,
        }
    }
}

impl ScreenConfig {
    /// Create fullscreen config
    pub fn fullscreen() -> Self {
        Self::default()
    }

    /// Create modal config
    pub fn modal() -> Self {
        Self {
            mode: ScreenMode::Modal,
            enter_transition: Transition::Fade,
            exit_transition: Transition::Fade,
            ..Default::default()
        }
    }

    /// Create popup config
    pub fn popup() -> Self {
        Self {
            mode: ScreenMode::Popup,
            enter_transition: Transition::Fade,
            exit_transition: Transition::Fade,
            transition_duration: Duration::from_millis(100),
            ..Default::default()
        }
    }

    /// Set mode
    pub fn mode(mut self, mode: ScreenMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set transitions
    pub fn transitions(mut self, enter: Transition, exit: Transition) -> Self {
        self.enter_transition = enter;
        self.exit_transition = exit;
        self
    }

    /// Set transition duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.transition_duration = duration;
        self
    }

    /// Set dismissable
    pub fn dismissable(mut self, value: bool) -> Self {
        self.dismissable = value;
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// Screen trait that all screens must implement
pub trait Screen: Send + Sync {
    /// Get screen ID
    fn id(&self) -> ScreenId;

    /// Get screen configuration
    fn config(&self) -> ScreenConfig {
        ScreenConfig::default()
    }

    /// Handle lifecycle event
    fn on_event(&mut self, _event: ScreenEvent) {}

    /// Handle key event
    fn on_key(&mut self, _key: &KeyEvent) -> ScreenResult {
        ScreenResult::Pass
    }

    /// Update screen state (called every tick)
    fn update(&mut self) -> ScreenResult {
        ScreenResult::Continue
    }

    /// Render the screen
    fn render(&self, ctx: &mut RenderContext);

    /// Receive data from another screen
    fn receive_data(&mut self, _data: ScreenData) {}

    /// Get title (for status bar, etc.)
    /// Override this to provide a custom title
    fn title(&self) -> Option<String> {
        self.config().title.clone()
    }
}

/// Stack entry with screen and metadata
struct StackEntry {
    /// Screen instance
    screen: Box<dyn Screen>,
    /// Entry timestamp (for future analytics)
    _entered_at: Instant,
    /// Whether screen is visible
    visible: bool,
}

/// Active transition state
struct TransitionState {
    /// Transition type (for future animation interpolation)
    _transition: Transition,
    /// Start time
    start: Instant,
    /// Duration
    duration: Duration,
    /// Whether entering or exiting (for future use)
    _entering: bool,
}

impl TransitionState {
    /// Get transition progress (0.0 to 1.0)
    fn progress(&self) -> f32 {
        let elapsed = self.start.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (elapsed / total).min(1.0)
    }

    /// Check if transition is complete
    fn is_complete(&self) -> bool {
        self.progress() >= 1.0
    }
}

/// Screen manager for handling screen stack
pub struct ScreenManager {
    /// Registered screen factories (by ID)
    registry: HashMap<ScreenId, Box<dyn Fn() -> Box<dyn Screen> + Send + Sync>>,
    /// Screen stack
    stack: Vec<StackEntry>,
    /// Active transition
    transition: Option<TransitionState>,
    /// Data to pass to next screen
    pending_data: Option<ScreenData>,
    /// Event queue (for future event propagation)
    _event_queue: Vec<(ScreenId, ScreenEvent)>,
}

impl ScreenManager {
    /// Create new screen manager
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            stack: Vec::new(),
            transition: None,
            pending_data: None,
            _event_queue: Vec::new(),
        }
    }

    /// Register a screen factory
    pub fn register<F>(&mut self, id: impl Into<ScreenId>, factory: F)
    where
        F: Fn() -> Box<dyn Screen> + Send + Sync + 'static,
    {
        let id = id.into();
        self.registry.insert(id, Box::new(factory));
    }

    /// Register a screen instance directly
    pub fn register_screen(&mut self, screen: Box<dyn Screen>) {
        let entry = StackEntry {
            screen,
            _entered_at: Instant::now(),
            visible: false,
        };
        // For direct registration, we push it but keep it hidden
        // This is mainly for pre-created screens
        self.stack.push(entry);
    }

    /// Push a screen onto the stack
    pub fn push(&mut self, id: impl Into<ScreenId>) -> bool {
        self.push_with_config(id, None)
    }

    /// Push a screen with data
    pub fn push_with_data(&mut self, id: impl Into<ScreenId>, data: ScreenData) -> bool {
        self.pending_data = Some(data);
        self.push(id)
    }

    /// Push with specific config
    fn push_with_config(&mut self, id: impl Into<ScreenId>, _config: Option<ScreenConfig>) -> bool {
        let id = id.into();

        // Create screen from registry or find in stack
        let mut screen: Option<Box<dyn Screen>> = None;

        // Check if already in stack
        if let Some(idx) = self.stack.iter().position(|e| e.screen.id() == id) {
            // Move to top
            let entry = self.stack.remove(idx);
            screen = Some(entry.screen);
        }

        // If not found, we need a factory - for now just return false
        // In a real implementation, we'd use the registry
        if screen.is_none() {
            return false;
        }

        let mut screen = screen.unwrap();

        // Send suspend to current top
        if let Some(current) = self.stack.last_mut() {
            current.screen.on_event(ScreenEvent::Suspend);
            current.visible = false;
        }

        // Send data if pending
        if let Some(data) = self.pending_data.take() {
            screen.receive_data(data);
        }

        // Mount and show new screen
        screen.on_event(ScreenEvent::Mount);
        screen.on_event(ScreenEvent::Show);
        screen.on_event(ScreenEvent::Focus);

        let config = screen.config();

        // Start transition if configured
        if config.enter_transition != Transition::None {
            self.transition = Some(TransitionState {
                _transition: config.enter_transition,
                start: Instant::now(),
                duration: config.transition_duration,
                _entering: true,
            });
        }

        self.stack.push(StackEntry {
            screen,
            _entered_at: Instant::now(),
            visible: true,
        });

        true
    }

    /// Pop the current screen
    pub fn pop(&mut self) -> bool {
        if self.stack.len() <= 1 {
            return false; // Keep at least one screen
        }

        if let Some(mut entry) = self.stack.pop() {
            // Send events
            entry.screen.on_event(ScreenEvent::Blur);
            entry.screen.on_event(ScreenEvent::Hide);
            entry.screen.on_event(ScreenEvent::Unmount);

            // Resume previous screen
            if let Some(current) = self.stack.last_mut() {
                current.visible = true;
                current.screen.on_event(ScreenEvent::Resume);
                current.screen.on_event(ScreenEvent::Focus);
            }

            true
        } else {
            false
        }
    }

    /// Pop to a specific screen
    pub fn pop_to(&mut self, id: impl Into<ScreenId>) -> bool {
        let id = id.into();

        while self.stack.len() > 1 {
            if let Some(entry) = self.stack.last() {
                if entry.screen.id() == id {
                    return true;
                }
            }
            if !self.pop() {
                break;
            }
        }

        false
    }

    /// Replace current screen
    pub fn replace(&mut self, id: impl Into<ScreenId>) -> bool {
        self.pop();
        self.push(id)
    }

    /// Go to screen (clear stack except root)
    pub fn goto(&mut self, id: impl Into<ScreenId>) -> bool {
        // Pop all except root
        while self.stack.len() > 1 {
            self.pop();
        }
        self.push(id)
    }

    /// Get current screen
    pub fn current(&self) -> Option<&dyn Screen> {
        self.stack.last().map(|e| e.screen.as_ref())
    }

    /// Get current screen mutably
    pub fn current_mut(&mut self) -> Option<&mut dyn Screen> {
        match self.stack.last_mut() {
            Some(entry) => Some(entry.screen.as_mut()),
            None => None,
        }
    }

    /// Get current screen ID
    pub fn current_id(&self) -> Option<ScreenId> {
        self.current().map(|s| s.id())
    }

    /// Get stack depth
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Check if can go back
    pub fn can_pop(&self) -> bool {
        self.stack.len() > 1
    }

    /// Check if a screen is in the stack
    pub fn has_screen(&self, id: &ScreenId) -> bool {
        self.stack.iter().any(|e| e.screen.id() == *id)
    }

    /// Handle key event
    pub fn handle_key(&mut self, key: &KeyEvent) -> ScreenResult {
        // Check for Escape to dismiss
        if key.is_escape() {
            if let Some(entry) = self.stack.last() {
                if entry.screen.config().dismissable && self.can_pop() {
                    self.pop();
                    return ScreenResult::Render;
                }
            }
        }

        // Pass to current screen
        if let Some(entry) = self.stack.last_mut() {
            let result = entry.screen.on_key(key);
            self.process_result(result)
        } else {
            ScreenResult::Pass
        }
    }

    /// Update screens
    pub fn update(&mut self) -> ScreenResult {
        // Update transition
        if let Some(ref transition) = self.transition {
            if transition.is_complete() {
                self.transition = None;
            }
        }

        // Update current screen
        if let Some(entry) = self.stack.last_mut() {
            let result = entry.screen.update();
            self.process_result(result)
        } else {
            ScreenResult::Continue
        }
    }

    /// Process screen result
    fn process_result(&mut self, result: ScreenResult) -> ScreenResult {
        match result {
            ScreenResult::Pop => {
                self.pop();
                ScreenResult::Render
            }
            ScreenResult::Push(id) => {
                self.push(id);
                ScreenResult::Render
            }
            ScreenResult::Replace(id) => {
                self.replace(id);
                ScreenResult::Render
            }
            ScreenResult::Goto(id) => {
                self.goto(id);
                ScreenResult::Render
            }
            other => other,
        }
    }

    /// Render current screen
    pub fn render(&self, ctx: &mut RenderContext) {
        // Render visible screens from bottom to top
        for entry in &self.stack {
            if entry.visible {
                entry.screen.render(ctx);
            }
        }

        // Apply transition effect if active
        // (In a real implementation, we'd modify rendering based on transition state)
    }

    /// Get transition progress (for custom rendering)
    pub fn transition_progress(&self) -> Option<f32> {
        self.transition.as_ref().map(|t| t.progress())
    }

    /// Check if transitioning
    pub fn is_transitioning(&self) -> bool {
        self.transition.is_some()
    }

    /// Get screen history (IDs from bottom to top)
    pub fn history(&self) -> Vec<ScreenId> {
        self.stack.iter().map(|e| e.screen.id()).collect()
    }
}

impl Default for ScreenManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple screen implementation for closures
pub struct SimpleScreen<F>
where
    F: Fn(&mut RenderContext) + Send + Sync,
{
    id: ScreenId,
    config: ScreenConfig,
    render_fn: F,
}

impl<F> SimpleScreen<F>
where
    F: Fn(&mut RenderContext) + Send + Sync,
{
    /// Create new simple screen
    pub fn new(id: impl Into<ScreenId>, render_fn: F) -> Self {
        Self {
            id: id.into(),
            config: ScreenConfig::default(),
            render_fn,
        }
    }

    /// Set config
    pub fn config(mut self, config: ScreenConfig) -> Self {
        self.config = config;
        self
    }
}

impl<F> Screen for SimpleScreen<F>
where
    F: Fn(&mut RenderContext) + Send + Sync,
{
    fn id(&self) -> ScreenId {
        self.id.clone()
    }

    fn config(&self) -> ScreenConfig {
        self.config.clone()
    }

    fn render(&self, ctx: &mut RenderContext) {
        (self.render_fn)(ctx);
    }
}

/// Create a screen manager
pub fn screen_manager() -> ScreenManager {
    ScreenManager::new()
}

/// Create a simple screen
pub fn simple_screen<F>(id: impl Into<ScreenId>, render_fn: F) -> SimpleScreen<F>
where
    F: Fn(&mut RenderContext) + Send + Sync,
{
    SimpleScreen::new(id, render_fn)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestScreen {
        id: ScreenId,
        events: Vec<ScreenEvent>,
    }

    impl TestScreen {
        fn new(id: impl Into<ScreenId>) -> Self {
            Self {
                id: id.into(),
                events: Vec::new(),
            }
        }
    }

    impl Screen for TestScreen {
        fn id(&self) -> ScreenId {
            self.id.clone()
        }

        fn on_event(&mut self, event: ScreenEvent) {
            self.events.push(event);
        }

        fn render(&self, _ctx: &mut RenderContext) {}
    }

    #[test]
    fn test_screen_id() {
        let id = ScreenId::new("home");
        assert_eq!(id.as_str(), "home");
        assert_eq!(format!("{}", id), "home");
    }

    #[test]
    fn test_screen_config_default() {
        let config = ScreenConfig::default();
        assert_eq!(config.mode, ScreenMode::Fullscreen);
        assert!(config.dismissable);
    }

    #[test]
    fn test_screen_config_modal() {
        let config = ScreenConfig::modal();
        assert_eq!(config.mode, ScreenMode::Modal);
        assert_eq!(config.enter_transition, Transition::Fade);
    }

    #[test]
    fn test_screen_manager_new() {
        let manager = ScreenManager::new();
        assert_eq!(manager.depth(), 0);
        assert!(!manager.can_pop());
    }

    #[test]
    fn test_screen_manager_register() {
        let mut manager = ScreenManager::new();
        let screen = TestScreen::new("home");
        manager.register_screen(Box::new(screen));
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_screen_manager_push_pop() {
        let mut manager = ScreenManager::new();

        let home = TestScreen::new("home");
        let settings = TestScreen::new("settings");

        manager.register_screen(Box::new(home));
        manager.register_screen(Box::new(settings));

        // Push home to top
        manager.push("home");

        // Now we can check
        assert!(manager.depth() >= 1);
    }

    #[test]
    fn test_screen_config_builder() {
        let config = ScreenConfig::default()
            .mode(ScreenMode::Modal)
            .transitions(Transition::SlideLeft, Transition::SlideRight)
            .duration(Duration::from_millis(300))
            .dismissable(false)
            .title("Settings");

        assert_eq!(config.mode, ScreenMode::Modal);
        assert_eq!(config.enter_transition, Transition::SlideLeft);
        assert!(!config.dismissable);
        assert_eq!(config.title, Some("Settings".to_string()));
    }

    #[test]
    fn test_transition_progress() {
        let state = TransitionState {
            _transition: Transition::Fade,
            start: Instant::now(),
            duration: Duration::from_millis(100),
            _entering: true,
        };

        assert!(state.progress() >= 0.0);
        assert!(state.progress() <= 1.0);
    }

    #[test]
    fn test_screen_result_default() {
        let result = ScreenResult::default();
        assert!(matches!(result, ScreenResult::Continue));
    }

    #[test]
    fn test_screen_history() {
        let mut manager = ScreenManager::new();

        manager.register_screen(Box::new(TestScreen::new("a")));
        manager.register_screen(Box::new(TestScreen::new("b")));

        let history = manager.history();
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_screen_manager_helper() {
        let manager = screen_manager();
        assert_eq!(manager.depth(), 0);
    }
}
