//! The incremental DOM against a fresh build (#1157).
//!
//! A tree and a list of changes are generated: insert, remove, reorder, change a
//! class or a text. After each change, the DOM that followed along must equal
//! the DOM of a new app that only ever saw the end result: the same nodes in the
//! same order, the same styles, and the same screen.
//!
//! Focus and hover follow a node's identity, which only keys give it. So they are
//! compared for keyed trees only, where the node a change keeps is the same node.

use proptest::prelude::*;
use revue::prelude::*;
use revue::testing::PipelineHarness;

const CSS: &str = "
    .a { color: rgb(255, 0, 0); }
    .b { color: rgb(0, 255, 0); }
    Text:first-child { background: rgb(1, 1, 1); }
    .a + .b { color: rgb(0, 0, 255); }
    .b ~ .a { background: rgb(2, 2, 2); }
    Text:focus { color: rgb(9, 9, 9); }
    Text:hover { color: rgb(8, 8, 8); }
";

#[derive(Clone, Debug)]
enum Node {
    Leaf { id: u32, class: u8, text: u8 },
    Group { id: u32, children: Vec<Node> },
}

impl Node {
    fn id(&self) -> u32 {
        match self {
            Node::Leaf { id, .. } | Node::Group { id, .. } => *id,
        }
    }
}

#[derive(Clone, Debug)]
struct Page {
    nodes: Vec<Node>,
    keyed: bool,
}

fn build(node: &Node, keyed: bool) -> Box<dyn View> {
    match node {
        Node::Leaf { id, class, text } => {
            let mut leaf = Text::new(format!("t{text}"))
                .element_id(format!("n{id}"))
                .class(if *class == 0 { "a" } else { "b" });
            if keyed {
                leaf = leaf.keyed(format!("k{id}"));
            }
            Box::new(leaf)
        }
        Node::Group { id, children } => {
            let mut group = vstack().element_id(format!("n{id}"));
            if keyed {
                group = group.keyed(format!("k{id}"));
            }
            for child in children {
                group = group.child(build(child, keyed));
            }
            Box::new(group)
        }
    }
}

impl View for Page {
    fn render(&self, ctx: &mut RenderContext) {
        let mut root = vstack();
        for node in &self.nodes {
            root = root.child(build(node, self.keyed));
        }
        root.render(ctx);
    }
}

/// A change to the tree. The numbers pick a place, modulo what is there.
#[derive(Clone, Debug)]
enum Change {
    Insert { at: usize, group: bool },
    Remove { at: usize },
    Move { from: usize, to: usize },
    Class { at: usize },
    Text { at: usize },
}

/// How many node lists the tree has: the top one and one per group.
fn list_count(nodes: &[Node]) -> usize {
    1 + nodes
        .iter()
        .map(|node| match node {
            Node::Group { children, .. } => list_count(children),
            Node::Leaf { .. } => 0,
        })
        .sum::<usize>()
}

/// Node list number `n`, the top one first.
fn list_at(nodes: &mut Vec<Node>, n: usize) -> &mut Vec<Node> {
    if n == 0 {
        return nodes;
    }
    let mut left = n - 1;
    let mut found = None;
    for (i, node) in nodes.iter().enumerate() {
        if let Node::Group { children, .. } = node {
            let lists = list_count(children);
            if left < lists {
                found = Some((i, left));
                break;
            }
            left -= lists;
        }
    }
    let (i, left) = found.expect("n is under list_count");
    match &mut nodes[i] {
        Node::Group { children, .. } => list_at(children, left),
        Node::Leaf { .. } => unreachable!("found at a group"),
    }
}

fn apply(page: &mut Page, change: &Change, next_id: &mut u32) {
    let count = list_count(&page.nodes);
    match *change {
        Change::Insert { at, group } => {
            let list = list_at(&mut page.nodes, at % count);
            *next_id += 1;
            let node = if group {
                Node::Group {
                    id: *next_id,
                    children: Vec::new(),
                }
            } else {
                Node::Leaf {
                    id: *next_id,
                    class: 0,
                    text: 0,
                }
            };
            let place = at % (list.len() + 1);
            list.insert(place, node);
        }
        Change::Remove { at } => {
            let list = list_at(&mut page.nodes, at % count);
            if !list.is_empty() {
                let place = at % list.len();
                list.remove(place);
            }
        }
        Change::Move { from, to } => {
            let list = list_at(&mut page.nodes, from % count);
            if !list.is_empty() {
                let node = list.remove(from % list.len());
                let place = to % (list.len() + 1);
                list.insert(place, node);
            }
        }
        Change::Class { at } | Change::Text { at } => {
            let list = list_at(&mut page.nodes, at % count);
            let found = list.iter_mut().find_map(|node| match node {
                Node::Leaf { class, text, .. } => Some((class, text)),
                Node::Group { .. } => None,
            });
            if let Some((class, text)) = found {
                match change {
                    Change::Class { .. } => *class ^= 1,
                    _ => *text = text.wrapping_add(1),
                }
            }
        }
    }
}

fn has_id(nodes: &[Node], id: u32) -> bool {
    nodes.iter().any(|node| {
        node.id() == id || matches!(node, Node::Group { children, .. } if has_id(children, id))
    })
}

fn harness() -> PipelineHarness {
    PipelineHarness::with_css(CSS, 30, 30)
        .dom_from_render(true)
        .incremental_dom(true)
}

