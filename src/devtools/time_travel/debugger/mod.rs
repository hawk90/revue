//! Time-travel debugger implementation

mod render;

use super::json;
use super::{
    Action, SnapshotValue, StateDiff, StateSnapshot, TimeTravelConfig, TimeTravelImportError,
    TimeTravelView,
};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Time-travel debugger
pub struct TimeTravelDebugger {
    /// Configuration
    pub config: TimeTravelConfig,
    /// All recorded snapshots (pub for tests)
    pub(crate) snapshots: Vec<StateSnapshot>,
    /// Current position in history (index)
    position: usize,
    /// Is recording paused
    paused: bool,
    /// Next snapshot ID
    next_id: u64,
    /// Current view mode
    view: TimeTravelView,
    /// Scroll offset for lists
    scroll: usize,
    /// Selected item index (pub for tests)
    pub(crate) selected: Option<usize>,
    /// Last snapshot time (for rate limiting)
    last_snapshot: Option<Instant>,
    /// Is "traveling" (viewing past state)
    is_traveling: bool,
}

impl TimeTravelDebugger {
    /// Create new time travel debugger
    pub fn new() -> Self {
        Self {
            config: TimeTravelConfig::default(),
            snapshots: Vec::new(),
            position: 0,
            paused: false,
            next_id: 0,
            view: TimeTravelView::Timeline,
            scroll: 0,
            selected: None,
            last_snapshot: None,
            is_traveling: false,
        }
    }

    /// Set configuration
    pub fn with_config(mut self, config: TimeTravelConfig) -> Self {
        self.config = config;
        self
    }

    /// Set max snapshots
    pub fn max_snapshots(mut self, max: usize) -> Self {
        self.config.max_snapshots = max;
        self
    }

    // -------------------------------------------------------------------------
    // Recording
    // -------------------------------------------------------------------------

    /// Record a new snapshot
    pub fn record(&mut self, snapshot: StateSnapshot) {
        if self.paused {
            return;
        }

        // Rate limit if configured
        if let Some(last) = self.last_snapshot {
            if last.elapsed() < self.config.record_interval {
                return;
            }
        }

        // If we're traveling in history, truncate future snapshots
        if self.is_traveling && self.position < self.snapshots.len() {
            self.snapshots.truncate(self.position + 1);
            self.is_traveling = false;
        }

        // Assign ID
        let mut snapshot = snapshot;
        snapshot.id = self.next_id;
        self.next_id += 1;

        self.snapshots.push(snapshot);
        self.position = self.snapshots.len() - 1;
        self.last_snapshot = Some(Instant::now());

        // Trim old snapshots if over limit
        while self.snapshots.len() > self.config.max_snapshots {
            self.snapshots.remove(0);
            if self.position > 0 {
                self.position -= 1;
            }
        }
    }

    /// Record state with action
    pub fn record_action(&mut self, action: Action, state: HashMap<String, SnapshotValue>) {
        let snapshot = StateSnapshot {
            id: 0,
            timestamp: std::time::SystemTime::now(),
            state,
            action: Some(action),
            label: None,
        };
        self.record(snapshot);
    }

    /// Pause recording
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume recording
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Toggle recording
    pub fn toggle_recording(&mut self) {
        self.paused = !self.paused;
    }

    /// Is recording paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Clear all snapshots
    pub fn clear(&mut self) {
        self.snapshots.clear();
        self.position = 0;
        self.is_traveling = false;
        self.next_id = 0;
    }

    // -------------------------------------------------------------------------
    // Navigation
    // -------------------------------------------------------------------------

    /// Get current snapshot
    pub fn current(&self) -> Option<&StateSnapshot> {
        self.snapshots.get(self.position)
    }

    /// Get snapshot at index
    pub fn get(&self, index: usize) -> Option<&StateSnapshot> {
        self.snapshots.get(index)
    }

    /// Get all snapshots
    pub fn snapshots(&self) -> &[StateSnapshot] {
        &self.snapshots
    }

