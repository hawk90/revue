//! Rating widget for star ratings and feedback
//!
//! Displays customizable star ratings with hover and selection support.

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Rating display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RatingStyle {
    /// Star icons ★☆
    #[default]
    Star,
    /// Heart icons ♥♡
    Heart,
    /// Circle icons ●○
    Circle,
    /// Square icons ■□
    Square,
    /// Numeric display (1-5)
    Numeric,
    /// Custom characters
    Custom(char, char),
}

impl RatingStyle {
    fn chars(&self) -> (char, char) {
        match self {
            RatingStyle::Star => ('★', '☆'),
            RatingStyle::Heart => ('♥', '♡'),
            RatingStyle::Circle => ('●', '○'),
            RatingStyle::Square => ('■', '□'),
            RatingStyle::Numeric => ('●', '○'),
            RatingStyle::Custom(filled, empty) => (*filled, *empty),
        }
    }

    /// Get half-filled character for this style
    fn half_char(&self) -> char {
        match self {
            RatingStyle::Star => '⯪',                  // Half-filled star
            RatingStyle::Heart => '❥',                 // Rotated heart (visual half)
            RatingStyle::Circle => '◐',                // Left half black circle
            RatingStyle::Square => '◧',                // Left half black square
            RatingStyle::Numeric => '◐',               // Same as circle for numeric
            RatingStyle::Custom(filled, _) => *filled, // Use filled for custom
        }
    }
}

/// Rating size
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RatingSize {
    /// Small (1 char per star)
    Small,
    /// Medium (2 chars per star)
    #[default]
    Medium,
    /// Large (3 chars per star)
    Large,
}

impl RatingSize {
    fn spacing(&self) -> usize {
        match self {
            RatingSize::Small => 1,
            RatingSize::Medium => 2,
            RatingSize::Large => 3,
        }
    }
}

/// Rating widget for displaying and selecting ratings
#[derive(Clone, Debug)]
pub struct Rating {
    /// Current rating value (0.0 to max_value)
    value: f32,
    /// Maximum rating value
    max_value: u8,
    /// Rating style
    style: RatingStyle,
    /// Size
    size: RatingSize,
    /// Allow half stars
    half_stars: bool,
    /// Read-only mode
    readonly: bool,
    /// Filled star color
    filled_color: Color,
    /// Empty star color
    empty_color: Color,
    /// Hover color
    hover_color: Color,
    /// Currently hovered value (for preview)
    hover_value: Option<f32>,
    /// Show numeric value
    show_value: bool,
    /// Label text
    label: Option<String>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Rating {
    /// Create a new rating widget
    pub fn new() -> Self {
        Self {
            value: 0.0,
            max_value: 5,
            style: RatingStyle::Star,
            size: RatingSize::Medium,
            half_stars: true,
            readonly: false,
            filled_color: Color::rgb(255, 200, 0), // Gold
            empty_color: Color::rgb(100, 100, 100),
            hover_color: Color::rgb(255, 220, 100),
            hover_value: None,
            show_value: false,
            label: None,
            props: WidgetProps::new(),
        }
    }

    /// Set the rating value
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, self.max_value as f32);
        self
    }

    /// Set maximum rating value
    pub fn max_value(mut self, max: u8) -> Self {
        self.max_value = max.max(1);
        self.value = self.value.min(max as f32);
        self
    }

    /// Set rating style
    pub fn style(mut self, style: RatingStyle) -> Self {
        self.style = style;
        self
    }

    /// Set size
    pub fn size(mut self, size: RatingSize) -> Self {
        self.size = size;
        self
    }

    /// Enable/disable half stars
    pub fn half_stars(mut self, enable: bool) -> Self {
        self.half_stars = enable;
        self
    }

    /// Set read-only mode
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Set filled star color
    pub fn filled_color(mut self, color: Color) -> Self {
        self.filled_color = color;
        self
    }

    /// Set empty star color
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_color = color;
        self
    }

    /// Set hover color
    pub fn hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    /// Show numeric value
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get current value
    pub fn get_value(&self) -> f32 {
        self.value
    }

    /// Set value programmatically
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0.0, self.max_value as f32);
    }

    /// Set hover preview
    pub fn set_hover(&mut self, value: Option<f32>) {
        self.hover_value = value.map(|v| v.clamp(0.0, self.max_value as f32));
    }

    /// Increment rating
    pub fn increment(&mut self) {
        let step = if self.half_stars { 0.5 } else { 1.0 };
        self.value = (self.value + step).min(self.max_value as f32);
    }

    /// Decrement rating
    pub fn decrement(&mut self) {
        let step = if self.half_stars { 0.5 } else { 1.0 };
        self.value = (self.value - step).max(0.0);
    }

    /// Clear rating
    pub fn clear(&mut self) {
        self.value = 0.0;
    }

    // Presets

    /// Create a 5-star rating
    pub fn five_star() -> Self {
        Self::new().max_value(5)
    }

    /// Create a 10-star rating
    pub fn ten_star() -> Self {
        Self::new().max_value(10)
    }

    /// Create a heart rating
    pub fn hearts() -> Self {
        Self::new().style(RatingStyle::Heart)
    }

    /// Create a thumbs up/down (2-star)
    pub fn thumbs() -> Self {
        Self::new()
            .max_value(2)
            .half_stars(false)
            .style(RatingStyle::Custom('👍', '👎'))
    }
}

