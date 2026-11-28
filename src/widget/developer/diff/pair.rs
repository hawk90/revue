//! Pairing the removed and added lines of a change into modified lines, and
//! the words that changed between the two lines of a pair
//!
//! The pairing follows delta's homologous-line inference (`infer_edits` in
//! delta's `src/edits.rs`): each removed line, in order, is paired with the
//! first added line after the last pair that is within
//! [`MAX_LINE_DISTANCE`] of it; the added lines skipped on the way stay
//! added. Pairs therefore never cross.
//!
//! The changed words of a pair are then drawn as delta draws them: blanks
//! that sit between changed words on both lines join the change, so
//! `foo bar` to `baz qux` is one changed run on each side, not two with a
//! shared space between (delta's `coalesce_space_with_previous` in
//! `annotate`). Blanks at the start or end of a line, or next to shared
//! text on either side, stay shared.

use similar::{ChangeTag, DiffTag, InlineChangeMode, InlineChangeOptions, TextDiff};
use std::ops::Range;

use crate::utils::display_width;

/// The largest distance (0.0 to 1.0) between a removed and an added line
/// for them to be one modified line: delta's `--max-line-distance` default
pub(super) const MAX_LINE_DISTANCE: f64 = 0.6;

/// The line comparisons one change may make before its remaining lines stay
/// removed and added
///
/// A guard of ours (delta has none): pairing is quadratic in the lines of a
/// change, and each comparison diffs the words of two lines, so a change of
/// thousands of unlike lines would otherwise take seconds to lay out. A
/// change of up to 100 removed and 100 added lines always pairs in full.
pub(super) const MAX_COMPARISONS: usize = 10_000;

/// The two lines of a modified pair cut into runs, each a byte range of its
/// line and whether it changed
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Segments {
    /// Runs of the old (left) line
    pub old: Vec<(bool, Range<usize>)>,
    /// Runs of the new (right) line
    pub new: Vec<(bool, Range<usize>)>,
}

/// A run of an inline row: text both lines share, or text one of them has
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Part {
    /// In both lines
    Equal,
    /// Only in the old line
    Removed,
    /// Only in the new line
    Added,
}

/// The words of `old` and `new` that differ, by `similar`'s inline changes
/// over Unicode word boundaries (a run of letters or digits is one token,
/// any other character its own: the same tokens as delta's default
/// `--word-diff-regex='\w+'`)
pub(super) fn segments(old: &str, new: &str) -> Segments {
    let diff = TextDiff::from_lines(old, new);
    let mut options = InlineChangeOptions::new();
    // Always refine: whether the lines are alike is the pairing's call
    options.mode(InlineChangeMode::UnicodeWords).min_ratio(0.0);

    let mut segments = Segments::default();
    let (mut old_at, mut new_at) = (0, 0);
    for op in diff.ops() {
        // Only a replaced line has words marked; a line only one side has
        // changed as a whole
        let whole = match op.tag() {
            DiffTag::Equal => Some(false),
            DiffTag::Delete | DiffTag::Insert => Some(true),
            DiffTag::Replace => None,
        };
        for change in diff.iter_inline_changes_with_options(op, options) {
            for &(emphasized, value) in change.values() {
                let changed = whole.unwrap_or(emphasized);
                match change.tag() {
                    ChangeTag::Delete => push(&mut segments.old, &mut old_at, changed, value.len()),
                    ChangeTag::Insert => push(&mut segments.new, &mut new_at, changed, value.len()),
                    ChangeTag::Equal => {
                        push(&mut segments.old, &mut old_at, false, value.len());
                        push(&mut segments.new, &mut new_at, false, value.len());
                    }
                }
            }
        }
    }
    segments
}

/// Append `len` bytes to `runs`, merged into the last run when it has the
/// same flag
fn push(runs: &mut Vec<(bool, Range<usize>)>, at: &mut usize, changed: bool, len: usize) {
    if len == 0 {
        return;
    }
    let end = *at + len;
    match runs.last_mut() {
        Some((flag, range)) if *flag == changed => range.end = end,
        _ => runs.push((changed, *at..end)),
    }
    *at = end;
}

