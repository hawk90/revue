//! Screen stack for multi-screen navigation
//!
//! Provides a way to manage multiple screens with push/pop navigation,
//! similar to mobile app navigation patterns.

use crate::event::Key;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};
use std::collections::HashMap;

/// Screen identifier
pub type ScreenId = &'static str;

/// Type alias for screen render callback
type ScreenRenderer = Box<dyn Fn(&Screen, &mut RenderContext)>;

/// A screen in the stack
pub struct Screen {
    /// Screen identifier
    pub id: ScreenId,
    /// Screen title
    pub title: String,
    /// Whether the screen is modal (blocks input to screens below)
    pub modal: bool,
    /// Screen-specific data
    data: HashMap<String, String>,
}

impl Screen {
    /// Create a new screen
    pub fn new(id: ScreenId) -> Self {
        Self {
            id,
            title: id.to_string(),
            modal: false,
            data: HashMap::new(),
        }
    }

    /// Set screen title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set as modal
    pub fn modal(mut self) -> Self {
        self.modal = true;
        self
    }

    /// Set screen data
    pub fn data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// Get screen data
    pub fn get_data(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

/// Screen transition animation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScreenTransition {
    /// No animation
    #[default]
    None,
    /// Slide from right
    SlideRight,
    /// Slide from bottom
    SlideUp,
    /// Fade in
    Fade,
    /// Zoom in
    Zoom,
}

/// Screen stack manager
pub struct ScreenStack {
    /// Stack of screens
    screens: Vec<Screen>,
    /// Screen render callbacks
    renderers: HashMap<ScreenId, ScreenRenderer>,
    /// Transition animation
    transition: ScreenTransition,
    /// Transition progress (0.0 - 1.0)
    transition_progress: f32,
    /// Whether transitioning
    transitioning: bool,
    /// History for back navigation
    history: Vec<ScreenId>,
    /// Maximum history size
    max_history: usize,
    /// Widget properties
    props: WidgetProps,
}

impl ScreenStack {
    /// Create a new screen stack
    pub fn new() -> Self {
        Self {
            screens: Vec::new(),
            renderers: HashMap::new(),
            transition: ScreenTransition::None,
            transition_progress: 1.0,
            transitioning: false,
            history: Vec::new(),
            max_history: 50,
            props: WidgetProps::new(),
        }
    }

    /// Set transition animation
    pub fn transition(mut self, transition: ScreenTransition) -> Self {
        self.transition = transition;
        self
    }

    /// Register a screen renderer
    pub fn register<F>(mut self, id: ScreenId, renderer: F) -> Self
    where
        F: Fn(&Screen, &mut RenderContext) + 'static,
    {
        self.renderers.insert(id, Box::new(renderer));
        self
    }

    /// Push a new screen onto the stack
    pub fn push(&mut self, screen: Screen) {
        if let Some(current_id) = self.current().map(|s| s.id) {
            if self.history.len() >= self.max_history {
                self.history.remove(0);
            }
            self.history.push(current_id);
        }
        self.screens.push(screen);
        self.start_transition();
    }

    /// Pop the top screen
    pub fn pop(&mut self) -> Option<Screen> {
        if self.screens.len() > 1 {
            self.start_transition();
            self.screens.pop()
        } else {
            None
        }
    }

    /// Pop to a specific screen
    pub fn pop_to(&mut self, id: ScreenId) -> Vec<Screen> {
        let mut popped = Vec::new();
        while self.screens.len() > 1 {
            if let Some(current) = self.current() {
                if current.id == id {
                    break;
                }
            }
            if let Some(screen) = self.screens.pop() {
                popped.push(screen);
            }
        }
        if !popped.is_empty() {
            self.start_transition();
        }
        popped
    }

    /// Pop to root screen
    pub fn pop_to_root(&mut self) -> Vec<Screen> {
        let mut popped = Vec::new();
        while self.screens.len() > 1 {
            if let Some(screen) = self.screens.pop() {
                popped.push(screen);
            }
        }
        if !popped.is_empty() {
            self.start_transition();
        }
        popped
    }

    /// Replace current screen
    pub fn replace(&mut self, screen: Screen) {
        self.screens.pop();
        self.screens.push(screen);
        self.start_transition();
    }

    /// Get current screen
    pub fn current(&self) -> Option<&Screen> {
        self.screens.last()
    }

    /// Get current screen mutably
    pub fn current_mut(&mut self) -> Option<&mut Screen> {
        self.screens.last_mut()
    }