fn change() -> impl Strategy<Value = Change> {
    let at = 0usize..64;
    prop_oneof![
        (at.clone(), any::<bool>()).prop_map(|(at, group)| Change::Insert { at, group }),
        at.clone().prop_map(|at| Change::Remove { at }),
        (at.clone(), at.clone()).prop_map(|(from, to)| Change::Move { from, to }),
        at.clone().prop_map(|at| Change::Class { at }),
        at.prop_map(|at| Change::Text { at }),
    ]
}

/// The first id free after the starting tree.
const FIRST_NEW_ID: u32 = 3;

fn start(keyed: bool) -> Page {
    Page {
        keyed,
        nodes: vec![
            Node::Leaf {
                id: 1,
                class: 0,
                text: 0,
            },
            Node::Group {
                id: 2,
                children: vec![Node::Leaf {
                    id: 3,
                    class: 1,
                    text: 1,
                }],
            },
        ],
    }
}

/// Draws every change, comparing against a fresh app each time. `watch` picks an
/// element id to hold focus and another to hold hover, keyed trees only.
fn run(keyed: bool, changes: &[Change], watch: bool) -> std::result::Result<(), TestCaseError> {
    let mut page = start(keyed);
    let mut next_id = FIRST_NEW_ID;
    let mut live = harness();
    live.draw(&page);
    // The first node: whether it survives is up to the changes.
    let focused = 1;
    if watch {
        live.focus(Some("n1"));
        live.hover(Some("n3"));
        live.draw(&page);
    }

    for change in changes {
        apply(&mut page, change, &mut next_id);
        live.draw(&page);

        let mut fresh = harness();
        fresh.draw(&page);
        if watch {
            if has_id(&page.nodes, focused) {
                fresh.focus(Some("n1"));
            }
            // Hover outlives a removed node on its ancestors until the pointer
            // next moves, which is what a browser does too. Compare until then.
            if has_id(&page.nodes, 3) {
                fresh.hover(Some("n3"));
            } else {
                live.hover(None);
                live.draw(&page);
            }
            fresh.draw(&page);
        }

        prop_assert_eq!(
            live.dom_outline(),
            fresh.dom_outline(),
            "after {:?}",
            change
        );
        prop_assert_eq!(
            live.screen_text(),
            fresh.screen_text(),
            "after {:?}",
            change
        );
    }
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 128,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn a_keyed_tree_follows_any_change(changes in prop::collection::vec(change(), 1..24)) {
        run(true, &changes, false)?;
    }

    #[test]
    fn a_keyless_tree_follows_any_change(changes in prop::collection::vec(change(), 1..24)) {
        run(false, &changes, false)?;
    }

    #[test]
    fn focus_and_hover_stay_on_their_keyed_nodes(changes in prop::collection::vec(change(), 1..24)) {
        run(true, &changes, true)?;
    }
}

const SIBLING_CSS: &str = ".a { color: rgb(255, 0, 0); } .b { color: rgb(0, 255, 0); } \
    .a + .b { color: rgb(0, 0, 255); } .b ~ .a { background: rgb(2, 2, 2); }";

/// Draws `before` then `after`, and compares with a new app that only saw `after`.
fn assert_follows(before: Vec<Node>, after: Vec<Node>) {
    let new_harness = || {
        PipelineHarness::with_css(SIBLING_CSS, 30, 30)
            .dom_from_render(true)
            .incremental_dom(true)
    };
    let page = |nodes| Page { nodes, keyed: true };
    let mut live = new_harness();
    live.draw(&page(before));
    let after = page(after);
    live.draw(&after);

    let mut fresh = new_harness();
    fresh.draw(&after);
    assert_eq!(live.dom_outline(), fresh.dom_outline());
}

fn leaf(id: u32, class: u8) -> Node {
    Node::Leaf { id, class, text: 0 }
}

fn empty_group(id: u32) -> Node {
    Node::Group {
        id,
        children: Vec::new(),
    }
}

/// The last child keeps its index, so only its neighbors moved (#1408).
#[test]
fn a_sibling_rule_follows_a_reorder_of_the_siblings_before_it() {
    assert_follows(
        vec![empty_group(9), leaf(1, 0), leaf(2, 1)],
        vec![leaf(1, 0), empty_group(9), leaf(2, 1)],
    );
}

#[test]
fn a_sibling_rule_starts_to_match_when_a_reorder_makes_it() {
    assert_follows(
        vec![leaf(1, 0), empty_group(9), leaf(2, 1)],
        vec![empty_group(9), leaf(1, 0), leaf(2, 1)],
    );
}

#[test]
fn a_general_sibling_rule_follows_a_reorder() {
    assert_follows(
        vec![leaf(1, 0), leaf(2, 1), leaf(3, 0)],
        vec![leaf(2, 1), leaf(1, 0), leaf(3, 0)],
    );
    assert_follows(
        vec![leaf(2, 1), leaf(1, 0), leaf(3, 0)],
        vec![leaf(1, 0), leaf(3, 0), leaf(2, 1)],
    );
}

#[test]
fn a_sibling_rule_follows_a_removed_neighbor() {
    assert_follows(vec![leaf(1, 0), leaf(2, 1)], vec![leaf(2, 1)]);
}

#[test]
fn a_sibling_rule_follows_an_inserted_neighbor() {
    assert_follows(vec![leaf(2, 1)], vec![leaf(1, 0), leaf(2, 1)]);
    assert_follows(
        vec![leaf(1, 0), leaf(2, 1)],
        vec![leaf(1, 0), empty_group(9), leaf(2, 1)],
    );
}
