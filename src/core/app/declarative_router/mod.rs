//! Declarative router module
//!
//! Wraps the existing `Router` with route-to-view mapping, reactive state, and a `Link` widget.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::app::declarative_router::*;
//!
//! let mut router = DeclarativeRouter::new()
//!     .route("/", "home", |ctx, render_ctx| {
//!         // render home view
//!     })
//!     .route("/users/:id", "user", |ctx, render_ctx| {
//!         let user_id = ctx.params.get("id").unwrap();
//!         // render user view
//!     })
//!     .fallback(|ctx, render_ctx| {
//!         // render 404
//!     });
//!
//! router.push("/users/42");
//! assert_eq!(router.param("id"), Some("42"));
//! ```

pub mod core;
pub mod hooks;
pub mod link;
pub mod types;

pub use self::core::{declarative_router, DeclarativeRouter};
pub use hooks::{is_active, use_param, use_params, use_path, use_route};
pub use link::{link, Link};
pub use types::{ReactiveRouteState, RouteContext, RouteRenderer};
