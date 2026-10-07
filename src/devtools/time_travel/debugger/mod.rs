//! Time-travel debugger implementation

mod render;

use super::{Action, SnapshotValue, StateDiff, StateSnapshot, TimeTravelConfig, TimeTravelView};
use std::collections::HashMap;
use std::time::Instant;

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
    pub fn export(&self) -> String {
        let mut json = String::from("{\n");
        json.push_str(&format!(
            "  \"snapshot_count\": {},\n",
            self.snapshots.len()
        ));
        json.push_str(&format!("  \"current_position\": {},\n", self.position));
        json.push_str("  \"snapshots\": [\n");

        for (i, snapshot) in self.snapshots.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"id\": {},\n", snapshot.id));
            if let Some(label) = &snapshot.label {
                json.push_str(&format!("      \"label\": {},\n", json_string(label)));
            }
            if let Some(action) = &snapshot.action {
                json.push_str(&format!(
                    "      \"action\": {},\n",
                    json_string(&action.name)
                ));
            }
            json.push_str(&format!("      \"state_keys\": {}\n", snapshot.state.len()));
            json.push_str("    }");
            if i < self.snapshots.len() - 1 {
                json.push(',');
            }
            json.push('\n');
        }

        json.push_str("  ]\n");
        json.push_str("}\n");
        json
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

/// `s` as a quoted JSON string: quotes, backslashes and control characters
/// are escaped, so a label or action name cannot break the exported JSON
fn json_string(s: &str) -> String {
    use std::fmt::Write;

    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
