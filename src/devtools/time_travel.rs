//! Time-Travel Debugging for Revue applications
//!
//! Records state snapshots and allows stepping through history.
//!
//! # Features
//!
//! - Automatic state snapshot recording
//! - Step forward/backward through history
//! - Jump to specific snapshot
//! - Action/event log with timestamps
//! - State diff visualization
//! - Export/import session history
//! - Configurable snapshot limits
//! - Pause/resume recording

use super::helpers::draw_text_overlay;
use super::DevToolsConfig;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};

// =============================================================================
// State Snapshot
// =============================================================================

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

// =============================================================================
// Snapshot Value
// =============================================================================

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

// =============================================================================
// State Diff
// =============================================================================

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

// =============================================================================
// Action
// =============================================================================

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

// =============================================================================
// Time Travel View
// =============================================================================

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

// =============================================================================
// Time Travel Debugger
// =============================================================================

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

/// Time-travel debugger
pub struct TimeTravelDebugger {
    /// Configuration
    config: TimeTravelConfig,
    /// All recorded snapshots
    snapshots: Vec<StateSnapshot>,
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
    /// Selected item index
    selected: Option<usize>,
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
            timestamp: SystemTime::now(),
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
                json.push_str(&format!("      \"label\": \"{}\",\n", label));
            }
            if let Some(action) = &snapshot.action {
                json.push_str(&format!("      \"action\": \"{}\",\n", action.name));
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

    // -------------------------------------------------------------------------
    // Rendering
    // -------------------------------------------------------------------------

    /// Render time travel debugger content
    pub fn render_content(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y + area.height;

        // Header with status
        let status = if self.paused { "⏸ PAUSED" } else { "● REC" };
        let header = format!(
            "{} | {} snapshots | pos {}/{}",
            status,
            self.snapshots.len(),
            self.position + 1,
            self.snapshots.len()
        );
        Self::draw_text(buffer, area.x, y, &header, config.accent_color);
        y += 1;

        // View tabs
        self.render_view_tabs(buffer, area.x, y, area.width, config);
        y += 2;

        if y >= max_y {
            return;
        }

        // Content based on view
        let content_area = Rect::new(area.x, y, area.width, max_y.saturating_sub(y));

        match self.view {
            TimeTravelView::Timeline => self.render_timeline(buffer, content_area, config),
            TimeTravelView::Diff => self.render_diff(buffer, content_area, config),
            TimeTravelView::Actions => self.render_actions(buffer, content_area, config),
            TimeTravelView::State => self.render_state(buffer, content_area, config),
        }
    }

    fn render_view_tabs(
        &self,
        buffer: &mut Buffer,
        x: u16,
        y: u16,
        width: u16,
        config: &DevToolsConfig,
    ) {
        let mut px = x;
        for view in TimeTravelView::all() {
            let label = format!(" {} ", view.label());
            let is_active = *view == self.view;

            let (fg, bg) = if is_active {
                (config.bg_color, config.accent_color)
            } else {
                (config.fg_color, config.bg_color)
            };

            for ch in label.chars() {
                if px < x + width {
                    if let Some(cell) = buffer.get_mut(px, y) {
                        cell.symbol = ch;
                        cell.fg = Some(fg);
                        cell.bg = Some(bg);
                    }
                    px += 1;
                }
            }
            px += 1;
        }
    }

    fn render_timeline(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y + area.height;

        if self.snapshots.is_empty() {
            Self::draw_text(buffer, area.x, y, "No snapshots recorded", config.fg_color);
            return;
        }

        // Draw timeline slider
        let slider_width = area.width.saturating_sub(4);
        if slider_width > 0 && self.snapshots.len() > 1 {
            let progress = self.position as f32 / (self.snapshots.len() - 1) as f32;
            let filled = (slider_width as f32 * progress) as u16;

            Self::draw_text(buffer, area.x, y, "[", config.fg_color);
            for i in 0..slider_width {
                let ch = if i == filled { '●' } else { '─' };
                let color = if i <= filled {
                    config.accent_color
                } else {
                    config.fg_color
                };
                if let Some(cell) = buffer.get_mut(area.x + 1 + i, y) {
                    cell.symbol = ch;
                    cell.fg = Some(color);
                }
            }
            Self::draw_text(buffer, area.x + 1 + slider_width, y, "]", config.fg_color);
            y += 2;
        }

        // List recent snapshots
        let start = self.scroll;
        for (i, snapshot) in self.snapshots.iter().enumerate().skip(start) {
            if y >= max_y {
                break;
            }

            let is_current = i == self.position;
            let marker = if is_current { "▸" } else { " " };
            let action_name = snapshot
                .action
                .as_ref()
                .map(|a| a.name.as_str())
                .unwrap_or("snapshot");
            let label = snapshot.label.as_deref().unwrap_or("");

            let line = if label.is_empty() {
                format!("{} #{}: {}", marker, snapshot.id, action_name)
            } else {
                format!("{} #{}: {} ({})", marker, snapshot.id, action_name, label)
            };

            let color = if is_current {
                config.accent_color
            } else {
                config.fg_color
            };
            Self::draw_text(buffer, area.x, y, &line, color);
            y += 1;
        }
    }

    fn render_diff(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y + area.height;

        let diff = match self.current_diff() {
            Some(d) => d,
            None => {
                Self::draw_text(
                    buffer,
                    area.x,
                    y,
                    "No previous snapshot to compare",
                    config.fg_color,
                );
                return;
            }
        };

        if diff.is_empty() {
            Self::draw_text(buffer, area.x, y, "No changes", config.fg_color);
            return;
        }

        // Added
        let added_color = Color::rgb(100, 200, 100);
        for (key, value) in &diff.added {
            if y >= max_y {
                break;
            }
            let line = format!("+ {}: {}", key, value.display());
            Self::draw_text(buffer, area.x, y, &line, added_color);
            y += 1;
        }

        // Removed
        let removed_color = Color::rgb(200, 100, 100);
        for (key, value) in &diff.removed {
            if y >= max_y {
                break;
            }
            let line = format!("- {}: {}", key, value.display());
            Self::draw_text(buffer, area.x, y, &line, removed_color);
            y += 1;
        }

        // Changed
        let changed_color = Color::rgb(200, 200, 100);
        for (key, (old, new)) in &diff.changed {
            if y >= max_y {
                break;
            }
            let line = format!("~ {}: {} → {}", key, old.display(), new.display());
            Self::draw_text(buffer, area.x, y, &line, changed_color);
            y += 1;
        }
    }

    fn render_actions(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y + area.height;

        let actions: Vec<_> = self
            .snapshots
            .iter()
            .filter_map(|s| s.action.as_ref().map(|a| (s.id, a)))
            .collect();

        if actions.is_empty() {
            Self::draw_text(buffer, area.x, y, "No actions recorded", config.fg_color);
            return;
        }

        for (id, action) in actions.iter().skip(self.scroll) {
            if y >= max_y {
                break;
            }

            let source = action.source.as_deref().unwrap_or("");
            let line = if source.is_empty() {
                format!("#{}: {}", id, action.name)
            } else {
                format!("#{}: {} ({})", id, action.name, source)
            };

            Self::draw_text(buffer, area.x, y, &line, config.fg_color);
            y += 1;
        }
    }

    fn render_state(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y + area.height;

        let snapshot = match self.current() {
            Some(s) => s,
            None => {
                Self::draw_text(buffer, area.x, y, "No snapshot selected", config.fg_color);
                return;
            }
        };

        if snapshot.state.is_empty() {
            Self::draw_text(buffer, area.x, y, "Empty state", config.fg_color);
            return;
        }

        let mut entries: Vec<_> = snapshot.state.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));

        for (key, value) in entries.iter().skip(self.scroll) {
            if y >= max_y {
                break;
            }

            let line = format!("{}: {} ({})", key, value.display(), value.type_name());
            Self::draw_text(buffer, area.x, y, &line, config.fg_color);
            y += 1;
        }
    }

    fn draw_text(buffer: &mut Buffer, x: u16, y: u16, text: &str, color: Color) {
        draw_text_overlay(buffer, x, y, text, color);
    }
}

