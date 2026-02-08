//! Badge widget for labels and status indicators

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Badge variant/color preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    /// Default/neutral (gray)
    #[default]
    Default,
    /// Primary (blue)
    Primary,
    /// Success (green)
    Success,
    /// Warning (yellow/orange)
    Warning,
    /// Error/Danger (red)
    Error,
    /// Info (cyan)
    Info,
}

impl BadgeVariant {
    /// Get colors for this variant (bg, fg)
    pub fn colors(&self) -> (Color, Color) {
        match self {
            BadgeVariant::Default => (Color::rgb(80, 80, 80), Color::WHITE),
            BadgeVariant::Primary => (Color::rgb(50, 100, 200), Color::WHITE),
            BadgeVariant::Success => (Color::rgb(40, 160, 80), Color::WHITE),
            BadgeVariant::Warning => (Color::rgb(200, 150, 40), Color::BLACK),
            BadgeVariant::Error => (Color::rgb(200, 60, 60), Color::WHITE),
            BadgeVariant::Info => (Color::rgb(60, 160, 180), Color::WHITE),
        }
    }
}

/// Badge shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeShape {
    /// Rounded with space padding (default)
    #[default]
    Rounded,
    /// Square/rectangular
    Square,
    /// Pill shape (extra rounded)
    Pill,
    /// Dot indicator
    Dot,
}

/// A badge widget for labels, counts, and status indicators
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// hstack()
///     .child(text("Messages"))
///     .child(badge("5").primary())
/// ```
pub struct Badge {
    /// Content text
    text: String,
    /// Variant
    variant: BadgeVariant,
    /// Shape
    shape: BadgeShape,
    /// Custom background color
    bg_color: Option<Color>,
    /// Custom foreground color
    fg_color: Option<Color>,
    /// Bold text
    bold: bool,
    /// Max width (0 = auto)
    max_width: u16,
    props: WidgetProps,
}

