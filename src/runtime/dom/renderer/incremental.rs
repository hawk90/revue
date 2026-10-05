//! Incremental DOM update logic for DomRenderer

use crate::dom::renderer::build::build_children_internal;
use crate::dom::renderer::types::DomRenderer;
use crate::dom::DomId;
use crate::dom::{DomTree, WidgetKey, WidgetMeta};
use crate::widget::View;

impl DomRenderer {
    /// Build DOM from a View hierarchy
    ///
    /// Builds from scratch on the first call and reconciles against the
    /// existing tree afterwards, reusing nodes that match by [`WidgetKey`],
    /// element id, or position + widget type - in that order. Reused nodes keep
    /// their [`DomId`], their state (focus, hover, selection) and their cached
    /// style.
    pub fn build<V: View>(&mut self, root: &V) {
        if self.tree.is_empty() {
            // First build - create from scratch
            self.build_fresh(root);
        } else {
            // Incremental update
            self.build_incremental(root);
        }
    }

    /// Incremental DOM update - reuses existing nodes when possible
    pub(crate) fn build_incremental<V: View>(&mut self, root: &V) {
        let Some(root_id) = self.tree.root_id() else {
            // No root, do fresh build
            self.build_fresh(root);
            return;
        };

        // Update root node
        let new_meta = root.meta();
        if !update_node_meta_matched(self, root_id, &new_meta, MatchKind::Positional) {
            // Root changed - full rebuild needed
            self.build_fresh(root);
            return;
        }

        // Recursively update children
        update_children_internal(self, root_id, root.children());
    }

    /// Remove a node and its entire subtree
    pub(crate) fn remove_subtree(&mut self, node_id: DomId) {
        // Collect all descendant IDs first
        let descendants = collect_descendants_internal(&self.tree, node_id);

        // Remove styles for all nodes
        self.styles.remove(&node_id);
        for &id in &descendants {
            self.styles.remove(&id);
        }

        // Remove from tree
        self.tree.remove(node_id);
    }
}

/// How an existing node was matched to a new child view.
///
/// The match kind decides how much of the node's metadata may change while the
/// node is still considered "the same widget".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchKind {
    /// Matched by [`WidgetKey`]. The key *is* the identity, so the element id
    /// may change freely.
    Keyed,
    /// Matched by element id. The ids are equal by construction.
    ById,
    /// Matched by position and widget type. This is a guess, so an element id
    /// mismatch means it guessed wrong.
    Positional,
}

/// Update a matched node's metadata, or reject the match.
///
/// Returns `false` when the node cannot represent `new_meta`; the caller then
/// removes it and builds a fresh one.
fn update_node_meta_matched(
    renderer: &mut DomRenderer,
    node_id: DomId,
    new_meta: &WidgetMeta,
    how: MatchKind,
) -> bool {
    let Some(node) = renderer.tree.get(node_id) else {
        return false;
    };

    // Widget type must match under every match kind - a Button cannot become a
    // Table and keep its state.
    if node.meta.widget_type != new_meta.widget_type {
        return false;
    }

    // A positional match is a guess based on index alone. If the ids disagree,
    // the guess was wrong. A keyed match carries its own identity, so the id is
    // free to change; an id match already agrees by construction.
    if how == MatchKind::Positional && node.meta.id != new_meta.id {
        return false;
    }

    // `apply_meta` keeps `id_map` and `class_index` in step - writing
    // `node.meta` directly here would leave both stale.
    if renderer.tree.apply_meta(node_id, new_meta) {
        if let Some(node) = renderer.tree.get_mut(node_id) {
            node.state.dirty = true;
        }
        renderer.styles.remove(&node_id);
        // The style walk turns back at settled nodes, so an unchanged ancestor
        // would hide this one - which is how a class added after the first
        // frame used to change nothing at all.
        renderer.tree.mark_subtree_dirty(node_id);
        // A `+` or `~` rule keyed on this node's class matches its siblings,
        // not itself.
        renderer.invalidate_following_siblings(node_id);
    }

    true
}

