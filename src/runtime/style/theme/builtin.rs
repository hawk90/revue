//! Predefined popular themes (Dracula, Nord, Monokai, Solarized)

use super::{Palette, Theme, ThemeColors, ThemeVariant};
use crate::style::properties::Color;

/// Predefined themes
pub struct Themes;

impl Themes {
    /// Dracula theme
    pub fn dracula() -> Theme {
        Theme {
            name: "Dracula".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette {
                primary: Color::rgb(139, 233, 253),   // Cyan
                secondary: Color::rgb(255, 121, 198), // Pink
                success: Color::rgb(80, 250, 123),    // Green
                warning: Color::rgb(241, 250, 140),   // Yellow
                error: Color::rgb(255, 85, 85),       // Red
                info: Color::rgb(189, 147, 249),      // Purple
            },
            colors: ThemeColors {
                background: Color::rgb(40, 42, 54),
                surface: Color::rgb(68, 71, 90),
                text: Color::rgb(248, 248, 242),
                text_muted: Color::rgb(98, 114, 164),
                border: Color::rgb(68, 71, 90),
                divider: Color::rgb(68, 71, 90),
                selection: Color::rgb(68, 71, 90),
                selection_text: Color::rgb(248, 248, 242),
                focus: Color::rgb(139, 233, 253),
            },
        }
    }

    /// Nord theme
    pub fn nord() -> Theme {
        Theme {
            name: "Nord".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette {
                primary: Color::rgb(136, 192, 208),   // Frost
                secondary: Color::rgb(180, 142, 173), // Aurora purple
                success: Color::rgb(163, 190, 140),   // Aurora green
                warning: Color::rgb(235, 203, 139),   // Aurora yellow
                error: Color::rgb(191, 97, 106),      // Aurora red
                info: Color::rgb(129, 161, 193),      // Frost blue
            },
            colors: ThemeColors {
                background: Color::rgb(46, 52, 64),
                surface: Color::rgb(59, 66, 82),
                text: Color::rgb(236, 239, 244),
                text_muted: Color::rgb(147, 161, 161),
                border: Color::rgb(76, 86, 106),
                divider: Color::rgb(67, 76, 94),
                selection: Color::rgb(76, 86, 106),
                selection_text: Color::rgb(236, 239, 244),
                focus: Color::rgb(136, 192, 208),
            },
        }
    }

    /// Monokai theme
    pub fn monokai() -> Theme {
        Theme {
            name: "Monokai".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette {
                primary: Color::rgb(102, 217, 239),   // Cyan
                secondary: Color::rgb(174, 129, 255), // Purple
                success: Color::rgb(166, 226, 46),    // Green
                warning: Color::rgb(253, 151, 31),    // Orange
                error: Color::rgb(249, 38, 114),      // Red/Pink
                info: Color::rgb(102, 217, 239),      // Cyan
            },
            colors: ThemeColors {
                background: Color::rgb(39, 40, 34),
                surface: Color::rgb(49, 50, 44),
                text: Color::rgb(248, 248, 242),
                text_muted: Color::rgb(117, 113, 94),
                border: Color::rgb(73, 72, 62),
                divider: Color::rgb(73, 72, 62),
                selection: Color::rgb(73, 72, 62),
                selection_text: Color::rgb(248, 248, 242),
                focus: Color::rgb(166, 226, 46),
            },
        }
    }

    /// Solarized Dark theme
    pub fn solarized_dark() -> Theme {
        Theme {
            name: "Solarized Dark".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette {
                primary: Color::rgb(38, 139, 210),    // Blue
                secondary: Color::rgb(108, 113, 196), // Violet
                success: Color::rgb(133, 153, 0),     // Green
                warning: Color::rgb(181, 137, 0),     // Yellow
                error: Color::rgb(220, 50, 47),       // Red
                info: Color::rgb(42, 161, 152),       // Cyan
            },
            colors: ThemeColors {
                background: Color::rgb(0, 43, 54),
                surface: Color::rgb(7, 54, 66),
                text: Color::rgb(131, 148, 150),
                text_muted: Color::rgb(88, 110, 117),
                border: Color::rgb(7, 54, 66),
                divider: Color::rgb(7, 54, 66),
                selection: Color::rgb(7, 54, 66),
                selection_text: Color::rgb(147, 161, 161),
                focus: Color::rgb(38, 139, 210),
            },
        }
    }

    /// Solarized Light theme
    pub fn solarized_light() -> Theme {
        Theme {
            name: "Solarized Light".to_string(),
            variant: ThemeVariant::Light,
            palette: Palette {
                primary: Color::rgb(38, 139, 210),    // Blue
                secondary: Color::rgb(108, 113, 196), // Violet
                success: Color::rgb(133, 153, 0),     // Green
                warning: Color::rgb(181, 137, 0),     // Yellow
                error: Color::rgb(220, 50, 47),       // Red
                info: Color::rgb(42, 161, 152),       // Cyan
            },
            colors: ThemeColors {
                background: Color::rgb(253, 246, 227),
                surface: Color::rgb(238, 232, 213),
                text: Color::rgb(101, 123, 131),
                text_muted: Color::rgb(147, 161, 161),
                border: Color::rgb(238, 232, 213),
                divider: Color::rgb(238, 232, 213),
                selection: Color::rgb(238, 232, 213),
                selection_text: Color::rgb(88, 110, 117),
                focus: Color::rgb(38, 139, 210),
            },
        }
    }
}
