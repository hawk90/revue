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
            self.clear_selection();
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
                EditOp::ReplaceBlocks {
                    index,
                    old,
                    new,
                    cursor_before,
                    cursor_after,
                } => {
                    self.blocks
                        .splice(index..index + new.len(), old.iter().cloned());
                    self.cursor = cursor_before;
                    self.redo_stack.push(EditOp::ReplaceBlocks {
                        index,
                        old,
                        new,
                        cursor_before,
                        cursor_after,
                    });
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
            self.clear_selection();
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
                EditOp::ReplaceBlocks {
                    index,
                    old,
                    new,
                    cursor_before,
                    cursor_after,
                } => {
                    self.blocks
                        .splice(index..index + old.len(), new.iter().cloned());
                    self.cursor = cursor_after;
                    self.undo_stack.push(EditOp::ReplaceBlocks {
                        index,
                        old,
                        new,
                        cursor_before,
                        cursor_after,
                    });
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

    /// The char count of blocks `block` and `block + 1` joined, if both
    /// exist
    fn joined_chars(&self, block: usize) -> Option<usize> {
        Some(self.chars_in(block)? + self.chars_in(block + 1)?)
    }

    /// Whether undoing `op` stays inside the blocks; an op that no longer
    /// matches them is dropped with the history instead of panicking (#887)
    fn undo_fits(&self, op: &EditOp) -> bool {
        let len = self.blocks.len();
        match *op {
            EditOp::InsertChar { block, col, .. } => self.chars_in(block).is_some_and(|n| col < n),
            EditOp::DeleteChar { block, col, .. } => self.chars_in(block).is_some_and(|n| col <= n),
            // The cursor lands on `col` of the joined block
            EditOp::SplitBlock { block, col } => self.joined_chars(block).is_some_and(|n| col <= n),
            EditOp::InsertBlock { index, .. } => index < len && len > 1,
            EditOp::DeleteBlock { index, .. } => index <= len,
            EditOp::ReplaceBlocks { index, ref new, .. } => index + new.len() <= len,
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
            EditOp::ReplaceBlocks { index, ref old, .. } => index + old.len() <= len,
            EditOp::SetFormat { block, .. } | EditOp::ChangeBlockType { block, .. } => block < len,
        }
    }

    /// Make `edit` to blocks `index..index + count` one undo step: the blocks
    /// before and after it are kept, with the cursor (#1131)
    pub(super) fn record_blocks(
        &mut self,
        index: usize,
        count: usize,
        edit: impl FnOnce(&mut Self),
    ) {
        let len_before = self.blocks.len();
        let old = self.blocks[index..index + count].to_vec();
        let cursor_before = self.cursor;
        edit(self);
        let new_count = (count + self.blocks.len()).saturating_sub(len_before);
        let new = self.blocks[index..index + new_count].to_vec();
        self.undo_stack.push(EditOp::ReplaceBlocks {
            index,
            old,
            new,
            cursor_before,
            cursor_after: self.cursor,
        });
        if self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
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
