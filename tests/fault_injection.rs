//! Fault injection: the library under hostile I/O, hostile events and hostile
//! text, checked for "no panic, no hang, errors reported" - and a little more
//! per layer.
//!
//! Five layers, one file each, so a failure names its layer:
//!
//! - `fault/output.rs` - writers that fail, write short, refuse to flush or
//!   are interrupted, under `Terminal` and `CrosstermBackend`;
//! - `fault/events.rs` - degenerate resizes, off-screen mouse, a 1 MB paste,
//!   a 10 000-key burst and odd keys through the real `App` event path;
//! - `fault/parsers.rs` - arbitrary, token-soup and nasty text into every
//!   parser;
//! - `fault/resources.rs` - missing, unreadable, huge, corrupt and streaming
//!   files, clipboard tools that fail or hang, a file watcher losing its
//!   files, and an HTTP backend that errors or sends garbage;
//! - `fault/concurrency.rs` - tasks that panic, never finish or are
//!   cancelled, full queues, poisoned locks, re-entrant reactive callbacks
//!   and failing plugins.
//!
//! Known failures live in `fault/known_failures.rs`; the list may only shrink.
//! See `docs/refactor/findings-fault-injection.md`.

mod fault;
