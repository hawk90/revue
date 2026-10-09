//! What a split remembers between frames

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use super::layout::{self, PaneSpec};
use crate::event::{Key, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::utils::lock::lock_or_recover;
use crate::widget::layout::splitter::SplitOrientation;

/// The state of a [`SplitView`](super::SplitView): pane sizes the user has
/// dragged, which panes are collapsed, and the divider being moved.
///
/// The app keeps it (as a field of its view, say) and builds the
/// `SplitView` from it every frame. Sizes and collapsed panes are keyed by
/// pane id, so they hold when panes come and go between frames.
///
/// It is a handle: clones share one state, as [`Signal`](crate::reactive::Signal)
/// clones share a value. The view keeps a clone, so it does not borrow the
/// app and can sit in any container.
///
/// It also remembers where the split was last drawn, so
/// [`handle_mouse`](Self::handle_mouse) and [`handle_key`](Self::handle_key)
/// need no area: they act on the last frame. Before the first frame they
/// take nothing.
#[derive(Clone, Debug, Default)]
pub struct SplitState {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug, Default)]
struct Inner {
    /// Weights set by resizing, by pane id
    weights: HashMap<String, f32>,
    /// Collapsed or expanded by the app, by pane id
    collapsed: HashMap<String, bool>,
    /// The divider being moved, counted among the visible panes
    resizing: Option<usize>,
    /// The last frame
    frame: Option<Frame>,
}

/// Where and how a split was last drawn
#[derive(Clone, Debug)]
pub(super) struct Frame {
    pub orientation: SplitOrientation,
    pub area: Rect,
    /// Every pane, collapsed or not, in order
    pub panes: Vec<FramePane>,
}

/// A pane as declared in the last frame
#[derive(Clone, Debug)]
pub(super) struct FramePane {
    pub id: String,
    /// The pane's declared weight, before any resizing
    pub weight: f32,
    pub min: u16,
    pub max: u16,
    /// Declared collapsed (`Pane::collapsed`)
    pub collapsed: bool,
}

/// The visible panes of a frame, laid out
pub(super) struct Placed {
    /// Index into `Frame::panes`
    pub index: usize,
    pub spec: PaneSpec,
    /// Start along the split axis, in buffer coordinates
    pub start: u16,
    pub size: u16,
}

impl Frame {
    /// The extent along the split axis and where it starts
    fn axis(&self) -> (u16, u16) {
        match self.orientation {
            SplitOrientation::Horizontal => (self.area.x, self.area.width),
            SplitOrientation::Vertical => (self.area.y, self.area.height),
        }
    }
}

impl SplitState {
    /// A split with every pane at its declared size
    pub fn new() -> Self {
        Self::default()
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        lock_or_recover(&self.inner)
    }

    /// Whether pane `id` is collapsed: as set by
    /// [`set_collapsed`](Self::set_collapsed) or [`toggle`](Self::toggle),
    /// else as declared (`Pane::collapsed`) in the last frame
    pub fn is_collapsed(&self, id: &str) -> bool {
        self.lock().is_collapsed(id)
    }

    /// Collapse or expand pane `id`. A collapsed pane is not drawn and its
    /// room goes to the others. Stops a resize in progress, since the
    /// dividers change.
    pub fn set_collapsed(&mut self, id: impl Into<String>, collapsed: bool) {
        self.lock().set_collapsed(id.into(), collapsed);
    }

    /// Collapse pane `id` if it is expanded, expand it if collapsed
    pub fn toggle(&mut self, id: &str) {
        let mut inner = self.lock();
        let collapsed = inner.is_collapsed(id);
        inner.set_collapsed(id.to_string(), !collapsed);
    }

    /// Forget resized sizes and collapsed panes, back to what the panes
    /// declare
    pub fn reset(&mut self) {
        let mut inner = self.lock();
        inner.weights.clear();
        inner.collapsed.clear();
        inner.resizing = None;
    }

    /// Whether a divider is being moved, by mouse or keyboard
    pub fn is_resizing(&self) -> bool {
        self.lock().resizing.is_some()
    }

    /// The divider being moved: 0 is the one after the first visible pane
    pub fn resizing(&self) -> Option<usize> {
        self.lock().resizing
    }

    /// Start moving `divider` (0 is the one after the first visible pane)
    /// with the arrow keys; see [`handle_key`](Self::handle_key). Does
    /// nothing if the last frame has no such divider.
    pub fn start_resize(&mut self, divider: usize) {
        let mut inner = self.lock();
        if divider + 1 < inner.placed().len() {
            inner.resizing = Some(divider);
        }
    }

    /// Stop moving the divider
    pub fn stop_resize(&mut self) {
        self.lock().resizing = None;
    }

