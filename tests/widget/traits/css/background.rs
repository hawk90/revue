//! A node's CSS `background` fills its box, not just the cells its widget
//! happens to write.
//!
//! Widgets paint glyphs. A `Border` writes its frame and nothing inside it; a
//! `Text` writes its characters and nothing after them. Before this, a
//! stylesheet's `background` reached only the cells a widget wrote *and* chose
//! to tint - `* { background: ... }` in `examples/dashboard.rs` came out as a
//! patchwork of frames and words over the terminal's own background.
//!
//! The fill goes *under* the widget: after it paints, every cell in its box
//! that still has no background takes the node's. So a background the widget
//! set itself - from a builder, or from a child's own rule - is kept.

use revue::prelude::*;
use revue::testing::PipelineHarness;
use revue::widget::ZenMode;

const BOX: Color = Color::rgb(0x10, 0x20, 0x30);
const INNER: Color = Color::rgb(0x40, 0x50, 0x60);

/// Places one child at a fixed area, so the test knows where its box is.
struct At<V: View>(V, Rect);

impl<V: View> View for At<V> {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.render_child(&self.0, self.1);
    }
}

fn harness(css: &str) -> PipelineHarness {
    PipelineHarness::with_css(css, 20, 8).dom_from_render(true)
}

fn bg(h: &PipelineHarness, x: u16, y: u16) -> Option<Color> {
    h.buffer().get(x, y).and_then(|c| c.bg)
}

fn framed(child: impl View + 'static) -> At<Border> {
    At(
        Border::single().element_id("box").child(child),
        Rect::new(2, 1, 10, 5),
    )
}

#[test]
fn background_fills_the_cells_the_widget_left_empty() {
    let mut h = harness("#box { background: #102030; }");
    h.draw(&framed(Text::new("hi")));

    // Inside the frame, past the text: nobody wrote these.
    assert_eq!(bg(&h, 8, 3), Some(BOX), "the box's interior");
    // The frame itself.
    assert_eq!(bg(&h, 2, 1), Some(BOX), "the box's frame");
}

#[test]
fn background_stays_inside_the_box() {
    let mut h = harness("#box { background: #102030; }");
    h.draw(&framed(Text::new("hi")));

    assert_eq!(bg(&h, 1, 1), None, "left of the box");
    assert_eq!(bg(&h, 12, 1), None, "right of the box");
    assert_eq!(bg(&h, 2, 6), None, "below the box");
}

#[test]
fn a_child_without_a_background_shows_its_parents() {
    let mut h = harness("#box { background: #102030; }");
    h.draw(&framed(Text::new("hi").element_id("t")));

    let t = h.painted_rect("t").expect("text was painted");
    assert_eq!(bg(&h, t.x, t.y), Some(BOX), "the text's own glyph");
}

#[test]
fn a_childs_own_background_wins_inside_the_child() {
    let mut h = harness("#box { background: #102030; } #t { background: #405060; }");
    h.draw(&framed(Text::new("hi").element_id("t")));

    let t = h.painted_rect("t").expect("text was painted");
    assert_eq!(bg(&h, t.x, t.y), Some(INNER), "inside the child");
    // The text is handed the whole interior, so the frame is the parent's alone.
    assert_eq!(bg(&h, 2, 1), Some(BOX), "the parent's frame");
}

#[test]
fn a_background_the_widget_painted_itself_is_kept() {
    let mut h = harness("#box { background: #102030; }");
    h.draw(&framed(Text::new("hi").element_id("t").bg(Color::RED)));

    let t = h.painted_rect("t").expect("text was painted");
    assert_eq!(bg(&h, t.x, t.y), Some(Color::RED));
}

#[test]
fn no_background_rule_leaves_the_terminal_background() {
    let mut h = harness("#box { color: red; }");
    h.draw(&framed(Text::new("hi")));

    assert_eq!(bg(&h, 8, 3), None);
}

// ---------------------------------------------------------------------------
// A widget that fills its own box
// ---------------------------------------------------------------------------

const ZEN: Color = Color::rgb(15, 15, 25);

fn zen_at() -> At<ZenMode> {
    let mut z = ZenMode::new(Text::new("focus").element_id("t"))
        .bg(ZEN)
        .padding(0)
        .element_id("zen");
    z.enable();
    At(z, Rect::new(0, 0, 12, 3))
}

/// The bug this replaced, with no CSS at all: `ZenMode` filled its box, then
/// its content wrote "focus" without a background - and `Buffer::set` replaces
/// the whole cell, so each letter showed the terminal through the fill.
#[test]
fn a_widgets_own_fill_shows_under_its_content() {
    for dom_from_render in [false, true] {
        let mut h = PipelineHarness::new(20, 8).dom_from_render(dom_from_render);
        h.draw(&zen_at());
        assert_eq!(
            bg(&h, 0, 0),
            Some(ZEN),
            "dom_from_render={dom_from_render}: under the 'f'"
        );
    }
}

/// The widget's fill is what the builder said; the stylesheet does not get to
/// show through the holes in it.
#[test]
fn a_widgets_own_fill_beats_the_stylesheet() {
    let mut h = harness("#zen { background: #ff0000; }");
    h.draw(&zen_at());
    assert_eq!(bg(&h, 0, 0), Some(ZEN), "under the 'f'");
}