    /// Get current position
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get total snapshot count
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }

    /// Step backward one snapshot
    pub fn step_back(&mut self) {
        if self.position > 0 {
            self.position -= 1;
            self.is_traveling = true;
        }
    }

    /// Step forward one snapshot
    pub fn step_forward(&mut self) {
        if self.position < self.snapshots.len().saturating_sub(1) {
            self.position += 1;
        }
        if self.position == self.snapshots.len().saturating_sub(1) {
            self.is_traveling = false;
        }
    }

    /// Jump to specific snapshot
    pub fn jump_to(&mut self, index: usize) {
        if index < self.snapshots.len() {
            self.position = index;
            self.is_traveling = index < self.snapshots.len().saturating_sub(1);
        }
    }

    /// Jump to latest snapshot
    pub fn jump_to_latest(&mut self) {
        if !self.snapshots.is_empty() {
            self.position = self.snapshots.len() - 1;
            self.is_traveling = false;
        }
    }

    /// Jump to first snapshot
    pub fn jump_to_first(&mut self) {
        if !self.snapshots.is_empty() {
            self.position = 0;
            self.is_traveling = true;
        }
    }

    /// Is currently traveling in history
    pub fn is_traveling(&self) -> bool {
        self.is_traveling
    }

    // -------------------------------------------------------------------------
    // Diff
    // -------------------------------------------------------------------------

    /// Get diff between current and previous snapshot
    pub fn current_diff(&self) -> Option<StateDiff> {
        if self.position == 0 || self.snapshots.is_empty() {
            return None;
        }

        let current = &self.snapshots[self.position];
        let previous = &self.snapshots[self.position - 1];
        Some(current.diff(previous))
    }

    /// Get diff between two positions
    pub fn diff_between(&self, from: usize, to: usize) -> Option<StateDiff> {
        let from_snapshot = self.snapshots.get(from)?;
        let to_snapshot = self.snapshots.get(to)?;
        Some(to_snapshot.diff(from_snapshot))
    }

    // -------------------------------------------------------------------------
    // Export/Import
    // -------------------------------------------------------------------------

    /// Export session history as JSON string
    ///
    /// Each snapshot has its `id`, `label`, `timestamp_ms` (milliseconds since
    /// the Unix epoch), its `state`, and the action that caused it: `action`
    /// (the name), `action_payload`, `action_source`, `action_timestamp_ms`
    /// and `action_duration_ms`. `state_keys` counts the state entries.
    /// [`import_json`](Self::import_json) reads it back.
    ///
    /// Timestamps keep millisecond precision, and a float that is NaN or
    /// infinite is written as `null`; everything else round-trips exactly.
    pub fn export(&self) -> String {
        let mut out = String::from("{\n");
        out.push_str(&format!(
            "  \"snapshot_count\": {},\n",
            self.snapshots.len()
        ));
        out.push_str(&format!("  \"current_position\": {},\n", self.position));
        out.push_str("  \"snapshots\": [\n");

        for (i, snapshot) in self.snapshots.iter().enumerate() {
            let mut fields = vec![
                format!("\"id\": {}", snapshot.id),
                format!("\"timestamp_ms\": {}", millis(snapshot.timestamp)),
            ];
            if let Some(label) = &snapshot.label {
                fields.push(format!("\"label\": {}", json::string(label)));
            }
            if let Some(action) = &snapshot.action {
                fields.push(format!("\"action\": {}", json::string(&action.name)));
                if let Some(payload) = &action.payload {
                    fields.push(format!("\"action_payload\": {}", json::value(payload)));
                }
                if let Some(source) = &action.source {
                    fields.push(format!("\"action_source\": {}", json::string(source)));
                }
                fields.push(format!(
                    "\"action_timestamp_ms\": {}",
                    millis(action.timestamp)
                ));
                if let Some(duration) = action.duration {
                    fields.push(format!("\"action_duration_ms\": {}", duration.as_millis()));
                }
            }
            fields.push(format!("\"state_keys\": {}", snapshot.state.len()));
            fields.push(format!(
                "\"state\": {}",
                json::value(&SnapshotValue::Object(snapshot.state.clone()))
            ));

            out.push_str("    {\n      ");
            out.push_str(&fields.join(",\n      "));
            out.push_str("\n    }");
            if i + 1 < self.snapshots.len() {
                out.push(',');
            }
            out.push('\n');
        }

        out.push_str("  ]\n");
        out.push_str("}\n");
        out
    }

    /// Import session from exported data
    pub fn import(&mut self, snapshots: Vec<StateSnapshot>) {
        self.clear();
        for snapshot in snapshots {
            self.snapshots.push(snapshot);
        }
        if !self.snapshots.is_empty() {
            self.position = self.snapshots.len() - 1;
            self.next_id = self.snapshots.iter().map(|s| s.id).max().unwrap_or(0) + 1;
        }
    }

    /// Replace the history with a session saved by [`export`](Self::export),
    /// and return how many snapshots it has.
    ///
    /// The position is restored too. Only `id` is required of a snapshot;
    /// without `state` it is empty, without `timestamp_ms` it is the epoch.
    /// On an error the history is left as it was.
    ///
    /// ```
    /// use revue::devtools::{SnapshotValue, StateSnapshot, TimeTravelDebugger};
    ///
    /// let mut debugger = TimeTravelDebugger::new();
    /// debugger.record(StateSnapshot::new(0).with_state("count", SnapshotValue::Int(1)));
    /// let saved = debugger.export();
    ///
    /// let mut restored = TimeTravelDebugger::new();
    /// assert_eq!(restored.import_json(&saved).unwrap(), 1);
    /// assert_eq!(restored.snapshots()[0].state["count"], SnapshotValue::Int(1));
    /// ```
    pub fn import_json(&mut self, text: &str) -> Result<usize, TimeTravelImportError> {
        let root = json::parse(text).map_err(TimeTravelImportError::new)?;
        let SnapshotValue::Object(mut root) = root else {
            return Err(TimeTravelImportError::new(
                "the session is not a JSON object",
            ));
        };
        let Some(SnapshotValue::Array(items)) = root.remove("snapshots") else {
            return Err(TimeTravelImportError::new(
                "`snapshots` is missing or not an array",
            ));
        };

        let mut snapshots = Vec::with_capacity(items.len());
        for (i, item) in items.into_iter().enumerate() {
            snapshots.push(
                snapshot_from(item)
                    .map_err(|e| TimeTravelImportError::new(format!("snapshot {i}: {e}")))?,
            );
        }

        let position = match root.get("current_position") {
            Some(SnapshotValue::Int(p)) => usize::try_from(*p).ok(),
            _ => None,
        };
        let count = snapshots.len();
        self.import(snapshots);
        if let Some(p) = position.filter(|&p| p < count) {
            self.position = p;
        }
        Ok(count)
    }

    // -------------------------------------------------------------------------
    // View
    // -------------------------------------------------------------------------

    /// Set view mode
    pub fn set_view(&mut self, view: TimeTravelView) {
        self.view = view;
        self.scroll = 0;
        self.selected = None;
    }

    /// Get current view
    pub fn view(&self) -> TimeTravelView {
        self.view
    }

    /// Next view
    pub fn next_view(&mut self) {
        self.view = self.view.next();
        self.scroll = 0;
    }

    /// Select next item
    pub fn select_next(&mut self) {
        let count = match self.view {
            TimeTravelView::Timeline | TimeTravelView::Actions => self.snapshots.len(),
            TimeTravelView::State => self.current().map(|s| s.state.len()).unwrap_or(0),
            TimeTravelView::Diff => self.current_diff().map(|d| d.count()).unwrap_or(0),
        };

        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => (i + 1).min(count - 1),
            None => 0,
        });
    }

    /// Select previous item
    pub fn select_prev(&mut self) {
        if let Some(i) = self.selected {
            self.selected = Some(i.saturating_sub(1));
        }
    }
}

