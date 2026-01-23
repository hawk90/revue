//! CSS style cascade and specificity
//!
//! Implements CSS cascade algorithm:
//! 1. Find all matching rules
//! 2. Sort by specificity
//! 3. Apply in order (lowest to highest specificity)
//! 4. Apply inline styles last

mod merge;
mod resolver;
mod specificity;

#[cfg(test)]
mod tests;

// Public exports
#[allow(unused_imports)]
pub use merge::StyleMerge;
pub use resolver::{MatchedRule, StyleResolver};
pub use specificity::Specificity;
