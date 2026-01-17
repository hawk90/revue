//! State debugger for reactive signals

use super::helpers::{draw_separator, draw_text_overlay};
use super::DevToolsConfig;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::collections::HashMap;

/// Helper context for rendering devtools panels
struct RenderCtx<'a> {
    buffer: &'a mut Buffer,
    x: u16,
    width: u16,
    config: &'a DevToolsConfig,
}

impl<'a> RenderCtx<'a> {
    fn new(buffer: &'a mut Buffer, x: u16, width: u16, config: &'a DevToolsConfig) -> Self {
        Self {
            buffer,
            x,
            width,
            config,
        }
    }

    fn draw_text(&mut self, y: u16, text: &str, color: Color) {
        draw_text_overlay(self.buffer, self.x, y, text, color);
    }

    fn draw_separator(&mut self, y: u16) {
        draw_separator(self.buffer, self.x, y, self.width, self.config.accent_color);
    }
}

/// State value representation
#[derive(Debug, Clone)]
pub enum StateValue {
    /// String value
    String(String),
    /// Integer value
    Int(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Bool(bool),
    /// List of values
    List(Vec<StateValue>),
    /// Map of values
    Map(HashMap<String, StateValue>),
    /// Null/None
    Null,
}

impl StateValue {
    /// Format value for display
    pub fn display(&self) -> String {
        match self {
            Self::String(s) => format!("\"{}\"", s),
            Self::Int(i) => i.to_string(),
            Self::Float(f) => format!("{:.2}", f),
            Self::Bool(b) => b.to_string(),
            Self::List(items) => format!("[{} items]", items.len()),
            Self::Map(items) => format!("{{{}  keys}}", items.len()),
            Self::Null => "null".to_string(),
        }
    }

    /// Get type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::String(_) => "String",
            Self::Int(_) => "i64",
            Self::Float(_) => "f64",
            Self::Bool(_) => "bool",
            Self::List(_) => "Vec",
            Self::Map(_) => "Map",
            Self::Null => "null",
        }
    }
}

/// State entry for debugging
#[derive(Debug, Clone)]
pub struct StateEntry {
    /// Signal/state name
    pub name: String,
    /// Type name
    pub type_name: String,
    /// Current value
    pub value: StateValue,
    /// Number of subscribers
    pub subscribers: usize,
    /// Update count
    pub update_count: u64,
    /// Is computed (derived)
    pub is_computed: bool,
}

impl StateEntry {
    /// Create new state entry
    pub fn new(name: impl Into<String>, value: StateValue) -> Self {
        Self {
            name: name.into(),
            type_name: value.type_name().to_string(),
            value,
            subscribers: 0,
            update_count: 0,
            is_computed: false,
        }
    }

    /// Set as computed
    pub fn computed(mut self) -> Self {
        self.is_computed = true;
        self
    }

    /// Set subscribers count
    pub fn subscribers(mut self, count: usize) -> Self {
        self.subscribers = count;
        self
    }

    /// Increment update count
    pub fn updated(&mut self) {
        self.update_count += 1;
    }
}

/// State debugger
#[derive(Debug, Default)]
pub struct StateDebugger {
    /// Tracked states
    states: Vec<StateEntry>,
    /// Selected index
    selected: Option<usize>,
    /// Scroll offset
    scroll: usize,
    /// Filter text
    filter: String,
    /// Show computed values
    show_computed: bool,
    /// Show update counts (for future UI)
    _show_updates: bool,
}

impl StateDebugger {
    /// Create new state debugger
    pub fn new() -> Self {
        Self {
            show_computed: true,
            _show_updates: true,
            ..Default::default()
        }
    }

    /// Clear all states
    pub fn clear(&mut self) {
        self.states.clear();
        self.selected = None;
    }

    /// Add a state entry
    pub fn add(&mut self, entry: StateEntry) {
        self.states.push(entry);
    }

    /// Update a state value
    pub fn update(&mut self, name: &str, value: StateValue) {
        if let Some(entry) = self.states.iter_mut().find(|e| e.name == name) {
            entry.value = value;
            entry.updated();
        }
    }

