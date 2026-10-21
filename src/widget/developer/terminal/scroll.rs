//! Scrolling through the scrollback: a ScrollState over the lines, from the
//! top, that follows the newest line until scrolled back (#973)

use super::Terminal;

impl Terminal {
    /// Scroll up, back through the scrollback
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll
            .scroll_by(-(lines.min(isize::MAX as usize) as isize));
    }

    /// Scroll down, toward the newest line; reaching it follows new output
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll
            .scroll_by(lines.min(isize::MAX as usize) as isize);
    }

    /// Scroll to bottom and follow new output
    pub fn scroll_to_bottom(&mut self) {
        self.scroll.scroll_to_end();
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll.scroll_to_top();
    }

    /// Get visible lines range
    pub(super) fn visible_range(&self) -> std::ops::Range<usize> {
        self.scroll.visible()
    }

    /// How many lines are below the view: 0 when it shows the newest
    pub(super) fn lines_below(&self) -> usize {
        self.lines.len() - self.scroll.visible().end
    }

    /// The lines changed at the end: give the scroll their count
    pub(super) fn sync_scroll(&self) {
        self.scroll.set_content(self.lines.len());
    }
}
