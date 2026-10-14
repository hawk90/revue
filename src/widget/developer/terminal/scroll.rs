//! Scrolling through the scrollback: the offset from the bottom and the
//! lines it shows

use super::Terminal;

impl Terminal {
    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        let max_scroll = self.lines.len().saturating_sub(self.height as usize);
        self.scroll_offset = (self.scroll_offset + lines).min(max_scroll);
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = self.lines.len().saturating_sub(self.height as usize);
    }

    /// Get visible lines range
    pub(super) fn visible_range(&self) -> std::ops::Range<usize> {
        let total = self.lines.len();
        let visible = self.height as usize;
        let end = total.saturating_sub(self.scroll_offset);
        let start = end.saturating_sub(visible);
        start..end
    }
}
