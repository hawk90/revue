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
        self.with_alpha((alpha.clamp(0.0, 1.0) * 255.0) as u8)
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
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
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
        let factor = pct.clamp(0.0, 1.0);
        Self {
            r: (self.r as f32 + (255.0 - self.r as f32) * factor) as u8,
            g: (self.g as f32 + (255.0 - self.g as f32) * factor) as u8,
            b: (self.b as f32 + (255.0 - self.b as f32) * factor) as u8,
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
            r: (self.r as f32 * inv + other.r as f32 * ratio) as u8,
            g: (self.g as f32 * inv + other.g as f32 * ratio) as u8,
            b: (self.b as f32 * inv + other.b as f32 * ratio) as u8,
            a: (self.a as f32 * inv + other.a as f32 * ratio) as u8,
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
