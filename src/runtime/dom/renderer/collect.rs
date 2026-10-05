//! Building the DOM from the render traversal.
//!
//! The DOM used to be built by walking [`View::children`], which almost nothing
//! implements: the idiomatic widget assembles its tree inside `render` and
//! returns an empty slice from `children()`. A real application therefore had a
//! DOM of one node, and CSS matching, `:focus`/`:hover` and devtools all
//! operated on a tree that did not describe it.
//!
//! The render traversal *is* the tree, so that is where the DOM is built now.
//! A frame renders the view twice:
//!
//! ```text
//! collect pass  ->  reconcile DOM  ->  compute styles  ->  paint pass
//! ```
//!
//! The collect pass records what each widget renders, in order. The paint pass
//! walks the same traversal and hands every widget the computed style and state
//! of its own node.
//!
//! Two passes rather than one because a node's style depends on the whole tree -
//! `:last-child` and `:nth-child` cannot be resolved until the siblings are
//! known. Rendering into a buffer is memory traffic, so the second traversal is
//! cheap next to the correctness it buys; the collect pass paints into the back
//! buffer and the paint pass clears it first.
//!
//! The two traversals are aligned by a counter, which assumes `View::render` is
//! deterministic for a given `&self`. That holds for every widget in this crate
//! and is the same assumption `render` already makes by taking `&self`.

use std::collections::HashMap;

use super::types::DomRenderer;
use crate::dom::{DomId, WidgetMeta};

/// One widget seen by the collect pass.
pub(crate) struct CollectedNode {
    pub(crate) meta: WidgetMeta,
    /// Index of the parent in [`CollectSink::nodes`]; `None` for the root.
    pub(crate) parent: Option<usize>,
}

/// What the collect pass records.
#[derive(Default)]
pub struct CollectSink {
    pub(crate) nodes: Vec<CollectedNode>,
}

impl CollectSink {
    /// Create an empty sink.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record the root, which has no parent.
    pub fn push_root(&mut self, meta: WidgetMeta) -> usize {
        self.push(meta, None)
    }

    /// Record a widget and return its index.
    pub fn push(&mut self, meta: WidgetMeta, parent: Option<usize>) -> usize {
        self.nodes.push(CollectedNode { meta, parent });
        self.nodes.len() - 1
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Size of each node's subtree, itself included, in collection order.
    ///
    /// The sink is pre-order, so a parent always precedes its descendants and
    /// a subtree occupies a contiguous run. Accumulating from the back
    /// therefore fills every child before its parent in a single pass.
    pub(crate) fn subtree_lens(&self) -> Vec<usize> {
        let mut lens = vec![1usize; self.nodes.len()];
        for idx in (1..self.nodes.len()).rev() {
            if let Some(parent) = self.nodes[idx].parent {
                lens[parent] += lens[idx];
            }
        }
        lens
    }
}

impl DomRenderer {
    /// Reconcile the DOM against what the collect pass recorded.
    ///
    /// Returns the [`DomId`] of every collected node, in collection order, so
    /// the paint pass can align to it with a counter.
    ///
    /// Matching follows the same priority as [`DomRenderer::build`]: key,
    /// then element id, then position and widget type. Nodes that survive keep
    /// their `DomId`, their state and their cached style.
    pub(crate) fn reconcile_collected(&mut self, sink: &CollectSink) -> Vec<DomId> {
        if sink.is_empty() {
            return Vec::new();
        }

        // Children of each collected node, in order.
        let mut children: Vec<Vec<usize>> = vec![Vec::new(); sink.nodes.len()];
        for (idx, node) in sink.nodes.iter().enumerate() {
            if let Some(parent) = node.parent {
                children[parent].push(idx);
            }
        }

        // The root. A different root widget type means nothing below it can be
        // reused, which is what `build_fresh` would have concluded too.
        let root_meta = &sink.nodes[0].meta;
        let root_id = match self.tree.root_id() {
            Some(id)
                if self
                    .tree
                    .get(id)
                    .is_some_and(|n| n.meta.widget_type == root_meta.widget_type) =>
            {
                if self.tree.apply_meta(id, root_meta) {
                    // The walk starts here, so there is no ancestor to mark -
                    // but the cache still answers before the dirty flag, so the
                    // entry has to go.
                    self.styles.remove(&id);
                    if let Some(node) = self.tree.get_mut(id) {
                        node.state.dirty = true;
                    }
                }
                id
            }
            _ => {
                self.tree = crate::dom::DomTree::new();
                self.styles.clear();
                self.structure_dirty = true;
                self.tree.create_root(root_meta.clone())
            }
        };

        let mut ids: HashMap<usize, DomId> = HashMap::with_capacity(sink.nodes.len());
        ids.insert(0, root_id);

        // Breadth-first so a parent's DomId is always known before its children.
        let mut queue = vec![0usize];
        while let Some(idx) = queue.pop() {
            let parent_id = ids[&idx];
            let kids = &children[idx];
            if kids.is_empty() {
                // A subtree that disappeared this frame takes its state with it.
                if !self
                    .tree
                    .get(parent_id)
                    .is_some_and(|n| n.children.is_empty())
                {
                    let stale: Vec<DomId> = self
                        .tree
                        .get(parent_id)
                        .map(|n| n.children.clone())
                        .unwrap_or_default();
                    for id in stale {
                        self.remove_subtree(id);
                        self.structure_dirty = true;
                    }
                    self.tree.set_children(parent_id, Vec::new());
                }
                continue;
            }

            let metas: Vec<&WidgetMeta> = kids.iter().map(|&k| &sink.nodes[k].meta).collect();
            let assigned =
                super::incremental::reconcile_children_from_metas(self, parent_id, &metas);

            for (&k, id) in kids.iter().zip(assigned) {
                ids.insert(k, id);
                queue.push(k);
            }
        }

        (0..sink.nodes.len()).map(|i| ids[&i]).collect()
    }
}
