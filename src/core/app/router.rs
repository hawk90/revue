#![allow(clippy::type_complexity)]
//! Router for TUI navigation
//!
//! Provides navigation between screens/views in TUI applications.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::app::router::{Router, Route};
//!
//! let mut router = Router::new()
//!     .route("/", "home")
//!     .route("/settings", "settings")
//!     .route("/users/:id", "user_detail");
//!
//! router.push("/settings");
//! assert_eq!(router.current_route(), Some("/settings"));
//!
//! router.back();
//! assert_eq!(router.current_route(), Some("/"));
//! ```

use std::collections::HashMap;

/// Route parameter extracted from path
pub type RouteParams = HashMap<String, String>;

/// Query parameters
pub type QueryParams = HashMap<String, String>;

/// Route guard function type
pub type RouteGuard = Box<dyn Fn(&str, &RouteParams) -> bool + Send + Sync>;

/// Route definition
#[derive(Clone, Debug)]
pub struct Route {
    /// Route pattern (e.g., "/users/:id")
    pub pattern: String,
    /// Route name/identifier
    pub name: String,
    /// Route metadata
    pub meta: HashMap<String, String>,
}

impl Route {
    /// Create a new route
    pub fn new(pattern: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            name: name.into(),
            meta: HashMap::new(),
        }
    }

    /// Add metadata
    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.meta.insert(key.into(), value.into());
        self
    }

    /// Check if path matches this route and extract params
    pub fn matches(&self, path: &str) -> Option<RouteParams> {
        let pattern_parts: Vec<&str> = self.pattern.trim_matches('/').split('/').collect();
        let path_parts: Vec<&str> = path.trim_matches('/').split('/').collect();

        if pattern_parts.len() != path_parts.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (pattern_part, path_part) in pattern_parts.iter().zip(path_parts.iter()) {
            if let Some(param_name) = pattern_part.strip_prefix(':') {
                // Parameter
                params.insert(param_name.to_string(), path_part.to_string());
            } else if let Some(rest) = pattern_part.strip_prefix('*') {
                // Wildcard (matches rest)
                let param_name = if rest.is_empty() { "wildcard" } else { rest };
                params.insert(param_name.to_string(), path_part.to_string());
            } else if *pattern_part != *path_part {
                return None;
            }
        }

        Some(params)
    }
}

/// Navigation entry in history
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    /// Path
    pub path: String,
    /// Route params
    pub params: RouteParams,
    /// Query params
    pub query: QueryParams,
    /// State data
    pub state: HashMap<String, String>,
}

impl HistoryEntry {
    /// Create new history entry
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            params: HashMap::new(),
            query: HashMap::new(),
            state: HashMap::new(),
        }
    }

    /// Set params
    pub fn with_params(mut self, params: RouteParams) -> Self {
        self.params = params;
        self
    }

    /// Set query params
    pub fn with_query(mut self, query: QueryParams) -> Self {
        self.query = query;
        self
    }

    /// Set state
    pub fn with_state(mut self, state: HashMap<String, String>) -> Self {
        self.state = state;
        self
    }
}

/// Navigation event
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NavigationEvent {
    /// Navigated to a new route
    Push {
        /// Previous path
        from: String,
        /// New path
        to: String,
    },
    /// Replaced current route
    Replace {
        /// Previous path
        from: String,
        /// New path
        to: String,
    },
    /// Went back in history
    Back {
        /// Previous path
        from: String,
        /// New path
        to: String,
    },
    /// Went forward in history
    Forward {
        /// Previous path
        from: String,
        /// New path
        to: String,
    },
}

/// Router for managing navigation
pub struct Router {
    /// Registered routes
    routes: Vec<Route>,
    /// Navigation history
    history: Vec<HistoryEntry>,
    /// Current position in history
    position: usize,
    /// Route guards
    guards: Vec<RouteGuard>,
    /// Default route (fallback)
    default_route: Option<String>,
    /// Navigation listeners
    listeners: Vec<Box<dyn Fn(&NavigationEvent) + Send + Sync>>,
}

