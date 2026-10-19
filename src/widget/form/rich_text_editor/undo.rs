//! RichTextEditor undo/redo functionality
//!
//! This module contains the undo and redo system.

use super::RichTextEditor;
use super::{Block, EditOp, TextFormat};

/// Maximum undo history size
pub const MAX_UNDO_HISTORY: usize = 100;

impl RichTextEditor {
    /// Undo last operation
    pub fn undo(&mut self) {
        if let Some(op) = self.undo_stack.pop() {
            if !self.undo_fits(&op) {
                self.forget_history();
                return;
            }
            match op {
                EditOp::InsertChar { block, col, ch } => {
                    self.blocks[block].remove_char_at(col);
                    self.cursor = (block, col);
                    self.redo_stack.push(EditOp::InsertChar { block, col, ch });
                }
                EditOp::DeleteChar { block, col, ch } => {
                    self.blocks[block].insert_char_at(col, ch);
                    self.cursor = (block, col + 1);
                    self.redo_stack.push(EditOp::DeleteChar { block, col, ch });
                }
                EditOp::SplitBlock { block, col } => {
                    // Merge blocks back
                    self.join_with_next(block);
                    self.cursor = (block, col);
                    self.redo_stack.push(EditOp::SplitBlock { block, col });
                }
                EditOp::InsertBlock { index, block: _ } => {
                    let removed = self.blocks.remove(index);
                    self.cursor = (index.min(self.blocks.len() - 1), 0);
                    self.redo_stack.push(EditOp::InsertBlock {
                        index,
                        block: removed,
                    });
                }
                EditOp::DeleteBlock { index, block } => {
                    self.blocks.insert(index, block.clone());
                    self.cursor = (index, 0);
                    self.redo_stack.push(EditOp::DeleteBlock { index, block });
                }
                EditOp::SetFormat {
                    block,
                    start,
                    end,
                    old,
                    new: new_fmt,
                } => {
                    // Apply old format to the affected span range
                    self.set_range_format(block, start, end, old);
                    self.redo_stack.push(EditOp::SetFormat {
                        block,
                        start,
                        end,
                        old: new_fmt,
                        new: old,
                    });
                }
                EditOp::MergeBlocks { index, split_col } => {
                    // Undo merge: split the block at the stored column
                    let (first_text, second_text) = self.blocks[index].split_text_at(split_col);

                    // Create new blocks preserving the block type
                    let original_type = self.blocks[index].block_type;

                    self.blocks[index].set_text(first_text);
                    let mut new_block = Block::new(original_type);
                    new_block.set_text(second_text);
                    self.blocks.insert(index + 1, new_block);

                    self.cursor = (index, split_col);
                    self.redo_stack
                        .push(EditOp::MergeBlocks { index, split_col });
                }
                EditOp::ChangeBlockType { block, old, new } => {
                    self.blocks[block].block_type = old;
                    self.redo_stack.push(EditOp::ChangeBlockType {
                        block,
                        old: new,
                        new: old,
                    });
                }
            }
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) {
        if let Some(op) = self.redo_stack.pop() {
            if !self.redo_fits(&op) {
                self.forget_history();
                return;
            }
            match op {
                EditOp::InsertChar { block, col, ch } => {
                    self.blocks[block].insert_char_at(col, ch);
                    self.cursor = (block, col + 1);
                    self.undo_stack.push(EditOp::InsertChar { block, col, ch });
                }
                EditOp::DeleteChar { block, col, ch } => {
                    self.blocks[block].remove_char_at(col);
                    self.cursor = (block, col);
                    self.undo_stack.push(EditOp::DeleteChar { block, col, ch });
                }
                EditOp::SplitBlock { block, col } => {
                    let (first_text, second_text) = self.blocks[block].split_text_at(col);
                    self.blocks[block].set_text(first_text);
                    self.blocks.insert(block + 1, Block::paragraph(second_text));
                    self.cursor = (block + 1, 0);
                    self.undo_stack.push(EditOp::SplitBlock { block, col });
                }
                EditOp::InsertBlock { index, block } => {
                    self.blocks.insert(index, block.clone());
                    self.cursor = (index, 0);
                    self.undo_stack.push(EditOp::InsertBlock { index, block });
                }
                EditOp::DeleteBlock { index, block: _ } => {
                    let removed = self.blocks.remove(index);
                    self.cursor = (index.min(self.blocks.len() - 1), 0);
                    self.undo_stack.push(EditOp::DeleteBlock {
                        index,
                        block: removed,
                    });
                }
                EditOp::SetFormat {
                    block,
                    start,
                    end,
                    old: old_fmt,
                    new,
                } => {
                    // Apply new format to the affected span range
                    self.set_range_format(block, start, end, new);
                    self.undo_stack.push(EditOp::SetFormat {
                        block,
                        start,
                        end,
                        old: old_fmt,
                        new,
                    });
                }
                EditOp::MergeBlocks { index, split_col } => {
                    // Redo merge: merge the block with the next one
                    self.join_with_next(index);

                    self.cursor = (index, split_col);
                    self.undo_stack
                        .push(EditOp::MergeBlocks { index, split_col });
                }
                EditOp::ChangeBlockType { block, old, new } => {
                    self.blocks[block].block_type = new;
                    self.undo_stack
                        .push(EditOp::ChangeBlockType { block, old, new });
                }
            }
        }
    }

    /// The char count of block `block`, if it exists
    fn chars_in(&self, block: usize) -> Option<usize> {
        self.blocks.get(block).map(Block::char_count)
    }

    /// Whether undoing `op` stays inside the blocks; an op that no longer
    /// matches them is dropped with the history instead of panicking (#887)
    fn undo_fits(&self, op: &EditOp) -> bool {
        let len = self.blocks.len();
        match *op {
            EditOp::InsertChar { block, col, .. } => self.chars_in(block).is_some_and(|n| col < n),
            EditOp::DeleteChar { block, col, .. } => self.chars_in(block).is_some_and(|n| col <= n),
            EditOp::SplitBlock { block, .. } => block + 1 < len,
            EditOp::InsertBlock { index, .. } => index < len && len > 1,
            EditOp::DeleteBlock { index, .. } => index <= len,
            EditOp::MergeBlocks { index, split_col } => {
                self.chars_in(index).is_some_and(|n| split_col <= n)
            }
            EditOp::SetFormat { block, .. } | EditOp::ChangeBlockType { block, .. } => block < len,
        }
    }

    /// Whether redoing `op` stays inside the blocks; see `undo_fits`
    fn redo_fits(&self, op: &EditOp) -> bool {
        let len = self.blocks.len();
        match *op {
            EditOp::InsertChar { block, col, .. } => self.chars_in(block).is_some_and(|n| col <= n),
            EditOp::DeleteChar { block, col, .. } => self.chars_in(block).is_some_and(|n| col < n),
            EditOp::SplitBlock { block, col } => self.chars_in(block).is_some_and(|n| col <= n),
            EditOp::InsertBlock { index, .. } => index <= len,
            EditOp::DeleteBlock { index, .. } => index < len && len > 1,
            EditOp::MergeBlocks { index, .. } => index + 1 < len,
            EditOp::SetFormat { block, .. } | EditOp::ChangeBlockType { block, .. } => block < len,
        }
    }

    /// Drop the undo and redo history after an edit that is not recorded,
    /// so undo never replays an operation against blocks it no longer
    /// matches (#887)
    pub(super) fn forget_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Give `format` to every span of `block` that overlaps chars
    /// `start..end`
    fn set_range_format(&mut self, block: usize, start: usize, end: usize, format: TextFormat) {
        let block_ref = &mut self.blocks[block];
        let mut char_idx = 0;
        for span in &mut block_ref.spans {
            let span_start = char_idx;
            let span_end = char_idx + span.text.chars().count();
            // Check if this span overlaps with the format range
            if span_end > start && span_start < end {
                span.format = format;
            }
            char_idx = span_end;
        }
    }
}
