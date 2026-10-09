//! Dock: an IDE-style layout of left, right, top, bottom and center areas
//!
//! Each area holds one or more panels (widgets); with two or more, they
//! show as tabs. The areas are a [`SplitView`](super::SplitView) of rows
//! (top, middle, bottom) whose middle is a split of columns (left, center,
//! right), and each area a [`TabView`]. So the dividers
//! drag, areas collapse, and every panel is in the DOM. An area with no
//! panels is left out.
//!
//! The state - area sizes, collapsed areas, each area's selected tab -
//! lives in a [`DockState`] the app keeps, so the dock can be built afresh
//! every frame from the app's state.
//!
//! ```
//! use revue::event::{Event, Key, KeyEvent};
//! use revue::widget::{dock, DockPosition, DockState, RenderContext, Text, View};
//!
//! use DockPosition::{Bottom, Center, Left};
//!
//! struct Ide {
//!     dock: DockState,
//! }
//!
//! impl View for Ide {
//!     fn render(&self, ctx: &mut RenderContext) {
//!         dock(&self.dock)
//!             .panel(Left, "Files", Text::new("src/"))
//!             .panel(Center, "main.rs", Text::new("fn main() {}"))
//!             .panel(Center, "lib.rs", Text::new("pub mod app;"))
//!             .panel(Bottom, "Terminal", Text::new("$ cargo run"))
//!             .size(Left, 0.2)
//!             .min_size(Left, 12)
//!             .render(ctx);
//!     }
//! }
//!
//! impl Ide {
//!     /// From the app's event handler
//!     fn handle(&mut self, event: &Event) -> bool {
//!         match event {
//!             // Drag a divider, click a tab
//!             Event::Mouse(mouse) => self.dock.handle_mouse(mouse),
//!             // Ctrl+B shows or hides the file tree
//!             Event::Key(KeyEvent { key: Key::Char('b'), ctrl: true, .. }) => {
//!                 self.dock.toggle(Left);
//!                 true
//!             }
//!             _ => false,
//!         }
//!     }
//! }
//! ```

use super::split_view::{split_view, SplitState};
use super::splitter::{pane, Pane};
use super::tab_view::{tab_view, TabBar, TabState, TabView};
use crate::event::MouseEvent;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// An area of a [`Dock`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DockPosition {
    /// Left of the center, between the top and bottom areas
    Left,
    /// Right of the center, between the top and bottom areas
    Right,
    /// Across the top
    Top,
    /// Across the bottom
    Bottom,
    /// The middle; takes the room the others leave
    Center,
}

impl DockPosition {
    fn index(self) -> usize {
        self as usize
    }

    /// Pane id in the row or column split
    fn id(self) -> &'static str {
        match self {
            DockPosition::Left => "left",
            DockPosition::Right => "right",
            DockPosition::Top => "top",
            DockPosition::Bottom => "bottom",
            DockPosition::Center => "center",
        }
    }

    /// Whether the area is a row (in the top-to-bottom split) rather than a
    /// column of the middle row
    fn is_row(self) -> bool {
        matches!(self, DockPosition::Top | DockPosition::Bottom)
    }

    /// The share of the room an area takes when the dock does not set one
    fn default_size(self) -> f32 {
        match self {
            DockPosition::Left | DockPosition::Right => 0.2,
            DockPosition::Top | DockPosition::Bottom => 0.25,
            DockPosition::Center => 1.0,
        }
    }
}

/// The state of a [`Dock`]: area sizes the user has dragged, collapsed
/// areas, and each area's selected tab. The app keeps it and builds the
/// `Dock` from it every frame.
///
/// Like the [`SplitState`] and [`TabState`] it is made of, it is a handle:
/// clones share one state, and the dock keeps a clone, so it does not
/// borrow the app and can sit in any container.
#[derive(Clone, Debug, Default)]
pub struct DockState {
    /// Top, middle, bottom
    rows: SplitState,
    /// Left, center, right
    columns: SplitState,
    /// By `DockPosition::index`
    tabs: [TabState; 5],
}

