//! Password strength and length validation

use super::{MaskedInput, ValidationState};
use crate::style::Color;

impl MaskedInput {
    /// Password strength, 0 to 4: a point each for 8 and 12 characters, mixed
    /// case, a digit and a symbol. Below 8 characters it is 1 at most
    /// (Weak), however many kinds it mixes: 8 is the least NIST SP 800-63B
    /// accepts.
    pub fn password_strength(&self) -> usize {
        let len = self.len();
        let has_lower = self.get_value().chars().any(|c| c.is_lowercase());
        let has_upper = self.get_value().chars().any(|c| c.is_uppercase());
        let has_digit = self.get_value().chars().any(|c| c.is_ascii_digit());
        let has_special = self.get_value().chars().any(|c| !c.is_alphanumeric());

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

        if len < 8 {
            strength.min(1)
        } else {
            strength.min(4)
        }
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
        if self.min_length > 0 && self.len() < self.min_length {
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
