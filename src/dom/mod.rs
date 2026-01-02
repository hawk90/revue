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

mod node;
mod selector;
mod cascade;
mod query;
mod renderer;
mod pool;

pub use node::{DomNode, DomId, NodeState, WidgetMeta};
pub use selector::{Selector, SelectorPart, Combinator, PseudoClass, parse_selector, parse_selectors};
pub use cascade::{StyleResolver, Specificity, MatchedRule};
pub use query::{Query, QueryResult, DomTree};
pub use renderer::{DomRenderer, styled_context};
pub use pool::{
    ObjectPool, SyncObjectPool, BufferPool, StringPool, SyncStringPool, VecPool,
    Pooled, PoolStats, object_pool, buffer_pool, string_pool, vec_pool,
};

/// Unique identifier for DOM nodes
pub type NodeId = u64;

/// Generate a unique node ID
pub fn generate_node_id() -> NodeId {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
