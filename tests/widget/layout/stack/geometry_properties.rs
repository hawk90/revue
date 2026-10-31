//! Layout invariants over arbitrary trees and areas, down to 0×0 and up to
//! `u16::MAX` (#1156).

use std::cell::RefCell;

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{hstack, vstack, Border, Card, Stack, Text};

thread_local! {
    /// The area each leaf was last handed.
    static LEAVES: RefCell<Vec<Rect>> = const { RefCell::new(Vec::new()) };
}

/// A leaf that fills its area with `#`, and records it.
struct Fills;

impl View for Fills {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        LEAVES.with(|l| l.borrow_mut().push(area));
        for y in 0..area.height.min(200) {
            for x in 0..area.width.min(200) {
                ctx.draw_char(x, y, '#', revue::style::Color::WHITE);
            }
        }
    }
}

/// How a child is added to its stack.
#[derive(Debug, Clone)]
enum Added {
    Auto,
    Sized(u16),
    Flex(f32),
}

/// A view tree as data, so proptest can show and shrink it.
#[derive(Debug, Clone)]
enum Tree {
    Fills,
    Text(u16),
    Border(Box<Tree>),
    Card(Box<Tree>),
    Stack {
        row: bool,
        gap: u16,
        children: Vec<(Added, Tree)>,
    },
}

impl Tree {
    fn build(&self) -> Box<dyn View> {
        match self {
            Tree::Fills => Box::new(Fills),
            Tree::Text(lines) => Box::new(Text::new(vec!["text"; usize::from(*lines)].join("\n"))),
            Tree::Border(child) => Box::new(Border::single().child(child.build())),
            Tree::Card(child) => Box::new(Card::new().title("T").body(child.build())),
            Tree::Stack { row, gap, children } => {
                let mut stack: Stack = if *row { hstack() } else { vstack() };
                stack = stack.gap(*gap);
                for (added, child) in children {
                    stack = match added {
                        Added::Auto => stack.child(child.build()),
                        Added::Sized(n) => stack.child_sized(child.build(), *n),
                        Added::Flex(g) => stack.child_flex(child.build(), *g),
                    };
                }
                Box::new(stack)
            }
        }
    }
}

fn size() -> impl Strategy<Value = u16> {
    prop_oneof![
        Just(0u16),
        Just(1),
        Just(2),
        3u16..12,
        Just(40),
        Just(u16::MAX)
    ]
}

fn tree() -> impl Strategy<Value = Tree> {
    let leaf = prop_oneof![Just(Tree::Fills), (0u16..5).prop_map(Tree::Text)];
    leaf.prop_recursive(4, 24, 4, |inner| {
        let added = prop_oneof![
            Just(Added::Auto),
            size().prop_map(Added::Sized),
            prop_oneof![Just(0.5f32), Just(1.0), Just(2.5)].prop_map(Added::Flex),
        ];
        prop_oneof![
            inner.clone().prop_map(|c| Tree::Border(Box::new(c))),
            inner.clone().prop_map(|c| Tree::Card(Box::new(c))),
            (
                any::<bool>(),
                prop_oneof![Just(0u16), Just(1), Just(u16::MAX)],
                prop::collection::vec((added, inner), 0..5)
            )
                .prop_map(|(row, gap, children)| Tree::Stack { row, gap, children }),
        ]
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Nothing panics, and nothing is painted outside the area.
    #[test]
    fn a_tree_paints_inside_its_area(
        tree in tree(),
        w in 0u16..40,
        h in 0u16..20,
    ) {
        const MARGIN: u16 = 3;
        let (bw, bh) = (w + 2 * MARGIN, h + 2 * MARGIN);
        let mut buffer = Buffer::new(bw, bh);
        let view = tree.build();
        let area = Rect::new(MARGIN, MARGIN, w, h);
        let mut ctx = RenderContext::new(&mut buffer, area);
        view.render(&mut ctx);
        for y in 0..bh {
            for x in 0..bw {
                if !area.contains(x, y) {
                    prop_assert_eq!(buffer.get(x, y).unwrap().symbol, ' ', "({}, {})", x, y);
                }
            }
        }
    }

    /// However much a stack overflows, the area a leaf is handed stays inside
    /// the root's.
    #[test]
    fn a_leaf_is_handed_an_area_inside_the_root(tree in tree(), w in 1u16..40, h in 1u16..20) {
        LEAVES.with(|l| l.borrow_mut().clear());
        let mut buffer = Buffer::new(w, h);
        let view = tree.build();
        let root = Rect::new(0, 0, w, h);
        let mut ctx = RenderContext::new(&mut buffer, root);
        view.render(&mut ctx);
        for leaf in LEAVES.with(|l| l.borrow().clone()) {
            if leaf.width > 0 && leaf.height > 0 {
                prop_assert!(leaf.right() <= w && leaf.bottom() <= h, "{:?} in {:?}", leaf, root);
            }
        }
    }

    /// Nothing panics at the far corner of the coordinate space, either.
    #[test]
    fn a_tree_survives_any_area(
        tree in tree(),
        x in size(),
        y in size(),
        w in size(),
        h in size(),
    ) {
        let mut buffer = Buffer::new(8, 8);
        let view = tree.build();
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(x, y, w, h));
        view.render(&mut ctx);
    }

    /// `measure` stays within any offer.
    #[test]
    fn a_tree_measures_within_any_offer(tree in tree(), w in size(), h in size()) {
        let view = tree.build();
        if let Some((mw, mh)) = view.measure(w, h) {
            prop_assert!(mw <= w && mh <= h, "measured {}x{} of {}x{}", mw, mh, w, h);
        }
    }
}
