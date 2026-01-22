//! JsonViewer widget implementation

use super::helpers::{flatten_tree, line_number_width};
use super::parser::parse_json;
use super::search::{Search, SearchState};
use super::types::{JsonNode, JsonType};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};
use std::collections::HashSet;

/// JSON Viewer widget
#[derive(Clone, Debug)]
pub struct JsonViewer {
    /// Root node of the JSON tree
    root: Option<JsonNode>,
    /// Raw JSON string (for display)
    raw_json: String,
    /// Collapsed node paths
    collapsed: HashSet<String>,
    /// Selected node index (in flattened tree)
    selected: usize,
    /// Scroll offset
    scroll: usize,
    /// Show line numbers
    show_line_numbers: bool,
    /// Indent size
    indent_size: u16,
    /// Whether to show type badges
    show_type_badges: bool,
    /// Search state
    search_state: SearchState,
    // Styling
    key_fg: Option<Color>,
    string_fg: Option<Color>,
    number_fg: Option<Color>,
    bool_fg: Option<Color>,
    null_fg: Option<Color>,
    bracket_fg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    match_fg: Option<Color>,
    match_bg: Option<Color>,
    line_number_fg: Option<Color>,
    fg: Option<Color>,
    bg: Option<Color>,
    /// Widget props
    props: WidgetProps,
}

impl JsonViewer {
    /// Create a new JSON viewer
    pub fn new() -> Self {
        Self {
            root: None,
            raw_json: String::new(),
            collapsed: HashSet::new(),
            selected: 0,
            scroll: 0,
            show_line_numbers: true,
            indent_size: 2,
            show_type_badges: false,
            search_state: SearchState::new(),
            key_fg: Some(Color::CYAN),
            string_fg: Some(Color::GREEN),
            number_fg: Some(Color::YELLOW),
            bool_fg: Some(Color::MAGENTA),
            null_fg: Some(Color::rgb(128, 128, 128)),
            bracket_fg: Some(Color::WHITE),
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            match_fg: Some(Color::BLACK),
            match_bg: Some(Color::YELLOW),
            line_number_fg: Some(Color::rgb(100, 100, 100)),
            fg: None,
            bg: None,
            props: WidgetProps::new(),
        }
    }

    /// Parse JSON from string content
    pub fn from_content(json: &str) -> Self {
        let mut viewer = Self::new();
        viewer.parse(json);
        viewer
    }

    /// Parse JSON content
    pub fn parse(&mut self, json: &str) {
        self.raw_json = json.to_string();
        self.root = parse_json(json);
        self.collapsed.clear();
        self.selected = 0;
        self.scroll = 0;
        self.clear_search();
    }

    /// Set JSON data
    pub fn json(mut self, json: &str) -> Self {
        self.parse(json);
        self
    }

    /// Show/hide line numbers
    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Set indent size
    pub fn indent_size(mut self, size: u16) -> Self {
        self.indent_size = size;
        self
    }

    /// Show/hide type badges
    pub fn show_type_badges(mut self, show: bool) -> Self {
        self.show_type_badges = show;
        self
    }

    /// Set key color
    pub fn key_color(mut self, color: Color) -> Self {
        self.key_fg = Some(color);
        self
    }

    /// Set string color
    pub fn string_color(mut self, color: Color) -> Self {
        self.string_fg = Some(color);
        self
    }

    /// Set number color
    pub fn number_color(mut self, color: Color) -> Self {
        self.number_fg = Some(color);
        self
    }

    /// Set boolean color
    pub fn bool_color(mut self, color: Color) -> Self {
        self.bool_fg = Some(color);
        self
    }

    /// Set null color
    pub fn null_color(mut self, color: Color) -> Self {
        self.null_fg = Some(color);
        self
    }

