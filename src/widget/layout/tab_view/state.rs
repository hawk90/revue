//! What a tab view remembers between frames

use std::cell::RefCell;

use crate::event::{Key, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;

/// The state of a [`TabView`](super::TabView): which tab is selected.
///
/// The app keeps it and builds the `TabView` from it every frame. The
/// selection is held by tab id, so it stays on its tab when tabs are added,
/// moved or closed around it. When the selected tab itself is closed, the
/// tab that moves into its place is selected (or the new last tab).
///
/// It also remembers the last frame's tabs and where their labels were
/// drawn, so [`handle_key`](Self::handle_key) and
/// [`handle_mouse`](Self::handle_mouse) need no area. Before the first frame
/// they take nothing.
#[derive(Clone, Debug, Default)]
pub struct TabState {
    /// The selected tab's id, if one was selected. Drawing a frame without
    /// it moves it to the tab in its place.
    selected: RefCell<Option<String>>,
    /// The last frame
    frame: RefCell<Option<Frame>>,
}

/// The tabs of a frame and where the bar was drawn
#[derive(Clone, Debug)]
pub(super) struct Frame {
    pub ids: Vec<String>,
    /// Index of the tab shown
    pub shown: usize,
    /// The bar's row, in buffer coordinates
    pub bar: Rect,
    /// Columns of each label, relative to `bar.x`
    pub spans: Vec<(u16, u16)>,
}

impl TabState {
    /// No tab selected yet: the first tab is shown
    pub fn new() -> Self {
        Self::default()
    }

    /// The id of the tab shown: the one selected, if the last frame had it;
    /// else the one the last frame showed in its place
    pub fn selected(&self) -> Option<String> {
        let frame = self.frame.borrow();
        match (self.selected.borrow().as_ref(), frame.as_ref()) {
            (Some(id), None) => Some(id.clone()),
            (Some(id), Some(f)) if f.ids.contains(id) => Some(id.clone()),
            (_, Some(f)) => f.ids.get(f.shown).cloned(),
            (None, None) => None,
        }
    }

    /// Select the tab with this id
    pub fn select(&mut self, id: impl Into<String>) {
        *self.selected.get_mut() = Some(id.into());
    }

    /// Select the next tab, wrapping to the first. Returns whether the
    /// selection changed.
    pub fn select_next(&mut self) -> bool {
        self.step(|i, n| (i + 1) % n)
    }

    /// Select the previous tab, wrapping to the last. Returns whether the
    /// selection changed.
    pub fn select_prev(&mut self) -> bool {
        self.step(|i, n| (i + n - 1) % n)
    }

    /// Select the tab at `index` in the last frame. Returns whether the
    /// selection changed; `false` too if there is no such tab.
    pub fn select_index(&mut self, index: usize) -> bool {
        self.step(|_, n| if index < n { index } else { usize::MAX })
    }

    /// Left/Right (and `h`/`l`) select the previous/next tab, Home/End the
    /// first/last, `1`-`9` that tab. Returns whether the key was used to
    /// change the selection. Call it while the tab bar has the keyboard;
    /// the selected tab's widget may want these keys otherwise.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Left | Key::Char('h') => self.select_prev(),
            Key::Right | Key::Char('l') => self.select_next(),
            Key::Home => self.select_index(0),
            Key::End => self.step(|_, n| n - 1),
            Key::Char(c @ '1'..='9') => self.select_index(*c as usize - '1' as usize),
            _ => false,
        }
    }

    /// A left click on a tab's label selects it. Returns whether the click
    /// was on a label; clicks elsewhere, the body included, are left alone.
    pub fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        if event.kind != MouseEventKind::Down(MouseButton::Left) {
            return false;
        }
        let hit = {
            let frame = self.frame.borrow();
            let Some(frame) = frame.as_ref() else {
                return false;
            };
            let bar = frame.bar;
            let on_bar = bar.height > 0
                && event.y == bar.y
                && event.x >= bar.x
                && u32::from(event.x) < u32::from(bar.x) + u32::from(bar.width);
            on_bar
                .then(|| {
                    let x = event.x - bar.x;
                    frame.spans.iter().position(|&(s, e)| x >= s && x < e)
                })
                .flatten()
        };
        match hit {
            Some(index) => {
                self.select_index(index);
                true
            }
            None => false,
        }
    }

    /// Record the frame about to be drawn with `ids`; returns the index of
    /// the tab to show
    pub(super) fn record(&self, ids: Vec<String>, bar: Rect, spans: Vec<(u16, u16)>) -> usize {
        let mut frame = self.frame.borrow_mut();
        let mut selected = self.selected.borrow_mut();
        let shown = match selected.as_ref() {
            Some(id) => ids.iter().position(|t| t == id).unwrap_or_else(|| {
                // The selected tab is gone: the one now in its place
                let was = frame.as_ref().map_or(0, |f| f.shown);
                was.min(ids.len().saturating_sub(1))
            }),
            None => 0,
        };
        if selected.is_some() {
            *selected = ids.get(shown).cloned();
        }
        *frame = Some(Frame {
            ids,
            shown,
            bar,
            spans,
        });
        shown
    }

    /// Select the tab `pick(shown, count)` of the last frame; an index past
    /// the end selects nothing
    fn step(&mut self, pick: impl Fn(usize, usize) -> usize) -> bool {
        let next = {
            let frame = self.frame.borrow();
            let Some(frame) = frame.as_ref().filter(|f| !f.ids.is_empty()) else {
                return false;
            };
            let current = self.shown_index(frame);
            let index = pick(current, frame.ids.len());
            match frame.ids.get(index) {
                Some(id) if index != current => id.clone(),
                _ => return false,
            }
        };
        *self.selected.get_mut() = Some(next);
        true
    }

    /// Index of the selected tab in `frame`, or of the one it showed
    fn shown_index(&self, frame: &Frame) -> usize {
        self.selected
            .borrow()
            .as_ref()
            .and_then(|id| frame.ids.iter().position(|t| t == id))
            .unwrap_or(frame.shown)
    }
}
