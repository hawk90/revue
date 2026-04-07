//! Event logging for debug overlay

use std::collections::VecDeque;
use std::time::Instant;

/// Debug event types
#[derive(Debug, Clone)]
pub enum DebugEvent {
    /// Key press
    KeyPress(String),
    /// Mouse event
    Mouse(String),
    /// State change
    StateChange(String),
    /// Custom event
    Custom(String),
}

/// Event log for debugging
#[derive(Debug, Clone, Default)]
pub struct EventLog {
    /// Logged events
    pub(crate) events: VecDeque<(Instant, DebugEvent)>,
    /// Max events to keep
    pub(crate) max_events: usize,
}

impl EventLog {
    /// Create new event log
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: 50,
        }
    }

    /// Log an event
    pub fn log(&mut self, event: DebugEvent) {
        self.events.push_back((Instant::now(), event));
        if self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }

    /// Get recent events
    pub fn recent(&self, count: usize) -> impl Iterator<Item = &(Instant, DebugEvent)> {
        self.events.iter().rev().take(count)
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_event_key_press() {
        let event = DebugEvent::KeyPress("Enter".to_string());
        assert!(matches!(event, DebugEvent::KeyPress(_)));
    }

    #[test]
    fn test_debug_event_mouse() {
        let event = DebugEvent::Mouse("click".to_string());
        assert!(matches!(event, DebugEvent::Mouse(_)));
    }

    #[test]
    fn test_debug_event_state_change() {
        let event = DebugEvent::StateChange("focus".to_string());
        assert!(matches!(event, DebugEvent::StateChange(_)));
    }

    #[test]
    fn test_debug_event_custom() {
        let event = DebugEvent::Custom("custom".to_string());
        assert!(matches!(event, DebugEvent::Custom(_)));
    }

    #[test]
    fn test_event_log_new() {
        let log = EventLog::new();
        assert!(log.events.is_empty());
        assert_eq!(log.max_events, 50);
    }

    #[test]
    fn test_event_log_default() {
        let log = EventLog::default();
        assert!(log.events.is_empty());
    }

    #[test]
    fn test_event_log_log_and_recent() {
        let mut log = EventLog::new();
        log.log(DebugEvent::KeyPress("a".to_string()));
        log.log(DebugEvent::Mouse("click".to_string()));

        assert_eq!(log.recent(10).count(), 2);
    }

    #[test]
    fn test_event_log_clear() {
        let mut log = EventLog::new();
        log.log(DebugEvent::KeyPress("a".to_string()));
        log.clear();
        assert!(log.events.is_empty());
    }

    #[test]
    fn test_event_log_recent_empty() {
        let log = EventLog::new();
        assert_eq!(log.recent(10).count(), 0);
    }

    #[test]
    fn test_event_log_max_events() {
        let mut log = EventLog::new();
        for i in 0..100 {
            log.log(DebugEvent::KeyPress(i.to_string()));
        }
        assert_eq!(log.events.len(), 50);
    }
}
