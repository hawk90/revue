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
        let mut screen: Option<Box<dyn Screen>> = None;

        // Check if already in stack
        if let Some(idx) = self.stack.iter().position(|e| e.screen.id() == id) {
            // Move to top
            let entry = self.stack.remove(idx);
            screen = Some(entry.screen);
        }

        // If not found in stack, try to create from registry
        if screen.is_none() {
            if let Some(factory) = self.registry.get(&id) {
                screen = Some(factory());
            } else {
                return false;
            }
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
