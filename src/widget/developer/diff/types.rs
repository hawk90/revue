//! Diff display mode, line, change type and color scheme types

use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, SEPARATOR_COLOR};

/// Diff display mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiffMode {
    /// Side-by-side comparison: old text on the left, new on the right,
    /// a modified line as one row with its changed words emphasized
    #[default]
    Split,
    /// Unified diff format: one column, a modified line drawn as its `-`
    /// row and its `+` row (as `git diff` and delta do), each with its
    /// changed words emphasized
    Unified,
    /// Like [`DiffMode::Unified`], but a modified line is one `~` row: the
    /// shared text once, and where the lines differ the removed words then
    /// the added ones in their colors, as `git diff --word-diff=color` does
    Inline,
}

/// A line in the diff
#[derive(Clone, Debug)]
pub struct DiffLine {
    /// Line number in left file (None if added)
    pub left_num: Option<usize>,
    /// Line number in right file (None if removed)
    pub right_num: Option<usize>,
    /// Left content
    pub left: String,
    /// Right content
    pub right: String,
    /// Change type
    pub change: ChangeType,
}

/// Type of change
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeType {
    /// No change
    Equal,
    /// Line was removed
    Removed,
    /// Line was added
    Added,
    /// Line was modified: a removed and an added line of one change that
    /// are alike, paired as delta does. Both numbers and both texts are
    /// set. Lines are alike when the changed words take at most 60% of
    /// the columns (changed columns over changed plus twice the shared
    /// ones, blanks at the ends of each run not counted); each removed
    /// line, in order, pairs with the first alike added line after the
    /// previous pair, so pairs keep the order of both texts
    Modified,
}

/// Diff color scheme
#[derive(Clone, Debug)]
pub struct DiffColors {
    /// Added line background, and the background of the added words of a
    /// modified line
    pub added_bg: Color,
    /// Added line foreground, and the foreground of the added words of a
    /// modified line
    pub added_fg: Color,
    /// Removed line background, and the background of the removed words of
    /// a modified line
    pub removed_bg: Color,
    /// Removed line foreground, and the foreground of the removed words of
    /// a modified line
    pub removed_fg: Color,
    /// Modified line background: behind the text a modified line's old and
    /// new text share (drawn in the default foreground) and the rest of its
    /// split half, and behind the inline view's `~`
    pub modified_bg: Color,
    /// Line number color
    pub line_number: Color,
    /// Separator color
    pub separator: Color,
    /// Header background
    pub header_bg: Color,
}

impl Default for DiffColors {
    fn default() -> Self {
        Self {
            added_bg: Color::rgb(30, 60, 30),
            added_fg: Color::rgb(150, 255, 150),
            removed_bg: Color::rgb(60, 30, 30),
            removed_fg: Color::rgb(255, 150, 150),
            modified_bg: Color::rgb(60, 60, 30),
            line_number: DISABLED_FG,
            separator: SEPARATOR_COLOR,
            header_bg: Color::rgb(40, 40, 60),
        }
    }
}

impl DiffColors {
    /// GitHub-style colors
    pub fn github() -> Self {
        Self {
            added_bg: Color::rgb(35, 134, 54),
            added_fg: Color::WHITE,
            removed_bg: Color::rgb(218, 54, 51),
            removed_fg: Color::WHITE,
            modified_bg: Color::rgb(210, 153, 34),
            line_number: Color::rgb(140, 140, 140),
            separator: Color::rgb(48, 54, 61),
            header_bg: Color::rgb(22, 27, 34),
        }
    }
}
