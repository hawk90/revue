//! Time-travel debugger implementation

use super::super::helpers::draw_text_overlay;
use super::super::DevToolsConfig;
use super::{Action, SnapshotValue, StateDiff, StateSnapshot, TimeTravelConfig, TimeTravelView};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
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
