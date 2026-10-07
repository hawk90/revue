//! Container-relative queries and resolution

use super::{Breakpoint, Breakpoints, ResponsiveValue};

/// Container-relative query for adapting widget layout based on container size
///
/// Unlike `MediaQuery` which operates on the full viewport, container queries
/// allow widgets to respond to their allocated container dimensions.
///
/// # Example
///
/// ```rust,ignore
/// use revue::layout::ContainerQuery;
///
/// let query = ContainerQuery::MinWidth(40);
/// if query.matches(current_width, current_height) {
///     // Render expanded layout
/// } else {
///     // Render compact layout
/// }
/// ```
#[derive(Debug, Clone)]
pub enum ContainerQuery {
    /// Minimum container width
    MinWidth(u16),
    /// Maximum container width
    MaxWidth(u16),
    /// Minimum container height
    MinHeight(u16),
    /// Maximum container height
    MaxHeight(u16),
    /// Width in range (inclusive)
    WidthRange(u16, u16),
    /// Combine with AND
    And(Box<ContainerQuery>, Box<ContainerQuery>),
    /// Combine with OR
    Or(Box<ContainerQuery>, Box<ContainerQuery>),
    /// Negate
    Not(Box<ContainerQuery>),
}

impl ContainerQuery {
    /// Check if the query matches the given container dimensions
    pub fn matches(&self, width: u16, height: u16) -> bool {
        match self {
            ContainerQuery::MinWidth(w) => width >= *w,
            ContainerQuery::MaxWidth(w) => width <= *w,
            ContainerQuery::MinHeight(h) => height >= *h,
            ContainerQuery::MaxHeight(h) => height <= *h,
            ContainerQuery::WidthRange(min, max) => width >= *min && width <= *max,
            ContainerQuery::And(a, b) => a.matches(width, height) && b.matches(width, height),
            ContainerQuery::Or(a, b) => a.matches(width, height) || b.matches(width, height),
            ContainerQuery::Not(q) => !q.matches(width, height),
        }
    }

    /// Combine with AND
    pub fn and(self, other: ContainerQuery) -> ContainerQuery {
        ContainerQuery::And(Box::new(self), Box::new(other))
    }

    /// Combine with OR
    pub fn or(self, other: ContainerQuery) -> ContainerQuery {
        ContainerQuery::Or(Box::new(self), Box::new(other))
    }

    /// Negate
    pub fn not(self) -> ContainerQuery {
        ContainerQuery::Not(Box::new(self))
    }
}

/// Responsive container that resolves values based on container dimensions
///
/// Unlike `ResponsiveLayout` which uses viewport dimensions, this resolves
/// based on the container's allocated area.
#[derive(Debug, Clone)]
pub struct ResponsiveContainer {
    breakpoints: Breakpoints,
}

impl ResponsiveContainer {
    /// Create a new responsive container with standard breakpoints
    pub fn new() -> Self {
        Self {
            breakpoints: Breakpoints::terminal(),
        }
    }

    /// Create with custom breakpoints
    pub fn with_breakpoints(breakpoints: Breakpoints) -> Self {
        Self { breakpoints }
    }

    /// Resolve a responsive value based on container width
    pub fn resolve<T: Clone>(&self, value: &ResponsiveValue<T>, container_width: u16) -> T {
        value.resolve(&self.breakpoints, container_width)
    }

    /// Get current breakpoint for container width
    pub fn current(&self, container_width: u16) -> &Breakpoint {
        self.breakpoints.current(container_width)
    }
}

impl Default for ResponsiveContainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a container query for minimum width
pub fn container_min_width(w: u16) -> ContainerQuery {
    ContainerQuery::MinWidth(w)
}

/// Create a container query for maximum width
pub fn container_max_width(w: u16) -> ContainerQuery {
    ContainerQuery::MaxWidth(w)
}
