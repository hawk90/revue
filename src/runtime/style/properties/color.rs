//! Color manipulation helpers and interaction state colors

use super::types::Color as ColorType;

impl ColorType {
    /// Check if this is the default (unset) color
    pub fn is_default(&self) -> bool {
        self.a == 0 && self.r == 0 && self.g == 0 && self.b == 0
    }

    /// Check if color is fully transparent
    pub const fn is_transparent(&self) -> bool {
        self.a == 0
    }

    /// Check if color is fully opaque
    pub const fn is_opaque(&self) -> bool {
        self.a == 255
    }

    /// Create color from RGB components (fully opaque)
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create color from RGBA components
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create color from hex value (0xRRGGBB, fully opaque)
    pub const fn hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    /// Create color from hex value with alpha (0xRRGGBBAA)
    pub const fn hexa(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }

    /// Create a new color with modified alpha
    pub const fn with_alpha(self, a: u8) -> Self {
        Self { a, ..self }
    }

    /// Create a semi-transparent version (alpha = 128)
    pub const fn semi_transparent(self) -> Self {
        self.with_alpha(128)
    }

    /// Get alpha as float (0.0 = transparent, 1.0 = opaque)
    pub fn alpha_f32(&self) -> f32 {
        self.a as f32 / 255.0
    }

    /// Create color with alpha from float (0.0 = transparent, 1.0 = opaque)
    pub fn with_alpha_f32(self, alpha: f32) -> Self {
        self.with_alpha((alpha.clamp(0.0, 1.0) * 255.0).round() as u8)
    }

    /// White color (#FFFFFF)
    pub const WHITE: ColorType = ColorType::rgb(255, 255, 255);
    /// Black color (#000000)
    pub const BLACK: ColorType = ColorType::rgb(0, 0, 0);
    /// Red color (#FF0000)
    pub const RED: ColorType = ColorType::rgb(255, 0, 0);
    /// Green color (#00FF00)
    pub const GREEN: ColorType = ColorType::rgb(0, 255, 0);
    /// Blue color (#0000FF)
    pub const BLUE: ColorType = ColorType::rgb(0, 0, 255);
    /// Cyan color (#00FFFF)
    pub const CYAN: ColorType = ColorType::rgb(0, 255, 255);
    /// Yellow color (#FFFF00)
    pub const YELLOW: ColorType = ColorType::rgb(255, 255, 0);
    /// Magenta color (#FF00FF)
    pub const MAGENTA: ColorType = ColorType::rgb(255, 0, 255);
    /// Transparent (fully transparent black)
    pub const TRANSPARENT: ColorType = ColorType::rgba(0, 0, 0, 0);

    // ─────────────────────────────────────────────────────────────────────────
    // Interaction Color Helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Darken the color by subtracting from RGB components
    ///
    /// # Arguments
    /// * `amount` - Value to subtract from each RGB component (0-255)
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(100, 150, 200);
    /// let darker = base.darken(30);  // rgb(70, 120, 170)
    /// ```
    #[must_use]
    pub const fn darken(self, amount: u8) -> Self {
        Self {
            r: self.r.saturating_sub(amount),
            g: self.g.saturating_sub(amount),
            b: self.b.saturating_sub(amount),
            a: self.a,
        }
    }

    /// Lighten the color by adding to RGB components
    ///
    /// # Arguments
    /// * `amount` - Value to add to each RGB component (0-255)
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(100, 150, 200);
    /// let lighter = base.lighten(30);  // rgb(130, 180, 230)
    /// ```
    #[must_use]
    pub const fn lighten(self, amount: u8) -> Self {
        Self {
            r: self.r.saturating_add(amount),
            g: self.g.saturating_add(amount),
            b: self.b.saturating_add(amount),
            a: self.a,
        }
    }

    /// Darken by percentage (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(100, 100, 100);
    /// let darker = base.darken_pct(0.2);  // 20% darker
    /// ```
    #[must_use]
    pub fn darken_pct(self, pct: f32) -> Self {
        let factor = 1.0 - pct.clamp(0.0, 1.0);
        Self {
            r: (self.r as f32 * factor).round() as u8,
            g: (self.g as f32 * factor).round() as u8,
            b: (self.b as f32 * factor).round() as u8,
            a: self.a,
        }
    }

