//! Tabs widget for tabbed navigation

mod input;
mod view;

pub(crate) use view::{bar_window, label_spans, shift_spans};

use crate::layout::Rect;
use crate::style::Color;
use crate::utils::{ScrollState, Selection};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Tab item
#[derive(Clone)]
pub struct Tab {
    /// Tab label
    pub label: String,
}

impl Tab {
    /// Create a new tab
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

/// Tabs widget for tabbed navigation
#[derive(Clone)]
pub struct Tabs {
    tabs: Vec<Tab>,
    selection: Selection,
    /// Where the bar is scrolled to, in columns
    scroll: ScrollState,
    /// The first tab to draw, when the caller keeps the scroll (TabView)
    first_tab: Option<usize>,
    fg: Option<Color>,
    bg: Option<Color>,
    active_fg: Option<Color>,
    active_bg: Option<Color>,
    divider: char,
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
    props: WidgetProps,
}

impl Tabs {
    /// Create a new tabs widget
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            selection: Selection::new(0),
            scroll: ScrollState::new(),
            first_tab: None,
            fg: None,
            bg: None,
            active_fg: Some(Color::rgb(220, 220, 220)),
            active_bg: Some(Color::BLUE),
            divider: '│',
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            props: WidgetProps::new(),
        }
    }

    /// Set tabs
    pub fn tabs(mut self, tabs: Vec<impl Into<String>>) -> Self {
        self.tabs = tabs.into_iter().map(|t| Tab::new(t)).collect();
        self.selection.set_len(self.tabs.len());
        self
    }

    /// Add a tab
    pub fn tab(mut self, label: impl Into<String>) -> Self {
        self.tabs.push(Tab::new(label));
        self.selection.set_len(self.tabs.len());
        self
    }

    /// Set selected tab index
    pub fn selected(mut self, index: usize) -> Self {
        self.selection.set(index);
        self
    }

    /// Draw from tab `first`: for a caller that keeps the bar's scroll
    /// across frames, as TabView does in its TabState
    pub(crate) fn first_tab(mut self, first: usize) -> Self {
        self.first_tab = Some(first);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set active tab colors
    pub fn active_style(mut self, fg: Color, bg: Color) -> Self {
        self.active_fg = Some(fg);
        self.active_bg = Some(bg);
        self
    }

    /// Set divider character
    pub fn divider(mut self, ch: char) -> Self {
        self.divider = ch;
        self
    }

    /// Set minimum width constraint
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set minimum height constraint
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set maximum width constraint (0 = no limit)
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set maximum height constraint (0 = no limit)
    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = height;
        self
    }

    /// Set both min width and height
    pub fn min_size(self, width: u16, height: u16) -> Self {
        self.min_width(width).min_height(height)
    }

    /// Set both max width and height (0 = no limit)
    pub fn max_size(self, width: u16, height: u16) -> Self {
        self.max_width(width).max_height(height)
    }

    /// Set all size constraints at once
    pub fn constrain(self, min_w: u16, min_h: u16, max_w: u16, max_h: u16) -> Self {
        self.min_width(min_w)
            .min_height(min_h)
            .max_width(max_w)
            .max_height(max_h)
    }

    /// Apply size constraints to the available area
    fn apply_constraints(&self, area: Rect) -> Rect {
        let eff_max_w = if self.max_width > 0 {
            self.max_width.max(self.min_width)
        } else {
            u16::MAX
        };
        let eff_max_h = if self.max_height > 0 {
            self.max_height.max(self.min_height)
        } else {
            u16::MAX
        };
        let width = area.width.clamp(self.min_width, eff_max_w);
        let height = area.height.clamp(self.min_height, eff_max_h);

        Rect::new(area.x, area.y, width, height)
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create tabs
pub fn tabs() -> Tabs {
    Tabs::new()
}

impl_styled_view!(Tabs);
impl_props_builders!(Tabs);

// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests;
