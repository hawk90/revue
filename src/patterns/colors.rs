//! GitHub Dark theme color constants
//!
//! Provides a consistent color palette based on GitHub's dark theme.
//! These colors have been tested for readability in terminal environments.
//!
//! # Example
//!
//! ```ignore
//! use revue::patterns::colors::*;
//!
//! ctx.draw_text(x, y, "Success", GREEN);
//! ctx.draw_text(x, y, "Error", RED);
//! ctx.draw_text(x, y, "Warning", YELLOW);
//! ```

use crossterm::style::Color;

// ============================================================================
// Primary Colors
// ============================================================================

/// Cyan - For highlights, borders, titles
/// RGB(118, 217, 224) - GitHub Dark accent color
pub const CYAN: Color = Color::Rgb {
    r: 118,
    g: 217,
    b: 224,
};

/// Green - For success states, checkmarks, positive indicators
/// RGB(63, 185, 80) - GitHub success green
pub const GREEN: Color = Color::Rgb {
    r: 63,
    g: 185,
    b: 80,
};

/// Yellow - For warnings, in-progress states, messages
/// RGB(210, 153, 34) - GitHub warning yellow
pub const YELLOW: Color = Color::Rgb {
    r: 210,
    g: 153,
    b: 34,
};

/// Red - For errors, failures, destructive actions
/// RGB(248, 81, 73) - GitHub danger red
pub const RED: Color = Color::Rgb {
    r: 248,
    g: 81,
    b: 73,
};

/// Blue - For info, links, secondary highlights
/// RGB(88, 166, 255) - GitHub info blue
pub const BLUE: Color = Color::Rgb {
    r: 88,
    g: 166,
    b: 255,
};

/// Purple - For special states, tertiary highlights
/// RGB(163, 113, 247) - GitHub done purple
pub const PURPLE: Color = Color::Rgb {
    r: 163,
    g: 113,
    b: 247,
};

/// Orange - For attention, moderate warnings
/// RGB(219, 109, 40) - GitHub attention orange
pub const ORANGE: Color = Color::Rgb {
    r: 219,
    g: 109,
    b: 40,
};

// ============================================================================
// Foreground Colors
// ============================================================================

/// Primary foreground - For main text content
/// RGB(230, 237, 243) - GitHub foreground default
pub const FG: Color = Color::Rgb {
    r: 230,
    g: 237,
    b: 243,
};

/// Dimmed foreground - For secondary text, hints, disabled items
/// RGB(139, 148, 158) - GitHub foreground muted
pub const FG_DIM: Color = Color::Rgb {
    r: 139,
    g: 148,
    b: 158,
};

/// Subtle foreground - For very low emphasis text
/// RGB(88, 96, 105) - GitHub foreground subtle
pub const FG_SUBTLE: Color = Color::Rgb {
    r: 88,
    g: 96,
    b: 105,
};

// ============================================================================
// Background Colors
// ============================================================================

/// Primary background - For main canvas
/// RGB(13, 17, 23) - GitHub canvas default
pub const BG: Color = Color::Rgb {
    r: 13,
    g: 17,
    b: 23,
};

/// Subtle background - For panels, cards
/// RGB(22, 27, 34) - GitHub canvas subtle
pub const BG_SUBTLE: Color = Color::Rgb {
    r: 22,
    g: 27,
    b: 34,
};

/// Inset background - For inset/recessed areas
/// RGB(1, 4, 9) - GitHub canvas inset
pub const BG_INSET: Color = Color::Rgb { r: 1, g: 4, b: 9 };

// ============================================================================
// Border Colors
// ============================================================================

/// Default border - For standard borders
/// RGB(48, 54, 61) - GitHub border default
pub const BORDER: Color = Color::Rgb {
    r: 48,
    g: 54,
    b: 61,
};

/// Muted border - For subtle dividers
/// RGB(33, 38, 45) - GitHub border muted
pub const BORDER_MUTED: Color = Color::Rgb {
    r: 33,
    g: 38,
    b: 45,
};

// ============================================================================
// Semantic Colors
// ============================================================================

/// Success color (alias for GREEN)
pub const SUCCESS: Color = GREEN;

/// Error color (alias for RED)
pub const ERROR: Color = RED;

/// Warning color (alias for YELLOW)
pub const WARNING: Color = YELLOW;

/// Info color (alias for BLUE)
pub const INFO: Color = BLUE;

// ============================================================================
// Helper Functions
// ============================================================================

/// Get color for success/failure states
///
/// # Example
///
/// ```ignore
/// let color = status_color(build.is_success());
/// ctx.draw_text(x, y, "Status", color);
/// ```
pub fn status_color(success: bool) -> Color {
    if success {
        SUCCESS
    } else {
        ERROR
    }
}

