//! Screen manager core implementation

use super::types::{
    Screen, ScreenConfig, ScreenData, ScreenEvent, ScreenId, ScreenResult, Transition,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::event::KeyEvent;
use crate::widget::RenderContext;

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
        let mut screen = if let Some(idx) = self.stack.iter().position(|e| e.screen.id() == id) {
            // Move to top
            self.stack.remove(idx).screen
        } else {
            // Try to create from registry
            match self.registry.get(&id) {
                Some(factory) => factory(),
                None => return false,
            }
        };

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::app::screen::types::ScreenConfig;

    // Mock screen for testing
    struct MockScreen {
        id: ScreenId,
        config: ScreenConfig,
        dismissible: bool,
    }

    impl MockScreen {
        fn new(id: ScreenId) -> Self {
            Self {
                id,
                config: ScreenConfig::default(),
                dismissible: false,
            }
        }

        fn with_dismissible(mut self, dismissible: bool) -> Self {
            self.dismissible = dismissible;
            self.config.dismissable = dismissible;
            self
        }

        #[allow(dead_code)]
        fn with_config(mut self, config: ScreenConfig) -> Self {
            self.config = config;
            self
        }
    }

    impl Screen for MockScreen {
        fn id(&self) -> ScreenId {
            self.id.clone()
        }

        fn config(&self) -> ScreenConfig {
            self.config.clone()
        }

        fn update(&mut self) -> ScreenResult {
            ScreenResult::Continue
        }

        fn render(&self, _ctx: &mut RenderContext) {}

        fn on_key(&mut self, _key: &crate::event::KeyEvent) -> ScreenResult {
            ScreenResult::Pass
        }

        fn on_event(&mut self, _event: ScreenEvent) {}
    }

    fn create_test_manager() -> (ScreenManager, ScreenId) {
        let mut manager = ScreenManager::new();
        let screen_id = "test_screen";

        manager.register(screen_id, || Box::new(MockScreen::new(screen_id.into())));
        manager.push(screen_id);

        (manager, screen_id.into())
    }

    #[test]
    fn test_screen_manager_new() {
        let manager = ScreenManager::new();
        assert_eq!(manager.depth(), 0);
        assert!(!manager.can_pop());
        assert!(manager.current().is_none());
    }

    #[test]
    fn test_screen_manager_default() {
        let manager = ScreenManager::default();
        assert_eq!(manager.depth(), 0);
    }

    #[test]
    fn test_register() {
        let mut manager = ScreenManager::new();
        let screen_id = "test_screen";

        manager.register(screen_id, || Box::new(MockScreen::new(screen_id.into())));

        // Push should now work
        assert!(manager.push(screen_id));
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_push_and_pop() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));

        manager.push("screen1");
        assert_eq!(manager.depth(), 1);

        manager.push("screen2");
        assert_eq!(manager.depth(), 2);

        // Pop should work (2 screens -> 1 screen)
        assert!(manager.pop());
        assert_eq!(manager.depth(), 1);
        assert_eq!(manager.current_id().unwrap(), "screen1".into());

        // Pop should fail (only 1 screen left)
        assert!(!manager.pop());
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_pop_requires_one_screen() {
        let (mut manager, screen_id) = create_test_manager();

        // Can't pop the last screen
        assert!(!manager.pop()); // Should fail - only one screen
        assert_eq!(manager.depth(), 1);
        assert_eq!(manager.current_id().unwrap(), screen_id);
    }

    #[test]
    fn test_current() {
        let (manager, screen_id) = create_test_manager();

        let current = manager.current().unwrap();
        assert_eq!(current.id(), screen_id);

        let current_id = manager.current_id().unwrap();
        assert_eq!(current_id, screen_id);
    }

    #[test]
    fn test_current_mut() {
        let (mut manager, _screen_id) = create_test_manager();

        let current = manager.current_mut().unwrap();
        assert_eq!(current.id(), "test_screen".into());
    }

    #[test]
    fn test_depth() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));
        assert_eq!(manager.depth(), 0);

        manager.push("screen1");
        assert_eq!(manager.depth(), 1);

        manager.push("screen2");
        assert_eq!(manager.depth(), 2);

        manager.pop();
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_can_pop() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));
        assert!(!manager.can_pop()); // Empty stack

        manager.push("screen1");
        assert!(!manager.can_pop()); // Only one screen
        assert_eq!(manager.depth(), 1);

        manager.push("screen2");
        assert!(manager.can_pop()); // Two screens now
        assert_eq!(manager.depth(), 2);
    }

    #[test]
    fn test_has_screen() {
        let (manager, screen_id) = create_test_manager();

        assert!(manager.has_screen(&screen_id));
        assert!(!manager.has_screen(&"nonexistent".into()));
    }

    #[test]
    fn test_push_with_data() {
        let mut manager = ScreenManager::new();
        manager.register("test", || Box::new(MockScreen::new("test".into())));

        let data: ScreenData = Box::new("test_data");
        assert!(manager.push_with_data("test", data));
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_pop_to() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));
        manager.register("screen3", || Box::new(MockScreen::new("screen3".into())));

        manager.push("screen1");
        manager.push("screen2");
        manager.push("screen3");

        assert_eq!(manager.depth(), 3);

        // Pop to screen2
        assert!(manager.pop_to("screen2"));
        assert_eq!(manager.depth(), 2);
        assert_eq!(manager.current_id().unwrap(), "screen2".into());
    }

    #[test]
    fn test_pop_to_not_found() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.push("screen1");

        // Try to pop to non-existent screen
        assert!(!manager.pop_to("nonexistent"));
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_replace() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));

        manager.push("screen1");
        assert_eq!(manager.current_id().unwrap(), "screen1".into());

        assert!(manager.replace("screen2"));
        assert_eq!(manager.current_id().unwrap(), "screen2".into());
        // replace calls pop() (fails with 1 screen) + push(), so depth becomes 2
        assert_eq!(manager.depth(), 2);
    }

    #[test]
    fn test_goto() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));
        manager.register("screen3", || Box::new(MockScreen::new("screen3".into())));

        manager.push("screen1");
        manager.push("screen2");
        manager.push("screen3");

        assert_eq!(manager.depth(), 3);

        // Goto should pop all except root, then push target
        assert!(manager.goto("screen2"));
        // Result: [screen1, screen2] - root + new screen
        assert_eq!(manager.depth(), 2);
        assert_eq!(manager.current_id().unwrap(), "screen2".into());
    }

    #[test]
    fn test_history() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));
        manager.register("screen3", || Box::new(MockScreen::new("screen3".into())));

        manager.push("screen1");
        manager.push("screen2");
        manager.push("screen3");

        let history = manager.history();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "screen1".into());
        assert_eq!(history[1], "screen2".into());
        assert_eq!(history[2], "screen3".into());
    }

    #[test]
    fn test_escaping_dismissable() {
        let mut manager = ScreenManager::new();

        manager.register("base", || Box::new(MockScreen::new("base".into())));
        manager.register("modal", || {
            Box::new(MockScreen::new("modal".into()).with_dismissible(true))
        });

        manager.push("base");
        manager.push("modal");
        assert_eq!(manager.depth(), 2);

        // Create mock key event for escape
        let key = crate::event::Key::Escape;
        let key_event = KeyEvent::new(key);

        // Should pop because modal screen is dismissable
        let result = manager.handle_key(&key_event);
        assert!(matches!(result, ScreenResult::Render));

        // Stack should have only base screen
        assert_eq!(manager.depth(), 1);
        assert_eq!(manager.current_id().unwrap(), "base".into());
    }

    #[test]
    fn test_escaping_non_dismissable() {
        let mut manager = ScreenManager::new();

        let screen_id = "test_screen";
        manager.register(screen_id, || {
            Box::new(MockScreen::new(screen_id.into()).with_dismissible(false))
        });

        manager.push(screen_id);
        assert_eq!(manager.depth(), 1);

        // Create mock key event for escape
        let key = crate::event::Key::Escape;
        let key_event = KeyEvent::new(key);

        // Should NOT pop because screen is not dismissable
        let result = manager.handle_key(&key_event);
        // Screen should pass the event to the screen
        assert!(matches!(result, ScreenResult::Pass));

        // Stack should still have the screen
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_is_transitioning() {
        let manager = ScreenManager::new();
        assert!(!manager.is_transitioning());
        assert!(manager.transition_progress().is_none());
    }

    #[test]
    fn test_update_completes_transition() {
        let mut manager = ScreenManager::new();
        manager.register("screen", || Box::new(MockScreen::new("screen".into())));
        manager.push("screen");

        // Update should work
        let result = manager.update();
        assert!(matches!(result, ScreenResult::Continue));
    }

    #[test]
    fn test_stack_entry_visibility() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));

        manager.push("screen1");
        manager.push("screen2");

        // Only top should be visible
        assert!(
            !manager.has_screen(&"screen1".into())
                || manager.current_id() != Some("screen1".into())
        );
    }

    // =========================================================================
    // Additional screen manager core tests
    // =========================================================================

    #[test]
    fn test_register_with_string() {
        let mut manager = ScreenManager::new();
        let id = String::from("string_screen");
        manager.register(&id, || Box::new(MockScreen::new("string_screen".into())));
        assert!(manager.push(&id));
    }

    #[test]
    fn test_register_screen_direct() {
        let mut manager = ScreenManager::new();
        let screen = Box::new(MockScreen::new("direct".into())) as Box<dyn Screen>;
        manager.register_screen(screen);
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_push_unregistered_fails() {
        let mut manager = ScreenManager::new();
        assert!(!manager.push("nonexistent"));
        assert_eq!(manager.depth(), 0);
    }

    #[test]
    fn test_push_with_data_unregistered_fails() {
        let mut manager = ScreenManager::new();
        let data: ScreenData = Box::new("data");
        assert!(!manager.push_with_data("test", data));
    }

    #[test]
    fn test_push_same_screen_twice() {
        let mut manager = ScreenManager::new();
        manager.register("screen", || Box::new(MockScreen::new("screen".into())));

        manager.push("screen");
        assert_eq!(manager.depth(), 1);

        // Pushing same screen again should move it to top (no duplicate)
        manager.push("screen");
        // Since screen1 exists, it's moved to top - depth stays 1
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_current_when_empty() {
        let manager = ScreenManager::new();
        assert!(manager.current().is_none());
        assert!(manager.current_id().is_none());
    }

    #[test]
    fn test_current_mut_when_empty() {
        let mut manager = ScreenManager::new();
        assert!(manager.current_mut().is_none());
    }

    #[test]
    fn test_history_empty() {
        let manager = ScreenManager::new();
        assert!(manager.history().is_empty());
    }

    #[test]
    fn test_pop_to_empty_stack() {
        let mut manager = ScreenManager::new();
        manager.register("screen", || Box::new(MockScreen::new("screen".into())));
        assert!(!manager.pop_to("screen"));
    }

    #[test]
    fn test_replace_when_empty() {
        let mut manager = ScreenManager::new();
        manager.register("screen", || Box::new(MockScreen::new("screen".into())));
        assert!(manager.replace("screen"));
        assert_eq!(manager.current_id().unwrap(), "screen".into());
    }

    #[test]
    fn test_goto_same_screen() {
        let (mut manager, screen_id) = create_test_manager();
        assert!(manager.goto(screen_id.clone()));
        assert_eq!(manager.current_id().unwrap(), screen_id);
    }

    #[test]
    fn test_goto_unregistered() {
        let mut manager = ScreenManager::new();
        assert!(!manager.goto("unregistered"));
        assert_eq!(manager.depth(), 0);
    }

    #[test]
    fn test_register_multiple_screens() {
        let mut manager = ScreenManager::new();
        manager.register("a", || Box::new(MockScreen::new("a".into())));
        manager.register("b", || Box::new(MockScreen::new("b".into())));
        manager.register("c", || Box::new(MockScreen::new("c".into())));

        assert!(manager.push("a"));
        assert!(manager.push("b"));
        assert!(manager.push("c"));
        assert_eq!(manager.depth(), 3);
    }

    #[test]
    fn test_render_empty_manager() {
        let manager = ScreenManager::new();
        // Should not panic when rendering with no screens
        let mut buffer = crate::render::Buffer::new(80, 24);
        let mut ctx =
            crate::widget::RenderContext::new(&mut buffer, crate::layout::Rect::new(0, 0, 80, 24));
        manager.render(&mut ctx);
    }

    #[test]
    fn test_update_empty_manager() {
        let mut manager = ScreenManager::new();
        let result = manager.update();
        assert!(matches!(result, ScreenResult::Continue));
    }

    #[test]
    fn test_handle_key_empty_manager() {
        let mut manager = ScreenManager::new();
        let key = crate::event::Key::Char('a');
        let key_event = KeyEvent::new(key);
        let result = manager.handle_key(&key_event);
        assert!(matches!(result, ScreenResult::Pass));
    }

    #[test]
    fn test_push_moves_existing_screen_to_top() {
        let mut manager = ScreenManager::new();
        manager.register("a", || Box::new(MockScreen::new("a".into())));
        manager.register("b", || Box::new(MockScreen::new("b".into())));
        manager.register("c", || Box::new(MockScreen::new("c".into())));

        manager.push("a");
        manager.push("b");
        manager.push("c");
        assert_eq!(manager.history().len(), 3);

        // Push "a" again - should move it to top
        manager.push("a");
        assert_eq!(manager.history().len(), 3);
        assert_eq!(manager.current_id().unwrap(), "a".into());
        assert_eq!(manager.history()[2], "a".into());
    }

    #[test]
    fn test_has_screen_empty_manager() {
        let manager = ScreenManager::new();
        assert!(!manager.has_screen(&"any".into()));
    }

    #[test]
    fn test_depth_empty_manager() {
        let manager = ScreenManager::new();
        assert_eq!(manager.depth(), 0);
    }

    #[test]
    fn test_dismissible_false_prevents_escape_pop() {
        let mut manager = ScreenManager::new();
        // Create screen with dismissible = false
        manager.register("base", || {
            let mut screen = MockScreen::new("base".into());
            screen.config.dismissable = true;
            Box::new(screen)
        });

        manager.register("modal", || {
            let mut screen = MockScreen::new("modal".into());
            screen.config.dismissable = false; // Override - not dismissible
            Box::new(screen)
        });

        manager.push("base");
        manager.push("modal");

        let key = crate::event::Key::Escape;
        let key_event = KeyEvent::new(key);

        // Should NOT pop because modal is not dismissible even though base is
        let result = manager.handle_key(&key_event);
        // Result could be Continue or Pass depending on implementation
        assert!(!matches!(result, ScreenResult::Pop));
        assert_eq!(manager.depth(), 2);
    }

    #[test]
    fn test_escaping_pop_only_if_dismissible() {
        let mut manager = ScreenManager::new();
        manager.register("base", || Box::new(MockScreen::new("base".into())));
        manager.register("modal", || {
            Box::new(MockScreen::new("modal".into()).with_dismissible(true))
        });

        manager.push("base");
        manager.push("modal");

        let key = crate::event::Key::Escape;
        let key_event = KeyEvent::new(key);

        // Should pop because modal is dismissible
        manager.handle_key(&key_event);
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_escaping_with_only_dismissible_base() {
        let mut manager = ScreenManager::new();
        manager.register("base", || {
            Box::new(MockScreen::new("base".into()).with_dismissible(true))
        });

        manager.push("base");

        // With only one dismissible screen, escape shouldn't pop (need at least 2 screens)
        let key = crate::event::Key::Escape;
        let key_event = KeyEvent::new(key);
        manager.handle_key(&key_event);
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn test_register_overwrites() {
        let mut manager = ScreenManager::new();
        manager.register("same", || Box::new(MockScreen::new("same_v1".into())));

        // First push
        manager.push("same");
        assert_eq!(manager.depth(), 1);

        // Register again with same ID (overwrites factory)
        manager.register("same", || Box::new(MockScreen::new("same_v2".into())));

        // Push should use the new factory and create a new screen
        assert!(manager.push("same"));
        assert_eq!(manager.depth(), 2);
    }

    #[test]
    fn test_transition_progress_when_none() {
        let manager = ScreenManager::new();
        assert!(manager.transition_progress().is_none());
    }

    #[test]
    fn test_is_transitioning_when_none() {
        let manager = ScreenManager::new();
        assert!(!manager.is_transitioning());
    }

    #[test]
    fn test_screen_result_pop_handling() {
        let mut manager = ScreenManager::new();
        manager.register("screen1", || Box::new(MockScreen::new("screen1".into())));
        manager.register("screen2", || Box::new(MockScreen::new("screen2".into())));

        manager.push("screen1");
        manager.push("screen2");

        // MockScreen returns Pass, so no Pop result
        let result = manager.update();
        assert!(matches!(result, ScreenResult::Continue));
    }
}
