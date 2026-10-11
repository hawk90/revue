//! Paint-level checks for the developer tools.
//!
//! Every one of these paints a palette that *is* the information - a blue GET
//! against a red DELETE, ten stacked series, the colors an ANSI stream asked
//! for. A blanket `color` rule cannot name any of those separately, so it
//! reaches the chrome around them: the URL, the streamed text, the terminal's
//! own default, the chart's labels.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{AiStream, HttpClient, HttpMethod, StreamLayer, Streamline, Terminal};

/// A color none of these widgets paints on its own.
///
/// These palettes contain red, green, yellow, cyan and white, so a sentinel
/// picked from them would pass with the wiring reverted - the mistake #643
/// caught.
const INK: Color = Color {
    r: 17,
    g: 34,
    b: 51,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 60, 14).dom_from_render(true);
    h.draw(view);
    h
}

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

fn any_bg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.bg) == Some(want)))
}

macro_rules! wrap {
    ($name:ident, $build:expr) => {
        struct $name;
        impl View for $name {
            fn render(&self, ctx: &mut RenderContext) {
                vstack().child($build).render(ctx);
            }
            fn widget_type(&self) -> &'static str {
                stringify!($name)
            }
            fn id(&self) -> Option<&str> {
                Some("root")
            }
        }
    };
}

// `content()` starts the typing effect at zero revealed characters, so the
// stream has to be completed before anything is on screen.
wrap!(AiStreamView, {
    let mut stream = AiStream::new().content("streamed answer").element_id("w");
    stream.complete();
    stream
});

wrap!(
    HttpClientView,
    HttpClient::new()
        .method(HttpMethod::DELETE)
        .url("https://example.test/things/1")
        .element_id("w")
);

wrap!(TerminalView, {
    let mut term = Terminal::new(50, 8).element_id("w");
    term.writeln("boot");
    term
});

wrap!(AnsiTerminalView, {
    let mut term = Terminal::new(50, 8).element_id("w");
    term.writeln("\x1b[32mgreen\x1b[0m");
    term
});

wrap!(
    StreamlineView,
    Streamline::new()
        .title("traffic")
        .layer(StreamLayer::new("a").data(vec![1.0, 3.0, 2.0]))
        .layer(StreamLayer::new("b").data(vec![2.0, 1.0, 4.0]))
        .element_id("w")
);

#[test]
fn color_reaches_the_streamed_text() {
    let h = draw("#w { color: #112233; }", &AiStreamView);
    assert!(any_fg(&h, INK), "`color` did not reach AiStream's text");
}

#[test]
fn color_reaches_an_http_clients_url() {
    let h = draw("#w { color: #112233; }", &HttpClientView);
    assert!(any_fg(&h, INK), "`color` did not reach the URL");
}

/// The method badge is colored by which method it is. That reading survives a
/// `color` rule, because the rule has no way to restore it.
#[test]
fn an_http_method_keeps_its_own_color() {
    let h = draw("#w { color: #112233; }", &HttpClientView);
    assert!(
        any_fg(&h, HttpMethod::DELETE.color()),
        "`color` flattened the method badge it should not address"
    );
}

#[test]
fn color_reaches_a_terminals_default() {
    let h = draw("#w { color: #112233; }", &TerminalView);
    assert!(
        any_fg(&h, INK),
        "`color` did not reach the terminal's default"
    );
}

/// A cell the content colored through ANSI keeps that color - it is the
/// program's output, the same reading a syntax highlighter's tokens get.
#[test]
fn an_ansi_colored_cell_keeps_its_color() {
    let h = draw("#w { color: #112233; }", &AnsiTerminalView);
    assert!(
        any_fg(&h, Color::GREEN),
        "`color` overwrote a cell the content had colored"
    );
}

#[test]
fn background_reaches_a_terminals_fill() {
    let h = draw("#w { background: #112233; }", &TerminalView);
    assert!(
        any_bg(&h, INK),
        "`background` did not reach the terminal's fill"
    );
}

#[test]
fn color_reaches_a_streamlines_chrome() {
    let h = draw("#w { color: #112233; }", &StreamlineView);
    assert!(any_fg(&h, INK), "`color` did not reach Streamline's labels");
}
