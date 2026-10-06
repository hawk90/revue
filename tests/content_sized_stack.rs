//! `Stack::content_sized` - unsized children take the size of their content.
//!
//! Without it, a stack gives every child added with `.child(...)` an equal
//! share of the space. Twenty of the shipped examples were written as if the
//! opposite were true. Background: `docs/refactor/design-content-sized-stack.md`.

use revue::prelude::*;
use revue::testing::PipelineHarness;
use revue::widget::{card, Alert, Switch};

/// A view that fills whatever it is given and does not measure - the default
/// for every widget that has not learned to answer.
struct Fill(&'static str);

impl View for Fill {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        for y in 0..area.height {
            ctx.draw_text(0, y, self.0, Color::WHITE);
        }
    }
}

fn rows(view: &impl View, height: u16) -> Vec<String> {
    let mut h = PipelineHarness::new(20, height);
    h.draw(view);
    h.screen_text()
        .lines()
        .map(|l| l.trim_end().to_string())
        .collect()
}

fn three_lines() -> Stack {
    vstack()
        .child(Text::new("a"))
        .child(Text::new("b"))
        .child(Text::new("c"))
}

/// The 2.x rule, kept while the flag is off: equal shares.
#[test]
fn without_the_flag_children_share_equally() {
    let screen = rows(&three_lines(), 9);
    assert_eq!(screen[0], "a");
    assert_eq!(screen[3], "b");
    assert_eq!(screen[6], "c");
}

#[test]
fn a_column_stacks_text_line_after_line() {
    let screen = rows(&three_lines().content_sized(true), 9);
    assert_eq!(&screen[..3], ["a", "b", "c"]);
}

#[test]
fn a_row_packs_text_side_by_side() {
    let view = hstack()
        .content_sized(true)
        .child(Text::new("12 "))
        .child(Text::new("fn main()"));
    assert_eq!(rows(&view, 1)[0], "12 fn main()");
}

/// The shape most examples have: a header, a body that fills, a footer.
#[test]
fn a_child_that_fills_gets_what_the_measured_ones_leave() {
    let view = vstack()
        .content_sized(true)
        .child(Text::new("header"))
        .child(Fill("body"))
        .child(Text::new("footer"));
    let screen = rows(&view, 6);
    assert_eq!(screen, ["header", "body", "body", "body", "body", "footer"]);
}

#[test]
fn an_explicit_size_still_wins() {
    let view = vstack()
        .content_sized(true)
        .child_sized(Text::new("a"), 3)
        .child(Text::new("b"));
    let screen = rows(&view, 6);
    assert_eq!(screen[0], "a");
    assert_eq!(screen[3], "b");
}

#[test]
fn a_border_is_its_content_plus_the_frame() {
    let view = vstack()
        .content_sized(true)
        .child(Border::single().child(Text::new("inside")))
        .child(Text::new("after"));
    let screen = rows(&view, 8);
    assert!(screen[1].contains("inside"), "{screen:?}");
    assert!(screen[2].starts_with('└'), "{screen:?}");
    assert_eq!(screen[3], "after");
}

#[test]
fn a_nested_stack_is_the_sum_of_its_children() {
    let view = vstack()
        .content_sized(true)
        .child(three_lines())
        .child(Text::new("d"));
    // The inner stack is not content-sized itself, so it spreads its three
    // lines over the three rows it measured - which is one each.
    assert_eq!(&rows(&view, 9)[..4], ["a", "b", "c", "d"]);
}

#[test]
fn measure_answers() {
    assert_eq!(Text::new("hello").measure(80, 24), Some((5, 1)));
    assert_eq!(Text::new("hello").measure(3, 24), Some((3, 1)));
    assert_eq!(Text::new("").measure(80, 24), Some((0, 1)));
    assert_eq!(
        Border::single().child(Text::new("hi")).measure(80, 24),
        Some((4, 3))
    );
    assert_eq!(three_lines().measure(80, 24), Some((1, 3)));
    assert_eq!(vstack().child(Fill("x")).measure(80, 24), None);
    assert_eq!(Fill("x").measure(80, 24), None);
}

// ==================== Widgets that measure ====================
//
// Before these widgets answered `measure`, each one claimed all the space the
// measured children left, pushing the next sibling to the far edge.

#[test]
fn a_switch_and_its_label_sit_side_by_side() {
    let view = hstack()
        .content_sized(true)
        .child(Switch::new())
        .child(Text::new("Wi-Fi"));
    assert_eq!(rows(&view, 1)[0], "[●━━━]Wi-Fi");
}

#[test]
fn a_form_column_packs_its_controls() {
    let view = vstack()
        .content_sized(true)
        .child(Text::new("Name"))
        .child(Input::new().value("Ada"))
        .child(Checkbox::new("Subscribe"))
        .child(Button::new("Send"));
    let screen = rows(&view, 10);
    assert_eq!(screen[0], "Name");
    assert!(screen[1].starts_with("Ada"), "{screen:?}");
    assert_eq!(screen[2], "[ ] Subscribe");
    assert!(screen[3].contains("Send"), "{screen:?}");
    assert!(screen[4..].iter().all(|r| r.is_empty()), "{screen:?}");
}

#[test]
fn buttons_in_a_row_sit_next_to_each_other() {
    let view = hstack()
        .content_sized(true)
        .gap(1)
        .child(Button::new("OK"))
        .child(Button::new("Cancel"));
    assert_eq!(rows(&view, 1)[0], "  OK     Cancel");
}

#[test]
fn a_divider_between_two_lines_is_one_row() {
    let view = vstack()
        .content_sized(true)
        .child(Text::new("above"))
        .child(Divider::new())
        .child(Text::new("below"));
    let screen = rows(&view, 9);
    assert_eq!(screen[0], "above");
    assert_eq!(screen[1], "─".repeat(20));
    assert_eq!(screen[2], "below");
}

#[test]
fn a_spinner_and_its_label_sit_side_by_side() {
    let view = hstack()
        .content_sized(true)
        .child(Spinner::new())
        .child(Text::new(" Loading"));
    assert_eq!(rows(&view, 1)[0], "⠋ Loading");
}

#[test]
fn an_alert_and_a_card_take_their_rows() {
    let view = vstack()
        .content_sized(true)
        .child(Alert::new("Saved"))
        .child(card().title("Card").body(Text::new("body")))
        .child(Text::new("after"));
    let screen = rows(&view, 16);
    // A filled alert is its message between two border rows.
    assert!(screen[1].contains("Saved"), "{screen:?}");
    // The card: frame, title, separator, body, frame.
    assert!(screen[4].contains("Card"), "{screen:?}");
    assert!(screen[6].contains("body"), "{screen:?}");
    assert_eq!(screen[8], "after");
}
