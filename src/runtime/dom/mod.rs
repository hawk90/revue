//! DOM (Document Object Model) system for widget styling
//!
//! Provides a DOM-like tree structure for CSS selector matching and style cascade.
//! Enables Textual-like CSS styling with full selector support.
//!
//! # Overview
//!
//! The DOM system enables powerful CSS styling:
//!
//! ```css
//! /* Type selectors */
//! Button { background: blue; }
//!
//! /* Class selectors */
//! .primary { color: white; }
//!
//! /* ID selectors */
//! #submit { border: thick; }
//!
//! /* Pseudo-classes */
//! Button:focus { border-color: cyan; }
//! Input:disabled { opacity: 0.5; }
//!
//! /* Combinators */
//! .sidebar > Button { width: 100%; }
//! .card Button.danger { background: red; }
//! ```
//!
//! # Core Components
//!
//! | Component | Description | Use Case |
//! |-----------|-------------|----------|
//! | [`DomNode`] | DOM tree node | Widget metadata and state |
//! | [`DomId`] | Unique node identifier | Tracking nodes |
//! | [`DomRenderer`] | DOM manager | Style resolution and rendering |
//! | [`StyleResolver`] | CSS matcher | Selector matching |
//! | [`Query`] | DOM queries | Finding nodes by selector |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::prelude::*;
//! use revue::dom::{DomNode, WidgetMeta};
//!
//! // Create a DOM node with metadata
//! let node = DomNode::new(
//!     DomId::new(),
//!     WidgetMeta::new("Button")
//!         .class("primary")
//!         .id("submit")
//! );
//! ```
//!
//! # CSS Selector Support
//!
//! ## Supported Selectors
//!
//! | Selector | Example | Matches |
//! |----------|---------|---------|
//! | Type | `Button` | All Button widgets |
//! | Class | `.primary` | Widgets with class "primary" |
//! | ID | `#submit` | Widget with id "submit" |
//! | Universal | `*` | All widgets |
//! | Pseudo-class | `:hover` | Widgets in hover state |
//! | Descendant | `.panel Button` | Buttons inside .panel |
//! | Child | `.panel > Button` | Direct children of .panel |
//!
//! ## Selector Specificity
//!
//! Selectors are ranked by specificity (highest to lowest):
//! 1. ID selectors (`#id`)
//! 2. Class selectors (`.class`)
//! 3. Type selectors (`Button`)
//! 4. Universal selectors (`*`)
//!
//! # DOM Queries
//!
//! ```rust,ignore
//! use revue::dom::Query;
//!
//! // Query nodes by CSS selector
//! let results = Query::new(".primary Button");
//! for node in results.find_all(&dom) {
//!     println!("Found button: {:?}", node);
//! }
//!
//! // Query single node
//! if let Some(node) = Query::new("#submit").find_first(&dom) {
//!     println!("Found submit button");
//! }
//! ```
//!
//! # Widget Metadata
//!
//! ```rust,ignore
//! use revue::dom::WidgetMeta;
//!
//! let meta = WidgetMeta::new("Button")
//!     .id("my-button")
//!     .class("primary")
//!     .class("large")
//!     .state("hover", true)
//!     .state("focus", false);
//! ```
//!
//! # Style Resolution
//!
//! The style resolution process:
//! 1. Parse CSS into stylesheet
//! 2. Match selectors to DOM nodes
//! 3. Sort by specificity
//! 4. Apply declarations in order
//! 5. Merge with inline styles
//! 6. Inherit from parent
//!
//! # Performance
//!
//! - DOM nodes use object pooling for efficiency
//! - Style resolution is cached per node
//! - Selector matching uses optimized algorithms
//! - Dirty tracking minimizes recomputation

mod cascade;
mod node;
mod pool;
mod query;
mod renderer;
mod selector;

pub use cascade::{MatchedRule, Specificity, StyleResolver};
pub use node::{DomId, DomNode, NodeState, WidgetMeta};
pub use pool::{
    buffer_pool, object_pool, string_pool, vec_pool, BufferPool, ObjectPool, PoolStats, Pooled,
    StringPool, SyncObjectPool, SyncStringPool, VecPool,
};
pub use query::{DomTree, Query, QueryResult};
pub use renderer::{styled_context, DomRenderer};
pub use selector::{
    parse_selector, parse_selectors, Combinator, PseudoClass, Selector, SelectorPart,
};

/// Unique identifier for DOM nodes
pub type NodeId = u64;

/// Generate a unique node ID
pub fn generate_node_id() -> NodeId {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
