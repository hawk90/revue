//! Text difference utilities
//!
//! Provides utilities for comparing text and generating diffs.
//! Useful for showing changes in TextArea, RichLog, and version comparisons.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::diff::{diff_lines, diff_chars, DiffOp};
//!
//! let old = "Hello\nWorld";
//! let new = "Hello\nRust";
//!
//! for change in diff_lines(old, new) {
//!     match change.op {
//!         DiffOp::Equal => println!("  {}", change.text),
//!         DiffOp::Insert => println!("+ {}", change.text),
//!         DiffOp::Delete => println!("- {}", change.text),
//!     }
//! }
//! ```

/// Type of diff operation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffOp {
    /// Text is the same in both versions
    Equal,
    /// Text was inserted (only in new)
    Insert,
    /// Text was deleted (only in old)
    Delete,
}

/// A single change in a diff
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffChange {
    /// The operation type
    pub op: DiffOp,
    /// The text content
    pub text: String,
    /// Line number in old text (for line diffs)
    pub old_line: Option<usize>,
    /// Line number in new text (for line diffs)
    pub new_line: Option<usize>,
}

impl DiffChange {
    /// Create a new diff change
    pub fn new(op: DiffOp, text: impl Into<String>) -> Self {
        Self {
            op,
            text: text.into(),
            old_line: None,
            new_line: None,
        }
    }

    /// Create an equal change
    pub fn equal(text: impl Into<String>) -> Self {
        Self::new(DiffOp::Equal, text)
    }

    /// Create an insert change
    pub fn insert(text: impl Into<String>) -> Self {
        Self::new(DiffOp::Insert, text)
    }

    /// Create a delete change
    pub fn delete(text: impl Into<String>) -> Self {
        Self::new(DiffOp::Delete, text)
    }

    /// Set line numbers
    pub fn with_lines(mut self, old: Option<usize>, new: Option<usize>) -> Self {
        self.old_line = old;
        self.new_line = new;
        self
    }

    /// Check if this is an equal operation
    pub fn is_equal(&self) -> bool {
        self.op == DiffOp::Equal
    }

    /// Check if this is an insert operation
    pub fn is_insert(&self) -> bool {
        self.op == DiffOp::Insert
    }

    /// Check if this is a delete operation
    pub fn is_delete(&self) -> bool {
        self.op == DiffOp::Delete
    }
}

/// Compute line-by-line diff between two texts
///
/// Uses the Myers diff algorithm for efficient comparison.
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::diff::diff_lines;
///
/// let changes = diff_lines("a\nb\nc", "a\nx\nc");
/// // Equal("a"), Delete("b"), Insert("x"), Equal("c")
/// ```
pub fn diff_lines(old: &str, new: &str) -> Vec<DiffChange> {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    diff_sequences(&old_lines, &new_lines)
}

/// Compute character-by-character diff between two strings
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::diff::diff_chars;
///
/// let changes = diff_chars("hello", "hallo");
/// // Equal("h"), Delete("e"), Insert("a"), Equal("llo")
/// ```
pub fn diff_chars(old: &str, new: &str) -> Vec<DiffChange> {
    // Use LCS-based approach for characters
    diff_chars_simple(old, new)
}

/// Simple character diff using LCS
fn diff_chars_simple(old: &str, new: &str) -> Vec<DiffChange> {
    let old_chars: Vec<char> = old.chars().collect();
    let new_chars: Vec<char> = new.chars().collect();

    let lcs = longest_common_subsequence(&old_chars, &new_chars);

    let mut changes = Vec::new();
    let mut old_idx = 0;
    let mut new_idx = 0;
    let mut lcs_idx = 0;

    while old_idx < old_chars.len() || new_idx < new_chars.len() {
        if lcs_idx < lcs.len() {
            let (lcs_old, lcs_new) = lcs[lcs_idx];

            // Deletions from old
            while old_idx < lcs_old {
                changes.push(DiffChange::delete(old_chars[old_idx].to_string()));
                old_idx += 1;
            }

            // Insertions to new
            while new_idx < lcs_new {
                changes.push(DiffChange::insert(new_chars[new_idx].to_string()));
                new_idx += 1;
            }

            // Equal character
            changes.push(DiffChange::equal(old_chars[old_idx].to_string()));
            old_idx += 1;
            new_idx += 1;
            lcs_idx += 1;
        } else {
            // Remaining deletions
            while old_idx < old_chars.len() {
                changes.push(DiffChange::delete(old_chars[old_idx].to_string()));
                old_idx += 1;
            }

            // Remaining insertions
            while new_idx < new_chars.len() {
                changes.push(DiffChange::insert(new_chars[new_idx].to_string()));
                new_idx += 1;
            }
        }
    }

    // Merge consecutive same-op changes
    merge_changes(changes)
}