/// Decide, for each new child, which existing child *slot* it should reuse.
///
/// Runs against an immutably borrowed tree and mutates nothing. That is what
/// lets the lookup tables borrow element ids and keys straight out of the
/// existing nodes: reconciliation runs on every frame once enabled, and cloning
/// every sibling's id and widget type per frame was the single largest cost in
/// the first version of this code.
///
/// Results are indices into `old_children`, not [`DomId`]s, so the caller can
/// track what has been claimed with a `Vec<bool>` instead of a hash set.
fn plan_matches(
    tree: &DomTree,
    old_children: &[DomId],
    new_children: &[&WidgetMeta],
    claimed: &mut [bool],
) -> Vec<Option<(usize, MatchKind)>> {
    // Only build a lookup table that something will actually read. A subtree
    // whose children carry no keys should not pay for a key map.
    let want_keys = new_children.iter().any(|m| m.key.is_some());
    let want_ids = new_children.iter().any(|m| m.id.is_some());

    let mut by_key: std::collections::HashMap<&WidgetKey, usize> = std::collections::HashMap::new();
    let mut by_id: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

    if want_keys || want_ids {
        for (idx, &child_id) in old_children.iter().enumerate() {
            let Some(node) = tree.get(child_id) else {
                continue;
            };
            if want_keys {
                if let Some(ref key) = node.meta.key {
                    by_key.entry(key).or_insert(idx);
                }
            }
            if want_ids {
                if let Some(ref id) = node.meta.id {
                    by_id.insert(id.as_str(), idx);
                }
            }
        }
    }

    let mut plan = Vec::with_capacity(new_children.len());

    for (pos, child) in new_children.iter().enumerate() {
        // Matching priority: key > element id > position + widget type.
        //
        // A key is an explicit identity claim, so it outranks everything. An
        // element id is unique per document and therefore nearly as good. A
        // position match is the fallback, and the reason unkeyed collections
        // reconcile badly.
        let matched = if let Some(key) = child.key.as_ref() {
            by_key.get(key).copied().map(|i| (i, MatchKind::Keyed))
        } else if let Some(id) = child.id.as_deref() {
            by_id.get(id).copied().map(|i| (i, MatchKind::ById))
        } else {
            old_children
                .get(pos)
                .and_then(|&old_id| {
                    let node = tree.get(old_id)?;
                    // A node that carries a key must not be silently reused by
                    // a keyless sibling that happens to land on its index -
                    // that is the shift-by-one bug keys exist to prevent.
                    (node.meta.key.is_none() && node.meta.widget_type == child.widget_type)
                        .then_some(pos)
                })
                .map(|i| (i, MatchKind::Positional))
        };

        // A slot already claimed by an earlier sibling (a duplicate key) is not
        // available; that child gets a fresh node instead.
        let matched = matched.filter(|&(i, _)| {
            if claimed[i] {
                false
            } else {
                claimed[i] = true;
                true
            }
        });

        plan.push(matched);
    }

    plan
}

