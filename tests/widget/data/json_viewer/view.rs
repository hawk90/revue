//! JsonViewer rendering: empty input, the row layout and colors, a matrix
//! over random documents and options, and the widget in a stack
//!
//! Only behaviour that holds today is asserted. Open bugs (#860 speed, the
//! view not following the selection #973, wide characters #974) stay with
//! their issues.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::json_viewer::{JsonViewer, Search};
use revue::widget::traits::{RenderContext, View};

fn render(viewer: &JsonViewer, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width.max(1), height.max(1));
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    viewer.render(&mut ctx);
    buffer
}

fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Column of the first occurrence of `needle` in row `y`
fn column_of(buffer: &Buffer, y: u16, needle: char) -> Option<u16> {
    (0..buffer.width()).find(|&x| buffer.get(x, y).map(|c| c.symbol) == Some(needle))
}

const DOC: &str = r#"{"s":"x","n":1,"b":true,"z":null,"o":{},"l":[]}"#;

// =========================================================================
// Empty input
// =========================================================================

#[test]
fn test_json_viewer_empty_draws_nothing_at_any_size() {
    for viewer in [JsonViewer::new(), JsonViewer::from_content("{bad")] {
        assert!(!viewer.has_data());
        assert_eq!(viewer.visible_count(), 0);
        for w in 0..30 {
            for h in 0..6 {
                let buffer = render(&viewer, w, h);
                assert!((0..buffer.height()).all(|y| row_text(&buffer, y).is_empty()));
            }
        }
    }
}

#[test]
fn test_json_viewer_empty_ignores_every_action() {
    let mut viewer = JsonViewer::new();
    viewer.select_down();
    viewer.select_up();
    viewer.select_first();
    viewer.select_last();
    viewer.page_down(10);
    viewer.page_up(10);
    viewer.toggle();
    viewer.expand();
    viewer.collapse();
    viewer.expand_all();
    viewer.collapse_all();
    viewer.search("x");
    viewer.next_match();
    viewer.prev_match();
    assert_eq!(viewer.match_count(), 0);
    viewer.clear_search();
    assert_eq!(viewer.selected_index(), 0);
    assert_eq!(viewer.selected_path(), None);
    assert_eq!(viewer.selected_value(), None);
    assert_eq!(viewer.root_type(), None);
    assert_eq!(viewer.root_children_count(), 0);
}

#[test]
fn test_json_viewer_scalar_root_is_one_row() {
    let viewer = JsonViewer::from_content("42").show_line_numbers(false);
    assert_eq!(viewer.visible_count(), 1);
    assert_eq!(row_text(&render(&viewer, 20, 3), 0), "42");
}

// =========================================================================
// Edges: the row layout and colors
// =========================================================================

