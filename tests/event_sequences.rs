//! Event sequences: stateful widgets driven with random sequences of keys,
//! mouse events, data mutations and renders, checked after every step for no
//! panic, nothing written outside the area, and the widget's own invariant
//! (selection inside the items, cursor inside the text, value inside its
//! range, ...).
//!
//! Two layers, one file each, so a failure names its layer:
//!
//! - `sequences/keys.rs` - key sequences on every widget with a key handler;
//! - `sequences/mouse.rs` - mouse sequences on every widget with a mouse
//!   handler.
//!
//! Known failures live in `sequences/known_failures.rs`; the list may only
//! shrink. See `docs/refactor/findings-event-sequences.md`.

mod sequences;
