//! A filling child's minimum reaches a content-sized stack through every
//! wrapper, with the wrapper's own rows and columns around it (#1341).

use std::cell::RefCell;
use std::rc::Rc;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    hstack, vstack, Accordion, Border, Card, CardVariant, Collapsible, DebugOverlay, ErrorBoundary,
    Grid, Layers, List, Positioned, ScreenStack, ScrollView, Sidebar, Tabs, Text, ZenMode,
};

/// Fills what it is given with `#` and asks for `(w, h)`.
#[derive(Clone, Copy)]
struct Least(u16, u16);

impl View for Least {
    fn render(&self, ctx: &mut RenderContext) {
        for y in 0..ctx.area.height {
            for x in 0..ctx.area.width {
                ctx.draw_char(x, y, '#', revue::style::Color::WHITE);
            }
        }
    }

    fn min_size(&self) -> (u16, u16) {
        (self.0, self.1)
    }
}

/// Panics when drawn.
struct Panics;

impl View for Panics {
    fn render(&self, _ctx: &mut RenderContext) {
        panic!("drawn");
    }
}

fn rows(view: &dyn View, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    view.render(&mut ctx);
    (0..h)
        .map(|y| (0..w).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect()
}

/// The size of the block of `#`, which is the area the child was given.
fn given(rows: &[String]) -> (usize, usize) {
    let height = rows.iter().filter(|r| r.contains('#')).count();
    let width = rows
        .iter()
        .map(|r| r.chars().filter(|c| *c == '#').count())
        .max()
        .unwrap_or(0);
    (width, height)
}

/// `wrap` around a child asking for 3 rows, between a tall text and a footer,
/// in exactly the rows the footer, one row of the text and the wrapper's
/// minimum take. The child gets its 3 rows and the footer stays on the last.
fn assert_gets_its_rows(name: &str, wrap: impl Fn(Least) -> Box<dyn View>) {
    let wrapper = wrap(Least(0, 3));
    let height = View::min_size(&wrapper).1 + 2;
    let stack = vstack()
        .child(Text::new("1\n2\n3\n4\n5\n6\n7\n8\n9"))
        .child(wrapper)
        .child(Text::new("foot"));
    let drawn = rows(&stack, 16, height);
    assert_eq!(given(&drawn).1, 3, "{name}: {drawn:#?}");
    assert!(drawn[0].starts_with('1'), "{name}: {drawn:#?}");
    assert!(
        drawn[height as usize - 1].starts_with("foot"),
        "{name}: {drawn:#?}"
    );
}

/// The same along a row: the child gets its 3 columns.
fn assert_gets_its_columns(name: &str, wrap: impl Fn(Least) -> Box<dyn View>) {
    let wrapper = wrap(Least(3, 0));
    let width = View::min_size(&wrapper).0 + 2;
    let stack = hstack()
        .child(Text::new("123456789"))
        .child(wrapper)
        .child(Text::new("z"));
    let drawn = rows(&stack, width, 9);
    assert_eq!(given(&drawn).0, 3, "{name}: {drawn:#?}");
    assert_eq!(drawn[0].chars().next(), Some('1'), "{name}: {drawn:#?}");
    assert_eq!(
        drawn[0].chars().last(),
        Some('z'),
        "{name}: the last column belongs to the sibling: {drawn:#?}"
    );
}

type Wrap = fn(Least) -> Box<dyn View>;

/// Every wrapper, each with its own rows and columns around the child.
fn wrappers() -> Vec<(&'static str, Wrap, (u16, u16))> {
    vec![
        ("border", |c| Box::new(Border::single().child(c)), (2, 2)),
        (
            "border in a border",
            |c| Box::new(Border::single().child(Border::single().child(c))),
            (4, 4),
        ),
        ("card", |c| Box::new(Card::new().body(c)), (4, 2)),
        (
            "card with a title and a separator",
            |c| Box::new(Card::new().title("T").body(c)),
            (4, 4),
        ),
        (
            "card with a subtitle and a header",
            |c| Box::new(Card::new().subtitle("S").header(Text::new("H")).body(c)),
            (4, 5),
        ),
        (
            "card with a footer",
            |c| Box::new(Card::new().footer(Text::new("F")).body(c)),
            (4, 4),
        ),
        (
            "elevated card",
            |c| Box::new(Card::new().variant(CardVariant::Elevated).body(c)),
            (5, 3),
        ),
        (
            "card without padding",
            |c| Box::new(Card::new().padding(0).body(c)),
            (2, 2),
        ),
        (
            "borderless card",
            |c| {
                Box::new(
                    Card::new()
                        .border_style(revue::widget::BorderType::None)
                        .body(c),
                )
            },
            (2, 0),
        ),
        (
            "error boundary",
            |c| Box::new(ErrorBoundary::new().child(c)),
            (0, 0),
        ),
        ("zen off", |c| Box::new(ZenMode::new(c)), (0, 0)),
        (
            "zen on",
            |c| {
                let mut zen = ZenMode::new(c).padding_x(2).padding_y(1);
                zen.enable();
                Box::new(zen)
            },
            (4, 2),
        ),
        (
            "hidden debug overlay",
            |c| Box::new(DebugOverlay::wrap(c).visible(false)),
            (0, 0),
        ),
        ("nested stack", |c| Box::new(vstack().child(c)), (0, 0)),
    ]
}

#[test]
fn each_wrapper_adds_its_own_rows_and_columns_to_what_its_child_asks_for() {
    for (name, wrap, (w, h)) in wrappers() {
        assert_eq!(View::min_size(&wrap(Least(3, 5))), (3 + w, 5 + h), "{name}");
        assert_eq!(View::min_size(&wrap(Least(0, 0))), (0, 0), "{name}");
        assert_eq!(View::min_size(&wrap(Least(3, 0))), (3 + w, 0), "{name}");
        assert_eq!(View::min_size(&wrap(Least(0, 5))), (0, 5 + h), "{name}");
    }
}

#[test]
fn a_child_gets_its_rows_through_each_wrapper() {
    for (name, wrap, _) in wrappers() {
        assert_gets_its_rows(name, wrap);
    }
}

#[test]
fn a_child_gets_its_columns_through_each_wrapper() {
    for (name, wrap, _) in wrappers() {
        // A stack passes what its children ask for along its own axis only.
        if name != "nested stack" {
            assert_gets_its_columns(name, wrap);
        }
    }
}

#[test]
fn the_issue_example_draws_the_zone_inside_the_border() {
    let stack = vstack()
        .child(Text::new("t1\nt2"))
        .child(Border::single().child(Least(0, 3)))
        .child(Text::new("foot"));
    assert_eq!(
        rows(&stack, 8, 7),
        [
            "t1      ",
            "┌──────┐",
            "│######│",
            "│######│",
            "│######│",
            "└──────┘",
            "foot    ",
        ]
    );
}

#[test]
fn with_too_little_room_a_wrapper_gets_what_is_left_and_the_siblings_stay() {
    let stack = vstack()
        .child(Text::new("t1\nt2"))
        .child(Border::single().child(Least(0, 3)))
        .child(Text::new("foot"));
    assert_eq!(
        rows(&stack, 8, 5),
        ["t1      ", "┌──────┐", "│######│", "└──────┘", "foot    "]
    );
}

#[test]
fn a_wrapper_asking_for_nothing_does_not_shrink_its_sibling_for_its_frame() {
    let stack = vstack()
        .child(Text::new("t1\nt2"))
        .child(Border::single().child(Least(0, 0)));
    assert_eq!(rows(&stack, 4, 2), ["t1  ", "t2  "]);
}

#[test]
fn a_border_or_card_caps_what_its_child_asks_for_at_its_maximum() {
    let border = |b: Border| View::min_size(&b.child(Least(1, 3)));
    assert_eq!(border(Border::single()), (3, 5));
    assert_eq!(border(Border::single().min_height(9)), (3, 5));
    assert_eq!(border(Border::single().max_height(4)), (3, 4));
    assert_eq!(border(Border::single().max_width(2)), (2, 5));

    let card = |c: Card| View::min_size(&c.body(Least(1, 3)));
    assert_eq!(card(Card::new()), (5, 5));
    assert_eq!(card(Card::new().min_height(9)), (5, 5));
    assert_eq!(card(Card::new().max_height(4)), (5, 4));
    assert_eq!(card(Card::new().max_width(2)), (2, 5));
}

#[test]
fn a_collapsed_card_asks_nothing_for_the_body_it_does_not_show() {
    let collapsed = Card::new()
        .title("T")
        .footer(Text::new("F"))
        .collapsible(true)
        .expanded(false)
        .body(Least(3, 3));
    assert_eq!(View::min_size(&collapsed), (0, 0));
}

#[test]
fn an_error_boundary_asks_for_what_it_is_showing() {
    let boundary = ErrorBoundary::new().child(Panics).fallback(Least(0, 4));
    assert_eq!(View::min_size(&boundary), (0, 0));
    rows(&boundary, 4, 4);
    assert!(boundary.has_error());
    assert_eq!(View::min_size(&boundary), (0, 4));

    let boundary = ErrorBoundary::new().child(Least(0, 2)).fallback(Panics);
    assert_eq!(View::min_size(&boundary), (0, 2));
    assert_eq!(View::min_size(&ErrorBoundary::new()), (0, 0));
}

#[test]
fn a_shown_debug_overlay_still_asks_for_what_its_view_asks_for() {
    assert_eq!(
        View::min_size(&DebugOverlay::wrap(Least(2, 3)).visible(true)),
        (2, 3)
    );
}

#[test]
fn a_nested_stack_asks_for_its_gaps_too() {
    let zones = || vstack().gap(1).child(Least(0, 3)).child(Least(0, 3));
    assert_eq!(View::min_size(&zones()), (0, 7));
    assert_eq!(View::min_size(&vstack().gap(1).child(Least(0, 3))), (0, 3));
    assert_eq!(
        View::min_size(&vstack().gap(1).child(Least(0, 0)).child(Least(0, 0))),
        (0, 0),
        "no minimum, so no reason to keep room for the gaps"
    );
    assert_eq!(
        View::min_size(&hstack().gap(2).child(Least(2, 0)).child(Text::new("x"))),
        (4, 0),
        "a gap beside a measured sibling counts"
    );

    let stack = vstack().child(Text::new("1\n2\n3\n4")).child(zones());
    assert_eq!(
        rows(&stack, 2, 8),
        ["1 ", "##", "##", "##", "  ", "##", "##", "##"]
    );
}

#[test]
fn a_view_that_is_not_rendered_opens_no_gap() {
    struct Absent;
    impl View for Absent {
        fn render(&self, _ctx: &mut RenderContext) {}
        fn needs_render(&self) -> bool {
            false
        }
    }
    let stack = vstack().gap(1).child(Least(0, 3)).child(Absent);
    assert_eq!(View::min_size(&stack), (0, 3));
}

#[test]
fn the_minimum_holds_when_the_runtime_styles_the_tree() {
    use revue::testing::PipelineHarness;
    use revue::widget::DropZone;

    let stack = vstack()
        .child(Text::new("t1\nt2"))
        .child(Border::single().child(DropZone::new("Drop").min_height(3)))
        .child(Text::new("foot"));
    let mut harness = PipelineHarness::with_css("", 8, 7).dom_from_render(true);
    harness.draw(&stack);
    let symbol = |x, y| harness.buffer().get(x, y).unwrap().symbol;
    let drawn: Vec<String> = (0..7)
        .map(|y| (0..8).map(|x| symbol(x, y)).collect())
        .collect();
    assert_eq!(
        drawn,
        [
            "t1      ",
            "┌──────┐",
            "│┌────┐│",
            "││ Dr ││",
            "│└────┘│",
            "└──────┘",
            "foot    ",
        ]
    );
}

// ---------------------------------------------------------------------------
// A widget's own minimum, which its `render` puts back (#1347)
// ---------------------------------------------------------------------------

/// Records where it was drawn.
#[derive(Clone)]
struct Probe(Rc<RefCell<Option<Rect>>>);

impl View for Probe {
    fn render(&self, ctx: &mut RenderContext) {
        *self.0.borrow_mut() = Some(ctx.area);
    }
}

type Own = (&'static str, Box<dyn View>);

/// Every widget with `min_height`, set to 4.
fn own_minimums() -> Vec<Own> {
    fn entry(name: &'static str, view: impl View + 'static) -> Own {
        (name, Box::new(view))
    }
    vec![
        entry(
            "Border, filling",
            Border::single().min_height(4).child(List::new(vec!["a"])),
        ),
        entry(
            "Border, measured",
            Border::single().min_height(4).child(Text::new("x")),
        ),
        entry(
            "Card, filling",
            Card::new().min_height(4).body(List::new(vec!["a"])),
        ),
        entry(
            "Card, measured",
            Card::new().min_height(4).body(Text::new("x")),
        ),
        entry(
            "Stack, filling",
            vstack().min_height(4).child(List::new(vec!["a"])),
        ),
        entry(
            "Stack, measured",
            vstack().min_height(4).child(Text::new("x")),
        ),
        entry("Grid", Grid::new().min_height(4)),
        entry("ScrollView", ScrollView::new().min_height(4)),
        entry("Tabs", Tabs::new().min_height(4)),
        entry("Accordion", Accordion::new().min_height(4)),
        entry("Collapsible", Collapsible::new("t").min_height(4)),
        entry("Sidebar", Sidebar::new().min_height(4)),
        entry("ScreenStack", ScreenStack::new().min_height(4)),
        entry("Layers", Layers::new().min_height(4)),
        entry("Positioned", Positioned::new(Text::new("x")).min_height(4)),
    ]
}

#[test]
fn the_next_sibling_starts_past_a_widgets_own_minimum() {
    for height in [1, 3, 4, 5, 6] {
        for (name, view) in own_minimums() {
            let probe = Probe(Rc::new(RefCell::new(None)));
            let stack = vstack()
                .child(view)
                .child(Text::new("1\n2\n3"))
                .child(probe.clone());
            rows(&stack, 16, height);
            let drawn = *probe.0.borrow();
            if let Some(rect) = drawn {
                assert!(rect.y >= 4, "{name} in {height}: {rect:?}");
            }
        }
    }
}

#[test]
fn a_measured_sibling_gives_way_before_a_widgets_own_minimum() {
    for (name, view) in own_minimums() {
        let probe = Probe(Rc::new(RefCell::new(None)));
        let stack = vstack()
            .child(Text::new("1\n2\n3"))
            .child(view)
            .child(probe.clone());
        rows(&stack, 16, 6);
        let drawn = *probe.0.borrow();
        let rect = drawn.expect(name);
        assert!(rect.y >= 5, "{name}: {rect:?}");
    }
}

#[test]
fn a_widgets_own_minimum_is_held_through_a_wrapper() {
    let inner = || vstack().min_height(4);
    assert_eq!(View::held_size(&Border::single().child(inner())), (0, 6));
    assert_eq!(
        View::held_size(&Card::new().title("T").body(inner())),
        (0, 8)
    );
    assert_eq!(
        View::held_size(&ErrorBoundary::new().child(inner())),
        (0, 4)
    );
    assert_eq!(View::held_size(&DebugOverlay::wrap(inner())), (0, 4));
    assert_eq!(View::held_size(&ZenMode::new(inner())), (0, 4));
    assert_eq!(
        View::held_size(&vstack().gap(1).child(inner()).child(inner())),
        (0, 9)
    );
    assert_eq!(View::held_size(&vstack().child(Text::new("x"))), (0, 0));
}
