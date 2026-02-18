//! Router hook helpers

use crate::core::app::router::RouteParams;
use crate::state::reactive::Signal;

use super::core::DeclarativeRouter;
use super::types::ReactiveRouteState;

/// Get the reactive route signal from a router
pub fn use_route(router: &DeclarativeRouter) -> &Signal<ReactiveRouteState> {
    router.route_signal()
}

/// Get the current path from a router
pub fn use_path(router: &DeclarativeRouter) -> String {
    router.current_path().to_string()
}

/// Get all route params from a router
pub fn use_params(router: &DeclarativeRouter) -> RouteParams {
    router.params().clone()
}

/// Get a specific route param from a router
pub fn use_param(router: &DeclarativeRouter, name: &str) -> Option<String> {
    router.param(name).map(|s| s.to_string())
}

/// Check if a path is currently active
pub fn is_active(router: &DeclarativeRouter, path: &str) -> bool {
    router.is_active(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_path() {
        let router = DeclarativeRouter::new();
        assert_eq!(use_path(&router), "/");
    }

    #[test]
    fn test_use_params_empty() {
        let router = DeclarativeRouter::new();
        assert!(use_params(&router).is_empty());
    }

    #[test]
    fn test_use_param_none() {
        let router = DeclarativeRouter::new();
        assert_eq!(use_param(&router, "id"), None);
    }

    #[test]
    fn test_use_param_with_value() {
        let mut router = DeclarativeRouter::new().route("/users/:id", "user", |_, _| {});
        router.push("/users/99");
        assert_eq!(use_param(&router, "id"), Some("99".to_string()));
    }

    #[test]
    fn test_is_active_helper() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/about", "about", |_, _| {});

        assert!(is_active(&router, "/"));
        router.push("/about");
        assert!(is_active(&router, "/about"));
        assert!(!is_active(&router, "/"));
    }

    #[test]
    fn test_use_route_signal() {
        let mut router = DeclarativeRouter::new()
            .route("/", "home", |_, _| {})
            .route("/test", "test", |_, _| {});

        router.push("/test");
        let signal = use_route(&router);
        let state = signal.get();
        assert_eq!(state.path, "/test");
    }
}
