//! Drop zone widget for drag-and-drop targets
//!
//! A configurable drop target area that accepts dragged items.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::DropZone;
//!
//! DropZone::new("Drop files here")
//!     .accepts(&["file", "text"])
//!     .on_drop(|data| {
//!         println!("Dropped: {:?}", data);
//!         true
//!     })
//! ```

mod core;
mod helper;
mod types;

// Public API tests extracted to tests/widget/dropzone/ (core.rs, helper.rs, types.rs)

// Re-exports
pub use core::DropZone;
pub use helper::drop_zone;
pub use types::DropZoneStyle;
