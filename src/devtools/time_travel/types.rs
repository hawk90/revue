//! Types for time-travel debugging

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// A snapshot of application state at a point in time
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    /// Unique snapshot ID
    pub id: u64,
    /// Timestamp when snapshot was taken
    pub timestamp: SystemTime,
    /// State data as key-value pairs
    pub state: HashMap<String, SnapshotValue>,
    /// Action that caused this snapshot
    pub action: Option<Action>,
    /// Optional label for the snapshot
    pub label: Option<String>,
}

impl StateSnapshot {
    /// Create a new snapshot
    pub fn new(id: u64) -> Self {
        Self {
            id,
            timestamp: SystemTime::now(),
            state: HashMap::new(),
            action: None,
            label: None,
        }
    }

    /// Add state value
    pub fn with_state(mut self, key: impl Into<String>, value: SnapshotValue) -> Self {
        self.state.insert(key.into(), value);
        self
    }

    /// Set action that caused this snapshot
    pub fn with_action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    /// Set label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get diff from another snapshot
    pub fn diff(&self, other: &StateSnapshot) -> StateDiff {
        let mut added = HashMap::new();
        let mut removed = HashMap::new();
        let mut changed = HashMap::new();

        // Find added and changed
        for (key, value) in &self.state {
            match other.state.get(key) {
                None => {
                    added.insert(key.clone(), value.clone());
                }
                Some(old_value) if old_value != value => {
                    changed.insert(key.clone(), (old_value.clone(), value.clone()));
                }
                _ => {}
            }
        }

        // Find removed
        for (key, value) in &other.state {
            if !self.state.contains_key(key) {
                removed.insert(key.clone(), value.clone());
            }
        }

        StateDiff {
            added,
            removed,
            changed,
        }
    }
}

/// Value stored in a snapshot
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotValue {
    /// Null value
    Null,
    /// Boolean
    Bool(bool),
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// String
    String(String),
    /// Array of values
    Array(Vec<SnapshotValue>),
    /// Object/map
    Object(HashMap<String, SnapshotValue>),
}

impl SnapshotValue {
    /// Display value as string
    pub fn display(&self) -> String {
        match self {
            Self::Null => "null".to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Int(i) => i.to_string(),
            Self::Float(f) => format!("{:.2}", f),
            Self::String(s) => format!("\"{}\"", s),
            Self::Array(arr) => format!("[{} items]", arr.len()),
            Self::Object(obj) => format!("{{{} keys}}", obj.len()),
        }
    }

    /// Get type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "bool",
            Self::Int(_) => "i64",
            Self::Float(_) => "f64",
            Self::String(_) => "String",
            Self::Array(_) => "Array",
            Self::Object(_) => "Object",
        }
    }
}

impl From<bool> for SnapshotValue {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<i32> for SnapshotValue {
    fn from(v: i32) -> Self {
        Self::Int(v as i64)
    }
}

impl From<i64> for SnapshotValue {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}

impl From<f64> for SnapshotValue {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl From<&str> for SnapshotValue {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<String> for SnapshotValue {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

/// Difference between two snapshots
#[derive(Debug, Clone, Default)]
pub struct StateDiff {
    /// Keys added
    pub added: HashMap<String, SnapshotValue>,
    /// Keys removed
    pub removed: HashMap<String, SnapshotValue>,
    /// Keys changed (old, new)
    pub changed: HashMap<String, (SnapshotValue, SnapshotValue)>,
}

impl StateDiff {
    /// Check if there are no changes
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }

    /// Total number of changes
    pub fn count(&self) -> usize {
        self.added.len() + self.removed.len() + self.changed.len()
    }
}

/// An action that caused a state change
#[derive(Debug, Clone)]
pub struct Action {
    /// Action type/name
    pub name: String,
    /// Action payload/data
    pub payload: Option<SnapshotValue>,
    /// Source component or location
    pub source: Option<String>,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Duration of action (if measured)
    pub duration: Option<Duration>,
}

impl Action {
    /// Create a new action
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            payload: None,
            source: None,
            timestamp: SystemTime::now(),
            duration: None,
        }
    }

    /// Set payload
    pub fn with_payload(mut self, payload: SnapshotValue) -> Self {
        self.payload = Some(payload);
        self
    }

    /// Set source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

/// View mode for time travel debugger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeTravelView {
    /// Timeline view with history slider
    #[default]
    Timeline,
    /// Diff view showing changes
    Diff,
    /// Action log
    Actions,
    /// State inspector at current position
    State,
}

impl TimeTravelView {
    /// Get view label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Timeline => "Timeline",
            Self::Diff => "Diff",
            Self::Actions => "Actions",
            Self::State => "State",
        }
    }

    /// Get all views
    pub fn all() -> &'static [TimeTravelView] {
        &[
            TimeTravelView::Timeline,
            TimeTravelView::Diff,
            TimeTravelView::Actions,
            TimeTravelView::State,
        ]
    }

    /// Next view
    pub fn next(&self) -> Self {
        match self {
            Self::Timeline => Self::Diff,
            Self::Diff => Self::Actions,
            Self::Actions => Self::State,
            Self::State => Self::Timeline,
        }
    }
}

/// Configuration for time travel debugger
#[derive(Debug, Clone)]
pub struct TimeTravelConfig {
    /// Maximum number of snapshots to keep
    pub max_snapshots: usize,
    /// Auto-record state changes
    pub auto_record: bool,
    /// Record interval (minimum time between auto-snapshots)
    pub record_interval: Duration,
}

impl Default for TimeTravelConfig {
    fn default() -> Self {
        Self {
            max_snapshots: 100,
            auto_record: true,
            record_interval: Duration::from_millis(100),
        }
    }
}