impl Default for TimeTravelDebugger {
    fn default() -> Self {
        Self::new()
    }
}

/// Milliseconds from the Unix epoch to `time` (0 before it).
fn millis(time: SystemTime) -> u128 {
    time.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis())
}

/// The time `ms` milliseconds after the Unix epoch.
fn from_millis(ms: i64) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(u64::try_from(ms).unwrap_or(0))
}

/// One exported snapshot object back as a [`StateSnapshot`].
fn snapshot_from(item: SnapshotValue) -> Result<StateSnapshot, String> {
    let SnapshotValue::Object(mut fields) = item else {
        return Err("not a JSON object".into());
    };
    let int = |fields: &HashMap<String, SnapshotValue>, key: &str| -> Result<Option<i64>, String> {
        match fields.get(key) {
            None => Ok(None),
            Some(SnapshotValue::Int(n)) => Ok(Some(*n)),
            Some(_) => Err(format!("`{key}` is not an integer")),
        }
    };
    let text = |fields: &mut HashMap<String, SnapshotValue>,
                key: &str|
     -> Result<Option<String>, String> {
        match fields.remove(key) {
            None => Ok(None),
            Some(SnapshotValue::String(s)) => Ok(Some(s)),
            Some(_) => Err(format!("`{key}` is not a string")),
        }
    };

    let id = int(&fields, "id")?.ok_or("`id` is missing")?;
    let id = u64::try_from(id).map_err(|_| "`id` is negative".to_string())?;
    let mut snapshot = StateSnapshot::new(id);
    snapshot.timestamp = from_millis(int(&fields, "timestamp_ms")?.unwrap_or(0));
    snapshot.label = text(&mut fields, "label")?;
    snapshot.state = match fields.remove("state") {
        None => HashMap::new(),
        Some(SnapshotValue::Object(state)) => state,
        Some(_) => return Err("`state` is not an object".into()),
    };
    if let Some(name) = text(&mut fields, "action")? {
        let mut action = Action::new(name);
        action.payload = fields.remove("action_payload");
        action.source = text(&mut fields, "action_source")?;
        action.timestamp = from_millis(int(&fields, "action_timestamp_ms")?.unwrap_or(0));
        action.duration = int(&fields, "action_duration_ms")?
            .map(|ms| Duration::from_millis(u64::try_from(ms).unwrap_or(0)));
        snapshot.action = Some(action);
    }
    Ok(snapshot)
}
