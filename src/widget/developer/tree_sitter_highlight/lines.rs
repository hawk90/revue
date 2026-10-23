//! Spreading the captures of a whole file over its lines

use super::captures::CaptureStyle;
use crate::widget::syntax::HighlightSpan;

/// One span list per line of `code` (as `str::lines` counts and cuts them:
/// a line ends before its `\n` or `\r\n`). A capture that crosses line
/// ends is cut at each one; offsets are characters from the start of the
/// line it lands on, as [`HighlightSpan`] has them; each line's spans are
/// sorted by start.
pub(super) fn split_by_lines(
    code: &str,
    captures: &[(usize, usize, CaptureStyle)],
) -> Vec<Vec<HighlightSpan>> {
    // Each line's byte range, its line end left out
    let mut lines: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;
    for piece in code.split_inclusive('\n') {
        let text = piece
            .strip_suffix('\n')
            .map_or(piece, |t| t.strip_suffix('\r').unwrap_or(t));
        lines.push((start, start + text.len()));
        start += piece.len();
    }

    let mut line_spans: Vec<Vec<HighlightSpan>> = lines.iter().map(|_| Vec::new()).collect();
    // Each line's char starts, made when a capture first lands on it, so a
    // long line with many captures is measured once
    let mut columns: Vec<Option<CharColumns>> = lines.iter().map(|_| None).collect();
    for &(start, end, style) in captures {
        // The first line that ends after the capture starts, and on while
        // the lines start before it ends
        let first = lines.partition_point(|&(_, line_end)| line_end <= start);
        for (i, &(line_start, line_end)) in lines.iter().enumerate().skip(first) {
            if line_start >= end {
                break;
            }
            let (a, b) = (start.max(line_start), end.min(line_end));
            if a < b {
                let cols =
                    columns[i].get_or_insert_with(|| CharColumns::new(&code[line_start..line_end]));
                line_spans[i].push(style.span(cols.at(a - line_start), cols.at(b - line_start)));
            }
        }
    }

    // Sort spans in each line
    for spans in &mut line_spans {
        spans.sort_by_key(|s| s.start);
    }

    line_spans
}

/// Byte offsets to character indices for one line, in a binary search
pub(super) struct CharColumns {
    /// The byte offset where each character starts
    starts: Vec<usize>,
}

impl CharColumns {
    pub(super) fn new(text: &str) -> Self {
        Self {
            starts: text.char_indices().map(|(i, _)| i).collect(),
        }
    }

    /// The character index of byte offset `byte` (the characters that
    /// start before it)
    pub(super) fn at(&self, byte: usize) -> usize {
        self.starts.partition_point(|&s| s < byte)
    }
}