    /// Lighten by percentage (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(100, 100, 100);
    /// let lighter = base.lighten_pct(0.2);  // 20% lighter
    /// ```
    #[must_use]
    pub fn lighten_pct(self, pct: f32) -> Self {
        let factor = (1.0 + pct.clamp(0.0, 1.0)).min(2.0);
        Self {
            r: (self.r as f32 * factor).round() as u8,
            g: (self.g as f32 * factor).round() as u8,
            b: (self.b as f32 * factor).round() as u8,
            a: self.a,
        }
    }

    /// Get pressed state color (standard darkening)
    ///
    /// # Example
    /// ```rust,ignore
    /// let button_bg = Color::rgb(37, 99, 235);
    /// let pressed_bg = button_bg.pressed();  // Darker by 30
    /// ```
    #[must_use]
    pub const fn pressed(self) -> Self {
        self.darken(30)
    }

    /// Get hover state color (standard lightening)
    ///
    /// # Example
    /// ```rust,ignore
    /// let button_bg = Color::rgb(37, 99, 235);
    /// let hover_bg = button_bg.hover();  // Lighter by 40
    /// ```
    #[must_use]
    pub const fn hover(self) -> Self {
        self.lighten(40)
    }

    /// Get focus state color (same as hover)
    ///
    /// # Example
    /// ```rust,ignore
    /// let button_bg = Color::rgb(37, 99, 235);
    /// let focus_bg = button_bg.focus();  // Lighter by 40
    /// ```
    #[must_use]
    pub const fn focus(self) -> Self {
        self.lighten(40)
    }

    /// Blend this color with another color
    ///
    /// # Arguments
    /// * `other` - The other color to blend with
    /// * `ratio` - Blend ratio (0.0 = self, 1.0 = other)
    ///
    /// # Example
    /// ```rust,ignore
    /// let red = Color::RED;
    /// let blue = Color::BLUE;
    /// let purple = red.blend(blue, 0.5);  // 50% mix
    /// ```
    #[must_use]
    pub fn blend(self, other: ColorType, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let inv = 1.0 - ratio;
        Self {
            r: (self.r as f32 * inv + other.r as f32 * ratio).round() as u8,
            g: (self.g as f32 * inv + other.g as f32 * ratio).round() as u8,
            b: (self.b as f32 * inv + other.b as f32 * ratio).round() as u8,
            a: (self.a as f32 * inv + other.a as f32 * ratio).round() as u8,
        }
    }

