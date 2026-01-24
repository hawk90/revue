//! Performance Profiler for DevTools
//!
//! Provides flamegraph visualization, timeline view, and render performance tracking.
//!
//! # Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | Flamegraph | Hierarchical render time visualization |
//! | Timeline | Frame-by-frame rendering timeline |
//! | Ranked View | Components sorted by render time |
//! | Render Counts | Track component render frequency |
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::devtools::{Profiler, ProfilerConfig};
//!
//! let profiler = Profiler::new();
//! profiler.start_recording();
//!
//! // ... run your app ...
//!
//! profiler.stop_recording();
//! let report = profiler.generate_report();
//! ```

mod core;
mod types;

pub use core::Profiler;
pub use types::*;
