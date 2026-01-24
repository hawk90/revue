//! Screen system types

use std::any::Any;
use std::time::Duration;

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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Transition {
    /// No transition
    #[default]
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

/// Screen mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScreenMode {
    /// Full screen (replaces previous)
    #[default]
    Fullscreen,
    /// Modal (overlays previous)
    Modal,
    /// Popup (small overlay)
    Popup,
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
#[derive(Clone, Debug, Default)]
pub enum ScreenResult {
    /// Continue normally
    #[default]
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
