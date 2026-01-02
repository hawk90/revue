//! DOM (Document Object Model) system for widget styling
//!
//! Provides a DOM-like tree structure for CSS selector matching and style cascade.
//!
//! # Overview
//!
//! The DOM system enables Textual-like CSS styling:
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
