//! Drag lifecycle state and drop outcome

/// Current state of a drag operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragState {
    /// No drag in progress
    #[default]
    Idle,
    /// Drag has started, waiting for threshold
    Pending,
    /// Actively dragging
    Dragging,
    /// Over a valid drop target
    OverTarget,
    /// Drag completed successfully
    Dropped,
    /// Drag was cancelled
    Cancelled,
}

impl DragState {
    /// Check if a drag is active (Dragging or OverTarget)
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Dragging | Self::OverTarget | Self::Pending)
    }

    /// Check if we're over a valid target
    pub fn is_over_target(&self) -> bool {
        matches!(self, Self::OverTarget)
    }
}

/// Result of a drop operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropResult {
    /// Drop was accepted
    Accepted,
    /// Drop was rejected (invalid target)
    Rejected,
    /// Drop was cancelled by user
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // DragState tests
    // =========================================================================

    #[test]
    fn test_drag_state_default() {
        assert_eq!(DragState::default(), DragState::Idle);
    }

    #[test]
    fn test_drag_state_is_active() {
        assert!(!DragState::Idle.is_active());
        assert!(DragState::Pending.is_active());
        assert!(DragState::Dragging.is_active());
        assert!(DragState::OverTarget.is_active());
        assert!(!DragState::Dropped.is_active());
        assert!(!DragState::Cancelled.is_active());
    }

    #[test]
    fn test_drag_state_is_over_target() {
        assert!(!DragState::Idle.is_over_target());
        assert!(!DragState::Dragging.is_over_target());
        assert!(DragState::OverTarget.is_over_target());
    }

    #[test]
    fn test_drag_state_copy() {
        let state = DragState::Dragging;
        let copied = state;
        assert_eq!(state, copied);
    }

    // =========================================================================
    // DropResult tests
    // =========================================================================

    #[test]
    fn test_drop_result_variants() {
        let _ = DropResult::Accepted;
        let _ = DropResult::Rejected;
        let _ = DropResult::Cancelled;
    }

    #[test]
    fn test_drop_result_equality() {
        assert_eq!(DropResult::Accepted, DropResult::Accepted);
        assert_ne!(DropResult::Accepted, DropResult::Rejected);
    }
}