impl Default for Rating {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Rating {
    crate::impl_view_meta!("Rating");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        let mut x = area.x;
        let y = area.y;

        // Render label if present
        if let Some(ref label) = self.label {
            for ch in label.chars() {
                if x >= area.x + area.width {
                    break;
                }
                ctx.buffer.set(x, y, Cell::new(ch).fg(Color::WHITE));
                x += 1;
            }
            x += 1; // Space after label
        }

        // Determine which value to display (hover or actual)
        let display_value = self.hover_value.unwrap_or(self.value);
        let (filled_char, empty_char) = self.style.chars();

        // Render stars
        for i in 0..self.max_value {
            if x >= area.x + area.width {
                break;
            }

            let star_value = i as f32 + 1.0;
            let (ch, color) = if display_value >= star_value {
                // Fully filled
                let color = if self.hover_value.is_some() {
                    self.hover_color
                } else {
                    self.filled_color
                };
                (filled_char, color)
            } else if self.half_stars && display_value >= star_value - 0.5 {
                // Half filled - use half-filled character
                let color = if self.hover_value.is_some() {
                    self.hover_color
                } else {
                    self.filled_color
                };
                (self.style.half_char(), color)
            } else {
                // Empty
                (empty_char, self.empty_color)
            };

            ctx.buffer.set(x, y, Cell::new(ch).fg(color));
            x += self.size.spacing() as u16;
        }

        // Show numeric value if enabled
        if self.show_value {
            x += 1;
            let value_str = format!("{:.1}/{}", display_value, self.max_value);
            for ch in value_str.chars() {
                if x >= area.x + area.width {
                    break;
                }
                ctx.buffer
                    .set(x, y, Cell::new(ch).fg(Color::rgb(150, 150, 150)));
                x += 1;
            }
        }
    }
}

impl_styled_view!(Rating);
impl_props_builders!(Rating);

