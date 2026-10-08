//! RTL (Right-to-Left) and BiDi (Bidirectional) text support
//!
//! Implements Unicode Bidirectional Algorithm (UAX #9) for proper handling
//! of mixed-direction text, including Arabic, Hebrew, and other RTL scripts.

mod helpers;
mod resolve;
mod types;

pub use helpers::{contains_rtl, detect_direction, is_rtl_char, mirror_char, reverse_graphemes};
#[allow(deprecated)] // never read (#799)
pub use types::BidiConfig;
pub use types::{
    BidiClass, BidiInfo, BidiRun, ResolvedDirection, RtlLayout, TextAlign, TextDirection,
};
