//! Diff Viewer widget for side-by-side code comparison
//!
//! Displays differences between two texts with syntax highlighting
//! and unified/split view modes.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};
use similar::{ChangeTag, TextDiff};

/// Line rendering layout parameters
struct LineLayout {
    x: u16,
    y: u16,
    line_num_width: u16,
    content_width: usize,
}

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
            line_number: Color::rgb(100, 100, 100),
            separator: Color::rgb(60, 60, 60),
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
    /// Context lines around changes
    context_lines: usize,
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
            context_lines: 3,
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

    /// Set context lines around changes
    pub fn context(mut self, lines: usize) -> Self {
        self.context_lines = lines;
        self
    }

    /// Set scroll position
    pub fn set_scroll(&mut self, scroll: usize) {
        self.scroll = scroll.min(self.diff_lines.len().saturating_sub(1));
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

    /// Render split view
    fn render_split(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 3 {
            return;
        }

        let half_width = (area.width / 2).saturating_sub(1);
        let line_num_width = if self.show_line_numbers { 5 } else { 0 };
        let content_width = half_width.saturating_sub(line_num_width) as usize;

        // Header
        self.render_header(ctx, half_width);

        // Content
        let visible_lines = (area.height - 1) as usize;
        for (i, line) in self
            .diff_lines
            .iter()
            .skip(self.scroll)
            .take(visible_lines)
            .enumerate()
        {
            let y = area.y + 1 + i as u16;

            // Left side
            let left_layout = LineLayout {
                x: area.x,
                y,
                line_num_width,
                content_width,
            };
            self.render_line_half(ctx, line, true, &left_layout);

            // Separator
            let mut sep = Cell::new('│');
            sep.fg = Some(self.colors.separator);
            ctx.buffer.set(area.x + half_width, y, sep);

            // Right side
            let right_layout = LineLayout {
                x: area.x + half_width + 1,
                y,
                line_num_width,
                content_width,
            };
            self.render_line_half(ctx, line, false, &right_layout);
        }
    }

    /// Render one half of a split line
    fn render_line_half(
        &self,
        ctx: &mut RenderContext,
        line: &DiffLine,
        is_left: bool,
        layout: &LineLayout,
    ) {
        let LineLayout {
            x,
            y,
            line_num_width,
            content_width,
        } = *layout;
        let (content, line_num, bg) = if is_left {
            (
                &line.left,
                line.left_num,
                match line.change {
                    ChangeType::Removed => Some(self.colors.removed_bg),
                    ChangeType::Modified => Some(self.colors.modified_bg),
                    _ => None,
                },
            )
        } else {
            (
                &line.right,
                line.right_num,
                match line.change {
                    ChangeType::Added => Some(self.colors.added_bg),
                    ChangeType::Modified => Some(self.colors.modified_bg),
                    _ => None,
                },
            )
        };

        // Line number
        if self.show_line_numbers {
            let num_str = line_num
                .map(|n| format!("{:>4}", n))
                .unwrap_or_else(|| "    ".to_string());
            for (i, ch) in num_str.chars().enumerate() {
                if i as u16 >= line_num_width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.colors.line_number);
                cell.bg = bg;
                ctx.buffer.set(x + i as u16, y, cell);
            }
        }

        // Content
        let fg = match line.change {
            ChangeType::Added => Some(self.colors.added_fg),
            ChangeType::Removed => Some(self.colors.removed_fg),
            _ => None,
        };

        for (i, ch) in content.chars().take(content_width).enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            ctx.buffer.set(x + line_num_width + i as u16, y, cell);
        }

        // Fill remaining with background
        for i in content.chars().count()..content_width {
            let mut cell = Cell::new(' ');
            cell.bg = bg;
            ctx.buffer.set(x + line_num_width + i as u16, y, cell);
        }
    }

    /// Render header
    fn render_header(&self, ctx: &mut RenderContext, half_width: u16) {
        let area = ctx.area;

        // Left header
        for (i, ch) in self.left_name.chars().enumerate() {
            if i as u16 >= half_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.bg = Some(self.colors.header_bg);
            cell.modifier = Modifier::BOLD;
            ctx.buffer.set(area.x + i as u16, area.y, cell);
        }

        // Fill left header
        for i in self.left_name.len()..half_width as usize {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.colors.header_bg);
            ctx.buffer.set(area.x + i as u16, area.y, cell);
        }

        // Separator
        let mut sep = Cell::new('│');
        sep.fg = Some(self.colors.separator);
        sep.bg = Some(self.colors.header_bg);
        ctx.buffer.set(area.x + half_width, area.y, sep);

        // Right header
        for (i, ch) in self.right_name.chars().enumerate() {
            if i as u16 >= half_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.bg = Some(self.colors.header_bg);
            cell.modifier = Modifier::BOLD;
            ctx.buffer
                .set(area.x + half_width + 1 + i as u16, area.y, cell);
        }

        // Fill right header
        for i in self.right_name.len()..half_width as usize {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.colors.header_bg);
            ctx.buffer
                .set(area.x + half_width + 1 + i as u16, area.y, cell);
        }
    }

    /// Render unified view
    fn render_unified(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let line_num_width = if self.show_line_numbers { 10u16 } else { 0 };
        let content_width = area.width.saturating_sub(line_num_width + 1) as usize;

        let visible_lines = area.height as usize;
        for (i, line) in self
            .diff_lines
            .iter()
            .skip(self.scroll)
            .take(visible_lines)
            .enumerate()
        {
            let y = area.y + i as u16;

            // Line numbers (left:right)
            if self.show_line_numbers {
                let num_str = format!(
                    "{:>4}:{:<4}",
                    line.left_num.map(|n| n.to_string()).unwrap_or_default(),
                    line.right_num.map(|n| n.to_string()).unwrap_or_default()
                );
                for (j, ch) in num_str.chars().enumerate() {
                    if j as u16 >= line_num_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.colors.line_number);
                    ctx.buffer.set(area.x + j as u16, y, cell);
                }
            }

            // Change indicator
            let (indicator, fg, bg) = match line.change {
                ChangeType::Added => ('+', self.colors.added_fg, self.colors.added_bg),
                ChangeType::Removed => ('-', self.colors.removed_fg, self.colors.removed_bg),
                ChangeType::Modified => ('~', self.colors.added_fg, self.colors.modified_bg),
                ChangeType::Equal => (' ', Color::WHITE, Color::default()),
            };

            let mut ind_cell = Cell::new(indicator);
            ind_cell.fg = Some(fg);
            ind_cell.bg = Some(bg);
            ctx.buffer.set(area.x + line_num_width, y, ind_cell);

            // Content
            let content = if !line.right.is_empty() {
                &line.right
            } else {
                &line.left
            };
            for (j, ch) in content.chars().take(content_width).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                ctx.buffer
                    .set(area.x + line_num_width + 1 + j as u16, y, cell);
            }
        }
    }
}

