//! Split view: panes that hold widgets, with dividers the user can drag
//!
//! Unlike [`Splitter`](super::Splitter), which draws only the dividers and
//! leaves the caller to render each pane, a `SplitView` renders a widget in
//! each pane, through [`RenderContext::render_child`], so the widgets are in
//! the DOM: CSS reaches them and the pointer finds them.
//!
//! The sizes the user drags to, and which panes are collapsed, live in a
//! [`SplitState`] the app keeps, so the view can be built afresh every frame
//! from the app's state, like the rest of the tree.
//!
//! ```
//! use revue::event::{Event, Key, KeyEvent};
//! use revue::widget::{pane, split_view, RenderContext, SplitState, Text, View};
//!
//! struct Ide {
//!     split: SplitState,
//! }
//!
//! impl View for Ide {
//!     fn render(&self, ctx: &mut RenderContext) {
//!         split_view(&self.split)
//!             .pane(pane("files").ratio(0.25).min_size(10), Text::new("src/"))
//!             .pane(pane("editor").ratio(0.75), Text::new("fn main() {}"))
//!             .render(ctx);
//!     }
//! }
//!
//! impl Ide {
//!     /// From the app's event handler
//!     fn handle(&mut self, event: &Event) -> bool {
//!         match event {
//!             // Drag a divider with the mouse
//!             Event::Mouse(mouse) => self.split.handle_mouse(mouse),
//!             // Ctrl+B shows or hides the file tree
//!             Event::Key(KeyEvent { key: Key::Char('b'), ctrl: true, .. }) => {
//!                 self.split.toggle("files");
//!                 true
//!             }
//!             // Arrow keys move a divider picked with `start_resize`
//!             Event::Key(key) => self.split.handle_key(&key.key),
//!             _ => false,
//!         }
//!     }
//! }
//! ```

mod layout;
mod state;

pub use state::SplitState;

use state::{Frame, FramePane};

use super::splitter::{Pane, SplitOrientation, SplitterStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::DARK_GRAY;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Panes side by side (or stacked), each holding a widget, with a divider
/// between neighbors. See the [module docs](self).
///
/// A pane's [`Pane`] sets its id, its share of the room
/// ([`ratio`](Pane::ratio), relative to the other panes), its bounds
/// ([`min_size`](Pane::min_size), [`max_size`](Pane::max_size)) and whether
/// it starts collapsed. Sizes the user drags to and panes the app collapses
/// are kept in the [`SplitState`], keyed by pane id.
///
/// `'a` is how long the child widgets borrow: `'static` when they are
/// owned, as they must be for the `SplitView` to go in a
/// [`Stack`](crate::widget::Stack) or [`Border`](crate::widget::Border). The state is
/// not borrowed.
pub struct SplitView<'a> {
    state: SplitState,
    panes: Vec<(Pane, Box<dyn View + 'a>)>,
    orientation: SplitOrientation,
    style: SplitterStyle,
    /// The color the builder named, if it named one
    color: Option<Color>,
    active_color: Color,
    props: WidgetProps,
}

impl<'a> SplitView<'a> {
    /// A side-by-side split drawn from `state` (a handle to it: the view
    /// does not borrow the state)
    pub fn new(state: &SplitState) -> Self {
        Self {
            state: state.clone(),
            panes: Vec::new(),
            orientation: SplitOrientation::Horizontal,
            style: SplitterStyle::Line,
            color: None,
            active_color: Color::CYAN,
            props: WidgetProps::new(),
        }
    }

    /// Add a pane holding `child`
    pub fn pane(mut self, pane: Pane, child: impl View + 'a) -> Self {
        self.panes.push((pane, Box::new(child)));
        self
    }

    /// Set the orientation
    pub fn orientation(mut self, orientation: SplitOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Panes side by side (the default)
    pub fn horizontal(self) -> Self {
        self.orientation(SplitOrientation::Horizontal)
    }

    /// Panes stacked top to bottom
    pub fn vertical(self) -> Self {
        self.orientation(SplitOrientation::Vertical)
    }

    /// Set the divider style
    pub fn style(mut self, style: SplitterStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the divider color (else the stylesheet's `color`, else dark gray)
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the color of the divider being moved
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }
}

impl View for SplitView<'_> {
    crate::impl_view_meta!("SplitView");

    /// All the room it is offered
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, max_height))
    }

    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::BOTH
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let (placed, resizing) = self.state.record(Frame {
            orientation: self.orientation,
            area,
            panes: self
                .panes
                .iter()
                .map(|(pane, _)| FramePane {
                    id: pane.id.clone(),
                    weight: pane.ratio,
                    min: pane.min_size,
                    max: pane.max_size,
                    collapsed: pane.collapsed,
                })
                .collect(),
        });

        let idle = self.color.unwrap_or_else(|| ctx.css_color(DARK_GRAY));
        let ch = self.style.char(self.orientation);

        for (i, pane) in placed.iter().enumerate() {
            let child_area = match self.orientation {
                SplitOrientation::Horizontal => {
                    crate::layout::Rect::new(pane.start, area.y, pane.size, area.height)
                }
                SplitOrientation::Vertical => {
                    crate::layout::Rect::new(area.x, pane.start, area.width, pane.size)
                }
            };
            let child = self.panes[pane.index].1.as_ref();
            if child.needs_render() {
                ctx.render_child(child, child_area);
            }

            // The divider after every pane but the last
            if i + 1 == placed.len() {
                break;
            }
            let fg = if resizing == Some(i) {
                self.active_color
            } else {
                idle
            };
            let at = pane.start.saturating_add(pane.size);
            match self.orientation {
                SplitOrientation::Horizontal => {
                    let x = at.saturating_sub(area.x);
                    for y in 0..area.height {
                        ctx.set(x, y, Cell::new(ch).fg(fg));
                    }
                }
                SplitOrientation::Vertical => {
                    let y = at.saturating_sub(area.y);
                    for x in 0..area.width {
                        ctx.set(x, y, Cell::new(ch).fg(fg));
                    }
                }
            }
        }
    }
}

impl_styled_view!(SplitView<'_>);
impl_props_builders!(SplitView<'_>);

/// A side-by-side split drawn from `state`; see [`SplitView`]
pub fn split_view<'a>(state: &SplitState) -> SplitView<'a> {
    SplitView::new(state)
}
