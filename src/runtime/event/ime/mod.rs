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
//! # State machine
//!
//! [`ImeState`] is `Idle`, `Composing` or `Selecting`
//! ([`CompositionState`]). It never composes by itself: the app feeds it
//! what the platform input method reports, and it keeps the composition
//! consistent and tells the [`on_composition`](ImeState::on_composition)
//! callbacks.
//!
//! | Call | From | To | Event |
//! |---|---|---|---|
//! | `start_composition` | any | `Composing` (text, cursor and candidates reset) | `Start` |
//! | `update_composition(text, cursor)` | `Idle` | `Composing` | `Start`, then `Update` |
//! | `update_composition(text, cursor)` | `Composing` / `Selecting` | unchanged | `Update` (cursor clamped to the text) |
//! | `set_candidates(non-empty)` | any | `Selecting` | `CandidatesChanged` |
//! | `set_candidates(empty)` | `Selecting` | `Composing` | `CandidatesChanged` |
//! | `next_candidate` / `prev_candidate` / `select_candidate` | with candidates | unchanged (wraps around) | `CandidatesChanged` |
//! | `commit(text)` | `Composing` / `Selecting` | `Idle` | `End { text: Some(text) }` |
//! | `commit_selected` | `Composing` / `Selecting` | `Idle` | `End` with the selected candidate, else the composing text |
//! | `cancel` | `Composing` / `Selecting` | `Idle` | `End { text: None }` |
//! | `backspace` | composing, cursor > 0 | unchanged, or `Idle` when the text empties | `Update`, or `End { text: None }` |
//! | `move_cursor_left` / `move_cursor_right` | composing | unchanged | `Update` |
//! | `disable` | `Composing` / `Selecting` | `Idle` | `End { text: None }` |
//!
//! Calls that do not apply change nothing and emit nothing: `commit` and
//! `cancel` while `Idle`, `backspace` at the start of the text, candidate
//! moves with no candidates. While disabled, `start_composition` and
//! `update_composition` are ignored. A composition longer than the limit, or
//! more candidates than the limit, is ignored rather than truncated.
//!
//! `commit(text)` commits `text`, whatever is being composed: it is how the
//! platform's final string gets in. `commit_selected` is for an app that runs
//! the candidate list itself.
//!
//! # Example
//!
//! ```
//! use revue::event::{Candidate, CompositionEvent, CompositionState, ImeState};
//! use std::sync::{Arc, Mutex};
//!
//! let mut ime = ImeState::new();
//! let log = Arc::new(Mutex::new(Vec::new()));
//! let sink = log.clone();
//! ime.on_composition(move |event| sink.lock().unwrap().push(event.clone()));
//!
//! ime.update_composition("か", 1); // starts the composition
//! ime.update_composition("かん", 2);
//! ime.set_candidates(vec![Candidate::new("漢"), Candidate::new("感")]);
//! assert_eq!(ime.state(), CompositionState::Selecting);
//! ime.next_candidate();
//! assert_eq!(ime.commit_selected().as_deref(), Some("感"));
//! assert_eq!(ime.state(), CompositionState::Idle);
//!
//! let log = log.lock().unwrap();
//! assert_eq!(log[0], CompositionEvent::Start);
//! assert_eq!(
//!     log.last(),
//!     Some(&CompositionEvent::End { text: Some("感".into()) })
//! );
//! ```

mod preedit;
mod state;
mod types;

pub use preedit::{PreeditSegment, PreeditString};
pub use state::ImeState;
pub use types::{Candidate, CompositionEvent, CompositionState, CompositionStyle, ImeConfig};
