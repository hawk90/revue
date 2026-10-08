//! `max_width` / `max_height` on the layout widgets that used to store them and
//! draw in the full area anyway (#799): Layers, ScreenStack, ScrollView,
//! Accordion, Sidebar and Card.
//!
//! Each widget is drawn into a 20x6 buffer, once as is - it must reach past
//! the limit, or the check proves nothing - and once with `max_width(8)` and
//! `max_height(3)`, after which nothing may be painted outside the top-left
//! 8x3.

use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::widget::{
    accordion, card, layers, screen, screen_stack, section, sidebar_item, sidebar_section,
    RenderContext, ScrollView, Sidebar, Text, View,
};

const W: u16 = 20;
const H: u16 = 6;
const MAX_W: u16 = 8;
const MAX_H: u16 = 3;

/// Fills whatever area it is given.
#[derive(Clone)]
struct Filler;

impl View for Filler {
    fn render(&self, ctx: &mut RenderContext) {
        for y in 0..ctx.area.height {
            for x in 0..ctx.area.width {
                ctx.set(x, y, Cell::new('#'));
            }
        }
    }
}

fn painted(draw: impl FnOnce(&mut RenderContext)) -> Vec<(u16, u16)> {
    let mut buffer = Buffer::new(W, H);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, W, H));
    draw(&mut ctx);
    let blank = Cell::default();
    let mut out = Vec::new();
    for y in 0..H {
        for x in 0..W {
            if buffer.get(x, y).copied().unwrap_or(blank) != blank {
                out.push((x, y));
            }
        }
    }
    out
}

fn outside(cells: &[(u16, u16)]) -> Vec<(u16, u16)> {
    cells
        .iter()
        .copied()
        .filter(|&(x, y)| x >= MAX_W || y >= MAX_H)
        .collect()
}

fn check(name: &str, free: Vec<(u16, u16)>, limited: Vec<(u16, u16)>) {
    assert!(
        !outside(&free).is_empty(),
        "{name}: never paints past {MAX_W}x{MAX_H} unconstrained, so the test proves nothing"
    );
    assert!(
        !limited.is_empty(),
        "{name}: the constrained widget drew nothing"
    );
    let escaped = outside(&limited);
    assert!(
        escaped.is_empty(),
        "{name}: painted outside max_width({MAX_W}) / max_height({MAX_H}) at {escaped:?}"
    );
}

#[test]
fn layers_honor_max_size() {
    check(
        "Layers",
        painted(|ctx| layers().child(Filler).render(ctx)),
        painted(|ctx| {
            layers()
                .child(Filler)
                .max_width(MAX_W)
                .max_height(MAX_H)
                .render(ctx)
        }),
    );
}

#[test]
fn a_screen_stack_honors_max_size() {
    let stack = |limited: bool| {
        let mut s = screen_stack().register("home", |_, ctx| Filler.render(ctx));
        if limited {
            s = s.max_width(MAX_W).max_height(MAX_H);
        }
        s.push(screen("home"));
        s
    };
    check(
        "ScreenStack",
        painted(|ctx| stack(false).render(ctx)),
        painted(|ctx| stack(true).render(ctx)),
    );
}

#[test]
fn a_scroll_view_honors_max_size_for_its_content() {
    let draw = |view: ScrollView, ctx: &mut RenderContext| {
        let mut content = view.create_content_buffer(W);
        for y in 0..H {
            for x in 0..W {
                content.set(x, y, Cell::new('#'));
            }
        }
        view.render_content(ctx, &content);
    };
    check(
        "ScrollView",
        painted(|ctx| draw(ScrollView::new().content_height(H), ctx)),
        painted(|ctx| {
            draw(
                ScrollView::new()
                    .content_height(H)
                    .max_width(MAX_W)
                    .max_height(MAX_H),
                ctx,
            )
        }),
    );
}

#[test]
fn an_accordion_honors_max_size() {
    let make = || {
        accordion()
            .section(section("A rather long title").content("and a long line of content"))
            .section(section("Second").expanded(true).content("more"))
    };
    check(
        "Accordion",
        painted(|ctx| make().render(ctx)),
        painted(|ctx| make().max_width(MAX_W).max_height(MAX_H).render(ctx)),
    );
}

#[test]
fn a_sidebar_honors_max_size() {
    let make = || {
        Sidebar::new().section(sidebar_section(vec![
            sidebar_item("a", "A long sidebar label"),
            sidebar_item("b", "Another item"),
            sidebar_item("c", "Third"),
            sidebar_item("d", "Fourth"),
        ]))
    };
    check(
        "Sidebar",
        painted(|ctx| make().render(ctx)),
        painted(|ctx| make().max_width(MAX_W).max_height(MAX_H).render(ctx)),
    );
}

#[test]
fn a_card_honors_max_size() {
    let make = || {
        card()
            .title("Card title")
            .body(Text::new("a long body line here"))
    };
    check(
        "Card",
        painted(|ctx| make().render(ctx)),
        painted(|ctx| make().max_width(MAX_W).max_height(MAX_H).render(ctx)),
    );
}

#[test]
fn a_card_measures_within_max_width() {
    let c = card().title("t").max_width(MAX_W).min_width(4);
    let (w, _) = c.measure(W, H).expect("a card has a size");
    assert_eq!(w, MAX_W, "measure ignored max_width");
    let narrow = card().title("t").min_width(12);
    assert_eq!(narrow.measure(W, H).unwrap().0, W);
}

/// A child that panics while drawn in a constrained area leaves `ctx.area`
/// constrained; `ErrorBoundary` must draw its fallback in its own area.
#[test]
fn an_error_boundary_fallback_gets_its_whole_area_after_a_constrained_panic() {
    #[derive(Clone)]
    struct Panics;
    impl View for Panics {
        fn render(&self, _ctx: &mut RenderContext) {
            panic!("boom inside a constrained layer");
        }
    }
    #[derive(Clone)]
    struct Wide;
    impl View for Wide {
        fn render(&self, ctx: &mut RenderContext) {
            Filler.render(ctx);
        }
    }
    let cells = painted(|ctx| {
        revue::widget::error_boundary()
            .child(
                screen_stack()
                    .register("x", |_, ctx| Panics.render(ctx))
                    .max_width(2)
                    .tap_push(),
            )
            .fallback(Wide)
            .render(ctx)
    });
    assert!(
        cells.iter().any(|&(x, _)| x >= 2),
        "the fallback was drawn in the panicked child's 2-column area"
    );
}

trait TapPush {
    fn tap_push(self) -> Self;
}

impl TapPush for revue::widget::ScreenStack {
    fn tap_push(mut self) -> Self {
        self.push(screen("x"));
        self
    }
}
