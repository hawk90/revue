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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_config() {
        let config = ProgressBarConfig {
            x: 10,
            y: 20,
            width: 30,
            progress: 0.5,
            filled_char: '█',
            empty_char: '░',
            fg: Color::GREEN,
        };
        assert_eq!(config.x, 10);
        assert_eq!(config.y, 20);
        assert_eq!(config.width, 30);
        assert_eq!(config.progress, 0.5);
        assert_eq!(config.filled_char, '█');
        assert_eq!(config.empty_char, '░');
    }

    #[test]
    fn test_progress_bar_config_fields() {
        let config = ProgressBarConfig {
            x: 5,
            y: 10,
            width: 25,
            progress: 0.75,
            filled_char: '#',
            empty_char: '-',
            fg: Color::BLUE,
        };
        assert_eq!(config.x, 5);
        assert_eq!(config.y, 10);
        assert_eq!(config.width, 25);
        assert_eq!(config.progress, 0.75);
        assert_eq!(config.filled_char, '#');
        assert_eq!(config.empty_char, '-');
        assert_eq!(config.fg, Color::BLUE);
    }
}
