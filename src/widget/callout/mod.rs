//! Callout widget for highlighting important information blocks

mod callout_type;
mod core;
mod helpers;
mod impls;
#[cfg(test)]
mod tests;
mod types;
mod view;

pub use core::Callout;
pub use helpers::*;
pub use types::{CalloutType, CalloutVariant};