/// Get color for build/job status
///
/// # Arguments
///
/// * `building` - Whether currently building/in-progress
/// * `success` - Whether last build/action succeeded
///
/// # Example
///
/// ```ignore
/// let color = build_color(job.is_building(), job.last_success);
/// ```
pub fn build_color(building: bool, success: bool) -> Color {
    if building {
        YELLOW
    } else if success {
        GREEN
    } else {
        RED
    }
}

/// Get color for priority levels (0=highest, 4=lowest)
///
/// # Example
///
/// ```ignore
/// let color = priority_color(issue.priority_level);
/// ctx.draw_text(x, y, &issue.priority, color);
/// ```
pub fn priority_color(level: u8) -> Color {
    match level {
        0 => RED,    // Highest
        1 => ORANGE, // High
        2 => YELLOW, // Medium
        3 => BLUE,   // Low
        _ => FG_DIM, // Lowest/None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_color() {
        assert_eq!(status_color(true), SUCCESS);
        assert_eq!(status_color(false), ERROR);
    }

    #[test]
    fn test_build_color() {
        assert_eq!(build_color(true, false), YELLOW);
        assert_eq!(build_color(false, true), GREEN);
        assert_eq!(build_color(false, false), RED);
    }

    #[test]
    fn test_priority_color() {
        assert_eq!(priority_color(0), RED);
        assert_eq!(priority_color(1), ORANGE);
        assert_eq!(priority_color(2), YELLOW);
        assert_eq!(priority_color(3), BLUE);
        assert_eq!(priority_color(4), FG_DIM);
    }

    #[test]
    fn test_priority_color_high_values() {
        // Values > 4 should return FG_DIM
        assert_eq!(priority_color(5), FG_DIM);
        assert_eq!(priority_color(100), FG_DIM);
        assert_eq!(priority_color(255), FG_DIM);
    }

    #[test]
    fn test_primary_colors_rgb() {
        // Verify CYAN
        if let Color::Rgb { r, g, b } = CYAN {
            assert_eq!(r, 118);
            assert_eq!(g, 217);
            assert_eq!(b, 224);
        }

        // Verify GREEN
        if let Color::Rgb { r, g, b } = GREEN {
            assert_eq!(r, 63);
            assert_eq!(g, 185);
            assert_eq!(b, 80);
        }

        // Verify RED
        if let Color::Rgb { r, g, b } = RED {
            assert_eq!(r, 248);
            assert_eq!(g, 81);
            assert_eq!(b, 73);
        }
    }

    #[test]
    fn test_foreground_colors() {
        // Verify FG is brightest
        if let Color::Rgb { r, g, b } = FG {
            assert!(r > 200 && g > 200 && b > 200);
        }

        // Verify FG_DIM is dimmer
        if let Color::Rgb { r, g, b } = FG_DIM {
            assert!(r < 200 && g < 200 && b < 200);
        }

        // Verify FG_SUBTLE is dimmest
        if let Color::Rgb { r, g, b } = FG_SUBTLE {
            assert!(r < 100 && g < 100 && b < 110);
        }
    }

    #[test]
    fn test_background_colors() {
        // Verify BG is darkest
        if let Color::Rgb { r, g, b } = BG {
            assert!(r < 20 && g < 20 && b < 30);
        }

        // Verify BG_INSET is even darker
        if let Color::Rgb { r, g, b } = BG_INSET {
            assert!(r < 10 && g < 10 && b < 15);
        }
    }

    #[test]
    fn test_semantic_color_aliases() {
        assert_eq!(SUCCESS, GREEN);
        assert_eq!(ERROR, RED);
        assert_eq!(WARNING, YELLOW);
        assert_eq!(INFO, BLUE);
    }

    #[test]
    fn test_border_colors() {
        // Verify BORDER is between BG and FG brightness
        if let Color::Rgb { r, g, b } = BORDER {
            assert!(r > 40 && r < 60);
            assert!(g > 50 && g < 60);
            assert!(b > 55 && b < 70);
        }

        // Verify BORDER_MUTED is dimmer than BORDER
        if let (Color::Rgb { r: r1, .. }, Color::Rgb { r: r2, .. }) = (BORDER, BORDER_MUTED) {
            assert!(r1 > r2);
        }
    }

    #[test]
    fn test_build_color_all_states() {
        // Test all combinations
        assert_eq!(build_color(true, true), YELLOW); // Building, was success
        assert_eq!(build_color(true, false), YELLOW); // Building, was failure
        assert_eq!(build_color(false, true), GREEN); // Not building, success
        assert_eq!(build_color(false, false), RED); // Not building, failure
    }
}
