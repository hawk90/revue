//! Mocking utilities for testing Revue applications
//!
//! Provides utilities for mocking events, time, terminal size, and more.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::testing::{MockTerminal, MockTime, EventSimulator};
//!
//! // Mock terminal size
//! let terminal = MockTerminal::new(80, 24);
//!
//! // Mock time progression
//! let time = MockTime::new();
//! time.advance(Duration::from_secs(1));
//!
//! // Simulate user interactions
//! let mut sim = EventSimulator::new();
//! sim.key(Key::Enter).wait_ms(100).key(Key::Escape);
//! ```

mod capture;
mod helpers;
mod simulator;
mod state;
mod terminal;
mod time;
mod types;

#[cfg(test)]
mod tests;

// Re-export public API
pub use capture::RenderCapture;
pub use helpers::{
    capture_render, mock_alt_key, mock_click, mock_ctrl_key, mock_key, mock_mouse, mock_terminal,
    mock_time, simulate_user,
};
pub use simulator::EventSimulator;
pub use state::MockState;
pub use terminal::MockTerminal;
pub use time::MockTime;
pub use types::SimulatedEvent;
