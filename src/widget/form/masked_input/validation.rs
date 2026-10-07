//! Password strength and length validation

use super::{MaskedInput, ValidationState};
use crate::style::Color;

impl MaskedInput {
    /// Calculate password strength (0-4)
    pub fn password_strength(&self) -> usize {
        let len = self.char_count();
        let has_lower = self.value.chars().any(|c| c.is_lowercase());
        let has_upper = self.value.chars().any(|c| c.is_uppercase());
        let has_digit = self.value.chars().any(|c| c.is_ascii_digit());
        let has_special = self.value.chars().any(|c| !c.is_alphanumeric());

        let mut strength = 0;

        if len >= 8 {
            strength += 1;
        }
        if len >= 12 {
            strength += 1;
        }
        if has_lower && has_upper {
            strength += 1;
        }
        if has_digit {
            strength += 1;
        }
        if has_special {
            strength += 1;
        }

        strength.min(4)
    }

    /// Get strength label
    pub fn strength_label(&self) -> &str {
        match self.password_strength() {
            0 => "Very Weak",
            1 => "Weak",
            2 => "Fair",
            3 => "Strong",
            _ => "Very Strong",
        }
    }

    /// Get strength color
    pub fn strength_color(&self) -> Color {
        match self.password_strength() {
            0 => Color::RED,
            1 => Color::rgb(255, 128, 0), // Orange
            2 => Color::YELLOW,
            3 => Color::rgb(128, 255, 0), // Light green
            _ => Color::GREEN,
        }
    }

    /// Validate the input
    pub fn validate(&mut self) -> bool {
        if self.min_length > 0 && self.char_count() < self.min_length {
            self.validation = ValidationState::Invalid(format!(
                "Minimum {} characters required",
                self.min_length
            ));
            return false;
        }

        self.validation = ValidationState::Valid;
        true
    }
}