#[test]
fn test_json_viewer_rows_show_key_and_value_of_each_type() {
    let viewer = JsonViewer::from_content(DOC);
    let buffer = render(&viewer, 40, 8);
    let rows: Vec<String> = (0..7).map(|y| row_text(&buffer, y)).collect();
    assert_eq!(rows[0], " 1 ▼ {");
    assert_eq!(rows[1], r#" 2   "s": "x""#);
    assert_eq!(rows[2], r#" 3   "n": 1"#);
    assert_eq!(rows[3], r#" 4   "b": true"#);
    assert_eq!(rows[4], r#" 5   "z": null"#);
    assert_eq!(rows[5], r#" 6   ▼ "o": {}"#);
    assert_eq!(rows[6], r#" 7   ▼ "l": []"#);
}

#[test]
fn test_json_viewer_tokens_take_their_type_colors() {
    let viewer = JsonViewer::from_content(DOC).selected_style(Color::WHITE, Color::BLUE);
    let buffer = render(&viewer, 40, 8);
    let fg_at = |y: u16, ch: char| {
        buffer
            .get(column_of(&buffer, y, ch).unwrap(), y)
            .unwrap()
            .fg
    };
    assert_eq!(fg_at(1, 's'), Some(Color::CYAN), "key");
    assert_eq!(fg_at(1, 'x'), Some(Color::GREEN), "string");
    assert_eq!(fg_at(2, '1'), Some(Color::YELLOW), "number");
    assert_eq!(fg_at(3, 't'), Some(Color::MAGENTA), "boolean");
}

#[test]
fn test_json_viewer_collapsed_container_shows_a_summary() {
    let mut viewer = JsonViewer::from_content(DOC).show_line_numbers(false);
    viewer.toggle();
    assert_eq!(viewer.visible_count(), 1);
    assert_eq!(row_text(&render(&viewer, 40, 3), 0), "▶ {...} (6 items)");

    let mut array = JsonViewer::from_content("[1,2,3]").show_line_numbers(false);
    array.toggle();
    assert_eq!(row_text(&render(&array, 40, 3), 0), "▶ [...] (3 items)");
}

#[test]
fn test_json_viewer_type_badges_follow_the_value() {
    let viewer = JsonViewer::from_content(DOC)
        .show_line_numbers(false)
        .show_type_badges(true);
    let buffer = render(&viewer, 40, 8);
    assert!(row_text(&buffer, 0).ends_with(" object"));
    assert!(row_text(&buffer, 1).ends_with(r#""x" string"#));
    assert!(row_text(&buffer, 4).ends_with("null null"));
    assert!(row_text(&buffer, 6).ends_with("[] array"));
}

#[test]
fn test_json_viewer_selected_row_and_matches_are_colored_across_the_width() {
    let mut viewer = JsonViewer::from_content(DOC)
        .selected_style(Color::WHITE, Color::BLUE)
        .match_style(Color::BLACK, Color::YELLOW);
    viewer.search("n");
    let buffer = render(&viewer, 30, 8);
    let selected = viewer.selected_index() as u16;
    assert!((3..30).all(|x| buffer.get(x, selected).unwrap().bg == Some(Color::BLUE)));
    for y in 0..7u16 {
        if y == selected {
            continue;
        }
        let lit = buffer.get(29, y).unwrap().bg == Some(Color::YELLOW);
        let text = row_text(&buffer, y);
        // "n" matches the key `n`; a null value is not searched
        assert_eq!(lit, text.contains("\"n\""), "{text:?}");
    }
}

#[test]
fn test_json_viewer_long_string_is_cut_to_the_content_width() {
    let viewer =
        JsonViewer::from_content(r#""abcdefghijklmnopqrstuvwxyz""#).show_line_numbers(false);
    // The root has no key and no indent: the quoted string gets 12 - 2 columns
    assert_eq!(row_text(&render(&viewer, 12, 3), 0), "\"abcdefghi");
}

#[test]
fn test_json_viewer_line_number_gutter_grows_with_the_rows() {
    let many = format!("[{}]", vec!["0"; 120].join(","));
    let viewer = JsonViewer::from_content(&many);
    let buffer = render(&viewer, 20, 3);
    assert_eq!(row_text(&buffer, 0), "  1 ▼ [");
}

// =========================================================================
// Matrix
// =========================================================================

#[derive(Debug, Clone)]
enum Value {
    Num(u32),
    Str(String),
    Bool(bool),
    Null,
    Arr(Vec<Value>),
    Obj(Vec<Value>),
}

fn value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        any::<u32>().prop_map(Value::Num),
        "[a-z ]{0,8}".prop_map(Value::Str),
        any::<bool>().prop_map(Value::Bool),
        Just(Value::Null),
    ];
    leaf.prop_recursive(3, 30, 4, |inner| {
        prop_oneof![
            prop::collection::vec(inner.clone(), 0..4).prop_map(Value::Arr),
            prop::collection::vec(inner, 0..4).prop_map(Value::Obj),
        ]
    })
}

/// JSON text; object keys are k0, k1, ... so every path is distinct
fn to_json(v: &Value) -> String {
    match v {
        Value::Num(n) => n.to_string(),
        Value::Str(s) => format!("\"{s}\""),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".into(),
        Value::Arr(items) => format!(
            "[{}]",
            items.iter().map(to_json).collect::<Vec<_>>().join(",")
        ),
        Value::Obj(items) => format!(
            "{{{}}}",
            items
                .iter()
                .enumerate()
                .map(|(i, v)| format!("\"k{i}\":{}", to_json(v)))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn node_count(v: &Value) -> usize {
    match v {
        Value::Arr(items) | Value::Obj(items) => 1 + items.iter().map(node_count).sum::<usize>(),
        _ => 1,
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Expanded, every node is a row; collapsed, only the root. When every
    /// row fits, row y carries line number y + 1 and only the selected row
    /// has the selection background.
    #[test]
    fn rows_match_the_document(doc in value(), moves in 0usize..30, badges in any::<bool>()) {
        let json = to_json(&doc);
        let mut viewer = JsonViewer::from_content(&json)
            .show_type_badges(badges)
            .selected_style(Color::WHITE, Color::BLUE);
        let total = node_count(&doc);
        prop_assert_eq!(viewer.visible_count(), total, "{}", json);

        for _ in 0..moves {
            viewer.select_down();
        }
        prop_assert!(viewer.selected_index() < total);

        let height = (total as u16).max(2);
        let buffer = render(&viewer, 80, height);
        let digits = total.max(1).to_string().len().max(2);
        for y in 0..total as u16 {
            let number: String = (0..digits as u16).map(|x| buffer.get(x, y).unwrap().symbol).collect();
            prop_assert_eq!(number.trim(), (y + 1).to_string());
        }
        let lit: Vec<u16> = (0..height)
            .filter(|&y| buffer.get(79, y).unwrap().bg == Some(Color::BLUE))
            .collect();
        prop_assert_eq!(lit, vec![viewer.selected_index() as u16]);

        viewer.collapse_all();
        prop_assert_eq!(viewer.visible_count(), 1);
        viewer.expand_all();
        prop_assert_eq!(viewer.visible_count(), total);
    }

    /// Any document, options and area: rendering never draws outside it
    #[test]
    fn render_stays_inside_its_area(
        doc in value(),
        moves in 0usize..10,
        indent in 0u16..6,
        numbers in any::<bool>(),
        w in 0u16..40,
        h in 0u16..8,
    ) {
        let mut viewer = JsonViewer::from_content(&to_json(&doc))
            .indent_size(indent)
            .show_line_numbers(numbers);
        for _ in 0..moves {
            viewer.select_down();
        }
        let sentinel = '#';
        let mut buffer = Buffer::new(w + 4, h + 4);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                buffer.set(x, y, revue::render::Cell::new(sentinel));
            }
        }
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 2, w, h));
        viewer.render(&mut ctx);
        for y in 0..h + 4 {
            for x in 0..w + 4 {
                let inside = (2..2 + w).contains(&x) && (2..2 + h).contains(&y);
                if !inside {
                    prop_assert_eq!(buffer.get(x, y).unwrap().symbol, sentinel, "({}, {})", x, y);
                }
            }
        }
    }
}

// =========================================================================
// Cross-module
// =========================================================================

/// Inside a vstack with a sibling, through the stylesheet and DOM path: a
/// plain row takes `background`, the selected row keeps its highlight, and
/// the sibling stays below the viewer's box.
#[test]
fn test_json_viewer_in_a_stack_with_a_stylesheet() {
    use revue::prelude::*;
    use revue::testing::PipelineHarness;

    struct Screen;
    impl View for Screen {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    JsonViewer::from_content(r#"{"a":1,"b":2}"#)
                        .selected_style(Color::WHITE, Color::BLUE)
                        .element_id("json"),
                )
                .child(Text::new("footer"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Screen"
        }
    }

    let paper = Color::rgb(200, 210, 220);
    let mut h = PipelineHarness::with_css("#json { background: #c8d2dc; height: 3; }", 30, 5)
        .dom_from_render(true);
    h.draw(&Screen);
    let buffer = h.buffer();
    let shown: Vec<String> = (0..5).map(|y| row_text(buffer, y)).collect();
    assert_eq!(shown[1], r#" 2   "a": 1"#, "{shown:#?}");
    assert_eq!(shown[3], "footer", "{shown:#?}");
    assert_eq!(buffer.get(29, 0).unwrap().bg, Some(Color::BLUE), "selected");
    assert_eq!(buffer.get(29, 1).unwrap().bg, Some(paper), "plain row");
}