/// Standalone function to recursively update children
fn update_children_internal(
    renderer: &mut DomRenderer,
    parent_id: DomId,
    new_children: &[Box<dyn View>],
) {
    // Get current children IDs
    let old_children: Vec<DomId> = renderer
        .tree
        .get(parent_id)
        .map(|n| n.children.clone())
        .unwrap_or_default();

    let metas: Vec<WidgetMeta> = new_children.iter().map(|c| c.meta()).collect();
    let meta_refs: Vec<&WidgetMeta> = metas.iter().collect();

    let mut claimed = vec![false; old_children.len()];
    let plan = plan_matches(&renderer.tree, &old_children, &meta_refs, &mut claimed);

    let mut new_child_ids: Vec<DomId> = Vec::with_capacity(new_children.len());

    for ((child_view, child_meta), matched) in new_children.iter().zip(metas).zip(plan) {
        let child_id = match matched {
            Some((slot, how)) => {
                let existing_id = old_children[slot];
                if update_node_meta_matched(renderer, existing_id, &child_meta, how) {
                    update_children_internal(renderer, existing_id, child_view.children());
                    existing_id
                } else {
                    // Type mismatch - remove old and create new. The slot stays
                    // claimed: the node is gone, so the sweep below must not
                    // try to remove it a second time.
                    renderer.remove_subtree(existing_id);
                    renderer.structure_dirty = true;
                    let new_id = renderer.tree.add_child(parent_id, child_meta);
                    build_children_internal(renderer, new_id, child_view.children());
                    new_id
                }
            }
            None => {
                renderer.structure_dirty = true;
                let new_id = renderer.tree.add_child(parent_id, child_meta);
                build_children_internal(renderer, new_id, child_view.children());
                new_id
            }
        };

        new_child_ids.push(child_id);
    }

    // Remove the old children nothing claimed.
    for (slot, &old_id) in old_children.iter().enumerate() {
        if !claimed[slot] {
            renderer.remove_subtree(old_id);
            renderer.structure_dirty = true;
        }
    }

    // A reorder changes nothing about which nodes exist, but layout reads the
    // child order, so it counts as a structural change too.
    if new_child_ids != old_children {
        renderer.structure_dirty = true;
    }

    // `set_children` refreshes each child's structural state, which
    // `:first-child` and friends read. Assigning `parent.children` directly
    // would leave that state describing the previous frame's order.
    renderer.tree.set_children(parent_id, new_child_ids);
}

/// Standalone function to collect all descendant node IDs
fn collect_descendants_internal(tree: &crate::dom::DomTree, node_id: DomId) -> Vec<DomId> {
    let mut result = Vec::new();
    let mut stack = vec![node_id];

    while let Some(id) = stack.pop() {
        if let Some(node) = tree.get(id) {
            for &child_id in &node.children {
                result.push(child_id);
                stack.push(child_id);
            }
        }
    }

    result
}

/// Reconcile one level of children described by metadata rather than by views.
///
/// Used by the collect pass, which has already flattened the render traversal
/// and walks it breadth-first - so unlike [`update_children_internal`] this
/// neither recurses nor builds grandchildren. Returns the node assigned to each
/// child, in order.
pub(crate) fn reconcile_children_from_metas(
    renderer: &mut DomRenderer,
    parent_id: DomId,
    new_children: &[&WidgetMeta],
) -> Vec<DomId> {
    let old_children: Vec<DomId> = renderer
        .tree
        .get(parent_id)
        .map(|n| n.children.clone())
        .unwrap_or_default();

    let mut claimed = vec![false; old_children.len()];
    let plan = plan_matches(&renderer.tree, &old_children, new_children, &mut claimed);

    let mut new_child_ids: Vec<DomId> = Vec::with_capacity(new_children.len());

    for (meta, matched) in new_children.iter().zip(plan) {
        let child_id = match matched {
            Some((slot, how)) => {
                let existing_id = old_children[slot];
                if update_node_meta_matched(renderer, existing_id, meta, how) {
                    existing_id
                } else {
                    renderer.remove_subtree(existing_id);
                    renderer.structure_dirty = true;
                    renderer.tree.add_child(parent_id, (*meta).clone())
                }
            }
            None => {
                renderer.structure_dirty = true;
                renderer.tree.add_child(parent_id, (*meta).clone())
            }
        };
        new_child_ids.push(child_id);
    }

    for (slot, &old_id) in old_children.iter().enumerate() {
        if !claimed[slot] {
            renderer.remove_subtree(old_id);
            renderer.structure_dirty = true;
        }
    }

    if new_child_ids != old_children {
        renderer.structure_dirty = true;
    }

    renderer.tree.set_children(parent_id, new_child_ids.clone());
    new_child_ids
}
