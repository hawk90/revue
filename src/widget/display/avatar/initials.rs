//! The text an avatar shows: its icon, its custom initials, or two initials
//! taken from the name

use super::Avatar;
use crate::utils::unicode::char_width;

impl Avatar {
    /// Get initials from name
    pub(super) fn get_initials(&self) -> String {
        // The icon is "instead of initials", whichever way they were given.
        if let Some(icon) = self.icon {
            return icon.to_string();
        }

        if let Some(ref initials) = self.initials {
            return initials.clone();
        }

        // The first drawn char of the first two words. A cell holds one char,
        // so an accent that follows its letter waits for grapheme cells (#1148).
        self.name
            .split_whitespace()
            .filter_map(|word| word.chars().find(|ch| char_width(*ch) > 0))
            .take(2)
            .map(upper_initial)
            .collect()
    }
}

/// `ch` in upper case, or as written when its upper case is more than one
/// char (`ß` is "SS"): an initial takes one place.
fn upper_initial(ch: char) -> char {
    let mut upper = ch.to_uppercase();
    match (upper.next(), upper.next()) {
        (Some(single), None) => single,
        _ => ch,
    }
}