    /// Set selected style
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set search match style
    pub fn match_style(mut self, fg: Color, bg: Color) -> Self {
        self.match_fg = Some(fg);
        self.match_bg = Some(bg);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // State getters
    // ─────────────────────────────────────────────────────────────────────────

    /// Get selected node path
    pub fn selected_path(&self) -> Option<String> {
        self.get_visible_nodes()
            .get(self.selected)
            .map(|n| n.path.clone())
    }

    /// Get selected value
    pub fn selected_value(&self) -> Option<String> {
        self.get_visible_nodes()
            .get(self.selected)
            .and_then(|n| n.value.clone())
    }

    /// Check if node is collapsed
    pub fn is_collapsed(&self, path: &str) -> bool {
        self.collapsed.contains(path)
    }

    /// Check if JSON data is loaded
    pub fn has_data(&self) -> bool {
        self.root.is_some()
    }

    /// Get the type of the root node
    pub fn root_type(&self) -> Option<&JsonType> {
        self.root.as_ref().map(|n| &n.value_type)
    }

    /// Get number of children at root level
    pub fn root_children_count(&self) -> usize {
        self.root.as_ref().map(|n| n.children.len()).unwrap_or(0)
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Get count of visible nodes
    pub fn visible_count(&self) -> usize {
        self.get_visible_nodes().len()
    }

    /// Get indent size
    pub fn get_indent_size(&self) -> u16 {
        self.indent_size
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Navigation
    // ─────────────────────────────────────────────────────────────────────────

    /// Move selection down
    pub fn select_down(&mut self) {
        let max = self.get_visible_nodes().len().saturating_sub(1);
        self.selected = (self.selected + 1).min(max);
        self.ensure_visible();
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.ensure_visible();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let max = self.get_visible_nodes().len().saturating_sub(1);
        self.selected = (self.selected + page_size).min(max);
        self.ensure_visible();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
        self.ensure_visible();
    }

    /// Go to first node
    pub fn select_first(&mut self) {
        self.selected = 0;
        self.ensure_visible();
    }

    /// Go to last node
    pub fn select_last(&mut self) {
        self.selected = self.get_visible_nodes().len().saturating_sub(1);
        self.ensure_visible();
    }

    fn ensure_visible(&mut self) {
        // Handled during render
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Expand/Collapse
    // ─────────────────────────────────────────────────────────────────────────

    /// Toggle selected node expansion
    pub fn toggle(&mut self) {
        if let Some(node) = self.get_visible_nodes().get(self.selected) {
            if node.is_container() {
                let path = node.path.clone();
                if self.collapsed.contains(&path) {
                    self.collapsed.remove(&path);
                } else {
                    self.collapsed.insert(path);
                }
            }
        }
    }

    /// Expand selected node
    pub fn expand(&mut self) {
        if let Some(node) = self.get_visible_nodes().get(self.selected) {
            self.collapsed.remove(&node.path);
        }
    }

    /// Collapse selected node
    pub fn collapse(&mut self) {
        if let Some(node) = self.get_visible_nodes().get(self.selected) {
            if node.is_container() {
                self.collapsed.insert(node.path.clone());
            }
        }
    }

    /// Expand all nodes
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
    }

    /// Collapse all nodes
    pub fn collapse_all(&mut self) {
        if let Some(root) = self.root.clone() {
            self.collapse_recursive(&root);
        }
    }

    fn collapse_recursive(&mut self, node: &JsonNode) {
        if node.is_container() {
            self.collapsed.insert(node.path.clone());
            for child in &node.children {
                self.collapse_recursive(child);
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Get flattened list of visible nodes
    fn get_visible_nodes(&self) -> Vec<JsonNode> {
        if let Some(root) = &self.root {
            flatten_tree(root, &self.collapsed)
        } else {
            Vec::new()
        }
    }

    /// Sync collapsed set to search state
    fn sync_collapsed_to_search(&mut self) {
        self.search_state.collapsed = self.collapsed.clone();
    }

    /// Sync collapsed set from search state
    fn sync_collapsed_from_search(&mut self) {
        self.collapsed = self.search_state.collapsed.clone();
    }
}

impl Default for JsonViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl Search for JsonViewer {
    fn search(&mut self, query: &str) {
        self.search_state.search_query = query.to_lowercase();
        self.search_state.search_matches.clear();
        self.search_state.current_match = 0;

        if self.search_state.search_query.is_empty() {
            return;
        }

        if let Some(root) = self.root.clone() {
            self.sync_collapsed_to_search();
            self.search_state.search_recursive(&root);
            self.sync_collapsed_from_search();
        }
    }

    fn clear_search(&mut self) {
        self.search_state.search_query.clear();
        self.search_state.search_matches.clear();
        self.search_state.current_match = 0;
    }

    fn match_count(&self) -> usize {
        self.search_state.search_matches.len()
    }

    fn is_searching(&self) -> bool {
        !self.search_state.search_query.is_empty()
    }

    fn next_match(&mut self) {
        if !self.search_state.search_matches.is_empty() {
            self.search_state.current_match =
                (self.search_state.current_match + 1) % self.search_state.search_matches.len();
            self.go_to_match();
        }
    }

    fn prev_match(&mut self) {
        if !self.search_state.search_matches.is_empty() {
            self.search_state.current_match = self
                .search_state
                .current_match
                .checked_sub(1)
                .unwrap_or(self.search_state.search_matches.len() - 1);
            self.go_to_match();
        }
    }
}

impl JsonViewer {
    fn go_to_match(&mut self) {
        self.sync_collapsed_to_search();
        self.search_state.go_to_match(&mut self.selected, |state| {
            let collapsed = state.collapsed.clone();
            if let Some(root) = &self.root {
                flatten_tree(root, &collapsed)
            } else {
                Vec::new()
            }
        });
        self.sync_collapsed_from_search();
    }
}

impl View for JsonViewer {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 2 {
            return;
        }

        let nodes = self.get_visible_nodes();
        let total_lines = nodes.len();
        let line_num_width = line_number_width(self.show_line_numbers, total_lines);
        let content_start_x = area.x + line_num_width;
        let content_width = area.width.saturating_sub(line_num_width);

        // Adjust scroll to keep selection visible
        let visible_rows = area.height as usize;
        let mut scroll = self.scroll;
        if self.selected < scroll {
            scroll = self.selected;
        } else if self.selected >= scroll + visible_rows {
            scroll = self.selected.saturating_sub(visible_rows - 1);
        }

        for (visible_idx, node) in nodes.iter().skip(scroll).take(visible_rows).enumerate() {
            let y = area.y + visible_idx as u16;
            if y >= area.y + area.height {
                break;
            }

            let line_idx = scroll + visible_idx;
            let is_selected = line_idx == self.selected;
            let is_match = self.search_state.search_matches.contains(&node.path);

            // Line number
            if self.show_line_numbers {
                let num_str = format!(
                    "{:>width$}",
                    line_idx + 1,
                    width = (line_num_width - 1) as usize
                );
                for (i, ch) in num_str.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = self.line_number_fg;
                    cell.bg = if is_selected {
                        self.selected_bg
                    } else {
                        self.bg
                    };
                    ctx.buffer.set(area.x + i as u16, y, cell);
                }
            }

            // Indent
            let indent = (node.depth as u16) * self.indent_size;
            let mut x = content_start_x + indent;

            // Draw collapse indicator for containers
            if node.is_container() {
                let indicator = if self.collapsed.contains(&node.path) {
                    "▶ "
                } else {
                    "▼ "
                };
                for ch in indicator.chars() {
                    if x < area.x + area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = self.bracket_fg;
                        cell.bg = if is_selected {
                            self.selected_bg
                        } else {
                            self.bg
                        };
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }
            }

            // Determine colors
            let (fg, bg) = if is_selected {
                (self.selected_fg, self.selected_bg)
            } else if is_match {
                (self.match_fg, self.match_bg)
            } else {
                (self.fg, self.bg)
            };

            // Draw key if present
            if !node.key.is_empty() {
                let key_display = format!("\"{}\"", node.key);
                for ch in key_display.chars() {
                    if x < area.x + area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = if is_selected { fg } else { self.key_fg };
                        cell.bg = bg;
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }

                // Colon separator
                let sep = ": ";
                for ch in sep.chars() {
                    if x < area.x + area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = fg.or(self.fg);
                        cell.bg = bg;
                        ctx.buffer.set(x, y, cell);
                        x += 1;
                    }
                }
            }

            // Draw value or container brackets
            match &node.value_type {
                JsonType::Object => {
                    let text = if self.collapsed.contains(&node.path) {
                        format!("{{...}} ({} items)", node.child_count())
                    } else if node.children.is_empty() {
                        "{}".to_string()
                    } else {
                        "{".to_string()
                    };
                    for ch in text.chars() {
                        if x < area.x + area.width {
                            let mut cell = Cell::new(ch);
                            cell.fg = if is_selected { fg } else { self.bracket_fg };
                            cell.bg = bg;
                            ctx.buffer.set(x, y, cell);
                            x += 1;
                        }
                    }
                }
                JsonType::Array => {
                    let text = if self.collapsed.contains(&node.path) {
                        format!("[...] ({} items)", node.child_count())
                    } else if node.children.is_empty() {
                        "[]".to_string()
                    } else {
                        "[".to_string()
                    };
                    for ch in text.chars() {
                        if x < area.x + area.width {
                            let mut cell = Cell::new(ch);
                            cell.fg = if is_selected { fg } else { self.bracket_fg };
                            cell.bg = bg;
                            ctx.buffer.set(x, y, cell);
                            x += 1;
                        }
                    }
                }
                JsonType::String => {
                    if let Some(value) = &node.value {
                        let display = format!("\"{}\"", value);
                        let truncated: String = display
                            .chars()
                            .take((content_width.saturating_sub(indent + 2)) as usize)
                            .collect();
                        for ch in truncated.chars() {
                            if x < area.x + area.width {
                                let mut cell = Cell::new(ch);
                                cell.fg = if is_selected { fg } else { self.string_fg };
                                cell.bg = bg;
                                ctx.buffer.set(x, y, cell);
                                x += 1;
                            }
                        }
                    }
                }
                JsonType::Number => {
                    if let Some(value) = &node.value {
                        for ch in value.chars() {
                            if x < area.x + area.width {
                                let mut cell = Cell::new(ch);
                                cell.fg = if is_selected { fg } else { self.number_fg };
                                cell.bg = bg;
                                ctx.buffer.set(x, y, cell);
                                x += 1;
                            }
                        }
                    }
                }
                JsonType::Boolean => {
                    if let Some(value) = &node.value {
                        for ch in value.chars() {
                            if x < area.x + area.width {
                                let mut cell = Cell::new(ch);
                                cell.fg = if is_selected { fg } else { self.bool_fg };
                                cell.bg = bg;
                                ctx.buffer.set(x, y, cell);
                                x += 1;
                            }
                        }
                    }
                }
                JsonType::Null => {
                    for ch in "null".chars() {
                        if x < area.x + area.width {
                            let mut cell = Cell::new(ch);
                            cell.fg = if is_selected { fg } else { self.null_fg };
                            cell.bg = bg;
                            ctx.buffer.set(x, y, cell);
                            x += 1;
                        }
                    }
                }
            }

            // Fill rest of line with background
            while x < area.x + area.width {
                let mut cell = Cell::new(' ');
                cell.bg = bg;
                ctx.buffer.set(x, y, cell);
                x += 1;
            }
        }
    }

    crate::impl_view_meta!("JsonViewer");
}

impl_styled_view!(JsonViewer);
impl_props_builders!(JsonViewer);
