//! RichTextEditor undo/redo functionality
//!
//! This module contains the undo and redo system.

use super::core::RichTextEditor;
use super::{Block, EditOp};

/// Maximum undo history size
pub const MAX_UNDO_HISTORY: usize = 100;

impl RichTextEditor {
    /// Undo last operation
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            match op {
                EditOp::InsertChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col].iter().chain(&chars[col + 1..]).collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col);
                    self.redo_stack.push(EditOp::InsertChar { block, col, ch });
                }
                EditOp::DeleteChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col]
                        .iter()
                        .chain(std::iter::once(&ch))
                        .chain(&chars[col..])
                        .collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col + 1);
                    self.redo_stack.push(EditOp::DeleteChar { block, col, ch });
                }
                EditOp::SplitBlock { block, col } => {
                    // Merge blocks back
                    let next_text = self.blocks[block + 1].text();
                    let current_text = self.blocks[block].text();
                    self.blocks[block].set_text(format!("{}{}", current_text, next_text));
                    self.blocks.remove(block + 1);
                    self.cursor = (block, col);
                    self.redo_stack.push(EditOp::SplitBlock { block, col });
                }
                EditOp::MergeBlocks { index } => {
                    // Split block back - this is complex, skip for now
                    self.redo_stack.push(EditOp::MergeBlocks { index });
                }
                EditOp::ChangeBlockType { block, old, new } => {
                    self.blocks[block].block_type = old;
                    self.redo_stack.push(EditOp::ChangeBlockType {
                        block,
                        old: new,
                        new: old,
                    });
                }
                _ => {}
            }
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            match op {
                EditOp::InsertChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col]
                        .iter()
                        .chain(std::iter::once(&ch))
                        .chain(&chars[col..])
                        .collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col + 1);
                    self.undo_stack.push(EditOp::InsertChar { block, col, ch });
                }
                EditOp::DeleteChar { block, col, ch } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let new_text: String = chars[..col].iter().chain(&chars[col + 1..]).collect();
                    self.blocks[block].set_text(new_text);
                    self.cursor = (block, col);
                    self.undo_stack.push(EditOp::DeleteChar { block, col, ch });
                }
                EditOp::SplitBlock { block, col } => {
                    let text = self.blocks[block].text();
                    let chars: Vec<char> = text.chars().collect();
                    let first_text: String = chars[..col].iter().collect();
                    let second_text: String = chars[col..].iter().collect();
                    self.blocks[block].set_text(first_text);
                    self.blocks.insert(block + 1, Block::paragraph(second_text));
                    self.cursor = (block + 1, 0);
                    self.undo_stack.push(EditOp::SplitBlock { block, col });
                }
                EditOp::ChangeBlockType { block, old, new } => {
                    self.blocks[block].block_type = new;
                    self.undo_stack
                        .push(EditOp::ChangeBlockType { block, old, new });
                }
                _ => {}
            }
        }
    }
}