    /// Remove a state
    pub fn remove(&mut self, name: &str) {
        self.states.retain(|e| e.name != name);
    }

    /// Get filtered states
    fn filtered(&self) -> Vec<&StateEntry> {
        self.states
            .iter()
            .filter(|e| {
                if !self.show_computed && e.is_computed {
                    return false;
                }
                if !self.filter.is_empty() {
                    return e.name.to_lowercase().contains(&self.filter.to_lowercase());
                }
                true
            })
            .collect()
    }

    /// Select next
    pub fn select_next(&mut self) {
        let count = self.filtered().len();
        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => (i + 1).min(count - 1),
            None => 0,
        });
    }

    /// Select previous
    pub fn select_prev(&mut self) {
        let count = self.filtered().len();
        if count == 0 {
            return;
        }

        self.selected = Some(match self.selected {
            Some(i) => i.saturating_sub(1),
            None => 0,
        });
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: impl Into<String>) {
        self.filter = filter.into();
        self.selected = None;
    }

    /// Render state debugger content
    pub fn render_content(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut ctx = RenderCtx::new(buffer, area.x, area.width, config);
        let mut y = area.y;
        let max_y = area.y + area.height;

        // Header
        let header = format!("{} signals tracked", self.states.len());
        ctx.draw_text(y, &header, config.accent_color);
        y += 2;

        // States list
        let filtered = self.filtered();
        for (i, entry) in filtered.iter().enumerate().skip(self.scroll) {
            if y >= max_y - 2 {
                break;
            }

            let is_selected = self.selected == Some(i);
            Self::render_entry(&mut ctx, y, entry, is_selected);
            y += 1;
        }

        // Selected details
        if let Some(idx) = self.selected {
            if let Some(entry) = filtered.get(idx) {
                if y + 2 < max_y {
                    y = max_y - 3;
                    ctx.draw_separator(y);
                    y += 1;
                    Self::render_details(&mut ctx, y, entry);
                }
            }
        }
    }

    fn render_entry(ctx: &mut RenderCtx<'_>, y: u16, entry: &StateEntry, selected: bool) {
        let prefix = if entry.is_computed { "⊙ " } else { "● " };
        let line = format!("{}{}: {}", prefix, entry.name, entry.value.display());

        let fg = if selected {
            ctx.config.bg_color
        } else {
            ctx.config.fg_color
        };
        let bg = if selected {
            Some(ctx.config.accent_color)
        } else {
            None
        };

        for (i, ch) in line.chars().enumerate() {
            if (i as u16) < ctx.width {
                if let Some(cell) = ctx.buffer.get_mut(ctx.x + i as u16, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                    if let Some(b) = bg {
                        cell.bg = Some(b);
                    }
                }
            }
        }
    }

    fn render_details(ctx: &mut RenderCtx<'_>, y: u16, entry: &StateEntry) {
        let details = format!(
            "Type: {} | Subs: {} | Updates: {}",
            entry.type_name, entry.subscribers, entry.update_count
        );
        ctx.draw_text(y, &details, ctx.config.fg_color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_value_display() {
        assert_eq!(StateValue::Int(42).display(), "42");
        assert_eq!(StateValue::Bool(true).display(), "true");
        assert_eq!(StateValue::String("hello".into()).display(), "\"hello\"");
    }

    #[test]
    fn test_state_entry() {
        let entry = StateEntry::new("count", StateValue::Int(0))
            .computed()
            .subscribers(3);

        assert!(entry.is_computed);
        assert_eq!(entry.subscribers, 3);
    }

    #[test]
    fn test_state_debugger_add() {
        let mut debugger = StateDebugger::new();
        debugger.add(StateEntry::new("test", StateValue::Int(1)));

        assert_eq!(debugger.states.len(), 1);
    }

    #[test]
    fn test_state_debugger_update() {
        let mut debugger = StateDebugger::new();
        debugger.add(StateEntry::new("count", StateValue::Int(0)));
        debugger.update("count", StateValue::Int(5));

        assert!(matches!(debugger.states[0].value, StateValue::Int(5)));
        assert_eq!(debugger.states[0].update_count, 1);
    }
}
