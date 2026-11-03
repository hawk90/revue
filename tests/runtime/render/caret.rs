//! The focused field's caret is shown with the terminal's own cursor
//! (#1457): an input method puts its preedit and candidate window at the
//! cursor, so the cursor has to be at the field. The screen shows the
//! caret's cells as they look without it; the buffer keeps the drawn caret
//! for output that has no cursor (snapshots, buffer tests).

use revue::event::Key;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{vstack, CodeEditor, Input, RichTextEditor, TextArea, View};

use crate::emulator::{cursor_difference, difference, emulate};

const SHOW: &[u8] = b"\x1b[?25h";
const HIDE: &[u8] = b"\x1b[?25l";

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

/// Draw `view` once; the emulator's screen must be the shown buffer, its
/// cursor at the caret. Returns the harness.
fn drawn<V: View>(view: &V, w: u16, h: u16) -> PipelineHarness {
    let mut harness = PipelineHarness::new(w, h);
    harness.draw(view);
    assert_reads_back(&harness, w, h);
    harness
}

fn assert_reads_back(harness: &PipelineHarness, w: u16, h: u16) {
    let parser = emulate(harness.terminal_output(), w, h);
    if let Some(diff) = difference(&harness.screen_buffer(), parser.screen())
        .or_else(|| cursor_difference(harness.cursor(), parser.screen()))
    {
        panic!("{diff}");
    }
}

/// The symbol and colors of a cell
fn look(buffer: &Buffer, x: u16, y: u16) -> (char, Option<Color>, Option<Color>) {
    let cell = buffer.get(x, y).unwrap();
    (cell.symbol, cell.fg, cell.bg)
}

// ---------------------------------------------------------------------------
// One-line fields
// ---------------------------------------------------------------------------

#[test]
fn the_cursor_shows_at_the_end_of_a_focused_input() {
    let h = drawn(&Input::new().value("abc"), 10, 1);
    assert_eq!(h.cursor(), Some((3, 0)));
    // The buffer keeps the drawn caret; the screen shows the blank under it
    assert_eq!(look(h.buffer(), 3, 0).2, Some(Color::WHITE));
    assert_eq!(look(&h.screen_buffer(), 3, 0), (' ', None, None));
    assert!(contains(h.terminal_output(), SHOW));
}

#[test]
fn the_cursor_shows_over_the_glyph_it_is_on() {
    let mut input = Input::new().value("abc");
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Left);
    let h = drawn(&input, 10, 1);
    assert_eq!(h.cursor(), Some((1, 0)));
    assert_eq!(
        look(h.buffer(), 1, 0),
        ('b', Some(Color::BLACK), Some(Color::WHITE))
    );
    // The glyph as the text draws it: no caret colors
    assert_eq!(look(&h.screen_buffer(), 1, 0), ('b', None, None));
}

#[test]
fn on_a_wide_glyph_the_cursor_is_at_its_first_column_and_both_halves_are_text() {
    let mut input = Input::new().value("a한b");
    input.handle_key(&Key::Left);
    input.handle_key(&Key::Left);
    let h = drawn(&input, 10, 1);
    assert_eq!(h.cursor(), Some((1, 0)));
    let screen = h.screen_buffer();
    assert_eq!(look(&screen, 1, 0), ('한', None, None));
    assert!(screen.get(2, 0).unwrap().is_continuation());
    assert_eq!(screen.get(2, 0).unwrap().bg, None);
}

#[test]
fn the_cursor_follows_a_scrolled_input() {
    // Ten chars in a field five wide: the end is in view at column 4
    let h = drawn(&Input::new().value("abcdefghij"), 5, 1);
    assert_eq!(h.cursor(), Some((4, 0)));
    // After two more cells of wide text the column counts glyph widths
    let h = drawn(&Input::new().value("ab한글"), 5, 1);
    assert_eq!(h.cursor(), Some((4, 0)));
}

#[test]
fn the_cursor_shows_over_the_placeholder_of_an_empty_input() {
    let h = drawn(&Input::new().placeholder("name"), 10, 1);
    assert_eq!(h.cursor(), Some((0, 0)));
    let screen = h.screen_buffer();
    assert_eq!(screen.get(0, 0).unwrap().symbol, 'n');
    // The placeholder's own color, not the caret's
    assert_ne!(screen.get(0, 0).unwrap().bg, Some(Color::WHITE));
}

