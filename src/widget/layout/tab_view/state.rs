//! What a tab view remembers between frames

use std::sync::{Arc, Mutex, MutexGuard};

use crate::event::{Key, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::utils::lock::lock_or_recover;

/// The state of a [`TabView`](super::TabView): which tab is selected.
///
/// The app keeps it and builds the `TabView` from it every frame. The
/// selection is held by tab id, so it stays on its tab when tabs are added,
/// moved or closed around it. When the selected tab itself is closed, the
/// tab that moves into its place is selected (or the new last tab).
///
/// It is a handle: clones share one state, as [`Signal`](crate::reactive::Signal)
/// clones share a value. The view keeps a clone, so it does not borrow the
/// app and can sit in any container.
///
/// It also remembers the last frame's tabs and where their labels were
/// drawn, so [`handle_key`](Self::handle_key) and
/// [`handle_mouse`](Self::handle_mouse) need no area. Before the first frame
/// they take nothing.
#[derive(Clone, Debug, Default)]
pub struct TabState {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug, Default)]
struct Inner {
    /// The selected tab's id, if one was selected. Drawing a frame without
    /// it moves it to the tab in its place.
    selected: Option<String>,
    /// The last frame
    frame: Option<Frame>,
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

    fn lock(&self) -> MutexGuard<'_, Inner> {
        lock_or_recover(&self.inner)
    }

    /// The id of the tab shown: the one selected, if the last frame had it;
    /// else the one the last frame showed in its place
    pub fn selected(&self) -> Option<String> {
        let inner = self.lock();
        match (inner.selected.as_ref(), inner.frame.as_ref()) {
            (Some(id), None) => Some(id.clone()),
            (Some(id), Some(f)) if f.ids.contains(id) => Some(id.clone()),
            (_, Some(f)) => f.ids.get(f.shown).cloned(),
            (None, None) => None,
        }
    }

    /// Select the tab with this id
    pub fn select(&mut self, id: impl Into<String>) {
        self.lock().selected = Some(id.into());
    }

    /// Select the next tab, wrapping to the first. Returns whether the
    /// selection changed.
    pub fn select_next(&mut self) -> bool {
        self.lock().step(|i, n| (i + 1) % n)
    }

    /// Select the previous tab, wrapping to the last. Returns whether the
    /// selection changed.
    pub fn select_prev(&mut self) -> bool {
        self.lock().step(|i, n| (i + n - 1) % n)
    }

    /// Select the tab at `index` in the last frame. Returns whether the
    /// selection changed; `false` too if there is no such tab.
    pub fn select_index(&mut self, index: usize) -> bool {
        self.lock()
            .step(|_, n| if index < n { index } else { usize::MAX })
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
            Key::End => self.lock().step(|_, n| n - 1),
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
            let inner = self.lock();
            let Some(frame) = inner.frame.as_ref() else {
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
        let mut inner = self.lock();
        let shown = match inner.selected.as_ref() {
            Some(id) => ids.iter().position(|t| t == id).unwrap_or_else(|| {
                // The selected tab is gone: the one now in its place
                let was = inner.frame.as_ref().map_or(0, |f| f.shown);
                was.min(ids.len().saturating_sub(1))
            }),
            None => 0,
        };
        if inner.selected.is_some() {
            inner.selected = ids.get(shown).cloned();
        }
        inner.frame = Some(Frame {
            ids,
            shown,
            bar,
            spans,
        });
        shown
    }
}

impl Inner {
    /// Select the tab `pick(shown, count)` of the last frame; an index past
    /// the end selects nothing
    fn step(&mut self, pick: impl Fn(usize, usize) -> usize) -> bool {
        let Some(frame) = self.frame.as_ref().filter(|f| !f.ids.is_empty()) else {
            return false;
        };
        // The selected tab's index in the frame, or the one it showed
        let current = self
            .selected
            .as_ref()
            .and_then(|id| frame.ids.iter().position(|t| t == id))
            .unwrap_or(frame.shown);
        let index = pick(current, frame.ids.len());
        match frame.ids.get(index) {
            Some(id) if index != current => {
                self.selected = Some(id.clone());
                true
            }
            _ => false,
        }
    }
}
