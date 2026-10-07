//! Diff display mode, line, change type and color scheme types

use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, SEPARATOR_COLOR};

/// Diff display mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiffMode {
    /// Side-by-side comparison
    #[default]
    Split,
    /// Unified diff format
    Unified,
    /// Inline differences (character-level)
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
    /// Line was modified
    Modified,
}

/// Diff color scheme
#[derive(Clone, Debug)]
pub struct DiffColors {
    /// Added line background
    pub added_bg: Color,
    /// Added line foreground
    pub added_fg: Color,
    /// Removed line background
    pub removed_bg: Color,
    /// Removed line foreground
    pub removed_fg: Color,
    /// Modified line background
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
