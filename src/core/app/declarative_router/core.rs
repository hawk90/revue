//! Declarative router implementation

use std::collections::HashMap;

use crate::core::app::router::{QueryParams, RouteParams, Router};
use crate::render::Cell;
use crate::state::reactive::Signal;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

use super::types::{ReactiveRouteState, RouteContext, RouteRenderer};

/// A declarative router that wraps `Router` with renderers and reactive state
pub struct DeclarativeRouter {
    router: Router,
    /// Pattern and renderer, in registration order: the order the router
    /// matches routes in, so both agree on which route a path is
    renderers: Vec<(String, RouteRenderer)>,
    fallback: Option<RouteRenderer>,
    current_route: Signal<ReactiveRouteState>,
    props: WidgetProps,
}

impl DeclarativeRouter {
    /// Create a new declarative router
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            renderers: Vec::new(),
            fallback: None,
            current_route: Signal::new(ReactiveRouteState {
                path: "/".to_string(),
                ..Default::default()
            }),
            props: WidgetProps::new(),
        }
    }

    /// Register a route with a renderer function
    pub fn route<F>(mut self, pattern: &str, name: &str, renderer: F) -> Self
    where
        F: Fn(&RouteContext, &mut RenderContext) + 'static,
    {
        self.router = self.router.route(pattern, name);
        let renderer: RouteRenderer = Box::new(renderer);
        match self.renderers.iter_mut().find(|(p, _)| p == pattern) {
            Some((_, existing)) => *existing = renderer,
            None => self.renderers.push((pattern.to_string(), renderer)),
        }
        self
    }

    /// Set fallback renderer for unmatched routes
    pub fn fallback<F>(mut self, renderer: F) -> Self
    where
        F: Fn(&RouteContext, &mut RenderContext) + 'static,
    {
        self.fallback = Some(Box::new(renderer));
        self
    }

    /// Add a navigation guard
    ///
    /// Checked by [`push`](Self::push) and [`replace`](Self::replace). The guard
    /// gets the destination: its path without the query, and its route params.
    pub fn guard<F>(mut self, guard: F) -> Self
    where
        F: Fn(&str, &RouteParams) -> bool + Send + Sync + 'static,
    {
        self.router = self.router.guard(guard);
        self
    }

    /// Navigate to a path
    pub fn push(&mut self, path: &str) -> bool {
        let result = self.router.push(path);
        if result {
            self.sync_state();
        }
        result
    }

    /// Replace current route
    pub fn replace(&mut self, path: &str) -> bool {
        let result = self.router.replace(path);
        if result {
            self.sync_state();
        }
        result
    }

    /// Go back in history
    pub fn back(&mut self) -> bool {
        let result = self.router.back();
        if result {
            self.sync_state();
        }
        result
    }

    /// Go forward in history
    pub fn forward(&mut self) -> bool {
        let result = self.router.forward();
        if result {
            self.sync_state();
        }
        result
    }

    /// Get current path
    pub fn current_path(&self) -> &str {
        self.router.current_path()
    }

    /// Get current route parameters
    pub fn params(&self) -> &RouteParams {
        self.router.params()
    }

    /// Get a specific parameter
    pub fn param(&self, name: &str) -> Option<&str> {
        self.router.param(name)
    }

    /// Get query parameters
    pub fn query(&self) -> &QueryParams {
        self.router.query()
    }

    /// Get a specific query parameter
    pub fn query_param(&self, name: &str) -> Option<&str> {
        self.router.query_param(name)
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.router.can_go_back()
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        self.router.can_go_forward()
    }

    /// Get the reactive route signal
    pub fn route_signal(&self) -> &Signal<ReactiveRouteState> {
        &self.current_route
    }

    /// Check if a path is the current active route
    pub fn is_active(&self, path: &str) -> bool {
        self.router.current_path() == path
    }

    /// Sync the reactive signal with the router state
    fn sync_state(&self) {
        let state = ReactiveRouteState {
            path: self.router.current_path().to_string(),
            params: self.router.params().clone(),
            query: self.router.query().clone(),
            name: self.router.current_route().map(|s| s.to_string()),
        };
        self.current_route.set(state);
    }

    /// Build the RouteContext for the current route
    fn current_context(&self) -> RouteContext {
        let meta = self
            .router
            .current_entry()
            .and(None::<HashMap<String, String>>)
            .unwrap_or_default();

        RouteContext {
            path: self.router.current_path().to_string(),
            params: self.router.params().clone(),
            query: self.router.query().clone(),
            meta,
        }
    }

    /// Find the renderer for the current route
    fn find_renderer(&self) -> Option<&RouteRenderer> {
        let path = self.router.current_path();
        // First match in registration order, as the router matches routes
        for (pattern, renderer) in &self.renderers {
            let route = crate::core::app::router::Route::new(pattern, "");
            if route.matches(path).is_some() {
                return Some(renderer);
            }
        }
        None
    }
}

impl Default for DeclarativeRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl View for DeclarativeRouter {
    crate::impl_view_meta!("DeclarativeRouter");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let route_ctx = self.current_context();

        if let Some(renderer) = self.find_renderer() {
            renderer(&route_ctx, ctx);
        } else if let Some(fallback) = &self.fallback {
            fallback(&route_ctx, ctx);
        } else {
            // Default: render "404" text
            let msg = format!("No route: {}", self.router.current_path());
            ctx.put_str_with(0, 0, &msg, area.width, Cell::new);
        }
    }
}

