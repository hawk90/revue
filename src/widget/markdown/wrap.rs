//! Breaking a line of styled text into rows of at most a given width, keeping
//! each piece's style: at word boundaries for prose, anywhere for code.

use super::StyledText;
use crate::text::{char_rows, word_rows};
use crate::utils::unicode::display_width;

/// `segments` as rows of at most `width` columns, broken where a line may
/// break (spaces, after hyphens, and between CJK characters or emoji), by
/// the same rules as every other wrapped text (`text::word_rows`, #1474).
/// The whitespace at a break is dropped. A word wider than `width` is
/// broken across rows.
///
/// A `width` of 0, or text that fits, gives the text back as one row.
pub(super) fn wrap_words(segments: &[StyledText], width: usize) -> Vec<Vec<StyledText>> {
    let text: String = segments.iter().map(|s| s.text.as_str()).collect();
    if width == 0 || display_width(&text) <= width {
        return vec![segments.to_vec()];
    }
    slice_rows(segments, word_rows(&text, width, width, true).into_iter())
}

/// `segments` as rows of at most `width` columns, broken at the character
/// that would not fit. Nothing is dropped: the rows joined are the text.
///
/// A `width` of 0, or text that fits, gives the text back as one row.
pub(super) fn wrap_chars(segments: &[StyledText], width: usize) -> Vec<Vec<StyledText>> {
    let text: String = segments.iter().map(|s| s.text.as_str()).collect();
    if width == 0 || display_width(&text) <= width {
        return vec![segments.to_vec()];
    }
    let rows = char_rows(&text, width)
        .into_iter()
        .map(|(from, to, _)| (from, to));
    slice_rows(segments, rows)
}

/// The parts of `segments` between the byte ranges `rows` of their joined
/// text, each piece in its own segment's style. The ranges come in order and
/// do not overlap, so one pass over the segments serves them all: a line of
/// many segments and many rows stays linear.
fn slice_rows(
    segments: &[StyledText],
    rows: impl Iterator<Item = (usize, usize)>,
) -> Vec<Vec<StyledText>> {
    let mut out = Vec::new();
    // The first segment a row may take from, and where it starts
    let (mut index, mut seg_start) = (0, 0);
    for (from, to) in rows {
        let mut row = Vec::new();
        while let Some(segment) = segments.get(index) {
            let seg_end = seg_start + segment.text.len();
            let (start, end) = (from.max(seg_start), to.min(seg_end));
            if start < end {
                row.push(StyledText {
                    text: segment.text[start - seg_start..end - seg_start].to_string(),
                    ..segment.clone()
                });
            }
            if seg_end > to {
                // The next row goes on in this segment
                break;
            }
            index += 1;
            seg_start = seg_end;
        }
        out.push(row);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Modifier;

    fn texts(rows: &[Vec<StyledText>]) -> Vec<String> {
        rows.iter()
            .map(|row| row.iter().map(|s| s.text.as_str()).collect())
            .collect()
    }

    #[test]
    fn words_break_at_spaces_and_drop_them() {
        let rows = wrap_words(&[StyledText::new("one two three four")], 9);
        assert_eq!(texts(&rows), ["one two", "three", "four"]);
    }

    #[test]
    fn a_row_may_be_exactly_the_width() {
        let rows = wrap_words(&[StyledText::new("abcd efgh")], 4);
        assert_eq!(texts(&rows), ["abcd", "efgh"]);
    }

    #[test]
    fn text_that_fits_is_one_row_as_given() {
        let rows = wrap_words(&[StyledText::new(" a  b ")], 6);
        assert_eq!(texts(&rows), [" a  b "]);
        assert_eq!(texts(&wrap_words(&[StyledText::new("abc")], 0)), ["abc"]);
    }

    #[test]
    fn a_word_wider_than_the_row_is_broken() {
        let rows = wrap_words(&[StyledText::new("ab abcdefgh c")], 4);
        assert_eq!(texts(&rows), ["ab", "abcd", "efgh", "c"]);
    }

    #[test]
    fn cjk_breaks_between_characters_by_width() {
        // Each character is two columns
        let rows = wrap_words(&[StyledText::new("가나다라마")], 4);
        assert_eq!(texts(&rows), ["가나", "다라", "마"]);
    }

    #[test]
    fn styles_stay_with_their_text_across_a_break() {
        let segments = [
            StyledText::new("plain "),
            StyledText::new("bold words").with_modifier(Modifier::BOLD),
        ];
        let rows = wrap_words(&segments, 10);
        assert_eq!(texts(&rows), ["plain bold", "words"]);
        assert_eq!(rows[0][0].modifier, Modifier::empty());
        assert_eq!(rows[0][1].text, "bold");
        assert_eq!(rows[0][1].modifier, Modifier::BOLD);
        assert_eq!(rows[1][0].modifier, Modifier::BOLD);
    }

    #[test]
    fn chars_keep_every_character() {
        let rows = wrap_chars(&[StyledText::new("let x  = 1;")], 4);
        assert_eq!(texts(&rows), ["let ", "x  =", " 1;"]);
    }

    #[test]
    fn chars_do_not_split_a_wide_character() {
        let rows = wrap_chars(&[StyledText::new("a가b")], 2);
        assert_eq!(texts(&rows), ["a", "가", "b"]);
        // Wider than the row: on a row of its own
        let rows = wrap_chars(&[StyledText::new("가a")], 1);
        assert_eq!(texts(&rows), ["가", "a"]);
    }
}
