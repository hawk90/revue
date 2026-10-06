//! Text a widget draws is laid out in terminal columns, not bytes or chars.
//!
//! A wide glyph (emoji, CJK) spans two cells: the glyph, then a continuation
//! cell. Placing glyph `i` at `x + i` drops the continuation and lets the next
//! glyph land on top of it; measuring with `str::len` counts bytes, so a
//! box, an offset or a truncation point comes out wrong - and slicing user
//! text at a byte count panics when it falls inside a multibyte glyph.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    AiStream, Canvas, Collapsible, ContextMenu, Diagram, DiagramNode, DrawContext, Gauge, Menu,
    MenuBar, MenuItem, PieChart, PieLabelStyle, Popover, Presentation, Slide, SlideAlign, Slider,
    Step, Stepper, Switch, SwitchStyle, Tooltip, TooltipPosition, Waveline,
};

fn render(view: &dyn View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
    buffer
}

/// A buffer row as the terminal shows it: a wide glyph is one `char`
/// spanning two cells, so its continuation cell contributes nothing.
fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect()
}

/// First cell holding `ch`, scanning rows top to bottom.
fn find(buffer: &Buffer, ch: char) -> (u16, u16) {
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if buffer.get(x, y).map(|c| c.symbol) == Some(ch) {
                return (x, y);
            }
        }
    }
    panic!("{ch:?} not drawn");
}

fn find_in_row(buffer: &Buffer, y: u16, ch: char) -> Option<u16> {
    (0..buffer.width()).find(|&x| buffer.get(x, y).map(|c| c.symbol) == Some(ch))
}

/// `wide` is drawn as a two-cell glyph and `next` starts right after it.
fn assert_wide_then(buffer: &Buffer, wide: char, next: char) -> (u16, u16) {
    let (x, y) = find(buffer, wide);
    assert!(
        buffer.get(x + 1, y).unwrap().is_continuation(),
        "{wide:?} at ({x},{y}) lost its continuation cell: {:?}",
        row_text(buffer, y)
    );
    assert_eq!(
        buffer.get(x + 2, y).unwrap().symbol,
        next,
        "row {y}: {:?}",
        row_text(buffer, y)
    );
    (x, y)
}

// ─── Stepper ────────────────────────────────────────────────────────────────

#[test]
fn stepper_truncating_a_multibyte_title_does_not_panic() {
    // One step in 6 columns leaves 5 for the title; the old code then sliced
    // the 12-byte title at byte 4, inside the second 3-byte glyph.
    let stepper = Stepper::new().step(Step::new("설정하기"));
    let buffer = render(&stepper, 6, 3);
    assert_eq!(row_text(&buffer, 1).trim_end(), "설정…");
}

#[test]
fn stepper_truncating_a_multibyte_description_does_not_panic() {
    let stepper = Stepper::new().step(Step::new("A").description("설정하기"));
    let buffer = render(&stepper, 6, 3);
    assert_eq!(row_text(&buffer, 2).trim_end(), "설정…");
}

#[test]
fn stepper_horizontal_wide_title_takes_two_cells_per_glyph() {
    let stepper = Stepper::new().step(Step::new("🔄Sync"));
    let buffer = render(&stepper, 20, 3);
    assert_wide_then(&buffer, '🔄', 'S');
    assert_eq!(row_text(&buffer, 1).trim_end(), "🔄Sync");
}

#[test]
fn stepper_vertical_wide_title_takes_two_cells_per_glyph() {
    let stepper = Stepper::new()
        .vertical()
        .step(Step::new("設定").description("🔄Sync"));
    let buffer = render(&stepper, 20, 4);
    assert_wide_then(&buffer, '設', '定');
    assert_wide_then(&buffer, '🔄', 'S');
}

#[test]
fn stepper_ascii_title_still_truncates_with_ellipsis() {
    let stepper = Stepper::new().step(Step::new("Configure"));
    let buffer = render(&stepper, 6, 3);
    assert_eq!(row_text(&buffer, 1).trim_end(), "Conf…");
}

