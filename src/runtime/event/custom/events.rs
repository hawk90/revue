use std::collections::HashMap;

use super::types::CustomEvent;

/// Application lifecycle event
#[derive(Debug, Clone)]
pub struct AppEvent {
    /// Event name
    pub name: String,
    /// Event data
    pub data: HashMap<String, String>,
}

impl AppEvent {
    /// Create a new app event
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: HashMap::new(),
        }
    }

    /// Add data
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }
}

impl CustomEvent for AppEvent {
    fn event_type() -> &'static str {
        "app"
    }
}

/// State change event
#[derive(Debug, Clone)]
pub struct StateChangeEvent {
    /// State key
    pub key: String,
    /// Old value (as string)
    pub old_value: Option<String>,
    /// New value (as string)
    pub new_value: Option<String>,
}

impl StateChangeEvent {
    /// Create a new state change event
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            old_value: None,
            new_value: None,
        }
    }

    /// Set old value
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.old_value = Some(value.into());
        self
    }

    /// Set new value
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.new_value = Some(value.into());
        self
    }
}

impl CustomEvent for StateChangeEvent {
    fn event_type() -> &'static str {
        "state_change"
    }
}

/// Navigation event
#[derive(Debug, Clone)]
pub struct NavigateEvent {
    /// Target path/route
    pub path: String,
    /// Navigation parameters
    pub params: HashMap<String, String>,
    /// Replace history instead of push
    pub replace: bool,
}

impl NavigateEvent {
    /// Create a new navigation event
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            params: HashMap::new(),
            replace: false,
        }
    }

    /// Add parameter
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Set replace mode
    pub fn replace(mut self, replace: bool) -> Self {
        self.replace = replace;
        self
    }
}

impl CustomEvent for NavigateEvent {
    fn event_type() -> &'static str {
        "navigate"
    }
}

/// Error event
#[derive(Debug, Clone)]
pub struct ErrorEvent {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error source
    pub source: Option<String>,
    /// Is recoverable
    pub recoverable: bool,
}

impl ErrorEvent {
    /// Create a new error event
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            source: None,
            recoverable: true,
        }
    }

    /// Set source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set recoverable flag
    pub fn recoverable(mut self, recoverable: bool) -> Self {
        self.recoverable = recoverable;
        self
    }
}

impl CustomEvent for ErrorEvent {
    fn event_type() -> &'static str {
        "error"
    }

    fn cancellable() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::event::custom::types::CustomEvent as CustomEventTrait;

    #[test]
    fn test_app_event_new() {
        let event = AppEvent::new("test_event");
        assert_eq!(event.name, "test_event");
        assert!(event.data.is_empty());
    }

    #[test]
    fn test_app_event_with_data() {
        let event = AppEvent::new("test_event")
            .with_data("key1", "value1")
            .with_data("key2", "value2");
        assert_eq!(event.data.len(), 2);
        assert_eq!(event.data.get("key1"), Some(&"value1".to_string()));
        assert_eq!(event.data.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_app_event_event_type() {
        assert_eq!(AppEvent::event_type(), "app");
    }

    #[test]
    fn test_state_change_event_new() {
        let event = StateChangeEvent::new("my_key");
        assert_eq!(event.key, "my_key");
        assert!(event.old_value.is_none());
        assert!(event.new_value.is_none());
    }

    #[test]
    fn test_state_change_event_from_to() {
        let event = StateChangeEvent::new("my_key").from("old").to("new");
        assert_eq!(event.old_value, Some("old".to_string()));
        assert_eq!(event.new_value, Some("new".to_string()));
    }

    #[test]
    fn test_state_change_event_event_type() {
        assert_eq!(StateChangeEvent::event_type(), "state_change");
    }

    #[test]
    fn test_navigate_event_new() {
        let event = NavigateEvent::new("/home");
        assert_eq!(event.path, "/home");
        assert!(event.params.is_empty());
        assert!(!event.replace);
    }

    #[test]
    fn test_navigate_event_with_param() {
        let event = NavigateEvent::new("/home")
            .with_param("id", "123")
            .with_param("tab", "profile");
        assert_eq!(event.params.len(), 2);
        assert_eq!(event.params.get("id"), Some(&"123".to_string()));
        assert_eq!(event.params.get("tab"), Some(&"profile".to_string()));
    }

    #[test]
    fn test_navigate_event_replace() {
        let event = NavigateEvent::new("/home").replace(true);
        assert!(event.replace);
    }

    #[test]
    fn test_navigate_event_event_type() {
        assert_eq!(NavigateEvent::event_type(), "navigate");
    }

    #[test]
    fn test_error_event_new() {
        let event = ErrorEvent::new("E001", "Something went wrong");
        assert_eq!(event.code, "E001");
        assert_eq!(event.message, "Something went wrong");
        assert!(event.source.is_none());
        assert!(event.recoverable); // default
    }

    #[test]
    fn test_error_event_with_source() {
        let event = ErrorEvent::new("E001", "Error").with_source("database");
        assert_eq!(event.source, Some("database".to_string()));
    }

    #[test]
    fn test_error_event_recoverable() {
        let event = ErrorEvent::new("E001", "Error").recoverable(false);
        assert!(!event.recoverable);
    }

    #[test]
    fn test_error_event_event_type() {
        assert_eq!(ErrorEvent::event_type(), "error");
    }

    #[test]
    fn test_error_event_cancellable() {
        assert!(!ErrorEvent::cancellable());
    }

    #[test]
    fn test_error_event_with_all_fields() {
        let event = ErrorEvent::new("E002", "Critical failure")
            .with_source("network")
            .recoverable(true);
        assert_eq!(event.code, "E002");
        assert_eq!(event.message, "Critical failure");
        assert_eq!(event.source, Some("network".to_string()));
        assert!(event.recoverable);
    }
}
