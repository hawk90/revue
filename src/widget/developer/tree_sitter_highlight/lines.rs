//! Spreading the captures of a whole file over its lines

use super::captures::CaptureStyle;
use crate::widget::syntax::HighlightSpan;

/// One span list per line of `code` (as `str::lines` counts them). A capture
/// that crosses line ends is cut at each one, with offsets made relative to
/// the line it lands on; each line's spans are sorted by start.
pub(super) fn split_by_lines(
    code: &str,
    captures: &[(usize, usize, CaptureStyle)],
) -> Vec<Vec<HighlightSpan>> {
    // Build line offset map
    let mut line_offsets: Vec<usize> = vec![0];
    for (i, c) in code.char_indices() {
        if c == '\n' {
            line_offsets.push(i + 1);
        }
    }

    let num_lines = code.lines().count();
    let mut line_spans: Vec<Vec<HighlightSpan>> = (0..num_lines).map(|_| Vec::new()).collect();

    for &(start, end, style) in captures {
        // Find which lines this span covers
        let start_line = line_offsets
            .iter()
            .position(|&offset| offset > start)
            .map(|p| p.saturating_sub(1))
            .unwrap_or(line_offsets.len().saturating_sub(1));
        let end_line = line_offsets
            .iter()
            .position(|&offset| offset > end)
            .map(|p| p.saturating_sub(1))
            .unwrap_or(line_offsets.len().saturating_sub(1));

        for line_idx in start_line..=end_line {
            if line_idx >= num_lines {
                break;
            }

            let line_start = line_offsets[line_idx];
            let line_end = line_offsets
                .get(line_idx + 1)
                .map(|&o| o.saturating_sub(1))
                .unwrap_or(code.len());

            let span_start_in_line = start.saturating_sub(line_start);
            let span_end_in_line = end.min(line_end).saturating_sub(line_start);

            if span_start_in_line < span_end_in_line {
                line_spans[line_idx].push(style.span(span_start_in_line, span_end_in_line));
            }
        }
    }

    // Sort spans in each line
    for spans in &mut line_spans {
        spans.sort_by_key(|s| s.start);
    }

    line_spans
}