    /// Get screen by id
    pub fn get(&self, id: ScreenId) -> Option<&Screen> {
        self.screens.iter().find(|s| s.id == id)
    }

    /// Check if screen exists in stack
    pub fn contains(&self, id: ScreenId) -> bool {
        self.screens.iter().any(|s| s.id == id)
    }

    /// Get stack depth
    pub fn depth(&self) -> usize {
        self.screens.len()
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.screens.len() > 1
    }

    /// Go back in history
    pub fn go_back(&mut self) -> bool {
        if self.can_go_back() {
            self.pop();
            true
        } else {
            false
        }
    }

    /// Start transition animation
    fn start_transition(&mut self) {
        if self.transition != ScreenTransition::None {
            self.transitioning = true;
            self.transition_progress = 0.0;
        }
    }

    /// Update transition animation
    pub fn update_transition(&mut self, delta: f32) {
        if self.transitioning {
            self.transition_progress += delta * 4.0; // Complete in ~250ms
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.transitioning = false;
            }
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Escape if self.can_go_back() => {
                self.go_back();
                true
            }
            _ => false,
        }
    }
}

impl Default for ScreenStack {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ScreenStack {
    crate::impl_view_meta!("ScreenStack");

    fn render(&self, ctx: &mut RenderContext) {
        // Render visible screens (modal screens hide those below)
        let mut start_idx = 0;
        for (i, screen) in self.screens.iter().enumerate().rev() {
            if screen.modal {
                start_idx = i;
                break;
            }
        }

        for screen in &self.screens[start_idx..] {
            if let Some(renderer) = self.renderers.get(screen.id) {
                renderer(screen, ctx);
            }
        }
    }
}

impl_styled_view!(ScreenStack);
impl_props_builders!(ScreenStack);

/// Helper to create a screen
pub fn screen(id: ScreenId) -> Screen {
    Screen::new(id)
}

/// Helper to create a screen stack
pub fn screen_stack() -> ScreenStack {
    ScreenStack::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ScreenTransition enum tests
    // =========================================================================

    #[test]
    fn test_screen_transition_default() {
        assert_eq!(ScreenTransition::default(), ScreenTransition::None);
    }

    #[test]
    fn test_screen_transition_clone() {
        let t = ScreenTransition::Fade;
        assert_eq!(t, t.clone());
    }

    #[test]
    fn test_screen_transition_copy() {
        let t1 = ScreenTransition::SlideUp;
        let t2 = t1;
        assert_eq!(t1, ScreenTransition::SlideUp);
        assert_eq!(t2, ScreenTransition::SlideUp);
    }

    #[test]
    fn test_screen_transition_debug() {
        let debug_str = format!("{:?}", ScreenTransition::Zoom);
        assert!(debug_str.contains("Zoom"));
    }

    #[test]
    fn test_screen_transition_partial_eq() {
        assert_eq!(ScreenTransition::None, ScreenTransition::None);
        assert_eq!(ScreenTransition::SlideRight, ScreenTransition::SlideRight);
        assert_eq!(ScreenTransition::SlideUp, ScreenTransition::SlideUp);
        assert_eq!(ScreenTransition::Fade, ScreenTransition::Fade);
        assert_eq!(ScreenTransition::Zoom, ScreenTransition::Zoom);
        assert_ne!(ScreenTransition::None, ScreenTransition::Fade);
    }

    // =========================================================================
    // Screen tests
    // =========================================================================

    #[test]
    fn test_screen_new() {
        let s = Screen::new("home");
        assert_eq!(s.id, "home");
        assert_eq!(s.title, "home");
        assert!(!s.modal);
        assert!(s.data.is_empty());
    }

    #[test]
    fn test_screen_title() {
        let s = Screen::new("test").title("Custom Title");
        assert_eq!(s.title, "Custom Title");
    }

    #[test]
    fn test_screen_title_string() {
        let s = Screen::new("test").title(String::from("Owned Title"));
        assert_eq!(s.title, "Owned Title");
    }

    #[test]
    fn test_screen_modal() {
        let s = Screen::new("test").modal();
        assert!(s.modal);
    }

    #[test]
    fn test_screen_data() {
        let s = Screen::new("test")
            .data("key1", "value1")
            .data("key2", "value2");

        assert_eq!(s.get_data("key1"), Some(&"value1".to_string()));
        assert_eq!(s.get_data("key2"), Some(&"value2".to_string()));
        assert_eq!(s.get_data("missing"), None);
    }

    #[test]
    fn test_screen_get_data_none() {
        let s = Screen::new("test");
        assert_eq!(s.get_data("key"), None);
    }

    #[test]
    fn test_screen_builder() {
        let s = Screen::new("settings")
            .title("Settings")
            .modal()
            .data("key", "value");

        assert_eq!(s.title, "Settings");
        assert!(s.modal);
        assert_eq!(s.get_data("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_screen_builder_chain() {
        let s = Screen::new("test")
            .title("Test Screen")
            .data("a", "1")
            .data("b", "2")
            .modal();

        assert_eq!(s.title, "Test Screen");
        assert!(s.modal);
        assert_eq!(s.get_data("a"), Some(&"1".to_string()));
        assert_eq!(s.get_data("b"), Some(&"2".to_string()));
    }

    // =========================================================================
    // ScreenStack builder tests
    // =========================================================================

    #[test]
    fn test_screen_stack_new() {
        let stack = ScreenStack::new();
        assert_eq!(stack.depth(), 0);
        assert!(!stack.can_go_back());
    }

    #[test]
    fn test_screen_stack_transition() {
        let stack = ScreenStack::new().transition(ScreenTransition::Fade);
        // Can't directly check transition field, but verify it compiles
        let _ = stack;
    }

    #[test]
    fn test_screen_stack_register() {
        let stack = ScreenStack::new().register("test", |_, _| {});
        // Can't directly check renderers, but verify it compiles
        let _ = stack;
    }

    // =========================================================================
    // ScreenStack push/pop tests
    // =========================================================================

    #[test]
    fn test_screen_stack_push_pop() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current().unwrap().id, "home");

        stack.push(Screen::new("settings"));
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current().unwrap().id, "settings");

        let popped = stack.pop();
        assert_eq!(popped.unwrap().id, "settings");
        assert_eq!(stack.depth(), 1);
    }