#[test]
fn an_unfocused_input_shows_no_cursor_and_hides_the_one_shown() {
    let h = drawn(&Input::new().value("abc").focused(false), 10, 1);
    assert_eq!(h.cursor(), None);
    assert!(!contains(h.terminal_output(), SHOW));

    let mut h = PipelineHarness::new(10, 1);
    h.draw(&Input::new().value("abc"));
    assert_eq!(h.cursor(), Some((3, 0)));
    h.clear_terminal_output();
    h.draw(&Input::new().value("abc").focused(false));
    assert_eq!(h.cursor(), None);
    assert!(contains(h.terminal_output(), HIDE));
}

#[test]
fn of_two_focused_inputs_the_last_drawn_has_the_cursor_and_the_other_keeps_its_caret() {
    let view = vstack()
        .child(Input::new().value("one"))
        .child(Input::new().value("two"));
    let h = drawn(&view, 10, 2);
    assert_eq!(h.cursor(), Some((3, 1)));
    // The first keeps the caret it drew, on screen too
    assert_eq!(look(&h.screen_buffer(), 3, 0).2, Some(Color::WHITE));
    assert_eq!(look(&h.screen_buffer(), 3, 1), (' ', None, None));
}

#[test]
fn a_caret_that_loses_the_cursor_is_drawn_again() {
    // Frame 1: only the first input has a caret, shown by the cursor. Frame
    // 2: the second takes the cursor; the first's caret cells are the same
    // in both buffers, so only the terminal knows to draw them back.
    let mut h = PipelineHarness::new(10, 2);
    h.draw(
        &vstack()
            .child(Input::new().value("one"))
            .child(Input::new().value("two").focused(false)),
    );
    assert_eq!(h.cursor(), Some((3, 0)));
    h.draw(
        &vstack()
            .child(Input::new().value("one"))
            .child(Input::new().value("two")),
    );
    assert_eq!(h.cursor(), Some((3, 1)));
    assert_reads_back(&h, 10, 2);
}

#[test]
fn an_unchanged_frame_writes_nothing() {
    let mut h = PipelineHarness::new(10, 1);
    h.draw(&Input::new().value("abc"));
    h.clear_terminal_output();
    h.draw(&Input::new().value("abc"));
    assert_eq!(h.terminal_output(), b"");
    assert_eq!(h.cursor(), Some((3, 0)));
}

#[test]
fn a_moved_caret_moves_the_cursor_and_redraws_both_places() {
    let mut h = PipelineHarness::new(10, 1);
    h.draw(&Input::new().value("abc"));
    let mut input = Input::new().value("abc");
    input.handle_key(&Key::Home);
    h.draw(&input);
    assert_eq!(h.cursor(), Some((0, 0)));
    assert_reads_back(&h, 10, 1);
}

#[test]
fn a_caret_outside_the_screen_shows_no_cursor() {
    // The input is on the second row of a view one row high
    let view = vstack()
        .child(Input::new().value("one").focused(false))
        .child(Input::new().value("two"));
    let h = drawn(&view, 10, 1);
    assert_eq!(h.cursor(), None);
}

// ---------------------------------------------------------------------------
// The buffer's caret
// ---------------------------------------------------------------------------

#[test]
fn a_cell_drawn_over_the_caret_takes_the_cursor_away() {
    use revue::widget::RenderContext;
    let mut buffer = Buffer::new(10, 1);
    let area = revue::layout::Rect::new(0, 0, 10, 1);
    Input::new()
        .value("abc")
        .render(&mut RenderContext::new(&mut buffer, area));
    let mut terminal = revue::render::Terminal::with_size(Vec::new(), 10, 1);
    terminal.force_redraw(&buffer).unwrap();
    assert!(contains(terminal.writer(), SHOW));

    // A popup over the caret: the cursor would show through it
    buffer.set(3, 0, Cell::new('x'));
    let mut terminal = revue::render::Terminal::with_size(Vec::new(), 10, 1);
    terminal.force_redraw(&buffer).unwrap();
    assert!(!contains(terminal.writer(), SHOW));
    let parser = emulate(terminal.writer(), 10, 1);
    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "x");
}