    /// Apply interaction state to color
    ///
    /// Convenience method that applies the appropriate color effect
    /// based on pressed/hovered/focused state.
    ///
    /// # Example
    /// ```rust,ignore
    /// let base = Color::rgb(37, 99, 235);
    /// let final_color = base.with_interaction(pressed, hovered, focused);
    /// ```
    #[must_use]
    pub const fn with_interaction(self, pressed: bool, hovered: bool, focused: bool) -> Self {
        if pressed {
            self.pressed()
        } else if hovered || focused {
            self.hover()
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_is_default() {
        let default = ColorType::default();
        assert!(default.is_default());

        let custom = ColorType::rgb(1, 0, 0);
        assert!(!custom.is_default());
    }

    #[test]
    fn test_color_is_transparent() {
        let transparent = ColorType::rgba(0, 0, 0, 0);
        assert!(transparent.is_transparent());

        let opaque = ColorType::rgb(255, 255, 255);
        assert!(!opaque.is_transparent());

        let semi = ColorType::rgba(255, 255, 255, 128);
        assert!(!semi.is_transparent());
    }

    #[test]
    fn test_color_is_opaque() {
        let opaque = ColorType::rgb(255, 255, 255);
        assert!(opaque.is_opaque());

        let transparent = ColorType::rgba(0, 0, 0, 0);
        assert!(!transparent.is_opaque());

        let semi = ColorType::rgba(255, 255, 255, 128);
        assert!(!semi.is_opaque());
    }

    #[test]
    fn test_color_rgb() {
        let color = ColorType::rgb(100, 150, 200);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_rgba() {
        let color = ColorType::rgba(100, 150, 200, 128);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn test_color_hex() {
        let color = ColorType::hex(0xFF8000);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_hex_white() {
        let color = ColorType::hex(0xFFFFFF);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_hex_black() {
        let color = ColorType::hex(0x000000);
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_hexa() {
        let color = ColorType::hexa(0xFF800080);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn test_color_with_alpha() {
        let color = ColorType::rgb(100, 150, 200);
        let with_alpha = color.with_alpha(128);
        assert_eq!(with_alpha.r, 100);
        assert_eq!(with_alpha.g, 150);
        assert_eq!(with_alpha.b, 200);
        assert_eq!(with_alpha.a, 128);
    }

    #[test]
    fn test_color_semi_transparent() {
        let color = ColorType::rgb(100, 150, 200);
        let semi = color.semi_transparent();
        assert_eq!(semi.a, 128);
        assert_eq!(semi.r, 100);
        assert_eq!(semi.g, 150);
        assert_eq!(semi.b, 200);
    }

    #[test]
    fn test_color_alpha_f32() {
        let opaque = ColorType::rgb(255, 255, 255);
        assert_eq!(opaque.alpha_f32(), 1.0);

        let transparent = ColorType::TRANSPARENT;
        assert_eq!(transparent.alpha_f32(), 0.0);

        let half = ColorType::rgba(255, 255, 255, 128);
        assert!((half.alpha_f32() - 0.502).abs() < 0.01);
    }

    #[test]
    fn test_color_with_alpha_f32() {
        let color = ColorType::rgb(100, 150, 200);
        let opaque = color.with_alpha_f32(1.0);
        assert_eq!(opaque.a, 255);

        let transparent = color.with_alpha_f32(0.0);
        assert_eq!(transparent.a, 0);

        let half = color.with_alpha_f32(0.5);
        assert_eq!(half.a, 128);
    }

    #[test]
    fn test_color_with_alpha_f32_clamps() {
        let color = ColorType::rgb(100, 150, 200);
        let over = color.with_alpha_f32(1.5);
        assert_eq!(over.a, 255);

        let under = color.with_alpha_f32(-0.5);
        assert_eq!(under.a, 0);
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(ColorType::WHITE, ColorType::rgb(255, 255, 255));
        assert_eq!(ColorType::BLACK, ColorType::rgb(0, 0, 0));
        assert_eq!(ColorType::RED, ColorType::rgb(255, 0, 0));
        assert_eq!(ColorType::GREEN, ColorType::rgb(0, 255, 0));
        assert_eq!(ColorType::BLUE, ColorType::rgb(0, 0, 255));
        assert_eq!(ColorType::CYAN, ColorType::rgb(0, 255, 255));
        assert_eq!(ColorType::YELLOW, ColorType::rgb(255, 255, 0));
        assert_eq!(ColorType::MAGENTA, ColorType::rgb(255, 0, 255));
        assert_eq!(ColorType::TRANSPARENT, ColorType::rgba(0, 0, 0, 0));
    }

    #[test]
    fn test_color_darken() {
        let color = ColorType::rgb(100, 150, 200);
        let darker = color.darken(30);
        assert_eq!(darker.r, 70);
        assert_eq!(darker.g, 120);
        assert_eq!(darker.b, 170);
        assert_eq!(darker.a, 255);
    }

    #[test]
    fn test_color_darken_saturates() {
        let color = ColorType::rgb(10, 20, 30);
        let darker = color.darken(50);
        assert_eq!(darker.r, 0);
        assert_eq!(darker.g, 0);
        assert_eq!(darker.b, 0);
        assert_eq!(darker.a, 255);
    }

    #[test]
    fn test_color_lighten() {
        let color = ColorType::rgb(100, 150, 200);
        let lighter = color.lighten(30);
        assert_eq!(lighter.r, 130);
        assert_eq!(lighter.g, 180);
        assert_eq!(lighter.b, 230);
        assert_eq!(lighter.a, 255);
    }

    #[test]
    fn test_color_lighten_saturates() {
        let color = ColorType::rgb(230, 240, 250);
        let lighter = color.lighten(30);
        assert_eq!(lighter.r, 255);
        assert_eq!(lighter.g, 255);
        assert_eq!(lighter.b, 255);
        assert_eq!(lighter.a, 255);
    }

    #[test]
    fn test_color_darken_pct() {
        let color = ColorType::rgb(100, 100, 100);
        let darker = color.darken_pct(0.2);
        assert_eq!(darker.r, 80);
        assert_eq!(darker.g, 80);
        assert_eq!(darker.b, 80);
    }

    #[test]
    fn test_color_darken_pct_clamps() {
        let color = ColorType::rgb(100, 100, 100);
        let darker = color.darken_pct(1.5);
        assert_eq!(darker.r, 0);
        assert_eq!(darker.g, 0);
        assert_eq!(darker.b, 0);
    }

    #[test]
    fn test_color_lighten_pct() {
        let color = ColorType::rgb(100, 100, 100);
        let lighter = color.lighten_pct(0.2);
        assert_eq!(lighter.r, 120);
        assert_eq!(lighter.g, 120);
        assert_eq!(lighter.b, 120);
    }

    #[test]
    fn test_color_lighten_pct_clamps() {
        let color = ColorType::rgb(200, 200, 200);
        let lighter = color.lighten_pct(1.5);
        assert_eq!(lighter.r, 255);
        assert_eq!(lighter.g, 255);
        assert_eq!(lighter.b, 255);
    }

    #[test]
    fn test_color_pressed() {
        let color = ColorType::rgb(100, 150, 200);
        let pressed = color.pressed();
        assert_eq!(pressed.r, 70);
        assert_eq!(pressed.g, 120);
        assert_eq!(pressed.b, 170);
    }

    #[test]
    fn test_color_hover() {
        let color = ColorType::rgb(100, 150, 200);
        let hover = color.hover();
        assert_eq!(hover.r, 140);
        assert_eq!(hover.g, 190);
        assert_eq!(hover.b, 240);
    }

    #[test]
    fn test_color_focus() {
        let color = ColorType::rgb(100, 150, 200);
        let focus = color.focus();
        assert_eq!(focus.r, 140);
        assert_eq!(focus.g, 190);
        assert_eq!(focus.b, 240);
    }

    #[test]
    fn test_color_blend() {
        let red = ColorType::RED;
        let blue = ColorType::BLUE;
        let purple = red.blend(blue, 0.5);
        assert_eq!(purple.r, 128);
        assert_eq!(purple.g, 0);
        assert_eq!(purple.b, 128);
    }

    #[test]
    fn test_color_blend_ratio_zero() {
        let red = ColorType::RED;
        let blue = ColorType::BLUE;
        let result = red.blend(blue, 0.0);
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 0);
    }

    #[test]
    fn test_color_blend_ratio_one() {
        let red = ColorType::RED;
        let blue = ColorType::BLUE;
        let result = red.blend(blue, 1.0);
        assert_eq!(result.r, 0);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 255);
    }

    #[test]
    fn test_color_blend_clamps() {
        let red = ColorType::RED;
        let blue = ColorType::BLUE;
        let over = red.blend(blue, 1.5);
        assert_eq!(over.r, 0);
        assert_eq!(over.g, 0);
        assert_eq!(over.b, 255);

        let under = red.blend(blue, -0.5);
        assert_eq!(under.r, 255);
        assert_eq!(under.g, 0);
        assert_eq!(under.b, 0);
    }

    #[test]
    fn test_color_with_interaction_pressed() {
        let color = ColorType::rgb(100, 150, 200);
        let result = color.with_interaction(true, false, false);
        assert_eq!(result, color.pressed());
    }

    #[test]
    fn test_color_with_interaction_hovered() {
        let color = ColorType::rgb(100, 150, 200);
        let result = color.with_interaction(false, true, false);
        assert_eq!(result, color.hover());
    }

    #[test]
    fn test_color_with_interaction_focused() {
        let color = ColorType::rgb(100, 150, 200);
        let result = color.with_interaction(false, false, true);
        assert_eq!(result, color.focus());
    }

    #[test]
    fn test_color_with_interaction_pressed_takes_priority() {
        let color = ColorType::rgb(100, 150, 200);
        let result = color.with_interaction(true, true, true);
        assert_eq!(result, color.pressed());
    }

    #[test]
    fn test_color_with_interaction_none() {
        let color = ColorType::rgb(100, 150, 200);
        let result = color.with_interaction(false, false, false);
        assert_eq!(result, color);
    }

    #[test]
    fn test_color_default_impl() {
        let color = ColorType::default();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 0);
    }

    #[test]
    fn test_color_preserves_alpha_on_darken() {
        let color = ColorType::rgba(100, 150, 200, 128);
        let darker = color.darken(30);
        assert_eq!(darker.a, 128);
    }

    #[test]
    fn test_color_preserves_alpha_on_lighten() {
        let color = ColorType::rgba(100, 150, 200, 128);
        let lighter = color.lighten(30);
        assert_eq!(lighter.a, 128);
    }

    #[test]
    fn test_color_blend_alpha() {
        let opaque = ColorType::rgba(255, 0, 0, 255);
        let transparent = ColorType::rgba(0, 0, 255, 0);
        let blended = opaque.blend(transparent, 0.5);
        assert_eq!(blended.a, 128);
    }
}
