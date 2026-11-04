//! Terminal backend types

use std::io::Write;

mod cursor;

/// Tracks current terminal styling state to minimize escape sequences
#[derive(Default)]
pub(crate) struct RenderState {
    pub(crate) fg: Option<crate::style::Color>,
    pub(crate) bg: Option<crate::style::Color>,
    pub(crate) modifier: crate::render::cell::Modifier,
    /// Current hyperlink ID (None means no hyperlink active)
    pub(crate) hyperlink_id: Option<u16>,
    /// Expected cursor position after last print (x, y)
    /// Used to avoid redundant MoveTo commands for contiguous cells
    pub(crate) cursor: Option<(u16, u16)>,
}

/// Terminal backend for rendering
pub struct Terminal<W: Write> {
    /// Output writer
    pub(crate) writer: W,
    /// Current buffer (what's on screen)
    pub(crate) current: crate::render::Buffer,
    /// Whether we're in raw mode
    pub(crate) raw_mode: bool,
    /// Whether mouse capture is enabled
    pub(crate) mouse_capture: bool,
    /// Where the terminal's cursor is shown, `None` while it is hidden
    cursor: Option<(u16, u16)>,
    /// The cells last drawn in place of a caret, as `(x, y, width)`: the
    /// screen shows them as they look without it
    lifted: Option<(u16, u16, u16)>,
}

impl<W: Write> Terminal<W> {
    /// A terminal writing to `writer`, showing `current`, its cursor hidden
    pub(crate) fn assemble(writer: W, current: crate::render::Buffer) -> Self {
        Self {
            writer,
            current,
            raw_mode: false,
            mouse_capture: false,
            cursor: None,
            lifted: None,
        }
    }
}