/// How far apart two lines are, 0.0 (alike) to 1.0: delta's `annotate`
/// distance, the columns of changed text over the columns of changed text
/// plus twice the shared text, each run counted without the blanks at its
/// ends. Two lines with no such columns are 0.0, as in delta.
pub(super) fn distance(old: &str, new: &str, segments: &Segments) -> f64 {
    let width = |line: &str, range: &Range<usize>| display_width(line[range.clone()].trim());
    let (mut changed, mut total) = (0, 0);
    for (flag, range) in &segments.old {
        let w = width(old, range);
        if *flag {
            changed += w;
            total += w;
        } else {
            total += 2 * w;
        }
    }
    for (flag, range) in &segments.new {
        if *flag {
            let w = width(new, range);
            changed += w;
            total += w;
        }
    }
    if total == 0 {
        0.0
    } else {
        changed as f64 / total as f64
    }
}

/// One row of a change after pairing: indices into the removed and added
/// lines, both set for a modified pair
pub(super) type Paired = (Option<usize>, Option<usize>, Option<Segments>);

/// Pair the `removed` and `added` lines of one change, in order: delta's
/// greedy scan. Each removed line takes the first added line after the
/// last pair within [`MAX_LINE_DISTANCE`]; added lines passed over come out
/// before the pair, unpaired removed lines where they are, the rest of the
/// added lines at the end.
pub(super) fn pair(removed: &[&str], added: &[&str]) -> Vec<Paired> {
    let mut rows = Vec::with_capacity(removed.len() + added.len());
    let mut next_added = 0;
    let mut comparisons = 0;
    for (r, old) in removed.iter().enumerate() {
        let mut found = None;
        for (a, new) in added.iter().enumerate().skip(next_added) {
            if comparisons >= MAX_COMPARISONS {
                break;
            }
            comparisons += 1;
            let segments = segments(old, new);
            // The distance counts the words as found, before the blanks
            // between them join; the blanks have no columns in it anyway
            if distance(old, new, &segments) <= MAX_LINE_DISTANCE {
                found = Some((a, join_blanks(old, &segments)));
                break;
            }
        }
        match found {
            Some((a, segments)) => {
                rows.extend((next_added..a).map(|skipped| (None, Some(skipped), None)));
                rows.push((Some(r), Some(a), Some(segments)));
                next_added = a + 1;
            }
            None => rows.push((Some(r), None, None)),
        }
    }
    rows.extend((next_added..added.len()).map(|a| (None, Some(a), None)));
    rows
}

/// A step of the walk over both lines of a pair: text both share (a byte
/// range of each line), or text one of them has
#[derive(Clone, Debug, PartialEq, Eq)]
enum Step {
    /// The same text in both lines
    Equal(Range<usize>, Range<usize>),
    /// A changed run of the old line
    Removed(Range<usize>),
    /// A changed run of the new line
    Added(Range<usize>),
}

/// Walk the runs of both lines in reading order: where the lines differ
/// the old runs and after them the new, then the shared text up to where
/// either side changes again
fn steps(segments: &Segments) -> Vec<Step> {
    let mut steps = Vec::new();
    let (mut o, mut n) = (
        segments.old.iter().peekable(),
        segments.new.iter().peekable(),
    );
    // Byte offsets into the current shared run of each side
    let (mut o_at, mut n_at) = (None::<usize>, None::<usize>);
    loop {
        while let Some((true, range)) = o.peek() {
            steps.push(Step::Removed(range.clone()));
            o.next();
        }
        while let Some((true, range)) = n.peek() {
            steps.push(Step::Added(range.clone()));
            n.next();
        }
        // Both sides are at shared text, which is the same on both: take
        // what both runs still have, then move past the shorter run
        match (o.peek(), n.peek()) {
            (Some((false, or)), Some((false, nr))) => {
                let os = o_at.unwrap_or(or.start);
                let ns = n_at.unwrap_or(nr.start);
                let len = (or.end - os).min(nr.end - ns);
                steps.push(Step::Equal(os..os + len, ns..ns + len));
                o_at = Some(os + len);
                n_at = Some(ns + len);
                if os + len == or.end {
                    o.next();
                    o_at = None;
                }
                if ns + len == nr.end {
                    n.next();
                    n_at = None;
                }
            }
            // Shared text on one side only cannot happen; show what is left
            (Some((_, range)), None) => {
                steps.push(Step::Removed(o_at.unwrap_or(range.start)..range.end));
                o.next();
                o_at = None;
            }
            (None, Some((_, range))) => {
                steps.push(Step::Added(n_at.unwrap_or(range.start)..range.end));
                n.next();
                n_at = None;
            }
            _ => break,
        }
    }
    steps
}

