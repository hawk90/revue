//! The text an avatar shows: its custom initials, its icon, or two initials
//! taken from the name

use super::Avatar;

impl Avatar {
    /// Get initials from name
    pub(super) fn get_initials(&self) -> String {
        if let Some(ref initials) = self.initials {
            return initials.clone();
        }

        if let Some(icon) = self.icon {
            return icon.to_string();
        }

        // Derive initials from name
        self.name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }
}
