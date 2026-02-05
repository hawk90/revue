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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_result(
        cancelled: bool,
        propagation_stopped: bool,
        handler_count: usize,
        error: Option<String>,
    ) -> DispatchResult {
        DispatchResult {
            event_id: EventId::new(),
            cancelled,
            propagation_stopped,
            handler_count,
            error,
        }
    }

    #[test]
    fn test_dispatch_result_public_fields() {
        let result = create_test_result(false, false, 0, None);
        assert!(result.event_id.value() > 0);
        assert!(!result.cancelled);
        assert!(!result.propagation_stopped);
        assert_eq!(result.handler_count, 0);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_dispatch_result_is_ok_no_error() {
        let result = create_test_result(false, false, 0, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dispatch_result_is_ok_with_error() {
        let result = create_test_result(false, false, 0, Some("error".to_string()));
        assert!(!result.is_ok());
    }

    #[test]
    fn test_dispatch_result_was_handled_with_handlers() {
        let result = create_test_result(false, false, 3, None);
        assert!(result.was_handled());
    }

    #[test]
    fn test_dispatch_result_was_handled_no_handlers() {
        let result = create_test_result(false, false, 0, None);
        assert!(!result.was_handled());
    }

    #[test]
    fn test_dispatch_result_cancelled() {
        let result = create_test_result(true, false, 1, None);
        assert!(result.cancelled);
        assert!(!result.propagation_stopped);
        assert!(result.was_handled());
    }

    #[test]
    fn test_dispatch_result_propagation_stopped() {
        let result = create_test_result(false, true, 1, None);
        assert!(!result.cancelled);
        assert!(result.propagation_stopped);
        assert!(result.was_handled());
    }

    #[test]
    fn test_dispatch_result_both_flags() {
        let result = create_test_result(true, true, 2, None);
        assert!(result.cancelled);
        assert!(result.propagation_stopped);
        assert!(result.was_handled());
        assert!(result.is_ok());
    }

    #[test]
    fn test_dispatch_result_with_error_and_cancelled() {
        let result = create_test_result(true, false, 0, Some("test error".to_string()));
        assert!(!result.is_ok());
        assert!(!result.was_handled());
        assert!(result.cancelled);
        assert_eq!(result.error, Some("test error".to_string()));
    }
}
