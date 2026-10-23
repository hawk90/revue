//! A terminal emulator (`vt100`) reading the bytes revue writes, and the
//! cell-by-cell comparison of its screen with a buffer (#1164)
//!
//! Checked per cell: the character, a wide glyph and its continuation, the
//! foreground and background (a color the cell does not set is the
//! terminal's default), bold, dim, italic, underline and reverse. The
//! emulator does not track strikethrough, and keeps bold and dim as one
//! intensity (a cell with both shows one).

use revue::render::{Buffer, Modifier};
use revue::style::Color;

/// Feed `bytes` to an emulator of `w` x `h`
pub fn emulate(bytes: &[u8], w: u16, h: u16) -> vt100::Parser {
    let mut parser = vt100::Parser::new(h, w, 0);
    parser.process(bytes);
    parser
}

fn color(c: Option<Color>) -> vt100::Color {
    match c {
        Some(c) => vt100::Color::Rgb(c.r, c.g, c.b),
        None => vt100::Color::Default,
    }
}

/// Where the screen differs from the buffer, if anywhere
pub fn difference(buffer: &Buffer, screen: &vt100::Screen) -> Option<String> {
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let want = buffer.get(x, y).unwrap();
            let Some(got) = screen.cell(y, x) else {
                return Some(format!("({x}, {y}): no cell on the screen"));
            };
            let at = |what: &str| format!("({x}, {y}) {what}: {want:?} vs {got:?}");
            if want.is_continuation() {
                if !got.is_wide_continuation() {
                    return Some(at("not a wide continuation"));
                }
                continue;
            }
            // A cell never written reads as empty
            let shown = match got.contents() {
                "" => " ",
                s => s,
            };
            if shown != want.symbol.to_string() {
                return Some(at("character"));
            }
            let wide = revue::utils::unicode::char_width(want.symbol) == 2;
            if got.is_wide() != wide {
                return Some(at("width"));
            }
            if got.fgcolor() != color(want.fg) || got.bgcolor() != color(want.bg) {
                return Some(at("color"));
            }
            let m = want.modifier;
            // The emulator keeps bold and dim as one intensity, the later
            // code winning: with both set, one of them is shown
            if m.contains(Modifier::BOLD | Modifier::DIM) {
                if !(got.bold() || got.dim()) {
                    return Some(at("intensity"));
                }
            } else if m.contains(Modifier::BOLD) != got.bold()
                || m.contains(Modifier::DIM) != got.dim()
            {
                return Some(at("modifier"));
            }
            let modifiers = [
                (Modifier::ITALIC, got.italic()),
                (Modifier::UNDERLINE, got.underline()),
                (Modifier::REVERSE, got.inverse()),
            ];
            if modifiers.iter().any(|&(flag, on)| m.contains(flag) != on) {
                return Some(at("modifier"));
            }
        }
    }
    None
}
