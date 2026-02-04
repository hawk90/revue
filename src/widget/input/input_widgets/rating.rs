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

    #[test]
    fn test_rating_new() {
        let r = Rating::new();
        assert_eq!(r.get_value(), 0.0);
        assert_eq!(r.max_value, 5);
    }

    #[test]
    fn test_rating_value() {
        let r = Rating::new().value(3.5);
        assert_eq!(r.get_value(), 3.5);
    }

    #[test]
    fn test_rating_increment_decrement() {
        let mut r = Rating::new().value(2.0);

        r.increment();
        assert_eq!(r.get_value(), 2.5);

        r.decrement();
        assert_eq!(r.get_value(), 2.0);
    }

    #[test]
    fn test_rating_bounds() {
        let mut r = Rating::new().value(5.0).max_value(5);

        r.increment();
        assert_eq!(r.get_value(), 5.0); // Should not exceed max

        r.set_value(0.0);
        r.decrement();
        assert_eq!(r.get_value(), 0.0); // Should not go below 0
    }

    #[test]
    fn test_rating_presets() {
        let five = Rating::five_star();
        assert_eq!(five.max_value, 5);

        let ten = Rating::ten_star();
        assert_eq!(ten.max_value, 10);

        let hearts = Rating::hearts();
        assert!(matches!(hearts.style, RatingStyle::Heart));
    }

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
    fn test_rating_style_chars() {
        let star = RatingStyle::Star.chars();
        assert_eq!(star, ('★', '☆'));

        let heart = RatingStyle::Heart.chars();
        assert_eq!(heart, ('♥', '♡'));
    }

    #[test]
    fn test_rating_helper() {
        let r = rating().value(4.0);
        assert_eq!(r.get_value(), 4.0);
    }
}