    /// While a divider is being moved, the arrow keys (and `h` `j` `k` `l`)
    /// move it a cell, and Enter or Escape stop. Returns whether the key was
    /// used; other keys, and every key when no divider is being moved, are
    /// left for the app.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        self.lock().handle_key(key)
    }

    /// Pressing the left button on a divider picks it up, dragging moves
    /// it to the pointer (within the panes' bounds), and releasing drops
    /// it. Returns whether the event was used.
    pub fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        self.lock().handle_mouse(event)
    }

    /// Record the frame about to be drawn; returns its visible panes, laid
    /// out with the current sizes, and the divider being moved
    pub(super) fn record(&self, frame: Frame) -> (Vec<Placed>, Option<usize>) {
        let mut inner = self.lock();
        inner.frame = Some(frame);
        (inner.placed(), inner.resizing)
    }
}

impl Inner {
    fn is_collapsed(&self, id: &str) -> bool {
        self.collapsed.get(id).copied().unwrap_or_else(|| {
            self.frame
                .as_ref()
                .is_some_and(|f| f.panes.iter().any(|p| p.id == id && p.collapsed))
        })
    }

    fn set_collapsed(&mut self, id: String, collapsed: bool) {
        self.collapsed.insert(id, collapsed);
        self.resizing = None;
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        let Some(divider) = self.resizing else {
            return false;
        };
        let step: i32 = match key {
            Key::Left | Key::Up | Key::Char('h') | Key::Char('k') => -1,
            Key::Right | Key::Down | Key::Char('l') | Key::Char('j') => 1,
            Key::Enter | Key::Escape => {
                self.resizing = None;
                return true;
            }
            _ => return false,
        };
        let placed = self.placed();
        if let Some(pane) = placed.get(divider) {
            let at = i32::from(pane.start) + i32::from(pane.size) + step;
            self.move_divider(divider, at.clamp(0, i32::from(u16::MAX)) as u16);
        }
        true
    }

    fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        let Some(frame) = self.frame.as_ref() else {
            return false;
        };
        let (along, across, cross_start, cross_len) = match frame.orientation {
            SplitOrientation::Horizontal => (event.x, event.y, frame.area.y, frame.area.height),
            SplitOrientation::Vertical => (event.y, event.x, frame.area.x, frame.area.width),
        };
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let inside = across >= cross_start
                    && u32::from(across) < u32::from(cross_start) + u32::from(cross_len);
                let placed = self.placed();
                let hit = placed
                    .iter()
                    .take(placed.len().saturating_sub(1))
                    .position(|p| u32::from(p.start) + u32::from(p.size) == u32::from(along));
                match hit.filter(|_| inside) {
                    Some(divider) => {
                        self.resizing = Some(divider);
                        true
                    }
                    None => false,
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => match self.resizing {
                Some(divider) => {
                    self.move_divider(divider, along);
                    true
                }
                None => false,
            },
            MouseEventKind::Up(MouseButton::Left) if self.resizing.is_some() => {
                self.resizing = None;
                true
            }
            _ => false,
        }
    }

    /// The last frame's visible panes, laid out with the current sizes
    fn placed(&self) -> Vec<Placed> {
        let Some(frame) = self.frame.as_ref() else {
            return Vec::new();
        };
        let visible: Vec<(usize, PaneSpec)> = frame
            .panes
            .iter()
            .enumerate()
            .filter(|(_, p)| !self.collapsed.get(&p.id).copied().unwrap_or(p.collapsed))
            .map(|(i, p)| {
                let spec = PaneSpec {
                    weight: self.weights.get(&p.id).copied().unwrap_or(p.weight),
                    min: p.min,
                    max: p.max,
                };
                (i, spec)
            })
            .collect();
        let (start, extent) = frame.axis();
        let dividers = u16::try_from(visible.len().saturating_sub(1)).unwrap_or(u16::MAX);
        let specs: Vec<PaneSpec> = visible.iter().map(|(_, s)| *s).collect();
        let sizes = layout::sizes(&specs, extent.saturating_sub(dividers));

        let mut at = start;
        visible
            .into_iter()
            .zip(sizes)
            .map(|((index, spec), size)| {
                let placed = Placed {
                    index,
                    spec,
                    start: at,
                    size,
                };
                at = at.saturating_add(size).saturating_add(1);
                placed
            })
            .collect()
    }

    /// Move `divider` so it sits at `at` along the split axis, as far as
    /// the bounds of the panes on either side allow
    fn move_divider(&mut self, divider: usize, at: u16) {
        let placed = self.placed();
        let (Some(before), Some(after)) = (placed.get(divider), placed.get(divider + 1)) else {
            return;
        };
        let Some(frame) = self.frame.as_ref() else {
            return;
        };
        let target = at
            .saturating_sub(before.start)
            .min(before.size + after.size);
        let specs: Vec<PaneSpec> = placed.iter().map(|p| p.spec).collect();
        let available = placed
            .iter()
            .map(|p| p.size)
            .fold(0u16, u16::saturating_add);
        let weight = layout::weight_for(&specs, available, divider, target);
        let pair = before.spec.weight.max(0.0) + after.spec.weight.max(0.0);

        let ids = (
            frame.panes[before.index].id.clone(),
            frame.panes[after.index].id.clone(),
        );
        self.weights.insert(ids.0, weight);
        self.weights.insert(ids.1, pair - weight);
    }
}
