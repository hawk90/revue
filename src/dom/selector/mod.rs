//! CSS selector parsing and representation
//!
//! Supports full CSS selector syntax:
//! - Type: `Button`, `Input`
//! - ID: `#submit`, `#main-content`
//! - Class: `.primary`, `.btn-large`
//! - Universal: `*`
//! - Attribute: `[disabled]`, `[type="text"]`
//! - Pseudo-class: `:focus`, `:hover`, `:nth-child(2)`
//! - Combinators: ` ` (descendant), `>` (child), `+` (adjacent), `~` (sibling)
//! - Grouping: `Button, Input` (comma-separated)

#![allow(dead_code)]

mod parser;
mod types;

// Re-export public API
pub use parser::{parse_selector, parse_selectors};
#[cfg(test)]
pub use types::SelectorParseError;
pub use types::{AttributeOp, AttributeSelector, Combinator, PseudoClass, Selector, SelectorPart};

#[cfg(test)]
mod tests;
