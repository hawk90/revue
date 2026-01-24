use super::types::*;
use std::sync::Arc;

// =============================================================================
// Gesture Callbacks
// =============================================================================

/// Callback for swipe gestures
pub type SwipeCallback = Arc<dyn Fn(&SwipeGesture) + Send + Sync>;

/// Callback for long press gestures
pub type LongPressCallback = Arc<dyn Fn(&LongPressGesture) + Send + Sync>;

/// Callback for drag gestures
pub type DragCallback = Arc<dyn Fn(&DragGesture) + Send + Sync>;

/// Callback for pinch gestures
pub type PinchCallback = Arc<dyn Fn(&PinchGesture) + Send + Sync>;

/// Callback for tap gestures
pub type TapCallback = Arc<dyn Fn(&TapGesture) + Send + Sync>;

/// Callback for any gesture type
pub type GestureCallback = Arc<dyn Fn(&Gesture) + Send + Sync>;