impl_styled_view!(DeclarativeRouter);
impl_props_builders!(DeclarativeRouter);

/// Helper function to create a declarative router
pub fn declarative_router() -> DeclarativeRouter {
    DeclarativeRouter::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declarative_router_new() {
        let router = DeclarativeRouter::new();
        assert_eq!(router.current_path(), "/");
        assert!(!router.can_go_back());
        assert!(!router.can_go_forward());
    }

    #[test]
    fn test_declarative_router_push() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/settings", "settings", |_, _| {});

        assert!(router.push("/settings"));
        assert_eq!(router.current_path(), "/settings");
    }

    #[test]
    fn test_declarative_router_params() {
        let mut router = DeclarativeRouter::new().route("/users/:id", "user", |_, _| {});

        router.push("/users/42");
        assert_eq!(router.param("id"), Some("42"));
    }

    #[test]
    fn test_declarative_router_signal_updates() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/next", "next", |_, _| {});

        router.push("/next");

        let state = router.route_signal().get();
        assert_eq!(state.path, "/next");
        assert_eq!(state.name, Some("next".to_string()));
    }

    #[test]
    fn test_declarative_router_back_forward() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/a", "a", |_, _| {})
            .route("/b", "b", |_, _| {});

        router.push("/a");
        router.push("/b");
        assert_eq!(router.current_path(), "/b");

        router.back();
        assert_eq!(router.current_path(), "/a");

        router.forward();
        assert_eq!(router.current_path(), "/b");
    }

    #[test]
    fn test_declarative_router_replace() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/new", "new", |_, _| {});

        router.replace("/new");
        assert_eq!(router.current_path(), "/new");
        assert!(!router.can_go_back());
    }

    #[test]
    fn test_declarative_router_guard() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/admin", "admin", |_, _| {})
            .guard(|path, _| path != "/admin");

        assert!(!router.push("/admin"));
        assert_eq!(router.current_path(), "/");
    }

    #[test]
    fn test_declarative_router_fallback() {
        let router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .fallback(|ctx, _| {
                let _ = ctx.path.clone();
            });
        // fallback is set
        assert!(router.fallback.is_some());
    }

    #[test]
    fn test_declarative_router_is_active() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/about", "about", |_, _| {});

        assert!(router.is_active("/"));
        router.push("/about");
        assert!(router.is_active("/about"));
        assert!(!router.is_active("/"));
    }

    #[test]
    fn test_declarative_router_query_params() {
        let mut router = DeclarativeRouter::new().route("/search", "search", |_, _| {});

        router.push("/search?q=hello&page=2");
        assert_eq!(router.query_param("q"), Some("hello"));
        assert_eq!(router.query_param("page"), Some("2"));
    }

    #[test]
    fn test_declarative_router_helper() {
        let router = declarative_router();
        assert_eq!(router.current_path(), "/");
    }

    fn guarded() -> DeclarativeRouter {
        DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/admin", "admin", |_, _| {})
            .route("/users/:id", "user", |_, _| {})
            .guard(|path, _| path != "/admin")
    }

    #[test]
    fn a_guard_stops_replace() {
        let mut router = guarded();
        assert!(!router.replace("/admin"));
        assert_eq!(router.current_path(), "/");
        assert!(router.replace("/users/1"));
    }

    #[test]
    fn a_query_does_not_get_past_a_guard() {
        let mut router = guarded();
        assert!(!router.push("/admin?x=1"));
        assert!(!router.replace("/admin?x=1"));
        assert_eq!(router.current_path(), "/");
    }

    #[test]
    fn a_guard_gets_the_params_of_the_destination() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/users/:id", "user", |_, _| {})
            .guard(|path, params| path == "/" || params.get("id").is_some_and(|id| id != "0"));

        assert!(router.push("/users/1"));
        assert!(
            !router.push("/users/0"),
            "params of /users/1 would allow it"
        );
        assert!(!router.replace("/users/0"));
        assert_eq!(router.param("id"), Some("1"));
    }

    fn drawn_by(router: &DeclarativeRouter) -> char {
        let mut buffer = crate::render::Buffer::new(4, 1);
        let area = crate::layout::Rect::new(0, 0, 4, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        router.render(&mut ctx);
        buffer.get(0, 0).map_or(' ', |cell| cell.symbol)
    }

    fn mark(c: char) -> impl Fn(&RouteContext, &mut RenderContext) + 'static {
        move |_, ctx| ctx.set(0, 0, Cell::new(c))
    }

    #[test]
    fn the_first_registered_route_that_matches_renders_whatever_the_hash_order() {
        // RandomState differs per map, so one lucky order cannot pass all of these.
        for _ in 0..64 {
            let mut router = DeclarativeRouter::new()
                .route("/users/new", "new", mark('A'))
                .route("/users/:id", "user", mark('B'))
                .route("/users/*rest", "rest", mark('C'));
            router.push("/users/new");
            assert_eq!(drawn_by(&router), 'A');
            router.push("/users/7");
            assert_eq!(drawn_by(&router), 'B');
        }
    }

    #[test]
    fn registering_a_pattern_again_replaces_its_renderer() {
        let mut router = DeclarativeRouter::new()
            .route("/a", "first", mark('A'))
            .route("/a", "second", mark('B'));
        router.push("/a");
        assert_eq!(drawn_by(&router), 'B');
    }
}
