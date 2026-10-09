//! Tab view: a tab bar and the selected tab's widget
//!
//! [`Tabs`] draws only the bar; a `TabView` also renders the
//! selected tab's widget below it, through
//! [`RenderContext::render_child`], so the widget is in the DOM. Only the
//! selected tab is rendered. The bar shows with two tabs or more, or as
//! [`TabBar`] says.
//!
//! Which tab is selected lives in a [`TabState`] the app keeps, so the view
//! can be built afresh every frame from the app's state.
//!
//! ```
//! use revue::event::Event;
//! use revue::widget::{tab_view, RenderContext, TabState, Text, View};
//!
//! struct Editor {
//!     files: Vec<(String, String)>, // (path, text)
//!     tabs: TabState,
//! }
//!
//! impl View for Editor {
//!     fn render(&self, ctx: &mut RenderContext) {
//!         self.files
//!             .iter()
//!             .fold(tab_view(&self.tabs), |view, (path, text)| {
//!                 let name = path.rsplit('/').next().unwrap_or(path);
//!                 view.tab_with_id(path.as_str(), name, Text::new(text.as_str()))
//!             })
//!             .render(ctx);
//!     }
//! }
//!
//! impl Editor {
//!     /// From the app's event handler
//!     fn handle(&mut self, event: &Event) -> bool {
//!         match event {
//!             // Click a label to switch
//!             Event::Mouse(mouse) => self.tabs.handle_mouse(mouse),
//!             // Left/Right, Home/End, 1-9 while the bar has the keyboard
//!             Event::Key(key) => self.tabs.handle_key(&key.key),
//!             _ => false,
//!         }
//!     }
//! }
//! ```

mod state;

pub use state::TabState;

use super::tabs::{label_spans, Tabs};
use crate::layout::Rect;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// When a [`TabView`] shows its tab bar
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabBar {
    /// With two tabs or more: a single widget shows on its own
    #[default]
    Auto,
    /// Always, even over a single tab
    Always,
    /// Never: the tab is switched by keys or by the app
    Never,
}

/// A tab bar on the first row and the selected tab's widget below it. See
/// the [module docs](self).
pub struct TabView<'a> {
    state: &'a TabState,
    /// (id, label, widget)
    tabs: Vec<(String, String, Box<dyn View + 'a>)>,
    bar: TabBar,
    props: WidgetProps,
}

impl<'a> TabView<'a> {
    /// A tab view drawn from `state`
    pub fn new(state: &'a TabState) -> Self {
        Self {
            state,
            tabs: Vec::new(),
            bar: TabBar::Auto,
            props: WidgetProps::new(),
        }
    }

    /// Add a tab labeled `label`, which is also its id, holding `child`
    pub fn tab(self, label: impl Into<String>, child: impl View + 'a) -> Self {
        let label = label.into();
        self.tab_with_id(label.clone(), label, child)
    }

    /// When to show the tab bar: by default ([`TabBar::Auto`]) with two
    /// tabs or more
    pub fn tab_bar(mut self, bar: TabBar) -> Self {
        self.bar = bar;
        self
    }

    /// Add a tab with an id apart from its label, for tabs whose labels
    /// repeat (two files named `mod.rs`, say)
    pub fn tab_with_id(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        child: impl View + 'a,
    ) -> Self {
        self.tabs.push((id.into(), label.into(), Box::new(child)));
        self
    }
}

impl View for TabView<'_> {
    crate::impl_view_meta!("TabView");

    /// All the room it is offered
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, max_height))
    }

    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::BOTH
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let show_bar = match self.bar {
            TabBar::Auto => self.tabs.len() > 1,
            TabBar::Always => true,
            TabBar::Never => false,
        };
        let bar_height = if show_bar { area.height.min(1) } else { 0 };
        let bar = Rect::new(area.x, area.y, area.width, bar_height);
        let labels = || self.tabs.iter().map(|(_, label, _)| label.as_str());
        let shown = self.state.record(
            self.tabs.iter().map(|(id, _, _)| id.clone()).collect(),
            bar,
            label_spans(labels()),
        );
        if self.tabs.is_empty() || area.height == 0 {
            return;
        }

        if show_bar {
            let tabs = Tabs::new().tabs(labels().collect()).selected(shown);
            ctx.render_child(&tabs, bar);
        }

        let body = Rect::new(
            area.x,
            area.y + bar_height,
            area.width,
            area.height - bar_height,
        );
        let child = self.tabs[shown].2.as_ref();
        if body.height > 0 && child.needs_render() {
            ctx.render_child(child, body);
        }
    }
}

impl_styled_view!(TabView<'_>);
impl_props_builders!(TabView<'_>);

/// A tab view drawn from `state`; see [`TabView`]
pub fn tab_view(state: &TabState) -> TabView<'_> {
    TabView::new(state)
}
