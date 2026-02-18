# Routing Guide

Revue provides a declarative router for building multi-screen TUI applications with route-to-view mapping, reactive state, and navigation guards.

## Overview

The `DeclarativeRouter` wraps Revue's core `Router` (pattern matching, history, guards) with:

- **Route-to-view mapping**: Associate routes with renderer functions
- **Reactive state**: `Signal<ReactiveRouteState>` auto-updates on navigation
- **Link widget**: Styled navigation links with active state
- **Hook helpers**: Convenient functions for accessing route state

## Quick Start

```rust
use revue::app::declarative_router::*;

let mut router = DeclarativeRouter::new()
    .route("/", "home", |ctx, render_ctx| {
        // render home view
    })
    .route("/users/:id", "user", |ctx, render_ctx| {
        let user_id = ctx.params.get("id").unwrap();
        // render user view
    })
    .fallback(|ctx, render_ctx| {
        // render 404 page
    });

router.push("/users/42");
assert_eq!(router.param("id"), Some("42"));
```

## Defining Routes

Register routes with a pattern, name, and renderer function:

```rust
let router = DeclarativeRouter::new()
    .route("/", "home", |ctx, render_ctx| {
        // Home page renderer
    })
    .route("/about", "about", |ctx, render_ctx| {
        // About page renderer
    })
    .route("/users/:id", "user-detail", |ctx, render_ctx| {
        // Dynamic route with parameter
    })
    .route("/posts/:category/:slug", "post", |ctx, render_ctx| {
        // Multiple parameters
    });
```

### Route Parameters

Parameters are defined with `:name` syntax and accessed via `RouteContext`:

```rust
.route("/users/:id/posts/:post_id", "user-post", |ctx, render_ctx| {
    let user_id = ctx.params.get("id").unwrap();
    let post_id = ctx.params.get("post_id").unwrap();
})
```

### Query Parameters

Query strings are parsed automatically:

```rust
router.push("/search?q=hello&page=2");
assert_eq!(router.query_param("q"), Some("hello"));
assert_eq!(router.query_param("page"), Some("2"));
```

### Fallback Route

Handle unmatched routes:

```rust
let router = DeclarativeRouter::new()
    .route("/", "home", |_, _| {})
    .fallback(|ctx, render_ctx| {
        // Render 404 - ctx.path contains the attempted path
    });
```

If no fallback is set, unmatched routes display "No route: /path".

## Navigation

### Push

Navigate to a new route (adds to history):

```rust
router.push("/about");
router.push("/users/42");
router.push("/search?q=hello");
```

### Replace

Replace the current route (no history entry):

```rust
router.replace("/login");
// Cannot go back to previous route
assert!(!router.can_go_back());
```

### Back / Forward

Navigate through history:

```rust
router.push("/a");
router.push("/b");

router.back();     // Now at "/a"
router.forward();  // Now at "/b"

// Check availability
if router.can_go_back() {
    router.back();
}
```

## Navigation Guards

Guards can prevent navigation:

```rust
let router = DeclarativeRouter::new()
    .route("/", "home", |_, _| {})
    .route("/admin", "admin", |_, _| {})
    .guard(|path, _params| {
        // Return false to block navigation
        path != "/admin" || is_authenticated()
    });

// Navigation blocked - returns false
let navigated = router.push("/admin");
assert!(!navigated);
```

Guards receive the target path and current route parameters. Multiple guards can be chained - all must return `true` for navigation to proceed.

## Reactive State

The router maintains a `Signal<ReactiveRouteState>` that updates on every navigation:

```rust
let signal = router.route_signal();
let state = signal.get();

// ReactiveRouteState fields:
// state.path    - Current path (String)
// state.params  - Route parameters (HashMap<String, String>)
// state.query   - Query parameters (HashMap<String, String>)
// state.name    - Route name if matched (Option<String>)
```

This integrates with Revue's reactive system - components that read the signal will automatically update when the route changes.

## Hook Helpers

Convenient functions for accessing route state:

```rust
use revue::app::declarative_router::*;

// Get the reactive signal
let signal = use_route(&router);

// Get current path
let path = use_path(&router); // String

// Get all parameters
let params = use_params(&router); // RouteParams

// Get a specific parameter
let id = use_param(&router, "id"); // Option<String>

// Check if a path is active
if is_active(&router, "/about") {
    // Highlight navigation item
}
```

## Link Widget

The `Link` widget renders styled navigation text with active state support:

```rust
use revue::app::declarative_router::{Link, link};

// Using constructor
let nav = Link::new("/home", "Home")
    .active(is_active(&router, "/home"));

// Using helper function
let nav = link("/about", "About")
    .active(is_active(&router, "/about"));
```

### Styling Links

```rust
use revue::style::Color;

let nav = Link::new("/settings", "Settings")
    .fg(Color::CYAN)              // Normal text color
    .active_fg(Color::WHITE)      // Active text color
    .active_bg(Color::BLUE)       // Active background color
    .underline(true)              // Underline when inactive
    .active(true);                // Mark as active
```

Default styles:
- **Inactive**: Cyan text, underlined
- **Active**: White text on blue background, bold, no underline

### Link Properties

| Method | Default | Description |
|--------|---------|-------------|
| `.fg(color)` | Cyan | Text color |
| `.active_fg(color)` | White | Active text color |
| `.active_bg(color)` | Blue | Active background |
| `.underline(bool)` | `true` | Underline inactive links |
| `.active(bool)` | `false` | Active state |

## View Integration

`DeclarativeRouter` implements the `View` trait, so it can be composed into your widget tree:

```rust
impl View for MyApp {
    fn render(&self, ctx: &mut RenderContext) {
        // Render navigation bar
        // ...

        // Render current route
        self.router.render(ctx);
    }
}
```

The router automatically calls the matched route's renderer function, or the fallback, or displays a default "No route" message.

## Complete Example

```rust
use revue::app::declarative_router::*;
use revue::style::Color;

// Build router
let mut router = DeclarativeRouter::new()
    .route("/", "home", |_ctx, _render_ctx| {
        // Render home page
    })
    .route("/users", "users", |_ctx, _render_ctx| {
        // Render user list
    })
    .route("/users/:id", "user-detail", |ctx, _render_ctx| {
        let _user_id = ctx.params.get("id").unwrap();
        // Render user detail
    })
    .route("/settings", "settings", |_ctx, _render_ctx| {
        // Render settings
    })
    .fallback(|ctx, _render_ctx| {
        // Render 404 for unknown routes
        let _ = format!("Page not found: {}", ctx.path);
    })
    .guard(|path, _| {
        // Block access to /admin
        !path.starts_with("/admin")
    });

// Build navigation links
let nav_items = vec![
    link("/", "Home").active(is_active(&router, "/")),
    link("/users", "Users").active(is_active(&router, "/users")),
    link("/settings", "Settings").active(is_active(&router, "/settings")),
];

// Navigate
router.push("/users");
router.push("/users/42");

// Read reactive state
let state = use_route(&router).get();
assert_eq!(state.path, "/users/42");
assert_eq!(state.name, Some("user-detail".to_string()));
```

## See Also

- [State Management Guide](state.md) - Signals and reactive state
- [Store Guide](store.md) - Centralized state management
