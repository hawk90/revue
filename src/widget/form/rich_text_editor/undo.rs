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
                    self.redo_stack.push(EditOp::InsertBlock {
                        index,
                        block: removed,
                    });
                }
                EditOp::DeleteBlock { index, block } => {
                    self.blocks.insert(index, block.clone());
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
                    self.undo_stack.push(EditOp::InsertBlock { index, block });
                }
                EditOp::DeleteBlock { index, block: _ } => {
                    let removed = self.blocks.remove(index);
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
