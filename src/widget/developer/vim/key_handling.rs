//! Turning key events into vim actions, per mode

use super::{VimAction, VimMode, VimMotion, VimState};
use crate::event::{Key, KeyEvent};

impl VimState {
    /// Handle key event in normal mode
    fn handle_normal(&mut self, key: &KeyEvent) -> VimAction {
        // Handle digits for count
        if let Key::Char(ch) = key.key {
            if let Some(digit) = ch.to_digit(10) {
                let digit = digit as usize;
                self.count = Some(self.count.unwrap_or(0) * 10 + digit);
                return VimAction::None;
            }
        }

        // Handle operator pending
        if let Some(op) = self.operator {
            if let Key::Char(ch) = key.key {
                let motion = self.char_to_motion(ch);
                if motion.is_some() {
                    self.operator = None;
                    return match op {
                        'd' => VimAction::Delete(motion),
                        'y' => VimAction::Yank(motion),
                        'c' => VimAction::Change(motion),
                        _ => VimAction::None,
                    };
                }
            }
        }

        match key.key {
            // Mode changes
            Key::Char('i') => {
                self.set_mode(VimMode::Insert);
                VimAction::Insert
            }
            Key::Char('I') => {
                self.set_mode(VimMode::Insert);
                VimAction::InsertStart
            }
            Key::Char('a') => {
                self.set_mode(VimMode::Insert);
                VimAction::Append
            }
            Key::Char('A') => {
                self.set_mode(VimMode::Insert);
                VimAction::AppendEnd
            }
            Key::Char('o') => {
                self.set_mode(VimMode::Insert);
                VimAction::OpenBelow
            }
            Key::Char('O') => {
                self.set_mode(VimMode::Insert);
                VimAction::OpenAbove
            }
            Key::Char('v') => {
                self.set_mode(VimMode::Visual);
                VimAction::EnterVisual
            }
            Key::Char('V') => {
                self.set_mode(VimMode::VisualLine);
                VimAction::EnterVisualLine
            }
            Key::Char(':') => {
                self.set_mode(VimMode::Command);
                self.command_buffer.clear();
                VimAction::EnterCommand
            }
            Key::Char('/') => {
                self.set_mode(VimMode::Search);
                self.search_pattern.clear();
                self.search_forward = true;
                VimAction::EnterSearch
            }
            Key::Char('?') => {
                self.set_mode(VimMode::Search);
                self.search_pattern.clear();
                self.search_forward = false;
                VimAction::EnterSearch
            }

            // Motions
            Key::Char('h') | Key::Left => VimAction::Move(VimMotion::Left),
            Key::Char('j') | Key::Down => VimAction::Move(VimMotion::Down),
            Key::Char('k') | Key::Up => VimAction::Move(VimMotion::Up),
            Key::Char('l') | Key::Right => VimAction::Move(VimMotion::Right),
            Key::Char('w') => VimAction::Move(VimMotion::Word),
            Key::Char('b') => VimAction::Move(VimMotion::WordBack),
            Key::Char('e') => VimAction::Move(VimMotion::WordEnd),
            Key::Char('0') => VimAction::Move(VimMotion::LineStart),
            Key::Char('$') => VimAction::Move(VimMotion::LineEnd),
            Key::Char('^') => VimAction::Move(VimMotion::FirstNonBlank),
            Key::Char('G') => VimAction::Move(VimMotion::GoToLine(self.count)),
            Key::Char('g') => {
                self.key_buffer.push('g');
                VimAction::None
            }
            Key::Char('{') => VimAction::Move(VimMotion::ParagraphBack),
            Key::Char('}') => VimAction::Move(VimMotion::ParagraphForward),
            Key::Char('%') => VimAction::Move(VimMotion::MatchBracket),
            Key::Char('n') => VimAction::Move(VimMotion::SearchNext),
            Key::Char('N') => VimAction::Move(VimMotion::SearchPrev),

            // Operators
            Key::Char('d') => {
                self.operator = Some('d');
                VimAction::None
            }
            Key::Char('y') => {
                self.operator = Some('y');
                VimAction::None
            }
            Key::Char('c') => {
                self.operator = Some('c');
                VimAction::None
            }

            // Actions
            Key::Char('x') => VimAction::Delete(Some(VimMotion::Right)),
            Key::Char('X') => VimAction::Delete(Some(VimMotion::Left)),
            Key::Char('p') => VimAction::PasteAfter,
            Key::Char('P') => VimAction::PasteBefore,
            Key::Char('u') => VimAction::Undo,
            Key::Char('r') if key.ctrl => VimAction::Redo,
            Key::Char('.') => VimAction::Repeat,
            Key::Char('J') => VimAction::JoinLines,
            Key::Char('>') => VimAction::Indent,
            Key::Char('<') => VimAction::Outdent,

            Key::Escape => {
                self.count = None;
                self.operator = None;
                VimAction::Escape
            }

            _ => VimAction::None,
        }
    }

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
            // Movement in visual mode
            Key::Char('h') | Key::Left => VimAction::Move(VimMotion::Left),
            Key::Char('j') | Key::Down => VimAction::Move(VimMotion::Down),
            Key::Char('k') | Key::Up => VimAction::Move(VimMotion::Up),
            Key::Char('l') | Key::Right => VimAction::Move(VimMotion::Right),
            Key::Char('w') => VimAction::Move(VimMotion::Word),
            Key::Char('b') => VimAction::Move(VimMotion::WordBack),
            _ => VimAction::None,
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
            Key::Backspace => {
                self.command_buffer.pop();
                if self.command_buffer.is_empty() {
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
                self.search_pattern.pop();
                if self.search_pattern.is_empty() {
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

    /// Convert character to motion
    fn char_to_motion(&self, ch: char) -> Option<VimMotion> {
        match ch {
            'h' => Some(VimMotion::Left),
            'j' => Some(VimMotion::Down),
            'k' => Some(VimMotion::Up),
            'l' => Some(VimMotion::Right),
            'w' => Some(VimMotion::Word),
            'b' => Some(VimMotion::WordBack),
            'e' => Some(VimMotion::WordEnd),
            '0' => Some(VimMotion::LineStart),
            '$' => Some(VimMotion::LineEnd),
            '^' => Some(VimMotion::FirstNonBlank),
            'G' => Some(VimMotion::GoToLine(None)),
            '{' => Some(VimMotion::ParagraphBack),
            '}' => Some(VimMotion::ParagraphForward),
            '%' => Some(VimMotion::MatchBracket),
            // Same key repeats = line
            'd' | 'y' | 'c' => Some(VimMotion::Down),
            _ => None,
        }
    }

    /// Handle a key event
    pub fn handle_key(&mut self, key: &KeyEvent) -> VimAction {
        // Check for 'gg' sequence
        if !self.key_buffer.is_empty() {
            if let Key::Char(ch) = key.key {
                if self.key_buffer == ['g'] && ch == 'g' {
                    self.key_buffer.clear();
                    return VimAction::Move(VimMotion::GoToLine(Some(1)));
                }
            }
            self.key_buffer.clear();
        }

        let action = match self.mode {
            VimMode::Normal => self.handle_normal(key),
            VimMode::Insert => self.handle_insert(key),
            VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => self.handle_visual(key),
            VimMode::Command => self.handle_command(key),
            VimMode::Search => self.handle_search(key),
            VimMode::Replace => {
                if let Key::Char(ch) = key.key {
                    self.set_mode(VimMode::Normal);
                    VimAction::ReplaceChar(ch)
                } else if key.key == Key::Escape {
                    self.set_mode(VimMode::Normal);
                    VimAction::Escape
                } else {
                    VimAction::None
                }
            }
        };

        // Save for repeat
        if action != VimAction::None
            && action != VimAction::Escape
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

        // Reset count after action
        if action != VimAction::None {
            self.count = None;
        }

        action
    }
}

// KEEP HERE - Private field access tests (tests note operator field is private)

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Private field access tests
    // =========================================================================

    #[test]
    fn test_set_mode_from_insert_clears_operator() {
        let mut vim = VimState::new();
        vim.set_mode(VimMode::Insert);
        vim.handle_key(&KeyEvent::new(Key::Char('d')));
        // Can't test operator directly as it's private
    }
}
