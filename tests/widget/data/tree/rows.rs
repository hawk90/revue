//! Matrix: random forests, expand states, selectable flags and keys
//!
//! Labels are numbered in pre-order over the whole forest, so the visible
//! rows can be checked against the model without reading private state. The
//! drawing is compared when every row fits; the view following the
//! selection in a shorter area is tested in `navigation.rs`.

use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::tree::{Tree, TreeNode};
use revue::widget::traits::{RenderContext, View};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    expanded: bool,
    selectable: bool,
    children: Vec<Node>,
}

fn node() -> impl Strategy<Value = Node> {
    let leaf =
        (any::<bool>(), prop::bool::weighted(0.85)).prop_map(|(expanded, selectable)| Node {
            expanded,
            selectable,
            children: vec![],
        });
    leaf.prop_recursive(3, 24, 4, |inner| {
        (
            any::<bool>(),
            prop::bool::weighted(0.85),
            prop::collection::vec(inner, 0..4),
        )
            .prop_map(|(expanded, selectable, children)| Node {
                expanded,
                selectable,
                children,
            })
    })
}

/// Labels are `n0`, `n1`, ... in pre-order over the whole forest
fn build(nodes: &[Node], next: &mut usize) -> Vec<TreeNode> {
    nodes
        .iter()
        .map(|n| {
            let label = format!("n{next}");
            *next += 1;
            TreeNode::new(&label)
                .id(&label)
                .expanded(n.expanded)
                .selectable(n.selectable)
                .children(build(&n.children, next))
        })
        .collect()
}

/// Parent label of every node, by label
fn parents(nodes: &[TreeNode], parent: Option<&str>, out: &mut HashMap<String, Option<String>>) {
    for node in nodes {
        out.insert(node.label.clone(), parent.map(str::to_string));
        parents(&node.children, Some(&node.label), out);
    }
}

/// Label of visible row `index`, read through the public selection
fn label_at(tree: &Tree, index: usize) -> Option<String> {
    let probe = tree.clone().selected(index);
    (probe.selected_index() == index)
        .then(|| probe.selected_label().map(str::to_string))
        .flatten()
}

fn keys() -> impl Strategy<Value = Vec<Key>> {
    prop::collection::vec(
        prop_oneof![
            Just(Key::Up),
            Just(Key::Down),
            Just(Key::Left),
            Just(Key::Right),
            Just(Key::Enter),
            Just(Key::Home),
            Just(Key::End),
        ],
        0..20,
    )
}

fn render(tree: &Tree, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width.max(1), height.max(1));
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    tree.render(&mut ctx);
    buffer
}

fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// After any keys: the visible rows are nodes in pre-order, each with its
    /// parent shown above it; the selection is a visible row; and when every
    /// row fits, row y shows the label of row y and only the selected row is
    /// highlighted.
    #[test]
    fn rows_follow_the_model(
        roots in prop::collection::vec(node(), 0..4),
        keys in keys(),
        multi in any::<bool>(),
    ) {
        let mut next = 0;
        let mut tree = Tree::new()
            .nodes(build(&roots, &mut next))
            .multi_select(multi)
            .selected_style(Color::BLACK, Color::CYAN);
        for key in &keys {
            tree.handle_key(key);
        }

        let mut parent_of = HashMap::new();
        parents(&build(&roots, &mut 0), None, &mut parent_of);
        let rows: Vec<String> = (0..tree.visible_count())
            .map(|index| label_at(&tree, index))
            .collect::<Option<_>>()
            .expect("every visible row has a node");
        let numbers: Vec<usize> = rows.iter().map(|l| l[1..].parse().unwrap()).collect();
        prop_assert!(numbers.windows(2).all(|p| p[0] < p[1]), "not pre-order: {:?}", rows);
        for (i, label) in rows.iter().enumerate() {
            if let Some(parent) = &parent_of[label] {
                let at = rows.iter().position(|r| r == parent);
                prop_assert!(at.is_some_and(|p| p < i), "{} shows without its parent {}: {:?}", label, parent, rows);
            }
        }

        if tree.visible_count() > 0 {
            prop_assert!(tree.selected_index() < tree.visible_count());
        }

        let height = tree.visible_count() as u16;
        if height > 0 {
            let buffer = render(&tree, 60, height);
            for (y, label) in rows.iter().enumerate() {
                prop_assert!(row_text(&buffer, y as u16).ends_with(label.as_str()), "row {}: {:?}", y, row_text(&buffer, y as u16));
            }
            let lit: Vec<u16> = (0..height)
                .filter(|&y| buffer.get(59, y).unwrap().bg == Some(Color::CYAN))
                .collect();
            prop_assert_eq!(lit, vec![tree.selected_index() as u16]);
        }
    }

    /// Any forest, indent and area: rendering never draws outside its area
    #[test]
    fn render_stays_inside_its_area(
        roots in prop::collection::vec(node(), 0..4),
        keys in keys(),
        indent in 0u16..6,
        w in 0u16..30,
        h in 0u16..8,
    ) {
        let mut next = 0;
        let mut tree = Tree::new().nodes(build(&roots, &mut next)).indent(indent);
        for key in &keys {
            tree.handle_key(key);
        }
        let sentinel = '#';
        let mut buffer = Buffer::new(w + 4, h + 4);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                buffer.set(x, y, revue::render::Cell::new(sentinel));
            }
        }
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 2, w, h));
        tree.render(&mut ctx);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                let inside = (2..2 + w).contains(&x) && (2..2 + h).contains(&y);
                if !inside {
                    prop_assert_eq!(buffer.get(x, y).unwrap().symbol, sentinel, "({}, {})", x, y);
                }
            }
        }
    }
}
