//! Core types for testing utilities

use crate::event::{KeyEvent, MouseEvent};
use std::time::Duration;

/// Simulated event for testing
#[derive(Debug, Clone)]
pub enum SimulatedEvent {
    /// Key press event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Wait for duration
    Wait(Duration),
    /// Custom callback
    Custom(String),
}
