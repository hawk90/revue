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
