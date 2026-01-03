//! Diff Viewer widget for side-by-side code comparison
//!
//! Displays differences between two texts with syntax highlighting
//! and unified/split view modes.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
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

    #[test]
    fn test_diff_viewer_creation() {
        let viewer = DiffViewer::new();
        assert_eq!(viewer.change_count(), 0);
    }

    #[test]
    fn test_diff_compare() {
        let viewer = diff("Hello\nWorld", "Hello\nRust");
        assert!(viewer.change_count() > 0);
    }

    #[test]
    fn test_diff_equal() {
        let viewer = diff("Same", "Same");
        assert_eq!(viewer.change_count(), 0);
    }

    #[test]
    fn test_diff_modes() {
        let viewer = diff("A\nB", "A\nC").mode(DiffMode::Unified);
        assert_eq!(viewer.mode, DiffMode::Unified);
    }

    #[test]
    fn test_diff_render() {
        let viewer = diff("Original\nLine", "Modified\nLine");

        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        viewer.render(&mut ctx);
    }

    #[test]
    fn test_diff_scroll() {
        let mut viewer = diff("1\n2\n3\n4\n5", "1\n2\n3\n4\n5\n6");
        viewer.scroll_down(2);
        assert_eq!(viewer.scroll, 2);
        viewer.scroll_up(1);
        assert_eq!(viewer.scroll, 1);
    }
}
