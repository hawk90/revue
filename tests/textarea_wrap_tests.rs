//! Soft-wrap rendering tests for TextArea.
//!
//! Regression coverage for the previously-stubbed `wrap` flag: with `wrap(true)`
//! a logical line longer than the viewport width must flow onto additional visual
//! rows instead of being truncated, and `wrap(false)` must keep the original
//! single-row (horizontally-scrolled) behavior.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::TextArea;

/// Render a TextArea into a fresh `w x h` buffer.
fn render(ta: &TextArea, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let area = Rect::new(0, 0, w, h);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ta.render(&mut ctx);
    buffer
}

/// Read visual row `y` as a string with trailing blanks trimmed.
fn row(buffer: &Buffer, y: u16, w: u16) -> String {
    let mut s = String::new();
    for x in 0..w {
        if let Some(cell) = buffer.get(x, y) {
            s.push(cell.symbol);
        }
    }
    s.trim_end().to_string()
}

#[test]
fn wrap_true_flows_long_line_onto_multiple_rows() {
    let ta = TextArea::new()
        .line_numbers(false)
        .wrap(true)
        .content("abcdefghij"); // 10 chars, width 5 -> two rows
    let buf = render(&ta, 5, 5);

    assert_eq!(row(&buf, 0, 5), "abcde");
    assert_eq!(row(&buf, 1, 5), "fghij");
    // Nothing lost, nothing on a third row.
    assert_eq!(row(&buf, 2, 5), "");
}

#[test]
fn wrap_false_truncates_to_single_row() {
    let ta = TextArea::new()
        .line_numbers(false)
        .wrap(false)
        .content("abcdefghij");
    let buf = render(&ta, 5, 5);

    // Single logical line stays on one row (truncated); no continuation row.
    assert_eq!(row(&buf, 0, 5), "abcde");
    assert_eq!(row(&buf, 1, 5), "");
}

#[test]
fn wrap_true_breaks_at_word_boundaries() {
    let ta = TextArea::new()
        .line_numbers(false)
        .wrap(true)
        .content("aa bb cc"); // width 3 -> "aa" / "bb" / "cc"
    let buf = render(&ta, 3, 5);

    assert_eq!(row(&buf, 0, 3), "aa");
    assert_eq!(row(&buf, 1, 3), "bb");
    assert_eq!(row(&buf, 2, 3), "cc");
}

#[test]
fn wrap_true_preserves_logical_newlines() {
    let ta = TextArea::new()
        .line_numbers(false)
        .wrap(true)
        .content("hello world\nfoo"); // line 0 wraps at width 5, line 1 short
    let buf = render(&ta, 5, 5);

    assert_eq!(row(&buf, 0, 5), "hello");
    assert_eq!(row(&buf, 1, 5), "world");
    assert_eq!(row(&buf, 2, 5), "foo");
}

#[test]
fn wrap_true_line_numbers_only_on_first_visual_row() {
    // Line numbers occupy a gutter; a wrapped continuation row must not repeat the number.
    let ta = TextArea::new()
        .line_numbers(true)
        .wrap(true)
        .content("abcdefgh"); // one logical line, wraps across rows
    let buf = render(&ta, 8, 5);

    // Gutter for a single line is "1" right-aligned in (digits+2)=3 cols: "1 " + separator.
    let r0 = row(&buf, 0, 8);
    let r1 = row(&buf, 1, 8);
    assert!(
        r0.trim_start().starts_with('1'),
        "row0 shows line number: {r0:?}"
    );
    // Continuation row must NOT start with the digit '1' in the gutter.
    assert!(
        !r1.trim_start().starts_with('1'),
        "continuation row must not repeat the line number: {r1:?}"
    );
    // But it must still contain wrapped text.
    assert!(!r1.is_empty(), "continuation row should carry wrapped text");
}
