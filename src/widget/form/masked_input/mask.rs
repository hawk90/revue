//! Masking the value for display: the mask styles and the peek
//!
//! The value is masked per character as the user sees it (grapheme
//! cluster), the unit the cursor moves by, so the mask and the cursor
//! moves never disagree: a mask char per char would let the cursor show
//! where a letter with its accent or an emoji sequence is (#1298). GTK's
//! password entry keeps the two in step the same way, in its unit
//! (`gtk_entry_move_logically`, "Prevent any leak of information").

use unicode_segmentation::UnicodeSegmentation;

use super::{MaskStyle, MaskedInput};

impl MaskedInput {
    /// The value as shown: each character (grapheme cluster) the style
    /// hides becomes one mask char
    pub fn masked_display(&self) -> String {
        let value = self.get_value();
        if matches!(self.mask_style, MaskStyle::Hidden) && !self.revealing {
            return String::new();
        }
        let len = self.len();
        let mut out = String::with_capacity(value.len());
        for (i, cluster) in value.graphemes(true).enumerate() {
            if self.shows(i, len) {
                out.push_str(cluster);
            } else {
                out.push(self.mask_char);
            }
        }
        out
    }

    /// Whether character `i` of `len` (grapheme clusters) is shown as it
    /// is rather than masked
    pub(super) fn shows(&self, i: usize, len: usize) -> bool {
        if self.revealing {
            return true;
        }
        match self.mask_style {
            MaskStyle::Full | MaskStyle::Hidden => false,
            MaskStyle::ShowLast(n) => i + n >= len,
            MaskStyle::ShowFirst(n) => i < n,
            // The character just typed, before the cursor, while it lasts
            MaskStyle::Peek => self.peek_countdown > 0 && i + 1 == self.state.cursor().col,
        }
    }

    /// The char index in [`masked_display`](Self::masked_display) of the
    /// position before character `col` (a grapheme cluster column): a
    /// masked character takes one char there, a shown one its own
    pub(super) fn masked_index(&self, col: usize) -> usize {
        let len = self.len();
        self.get_value()
            .graphemes(true)
            .take(col)
            .enumerate()
            .map(|(i, cluster)| {
                if self.shows(i, len) {
                    cluster.chars().count()
                } else {
                    1
                }
            })
            .sum()
    }
}