// ─── Tooltip ────────────────────────────────────────────────────────────────

fn tooltip(text: &str) -> Tooltip {
    Tooltip::new(text)
        .position(TooltipPosition::Bottom)
        .anchor(0, 0)
}

#[test]
fn tooltip_wide_title_and_text_fit_the_box() {
    let tip = tooltip("設定 ok").title("🔄Sync");
    let buffer = render(&tip, 30, 10);
    let (x, y) = assert_wide_then(&buffer, '🔄', 'S');
    // The box is sized by display width (title 6 + 2, plus border/padding 4),
    // and the title row ends exactly on the right border.
    let right = (x + 1..buffer.width()).find(|&cx| buffer.get(cx, y).unwrap().symbol == '│');
    assert_eq!(right, Some(x + 6 + 2 + 1));
    assert_wide_then(&buffer, '設', '定');
}

#[test]
fn tooltip_wraps_by_display_width() {
    // 17 columns but 25 bytes: fits a 20-column tooltip on one line.
    let tip = tooltip("가나다라 마바사아").max_width(20);
    let buffer = render(&tip, 40, 10);
    let (_, y) = find(&buffer, '가');
    assert!(
        row_text(&buffer, y).contains("가나다라 마바사아"),
        "{:?}",
        row_text(&buffer, y)
    );
}

#[test]
fn tooltip_ascii_title_unchanged() {
    let tip = tooltip("hello").title("Info");
    let buffer = render(&tip, 30, 10);
    let (_, y) = find(&buffer, 'I');
    assert_eq!(row_text(&buffer, y).trim_end(), "│ Info   │");
}

// ─── Popover ────────────────────────────────────────────────────────────────

fn popover(content: &str) -> Popover {
    Popover::new(content).anchor(0, 0).open(true)
}

/// Width of the popover box, read off its top border.
fn box_width(buffer: &Buffer) -> u16 {
    let (l, y) = find(buffer, '┌');
    let r = find_in_row(buffer, y, '┐').unwrap();
    r - l + 1
}

#[test]
fn popover_wide_title_is_sized_and_placed_by_columns() {
    let pop = popover("x").title("設定画面");
    let buffer = render(&pop, 40, 10);
    // title 8 + 2, plus border/padding 4 = 14 (bytes would have made it 18)
    assert_eq!(box_width(&buffer), 14);
    assert_wide_then(&buffer, '設', '定');
}

#[test]
fn popover_wide_content_is_sized_and_placed_by_columns() {
    let pop = popover("🔄 同期中です");
    let buffer = render(&pop, 40, 10);
    // content 2 + 1 + 10 = 13, plus 4 = 17
    assert_eq!(box_width(&buffer), 17);
    assert_wide_then(&buffer, '🔄', ' ');
    assert_wide_then(&buffer, '同', '期');
}

#[test]
fn popover_wraps_by_display_width() {
    let pop = popover("가나다라 마바사아").max_width(20);
    let buffer = render(&pop, 40, 10);
    let (_, y) = find(&buffer, '가');
    assert!(row_text(&buffer, y).contains("가나다라 마바사아"));
}

#[test]
fn popover_ascii_size_unchanged() {
    let pop = popover("hello world").title("Info");
    let buffer = render(&pop, 40, 10);
    assert_eq!(box_width(&buffer), 15);
}

// ─── Gauge ──────────────────────────────────────────────────────────────────

#[test]
fn gauge_wide_title_takes_two_cells_per_glyph() {
    let gauge = Gauge::new().title("🔄CPU").percent(50.0);
    let buffer = render(&gauge, 20, 4);
    assert_wide_then(&buffer, '🔄', 'C');
    assert!(row_text(&buffer, 0).starts_with("🔄CPU"));
}

#[test]
fn gauge_ascii_title_unchanged() {
    let gauge = Gauge::new().title("CPU").percent(50.0);
    let buffer = render(&gauge, 20, 4);
    assert!(row_text(&buffer, 0).starts_with("CPU "));
}

// ─── Diagram (mermaid) ──────────────────────────────────────────────────────

