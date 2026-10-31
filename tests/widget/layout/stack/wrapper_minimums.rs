//! A filling child's minimum reaches a content-sized stack through every
//! wrapper, with the wrapper's own rows and columns around it (#1341).

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    hstack, vstack, Border, Card, CardVariant, DebugOverlay, ErrorBoundary, Text, ZenMode,
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
fn a_border_or_card_keeps_its_own_bounds_in_what_it_asks_for() {
    let border = |b: Border| View::min_size(&b.child(Least(1, 3)));
    assert_eq!(border(Border::single()), (3, 5));
    assert_eq!(border(Border::single().min_height(9)), (3, 9));
    assert_eq!(border(Border::single().min_width(9)), (9, 5));
    assert_eq!(border(Border::single().max_height(4)), (3, 4));
    assert_eq!(border(Border::single().max_width(2)), (2, 5));
    assert_eq!(View::min_size(&Border::single().min_height(4)), (0, 4));

    let card = |c: Card| View::min_size(&c.body(Least(1, 3)));
    assert_eq!(card(Card::new()), (5, 5));
    assert_eq!(card(Card::new().min_height(9)), (5, 9));
    assert_eq!(card(Card::new().min_width(9)), (9, 5));
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
