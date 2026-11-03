//! Turning key events into vim actions, per mode

use super::normal::basic_motion;
use super::{VimAction, VimMode, VimMotion, VimState};
use crate::event::{Key, KeyEvent};

impl VimState {
    /// Handle key event in insert mode
    fn handle_insert(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            Key::Escape => {
                self.set_mode(VimMode::Normal);
                VimAction::Escape
            }
            _ => VimAction::None, // Let the widget handle insert keys
        }
    }

    /// Handle key event in visual mode
    fn handle_visual(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            Key::Escape => {
                self.set_mode(VimMode::Normal);
                VimAction::Escape
            }
            Key::Char('d') | Key::Char('x') => {
                self.set_mode(VimMode::Normal);
                VimAction::Delete(None)
            }
            Key::Char('y') => {
                self.set_mode(VimMode::Normal);
                VimAction::Yank(None)
            }
            Key::Char('c') => {
                self.set_mode(VimMode::Insert);
                VimAction::Change(None)
            }
            Key::Char('v') if key.ctrl => {
                self.set_mode(VimMode::VisualBlock);
                VimAction::EnterVisualBlock
            }
            // Movement in visual mode
            ref other => basic_motion(other).map_or(VimAction::None, VimAction::Move),
        }
    }

    /// Handle key event in command mode
    fn handle_command(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            Key::Escape => {
                self.set_mode(VimMode::Normal);
                self.command_buffer.clear();
                VimAction::Escape
            }
            Key::Enter => {
                let cmd = self.command_buffer.clone();
                self.set_mode(VimMode::Normal);
                self.command_buffer.clear();
                VimAction::ExecuteCommand(cmd)
            }
            // Leaves only on an empty line, as vim's does (#1081)
            Key::Backspace => {
                if self.command_buffer.pop().is_none() {
                    self.set_mode(VimMode::Normal);
                }
                VimAction::None
            }
            Key::Char(ch) => {
                self.command_buffer.push(ch);
                VimAction::None
            }
            _ => VimAction::None,
        }
    }

    /// Handle key event in search mode
    fn handle_search(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            Key::Escape => {
                self.set_mode(VimMode::Normal);
                self.search_pattern.clear();
                VimAction::Escape
            }
            Key::Enter => {
                self.set_mode(VimMode::Normal);
                VimAction::Move(if self.search_forward {
                    VimMotion::SearchNext
                } else {
                    VimMotion::SearchPrev
                })
            }
            Key::Backspace => {
                if self.search_pattern.pop().is_none() {
                    self.set_mode(VimMode::Normal);
                }
                VimAction::None
            }
            Key::Char(ch) => {
                self.search_pattern.push(ch);
                VimAction::None
            }
            _ => VimAction::None,
        }
    }

    /// Handle key event in replace mode: a char replaces the one under the
    /// cursor; after `r` that is all, after `R` it goes on until Escape
    fn handle_replace(&mut self, key: &KeyEvent) -> VimAction {
        if let Key::Char(ch) = key.key {
            if self.replace_once {
                // `3rx`: the count goes with the replace
                let count = self.count;
                self.set_mode(VimMode::Normal);
                self.count = count;
            }
            VimAction::ReplaceChar(ch)
        } else if key.key == Key::Escape {
            self.set_mode(VimMode::Normal);
            VimAction::Escape
        } else {
            VimAction::None
        }
    }

    /// Handle a key event
    pub fn handle_key(&mut self, key: &KeyEvent) -> VimAction {
        self.action_count = None;

        // `f`, `F`, `t` or `T` waiting for its char: any char is the target
        if let Some(find) = self.pending_find.take() {
            let action = self.finish_find(find, key);
            self.remember_for_repeat(&action);
            if action != VimAction::None {
                self.action_count = self.count.take();
            }
            return action;
        }

        // Check for 'gg' sequence
        if !self.key_buffer.is_empty() {
            let gg = self.key_buffer == ['g'] && key.key == Key::Char('g');
            self.key_buffer.clear();
            if gg {
                return self.finish_gg();
            }
            // `dgw` is no motion: it cancels, like any other key would.
            if self.operator.is_some() && key.key != Key::Escape {
                self.operator = None;
                self.operator_count = None;
                self.count = None;
                return VimAction::None;
            }
        }

        let action = match self.mode {
            VimMode::Normal => self.handle_normal(key),
            VimMode::Insert => self.handle_insert(key),
            VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => self.handle_visual(key),
            VimMode::Command => self.handle_command(key),
            VimMode::Search => self.handle_search(key),
            VimMode::Replace => self.handle_replace(key),
        };

        self.remember_for_repeat(&action);

        // The count goes with the action: it reads through `count` until the
        // next key
        if action != VimAction::None {
            if let Some(count) = self.count.take() {
                self.action_count = Some(count);
            }
        }

        action
    }

    /// The motion `f`/`F`/`t`/`T` names with `key`'s char, for a pending
    /// operator if there is one; any other key cancels both
    fn finish_find(&mut self, find: char, key: &KeyEvent) -> VimAction {
        let Key::Char(ch) = key.key else {
            self.operator = None;
            self.count = None;
            return VimAction::None;
        };
        let motion = match find {
            'f' => VimMotion::FindChar(ch),
            'F' => VimMotion::FindCharBack(ch),
            't' => VimMotion::TillChar(ch),
            _ => VimMotion::TillCharBack(ch),
        };
        self.operator_action(Some(motion))
    }

    /// `motion` for the pending operator, or as a move without one. The
    /// operator's count and the motion's multiply, and `c` enters insert
    /// mode, as visual `c` does (#1081, #1363).
    pub(super) fn operator_action(&mut self, motion: Option<VimMotion>) -> VimAction {
        let Some(op) = self.operator.take() else {
            return motion.map_or(VimAction::None, VimAction::Move);
        };
        let after = self.count.take();
        self.count = match (self.operator_count.take(), after) {
            (None, None) => None,
            (before, after) => Some(before.unwrap_or(1).saturating_mul(after.unwrap_or(1))),
        };
        if motion.is_none() {
            self.count = None;
            return VimAction::None;
        }
        match op {
            'd' => VimAction::Delete(motion),
            'y' => VimAction::Yank(motion),
            'c' => {
                let count = self.count;
                self.set_mode(VimMode::Insert);
                self.count = count;
                VimAction::Change(motion)
            }
            _ => VimAction::None,
        }
    }

    /// `gg`: a motion to the line the count names, the first by default, for a
    /// pending operator if there is one. The count is the line, not a repeat.
    fn finish_gg(&mut self) -> VimAction {
        let line = match (self.operator_count.take(), self.count.take()) {
            (None, None) => 1,
            (before, after) => before.unwrap_or(1).saturating_mul(after.unwrap_or(1)),
        };
        let action = self.operator_action(Some(VimMotion::GoToLine(Some(line))));
        self.remember_for_repeat(&action);
        action
    }

    /// Keep an edit or insert action for `.`
    fn remember_for_repeat(&mut self, action: &VimAction) {
        // Save for repeat
        if *action != VimAction::None
            && *action != VimAction::Escape
            && matches!(
                action,
                VimAction::Delete(_)
                    | VimAction::Yank(_)
                    | VimAction::Change(_)
                    | VimAction::Insert
                    | VimAction::Append
                    | VimAction::OpenBelow
                    | VimAction::OpenAbove
            )
        {
            self.last_action = Some(action.clone());
        }
    }
}

// KEEP HERE - Private field access tests (the pending operator and count are private)

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Private field access tests
    // =========================================================================

    #[test]
    fn test_set_mode_from_insert_clears_operator() {
        let mut vim = VimState::new();
        vim.handle_key(&KeyEvent::new(Key::Char('3')));
        vim.handle_key(&KeyEvent::new(Key::Char('d')));
        // The count typed before the operator waits with it (#1363)
        assert_eq!((vim.operator, vim.operator_count), (Some('d'), Some(3)));
        vim.set_mode(VimMode::Insert);
        assert_eq!(
            (vim.operator, vim.operator_count, vim.count),
            (None, None, None)
        );
        // `d` typed in insert mode is text, not an operator
        vim.handle_key(&KeyEvent::new(Key::Char('d')));
        assert_eq!(vim.operator, None);
    }
}