#[test]
fn diagram_wide_title_and_label_take_two_cells_per_glyph() {
    let diagram = Diagram::new()
        .title("🔄Flow")
        .node(DiagramNode::new("a", "設定"));
    let buffer = render(&diagram, 30, 10);
    assert_wide_then(&buffer, '🔄', 'F');
    let (x, y) = assert_wide_then(&buffer, '設', '定');
    // The box is sized by display width: 4 + 4 = 8, label centered in it.
    let left = (0..x)
        .rev()
        .find(|&cx| buffer.get(cx, y).unwrap().symbol == '│');
    let right = (x + 1..buffer.width()).find(|&cx| buffer.get(cx, y).unwrap().symbol == '│');
    let (left, right) = (left.unwrap(), right.unwrap());
    assert_eq!(right - left + 1, 8);
    assert_eq!(x - left, 2);
}

#[test]
fn diagram_label_longer_than_its_box_does_not_panic() {
    let diagram = Diagram::new().node(DiagramNode::new("a", "an extremely long node label"));
    let _ = render(&diagram, 12, 6);
}

#[test]
fn diagram_ascii_title_unchanged() {
    let diagram = Diagram::new()
        .title("Flow")
        .node(DiagramNode::new("a", "Go"));
    let buffer = render(&diagram, 30, 10);
    assert!(row_text(&buffer, 0).starts_with("Flow "));
}

// ─── MenuBar / ContextMenu ──────────────────────────────────────────────────

fn menu_bar() -> MenuBar {
    MenuBar::new()
        .menu(Menu::new("ファイル").item(MenuItem::new("x")))
        .menu(
            Menu::new("Edit")
                .item(MenuItem::new("🔄Redo").shortcut("⌘⇧Z"))
                .item(MenuItem::new("設定").shortcut("Ctrl+,")),
        )
}

#[test]
fn menu_bar_wide_title_advances_by_columns() {
    let bar = menu_bar();
    let buffer = render(&bar, 40, 8);
    assert_wide_then(&buffer, 'フ', 'ァ');
    // " ファイル " is 10 columns, so " Edit " starts at 10.
    assert_eq!(buffer.get(11, 0).unwrap().symbol, 'E');
    assert_eq!(row_text(&buffer, 0).trim_end(), " ファイル  Edit");
}

#[test]
fn menu_bar_dropdown_sits_under_its_title_and_fits_wide_labels() {
    let mut bar = menu_bar();
    bar.open_menu(1);
    let buffer = render(&bar, 40, 8);
    // Dropdown starts under " Edit ", at column 10 (bytes would say 16).
    assert_eq!(buffer.get(10, 1).unwrap().symbol, '┌');
    assert_wide_then(&buffer, '🔄', 'R');
    assert_wide_then(&buffer, '設', '定');
    // Widest row: "設定" (4) + shortcut "Ctrl+," (6) + 2 + 4 = 16 columns.
    assert_eq!(buffer.get(10 + 15, 1).unwrap().symbol, '┐');
    // A shortcut ends two cells in from the right border.
    let (_, y) = find(&buffer, 'R');
    assert_eq!(buffer.get(10 + 13, y).unwrap().symbol, 'Z');
}

#[test]
fn menu_bar_ascii_unchanged() {
    let mut bar = MenuBar::new()
        .menu(Menu::new("File").item(MenuItem::new("Open").shortcut("Ctrl+O")))
        .menu(Menu::new("Edit").item(MenuItem::new("Undo")));
    bar.open_menu(1);
    let buffer = render(&bar, 40, 8);
    assert_eq!(row_text(&buffer, 0).trim_end(), " File  Edit");
    assert_eq!(buffer.get(6, 1).unwrap().symbol, '┌');
}

#[test]
fn context_menu_is_as_wide_as_its_widest_label_in_columns() {
    let mut menu = ContextMenu::new().item(MenuItem::new("設定画面"));
    menu.show(0, 0);
    let buffer = render(&menu, 40, 6);
    // label 8 + 4 = 12 (bytes would have made it 16)
    assert_eq!(find_in_row(&buffer, 0, '┐'), Some(11));
    assert_wide_then(&buffer, '設', '定');
}

