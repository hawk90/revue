//! A widget that can hold focus shows that it has it (#1150, REV-UX-003), either
//! through its own focused flag or through the runtime's state on its node.

use revue::dom::NodeState;
use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Style;
use revue::widget::traits::{RenderContext, View};

use super::catalog::catalog;
use super::Content;

const AREA: (u16, u16) = (40, 12);

fn cells(view: &dyn View, node_focused: bool) -> Vec<Cell> {
    let (w, h) = AREA;
    let mut buffer = Buffer::new(w, h);
    let style = Style::default();
    let state = NodeState {
        focused: node_focused,
        ..NodeState::default()
    };
    let mut ctx = RenderContext::full(&mut buffer, Rect::new(0, 0, w, h), &style, &state);
    view.render(&mut ctx);
    (0..h)
        .flat_map(|y| (0..w).map(move |x| (x, y)))
        .map(|(x, y)| *buffer.get(x, y).unwrap())
        .collect()
}

#[test]
fn a_focusable_widget_shows_its_focus() {
    let content = Content::ordinary();
    let mut hidden = Vec::new();
    let mut count = 0;
    for entry in &catalog() {
        let plain = (entry.build)(&content, false);
        if !plain.meta().focusable {
            continue;
        }
        count += 1;
        let at_rest = cells(&*plain, false);
        let by_state = cells(&*plain, true);
        let by_flag = cells(&*(entry.build)(&content, true), false);
        if by_state == at_rest && by_flag == at_rest {
            hidden.push(entry.name);
        }
    }
    assert!(count >= 16, "only {count} focusable widgets");
    assert!(hidden.is_empty(), "focus does not show: {hidden:?}");
}

/// A widget that reacts to focus can get it: Tab does not skip what shows focus.
///
/// Some widgets draw the time or the machine's load, and can change between any
/// two renders or instances, so a difference proves nothing about those: a widget
/// counts only if it draws the same at rest every time, built again too, and
/// differs with focus every time.
#[test]
fn a_widget_that_shows_focus_is_focusable() {
    const TRIES: usize = 5;
    let content = Content::ordinary();
    let mut unreachable = Vec::new();
    for entry in &catalog() {
        let plain = (entry.build)(&content, false);
        if plain.meta().focusable {
            continue;
        }
        let reacts = |focused: &dyn Fn() -> Vec<Cell>| {
            (0..TRIES).all(|_| {
                let before = cells(&*(entry.build)(&content, false), false);
                let drawn = focused();
                let twin = cells(&*(entry.build)(&content, false), false);
                before == cells(&*plain, false) && before == twin && drawn != before
            })
        };
        // Built fresh each time, as the one it is compared with is: an instance
        // that draws the time differs from one made earlier.
        let flagged = || cells(&*(entry.build)(&content, true), false);
        if reacts(&|| cells(&*plain, true)) || reacts(&flagged) {
            unreachable.push(entry.name);
        }
    }
    assert!(
        unreachable.is_empty(),
        "shows focus but Tab skips it: {unreachable:?}"
    );
}