#[test]
fn a_caret_wider_than_its_field_shows_no_cursor_and_leaves_the_next_cell() {
    // A field one column wide, its cursor on a wide glyph it cannot show:
    // the glyph's cells would reach past the field
    use revue::widget::RenderContext;
    let mut buffer = Buffer::new(3, 1);
    buffer.set(1, 0, Cell::new('z'));
    let mut input = Input::new().value("한");
    input.handle_key(&Key::Home);
    input.render(&mut RenderContext::new(
        &mut buffer,
        revue::layout::Rect::new(0, 0, 1, 1),
    ));
    let mut terminal = revue::render::Terminal::with_size(Vec::new(), 3, 1);
    terminal.force_redraw(&buffer).unwrap();
    assert!(!contains(terminal.writer(), SHOW));
    let parser = emulate(terminal.writer(), 3, 1);
    assert_eq!(parser.screen().cell(0, 1).unwrap().contents(), "z");
}

// ---------------------------------------------------------------------------
// Editors
// ---------------------------------------------------------------------------

#[test]
fn a_textarea_shows_the_cursor_at_its_primary_cursor_only() {
    // An added cursor is the primary one
    let mut area = TextArea::new().content("ab\ncd").focused(true);
    area.add_cursor_at(1, 1);
    let h = drawn(&area, 10, 2);
    assert_eq!(h.cursor(), Some((1, 1)));
    let screen = h.screen_buffer();
    assert_ne!(screen.get(1, 1).unwrap().bg, Some(Color::WHITE));
    // The other cursor stays drawn
    assert_eq!(screen.get(0, 0).unwrap().bg, Some(Color::WHITE));
}

#[test]
fn a_textarea_cursor_on_a_tab_lifts_every_cell_of_it() {
    let area = TextArea::new().content("\tx").tab_width(4).focused(true);
    let h = drawn(&area, 10, 1);
    assert_eq!(h.cursor(), Some((0, 0)));
    let screen = h.screen_buffer();
    for x in 0..4 {
        assert_ne!(
            screen.get(x, 0).unwrap().bg,
            Some(Color::WHITE),
            "column {x}"
        );
    }
}

#[test]
fn a_textarea_cursor_past_the_line_end_shows_there() {
    let mut area = TextArea::new().content("ab").focused(true);
    area.handle_key(&Key::End);
    let h = drawn(&area, 10, 1);
    assert_eq!(h.cursor(), Some((2, 0)));
}

#[test]
fn a_code_editor_shows_the_cursor_on_and_past_the_line() {
    let editor = CodeEditor::new()
        .content("ab")
        .line_numbers(false)
        .focused(true);
    let h = drawn(&editor, 10, 1);
    assert_eq!(h.cursor(), Some((0, 0)));
    // The glyph without the caret's background the buffer drew it with
    let (caret, shown) = (h.buffer().get(0, 0).unwrap(), h.screen_buffer());
    assert_eq!(shown.get(0, 0).unwrap().symbol, 'a');
    assert_ne!(shown.get(0, 0).unwrap().bg, caret.bg);
    assert_eq!(shown.get(1, 0), h.buffer().get(1, 0));

    let mut editor = CodeEditor::new()
        .content("ab")
        .line_numbers(false)
        .focused(true);
    editor.handle_key(&Key::End);
    let h = drawn(&editor, 10, 1);
    assert_eq!(h.cursor(), Some((2, 0)));
}

#[test]
fn a_rich_text_editor_shows_the_cursor_on_and_past_the_text() {
    let editor = RichTextEditor::new().content("ab").focused(true);
    let h = drawn(&editor, 20, 4);
    let (x, y) = h.cursor().expect("a cursor");
    let (caret, shown) = (h.buffer().get(x, y).unwrap(), h.screen_buffer());
    assert_eq!(shown.get(x, y).unwrap().symbol, 'a');
    // The text's background, as the next glyph has it
    assert_ne!(shown.get(x, y).unwrap().bg, caret.bg);
    assert_eq!(shown.get(x, y).unwrap().bg, shown.get(x + 1, y).unwrap().bg);

    let mut editor = RichTextEditor::new().content("ab").focused(true);
    editor.handle_key(&Key::End);
    let h = drawn(&editor, 20, 4);
    assert_eq!(h.cursor(), Some((x + 2, y)));
}
