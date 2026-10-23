//! One line of text measured in grapheme clusters: columns, bytes, screen
//! columns, word categories and the code points Backspace takes

use unicode_segmentation::UnicodeSegmentation;

use crate::utils::unicode::display_width;

/// The clusters in `line`
pub(super) fn cluster_count(line: &str) -> usize {
    line.graphemes(true).count()
}

/// The byte offset where cluster `col` starts (the line's length past the
/// last one)
pub(super) fn byte_of(line: &str, col: usize) -> usize {
    line.grapheme_indices(true)
        .nth(col)
        .map_or(line.len(), |(i, _)| i)
}

/// The column of byte offset `byte`: the clusters that start before it, so
/// an offset inside a cluster rounds up to the cluster's end
pub(super) fn col_of(line: &str, byte: usize) -> usize {
    line.grapheme_indices(true)
        .take_while(|&(i, _)| i < byte)
        .count()
}

/// The cluster at column `col`
pub(super) fn cluster_at(line: &str, col: usize) -> Option<&str> {
    line.graphemes(true).nth(col)
}

/// The screen column cluster `col` starts at
pub(super) fn screen_col(line: &str, col: usize) -> usize {
    line.graphemes(true).take(col).map(display_width).sum()
}

/// The column whose cluster covers screen column `x`, or the line's end
/// when the line is narrower
pub(super) fn col_at_screen(line: &str, x: usize) -> usize {
    let mut left = 0;
    for (col, cluster) in line.graphemes(true).enumerate() {
        let width = display_width(cluster);
        if x < left + width.max(1) {
            return col;
        }
        left += width;
    }
    cluster_count(line)
}

/// What a cluster is for word moves and word deletes, by its first char
/// (as CodeMirror's `charCategorizer`)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Category {
    /// A letter or number in any script, or `_`
    Word,
    Space,
    Other,
}

pub(super) fn category(cluster: &str) -> Category {
    match cluster.chars().next() {
        Some(c) if c.is_whitespace() => Category::Space,
        Some(c) if c.is_alphabetic() || c.is_numeric() || c == '_' => Category::Word,
        _ => Category::Other,
    }
}

/// The bytes Backspace removes from the end of `cluster`: the whole of an
/// emoji sequence or of a one-code-point cluster, otherwise its last code
/// point (a dependent vowel, a Hangul jamo, a combining mark)
pub(super) fn backspace_len(cluster: &str) -> usize {
    let mut chars = cluster.chars();
    let last = chars.next_back().map_or(0, char::len_utf8);
    if chars.as_str().is_empty() || cluster.chars().any(is_emoji_joiner) {
        cluster.len()
    } else {
        last
    }
}

/// A code point that makes a cluster an emoji sequence: ZWJ, the emoji
/// variation selector, the keycap mark, a regional indicator (flags), a skin
/// tone modifier, or a tag character (subdivision flags)
fn is_emoji_joiner(c: char) -> bool {
    matches!(
        c,
        '\u{200D}'
            | '\u{FE0F}'
            | '\u{20E3}'
            | '\u{1F1E6}'..='\u{1F1FF}'
            | '\u{1F3FB}'..='\u{1F3FF}'
            | '\u{E0020}'..='\u{E007F}'
    )
}