// ─── DiffViewer ─────────────────────────────────────────────────────────────

#[cfg(feature = "diff")]
#[test]
fn diff_header_wide_names_take_two_cells_per_glyph() {
    use revue::widget::{DiffMode, DiffViewer};
    let diff = DiffViewer::new()
        .mode(DiffMode::Split)
        .left_name("旧版.rs")
        .right_name("🔄new.rs")
        .compare("a\n", "b\n");
    let buffer = render(&diff, 40, 5);
    assert_wide_then(&buffer, '旧', '版');
    assert_wide_then(&buffer, '版', '.');
    assert_wide_then(&buffer, '🔄', 'n');
}

// ─── Switch / Slider ────────────────────────────────────────────────────────

#[test]
fn switch_wide_text_takes_two_cells_and_closes_after_it() {
    let sw = Switch::new()
        .style(SwitchStyle::Text)
        .text("켜짐", "꺼짐")
        .on(true);
    let buffer = render(&sw, 20, 1);
    assert_wide_then(&buffer, '켜', '짐');
    assert_eq!(row_text(&buffer, 0).trim_end(), "[켜짐]");
}

#[test]
fn switch_ascii_text_unchanged() {
    let sw = Switch::new().style(SwitchStyle::Text).on(true);
    let buffer = render(&sw, 20, 1);
    assert_eq!(row_text(&buffer, 0).trim_end(), "[ON]");
}

#[test]
fn slider_wide_label_takes_two_cells_and_track_follows() {
    let slider = Slider::new().label("音量").value(50.0);
    let buffer = render(&slider, 30, 1);
    assert_wide_then(&buffer, '音', '量');
    // label (4) + 1 space: the track starts at column 5, not 7.
    let (x, _) = find(&buffer, '量');
    assert_eq!(buffer.get(x + 2, 0).unwrap().symbol, ' ');
    assert_ne!(buffer.get(x + 3, 0).unwrap().symbol, ' ');
}

// ─── Charts ─────────────────────────────────────────────────────────────────

#[test]
fn pie_chart_wide_label_takes_two_cells_per_glyph() {
    let pie = PieChart::new()
        .no_legend()
        .labels(PieLabelStyle::Label)
        .slice("設定", 1.0)
        .slice("🔄x", 1.0);
    let buffer = render(&pie, 60, 20);
    // The first slice's label falls right of the area; the second's is drawn
    // on the left.
    assert_wide_then(&buffer, '🔄', 'x');
}

#[test]
fn waveline_wide_label_takes_two_cells_per_glyph() {
    let wave = Waveline::new(vec![0.1, 0.5, 0.9]).label("🔄Wave");
    let buffer = render(&wave, 20, 5);
    assert_wide_then(&buffer, '🔄', 'W');
}

// ─── Presentation ───────────────────────────────────────────────────────────

fn presentation_slide(slide: Slide) -> Presentation {
    Presentation::new()
        .numbers(false)
        .progress(false)
        .slide(slide)
}

#[test]
fn presentation_wide_title_is_centered_by_columns() {
    let pres = presentation_slide(Slide::new("発表"));
    let buffer = render(&pres, 20, 12);
    // 4 columns in 20: starts at column 8 (chars would have said 9)
    assert_eq!(assert_wide_then(&buffer, '発', '表'), (8, 2));
    // The separator under it is as wide as the title in columns
    assert_eq!(row_text(&buffer, 4).trim(), "────");
}

#[test]
fn presentation_wide_left_and_right_lines_take_two_cells_per_glyph() {
    let left = presentation_slide(Slide::new("T").line("設定x").align(SlideAlign::Left));
    let buffer = render(&left, 20, 12);
    assert_eq!(assert_wide_then(&buffer, '設', '定'), (2, 6));
    assert_wide_then(&buffer, '定', 'x');

    let right = presentation_slide(Slide::new("T").line("設定x").align(SlideAlign::Right));
    let buffer = render(&right, 20, 12);
    // 5 columns ending 2 before the edge: starts at column 13
    assert_eq!(assert_wide_then(&buffer, '設', '定'), (13, 6));
    assert_wide_then(&buffer, '定', 'x');
}