/// Compute LCS indices
fn longest_common_subsequence<T: PartialEq>(a: &[T], b: &[T]) -> Vec<(usize, usize)> {
    let m = a.len();
    let n = b.len();

    if m == 0 || n == 0 {
        return vec![];
    }

    // Build LCS length table
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Backtrack to find LCS indices
    let mut result = Vec::new();
    let mut i = m;
    let mut j = n;

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push((i - 1, j - 1));
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.reverse();
    result
}

/// Diff sequences using LCS
fn diff_sequences(old: &[&str], new: &[&str]) -> Vec<DiffChange> {
    let lcs = longest_common_subsequence(old, new);

    let mut changes = Vec::new();
    let mut old_idx = 0;
    let mut new_idx = 0;
    let mut old_line = 1;
    let mut new_line = 1;

    for (lcs_old, lcs_new) in lcs {
        // Deletions from old
        while old_idx < lcs_old {
            changes.push(
                DiffChange::delete(old[old_idx])
                    .with_lines(Some(old_line), None)
            );
            old_idx += 1;
            old_line += 1;
        }

        // Insertions to new
        while new_idx < lcs_new {
            changes.push(
                DiffChange::insert(new[new_idx])
                    .with_lines(None, Some(new_line))
            );
            new_idx += 1;
            new_line += 1;
        }

        // Equal line
        changes.push(
            DiffChange::equal(old[old_idx])
                .with_lines(Some(old_line), Some(new_line))
        );
        old_idx += 1;
        new_idx += 1;
        old_line += 1;
        new_line += 1;
    }

    // Remaining deletions
    while old_idx < old.len() {
        changes.push(
            DiffChange::delete(old[old_idx])
                .with_lines(Some(old_line), None)
        );
        old_idx += 1;
        old_line += 1;
    }

    // Remaining insertions
    while new_idx < new.len() {
        changes.push(
            DiffChange::insert(new[new_idx])
                .with_lines(None, Some(new_line))
        );
        new_idx += 1;
        new_line += 1;
    }

    changes
}

/// Merge consecutive changes with the same operation
fn merge_changes(changes: Vec<DiffChange>) -> Vec<DiffChange> {
    if changes.is_empty() {
        return changes;
    }

    let mut merged = Vec::new();
    let mut current = changes[0].clone();

    for change in changes.into_iter().skip(1) {
        if change.op == current.op {
            current.text.push_str(&change.text);
        } else {
            merged.push(current);
            current = change;
        }
    }
    merged.push(current);

    merged
}

/// Compute word-by-word diff between two texts
pub fn diff_words(old: &str, new: &str) -> Vec<DiffChange> {
    let old_words: Vec<&str> = old.split_whitespace().collect();
    let new_words: Vec<&str> = new.split_whitespace().collect();

    diff_sequences(&old_words, &new_words)
}

/// Format diff as unified diff format
pub fn format_unified_diff(old: &str, new: &str, old_name: &str, new_name: &str) -> String {
    let changes = diff_lines(old, new);
    let mut output = String::new();

    output.push_str(&format!("--- {}\n", old_name));
    output.push_str(&format!("+++ {}\n", new_name));

    // Find hunks (groups of changes with context)
    let mut i = 0;
    while i < changes.len() {
        // Skip equal lines until we find a change
        if changes[i].is_equal() {
            i += 1;
            continue;
        }

        // Found a change, start a hunk
        let hunk_start = i.saturating_sub(3); // 3 lines of context before
        let mut hunk_end = i;

        // Find end of hunk
        while hunk_end < changes.len() {
            if changes[hunk_end].is_equal() {
                // Check if there are more changes after context
                let context_end = (hunk_end + 3).min(changes.len());
                let has_more_changes = changes[hunk_end..context_end]
                    .iter()
                    .any(|c| !c.is_equal());
                if !has_more_changes {
                    hunk_end = context_end.min(changes.len());
                    break;
                }
            }
            hunk_end += 1;
        }

        // Output hunk header
        let old_start = changes[hunk_start].old_line.unwrap_or(1);
        let new_start = changes[hunk_start].new_line.unwrap_or(1);
        let old_count = changes[hunk_start..hunk_end]
            .iter()
            .filter(|c| !c.is_insert())
            .count();
        let new_count = changes[hunk_start..hunk_end]
            .iter()
            .filter(|c| !c.is_delete())
            .count();

        output.push_str(&format!(
            "@@ -{},{} +{},{} @@\n",
            old_start, old_count, new_start, new_count
        ));

        // Output hunk lines
        for change in &changes[hunk_start..hunk_end] {
            let prefix = match change.op {
                DiffOp::Equal => ' ',
                DiffOp::Insert => '+',
                DiffOp::Delete => '-',
            };
            output.push(prefix);
            output.push_str(&change.text);
            output.push('\n');
        }

        i = hunk_end;
    }

    output
}