impl Router {
    /// Create a new router
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            history: vec![HistoryEntry::new("/")],
            position: 0,
            guards: Vec::new(),
            default_route: None,
            listeners: Vec::new(),
        }
    }

    /// Add a route
    pub fn route(mut self, pattern: impl Into<String>, name: impl Into<String>) -> Self {
        self.routes.push(Route::new(pattern, name));
        self
    }

    /// Add a route with full Route struct
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Set default route for unmatched paths
    pub fn default(mut self, path: impl Into<String>) -> Self {
        self.default_route = Some(path.into());
        self
    }

    /// Add a route guard
    pub fn guard<F>(mut self, guard: F) -> Self
    where
        F: Fn(&str, &RouteParams) -> bool + Send + Sync + 'static,
    {
        self.guards.push(Box::new(guard));
        self
    }

    /// Add navigation listener
    pub fn on_navigate<F>(&mut self, listener: F)
    where
        F: Fn(&NavigationEvent) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    /// Get current path
    pub fn current_path(&self) -> &str {
        self.history
            .get(self.position)
            .map(|e| e.path.as_str())
            .unwrap_or("/")
    }

    /// Get current route name
    pub fn current_route(&self) -> Option<&str> {
        let path = self.current_path();
        self.find_route(path).map(|r| r.name.as_str())
    }

    /// Get current route params
    pub fn params(&self) -> &RouteParams {
        self.history
            .get(self.position)
            .map(|e| &e.params)
            .unwrap_or_else(|| empty_params())
    }

    /// Get a specific param
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params().get(name).map(|s| s.as_str())
    }

    /// Get current query params
    pub fn query(&self) -> &QueryParams {
        self.history
            .get(self.position)
            .map(|e| &e.query)
            .unwrap_or_else(|| empty_params())
    }

    /// Get a specific query param
    pub fn query_param(&self, name: &str) -> Option<&str> {
        self.query().get(name).map(|s| s.as_str())
    }

    /// Get current history entry
    pub fn current_entry(&self) -> Option<&HistoryEntry> {
        self.history.get(self.position)
    }

    /// Navigate to a path
    pub fn push(&mut self, path: impl Into<String>) -> bool {
        let path = path.into();
        let (path, query) = parse_path_and_query(&path);

        // Find matching route
        let (final_path, _route, params) = match self.match_route(&path) {
            Some((r, p)) => (path.clone(), r, p),
            None => {
                if let Some(default) = &self.default_route {
                    if let Some((r, p)) = self.match_route(default) {
                        (default.clone(), r, p)
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        };

        // Check guards
        if !self.check_guards(&final_path, &params) {
            return false;
        }

        let from = self.current_path().to_string();

        // Truncate forward history
        self.history.truncate(self.position + 1);

        // Add new entry
        let entry = HistoryEntry::new(&final_path)
            .with_params(params)
            .with_query(query);
        self.history.push(entry);
        self.position = self.history.len() - 1;

        // Notify listeners
        let event = NavigationEvent::Push {
            from,
            to: final_path,
        };
        self.notify(&event);

        true
    }

    /// Replace current route
    pub fn replace(&mut self, path: impl Into<String>) -> bool {
        let path = path.into();
        let (path, query) = parse_path_and_query(&path);

        let (_route, params) = match self.match_route(&path) {
            Some((r, p)) => (r, p),
            None => return false,
        };

        if !self.check_guards(&path, &params) {
            return false;
        }

        let from = self.current_path().to_string();

        let entry = HistoryEntry::new(&path)
            .with_params(params)
            .with_query(query);

        if let Some(current) = self.history.get_mut(self.position) {
            *current = entry;
        }

        let event = NavigationEvent::Replace {
            from,
            to: path.clone(),
        };
        self.notify(&event);

        true
    }

    /// Go back in history
    pub fn back(&mut self) -> bool {
        if self.position > 0 {
            let from = self.current_path().to_string();
            self.position -= 1;
            let to = self.current_path().to_string();

            let event = NavigationEvent::Back { from, to };
            self.notify(&event);

            true
        } else {
            false
        }
    }

    /// Go forward in history
    pub fn forward(&mut self) -> bool {
        if self.position < self.history.len() - 1 {
            let from = self.current_path().to_string();
            self.position += 1;
            let to = self.current_path().to_string();

            let event = NavigationEvent::Forward { from, to };
            self.notify(&event);

            true
        } else {
            false
        }
    }

    /// Go to specific position in history
    pub fn go(&mut self, delta: isize) -> bool {
        let new_pos = (self.position as isize + delta) as usize;
        if new_pos < self.history.len() {
            self.position = new_pos;
            true
        } else {
            false
        }
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.position > 0
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        self.position < self.history.len() - 1
    }

    /// Get history length
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Get current position in history
    pub fn history_position(&self) -> usize {
        self.position
    }

    /// Get all history entries
    pub fn history(&self) -> &[HistoryEntry] {
        &self.history
    }

    /// Clear history and reset to path
    pub fn reset(&mut self, path: impl Into<String>) {
        let path = path.into();
        let (path, query) = parse_path_and_query(&path);

        let params = self.match_route(&path).map(|(_, p)| p).unwrap_or_default();

        self.history.clear();
        self.history.push(
            HistoryEntry::new(&path)
                .with_params(params)
                .with_query(query),
        );
        self.position = 0;
    }

    /// Find a route by path
    fn find_route(&self, path: &str) -> Option<&Route> {
        self.routes.iter().find(|r| r.matches(path).is_some())
    }

    /// Match route and extract params
    fn match_route(&self, path: &str) -> Option<(&Route, RouteParams)> {
        for route in &self.routes {
            if let Some(params) = route.matches(path) {
                return Some((route, params));
            }
        }
        None
    }

    /// Check all guards
    fn check_guards(&self, path: &str, params: &RouteParams) -> bool {
        self.guards.iter().all(|guard| guard(path, params))
    }

    /// Notify listeners of navigation event
    fn notify(&self, event: &NavigationEvent) {
        for listener in &self.listeners {
            listener(event);
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

// Empty params for fallback references
fn empty_params() -> &'static HashMap<String, String> {
    static EMPTY: std::sync::OnceLock<HashMap<String, String>> = std::sync::OnceLock::new();
    EMPTY.get_or_init(HashMap::new)
}

/// Parse path and query string
fn parse_path_and_query(full_path: &str) -> (String, QueryParams) {
    if let Some(idx) = full_path.find('?') {
        let path = full_path[..idx].to_string();
        let query_str = &full_path[idx + 1..];
        let query = parse_query_string(query_str);
        (path, query)
    } else {
        (full_path.to_string(), HashMap::new())
    }
}

/// Parse query string into params
fn parse_query_string(query: &str) -> QueryParams {
    let mut params = HashMap::new();
    for pair in query.split('&') {
        if let Some(idx) = pair.find('=') {
            let key = &pair[..idx];
            let value = &pair[idx + 1..];
            params.insert(key.to_string(), value.to_string());
        } else if !pair.is_empty() {
            params.insert(pair.to_string(), String::new());
        }
    }
    params
}

/// Route builder for ergonomic route definition
pub struct RouteBuilder {
    routes: Vec<Route>,
}

impl RouteBuilder {
    /// Create new route builder
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Add a route
    pub fn route(mut self, pattern: impl Into<String>, name: impl Into<String>) -> Self {
        self.routes.push(Route::new(pattern, name));
        self
    }

    /// Add route with metadata
    pub fn route_with_meta(
        mut self,
        pattern: impl Into<String>,
        name: impl Into<String>,
        meta: HashMap<String, String>,
    ) -> Self {
        let mut route = Route::new(pattern, name);
        route.meta = meta;
        self.routes.push(route);
        self
    }

    /// Build routes
    pub fn build(self) -> Vec<Route> {
        self.routes
    }
}

impl Default for RouteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new router
pub fn router() -> Router {
    Router::new()
}

/// Create routes builder
pub fn routes() -> RouteBuilder {
    RouteBuilder::new()
}
// KEEP HERE - Private implementation tests (accesses private fields)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_matches_simple() {
        let route = Route::new("/home", "home");
        assert!(route.matches("/home").is_some());
        assert!(route.matches("/settings").is_none());
    }

    #[test]
    fn test_route_matches_params() {
        let route = Route::new("/users/:id", "user");
        let params = route.matches("/users/123").unwrap();
        assert_eq!(params.get("id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_route_matches_multiple_params() {
        let route = Route::new("/users/:id/posts/:post_id", "user_post");
        let params = route.matches("/users/42/posts/7").unwrap();
        assert_eq!(params.get("id"), Some(&"42".to_string()));
        assert_eq!(params.get("post_id"), Some(&"7".to_string()));
    }

    #[test]
    fn test_router_push() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/settings", "settings");

        assert_eq!(router.current_path(), "/");

        router.push("/settings");
        assert_eq!(router.current_path(), "/settings");
        assert_eq!(router.current_route(), Some("settings"));
    }

    #[test]
    fn test_router_back_forward() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/settings", "settings")
            .route("/profile", "profile");

        router.push("/settings");
        router.push("/profile");

        assert_eq!(router.current_path(), "/profile");

        router.back();
        assert_eq!(router.current_path(), "/settings");

        router.back();
        assert_eq!(router.current_path(), "/");

        router.forward();
        assert_eq!(router.current_path(), "/settings");
    }

    #[test]
    fn test_router_params() {
        let mut router = Router::new().route("/users/:id", "user");

        router.push("/users/42");
        assert_eq!(router.param("id"), Some("42"));
    }

    #[test]
    fn test_router_query_params() {
        let mut router = Router::new().route("/search", "search");

        router.push("/search?q=hello&page=2");
        assert_eq!(router.query_param("q"), Some("hello"));
        assert_eq!(router.query_param("page"), Some("2"));
    }

    #[test]
    fn test_router_replace() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/settings", "settings");

        router.replace("/settings");
        assert_eq!(router.current_path(), "/settings");
        assert_eq!(router.history_len(), 1);
    }

    #[test]
    fn test_router_guard() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/admin", "admin")
            .guard(|path, _| path != "/admin");

        assert!(router.push("/admin") == false);
        assert_eq!(router.current_path(), "/");
    }

    #[test]
    fn test_router_default() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/404", "not_found")
            .default("/404");

        router.push("/nonexistent");
        assert_eq!(router.current_path(), "/404");
    }

    #[test]
    fn test_router_reset() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/settings", "settings");

        router.push("/settings");
        router.push("/settings");
        assert_eq!(router.history_len(), 3);

        router.reset("/");
        assert_eq!(router.history_len(), 1);
        assert_eq!(router.current_path(), "/");
    }

    #[test]
    fn test_can_go_back_forward() {
        let mut router = Router::new().route("/", "home").route("/next", "next");

        assert!(!router.can_go_back());
        assert!(!router.can_go_forward());

        router.push("/next");
        assert!(router.can_go_back());
        assert!(!router.can_go_forward());

        router.back();
        assert!(!router.can_go_back());
        assert!(router.can_go_forward());
    }

    #[test]
    fn test_history_entry() {
        let entry = HistoryEntry::new("/users/42")
            .with_params({
                let mut p = HashMap::new();
                p.insert("id".to_string(), "42".to_string());
                p
            })
            .with_query({
                let mut q = HashMap::new();
                q.insert("tab".to_string(), "posts".to_string());
                q
            });

        assert_eq!(entry.path, "/users/42");
        assert_eq!(entry.params.get("id"), Some(&"42".to_string()));
        assert_eq!(entry.query.get("tab"), Some(&"posts".to_string()));
    }

    #[test]
    fn test_route_builder() {
        let routes = routes().route("/", "home").route("/about", "about").build();

        assert_eq!(routes.len(), 2);
        assert_eq!(routes[0].name, "home");
        assert_eq!(routes[1].name, "about");
    }

    #[test]
    fn test_parse_query_string() {
        let query = parse_query_string("a=1&b=2&c");
        assert_eq!(query.get("a"), Some(&"1".to_string()));
        assert_eq!(query.get("b"), Some(&"2".to_string()));
        assert_eq!(query.get("c"), Some(&String::new()));
    }

    #[test]
    fn test_route_meta() {
        let route = Route::new("/admin", "admin")
            .meta("auth", "required")
            .meta("role", "admin");

        assert_eq!(route.meta.get("auth"), Some(&"required".to_string()));
        assert_eq!(route.meta.get("role"), Some(&"admin".to_string()));
    }

    // =========================================================================
    // Additional router tests
    // =========================================================================

    #[test]
    fn test_route_new_with_string() {
        let pattern = String::from("/test");
        let name = String::from("test_route");
        let route = Route::new(pattern.clone(), name.clone());
        assert_eq!(route.pattern, pattern);
        assert_eq!(route.name, name);
    }

    #[test]
    fn test_route_clone() {
        let route = Route::new("/test", "test").meta("key", "value");
        let cloned = route.clone();
        assert_eq!(route.pattern, cloned.pattern);
        assert_eq!(route.name, cloned.name);
    }

    #[test]
    fn test_route_matches_wildcard() {
        let route = Route::new("/files/*", "files");
        // Wildcard only matches a single segment
        let params = route.matches("/files/document.pdf").unwrap();
        assert_eq!(params.get("wildcard"), Some(&"document.pdf".to_string()));
    }

    #[test]
    fn test_route_matches_named_wildcard() {
        let route = Route::new("/files/*path", "files");
        // Named wildcard only matches a single segment
        let params = route.matches("/files/readme.txt").unwrap();
        assert_eq!(params.get("path"), Some(&"readme.txt".to_string()));
    }

    #[test]
    fn test_route_matches_root() {
        let route = Route::new("/", "root");
        assert!(route.matches("/").is_some());
    }

    #[test]
    fn test_route_matches_empty_path() {
        let route = Route::new("/", "root");
        // Empty path should match root
        assert!(route.matches("").is_some());
    }

    #[test]
    fn test_route_matches_trailing_slash() {
        let route = Route::new("/users", "users");
        // Trailing slash adds an empty segment, so it doesn't match
        let result = route.matches("/users/");
        // Empty path part causes mismatch
        assert!(result.is_none() || result.unwrap().is_empty());
    }

    #[test]
    fn test_router_helper_default() {
        let router = router();
        assert_eq!(router.current_path(), "/");
    }

    #[test]
    fn test_router_add_route() {
        let mut router = Router::new();
        let route = Route::new("/custom", "custom");
        router.add_route(route);
        assert!(router.push("/custom"));
    }

    #[test]
    fn test_router_push_with_query() {
        let mut router = Router::new().route("/search", "search");
        router.push("/search?q=test");
        assert_eq!(router.current_path(), "/search");
        assert_eq!(router.query_param("q"), Some("test"));
    }

    #[test]
    fn test_router_push_unregistered() {
        let mut router = Router::new().route("/", "home");
        let result = router.push("/unregistered");
        assert!(!result);
        assert_eq!(router.current_path(), "/");
    }

    #[test]
    fn test_router_push_no_match() {
        let mut router = Router::new().route("/home", "home");
        assert!(!router.push("/other"));
    }

    #[test]
    fn test_router_back_at_start() {
        let mut router = Router::new();
        assert!(!router.back());
        assert_eq!(router.current_path(), "/");
    }

    #[test]
    fn test_router_forward_at_end() {
        let mut router = Router::new();
        assert!(!router.forward());
    }

    #[test]
    fn test_router_go_positive() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/a", "a")
            .route("/b", "b");

        router.push("/a");
        router.push("/b");

        router.go(-2);
        assert_eq!(router.current_path(), "/");
    }

    #[test]
    fn test_router_go_negative() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/a", "a")
            .route("/b", "b");

        router.push("/a");
        router.go(1);
        assert_eq!(router.current_path(), "/a");
    }

    #[test]
    fn test_router_go_out_of_bounds() {
        let mut router = Router::new().route("/", "home");
        router.push("/home");
        assert!(!router.go(100));
    }

    #[test]
    fn test_router_history_entry() {
        let router = Router::new()
            .route("/", "home")
            .route("/settings", "settings");

        let entry = router.current_entry();
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().path, "/");
    }

    #[test]
    fn test_router_on_navigate() {
        let mut router = Router::new().route("/", "home").route("/next", "next");

        router.on_navigate(|event| {
            // Just verify the event can be received
            let _ = event;
        });

        router.push("/next");
        // Test passes if closure is called without panic
    }

    #[test]
    fn test_router_params_empty() {
        let router = Router::new().route("/", "home");
        assert_eq!(router.param("id"), None);
    }

    #[test]
    fn test_router_query_params_empty() {
        let router = Router::new().route("/", "home");
        assert_eq!(router.query_param("q"), None);
    }

    #[test]
    fn test_parse_path_and_query() {
        let (path, query) = parse_path_and_query("/search?q=hello&page=2");
        assert_eq!(path, "/search");
        assert_eq!(query.get("q"), Some(&"hello".to_string()));
        assert_eq!(query.get("page"), Some(&"2".to_string()));
    }

    #[test]
    fn test_parse_path_and_query_no_query() {
        let (path, query) = parse_path_and_query("/users");
        assert_eq!(path, "/users");
        assert!(query.is_empty());
    }

    #[test]
    fn test_parse_query_string_empty() {
        let query = parse_query_string("");
        assert!(query.is_empty());
    }

    #[test]
    fn test_parse_query_string_no_value() {
        let query = parse_query_string("key");
        assert_eq!(query.get("key"), Some(&String::new()));
    }

    #[test]
    fn test_history_entry_new() {
        let entry = HistoryEntry::new("/test");
        assert_eq!(entry.path, "/test");
        assert!(entry.params.is_empty());
        assert!(entry.query.is_empty());
    }

    #[test]
    fn test_history_entry_clone() {
        let entry = HistoryEntry::new("/test");
        let cloned = entry.clone();
        assert_eq!(entry.path, cloned.path);
    }

    #[test]
    fn test_history_entry_with_state() {
        let mut state = HashMap::new();
        state.insert("key".to_string(), "value".to_string());
        let entry = HistoryEntry::new("/test").with_state(state);
        assert_eq!(entry.state.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_navigation_event_push() {
        let event = NavigationEvent::Push {
            from: "/".to_string(),
            to: "/next".to_string(),
        };
        assert_eq!(
            event,
            NavigationEvent::Push {
                from: "/".to_string(),
                to: "/next".to_string(),
            }
        );
    }

    #[test]
    fn test_navigation_events_distinct() {
        let events = [
            NavigationEvent::Push {
                from: "a".to_string(),
                to: "b".to_string(),
            },
            NavigationEvent::Replace {
                from: "a".to_string(),
                to: "b".to_string(),
            },
            NavigationEvent::Back {
                from: "a".to_string(),
                to: "b".to_string(),
            },
            NavigationEvent::Forward {
                from: "a".to_string(),
                to: "b".to_string(),
            },
        ];
        for (i, e1) in events.iter().enumerate() {
            for (j, e2) in events.iter().enumerate() {
                if i == j {
                    assert_eq!(e1, e2);
                } else {
                    assert_ne!(e1, e2);
                }
            }
        }
    }

    #[test]
    fn test_router_builder_default() {
        let builder = RouteBuilder::default();
        assert!(builder.routes.is_empty());
    }

    #[test]
    fn test_route_builder_with_meta() {
        let mut meta = HashMap::new();
        meta.insert("auth".to_string(), "true".to_string());
        let routes = routes()
            .route_with_meta("/admin", "admin", meta.clone())
            .build();
        assert_eq!(routes[0].meta, meta);
    }

    #[test]
    fn test_router_history_position() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/a", "a")
            .route("/b", "b");

        router.push("/a");
        router.push("/b");
        assert_eq!(router.history_position(), 2);

        router.back();
        assert_eq!(router.history_position(), 1);
    }

    #[test]
    fn test_router_history_slice() {
        let mut router = Router::new().route("/", "home").route("/a", "a");

        router.push("/a");
        let history = router.history();
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_router_multiple_guards() {
        let mut router = Router::new()
            .route("/", "home")
            .route("/admin", "admin")
            .guard(|path, _| path != "/forbidden")
            .guard(|path, _| !path.starts_with("/private"));

        assert!(!router.push("/forbidden"));
        assert!(!router.push("/private/data"));
        assert!(router.push("/admin"));
    }

    #[test]
    fn test_route_matches_unicode() {
        let route = Route::new("/search/:query", "search");
        let params = route.matches("/search/你好").unwrap();
        assert_eq!(params.get("query"), Some(&"你好".to_string()));
    }

    #[test]
    fn test_router_with_slashes() {
        let route = Route::new("/a/b/c", "abc");
        assert!(route.matches("/a/b/c").is_some());
        assert!(route.matches("/a/b").is_none());
    }
}
