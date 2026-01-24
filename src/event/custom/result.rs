use super::types::EventId;

/// Result of dispatching an event
#[derive(Debug)]
pub struct DispatchResult {
    /// The event ID
    pub event_id: EventId,
    /// Whether the event was cancelled
    pub cancelled: bool,
    /// Whether propagation was stopped
    pub propagation_stopped: bool,
    /// Number of handlers that processed the event
    pub handler_count: usize,
    /// Error message if dispatch failed
    pub error: Option<String>,
}

impl DispatchResult {
    pub(crate) fn error(msg: &str) -> Self {
        Self {
            event_id: EventId::new(),
            cancelled: false,
            propagation_stopped: false,
            handler_count: 0,
            error: Some(msg.to_string()),
        }
    }

    /// Check if dispatch was successful
    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }

    /// Check if the event was handled by at least one handler
    pub fn was_handled(&self) -> bool {
        self.handler_count > 0
    }
}
