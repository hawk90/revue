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

mod builders;
mod drag;
mod helper;
mod types;
mod view;

use std::sync::atomic::{AtomicU64, Ordering};

use crate::event::drag::{DragData, DragId};
use crate::style::Color;
use crate::widget::traits::{WidgetProps, WidgetState};

// Re-exports
pub use helper::drop_zone;
pub use types::DropZoneStyle;

/// Atomic counter for generating unique drop zone IDs
static DROPZONE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
/// Drop zone widget
#[derive(Clone)]
pub struct DropZone<F = fn(DragData) -> bool>
where
    F: FnMut(DragData) -> bool,
{
    /// Unique identifier
    id: DragId,
    /// Placeholder text when empty
    placeholder: String,
    /// Accepted data types
    accepts: Vec<&'static str>,
    /// Visual style
    style: DropZoneStyle,
    /// Drop handler
    on_drop: Option<F>,
    /// Is currently hovered by a drag
    hovered: bool,
    /// Can accept current drag
    can_accept_current: bool,
    /// Normal border color
    border_color: Option<Color>,
    /// Border color while a drag it would accept hovers over it
    hover_color: Color,
    /// Reject indicator color
    reject_color: Color,
    /// Widget state
    state: WidgetState,
    /// Widget props for CSS
    props: WidgetProps,
    /// Minimum height
    min_height: u16,
}

impl DropZone<fn(DragData) -> bool> {
    /// Create a new drop zone with placeholder text
    pub fn new(placeholder: impl Into<String>) -> Self {
        let id = DROPZONE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            id,
            placeholder: placeholder.into(),
            accepts: Vec::new(),
            style: DropZoneStyle::default(),
            on_drop: None,
            hovered: false,
            can_accept_current: false,
            border_color: None,
            hover_color: Color::rgb(100, 200, 100),
            reject_color: Color::rgb(200, 100, 100),
            state: WidgetState::new(),
            props: WidgetProps::new(),
            min_height: 3,
        }
    }
}

// Without these, a widget reports a `WidgetMeta` it has no way to fill in:
// it takes part in the DOM and no rule can select it.
crate::impl_props_builders!(DropZone);