    #[test]
    fn test_screen_stack_pop_to() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        stack.push(Screen::new("a"));
        stack.push(Screen::new("b"));
        stack.push(Screen::new("c"));

        let popped = stack.pop_to("a");
        assert_eq!(popped.len(), 2);
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current().unwrap().id, "a");
    }

    #[test]
    fn test_screen_stack_pop_to_not_found() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        stack.push(Screen::new("a"));
        stack.push(Screen::new("b"));

        // Pop to non-existent screen - should stop at root
        let popped = stack.pop_to("missing");
        assert_eq!(popped.len(), 2);
        assert_eq!(stack.depth(), 1);
    }

    #[test]
    fn test_screen_stack_pop_to_root() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        stack.push(Screen::new("a"));
        stack.push(Screen::new("b"));
        stack.push(Screen::new("c"));

        assert_eq!(stack.depth(), 4);

        let popped = stack.pop_to_root();
        assert_eq!(popped.len(), 3);
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current().unwrap().id, "home");
    }

    #[test]
    fn test_screen_stack_pop_to_root_single_screen() {
        let mut stack = ScreenStack::new();
        stack.push(Screen::new("home"));

        let popped = stack.pop_to_root();
        assert_eq!(popped.len(), 0);
        assert_eq!(stack.depth(), 1);
    }

    #[test]
    fn test_screen_stack_replace() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        stack.push(Screen::new("old"));

        stack.replace(Screen::new("new"));

        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current().unwrap().id, "new");
    }

    #[test]
    fn test_screen_stack_replace_empty() {
        let mut stack = ScreenStack::new();
        stack.replace(Screen::new("only"));

        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.current().unwrap().id, "only");
    }

    // =========================================================================
    // ScreenStack query tests
    // =========================================================================

    #[test]
    fn test_screen_stack_current() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("first"));
        assert_eq!(stack.current().unwrap().id, "first");

        stack.push(Screen::new("second"));
        assert_eq!(stack.current().unwrap().id, "second");
    }

    #[test]
    fn test_screen_stack_current_empty() {
        let stack = ScreenStack::new();
        assert!(stack.current().is_none());
    }

    #[test]
    fn test_screen_stack_current_mut() {
        let mut stack = ScreenStack::new();
        stack.push(Screen::new("test"));

        if let Some(screen) = stack.current_mut() {
            screen.title = "Modified".to_string();
        }

        assert_eq!(stack.current().unwrap().title, "Modified");
    }

    #[test]
    fn test_screen_stack_get() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        stack.push(Screen::new("settings"));
        stack.push(Screen::new("detail"));

        assert_eq!(stack.get("home").unwrap().id, "home");
        assert_eq!(stack.get("settings").unwrap().id, "settings");
        assert_eq!(stack.get("detail").unwrap().id, "detail");
        assert!(stack.get("missing").is_none());
    }

    #[test]
    fn test_screen_stack_contains() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        stack.push(Screen::new("settings"));

        assert!(stack.contains("home"));
        assert!(stack.contains("settings"));
        assert!(!stack.contains("unknown"));
    }

    #[test]
    fn test_screen_stack_depth() {
        let mut stack = ScreenStack::new();

        assert_eq!(stack.depth(), 0);

        stack.push(Screen::new("a"));
        assert_eq!(stack.depth(), 1);

        stack.push(Screen::new("b"));
        assert_eq!(stack.depth(), 2);

        stack.pop();
        assert_eq!(stack.depth(), 1);
    }

    // =========================================================================
    // ScreenStack navigation tests
    // =========================================================================

    #[test]
    fn test_cannot_pop_last_screen() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        assert_eq!(stack.depth(), 1);

        let result = stack.pop();
        assert!(result.is_none());
        assert_eq!(stack.depth(), 1);
    }

    #[test]
    fn test_can_go_back() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        assert!(!stack.can_go_back());

        stack.push(Screen::new("detail"));
        assert!(stack.can_go_back());

        stack.pop();
        assert!(!stack.can_go_back());
    }

    #[test]
    fn test_go_back() {
        let mut stack = ScreenStack::new();

        stack.push(Screen::new("home"));
        assert!(!stack.can_go_back());

        stack.push(Screen::new("detail"));
        assert!(stack.can_go_back());

        assert!(stack.go_back());
        assert_eq!(stack.current().unwrap().id, "home");
    }

    #[test]
    fn test_go_back_returns_false() {
        let mut stack = ScreenStack::new();
        stack.push(Screen::new("home"));

        assert!(!stack.go_back());
        assert_eq!(stack.current().unwrap().id, "home");
    }

    // =========================================================================
    // ScreenStack transition tests
    // =========================================================================

    #[test]
    fn test_update_transition() {
        let mut stack = ScreenStack::new().transition(ScreenTransition::Fade);
        stack.push(Screen::new("test"));

        // Update transition multiple times
        stack.update_transition(0.1);
        stack.update_transition(0.1);
        stack.update_transition(0.1);
        stack.update_transition(0.1);
        stack.update_transition(0.1);

        // Should complete after ~5 updates (0.5 seconds at default rate)
        // Can't check private fields, but verify the call doesn't panic
        stack.update_transition(1.0);
    }

    // =========================================================================
    // ScreenStack handle_key tests
    // =========================================================================

    #[test]
    fn test_handle_key_escape_goes_back() {
        use crate::event::Key;

        let mut stack = ScreenStack::new();
        stack.push(Screen::new("home"));
        stack.push(Screen::new("detail"));

        assert!(stack.handle_key(&Key::Escape));
        assert_eq!(stack.current().unwrap().id, "home");
    }

    #[test]
    fn test_handle_key_escape_no_back() {
        use crate::event::Key;

        let mut stack = ScreenStack::new();
        stack.push(Screen::new("home"));

        assert!(!stack.handle_key(&Key::Escape));
        assert_eq!(stack.current().unwrap().id, "home");
    }

    #[test]
    fn test_handle_key_other_key() {
        use crate::event::Key;

        let mut stack = ScreenStack::new();
        stack.push(Screen::new("home"));
        stack.push(Screen::new("detail"));

        assert!(!stack.handle_key(&Key::Char('x')));
        assert_eq!(stack.current().unwrap().id, "detail");
    }

    // =========================================================================
    // ScreenStack Default tests
    // =========================================================================

    #[test]
    fn test_screen_stack_default() {
        let stack = ScreenStack::default();
        assert!(stack.screens.is_empty());
        assert_eq!(stack.transition, ScreenTransition::None);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_screen_helper() {
        let s = screen("test");
        assert_eq!(s.id, "test");
    }

    #[test]
    fn test_screen_stack_helper() {
        let stack = screen_stack();
        assert!(stack.screens.is_empty());
    }

    // =========================================================================
    // ScreenStack history tests
    // =========================================================================

    #[test]
    fn test_screen_data_multiple_values() {
        let s = Screen::new("test")
            .data("key", "value1")
            .data("key", "value2"); // Same key overwrites

        assert_eq!(s.get_data("key"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_screen_public_fields() {
        let s = Screen {
            id: "test",
            title: "Test".to_string(),
            modal: true,
            data: std::collections::HashMap::new(),
        };

        assert_eq!(s.id, "test");
        assert_eq!(s.title, "Test");
        assert!(s.modal);
    }
}
