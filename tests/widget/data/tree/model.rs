//! Tree against a model of expand, collapse and the selection (#1162)
//!
//! A random forest goes to the tree and to a model that keeps each node's
//! expanded flag and the selected node. After every key the shown rows, the
//! selected node and its row must equal the model's, and the key must report
//! a change exactly when the selection or the rows changed.

use proptest::prelude::*;
use revue::event::Key;
use revue::widget::data::tree::{Tree, TreeNode};

#[derive(Debug, Clone)]
struct Gen {
    expanded: bool,
    selectable: bool,
    children: Vec<Gen>,
}

fn forest() -> impl Strategy<Value = Vec<Gen>> {
    let leaf = (any::<bool>(), prop::bool::weighted(0.8)).prop_map(|(expanded, selectable)| Gen {
        expanded,
        selectable,
        children: vec![],
    });
    let node = leaf.prop_recursive(3, 24, 4, |inner| {
        (
            any::<bool>(),
            prop::bool::weighted(0.8),
            prop::collection::vec(inner, 0..4),
        )
            .prop_map(|(expanded, selectable, children)| Gen {
                expanded,
                selectable,
                children,
            })
    });
    prop::collection::vec(node, 0..4)
}

fn key() -> impl Strategy<Value = Key> {
    prop::sample::select(vec![
        Key::Up,
        Key::Down,
        Key::Left,
        Key::Right,
        Key::Enter,
        Key::Home,
        Key::End,
        Key::Char('j'),
        Key::Char('k'),
        Key::Char('h'),
        Key::Char('l'),
        Key::Char(' '),
    ])
}

/// A node of the model; node `i` is labelled `n{i}`, in pre-order
#[derive(Debug)]
struct Node {
    parent: Option<usize>,
    children: Vec<usize>,
    expanded: bool,
    selectable: bool,
}

#[derive(Debug, Default)]
struct Model {
    nodes: Vec<Node>,
    roots: Vec<usize>,
    selected: Option<usize>,
}

impl Model {
    fn add(&mut self, gens: &[Gen], parent: Option<usize>) -> Vec<TreeNode> {
        gens.iter()
            .map(|g| {
                let id = self.nodes.len();
                self.nodes.push(Node {
                    parent,
                    children: vec![],
                    expanded: g.expanded,
                    selectable: g.selectable,
                });
                match parent {
                    Some(p) => self.nodes[p].children.push(id),
                    None => self.roots.push(id),
                }
                let label = format!("n{id}");
                TreeNode::new(&label)
                    .expanded(g.expanded)
                    .selectable(g.selectable)
                    .children(self.add(&g.children, Some(id)))
            })
            .collect()
    }

    /// The shown nodes: the children of an expanded node follow it
    fn rows(&self) -> Vec<usize> {
        fn walk(m: &Model, ids: &[usize], out: &mut Vec<usize>) {
            for &id in ids {
                out.push(id);
                if m.nodes[id].expanded {
                    walk(m, &m.nodes[id].children, out);
                }
            }
        }
        let mut out = Vec::new();
        walk(self, &self.roots, &mut out);
        out
    }

    fn row(&self) -> Option<usize> {
        let id = self.selected?;
        self.rows().iter().position(|&r| r == id)
    }

    /// The first selectable row in `range`, from its start or its end
    fn find(&self, range: impl DoubleEndedIterator<Item = usize>, back: bool) -> Option<usize> {
        let rows = self.rows();
        let mut range: Box<dyn Iterator<Item = usize>> = if back {
            Box::new(range.rev())
        } else {
            Box::new(range)
        };
        range
            .find(|&i| self.nodes[rows[i]].selectable)
            .map(|i| rows[i])
    }

    /// The selection starts on the first row, moved off a node that cannot
    /// be selected: to the next one that can, else the one before
    fn start(&mut self) {
        let len = self.rows().len();
        self.selected = self.rows().first().copied();
        if let Some(id) = self.selected.filter(|&id| !self.nodes[id].selectable) {
            self.selected = self.find(1..len, false).or(Some(id));
        }
    }

    fn key(&mut self, key: &Key) {
        let Some(id) = self.selected else { return };
        let (len, at) = (self.rows().len(), self.row().unwrap());
        let node = &self.nodes[id];
        let has_children = !node.children.is_empty();
        let to = match key {
            Key::Up | Key::Char('k') => self.find(0..at, true),
            Key::Down | Key::Char('j') => self.find(at + 1..len, false),
            Key::Home => self.find(0..len, false),
            Key::End => self.find(0..len, true),
            Key::Enter | Key::Char(' ') if has_children => {
                self.nodes[id].expanded ^= true;
                None
            }
            Key::Right | Key::Char('l') if has_children => {
                self.nodes[id].expanded = true;
                None
            }
            Key::Left | Key::Char('h') if has_children && node.expanded => {
                self.nodes[id].expanded = false;
                None
            }
            // Else to the nearest ancestor that can be selected
            Key::Left | Key::Char('h') => {
                std::iter::successors(node.parent, |&p| self.nodes[p].parent)
                    .find(|&p| self.nodes[p].selectable)
            }
            _ => None,
        };
        if let Some(to) = to {
            self.selected = Some(to);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Any forest and keys: the rows, the selected node and its row equal
    /// the model's after each key. Collapsing and expanding keep the
    /// selection on its node and the expand state of the nodes inside.
    #[test]
    fn tree_follows_the_model(gens in forest(), keys in prop::collection::vec(key(), 0..40)) {
        let mut m = Model::default();
        let mut tree = Tree::new().nodes(m.add(&gens, None));
        m.start();
        for key in &keys {
            let before = (m.selected, m.rows().len());
            let changed = tree.handle_key(key);
            m.key(key);
            let rows = m.rows();
            prop_assert_eq!(tree.visible_count(), rows.len(), "after {:?}", key);
            prop_assert_eq!(
                tree.selected_label().map(str::to_string),
                m.selected.map(|id| format!("n{id}")),
                "after {:?}", key
            );
            prop_assert_eq!(tree.selected_index(), m.row().unwrap_or(0), "after {:?}", key);
            prop_assert_eq!(changed, before != (m.selected, rows.len()), "{:?} reported", key);
        }
    }
}
