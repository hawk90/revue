//! Diff Viewer widget for side-by-side code comparison
//!
//! Displays the line differences between two texts side by side (split),
//! in one column (unified), or in one column with each changed line as one
//! row (inline). The text is drawn in the change colors; it is not syntax
//! highlighted.
//!
//! Within a change, a removed line and an added line that are alike become
//! one [`ChangeType::Modified`] line, and the words that differ between
//! them are drawn in the removed and added colors while the rest of the
//! line is on [`DiffColors::modified_bg`]. Lines are alike when at most 60%
//! of their text changed (delta's `--max-line-distance` rule); the others
//! stay [`ChangeType::Removed`] and [`ChangeType::Added`]. Blanks between
//! two changed words join them, so `one two three` to `one 2 3` marks
//! `two three` and `2 3` as one run each, as delta does.
//!
//! Pairing compares lines two by two, so it stops after 10,000 line
//! comparisons in one change, a guard against quadratic time on a huge
//! change (delta has none); past that the lines stay removed and added.

mod pair;
mod render;
mod types;

pub use types::{ChangeType, DiffColors, DiffLine, DiffMode};

use crate::utils::ScrollState;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};
use similar::{ChangeTag, TextDiff};

/// Diff Viewer widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let diff = DiffViewer::new()
///     .left("Original text\nLine 2")
///     .right("Modified text\nLine 2\nLine 3")
///     .mode(DiffMode::Split);
/// ```
#[derive(Clone)]
pub struct DiffViewer {
    /// Left (original) content
    left_content: String,
    /// Right (modified) content
    right_content: String,
    /// Left file name
    left_name: String,
    /// Right file name
    right_name: String,
    /// Display mode
    mode: DiffMode,
    /// Colors
    colors: DiffColors,
    /// Show line numbers
    show_line_numbers: bool,
    /// Where the rows (diff lines and folds) are scrolled to
    scroll: ScrollState,
    /// Unchanged lines kept around each change; `None` shows every line
    context_lines: Option<usize>,
    /// Computed diff lines (cached)
    diff_lines: Vec<DiffLine>,
    /// For each diff line, the changed words of a modified pair
    segments: Vec<Option<pair::Segments>>,
    /// Widget properties
    props: WidgetProps,
}