impl DockState {
    /// Every area at its declared size, none collapsed
    pub fn new() -> Self {
        Self::default()
    }

    fn split(&self, position: DockPosition) -> &SplitState {
        if position.is_row() {
            &self.rows
        } else {
            &self.columns
        }
    }

    fn split_mut(&mut self, position: DockPosition) -> &mut SplitState {
        if position.is_row() {
            &mut self.rows
        } else {
            &mut self.columns
        }
    }

    /// Whether the area is collapsed
    pub fn is_collapsed(&self, position: DockPosition) -> bool {
        self.split(position).is_collapsed(position.id())
    }

    /// Collapse or expand an area. A collapsed area is not drawn and its
    /// room goes to the others.
    pub fn set_collapsed(&mut self, position: DockPosition, collapsed: bool) {
        self.split_mut(position)
            .set_collapsed(position.id(), collapsed);
    }

    /// Collapse an expanded area, expand a collapsed one
    pub fn toggle(&mut self, position: DockPosition) {
        self.split_mut(position).toggle(position.id());
    }

    /// The area's tab state: which of its panels is shown
    pub fn tabs(&self, position: DockPosition) -> &TabState {
        &self.tabs[position.index()]
    }

    /// The area's tab state, to select a panel or pass it keys
    pub fn tabs_mut(&mut self, position: DockPosition) -> &mut TabState {
        &mut self.tabs[position.index()]
    }

    /// Forget dragged sizes and collapsed areas
    pub fn reset_layout(&mut self) {
        self.rows.reset();
        self.columns.reset();
    }

    /// Dividers drag (see [`SplitState::handle_mouse`]) and tab labels
    /// select their panel (see [`TabState::handle_mouse`]). Returns whether
    /// the event was used.
    pub fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        self.rows.handle_mouse(event)
            || self.columns.handle_mouse(event)
            || self.tabs.iter_mut().any(|tabs| tabs.handle_mouse(event))
    }
}

/// One area's panels and settings
struct Area<'a> {
    /// (id, label, widget)
    panels: Vec<(String, String, Box<dyn View + 'a>)>,
    size: Option<f32>,
    min: Option<u16>,
    max: u16,
    bar: TabBar,
}

impl Default for Area<'_> {
    fn default() -> Self {
        Self {
            panels: Vec::new(),
            size: None,
            min: None,
            max: 0,
            bar: TabBar::Auto,
        }
    }
}

/// Left, right, top, bottom and center areas of panels. See the
/// [module docs](self).
///
/// `'a` is how long the child widgets borrow: `'static` when they are
/// owned, as they must be for the `Dock` to go in a
/// [`Stack`](crate::widget::Stack) or [`Border`](crate::widget::Border). The state is
/// not borrowed.
pub struct Dock<'a> {
    state: DockState,
    /// By `DockPosition::index`
    areas: [Area<'a>; 5],
    props: WidgetProps,
}

impl<'a> Dock<'a> {
    /// An empty dock drawn from `state` (a handle to it: the dock does not
    /// borrow the state)
    pub fn new(state: &DockState) -> Self {
        Self {
            state: state.clone(),
            areas: Default::default(),
            props: WidgetProps::new(),
        }
    }

