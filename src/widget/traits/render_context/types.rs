//! Types for render context

use crate::style::Color;

/// Progress bar rendering configuration
pub struct ProgressBarConfig {
    /// X position
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Total width of the bar
    pub width: u16,
    /// Progress value from 0.0 to 1.0
    pub progress: f32,
    /// Character for filled portion (e.g., '█')
    pub filled_char: char,
    /// Character for empty portion (e.g., '░')
    pub empty_char: char,
    /// Foreground color
    pub fg: Color,
}
