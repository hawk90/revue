//! Theme helper functions for the showcase

use revue::prelude::*;

/// Returns common theme colors as a tuple:
/// (primary, success, warning, error, info, text_muted, text, surface)
pub fn theme_colors() -> (Color, Color, Color, Color, Color, Color, Color, Color) {
    let t = use_theme().get();
    (
        t.palette.primary,
        t.palette.success,
        t.palette.warning,
        t.palette.error,
        t.palette.info,
        t.colors.text_muted,
        t.colors.text,
        t.colors.surface,
    )
}

/// Create a consistently styled gauge
/// - fill_color: theme color for the filled portion
/// - fill_background: darkened version (40% darker)
/// - empty_background: surface color from theme
pub fn themed_gauge(value: f64, label: &str, fill_color: Color, empty_bg: Color) -> Gauge {
    Gauge::new()
        .value(value)
        .label(label)
        .fill_color(fill_color)
        .fill_background(fill_color.darken_pct(0.4))
        .empty_background(empty_bg)
        .width(20)
}

/// Create a gauge that changes color based on threshold (e.g., CPU > 80% turns red)
pub fn threshold_gauge(
    value: f64,
    label: &str,
    normal_color: Color,
    warning_color: Color,
    empty_bg: Color,
    threshold: f64,
) -> Gauge {
    let fill_color = if value > threshold {
        warning_color
    } else {
        normal_color
    };
    themed_gauge(value, label, fill_color, empty_bg)
}
