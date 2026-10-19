//! Text highlighting utilities tests

use revue::style::Color;
use revue::utils::highlight::{
    highlight_matches, highlight_range, highlight_ranges, highlight_substring, HighlightSpan,
    Highlighter,
};

#[test]
fn test_highlight_matches_basic() {
    let spans = highlight_matches("Hello", &[0, 4]);

    // "H" (highlighted), "ell" (normal), "o" (highlighted)
    assert_eq!(spans.len(), 3);
    assert!(spans[0].highlighted);
    assert_eq!(spans[0].text, "H");
    assert!(!spans[1].highlighted);
    assert_eq!(spans[1].text, "ell");
    assert!(spans[2].highlighted);
    assert_eq!(spans[2].text, "o");
}

#[test]
fn test_highlight_matches_empty() {
    let spans = highlight_matches("Hello", &[]);
    assert_eq!(spans.len(), 1);
    assert!(!spans[0].highlighted);
    assert_eq!(spans[0].text, "Hello");
}

#[test]
fn test_highlight_matches_consecutive() {
    let spans = highlight_matches("Hello", &[0, 1, 2]);

    assert_eq!(spans.len(), 2);
    assert!(spans[0].highlighted);
    assert_eq!(spans[0].text, "Hel");
    assert!(!spans[1].highlighted);
    assert_eq!(spans[1].text, "lo");
}

#[test]
fn test_highlight_substring() {
    let spans = highlight_substring("Hello World", "World");

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].text, "Hello ");
    assert!(!spans[0].highlighted);
    assert_eq!(spans[1].text, "World");
    assert!(spans[1].highlighted);
}

#[test]
fn test_highlight_substring_case_insensitive() {
    let spans = highlight_substring("Hello WORLD", "world");

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[1].text, "WORLD");
    assert!(spans[1].highlighted);
}

#[test]
fn test_highlight_substring_multiple() {
    let spans = highlight_substring("foo bar foo", "foo");

    assert_eq!(spans.len(), 3);
    assert!(spans[0].highlighted);
    assert!(!spans[1].highlighted);
    assert!(spans[2].highlighted);
}

#[test]
fn test_highlight_range() {
    let spans = highlight_range("Hello World", 6, 11);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].text, "Hello ");
    assert_eq!(spans[1].text, "World");
    assert!(spans[1].highlighted);
}

#[test]
fn test_highlight_ranges_merge() {
    let spans = highlight_ranges("Hello World", &[(0, 3), (2, 5)]);

    // Should merge overlapping ranges
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].text, "Hello");
    assert!(spans[0].highlighted);
}

#[test]
fn test_highlighter_colors() {
    let h = Highlighter::yellow();

    let span_hi = HighlightSpan::highlighted("test", 0, 4);
    let span_no = HighlightSpan::normal("test", 0, 4);

    assert_eq!(h.fg_for(&span_hi), Some(Color::BLACK));
    assert_eq!(h.bg_for(&span_hi), Some(Color::YELLOW));
    assert_eq!(h.fg_for(&span_no), None);
}

#[test]
fn test_unicode_highlighting() {
    let spans = highlight_matches("안녕하세요", &[0, 2]);

    assert!(spans[0].highlighted); // 안
    assert!(!spans[1].highlighted); // 녕
    assert!(spans[2].highlighted); // 하
}

// =========================================================================
// Chars whose lowercase has another length (#875)
// =========================================================================

#[test]
fn test_highlight_substring_after_a_char_that_lowercases_to_two() {
    // `İ` is 2 bytes and lowercases to 3: the match is mapped back
    let spans = highlight_substring("İx", "x");
    assert_eq!(
        spans,
        vec![
            HighlightSpan::normal("İ", 0, 2),
            HighlightSpan::highlighted("x", 2, 3)
        ]
    );
    let spans = highlight_substring("İ", "i");
    assert_eq!(spans, vec![HighlightSpan::highlighted("İ", 0, 2)]);
}

#[test]
fn test_highlight_substring_after_the_kelvin_sign() {
    // The Kelvin sign is 3 bytes and lowercases to the 1-byte `k`
    let spans = highlight_substring("\u{212A}b", "b");
    assert_eq!(
        spans,
        vec![
            HighlightSpan::normal("\u{212A}", 0, 3),
            HighlightSpan::highlighted("b", 3, 4)
        ]
    );
}

mod props {
    use proptest::prelude::*;
    use revue::utils::highlight::highlight_substring_case;

    fn text() -> impl Strategy<Value = String> {
        prop::collection::vec(
            prop::sample::select(vec!['İ', '\u{212A}', 'ß', '한', 'i', 'k', 'x', 'X', ' ']),
            0..12,
        )
        .prop_map(String::from_iter)
    }

    proptest! {
        /// The spans tile the text in order, each one its own slice of it
        #[test]
        fn prop_highlight_spans_tile_the_text(
            text in text(),
            pattern in text(),
            case_sensitive in any::<bool>(),
        ) {
            let spans = highlight_substring_case(&text, &pattern, case_sensitive);
            let mut at = 0;
            for span in &spans {
                prop_assert_eq!(span.start, at);
                prop_assert_eq!(&span.text, &text[span.start..span.end]);
                at = span.end;
            }
            prop_assert_eq!(at, text.len());
        }
    }
}