#[test]
fn presentation_wide_title_slide_is_centered_by_columns() {
    let pres = Presentation::new().title("発表会").author("🔄me");
    let buffer = render(&pres, 20, 12);
    assert_eq!(assert_wide_then(&buffer, '発', '表').0, 7);
    assert_eq!(assert_wide_then(&buffer, '🔄', 'm').0, 8);
}

#[test]
fn presentation_ascii_slide_unchanged() {
    let pres = presentation_slide(Slide::new("Hello").line("abc").align(SlideAlign::Left));
    let buffer = render(&pres, 21, 12);
    assert_eq!(row_text(&buffer, 2), "        Hello        ");
    assert_eq!(row_text(&buffer, 4).trim(), "─────");
    assert_eq!(row_text(&buffer, 6).trim_end(), "  abc");
}

// ─── MarkdownPresentation ───────────────────────────────────────────────────

/// Length of the title separator: the first row (above the footer) drawn
/// only in `─`.
#[cfg(feature = "markdown")]
fn separator_len(buffer: &Buffer) -> usize {
    (0..buffer.height() - 1)
        .map(|y| row_text(buffer, y))
        .find(|row| !row.trim().is_empty() && row.trim().chars().all(|c| c == '─'))
        .map(|row| row.trim().chars().count())
        .expect("no separator row")
}

#[cfg(feature = "markdown")]
#[test]
fn markdown_presentation_separator_is_sized_by_title_columns() {
    use revue::widget::{MarkdownPresentation, ViewMode};
    // 8 glyphs, 16 columns, 24 bytes: twice the columns is 32 (bytes gave 48)
    let pres = MarkdownPresentation::new("# 日本語のタイトル\n\nbody\n")
        .mode(ViewMode::Slides)
        .text_sizing(false)
        .numbers(false)
        .progress(false);
    let buffer = render(&pres, 60, 20);
    assert_eq!(separator_len(&buffer), 32);
}

#[cfg(feature = "markdown")]
#[test]
fn markdown_presentation_ascii_separator_unchanged() {
    use revue::widget::{MarkdownPresentation, ViewMode};
    let pres = MarkdownPresentation::new("# A title that is long\n\nbody\n")
        .mode(ViewMode::Slides)
        .text_sizing(false)
        .numbers(false)
        .progress(false);
    let buffer = render(&pres, 60, 20);
    assert_eq!(separator_len(&buffer), 40);
}

// ─── AiStream ───────────────────────────────────────────────────────────────

fn ai_stream(text: &str) -> AiStream {
    let mut stream = AiStream::new();
    stream.set_content(text);
    stream
}

#[test]
fn ai_stream_wide_text_takes_two_cells_per_glyph() {
    let buffer = render(&ai_stream("設定🔄x"), 20, 2);
    assert_wide_then(&buffer, '設', '定');
    assert_wide_then(&buffer, '定', '🔄');
    assert_wide_then(&buffer, '🔄', 'x');
}

#[test]
fn ai_stream_wraps_by_display_width() {
    // 5 columns hold two 2-column glyphs; the third wraps
    let buffer = render(&ai_stream("日本語"), 5, 3);
    assert_eq!(row_text(&buffer, 0).trim_end(), "日本");
    assert_eq!(row_text(&buffer, 1).trim_end(), "語");
}

#[test]
fn ai_stream_ascii_wrap_unchanged() {
    let buffer = render(&ai_stream("abcdefg\nhi"), 5, 3);
    assert_eq!(row_text(&buffer, 0), "abcde");
    assert_eq!(row_text(&buffer, 1).trim_end(), "fg");
    assert_eq!(row_text(&buffer, 2).trim_end(), "hi");
}

// ─── Transition ─────────────────────────────────────────────────────────────

