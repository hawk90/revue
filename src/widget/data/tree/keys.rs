//! Tree key handling

use super::Tree;
use crate::event::Key;

impl Tree {
    /// Handle key input, returns true if selection or expansion changed
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.searchable && self.handle_search_key(key) {
            return true;
        }

        // A searchable tree takes letters into the query above, so only the
        // space bar still needs keeping from its tree meaning here.
        match key {
            Key::Up | Key::Char('k') => self.moved(Self::select_prev),
            Key::Down | Key::Char('j') => self.moved(Self::select_next),
            Key::Char(' ') if self.searchable => false,
            Key::Char(' ') if self.multi_select => {
                self.toggle_select();
                true
            }
            Key::Enter | Key::Char(' ') => self.resized(Self::toggle_expand),
            Key::Right | Key::Char('l') => self.resized(Self::expand),
            Key::Left | Key::Char('h') => self.collapse_or_go_to_parent(),
            Key::Home => self.moved(Self::select_first),
            Key::End => self.moved(Self::select_last),
            _ => false,
        }
    }

    /// Query editing and match navigation; true when `key` was used
    fn handle_search_key(&mut self, key: &Key) -> bool {
        if !self.query.is_empty() {
            match key {
                Key::Char('n') => return self.next_match(),
                Key::Char('N') => return self.prev_match(),
                Key::Escape => {
                    self.clear_query();
                    return true;
                }
                Key::Backspace => {
                    self.query.pop();
                    self.update_matches();
                    return true;
                }
                _ => {}
            }
        }
        match key {
            // Allow alphanumeric and common search chars
            Key::Char(c) if c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '/') => {
                self.query.push(*c);
                self.update_matches();
                true
            }
            _ => false,
        }
    }

    /// Collapse the selected node if it is expanded, else select its
    /// nearest selectable ancestor
    fn collapse_or_go_to_parent(&mut self) -> bool {
        let is_expanded = self
            .get_node_at(self.selection.index)
            .map(|(n, _)| n.expanded && n.has_children())
            .unwrap_or(false);
        if is_expanded {
            self.resized(Self::collapse)
        } else if let Some(parent_idx) = self.find_selectable_ancestor(self.selection.index) {
            let old = self.selection.index;
            self.selection.set(parent_idx);
            old != self.selection.index
        } else {
            false
        }
    }

    /// Run `action`; true when the selection moved
    fn moved(&mut self, action: fn(&mut Self)) -> bool {
        let old = self.selection.index;
        action(self);
        old != self.selection.index
    }

    /// Run `action`; true when the number of visible rows changed
    fn resized(&mut self, action: fn(&mut Self)) -> bool {
        let old_count = self.selection.len;
        action(self);
        old_count != self.selection.len
    }
}
