//! The visible rows of a tree: every root, and the children of every
//! expanded node, in pre-order

use super::types::TreeNode;
use super::Tree;

/// One visible row
pub(super) struct Row<'a> {
    pub(super) node: &'a TreeNode,
    pub(super) depth: usize,
    /// Row of the parent node; `None` for a root
    pub(super) parent: Option<usize>,
    /// Whether the node is the last of its siblings
    pub(super) is_last: bool,
    /// Child positions from the roots down to the node: its identity while
    /// rows come and go with expanding and collapsing (#975)
    pub(super) path: Vec<usize>,
}

impl Tree {
    /// The visible rows, in the order they are drawn
    pub(super) fn rows(&self) -> Vec<Row<'_>> {
        fn walk<'a>(
            nodes: &'a [TreeNode],
            depth: usize,
            parent: Option<usize>,
            path: &mut Vec<usize>,
            out: &mut Vec<Row<'a>>,
        ) {
            for (i, node) in nodes.iter().enumerate() {
                let index = out.len();
                path.push(i);
                out.push(Row {
                    node,
                    depth,
                    parent,
                    is_last: i == nodes.len() - 1,
                    path: path.clone(),
                });
                if node.expanded && !node.children.is_empty() {
                    walk(&node.children, depth + 1, Some(index), path, out);
                }
                path.pop();
            }
        }
        let mut out = Vec::new();
        walk(&self.root, 0, None, &mut Vec::new(), &mut out);
        out
    }

    /// Count visible nodes (expanded recursively)
    pub(super) fn count_visible(&self) -> usize {
        self.rows().len()
    }

    /// Get node at visible index
    pub(super) fn get_node_at(&self, index: usize) -> Option<(&TreeNode, usize)> {
        self.rows().get(index).map(|row| (row.node, row.depth))
    }

    /// Get mutable node at visible index
    pub(super) fn get_node_mut_at(&mut self, index: usize) -> Option<&mut TreeNode> {
        // Child positions from the roots down to the row, found on the
        // shared walk, then followed mutably
        let path = {
            let rows = self.rows();
            rows.get(index)?;
            let mut path = Vec::new();
            let mut current = Some(index);
            while let Some(row) = current {
                let siblings = match rows[row].parent {
                    Some(p) => &rows[p].node.children,
                    None => &self.root,
                };
                let position = siblings
                    .iter()
                    .position(|n| std::ptr::eq(n, rows[row].node))?;
                path.push(position);
                current = rows[row].parent;
            }
            path
        };
        let mut nodes = &mut self.root;
        let (&last, ancestors) = path.split_first()?;
        for &i in ancestors.iter().rev() {
            nodes = &mut nodes[i].children;
        }
        nodes.get_mut(last)
    }

    /// `selectable` flag of every visible node, in visible order
    pub(super) fn selectable_flags(&self) -> Vec<bool> {
        self.rows().iter().map(|row| row.node.selectable).collect()
    }

    /// The node at `path`, shown or not
    pub(super) fn node_at_path(&self, path: &[usize]) -> Option<&TreeNode> {
        let (&first, rest) = path.split_first()?;
        let mut node = self.root.get(first)?;
        for &i in rest {
            node = node.children.get(i)?;
        }
        Some(node)
    }

    /// Find the parent index of the node at the given visible index
    pub(super) fn find_parent_index(&self, index: usize) -> Option<usize> {
        self.rows().get(index)?.parent
    }
}