    fn area(&mut self, position: DockPosition) -> &mut Area<'a> {
        &mut self.areas[position.index()]
    }

    /// Add a panel to an area, labeled `label`, which is also its id
    pub fn panel(
        self,
        position: DockPosition,
        label: impl Into<String>,
        child: impl View + 'a,
    ) -> Self {
        let label = label.into();
        self.panel_with_id(position, label.clone(), label, child)
    }

    /// Add a panel with an id apart from its label, for panels whose labels
    /// repeat
    pub fn panel_with_id(
        mut self,
        position: DockPosition,
        id: impl Into<String>,
        label: impl Into<String>,
        child: impl View + 'a,
    ) -> Self {
        self.area(position)
            .panels
            .push((id.into(), label.into(), Box::new(child)));
        self
    }

    /// The share of the room an area starts with, from 0.0 to 1.0: of the
    /// width for left and right, of the height for top and bottom. Defaults:
    /// 0.2 for left and right, 0.25 for top and bottom. The center takes
    /// what they leave.
    pub fn size(mut self, position: DockPosition, share: f32) -> Self {
        self.area(position).size = Some(share.clamp(0.0, 1.0));
        self
    }

    /// The fewest cells an area keeps while the user resizes (default 5)
    pub fn min_size(mut self, position: DockPosition, cells: u16) -> Self {
        self.area(position).min = Some(cells);
        self
    }

    /// The most cells an area takes (0, the default, for no limit)
    pub fn max_size(mut self, position: DockPosition, cells: u16) -> Self {
        self.area(position).max = cells;
        self
    }

    /// When the area shows its tab bar: by default ([`TabBar::Auto`]) with
    /// two panels or more
    pub fn tab_bar(mut self, position: DockPosition, bar: TabBar) -> Self {
        self.area(position).bar = bar;
        self
    }

    /// The area as a tab view, if it has panels
    fn tab_view(&self, position: DockPosition) -> Option<TabView<'_>> {
        let area = &self.areas[position.index()];
        if area.panels.is_empty() {
            return None;
        }
        let view = area.panels.iter().fold(
            tab_view(self.state.tabs(position)).tab_bar(area.bar),
            |view, (id, label, child)| view.tab_with_id(id.as_str(), label.as_str(), &**child),
        );
        Some(view)
    }

    /// The split pane for an area, sized as declared
    fn pane(&self, position: DockPosition, share: f32) -> Pane {
        let area = &self.areas[position.index()];
        let pane = pane(position.id()).ratio(share).max_size(area.max);
        match area.min {
            Some(min) => pane.min_size(min),
            None => pane,
        }
    }

    /// The share of an area on the sides, or what the sides leave for the
    /// one in between
    fn share(&self, position: DockPosition, sides: [DockPosition; 2]) -> f32 {
        let size = |p: DockPosition| {
            let area = &self.areas[p.index()];
            if area.panels.is_empty() {
                0.0
            } else {
                area.size.unwrap_or(p.default_size())
            }
        };
        if sides.contains(&position) {
            size(position)
        } else {
            (1.0 - size(sides[0]) - size(sides[1])).max(0.1)
        }
    }
}

impl View for Dock<'_> {
    crate::impl_view_meta!("Dock");

    /// All the room it is offered
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, max_height))
    }

    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::BOTH
    }

    fn render(&self, ctx: &mut RenderContext) {
        use DockPosition::{Bottom, Center, Left, Right, Top};

        let column_sides = [Left, Right];
        let mut columns = split_view(&self.state.columns);
        for position in [Left, Center, Right] {
            if let Some(view) = self.tab_view(position) {
                let share = self.share(position, column_sides);
                columns = columns.pane(self.pane(position, share), view);
            }
        }

        let has_middle = [Left, Center, Right]
            .iter()
            .any(|&p| !self.areas[p.index()].panels.is_empty());
        let middle = has_middle.then_some(columns);

        let row_sides = [Top, Bottom];
        let mut rows = split_view(&self.state.rows).vertical();
        if let Some(view) = self.tab_view(Top) {
            rows = rows.pane(self.pane(Top, self.share(Top, row_sides)), view);
        }
        if let Some(columns) = middle {
            // The middle row's pane id is no area's, so it cannot collapse
            let share = self.share(Center, row_sides);
            rows = rows.pane(pane("middle").ratio(share), columns);
        }
        if let Some(view) = self.tab_view(Bottom) {
            rows = rows.pane(self.pane(Bottom, self.share(Bottom, row_sides)), view);
        }

        ctx.render_child(&rows, ctx.area);
    }
}

impl_styled_view!(Dock<'_>);
impl_props_builders!(Dock<'_>);

/// A dock drawn from `state`; see [`Dock`]
pub fn dock<'a>(state: &DockState) -> Dock<'a> {
    Dock::new(state)
}