#[test]
fn transition_wide_text_takes_two_cells_per_glyph() {
    let transition = revue::widget::AnimationTransition::new("設定🔄x");
    let buffer = render(&transition, 20, 1);
    assert_wide_then(&buffer, '設', '定');
    assert_wide_then(&buffer, '定', '🔄');
    assert_wide_then(&buffer, '🔄', 'x');
}

#[test]
fn transition_ascii_text_unchanged() {
    let transition = revue::widget::AnimationTransition::new("Hello");
    let buffer = render(&transition, 8, 1);
    assert_eq!(row_text(&buffer, 0), "Hello   ");
}

#[test]
fn transition_group_wide_items_take_two_cells_per_glyph() {
    let group = revue::widget::TransitionGroup::new(["設定x", "🔄y"]);
    let buffer = render(&group, 20, 2);
    assert_wide_then(&buffer, '設', '定');
    assert_wide_then(&buffer, '定', 'x');
    assert_eq!(assert_wide_then(&buffer, '🔄', 'y'), (0, 1));
}

// ─── Collapsible ────────────────────────────────────────────────────────────

#[test]
fn collapsible_wide_title_and_content_take_two_cells_per_glyph() {
    let c = Collapsible::new("設定x").line("日本y").expanded(true);
    let buffer = render(&c, 20, 4);
    assert_eq!(assert_wide_then(&buffer, '設', '定'), (2, 0));
    assert_wide_then(&buffer, '定', 'x');
    assert_eq!(assert_wide_then(&buffer, '日', '本'), (2, 1));
    assert_wide_then(&buffer, '本', 'y');
}

#[test]
fn collapsible_truncates_content_by_display_width() {
    // 10 wide leaves 7 columns for content: three glyphs, not seven
    let c = Collapsible::new("T").line("日本語のテスト").expanded(true);
    let buffer = render(&c, 10, 4);
    assert_eq!(row_text(&buffer, 1).trim_end(), "│ 日本語");
}

#[test]
fn collapsible_ascii_unchanged() {
    let c = Collapsible::new("Title").line("abcdefghij").expanded(true);
    let buffer = render(&c, 10, 4);
    assert_eq!(row_text(&buffer, 0), "▼ Title   ");
    assert_eq!(row_text(&buffer, 1).trim_end(), "│ abcdefg");
}

// ─── Canvas ─────────────────────────────────────────────────────────────────

#[test]
fn canvas_wide_text_takes_two_cells_per_glyph() {
    let canvas = Canvas::new(|ctx: &mut DrawContext| {
        ctx.text(1, 0, "設定x", None);
        ctx.text_bold(1, 1, "🔄y", None);
    });
    let buffer = render(&canvas, 10, 2);
    assert_eq!(assert_wide_then(&buffer, '設', '定'), (1, 0));
    assert_wide_then(&buffer, '定', 'x');
    assert_eq!(assert_wide_then(&buffer, '🔄', 'y'), (1, 1));
    let cont = buffer.get(2, 1).unwrap();
    assert!(cont.modifier.contains(revue::render::Modifier::BOLD));
}

#[test]
fn canvas_text_stops_before_a_wide_glyph_that_would_cross_the_edge() {
    let canvas = Canvas::new(|ctx: &mut DrawContext| ctx.text(0, 0, "ab設", None));
    let buffer = render(&canvas, 3, 1);
    assert_eq!(row_text(&buffer, 0), "ab ");
}

#[test]
fn canvas_text_bold_stays_inside_its_area() {
    // A canvas one row tall, inside a taller buffer: text on row 1 is
    // outside it
    let canvas = Canvas::new(|ctx: &mut DrawContext| ctx.text_bold(0, 1, "x", None));
    let mut buffer = Buffer::new(4, 3);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 4, 1));
    canvas.render(&mut ctx);
    assert_eq!(row_text(&buffer, 1), "    ");
}

#[test]
fn canvas_ascii_text_unchanged() {
    let canvas = Canvas::new(|ctx: &mut DrawContext| ctx.text(1, 0, "abcdef", None));
    let buffer = render(&canvas, 5, 1);
    assert_eq!(row_text(&buffer, 0), " abcd");
}
