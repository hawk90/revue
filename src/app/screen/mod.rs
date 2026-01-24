//! Screen system for multi-page TUI applications
//!
//! Provides a screen stack for managing multiple views/pages with transitions.

mod core;
mod state;
#[cfg(test)]
mod tests;
mod types;

pub use core::ScreenManager;
pub use state::{screen_manager, simple_screen, SimpleScreen};
pub use types::{
    Screen, ScreenConfig, ScreenData, ScreenEvent, ScreenId, ScreenMode, ScreenResult, Transition,
};
