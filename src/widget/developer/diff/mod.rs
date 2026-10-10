//! Diff Viewer widget for side-by-side code comparison
//!
//! Displays differences between two texts with syntax highlighting
//! and unified/split view modes.

mod render;
mod types;

pub use types::{ChangeType, DiffColors, DiffLine, DiffMode};

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
    /// Scroll position
    scroll: usize,
    /// Unchanged lines kept around each change; `None` shows every line
    context_lines: Option<usize>,
    /// Computed diff lines (cached)
    diff_lines: Vec<DiffLine>,
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
            scroll: 0,
            context_lines: None,
            diff_lines: Vec::new(),
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

    /// Set scroll position
    pub fn set_scroll(&mut self, scroll: usize) {
        self.scroll = scroll.min(self.rows().len().saturating_sub(1));
    }

    /// Scroll down
    pub fn scroll_down(&mut self, amount: usize) {
        self.set_scroll(self.scroll.saturating_add(amount));
    }

    /// Scroll up
    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_sub(amount);
    }

    /// Compute the diff
    fn compute_diff(&mut self) {
        let diff = TextDiff::from_lines(&self.left_content, &self.right_content);
        self.diff_lines.clear();

        let mut left_num = 0usize;
        let mut right_num = 0usize;

        for change in diff.iter_all_changes() {
            let (left_n, right_n, change_type) = match change.tag() {
                ChangeTag::Equal => {
                    left_num += 1;
                    right_num += 1;
                    (Some(left_num), Some(right_num), ChangeType::Equal)
                }
                ChangeTag::Delete => {
                    left_num += 1;
                    (Some(left_num), None, ChangeType::Removed)
                }
                ChangeTag::Insert => {
                    right_num += 1;
                    (None, Some(right_num), ChangeType::Added)
                }
            };

            let content = change.value().trim_end_matches('\n').to_string();

            self.diff_lines.push(DiffLine {
                left_num: left_n,
                right_num: right_n,
                left: if change.tag() != ChangeTag::Insert {
                    content.clone()
                } else {
                    String::new()
                },
                right: if change.tag() != ChangeTag::Delete {
                    content
                } else {
                    String::new()
                },
                change: change_type,
            });
        }
    }

    /// Get number of changes
    pub fn change_count(&self) -> usize {
        self.diff_lines
            .iter()
            .filter(|l| l.change != ChangeType::Equal)
            .count()
    }

    /// Get total lines
    pub fn line_count(&self) -> usize {
        self.diff_lines.len()
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