/// The segments with each blank shared run that has changed text on both
/// lines right before it and right after it made part of the change, as
/// delta does: the space in `foo bar` to `baz qux` is not shared text
/// between two changes but part of one
fn join_blanks(old: &str, segments: &Segments) -> Segments {
    let steps = steps(segments);
    // Whether the steps between two shared ones change both lines
    let both_change = |between: &[Step]| {
        between.iter().any(|s| matches!(s, Step::Removed(_)))
            && between.iter().any(|s| matches!(s, Step::Added(_)))
    };
    let shared: Vec<usize> = (0..steps.len())
        .filter(|&i| matches!(steps[i], Step::Equal(..)))
        .collect();
    let mut joined = Segments::default();
    let (mut old_at, mut new_at) = (0, 0);
    for (i, step) in steps.iter().enumerate() {
        match step {
            Step::Removed(range) => push(&mut joined.old, &mut old_at, true, range.len()),
            Step::Added(range) => push(&mut joined.new, &mut new_at, true, range.len()),
            Step::Equal(or, nr) => {
                let k = shared.partition_point(|&s| s < i);
                let before = shared.get(k.wrapping_sub(1)).map_or(0, |&s| s + 1);
                let after = shared.get(k + 1).copied().unwrap_or(steps.len());
                let blank = old[or.clone()].chars().all(char::is_whitespace);
                let changed =
                    blank && both_change(&steps[before..i]) && both_change(&steps[i + 1..after]);
                push(&mut joined.old, &mut old_at, changed, or.len());
                push(&mut joined.new, &mut new_at, changed, nr.len());
            }
        }
    }
    joined
}

/// The runs of an inline row for a modified pair, in reading order: shared
/// text once, then where the lines differ the old text and after it the
/// new, as `git diff --word-diff` writes `[-old-]{+new+}`
pub(super) fn inline_parts<'a>(
    old: &'a str,
    new: &'a str,
    segments: &Segments,
) -> Vec<(Part, &'a str)> {
    steps(segments)
        .into_iter()
        .map(|step| match step {
            Step::Equal(range, _) => (Part::Equal, &old[range]),
            Step::Removed(range) => (Part::Removed, &old[range]),
            Step::Added(range) => (Part::Added, &new[range]),
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    /// A blank shared run joins only with changes on both lines on both
    /// sides of it: here the old line changes after it but the new line
    /// goes on with shared text, so it stays shared on both
    #[test]
    fn blank_next_to_shared_text_on_one_line_stays_shared() {
        // Old "a bc" as [a]{ }[b]{c}, new "x c" as [x]{ c}: after the
        // blank only the old line changes. Made up: similar puts such a
        // blank in the change, but the runs could come out this way.
        let segments = Segments {
            old: vec![(true, 0..1), (false, 1..2), (true, 2..3), (false, 3..4)],
            new: vec![(true, 0..1), (false, 1..3)],
        };
        assert_eq!(join_blanks("a bc", &segments), segments);

        // Old "a b" as [a]{ }[b], new "xy z" as [xy]{ }[z]: the blank joins
        let segments = Segments {
            old: vec![(true, 0..1), (false, 1..2), (true, 2..3)],
            new: vec![(true, 0..2), (false, 2..3), (true, 3..4)],
        };
        let joined = Segments {
            old: vec![(true, 0..3)],
            new: vec![(true, 0..4)],
        };
        assert_eq!(join_blanks("a b", &segments), joined);
    }
}
