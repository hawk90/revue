//! Pagination widget for page navigation

mod nav;
mod view;

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Pagination style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaginationStyle {
    /// Full pagination with page numbers
    #[default]
    Full,
    /// Simple prev/next only
    Simple,
    /// Compact with current/total display
    Compact,
    /// Dots indicator
    Dots,
}

/// A pagination widget for navigating pages
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// pagination(10)
///     .current(3)
///     .on_change(|page| { /* ... */ })
/// ```
#[derive(Clone)]
pub struct Pagination {
    /// Total number of pages
    total: u16,
    /// Current page (1-indexed)
    current: u16,
    /// Style
    style: PaginationStyle,
    /// Max visible page buttons
    max_visible: u16,
    /// Show prev/next buttons
    show_arrows: bool,
    /// Show first/last buttons
    show_edges: bool,
    /// Active color
    active_color: Color,
    /// Inactive color
    /// The color the builder named, if it named one - see #656.
    inactive_color: Option<Color>,
    /// Is focused
    focused: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Pagination {
    /// Create a new pagination with total pages
    pub fn new(total: u16) -> Self {
        Self {
            total,
            current: 1,
            style: PaginationStyle::Full,
            max_visible: 7,
            show_arrows: true,
            show_edges: true,
            active_color: Color::rgb(60, 120, 200),
            inactive_color: None,
            focused: false,
            props: WidgetProps::new(),
        }
    }

    /// Set current page
    pub fn current(mut self, page: u16) -> Self {
        self.current = page.min(self.total).max(1);
        self
    }

    /// Set style
    pub fn style(mut self, style: PaginationStyle) -> Self {
        self.style = style;
        self
    }

    /// Simple style shorthand
    pub fn simple(mut self) -> Self {
        self.style = PaginationStyle::Simple;
        self
    }

    /// Compact style shorthand
    pub fn compact(mut self) -> Self {
        self.style = PaginationStyle::Compact;
        self
    }

    /// Dots style shorthand
    pub fn dots(mut self) -> Self {
        self.style = PaginationStyle::Dots;
        self
    }

    /// Set max visible page buttons
    pub fn max_visible(mut self, max: u16) -> Self {
        self.max_visible = max.max(3);
        self
    }

    /// Hide arrows
    pub fn no_arrows(mut self) -> Self {
        self.show_arrows = false;
        self
    }

    /// Hide first/last buttons
    pub fn no_edges(mut self) -> Self {
        self.show_edges = false;
        self
    }

    /// Set active color
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    /// Set inactive color
    pub fn inactive_color(mut self, color: Color) -> Self {
        self.inactive_color = Some(color);
        self
    }

    /// Set focused state
    ///
    /// Focused, the current page is drawn underlined.
    pub fn focused(mut self) -> Self {
        self.focused = true;
        self
    }

    // Getters for testing
    #[doc(hidden)]
    pub fn get_style(&self) -> PaginationStyle {
        self.style
    }

    #[doc(hidden)]
    pub fn get_max_visible(&self) -> u16 {
        self.max_visible
    }

    #[doc(hidden)]
    pub fn get_show_arrows(&self) -> bool {
        self.show_arrows
    }

    #[doc(hidden)]
    pub fn get_show_edges(&self) -> bool {
        self.show_edges
    }

    #[doc(hidden)]
    pub fn get_active_color(&self) -> Color {
        self.active_color
    }

    #[doc(hidden)]
    pub fn get_inactive_color(&self) -> Option<Color> {
        self.inactive_color
    }

    #[doc(hidden)]
    pub fn get_focused(&self) -> bool {
        self.focused
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new(1)
    }
}

impl_styled_view!(Pagination);
impl_props_builders!(Pagination);

/// Create a new pagination
pub fn pagination(total: u16) -> Pagination {
    Pagination::new(total)
}