impl Default for TimeTravelDebugger {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let snapshot = StateSnapshot::new(1)
            .with_state("count", SnapshotValue::Int(42))
            .with_label("initial");

        assert_eq!(snapshot.id, 1);
        assert_eq!(snapshot.label, Some("initial".to_string()));
        assert!(snapshot.state.contains_key("count"));
    }

    #[test]
    fn test_snapshot_diff() {
        let mut state1 = HashMap::new();
        state1.insert("a".to_string(), SnapshotValue::Int(1));
        state1.insert("b".to_string(), SnapshotValue::Int(2));

        let mut state2 = HashMap::new();
        state2.insert("a".to_string(), SnapshotValue::Int(1));
        state2.insert("b".to_string(), SnapshotValue::Int(3)); // changed
        state2.insert("c".to_string(), SnapshotValue::Int(4)); // added

        let snap1 = StateSnapshot {
            id: 1,
            timestamp: SystemTime::now(),
            state: state1,
            action: None,
            label: None,
        };

        let snap2 = StateSnapshot {
            id: 2,
            timestamp: SystemTime::now(),
            state: state2,
            action: None,
            label: None,
        };

        let diff = snap2.diff(&snap1);
        assert_eq!(diff.added.len(), 1);
        assert!(diff.added.contains_key("c"));
        assert_eq!(diff.changed.len(), 1);
        assert!(diff.changed.contains_key("b"));
        assert!(diff.removed.is_empty());
    }

    #[test]
    fn test_time_travel_record() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO; // Disable rate limiting

        tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(1)));
        tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(2)));
        tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(3)));

        assert_eq!(tt.count(), 3);
        assert_eq!(tt.position(), 2);
    }

    #[test]
    fn test_time_travel_navigation() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO; // Disable rate limiting

        tt.record(StateSnapshot::new(0).with_label("first"));
        tt.record(StateSnapshot::new(0).with_label("second"));
        tt.record(StateSnapshot::new(0).with_label("third"));

        assert_eq!(tt.position(), 2);

        tt.step_back();
        assert_eq!(tt.position(), 1);
        assert!(tt.is_traveling());

        tt.step_back();
        assert_eq!(tt.position(), 0);

        tt.step_forward();
        assert_eq!(tt.position(), 1);

        tt.jump_to_latest();
        assert_eq!(tt.position(), 2);
        assert!(!tt.is_traveling());
    }

    #[test]
    fn test_time_travel_pause() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO; // Disable rate limiting

        tt.record(StateSnapshot::new(0));
        assert_eq!(tt.count(), 1);

        tt.pause();
        tt.record(StateSnapshot::new(0));
        assert_eq!(tt.count(), 1); // Still 1, recording paused

        tt.resume();
        tt.record(StateSnapshot::new(0));
        assert_eq!(tt.count(), 2);
    }

    #[test]
    fn test_time_travel_max_snapshots() {
        let mut tt = TimeTravelDebugger::new().max_snapshots(3);
        tt.config.record_interval = Duration::ZERO;

        for i in 0..5 {
            tt.record(StateSnapshot::new(0).with_state("i", SnapshotValue::Int(i)));
        }

        assert_eq!(tt.count(), 3);
        // Should have kept the last 3
        assert_eq!(tt.snapshots[0].state.get("i"), Some(&SnapshotValue::Int(2)));
    }

    #[test]
    fn test_time_travel_branch_on_travel() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0).with_label("a"));
        tt.record(StateSnapshot::new(0).with_label("b"));
        tt.record(StateSnapshot::new(0).with_label("c"));

        // Go back and create new branch
        tt.step_back();
        tt.step_back();
        assert_eq!(tt.position(), 0);

        tt.record(StateSnapshot::new(0).with_label("d"));

        // Should have truncated b and c, now have a and d
        assert_eq!(tt.count(), 2);
        assert_eq!(tt.current().unwrap().label, Some("d".to_string()));
    }

    #[test]
    fn test_action_creation() {
        let action = Action::new("click")
            .with_source("Button")
            .with_payload(SnapshotValue::String("submit".to_string()));

        assert_eq!(action.name, "click");
        assert_eq!(action.source, Some("Button".to_string()));
    }

    #[test]
    fn test_snapshot_value_conversions() {
        assert_eq!(SnapshotValue::from(true), SnapshotValue::Bool(true));
        assert_eq!(SnapshotValue::from(42i32), SnapshotValue::Int(42));
        assert_eq!(SnapshotValue::from(3.14), SnapshotValue::Float(3.14));
        assert_eq!(
            SnapshotValue::from("hello"),
            SnapshotValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_view_cycle() {
        let view = TimeTravelView::Timeline;
        assert_eq!(view.next(), TimeTravelView::Diff);
        assert_eq!(view.next().next(), TimeTravelView::Actions);
        assert_eq!(view.next().next().next(), TimeTravelView::State);
        assert_eq!(view.next().next().next().next(), TimeTravelView::Timeline);
    }

    #[test]
    fn test_export() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(
            StateSnapshot::new(0)
                .with_label("test")
                .with_action(Action::new("init")),
        );

        let json = tt.export();
        assert!(json.contains("snapshot_count"));
        assert!(json.contains("\"label\": \"test\""));
        assert!(json.contains("\"action\": \"init\""));
    }

    #[test]
    fn test_clear() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));
        assert_eq!(tt.count(), 2);

        tt.clear();
        assert_eq!(tt.count(), 0);
        assert_eq!(tt.position(), 0);
    }

    #[test]
    fn test_snapshot_value_display() {
        assert_eq!(SnapshotValue::Null.display(), "null");
        assert_eq!(SnapshotValue::Bool(true).display(), "true");
        assert_eq!(SnapshotValue::Bool(false).display(), "false");
        assert_eq!(SnapshotValue::Int(42).display(), "42");
        assert_eq!(SnapshotValue::Float(3.14159).display(), "3.14");
        assert_eq!(
            SnapshotValue::String("hello".to_string()).display(),
            "\"hello\""
        );
        assert_eq!(
            SnapshotValue::Array(vec![SnapshotValue::Int(1), SnapshotValue::Int(2)]).display(),
            "[2 items]"
        );
        let mut obj = HashMap::new();
        obj.insert("a".to_string(), SnapshotValue::Int(1));
        assert_eq!(SnapshotValue::Object(obj).display(), "{1 keys}");
    }

    #[test]
    fn test_snapshot_value_type_name() {
        assert_eq!(SnapshotValue::Null.type_name(), "null");
        assert_eq!(SnapshotValue::Bool(true).type_name(), "bool");
        assert_eq!(SnapshotValue::Int(42).type_name(), "i64");
        assert_eq!(SnapshotValue::Float(3.14).type_name(), "f64");
        assert_eq!(
            SnapshotValue::String("test".to_string()).type_name(),
            "String"
        );
        assert_eq!(SnapshotValue::Array(vec![]).type_name(), "Array");
        assert_eq!(SnapshotValue::Object(HashMap::new()).type_name(), "Object");
    }

    #[test]
    fn test_snapshot_value_from_string_owned() {
        let s = String::from("owned");
        let val: SnapshotValue = s.into();
        assert_eq!(val, SnapshotValue::String("owned".to_string()));
    }

    #[test]
    fn test_snapshot_value_from_i64() {
        let val: SnapshotValue = 123i64.into();
        assert_eq!(val, SnapshotValue::Int(123));
    }

    #[test]
    fn test_state_diff_is_empty() {
        let diff = StateDiff::default();
        assert!(diff.is_empty());
        assert_eq!(diff.count(), 0);
    }

    #[test]
    fn test_state_diff_count() {
        let mut diff = StateDiff::default();
        diff.added.insert("a".to_string(), SnapshotValue::Int(1));
        diff.removed.insert("b".to_string(), SnapshotValue::Int(2));
        diff.changed.insert(
            "c".to_string(),
            (SnapshotValue::Int(3), SnapshotValue::Int(4)),
        );

        assert!(!diff.is_empty());
        assert_eq!(diff.count(), 3);
    }

    #[test]
    fn test_time_travel_view_default() {
        let view = TimeTravelView::default();
        assert_eq!(view, TimeTravelView::Timeline);
    }

    #[test]
    fn test_time_travel_view_labels() {
        assert_eq!(TimeTravelView::Timeline.label(), "Timeline");
        assert_eq!(TimeTravelView::Diff.label(), "Diff");
        assert_eq!(TimeTravelView::Actions.label(), "Actions");
        assert_eq!(TimeTravelView::State.label(), "State");
    }

    #[test]
    fn test_time_travel_view_all() {
        let views = TimeTravelView::all();
        assert_eq!(views.len(), 4);
        assert!(views.contains(&TimeTravelView::Timeline));
        assert!(views.contains(&TimeTravelView::Diff));
        assert!(views.contains(&TimeTravelView::Actions));
        assert!(views.contains(&TimeTravelView::State));
    }

    #[test]
    fn test_time_travel_config_default() {
        let config = TimeTravelConfig::default();
        assert_eq!(config.max_snapshots, 100);
        assert!(config.auto_record);
        assert_eq!(config.record_interval, Duration::from_millis(100));
    }

    #[test]
    fn test_time_travel_debugger_default() {
        let tt = TimeTravelDebugger::default();
        assert_eq!(tt.count(), 0);
        assert_eq!(tt.position(), 0);
        assert!(!tt.is_paused());
    }

    #[test]
    fn test_time_travel_debugger_with_config() {
        let config = TimeTravelConfig {
            max_snapshots: 50,
            auto_record: false,
            record_interval: Duration::from_millis(200),
        };
        let tt = TimeTravelDebugger::new().with_config(config);
        assert_eq!(tt.config.max_snapshots, 50);
    }

    #[test]
    fn test_time_travel_set_view() {
        let mut tt = TimeTravelDebugger::new();
        assert_eq!(tt.view(), TimeTravelView::Timeline);

        tt.set_view(TimeTravelView::State);
        assert_eq!(tt.view(), TimeTravelView::State);
    }

    #[test]
    fn test_time_travel_next_view() {
        let mut tt = TimeTravelDebugger::new();
        assert_eq!(tt.view(), TimeTravelView::Timeline);

        tt.next_view();
        assert_eq!(tt.view(), TimeTravelView::Diff);

        tt.next_view();
        assert_eq!(tt.view(), TimeTravelView::Actions);

        tt.next_view();
        assert_eq!(tt.view(), TimeTravelView::State);

        tt.next_view();
        assert_eq!(tt.view(), TimeTravelView::Timeline);
    }

    #[test]
    fn test_time_travel_toggle_recording() {
        let mut tt = TimeTravelDebugger::new();
        assert!(!tt.is_paused());

        tt.toggle_recording();
        assert!(tt.is_paused());

        tt.toggle_recording();
        assert!(!tt.is_paused());
    }

    #[test]
    fn test_time_travel_current_empty() {
        let tt = TimeTravelDebugger::new();
        assert!(tt.current().is_none());
    }

    #[test]
    fn test_time_travel_get_snapshot() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0).with_label("first"));
        tt.record(StateSnapshot::new(0).with_label("second"));

        assert!(tt.get(0).is_some());
        assert_eq!(tt.get(0).unwrap().label, Some("first".to_string()));
        assert!(tt.get(1).is_some());
        assert!(tt.get(5).is_none());
    }

    #[test]
    fn test_time_travel_snapshots() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));

        assert_eq!(tt.snapshots().len(), 2);
    }

    #[test]
    fn test_time_travel_jump_to_first() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));

        assert_eq!(tt.position(), 2);

        tt.jump_to_first();
        assert_eq!(tt.position(), 0);
        assert!(tt.is_traveling());
    }

    #[test]
    fn test_time_travel_jump_to() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));

        tt.jump_to(1);
        assert_eq!(tt.position(), 1);
        assert!(tt.is_traveling());

        // Out of bounds should not change
        tt.jump_to(100);
        assert_eq!(tt.position(), 1);
    }

    #[test]
    fn test_time_travel_current_diff_empty() {
        let tt = TimeTravelDebugger::new();
        assert!(tt.current_diff().is_none());
    }

    #[test]
    fn test_time_travel_current_diff_first_snapshot() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;
        tt.record(StateSnapshot::new(0));

        // At position 0, there's no previous snapshot
        assert!(tt.current_diff().is_none());
    }

    #[test]
    fn test_time_travel_current_diff() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(1)));
        tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(2)));

        let diff = tt.current_diff();
        assert!(diff.is_some());
        let diff = diff.unwrap();
        assert!(diff.changed.contains_key("x"));
    }

    #[test]
    fn test_time_travel_diff_between() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0).with_state("a", SnapshotValue::Int(1)));
        tt.record(StateSnapshot::new(0).with_state("a", SnapshotValue::Int(2)));
        tt.record(StateSnapshot::new(0).with_state("a", SnapshotValue::Int(3)));

        let diff = tt.diff_between(0, 2);
        assert!(diff.is_some());
        let diff = diff.unwrap();
        assert!(diff.changed.contains_key("a"));

        // Out of bounds
        assert!(tt.diff_between(0, 100).is_none());
        assert!(tt.diff_between(100, 0).is_none());
    }

    #[test]
    fn test_time_travel_import() {
        let mut tt = TimeTravelDebugger::new();

        let snapshots = vec![
            StateSnapshot::new(5),
            StateSnapshot::new(10),
            StateSnapshot::new(15),
        ];

        tt.import(snapshots);

        assert_eq!(tt.count(), 3);
        assert_eq!(tt.position(), 2);
    }

    #[test]
    fn test_time_travel_import_empty() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;
        tt.record(StateSnapshot::new(0));

        tt.import(vec![]);

        assert_eq!(tt.count(), 0);
        assert_eq!(tt.position(), 0);
    }

    #[test]
    fn test_time_travel_select_next() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));

        assert!(tt.selected.is_none());

        tt.select_next();
        assert_eq!(tt.selected, Some(0));

        tt.select_next();
        assert_eq!(tt.selected, Some(1));

        // Should not go beyond count
        tt.select_next();
        assert_eq!(tt.selected, Some(1));
    }

    #[test]
    fn test_time_travel_select_prev() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));

        tt.selected = Some(1);
        tt.select_prev();
        assert_eq!(tt.selected, Some(0));

        tt.select_prev();
        assert_eq!(tt.selected, Some(0));
    }

    #[test]
    fn test_time_travel_select_empty() {
        let mut tt = TimeTravelDebugger::new();

        tt.select_next();
        assert!(tt.selected.is_none());
    }

    #[test]
    fn test_action_with_duration() {
        let action = Action::new("test").with_duration(Duration::from_secs(1));

        assert_eq!(action.duration, Some(Duration::from_secs(1)));
    }

    #[test]
    fn test_snapshot_with_action() {
        let action = Action::new("click");
        let snapshot = StateSnapshot::new(0).with_action(action);

        assert!(snapshot.action.is_some());
        assert_eq!(snapshot.action.unwrap().name, "click");
    }

    #[test]
    fn test_record_action() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        let mut state = HashMap::new();
        state.insert("count".to_string(), SnapshotValue::Int(5));

        tt.record_action(Action::new("increment"), state);

        assert_eq!(tt.count(), 1);
        let snapshot = tt.current().unwrap();
        assert!(snapshot.action.is_some());
        assert_eq!(snapshot.state.get("count"), Some(&SnapshotValue::Int(5)));
    }

    #[test]
    fn test_snapshot_diff_removed() {
        let mut state1 = HashMap::new();
        state1.insert("a".to_string(), SnapshotValue::Int(1));
        state1.insert("b".to_string(), SnapshotValue::Int(2));

        let mut state2 = HashMap::new();
        state2.insert("a".to_string(), SnapshotValue::Int(1));
        // b is removed

        let snap1 = StateSnapshot {
            id: 1,
            timestamp: SystemTime::now(),
            state: state1,
            action: None,
            label: None,
        };
        let snap2 = StateSnapshot {
            id: 2,
            timestamp: SystemTime::now(),
            state: state2,
            action: None,
            label: None,
        };

        let diff = snap2.diff(&snap1);
        assert_eq!(diff.removed.len(), 1);
        assert!(diff.removed.contains_key("b"));
    }

    #[test]
    fn test_step_forward_at_end() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0));
        tt.record(StateSnapshot::new(0));

        // Already at end
        assert_eq!(tt.position(), 1);

        tt.step_forward();
        assert_eq!(tt.position(), 1);
        assert!(!tt.is_traveling());
    }

    #[test]
    fn test_step_back_at_start() {
        let mut tt = TimeTravelDebugger::new();
        tt.config.record_interval = Duration::ZERO;

        tt.record(StateSnapshot::new(0));

        tt.step_back();
        assert_eq!(tt.position(), 0);
    }
}
