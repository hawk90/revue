// ─────────────────────────────────────────────────────────────────────────────
// Global drag context (optional singleton)
// ─────────────────────────────────────────────────────────────────────────────

use std::sync::{Arc, OnceLock, RwLock};

use super::{DragContext, DragData, DragId};

static GLOBAL_DRAG_CTX: OnceLock<Arc<RwLock<DragContext>>> = OnceLock::new();

/// Get the global drag context
pub fn drag_context() -> Arc<RwLock<DragContext>> {
    GLOBAL_DRAG_CTX
        .get_or_init(|| Arc::new(RwLock::new(DragContext::new())))
        .clone()
}

/// Start a drag using the global context
///
/// Returns `true` if the drag was started successfully, `false` if the lock was poisoned.
pub fn start_drag(data: DragData, x: u16, y: u16) -> bool {
    match drag_context().write() {
        Ok(mut ctx) => {
            ctx.start_drag(data, x, y);
            true
        }
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in start_drag");
            crate::log_warn!("Drag context lock poisoned in start_drag - drag not started");
            false
        }
    }
}

/// Update drag position using the global context
///
/// Returns `true` if the position was updated successfully, `false` if the lock was poisoned.
pub fn update_drag_position(x: u16, y: u16) -> bool {
    match drag_context().write() {
        Ok(mut ctx) => {
            ctx.update_position(x, y);
            true
        }
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in update_drag_position");
            crate::log_warn!("Drag context lock poisoned in update_drag_position - update ignored");
            false
        }
    }
}

/// End drag using the global context
///
/// Returns `Some((data, target))` if a drag was in progress and ended successfully,
/// `None` if no drag was in progress or if the lock was poisoned.
pub fn end_drag() -> Option<(DragData, Option<DragId>)> {
    match drag_context().write() {
        Ok(mut ctx) => ctx.end_drag(),
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in end_drag");
            crate::log_warn!("Drag context lock poisoned in end_drag - returning None");
            None
        }
    }
}

/// Cancel drag using the global context
///
/// Returns `true` if the cancel was processed, `false` if the lock was poisoned.
pub fn cancel_drag() -> bool {
    match drag_context().write() {
        Ok(mut ctx) => {
            ctx.cancel();
            true
        }
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in cancel_drag");
            crate::log_warn!("Drag context lock poisoned in cancel_drag - cancel ignored");
            false
        }
    }
}

/// Check if dragging using the global context
///
/// Returns `false` if not dragging or if the lock was poisoned.
pub fn is_dragging() -> bool {
    match drag_context().read() {
        Ok(ctx) => ctx.is_dragging(),
        Err(_) => {
            debug_assert!(false, "drag context lock poisoned in is_dragging");
            crate::log_warn!("Drag context lock poisoned in is_dragging - returning false");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Global drag context functions tests
    // =========================================================================

    #[test]
    fn test_global_drag_context_singleton() {
        let ctx1 = drag_context();
        let ctx2 = drag_context();
        // Should return the same instance
        assert!(std::sync::Arc::ptr_eq(&ctx1, &ctx2));
    }

    #[test]
    fn test_start_drag_function() {
        // Just verify it doesn't panic - actual behavior depends on global state
        let data = DragData::text("test");
        let result = start_drag(data, 10, 20);
        // Result depends on whether lock was acquired
        let _ = result;
    }

    #[test]
    fn test_update_drag_position_function() {
        let result = update_drag_position(15, 20);
        // Result depends on whether lock was acquired
        let _ = result;
    }

    #[test]
    fn test_end_drag_function() {
        let result = end_drag();
        // Result depends on whether there was an active drag
        let _ = result;
    }

    #[test]
    fn test_cancel_drag_function() {
        let result = cancel_drag();
        // Result depends on whether lock was acquired
        let _ = result;
    }

    #[test]
    fn test_is_dragging_function() {
        let result = is_dragging();
        // Should not panic, returns false if not dragging
        let _ = result;
    }
}
