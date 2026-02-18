//! Types for the declarative router

use crate::core::app::router::{QueryParams, RouteParams};
use crate::widget::traits::RenderContext;

/// A route renderer function
pub type RouteRenderer = Box<dyn Fn(&RouteContext, &mut RenderContext)>;

/// Context passed to route renderers
#[derive(Clone, Debug, Default)]
pub struct RouteContext {
    /// Current path
    pub path: String,
    /// Route parameters (e.g., `:id` → value)
    pub params: RouteParams,
    /// Query parameters
    pub query: QueryParams,
    /// Route metadata
    pub meta: std::collections::HashMap<String, String>,
}

/// Reactive route state for Signal tracking
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReactiveRouteState {
    /// Current path
    pub path: String,
    /// Route parameters
    pub params: RouteParams,
    /// Query parameters
    pub query: QueryParams,
    /// Route name (if matched)
    pub name: Option<String>,
}
