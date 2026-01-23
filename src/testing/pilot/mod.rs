//! Pilot - automated UI testing controller

mod async_pilot;
mod core;

#[cfg(test)]
mod tests;

// Re-exports
pub use async_pilot::AsyncPilot;
pub use core::Pilot;