/// Helper function to create a rating widget
pub fn rating() -> Rating {
    Rating::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // RatingStyle Enum Tests
    // =========================================================================

    #[test]
    fn test_rating_style_default() {
        let style = RatingStyle::default();
        assert_eq!(style, RatingStyle::Star);
    }

    #[test]
    fn test_rating_style_clone() {
        let style1 = RatingStyle::Heart;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_rating_style_copy() {
        let style1 = RatingStyle::Circle;
        let style2 = style1;
        assert_eq!(style1, RatingStyle::Circle);
        assert_eq!(style2, RatingStyle::Circle);
    }

    #[test]
    fn test_rating_style_partial_eq() {
        assert_eq!(RatingStyle::Star, RatingStyle::Star);
        assert_eq!(RatingStyle::Heart, RatingStyle::Heart);
        assert_ne!(RatingStyle::Star, RatingStyle::Heart);
    }

    #[test]
    fn test_rating_style_all_variants() {
        let styles = [
            RatingStyle::Star,
            RatingStyle::Heart,
            RatingStyle::Circle,
            RatingStyle::Square,
            RatingStyle::Numeric,
            RatingStyle::Custom('✓', '✗'),
        ];
        assert_eq!(styles.len(), 6);
    }

    #[test]
    fn test_rating_style_chars_star() {
        let (filled, empty) = RatingStyle::Star.chars();
        assert_eq!(filled, '★');
        assert_eq!(empty, '☆');
    }

    #[test]
    fn test_rating_style_chars_heart() {
        let (filled, empty) = RatingStyle::Heart.chars();
        assert_eq!(filled, '♥');
        assert_eq!(empty, '♡');
    }

    #[test]
    fn test_rating_style_chars_circle() {
        let (filled, empty) = RatingStyle::Circle.chars();
        assert_eq!(filled, '●');
        assert_eq!(empty, '○');
    }

    #[test]
    fn test_rating_style_chars_square() {
        let (filled, empty) = RatingStyle::Square.chars();
        assert_eq!(filled, '■');
        assert_eq!(empty, '□');
    }

    #[test]
    fn test_rating_style_chars_numeric() {
        let (filled, empty) = RatingStyle::Numeric.chars();
        assert_eq!(filled, '●');
        assert_eq!(empty, '○');
    }

    #[test]
    fn test_rating_style_chars_custom() {
        let (filled, empty) = RatingStyle::Custom('✓', '✗').chars();
        assert_eq!(filled, '✓');
        assert_eq!(empty, '✗');
    }

    #[test]
    fn test_rating_style_half_char_star() {
        let half = RatingStyle::Star.half_char();
        assert_eq!(half, '⯪');
    }

    #[test]
    fn test_rating_style_half_char_heart() {
        let half = RatingStyle::Heart.half_char();
        assert_eq!(half, '❥');
    }

    #[test]
    fn test_rating_style_half_char_circle() {
        let half = RatingStyle::Circle.half_char();
        assert_eq!(half, '◐');
    }

    #[test]
    fn test_rating_style_half_char_square() {
        let half = RatingStyle::Square.half_char();
        assert_eq!(half, '◧');
    }

    #[test]
    fn test_rating_style_half_char_numeric() {
        let half = RatingStyle::Numeric.half_char();
        assert_eq!(half, '◐');
    }

    #[test]
    fn test_rating_style_half_char_custom() {
        let half = RatingStyle::Custom('A', 'B').half_char();
        assert_eq!(half, 'A');
    }

    #[test]
    fn test_rating_style_debug() {
        let debug_str = format!("{:?}", RatingStyle::Star);
        assert!(debug_str.contains("Star"));
    }

    // =========================================================================
    // RatingSize Enum Tests
    // =========================================================================

    #[test]
    fn test_rating_size_default() {
        let size = RatingSize::default();
        assert_eq!(size, RatingSize::Medium);
    }

    #[test]
    fn test_rating_size_clone() {
        let size1 = RatingSize::Large;
        let size2 = size1.clone();
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_rating_size_copy() {
        let size1 = RatingSize::Small;
        let size2 = size1;
        assert_eq!(size1, RatingSize::Small);
        assert_eq!(size2, RatingSize::Small);
    }

    #[test]
    fn test_rating_size_partial_eq() {
        assert_eq!(RatingSize::Small, RatingSize::Small);
        assert_ne!(RatingSize::Small, RatingSize::Large);
    }

    #[test]
    fn test_rating_size_all_variants() {
        let sizes = [RatingSize::Small, RatingSize::Medium, RatingSize::Large];
        assert_eq!(sizes.len(), 3);
    }

    #[test]
    fn test_rating_size_spacing_small() {
        assert_eq!(RatingSize::Small.spacing(), 1);
    }

    #[test]
    fn test_rating_size_spacing_medium() {
        assert_eq!(RatingSize::Medium.spacing(), 2);
    }

    #[test]
    fn test_rating_size_spacing_large() {
        assert_eq!(RatingSize::Large.spacing(), 3);
    }

    // =========================================================================
    // Rating Constructor Tests
    // =========================================================================

    #[test]
    fn test_rating_new() {
        let r = Rating::new();
        assert_eq!(r.get_value(), 0.0);
        assert_eq!(r.max_value, 5);
        assert!(matches!(r.style, RatingStyle::Star));
        assert!(matches!(r.size, RatingSize::Medium));
        assert!(r.half_stars);
        assert!(!r.readonly);
        assert_eq!(r.filled_color, Color::rgb(255, 200, 0));
        assert_eq!(r.empty_color, Color::rgb(100, 100, 100));
        assert_eq!(r.hover_color, Color::rgb(255, 220, 100));
        assert!(r.hover_value.is_none());
        assert!(!r.show_value);
        assert!(r.label.is_none());
    }

    #[test]
    fn test_rating_default() {
        let r = Rating::default();
        assert_eq!(r.get_value(), 0.0);
        assert_eq!(r.max_value, 5);
    }

    // =========================================================================
    // Builder Method Tests
    // =========================================================================

    #[test]
    fn test_rating_value() {
        let r = Rating::new().value(3.5);
        assert_eq!(r.get_value(), 3.5);
    }

    #[test]
    fn test_rating_value_clamps_to_max() {
        let r = Rating::new().value(10.0);
        assert_eq!(r.get_value(), 5.0); // Clamped to max_value (5)
    }

    #[test]
    fn test_rating_value_clamps_to_zero() {
        let r = Rating::new().value(-5.0);
        assert_eq!(r.get_value(), 0.0);
    }

    #[test]
    fn test_rating_max_value() {
        let r = Rating::new().max_value(10);
        assert_eq!(r.max_value, 10);
    }

    #[test]
    fn test_rating_max_value_minimum() {
        let r = Rating::new().max_value(0);
        assert_eq!(r.max_value, 1); // Minimum is 1
    }

    #[test]
    fn test_rating_max_value_clamps_existing_value() {
        let r = Rating::new().value(5.0).max_value(3);
        assert_eq!(r.get_value(), 3.0); // Value clamped to new max
    }

    #[test]
    fn test_rating_style() {
        let r = Rating::new().style(RatingStyle::Heart);
        assert!(matches!(r.style, RatingStyle::Heart));
    }

    #[test]
    fn test_rating_size() {
        let r = Rating::new().size(RatingSize::Large);
        assert!(matches!(r.size, RatingSize::Large));
    }

    #[test]
    fn test_rating_half_stars_true() {
        let r = Rating::new().half_stars(true);
        assert!(r.half_stars);
    }

    #[test]
    fn test_rating_half_stars_false() {
        let r = Rating::new().half_stars(false);
        assert!(!r.half_stars);
    }

    #[test]
    fn test_rating_readonly_true() {
        let r = Rating::new().readonly(true);
        assert!(r.readonly);
    }

    #[test]
    fn test_rating_readonly_false() {
        let r = Rating::new().readonly(false);
        assert!(!r.readonly);
    }

    #[test]
    fn test_rating_filled_color() {
        let color = Color::RED;
        let r = Rating::new().filled_color(color);
        assert_eq!(r.filled_color, color);
    }

    #[test]
    fn test_rating_empty_color() {
        let color = Color::BLUE;
        let r = Rating::new().empty_color(color);
        assert_eq!(r.empty_color, color);
    }

    #[test]
    fn test_rating_hover_color() {
        let color = Color::GREEN;
        let r = Rating::new().hover_color(color);
        assert_eq!(r.hover_color, color);
    }

    #[test]
    fn test_rating_show_value_true() {
        let r = Rating::new().show_value(true);
        assert!(r.show_value);
    }

    #[test]
    fn test_rating_show_value_false() {
        let r = Rating::new().show_value(false);
        assert!(!r.show_value);
    }

    #[test]
    fn test_rating_label() {
        let r = Rating::new().label("Rate this:");
        assert_eq!(r.label, Some("Rate this:".to_string()));
    }

    #[test]
    fn test_rating_label_string() {
        let r = Rating::new().label(String::from("Score"));
        assert_eq!(r.label, Some("Score".to_string()));
    }

    // =========================================================================
    // State Mutation Method Tests
    // =========================================================================

    #[test]
    fn test_rating_set_value() {
        let mut r = Rating::new();
        r.set_value(4.5);
        assert_eq!(r.get_value(), 4.5);
    }

    #[test]
    fn test_rating_set_value_clamps() {
        let mut r = Rating::new();
        r.set_value(10.0);
        assert_eq!(r.get_value(), 5.0); // Clamped to max
    }

    #[test]
    fn test_rating_set_hover() {
        let mut r = Rating::new();
        r.set_hover(Some(3.5));
        assert_eq!(r.hover_value, Some(3.5));
    }

    #[test]
    fn test_rating_set_hover_clamps() {
        let mut r = Rating::new();
        r.set_hover(Some(10.0));
        assert_eq!(r.hover_value, Some(5.0)); // Clamped to max
    }

    #[test]
    fn test_rating_set_hover_none() {
        let mut r = Rating::new();
        r.set_hover(Some(3.0));
        r.set_hover(None);
        assert!(r.hover_value.is_none());
    }

    #[test]
    fn test_rating_increment() {
        let mut r = Rating::new().value(2.0);
        r.increment();
        assert_eq!(r.get_value(), 2.5);
    }

    #[test]
    fn test_rating_increment_whole_stars() {
        let mut r = Rating::new().value(2.0).half_stars(false);
        r.increment();
        assert_eq!(r.get_value(), 3.0);
    }

    #[test]
    fn test_rating_increment_clamps_at_max() {
        let mut r = Rating::new().value(5.0);
        r.increment();
        assert_eq!(r.get_value(), 5.0);
    }

    #[test]
    fn test_rating_increment_half_to_full() {
        let mut r = Rating::new().value(4.5);
        r.increment();
        assert_eq!(r.get_value(), 5.0);
    }

    #[test]
    fn test_rating_decrement() {
        let mut r = Rating::new().value(3.0);
        r.decrement();
        assert_eq!(r.get_value(), 2.5);
    }

    #[test]
    fn test_rating_decrement_whole_stars() {
        let mut r = Rating::new().value(3.0).half_stars(false);
        r.decrement();
        assert_eq!(r.get_value(), 2.0);
    }

    #[test]
    fn test_rating_decrement_clamps_at_zero() {
        let mut r = Rating::new().value(0.0);
        r.decrement();
        assert_eq!(r.get_value(), 0.0);
    }

    #[test]
    fn test_rating_decrement_half_to_zero() {
        let mut r = Rating::new().value(0.5);
        r.decrement();
        assert_eq!(r.get_value(), 0.0);
    }

    #[test]
    fn test_rating_clear() {
        let mut r = Rating::new().value(4.5);
        r.clear();
        assert_eq!(r.get_value(), 0.0);
    }

    // =========================================================================
    // Getter Method Tests
    // =========================================================================

    #[test]
    fn test_rating_get_value() {
        let r = Rating::new().value(3.5);
        assert_eq!(r.get_value(), 3.5);
    }

    #[test]
    fn test_rating_get_value_zero() {
        let r = Rating::new();
        assert_eq!(r.get_value(), 0.0);
    }

    // =========================================================================
    // Preset Method Tests
    // =========================================================================

    #[test]
    fn test_rating_five_star() {
        let r = Rating::five_star();
        assert_eq!(r.max_value, 5);
        assert_eq!(r.get_value(), 0.0);
    }

    #[test]
    fn test_rating_ten_star() {
        let r = Rating::ten_star();
        assert_eq!(r.max_value, 10);
    }

    #[test]
    fn test_rating_hearts() {
        let r = Rating::hearts();
        assert!(matches!(r.style, RatingStyle::Heart));
    }

    #[test]
    fn test_rating_thumbs() {
        let r = Rating::thumbs();
        assert_eq!(r.max_value, 2);
        assert!(!r.half_stars);
    }

    #[test]
    fn test_rating_thumbs_style() {
        let r = Rating::thumbs();
        assert!(matches!(r.style, RatingStyle::Custom('👍', '👎')));
    }

    // =========================================================================
    // Render Tests
    // =========================================================================

    #[test]
    fn test_rating_render() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let r = Rating::new().value(3.0);
        r.render(&mut ctx);

        // Should render stars
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
    }

    #[test]
    fn test_rating_render_empty_area() {
        let mut buffer = Buffer::new(0, 0);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let r = Rating::new().value(3.0);
        r.render(&mut ctx);
        // Should not panic with empty area
    }

    #[test]
    fn test_rating_render_with_label() {
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let r = Rating::new().label("Rate:").value(3.0);
        r.render(&mut ctx);

        // Should render label
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'R');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'a');
    }

    #[test]
    fn test_rating_render_with_show_value() {
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let r = Rating::new().value(3.5).show_value(true);
        r.render(&mut ctx);

        // Should render numeric value
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
    }

    #[test]
    fn test_rating_render_different_styles() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);

        let styles = [
            RatingStyle::Star,
            RatingStyle::Heart,
            RatingStyle::Circle,
            RatingStyle::Square,
        ];

        for style in styles {
            let mut ctx = RenderContext::new(&mut buffer, area);
            let r = Rating::new().value(3.0).style(style);
            r.render(&mut ctx);
            // Should not panic for any style
        }
    }

    #[test]
    fn test_rating_render_half_star() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let r = Rating::new().value(2.5);
        r.render(&mut ctx);

        // Should render with half star at position 4 (3rd star, spacing of 2)
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '★'); // Star 1, full
        assert_eq!(buffer.get(2, 0).unwrap().symbol, '★'); // Star 2, full
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '⯪'); // Star 3, half
    }

    #[test]
    fn test_rating_render_no_half_stars() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let r = Rating::new().value(2.5).half_stars(false);
        r.render(&mut ctx);

        // Should round to whole stars
        let cell = buffer.get(4, 0).unwrap();
        assert_eq!(cell.symbol, '☆'); // Should be empty (rounds down)
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_rating_helper() {
        let r = rating().value(4.0);
        assert_eq!(r.get_value(), 4.0);
    }

    #[test]
    fn test_rating_helper_chain() {
        let r = rating()
            .value(4.5)
            .max_value(10)
            .style(RatingStyle::Heart)
            .size(RatingSize::Large);
        assert_eq!(r.get_value(), 4.5);
        assert_eq!(r.max_value, 10);
        assert!(matches!(r.style, RatingStyle::Heart));
        assert!(matches!(r.size, RatingSize::Large));
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_rating_zero_max_value() {
        let r = Rating::new().max_value(1);
        assert_eq!(r.max_value, 1);
        assert_eq!(r.get_value(), 0.0);
    }

    #[test]
    fn test_rating_large_max_value() {
        let r = Rating::new().max_value(100).value(50.0);
        assert_eq!(r.max_value, 100);
        assert_eq!(r.get_value(), 50.0);
    }

    #[test]
    fn test_rating_negative_value_clamped() {
        let r = Rating::new().value(-10.0);
        assert_eq!(r.get_value(), 0.0);
    }

    #[test]
    fn test_rating_exact_half() {
        let r = Rating::new().value(2.5);
        assert_eq!(r.get_value(), 2.5);
    }

    #[test]
    fn test_rating_quarter_values() {
        let mut r = Rating::new().value(2.25);
        // Should store the value as-is
        assert_eq!(r.get_value(), 2.25);
    }

    #[test]
    fn test_rating_multiple_increments() {
        let mut r = Rating::new();
        for _ in 0..10 {
            r.increment();
        }
        assert_eq!(r.get_value(), 5.0); // Clamped at max
    }

    #[test]
    fn test_rating_multiple_decrements() {
        let mut r = Rating::new().value(5.0);
        for _ in 0..10 {
            r.decrement();
        }
        assert_eq!(r.get_value(), 0.0); // Clamped at min
    }

    #[test]
    fn test_rating_toggle_half_stars() {
        let mut r = Rating::new().value(2.0).half_stars(true);
        assert!(r.half_stars);

        r.half_stars = false;
        r.increment();
        assert_eq!(r.get_value(), 3.0); // Whole star increment
    }

    #[test]
    fn test_rating_long_label() {
        let long_label = "This is a very long rating label that exceeds width";
        let r = Rating::new().label(long_label);
        assert_eq!(r.label, Some(long_label.to_string()));
    }

    #[test]
    fn test_rating_unicode_label() {
        let r = Rating::new().label("⭐ Rate this! ⭐");
        assert_eq!(r.label, Some("⭐ Rate this! ⭐".to_string()));
    }

    #[test]
    fn test_rating_clone() {
        let r1 = Rating::new()
            .value(3.5)
            .style(RatingStyle::Heart)
            .size(RatingSize::Large);
        let r2 = r1.clone();

        assert_eq!(r1.get_value(), r2.get_value());
        assert_eq!(r1.style, r2.style);
        assert_eq!(r1.size, r2.size);
    }

    #[test]
    fn test_rating_debug() {
        let r = Rating::new().value(3.5);
        let debug_str = format!("{:?}", r);
        assert!(debug_str.contains("Rating"));
    }

    #[test]
    fn test_rating_custom_style_preserved() {
        let custom_style = RatingStyle::Custom('🔥', '💧');
        let r = Rating::new().style(custom_style);
        assert!(matches!(r.style, RatingStyle::Custom('🔥', '💧')));
    }

    #[test]
    fn test_rating_hover_preview() {
        let mut r = Rating::new().value(2.0);
        r.set_hover(Some(4.0));

        // Hover value should be used for display
        assert_eq!(r.hover_value, Some(4.0));
        assert_eq!(r.get_value(), 2.0); // Actual value unchanged
    }

    #[test]
    fn test_rating_clear_hover() {
        let mut r = Rating::new();
        r.set_hover(Some(3.0));
        r.set_hover(None);
        assert!(r.hover_value.is_none());
    }
}
