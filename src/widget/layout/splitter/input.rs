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

    /// Focus next pane
    pub fn focus_next(&mut self) {
        let visible = self.visible_indices();

        if let Some(pos) = visible.iter().position(|&i| i == self.focused_pane) {
            let next = (pos + 1) % visible.len();
            self.focused_pane = visible[next];
        }
    }

    /// Focus previous pane
    pub fn focus_prev(&mut self) {
        let visible = self.visible_indices();

        if let Some(pos) = visible.iter().position(|&i| i == self.focused_pane) {
            let prev = if pos == 0 { visible.len() - 1 } else { pos - 1 };
            self.focused_pane = visible[prev];
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
    pub fn toggle_pane(&mut self, index: usize) {
        if let Some(pane) = self.panes.get_mut(index) {
            pane.toggle_collapse();
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
