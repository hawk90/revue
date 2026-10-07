//! IME (Input Method Editor) Composition Support
//!
//! Provides support for CJK and other complex input methods that require
//! composition (multiple keystrokes forming a single character).
//!
//! # Features
//!
//! - Composition start/update/end events
//! - Inline composition preview
//! - Candidate selection support
//! - Visual feedback (underline, highlight)
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::event::ime::*;
//!
//! let mut ime = ImeState::new();
//!
//! // Handle composition events
//! ime.on_composition(|event| {
//!     match event {
//!         CompositionEvent::Start => println!("Started composing"),
//!         CompositionEvent::Update { text, cursor } => {
//!             println!("Composing: {} (cursor at {})", text, cursor);
//!         }
//!         CompositionEvent::End { text } => {
//!             println!("Committed: {:?}", text);
//!         }
//!     }
//! });
//!
//! // Start composition
//! ime.start_composition();
//! ime.update_composition("か", 1);
//! ime.update_composition("かん", 2);
//! ime.commit("漢");
//! ```

mod preedit;
mod state;
mod types;

pub use preedit::{PreeditSegment, PreeditString};
pub use state::ImeState;
pub use types::{Candidate, CompositionEvent, CompositionState, CompositionStyle, ImeConfig};