impl DiffViewer {
    /// Create a new diff viewer
    pub fn new() -> Self {
        Self {
            left_content: String::new(),
            right_content: String::new(),
            left_name: "Original".to_string(),
            right_name: "Modified".to_string(),
            mode: DiffMode::default(),
            colors: DiffColors::default(),
            show_line_numbers: true,
            scroll: ScrollState::new(),
            context_lines: None,
            diff_lines: Vec::new(),
            segments: Vec::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set left (original) content
    pub fn left(mut self, content: impl Into<String>) -> Self {
        self.left_content = content.into();
        self.compute_diff();
        self
    }

    /// Set right (modified) content
    pub fn right(mut self, content: impl Into<String>) -> Self {
        self.right_content = content.into();
        self.compute_diff();
        self
    }

    /// Set left file name
    pub fn left_name(mut self, name: impl Into<String>) -> Self {
        self.left_name = name.into();
        self
    }

    /// Set right file name
    pub fn right_name(mut self, name: impl Into<String>) -> Self {
        self.right_name = name.into();
        self
    }

    /// Compare two files/strings
    pub fn compare(mut self, left: impl Into<String>, right: impl Into<String>) -> Self {
        self.left_content = left.into();
        self.right_content = right.into();
        self.compute_diff();
        self
    }

    /// Set display mode
    pub fn mode(mut self, mode: DiffMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set colors
    pub fn colors(mut self, colors: DiffColors) -> Self {
        self.colors = colors;
        self
    }

    /// Show/hide line numbers
    pub fn line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Keep `lines` unchanged lines around each change and fold the rest
    /// of each unchanged run into a `⋯ N unchanged lines` row
    ///
    /// Without it every line is shown.
    pub fn context(mut self, lines: usize) -> Self {
        self.context_lines = Some(lines);
        self
    }

    /// Set scroll position: the first row shown, kept so the last page is
    /// full once the viewer has been drawn (#973)
    pub fn set_scroll(&mut self, scroll: usize) {
        self.scroll.set_content(self.rows().len());
        self.scroll.scroll_to(scroll);
    }

    /// Scroll down
    pub fn scroll_down(&mut self, amount: usize) {
        self.scroll.set_content(self.rows().len());
        self.scroll
            .scroll_by(amount.min(isize::MAX as usize) as isize);
    }

    /// Scroll up
    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll.set_content(self.rows().len());
        self.scroll
            .scroll_by(-(amount.min(isize::MAX as usize) as isize));
    }

    /// Compute the diff: the lines `similar` finds, with the removed and
    /// added lines of each change paired into modified lines where they are
    /// alike (see [`pair`])
    fn compute_diff(&mut self) {
        let diff = TextDiff::from_lines(&self.left_content, &self.right_content);
        self.diff_lines.clear();
        self.segments.clear();

        let mut nums = (0usize, 0usize);
        let mut removed: Vec<&str> = Vec::new();
        let mut added: Vec<&str> = Vec::new();
        let mut lines = Vec::new();
        let mut segments = Vec::new();
        for change in diff.iter_all_changes() {
            let text = change.value().trim_end_matches('\n');
            match change.tag() {
                ChangeTag::Delete => removed.push(text),
                ChangeTag::Insert => added.push(text),
                ChangeTag::Equal => {
                    push_change(&mut lines, &mut segments, &mut nums, &removed, &added);
                    removed.clear();
                    added.clear();
                    nums.0 += 1;
                    nums.1 += 1;
                    lines.push(DiffLine {
                        left_num: Some(nums.0),
                        right_num: Some(nums.1),
                        left: text.to_string(),
                        right: text.to_string(),
                        change: ChangeType::Equal,
                    });
                    segments.push(None);
                }
            }
        }
        push_change(&mut lines, &mut segments, &mut nums, &removed, &added);
        self.diff_lines = lines;
        self.segments = segments;
    }

    /// Get number of changes: the lines that are not equal, a modified
    /// pair counted once
    pub fn change_count(&self) -> usize {
        self.diff_lines
            .iter()
            .filter(|l| l.change != ChangeType::Equal)
            .count()
    }

    /// Get total lines: the diff lines, a modified pair counted once (the
    /// unified view draws it as two rows)
    pub fn line_count(&self) -> usize {
        self.diff_lines.len()
    }
}

/// Append the lines of one change, its `removed` lines paired with its
/// `added` ones, numbered on from `nums` (left, right)
fn push_change(
    lines: &mut Vec<DiffLine>,
    segments: &mut Vec<Option<pair::Segments>>,
    nums: &mut (usize, usize),
    removed: &[&str],
    added: &[&str],
) {
    for (r, a, seg) in pair::pair(removed, added) {
        let left_num = r.map(|_| {
            nums.0 += 1;
            nums.0
        });
        let right_num = a.map(|_| {
            nums.1 += 1;
            nums.1
        });
        let change = match (r, a) {
            (Some(_), Some(_)) => ChangeType::Modified,
            (Some(_), None) => ChangeType::Removed,
            _ => ChangeType::Added,
        };
        lines.push(DiffLine {
            left_num,
            right_num,
            left: r.map_or_else(String::new, |r| removed[r].to_string()),
            right: a.map_or_else(String::new, |a| added[a].to_string()),
            change,
        });
        segments.push(seg);
    }
}

impl Default for DiffViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(DiffViewer);
impl_props_builders!(DiffViewer);

/// Create a new diff viewer
pub fn diff_viewer() -> DiffViewer {
    DiffViewer::new()
}

/// Create a diff viewer comparing two strings
pub fn diff(left: impl Into<String>, right: impl Into<String>) -> DiffViewer {
    DiffViewer::new().compare(left, right)
}
