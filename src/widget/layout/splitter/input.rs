//! Focus, divider resizing and collapsing, and the keys that drive them

use super::Splitter;
use crate::event::Key;

impl Splitter {
    /// Indices of the panes that are not collapsed, in order
    fn visible_indices(&self) -> Vec<usize> {
        self.panes
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.collapsed)
            .map(|(i, _)| i)
            .collect()
    }

    /// Focus next pane: the next visible one after the focused pane,
    /// wrapping around. From a collapsed pane it is the first visible one
    /// after it, so Tab never sticks on a hidden pane (#885).
    pub fn focus_next(&mut self) {
        let visible = self.visible_indices();
        let next = visible
            .iter()
            .copied()
            .find(|&i| i > self.focused_pane)
            .or_else(|| visible.first().copied());
        if let Some(next) = next {
            self.focused_pane = next;
        }
    }

    /// Focus previous pane: the previous visible one before the focused
    /// pane, wrapping around, from a collapsed pane too
    pub fn focus_prev(&mut self) {
        let visible = self.visible_indices();
        let prev = visible
            .iter()
            .copied()
            .rev()
            .find(|&i| i < self.focused_pane)
            .or_else(|| visible.last().copied());
        if let Some(prev) = prev {
            self.focused_pane = prev;
        }
    }

    /// The panes on either side of a divider. Dividers are counted as they
    /// are drawn - between visible panes, skipping collapsed ones.
    fn divider_panes(&self, divider: usize) -> Option<(usize, usize)> {
        let mut visible = self
            .panes
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.collapsed)
            .map(|(i, _)| i)
            .skip(divider);
        Some((visible.next()?, visible.next()?))
    }

    /// Start resizing divider
    ///
    /// `divider` counts the dividers as drawn: between visible panes, so
    /// collapsed panes are skipped.
    pub fn start_resize(&mut self, divider: usize) {
        if self.divider_panes(divider).is_some() {
            self.active_divider = Some(divider);
        }
    }

    /// Stop resizing
    pub fn stop_resize(&mut self) {
        self.active_divider = None;
    }

    /// Resize by delta
    pub fn resize(&mut self, delta: i16) {
        if let Some((before, after)) = self.active_divider.and_then(|d| self.divider_panes(d)) {
            let current_ratio = self.panes[before].ratio;
            let next_ratio = self.panes[after].ratio;

            let change = delta as f32 * 0.01;
            let new_current = (current_ratio + change).clamp(0.1, 0.9);
            let new_next = (next_ratio - change).clamp(0.1, 0.9);

            self.panes[before].ratio = new_current;
            self.panes[after].ratio = new_next;
        }
    }

    /// Toggle collapse for pane
    ///
    /// Collapsing the focused pane moves the focus to the next visible
    /// pane, so `focused` never names a hidden one (#885).
    pub fn toggle_pane(&mut self, index: usize) {
        if let Some(pane) = self.panes.get_mut(index) {
            pane.toggle_collapse();
        }
        let hidden = self
            .panes
            .get(self.focused_pane)
            .is_some_and(|p| p.collapsed);
        if hidden {
            self.focus_next();
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Tab => {
                self.focus_next();
                true
            }
            Key::Left | Key::Char('h') if self.active_divider.is_some() => {
                self.resize(-5);
                true
            }
            Key::Right | Key::Char('l') if self.active_divider.is_some() => {
                self.resize(5);
                true
            }
            Key::Up | Key::Char('k') if self.active_divider.is_some() => {
                self.resize(-5);
                true
            }
            Key::Down | Key::Char('j') if self.active_divider.is_some() => {
                self.resize(5);
                true
            }
            Key::Enter if self.active_divider.is_some() => {
                self.stop_resize();
                true
            }
            Key::Escape if self.active_divider.is_some() => {
                self.stop_resize();
                true
            }
            _ => false,
        }
    }
}
