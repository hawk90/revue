//! Navigation state pattern
//!
//! Provides browser-like navigation history for TUI applications.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::patterns::{NavigationState, Route};
//!
//! let mut nav = NavigationState::new("home");
//!
//! nav.push("list");
//! nav.push("detail/123");
//!
//! assert_eq!(nav.current(), "detail/123");
//!
//! nav.back();
//! assert_eq!(nav.current(), "list");
//!
//! nav.forward();
//! assert_eq!(nav.current(), "detail/123");
//! ```

use std::collections::HashMap;

/// Route with optional parameters
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Route {
    /// Route path (e.g., "home", "list", "detail")
    pub path: String,
    /// Route parameters (e.g., {"id": "123"})
    pub params: HashMap<String, String>,
    /// Optional state data
    pub state: Option<String>,
}

impl Route {
    /// Create a new route
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            params: HashMap::new(),
            state: None,
        }
    }

    /// Add a parameter
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Set state data
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Get a parameter value
    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Check if route matches a path pattern
    pub fn matches(&self, pattern: &str) -> bool {
        // Simple pattern matching: "detail/:id" matches "detail"
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let path_parts: Vec<&str> = self.path.split('/').collect();

        if pattern_parts.len() != path_parts.len() {
            // Check if pattern has wildcards
            if !pattern.contains(':') && !pattern.contains('*') {
                return false;
            }
        }

        for (p, s) in pattern_parts.iter().zip(path_parts.iter()) {
            if p.starts_with(':') {
                continue; // Parameter placeholder
            }
            if *p == "*" {
                return true; // Wildcard
            }
            if p != s {
                return false;
            }
        }

        true
    }
}

impl From<&str> for Route {
    fn from(path: &str) -> Self {
        Route::new(path)
    }
}

impl From<String> for Route {
    fn from(path: String) -> Self {
        Route::new(path)
    }
}

/// Navigation event type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NavigationEvent {
    /// Navigated to new route
    Push(Route),
    /// Replaced current route
    Replace(Route),
    /// Went back in history
    Back,
    /// Went forward in history
    Forward,
}

/// Navigation change listener
pub type NavigationListener = Box<dyn Fn(&Route, &NavigationEvent) + Send + Sync>;

/// Navigation state with history
pub struct NavigationState {
    /// History stack (past routes)
    history: Vec<Route>,
    /// Current route index in history
    current_index: usize,
    /// Maximum history size (0 = unlimited)
    max_history: usize,
    /// Navigation listeners
    listeners: Vec<NavigationListener>,
}

impl NavigationState {
    /// Create a new navigation state with initial route
    pub fn new(initial: impl Into<Route>) -> Self {
        let route = initial.into();
        Self {
            history: vec![route],
            current_index: 0,
            max_history: 100,
            listeners: Vec::new(),
        }
    }

    /// Set maximum history size
    pub fn max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// Get current route
    pub fn current(&self) -> &Route {
        &self.history[self.current_index]
    }

    /// Get current path
    pub fn path(&self) -> &str {
        &self.current().path
    }

    /// Navigate to a new route
    pub fn push(&mut self, route: impl Into<Route>) {
        let route = route.into();

        // Remove forward history if we're not at the end
        if self.current_index < self.history.len() - 1 {
            self.history.truncate(self.current_index + 1);
        }

        // Add new route
        self.history.push(route.clone());
        self.current_index = self.history.len() - 1;

        // Trim history if too long
        if self.max_history > 0 && self.history.len() > self.max_history {
            let excess = self.history.len() - self.max_history;
            self.history.drain(0..excess);
            self.current_index = self.current_index.saturating_sub(excess);
        }

        // Notify listeners
        let event = NavigationEvent::Push(route);
        self.notify(&event);
    }

    /// Replace current route
    pub fn replace(&mut self, route: impl Into<Route>) {
        let route = route.into();
        self.history[self.current_index] = route.clone();

        let event = NavigationEvent::Replace(route);
        self.notify(&event);
    }

    /// Go back in history
    pub fn back(&mut self) -> bool {
        if self.can_go_back() {
            self.current_index -= 1;
            self.notify(&NavigationEvent::Back);
            true
        } else {
            false
        }
    }

    /// Go forward in history
    pub fn forward(&mut self) -> bool {
        if self.can_go_forward() {
            self.current_index += 1;
            self.notify(&NavigationEvent::Forward);
            true
        } else {
            false
        }
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        self.current_index < self.history.len() - 1
    }

    /// Go to specific index in history
    pub fn go(&mut self, delta: isize) -> bool {
        let new_index = self.current_index as isize + delta;
        if new_index >= 0 && (new_index as usize) < self.history.len() {
            self.current_index = new_index as usize;
            true
        } else {
            false
        }
    }

    /// Get history length
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Get current position in history
    pub fn position(&self) -> usize {
        self.current_index
    }

    /// Check if current route matches a pattern
    pub fn is_at(&self, pattern: &str) -> bool {
        self.current().matches(pattern)
    }

    /// Get history as slice
    pub fn history(&self) -> &[Route] {
        &self.history
    }

    /// Add navigation listener
    pub fn on_navigate<F>(&mut self, listener: F)
    where
        F: Fn(&Route, &NavigationEvent) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    /// Notify listeners of navigation event
    fn notify(&self, event: &NavigationEvent) {
        let route = self.current();
        for listener in &self.listeners {
            listener(route, event);
        }
    }

