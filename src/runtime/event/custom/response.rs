/// Response from an event handler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResponse {
    /// Event was handled, continue propagation
    Handled,
    /// Event was not handled, continue propagation
    Ignored,
    /// Event was handled, stop propagation
    StopPropagation,
    /// Event was handled, cancel default action
    Cancel,
    /// Event was handled, stop propagation and cancel
    StopAndCancel,
}

impl EventResponse {
    /// Check if the event was handled
    pub fn is_handled(&self) -> bool {
        !matches!(self, Self::Ignored)
    }

    /// Check if propagation should stop
    pub fn should_stop(&self) -> bool {
        matches!(self, Self::StopPropagation | Self::StopAndCancel)
    }

    /// Check if event should be cancelled
    pub fn should_cancel(&self) -> bool {
        matches!(self, Self::Cancel | Self::StopAndCancel)
    }
}
