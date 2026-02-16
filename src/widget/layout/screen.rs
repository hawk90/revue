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