    /// Clear all history except current
    pub fn clear_history(&mut self) {
        let current = self.history[self.current_index].clone();
        self.history.clear();
        self.history.push(current);
        self.current_index = 0;
    }
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new("home")
    }
}

/// Breadcrumb item for display
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    /// Display label
    pub label: String,
    /// Route path
    pub path: String,
    /// Whether this is the current item
    pub current: bool,
}

/// Build breadcrumbs from navigation state
pub fn build_breadcrumbs(nav: &NavigationState, labels: &HashMap<&str, &str>) -> Vec<BreadcrumbItem> {
    let path = nav.path();
    let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

    let mut breadcrumbs = Vec::new();
    let mut current_path = String::new();

    // Add home
    breadcrumbs.push(BreadcrumbItem {
        label: labels.get("").copied().unwrap_or("Home").to_string(),
        path: String::new(),
        current: parts.is_empty(),
    });

    // Add path parts
    for (i, part) in parts.iter().enumerate() {
        current_path.push('/');
        current_path.push_str(part);

        let label = labels
            .get(current_path.as_str())
            .or_else(|| labels.get(*part))
            .copied()
            .unwrap_or(part);

        breadcrumbs.push(BreadcrumbItem {
            label: label.to_string(),
            path: current_path.clone(),
            current: i == parts.len() - 1,
        });
    }

    breadcrumbs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_push() {
        let mut nav = NavigationState::new("home");

        nav.push("list");
        assert_eq!(nav.path(), "list");

        nav.push("detail");
        assert_eq!(nav.path(), "detail");
        assert_eq!(nav.history_len(), 3);
    }

    #[test]
    fn test_navigation_back_forward() {
        let mut nav = NavigationState::new("home");
        nav.push("list");
        nav.push("detail");

        assert!(nav.back());
        assert_eq!(nav.path(), "list");

        assert!(nav.back());
        assert_eq!(nav.path(), "home");

        assert!(!nav.back()); // Can't go back further
        assert_eq!(nav.path(), "home");

        assert!(nav.forward());
        assert_eq!(nav.path(), "list");
    }

    #[test]
    fn test_navigation_replace() {
        let mut nav = NavigationState::new("home");
        nav.push("list");

        nav.replace("new-list");
        assert_eq!(nav.path(), "new-list");
        assert_eq!(nav.history_len(), 2);
    }

    #[test]
    fn test_can_go_back_forward() {
        let mut nav = NavigationState::new("home");

        assert!(!nav.can_go_back());
        assert!(!nav.can_go_forward());

        nav.push("page1");
        assert!(nav.can_go_back());
        assert!(!nav.can_go_forward());

        nav.back();
        assert!(!nav.can_go_back());
        assert!(nav.can_go_forward());
    }

    #[test]
    fn test_push_clears_forward_history() {
        let mut nav = NavigationState::new("home");
        nav.push("a");
        nav.push("b");
        nav.push("c");

        nav.back();
        nav.back();
        assert_eq!(nav.path(), "a");

        nav.push("d");
        assert_eq!(nav.path(), "d");
        assert!(!nav.can_go_forward());
    }

    #[test]
    fn test_route_params() {
        let route = Route::new("detail")
            .param("id", "123")
            .param("tab", "info");

        assert_eq!(route.get_param("id"), Some("123"));
        assert_eq!(route.get_param("tab"), Some("info"));
        assert_eq!(route.get_param("other"), None);
    }

    #[test]
    fn test_route_matches() {
        let route = Route::new("detail/123");

        assert!(route.matches("detail/123"));
        assert!(route.matches("detail/:id"));
        assert!(!route.matches("list"));
        assert!(!route.matches("detail/456"));
    }

    #[test]
    fn test_is_at() {
        let mut nav = NavigationState::new("home");
        nav.push("list");

        assert!(nav.is_at("list"));
        assert!(!nav.is_at("home"));
    }

    #[test]
    fn test_max_history() {
        let mut nav = NavigationState::new("0").max_history(3);

        nav.push("1");
        nav.push("2");
        nav.push("3");

        assert_eq!(nav.history_len(), 3);
        // Oldest entry "0" should be removed
    }

    #[test]
    fn test_clear_history() {
        let mut nav = NavigationState::new("home");
        nav.push("a");
        nav.push("b");

        nav.clear_history();
        assert_eq!(nav.history_len(), 1);
        assert_eq!(nav.path(), "b");
    }

    #[test]
    fn test_breadcrumbs() {
        let mut nav = NavigationState::new("home");
        nav.push("projects/123/issues");

        let labels: HashMap<&str, &str> = [
            ("", "Home"),
            ("projects", "Projects"),
        ].into();

        let crumbs = build_breadcrumbs(&nav, &labels);
        assert_eq!(crumbs.len(), 4);
        assert_eq!(crumbs[0].label, "Home");
        assert_eq!(crumbs[1].label, "Projects");
        assert!(crumbs[3].current);
    }

    #[test]
    fn test_go_delta() {
        let mut nav = NavigationState::new("a");
        nav.push("b");
        nav.push("c");
        nav.push("d");

        assert!(nav.go(-2));
        assert_eq!(nav.path(), "b");

        assert!(nav.go(1));
        assert_eq!(nav.path(), "c");

        assert!(!nav.go(10)); // Invalid
        assert_eq!(nav.path(), "c");
    }
}
