//! `flex-wrap: wrap` on a row inside a content-sized stack.
//!
//! The wrapping row used to split its area into two fixed halves. Inside a
//! content-sized column its area is its measured height - one line - so each
//! half was zero rows and the row drew nothing. A wrapped line is now as tall
//! as its tallest item, and the row measures as tall as its lines.

use revue::prelude::*;
use revue::testing::PipelineHarness;

/// `build()` as the whole screen.
struct Screen(fn() -> Stack);

impl View for Screen {
    fn render(&self, ctx: &mut RenderContext) {
        (self.0)().render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Screen"
    }
    fn id(&self) -> Option<&str> {
        Some("app")
    }
}

fn draw_at(css: &str, width: u16, height: u16, build: fn() -> Stack) -> String {
    let mut h = PipelineHarness::with_css(css, width, height)
        .dom_from_render(true)
        .css_layout(true);
    h.draw(&Screen(build));
    h.screen_text()
}

fn draw(css: &str, build: fn() -> Stack) -> String {
    draw_at(css, 20, 12, build)
}

fn tags() -> Stack {
    vstack()
        .child(
            hstack()
                .class("tags")
                .child(Text::new("aaa"))
                .child(Text::new("bbb"))
                .child(Text::new("ccc")),
        )
        .child(Text::new("after"))
}

#[test]
fn a_wrapping_row_that_fits_is_one_line() {
    assert_eq!(draw(".tags { flex-wrap: wrap; }", tags), "aaabbbccc\nafter");
}

#[test]
fn a_wrapping_row_takes_as_many_lines_as_it_needs() {
    assert_eq!(
        draw_at(".tags { flex-wrap: wrap; }", 7, 6, tags),
        "aaabbb\nccc\nafter"
    );
    assert_eq!(
        draw_at(".tags { flex-wrap: wrap; }", 5, 6, tags),
        "aaa\nbbb\nccc\nafter"
    );
}

#[test]
fn a_wrapping_rows_gap_separates_items_and_lines() {
    assert_eq!(
        draw_at(".tags { flex-wrap: wrap; gap: 1; }", 8, 6, tags),
        "aaa bbb\n\nccc\nafter"
    );
}

/// A line is as tall as its tallest item.
#[test]
fn a_wrapped_line_is_as_tall_as_its_tallest_item() {
    fn mixed() -> Stack {
        vstack()
            .child(
                hstack()
                    .class("tags")
                    .child(Text::new("aa"))
                    .child(Border::single().child(Text::new("b")))
                    .child(Text::new("cc")),
            )
            .child(Text::new("after"))
    }
    assert_eq!(
        draw_at(".tags { flex-wrap: wrap; }", 6, 6, mixed),
        "aa┌─┐\n  │b│\n  └─┘\ncc\nafter"
    );
}

/// The items' own margins are folded in, as in a row that does not wrap.
#[test]
fn a_wrapping_rows_items_keep_their_margins() {
    assert_eq!(
        draw_at(
            ".tags { flex-wrap: wrap; } .tags > * { margin-right: 1; }",
            9,
            6,
            tags
        ),
        "aaa bbb\nccc\nafter"
    );
}

/// An item's own inner spacing counts toward its line's height.
#[test]
fn a_wrapped_items_inner_spacing_counts() {
    fn nested() -> Stack {
        vstack()
            .child(
                hstack()
                    .class("tags")
                    .child(
                        vstack()
                            .class("pair")
                            .child(Text::new("a"))
                            .child(Text::new("b")),
                    )
                    .child(Text::new("c")),
            )
            .child(Text::new("after"))
    }
    assert_eq!(
        draw(".tags { flex-wrap: wrap; } .pair { gap: 1; }", nested),
        "ac\n\nb\nafter"
    );
}

/// The row gets the whole screen rather than a measured slot: its lines
/// still start at the top, one after another.
#[test]
fn a_wrapping_row_given_the_whole_screen_stacks_its_lines_from_the_top() {
    fn row() -> Stack {
        vstack().child_flex(
            hstack()
                .class("tags")
                .child(Text::new("aaa"))
                .child(Text::new("bbb"))
                .child(Text::new("ccc")),
            1.0,
        )
    }
    assert_eq!(
        draw_at(".tags { flex-wrap: wrap; }", 7, 6, row),
        "aaabbb\nccc"
    );
}
