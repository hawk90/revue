//! Continues the paint sweep into `src/widget/data/` - twenty-three widgets
//! carrying DOM nodes, none of which read a computed style.
//!
//! Same rule as the rest of the sweep: assert against the painted buffer.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::Column;

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 40, 10).dom_from_render(true);
    h.draw(view);
    h
}

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

struct TableView;

impl View for TableView {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(
                Table::new(vec![
                    Column::new("Name").width(10),
                    Column::new("Age").width(5),
                ])
                .rows(vec![
                    vec!["Ada".into(), "36".into()],
                    vec!["Grace".into(), "45".into()],
                ])
                .element_id("tbl"),
            )
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "TableView"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

/// The *unselected* row: a selected one keeps its highlight colors, which are
/// semantic state rather than something a plain `color` rule should override.
#[test]
fn color_reaches_a_tables_unselected_rows() {
    let h = draw("#tbl { color: #ff0000; }", &TableView);
    assert!(any_fg(&h, RED), "`color` did not reach Table's rows");
}

/// And the selection highlight still wins on the row that has it.
#[test]
fn a_selected_row_keeps_its_highlight() {
    let h = draw("#tbl { color: #ff0000; }", &TableView);
    // Row 0 is selected by default and painted in the highlight color.
    let first_row_fg = h.buffer().get(1, 3).and_then(|c| c.fg);
    assert_ne!(
        first_row_fg,
        Some(RED),
        "a plain rule overrode the selection"
    );
}

struct ListView;

impl View for ListView {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(List::new(vec!["alpha".to_string(), "beta".to_string()]).element_id("lst"))
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "ListView"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

/// Row 0 is selected by default, so this checks row 1 - the one that had no
/// color at all before.
#[test]
fn color_reaches_a_lists_unselected_rows() {
    let h = draw("#lst { color: #ff0000; }", &ListView);

    assert_eq!(
        h.buffer().get(0, 1).and_then(|c| c.fg),
        Some(RED),
        "`color` did not reach List's unselected rows"
    );
}

/// A widget that sets no highlight color of its own falls back to the
/// stylesheet even on the selected row - there is nothing to keep. `Table`
/// differs only because it *has* a default highlight, not because the rule
/// differs.
#[test]
fn a_list_without_a_highlight_color_falls_back_to_css() {
    let h = draw("#lst { color: #ff0000; }", &ListView);

    assert_eq!(h.buffer().get(0, 0).and_then(|c| c.fg), Some(RED));
}

/// An ordinary day takes `color`; today, weekends and the selection keep their
/// own, being state a rule cannot address separately.
#[test]
fn color_reaches_a_calendars_ordinary_days() {
    struct CalendarView;
    impl View for CalendarView {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(revue::widget::Calendar::new(2026, 8).element_id("cal"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "CalendarView"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }

    let h = draw("#cal { color: #ff0000; }", &CalendarView);
    assert!(any_fg(&h, RED), "`color` did not reach Calendar");
}

/// The foreground of the first cell, in reading order, whose symbol matches.
fn fg_where(h: &PipelineHarness, matches: impl Fn(char) -> bool) -> Option<Color> {
    let buffer = h.buffer();
    (0..buffer.height()).find_map(|y| {
        (0..buffer.width()).find_map(|x| {
            buffer
                .get(x, y)
                .filter(|c| matches(c.symbol))
                .and_then(|c| c.fg)
        })
    })
}

struct StopwatchView(Option<Color>);

impl View for StopwatchView {
    fn render(&self, ctx: &mut RenderContext) {
        let mut sw = revue::widget::Stopwatch::new().element_id("sw");
        if let Some(color) = self.0 {
            sw = sw.fg(color);
        }
        vstack().child(sw).render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "StopwatchView"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

/// The elapsed time's color - the digits of "00:00".
fn stopwatch_digit_fg(css: &str, builder: Option<Color>) -> Option<Color> {
    fg_where(&draw(css, &StopwatchView(builder)), |c| c == '0')
}

/// Untouched by a stylesheet, the stopwatch paints as it always has.
#[test]
fn a_stopwatch_keeps_its_default_color() {
    assert_eq!(stopwatch_digit_fg("", None), Some(Color::WHITE));
}

/// `Stopwatch` used `self.fg.unwrap_or(WHITE)` and never looked at its
/// computed style, while `Timer` in the same file did - the ratchet only read
/// the file's first widget.
#[test]
fn color_reaches_a_stopwatchs_time() {
    assert_eq!(
        stopwatch_digit_fg("#sw { color: #ff0000; }", None),
        Some(RED),
        "`color` did not reach Stopwatch"
    );
}

/// A color the builder named outranks the stylesheet.
#[test]
fn a_named_stopwatch_color_beats_css() {
    let green = Color::rgb(0, 255, 0);
    assert_eq!(
        stopwatch_digit_fg("#sw { color: #ff0000; }", Some(green)),
        Some(green)
    );
}