/// Get statistics about a diff
#[derive(Clone, Debug, Default)]
pub struct DiffStats {
    /// Number of equal items
    pub equal: usize,
    /// Number of insertions
    pub insertions: usize,
    /// Number of deletions
    pub deletions: usize,
}

impl DiffStats {
    /// Calculate stats from changes
    pub fn from_changes(changes: &[DiffChange]) -> Self {
        let mut stats = Self::default();
        for change in changes {
            match change.op {
                DiffOp::Equal => stats.equal += 1,
                DiffOp::Insert => stats.insertions += 1,
                DiffOp::Delete => stats.deletions += 1,
            }
        }
        stats
    }

    /// Total number of changes (insertions + deletions)
    pub fn total_changes(&self) -> usize {
        self.insertions + self.deletions
    }

    /// Similarity ratio (0.0 to 1.0)
    pub fn similarity(&self) -> f64 {
        let total = self.equal + self.insertions + self.deletions;
        if total == 0 {
            1.0
        } else {
            self.equal as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_lines_equal() {
        let changes = diff_lines("a\nb\nc", "a\nb\nc");
        assert_eq!(changes.len(), 3);
        assert!(changes.iter().all(|c| c.is_equal()));
    }

    #[test]
    fn test_diff_lines_insert() {
        let changes = diff_lines("a\nc", "a\nb\nc");
        assert_eq!(changes.len(), 3);
        assert!(changes[0].is_equal());
        assert!(changes[1].is_insert());
        assert_eq!(changes[1].text, "b");
        assert!(changes[2].is_equal());
    }

    #[test]
    fn test_diff_lines_delete() {
        let changes = diff_lines("a\nb\nc", "a\nc");
        assert_eq!(changes.len(), 3);
        assert!(changes[0].is_equal());
        assert!(changes[1].is_delete());
        assert_eq!(changes[1].text, "b");
        assert!(changes[2].is_equal());
    }

    #[test]
    fn test_diff_lines_replace() {
        let changes = diff_lines("a\nb\nc", "a\nx\nc");
        assert_eq!(changes.len(), 4);
        assert!(changes[0].is_equal());
        assert!(changes[1].is_delete());
        assert!(changes[2].is_insert());
        assert!(changes[3].is_equal());
    }

    #[test]
    fn test_diff_chars() {
        let changes = diff_chars("hello", "hallo");
        // Should have: h (equal), e (delete), a (insert), llo (equal)
        assert!(changes.iter().any(|c| c.is_delete() && c.text == "e"));
        assert!(changes.iter().any(|c| c.is_insert() && c.text == "a"));
    }

    #[test]
    fn test_diff_words() {
        let changes = diff_words("hello world", "hello rust world");
        assert!(changes.iter().any(|c| c.is_insert() && c.text == "rust"));
    }

    #[test]
    fn test_diff_stats() {
        let changes = diff_lines("a\nb\nc", "a\nx\nc");
        let stats = DiffStats::from_changes(&changes);
        assert_eq!(stats.equal, 2);
        assert_eq!(stats.insertions, 1);
        assert_eq!(stats.deletions, 1);
    }

    #[test]
    fn test_diff_similarity() {
        let changes = diff_lines("a\nb\nc\nd", "a\nb\nc\nd");
        let stats = DiffStats::from_changes(&changes);
        assert!((stats.similarity() - 1.0).abs() < 0.001);

        let changes = diff_lines("a", "b");
        let stats = DiffStats::from_changes(&changes);
        assert!(stats.similarity() < 0.5);
    }

    #[test]
    fn test_empty_diff() {
        let changes = diff_lines("", "");
        assert!(changes.is_empty());

        let changes = diff_lines("", "a");
        assert_eq!(changes.len(), 1);
        assert!(changes[0].is_insert());

        let changes = diff_lines("a", "");
        assert_eq!(changes.len(), 1);
        assert!(changes[0].is_delete());
    }
}