impl Badge {
    /// Create a new badge
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: BadgeVariant::Default,
            shape: BadgeShape::Rounded,
            bg_color: None,
            fg_color: None,
            bold: false,
            max_width: 0,
            props: WidgetProps::new(),
        }
    }

    /// Create a dot badge (status indicator)
    pub fn dot() -> Self {
        Self {
            text: String::new(),
            variant: BadgeVariant::Default,
            shape: BadgeShape::Dot,
            bg_color: None,
            fg_color: None,
            bold: false,
            max_width: 0,
            props: WidgetProps::new(),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set shape
    pub fn shape(mut self, shape: BadgeShape) -> Self {
        self.shape = shape;
        self
    }

    /// Primary variant shorthand
    pub fn primary(mut self) -> Self {
        self.variant = BadgeVariant::Primary;
        self
    }

    /// Success variant shorthand
    pub fn success(mut self) -> Self {
        self.variant = BadgeVariant::Success;
        self
    }

    /// Warning variant shorthand
    pub fn warning(mut self) -> Self {
        self.variant = BadgeVariant::Warning;
        self
    }

    /// Error variant shorthand
    pub fn error(mut self) -> Self {
        self.variant = BadgeVariant::Error;
        self
    }

    /// Info variant shorthand
    pub fn info(mut self) -> Self {
        self.variant = BadgeVariant::Info;
        self
    }

    /// Pill shape shorthand
    pub fn pill(mut self) -> Self {
        self.shape = BadgeShape::Pill;
        self
    }

    /// Square shape shorthand
    pub fn square(mut self) -> Self {
        self.shape = BadgeShape::Square;
        self
    }

    /// Set custom background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set custom foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Set custom colors
    pub fn colors(mut self, bg: Color, fg: Color) -> Self {
        self.bg_color = Some(bg);
        self.fg_color = Some(fg);
        self
    }

    /// Make text bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set max width
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Get effective colors
    fn effective_colors(&self) -> (Color, Color) {
        let (default_bg, default_fg) = self.variant.colors();
        (
            self.bg_color.unwrap_or(default_bg),
            self.fg_color.unwrap_or(default_fg),
        )
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Badge {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let (bg, fg) = self.effective_colors();

        match self.shape {
            BadgeShape::Dot => {
                // Just a colored dot
                let mut cell = Cell::new('●');
                cell.fg = Some(bg); // Use bg color as the dot color
                ctx.buffer.set(area.x, area.y, cell);
            }
            BadgeShape::Rounded | BadgeShape::Square | BadgeShape::Pill => {
                let text_len = self.text.chars().count() as u16;
                let padding = match self.shape {
                    BadgeShape::Pill => 2,
                    BadgeShape::Rounded => 1,
                    BadgeShape::Square => 1,
                    _ => 1,
                };

                let total_width = text_len + padding * 2;
                let width = if self.max_width > 0 {
                    total_width.min(self.max_width).min(area.width)
                } else {
                    total_width.min(area.width)
                };

                // Render background and text
                // Pre-collect chars to avoid O(n²) with .chars().nth() in loop
                let text_chars: Vec<char> = self.text.chars().collect();
                for i in 0..width {
                    let x = area.x + i;
                    let ch = if i < padding || i >= width - padding {
                        ' '
                    } else {
                        let char_idx = (i - padding) as usize;
                        text_chars.get(char_idx).copied().unwrap_or(' ')
                    };

                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    if self.bold {
                        cell.modifier |= Modifier::BOLD;
                    }
                    ctx.buffer.set(x, area.y, cell);
                }
            }
        }
    }

    crate::impl_view_meta!("Badge");
}

/// Create a new badge
pub fn badge(text: impl Into<String>) -> Badge {
    Badge::new(text)
}

/// Create a dot badge
pub fn dot_badge() -> Badge {
    Badge::dot()
}

impl_styled_view!(Badge);
impl_props_builders!(Badge);

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_new() {
        let b = Badge::new("Test");
        assert_eq!(b.text, "Test");
        assert_eq!(b.variant, BadgeVariant::Default);
    }

    #[test]
    fn test_badge_variants() {
        let b = badge("OK").success();
        assert_eq!(b.variant, BadgeVariant::Success);

        let b = badge("Error").error();
        assert_eq!(b.variant, BadgeVariant::Error);

        let b = badge("Info").info();
        assert_eq!(b.variant, BadgeVariant::Info);
    }

    #[test]
    fn test_badge_shapes() {
        let b = badge("Tag").pill();
        assert_eq!(b.shape, BadgeShape::Pill);

        let b = badge("Box").square();
        assert_eq!(b.shape, BadgeShape::Square);
    }

    #[test]
    fn test_badge_dot() {
        let b = Badge::dot().success();
        assert_eq!(b.shape, BadgeShape::Dot);
    }

    #[test]
    fn test_custom_colors() {
        let b = badge("Test").bg(Color::MAGENTA).fg(Color::BLACK);

        let (bg, fg) = b.effective_colors();
        assert_eq!(bg, Color::MAGENTA);
        assert_eq!(fg, Color::BLACK);
    }

    #[test]
    fn test_helper_functions() {
        let b = badge("Hi");
        assert_eq!(b.text, "Hi");

        let d = dot_badge();
        assert_eq!(d.shape, BadgeShape::Dot);
    }

    // =========================================================================
    // BadgeVariant enum tests
    // =========================================================================

    #[test]
    fn test_badge_variant_default() {
        let v = BadgeVariant::default();
        assert_eq!(v, BadgeVariant::Default);
    }

    #[test]
    fn test_badge_variant_clone() {
        let v = BadgeVariant::Success;
        let cloned = v.clone();
        assert_eq!(v, cloned);
    }

    #[test]
    fn test_badge_variant_copy() {
        let v1 = BadgeVariant::Error;
        let v2 = v1;
        assert_eq!(v1, BadgeVariant::Error);
        assert_eq!(v2, BadgeVariant::Error);
    }

    #[test]
    fn test_badge_variant_partial_eq() {
        assert_eq!(BadgeVariant::Primary, BadgeVariant::Primary);
        assert_ne!(BadgeVariant::Primary, BadgeVariant::Warning);
    }

    #[test]
    fn test_badge_variant_debug() {
        let v = BadgeVariant::Info;
        assert!(format!("{:?}", v).contains("Info"));
    }

    #[test]
    fn test_badge_variant_colors_default() {
        let (bg, fg) = BadgeVariant::Default.colors();
        assert_eq!(bg, Color::rgb(80, 80, 80));
        assert_eq!(fg, Color::WHITE);
    }

    #[test]
    fn test_badge_variant_colors_primary() {
        let (bg, fg) = BadgeVariant::Primary.colors();
        assert_eq!(bg, Color::rgb(50, 100, 200));
        assert_eq!(fg, Color::WHITE);
    }

    #[test]
    fn test_badge_variant_colors_success() {
        let (bg, fg) = BadgeVariant::Success.colors();
        assert_eq!(bg, Color::rgb(40, 160, 80));
        assert_eq!(fg, Color::WHITE);
    }

    #[test]
    fn test_badge_variant_colors_warning() {
        let (bg, fg) = BadgeVariant::Warning.colors();
        assert_eq!(bg, Color::rgb(200, 150, 40));
        assert_eq!(fg, Color::BLACK);
    }

    #[test]
    fn test_badge_variant_colors_error() {
        let (bg, fg) = BadgeVariant::Error.colors();
        assert_eq!(bg, Color::rgb(200, 60, 60));
        assert_eq!(fg, Color::WHITE);
    }

    #[test]
    fn test_badge_variant_colors_info() {
        let (bg, fg) = BadgeVariant::Info.colors();
        assert_eq!(bg, Color::rgb(60, 160, 180));
        assert_eq!(fg, Color::WHITE);
    }

    // =========================================================================
    // BadgeShape enum tests
    // =========================================================================

    #[test]
    fn test_badge_shape_default() {
        let s = BadgeShape::default();
        assert_eq!(s, BadgeShape::Rounded);
    }

    #[test]
    fn test_badge_shape_clone() {
        let s = BadgeShape::Pill;
        let cloned = s.clone();
        assert_eq!(s, cloned);
    }

    #[test]
    fn test_badge_shape_copy() {
        let s1 = BadgeShape::Square;
        let s2 = s1;
        assert_eq!(s1, BadgeShape::Square);
        assert_eq!(s2, BadgeShape::Square);
    }

    #[test]
    fn test_badge_shape_partial_eq() {
        assert_eq!(BadgeShape::Rounded, BadgeShape::Rounded);
        assert_ne!(BadgeShape::Rounded, BadgeShape::Dot);
    }

    #[test]
    fn test_badge_shape_debug() {
        let s = BadgeShape::Pill;
        assert!(format!("{:?}", s).contains("Pill"));
    }

    // =========================================================================
    // Badge builder tests
    // =========================================================================

    #[test]
    fn test_badge_variant_builder() {
        let b = Badge::new("Test").variant(BadgeVariant::Warning);
        assert_eq!(b.variant, BadgeVariant::Warning);
    }

    #[test]
    fn test_badge_shape_builder() {
        let b = Badge::new("Test").shape(BadgeShape::Square);
        assert_eq!(b.shape, BadgeShape::Square);
    }

    #[test]
    fn test_badge_primary_shorthand() {
        let b = badge("New").primary();
        assert_eq!(b.variant, BadgeVariant::Primary);
    }

    #[test]
    fn test_badge_success_shorthand() {
        let b = badge("Done").success();
        assert_eq!(b.variant, BadgeVariant::Success);
    }

    #[test]
    fn test_badge_warning_shorthand() {
        let b = badge("Wait").warning();
        assert_eq!(b.variant, BadgeVariant::Warning);
    }

    #[test]
    fn test_badge_error_shorthand() {
        let b = badge("Fail").error();
        assert_eq!(b.variant, BadgeVariant::Error);
    }

    #[test]
    fn test_badge_info_shorthand() {
        let b = badge("Help").info();
        assert_eq!(b.variant, BadgeVariant::Info);
    }

    #[test]
    fn test_badge_pill_shorthand() {
        let b = badge("Label").pill();
        assert_eq!(b.shape, BadgeShape::Pill);
    }

    #[test]
    fn test_badge_square_shorthand() {
        let b = badge("Box").square();
        assert_eq!(b.shape, BadgeShape::Square);
    }

    #[test]
    fn test_badge_colors_builder() {
        let b = badge("Test").colors(Color::CYAN, Color::MAGENTA);
        assert_eq!(b.bg_color, Some(Color::CYAN));
        assert_eq!(b.fg_color, Some(Color::MAGENTA));
    }

    #[test]
    fn test_badge_bold() {
        let b = badge("Bold").bold();
        assert!(b.bold);
    }

    #[test]
    fn test_badge_max_width() {
        let b = badge("Wide").max_width(10);
        assert_eq!(b.max_width, 10);
    }

    #[test]
    fn test_badge_new_with_str() {
        let b = Badge::new("Text");
        assert_eq!(b.text, "Text");
    }

    #[test]
    fn test_badge_new_with_string() {
        let s = String::from("Owned");
        let b = Badge::new(s);
        assert_eq!(b.text, "Owned");
    }

    #[test]
    fn test_badge_new_empty() {
        let b = Badge::new("");
        assert_eq!(b.text, "");
        assert!(b.text.is_empty());
    }

    #[test]
    fn test_badge_dot_with_variant() {
        let b = Badge::dot().error();
        assert_eq!(b.shape, BadgeShape::Dot);
        assert_eq!(b.variant, BadgeVariant::Error);
    }

    #[test]
    fn test_badge_effective_colors_fallback() {
        let b = badge("Test");
        let (bg, fg) = b.effective_colors();
        // Should use variant default colors
        let (expected_bg, expected_fg) = BadgeVariant::Default.colors();
        assert_eq!(bg, expected_bg);
        assert_eq!(fg, expected_fg);
    }

    #[test]
    fn test_badge_effective_colors_custom_bg() {
        let b = badge("Test").bg(Color::YELLOW);
        let (bg, fg) = b.effective_colors();
        assert_eq!(bg, Color::YELLOW);
        // fg should use variant default
        let (_, expected_fg) = BadgeVariant::Default.colors();
        assert_eq!(fg, expected_fg);
    }

    #[test]
    fn test_badge_effective_colors_custom_fg() {
        let b = badge("Test").fg(Color::CYAN);
        let (bg, fg) = b.effective_colors();
        // bg should use variant default
        let (expected_bg, _) = BadgeVariant::Default.colors();
        assert_eq!(bg, expected_bg);
        assert_eq!(fg, Color::CYAN);
    }

    // =========================================================================
    // Badge Default trait tests
    // =========================================================================

    #[test]
    fn test_badge_default_trait() {
        let b = Badge::default();
        assert_eq!(b.text, "");
        assert_eq!(b.variant, BadgeVariant::Default);
        assert_eq!(b.shape, BadgeShape::Rounded);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_badge_builder_chain() {
        let b = badge("Chain").primary().pill().bold().max_width(20);
        assert_eq!(b.text, "Chain");
        assert_eq!(b.variant, BadgeVariant::Primary);
        assert_eq!(b.shape, BadgeShape::Pill);
        assert!(b.bold);
        assert_eq!(b.max_width, 20);
    }

    #[test]
    fn test_badge_variant_chain() {
        let b = Badge::new("X")
            .variant(BadgeVariant::Info)
            .shape(BadgeShape::Square);
        assert_eq!(b.variant, BadgeVariant::Info);
        assert_eq!(b.shape, BadgeShape::Square);
    }
}