impl Default for DiffViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl View for DiffViewer {
    crate::impl_view_meta!("DiffViewer");

    fn render(&self, ctx: &mut RenderContext) {
        match self.mode {
            DiffMode::Split => self.render_split(ctx),
            DiffMode::Unified | DiffMode::Inline => self.render_unified(ctx),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // DiffMode enum trait tests
    // =========================================================================

    #[test]
    fn test_diff_mode_default() {
        assert_eq!(DiffMode::default(), DiffMode::Split);
    }

    #[test]
    fn test_diff_mode_clone() {
        let mode = DiffMode::Unified;
        assert_eq!(mode, mode.clone());
    }

    #[test]
    fn test_diff_mode_copy() {
        let mode1 = DiffMode::Inline;
        let mode2 = mode1;
        assert_eq!(mode1, DiffMode::Inline);
        assert_eq!(mode2, DiffMode::Inline);
    }

    #[test]
    fn test_diff_mode_equality() {
        assert_eq!(DiffMode::Split, DiffMode::Split);
        assert_eq!(DiffMode::Unified, DiffMode::Unified);
        assert_ne!(DiffMode::Split, DiffMode::Inline);
    }

    #[test]
    fn test_diff_mode_debug() {
        let debug_str = format!("{:?}", DiffMode::Split);
        assert!(debug_str.contains("Split"));
    }

    // =========================================================================
    // ChangeType enum trait tests
    // =========================================================================

    #[test]
    fn test_change_type_clone() {
        let change = ChangeType::Added;
        assert_eq!(change, change.clone());
    }

    #[test]
    fn test_change_type_copy() {
        let change1 = ChangeType::Removed;
        let change2 = change1;
        assert_eq!(change1, ChangeType::Removed);
        assert_eq!(change2, ChangeType::Removed);
    }

    #[test]
    fn test_change_type_equality() {
        assert_eq!(ChangeType::Equal, ChangeType::Equal);
        assert_eq!(ChangeType::Added, ChangeType::Added);
        assert_ne!(ChangeType::Added, ChangeType::Removed);
    }

    #[test]
    fn test_change_type_debug() {
        let debug_str = format!("{:?}", ChangeType::Modified);
        assert!(debug_str.contains("Modified"));
    }

    // =========================================================================
    // DiffLine struct tests
    // =========================================================================

    #[test]
    fn test_diff_line_clone() {
        let line = DiffLine {
            left_num: Some(1),
            right_num: Some(2),
            left: "hello".to_string(),
            right: "world".to_string(),
            change: ChangeType::Modified,
        };
        let cloned = line.clone();
        assert_eq!(line.left_num, cloned.left_num);
        assert_eq!(line.change, cloned.change);
    }

    #[test]
    fn test_diff_line_debug() {
        let line = DiffLine {
            left_num: None,
            right_num: Some(1),
            left: String::new(),
            right: "new line".to_string(),
            change: ChangeType::Added,
        };
        let debug_str = format!("{:?}", line);
        assert!(debug_str.contains("DiffLine"));
    }

    // =========================================================================
    // DiffColors struct tests
    // =========================================================================

    #[test]
    fn test_diff_colors_default() {
        let colors = DiffColors::default();
        assert_eq!(colors.added_bg, Color::rgb(30, 60, 30));
        assert_eq!(colors.removed_bg, Color::rgb(60, 30, 30));
        assert_eq!(colors.modified_bg, Color::rgb(60, 60, 30));
    }

    #[test]
    fn test_diff_colors_clone() {
        let colors1 = DiffColors::default();
        let colors2 = colors1.clone();
        assert_eq!(colors1.added_bg, colors2.added_bg);
        assert_eq!(colors1.separator, colors2.separator);
    }

    #[test]
    fn test_diff_colors_debug() {
        let colors = DiffColors::default();
        let debug_str = format!("{:?}", colors);
        assert!(debug_str.contains("DiffColors"));
    }

    #[test]
    fn test_diff_colors_github() {
        let colors = DiffColors::github();
        assert_eq!(colors.added_bg, Color::rgb(35, 134, 54));
        assert_eq!(colors.removed_bg, Color::rgb(218, 54, 51));
    }

    // =========================================================================
    // DiffViewer constructor tests
    // =========================================================================

    #[test]
    fn test_diff_viewer_new() {
        let viewer = DiffViewer::new();
        assert_eq!(viewer.change_count(), 0);
        assert_eq!(viewer.line_count(), 0);
    }

    #[test]
    fn test_diff_viewer_default() {
        let viewer = DiffViewer::default();
        assert_eq!(viewer.change_count(), 0);
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_left() {
        let viewer = DiffViewer::new().left("left content");
        // Verify builder compiles and returns self
        let _ = viewer.left("other");
    }

    #[test]
    fn test_right() {
        let viewer = DiffViewer::new().right("right content");
        let _ = viewer.right("other");
    }

    #[test]
    fn test_left_name() {
        let viewer = DiffViewer::new().left_name("original.txt");
        let _ = viewer.left_name("new.txt");
    }

    #[test]
    fn test_right_name() {
        let viewer = DiffViewer::new().right_name("modified.txt");
        let _ = viewer.right_name("new.txt");
    }

    #[test]
    fn test_compare() {
        let viewer = DiffViewer::new().compare("A\nB", "A\nC");
        assert!(viewer.change_count() > 0);
    }

    #[test]
    fn test_mode_unified() {
        let viewer = DiffViewer::new().mode(DiffMode::Unified);
        let _ = viewer.mode(DiffMode::Split);
    }

    #[test]
    fn test_mode_inline() {
        let viewer = DiffViewer::new().mode(DiffMode::Inline);
        let _ = viewer.mode(DiffMode::Split);
    }

    #[test]
    fn test_mode_split() {
        let viewer = DiffViewer::new().mode(DiffMode::Split);
        let _ = viewer.mode(DiffMode::Unified);
    }

    #[test]
    fn test_colors() {
        let custom_colors = DiffColors::github();
        let viewer = DiffViewer::new().colors(custom_colors.clone());
        let _ = viewer.colors(DiffColors::default());
    }

    #[test]
    fn test_line_numbers() {
        let viewer = DiffViewer::new().line_numbers(false);
        let _ = viewer.line_numbers(true);
    }

    #[test]
    fn test_context() {
        let viewer = DiffViewer::new().context(5);
        let _ = viewer.context(3);
    }

    // =========================================================================
    // State-changing method tests
    // =========================================================================

    #[test]
    fn test_set_scroll() {
        let mut viewer = diff("1\n2\n3\n4\n5", "1\n2\n3\n4\n5");
        viewer.set_scroll(2);
        viewer.set_scroll(0);
        // Just verify methods work without panicking
    }

    #[test]
    fn test_set_scroll_clamps() {
        let mut viewer = diff("1\n2", "1\n2");
        viewer.set_scroll(100);
        // Should clamp internally
    }

    #[test]
    fn test_scroll_down() {
        let mut viewer = diff("1\n2\n3\n4\n5", "1\n2\n3\n4\n5\n6");
        viewer.scroll_down(2);
        viewer.scroll_down(1);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_scroll_up() {
        let mut viewer = diff("1\n2\n3\n4\n5", "1\n2\n3\n4\n5\n6");
        viewer.scroll_down(5);
        viewer.scroll_up(2);
        viewer.scroll_up(10); // Should saturate
                              // Just verify it doesn't panic
    }

    // =========================================================================
    // Getter method tests
    // =========================================================================

    #[test]
    fn test_change_count() {
        let viewer = diff("A\nB\nC", "A\nX\nC");
        assert!(viewer.change_count() > 0);
    }

    #[test]
    fn test_change_count_all_different() {
        let viewer = diff("A", "B");
        assert!(viewer.change_count() >= 1);
    }

    #[test]
    fn test_change_count_identical() {
        let viewer = diff("Same\nContent", "Same\nContent");
        assert_eq!(viewer.change_count(), 0);
    }

    #[test]
    fn test_line_count() {
        let viewer = diff("1\n2\n3", "1\n2\n3");
        assert!(viewer.line_count() > 0);
    }

    #[test]
    fn test_line_count_empty() {
        let viewer = diff("", "");
        assert_eq!(viewer.line_count(), 0);
    }

    // =========================================================================
    // Diff computation tests
    // =========================================================================

    #[test]
    fn test_diff_addition() {
        let viewer = diff("Line 1\nLine 2", "Line 1\nLine 2\nNew Line");
        assert!(viewer.change_count() > 0);
    }

    #[test]
    fn test_diff_deletion() {
        let viewer = diff("Line 1\nLine 2\nLine 3", "Line 1\nLine 3");
        assert!(viewer.change_count() > 0);
    }

    #[test]
    fn test_diff_modification() {
        let viewer = diff("Hello World", "Hello Rust");
        assert!(viewer.change_count() > 0);
    }

    #[test]
    fn test_diff_empty_strings() {
        let viewer = diff("", "");
        assert_eq!(viewer.change_count(), 0);
    }

    #[test]
    fn test_diff_one_empty() {
        let viewer = diff("", "content");
        assert!(viewer.change_count() > 0);
    }

    #[test]
    fn test_diff_multiline() {
        let viewer = diff(
            "Line 1\nLine 2\nLine 3\nLine 4",
            "Line 1\nChanged\nLine 3\nLine 4",
        );
        assert!(viewer.change_count() > 0);
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_diff_render_split() {
        let viewer = diff("Original\nLine", "Modified\nLine");

        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        viewer.render(&mut ctx);
    }

    #[test]
    fn test_diff_render_unified() {
        let viewer = diff("A\nB", "A\nC").mode(DiffMode::Unified);

        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        viewer.render(&mut ctx);
    }

    #[test]
    fn test_diff_render_inline() {
        let viewer = diff("test", "toast").mode(DiffMode::Inline);

        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        viewer.render(&mut ctx);
    }

    #[test]
    fn test_diff_render_small_area() {
        let viewer = diff("A", "B");

        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        viewer.render(&mut ctx); // Should not panic
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_diff_viewer_helper() {
        let viewer = diff_viewer();
        assert_eq!(viewer.change_count(), 0);
    }

    #[test]
    fn test_diff_helper() {
        let viewer = diff("left", "right");
        assert!(viewer.line_count() > 0);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_builder_chain() {
        let colors = DiffColors::github();
        let _viewer = DiffViewer::new()
            .left("original")
            .right("modified")
            .left_name("old.txt")
            .right_name("new.txt")
            .mode(DiffMode::Unified)
            .colors(colors)
            .line_numbers(false)
            .context(2);
        // If it compiles, the chain works
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_unicode_content() {
        let viewer = diff("Hello 世界", "Hello 世界!");
        assert!(viewer.line_count() > 0);
    }

    #[test]
    fn test_long_lines() {
        let long_line = "A".repeat(1000);
        let viewer = diff(&long_line, &long_line);
        assert!(viewer.line_count() > 0);
    }

    #[test]
    fn test_many_lines() {
        let left = (1..100)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let right = left.clone();
        let viewer = diff(&left, &right);
        assert_eq!(viewer.change_count(), 0);
    }

    #[test]
    fn test_whitespace_changes() {
        let viewer = diff("Line 1\nLine 2", "Line 1  \nLine 2");
        assert!(viewer.change_count() > 0);
    }

    #[test]
    fn test_trailing_newlines() {
        let viewer = diff("Line 1\n", "Line 1");
        assert!(viewer.line_count() > 0);
    }

    #[test]
    fn test_only_newlines() {
        let viewer = diff("\n\n\n", "\n\n");
        assert!(viewer.line_count() > 0);
    }
}
