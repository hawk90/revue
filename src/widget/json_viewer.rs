//! JSON Viewer widget for displaying and navigating JSON data
//!
//! Features:
//! - Collapsible tree structure
//! - Syntax highlighting by type (string, number, boolean, null)
//! - Search functionality
//! - Expand/collapse all
//! - Copy path/value support
//! - Virtual scrolling for large documents

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};
use std::collections::HashSet;

/// JSON value type for styling
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonType {
    /// JSON object `{}`
    Object,
    /// JSON array `[]`
    Array,
    /// JSON string value
    String,
    /// JSON number value
    Number,
    /// JSON boolean (true/false)
    Boolean,
    /// JSON null value
    Null,
}

/// A node in the JSON tree
#[derive(Clone, Debug)]
pub struct JsonNode {
    /// Key name (empty for root or array elements)
    pub key: String,
    /// JSON path to this node
    pub path: String,
    /// Value type
    pub value_type: JsonType,
    /// String representation of value (for leaf nodes)
    pub value: Option<String>,
    /// Child nodes (for objects and arrays)
    pub children: Vec<JsonNode>,
    /// Depth in tree
    pub depth: usize,
    /// Index in flattened list (set during render)
    pub index: usize,
}

impl JsonNode {
    fn new(
        key: impl Into<String>,
        path: impl Into<String>,
        value_type: JsonType,
        depth: usize,
    ) -> Self {
        Self {
            key: key.into(),
            path: path.into(),
            value_type,
            value: None,
            children: Vec::new(),
            depth,
            index: 0,
        }
    }

    fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    #[allow(dead_code)]
    fn with_children(mut self, children: Vec<JsonNode>) -> Self {
        self.children = children;
        self
    }

    fn is_container(&self) -> bool {
        matches!(self.value_type, JsonType::Object | JsonType::Array)
    }

    fn child_count(&self) -> usize {
        self.children.len()
    }
}

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
    /// Search query
    search_query: String,
    /// Search matches (paths)
    search_matches: Vec<String>,
    /// Current search match index
    current_match: usize,
    /// Show line numbers
    show_line_numbers: bool,
    /// Indent size
    indent_size: u16,
    /// Whether to show type badges
    show_type_badges: bool,
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
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: 0,
            show_line_numbers: true,
            indent_size: 2,
            show_type_badges: false,
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

    /// Get search match count
    pub fn match_count(&self) -> usize {
        self.search_matches.len()
    }

    /// Check if search is active
    pub fn is_searching(&self) -> bool {
        !self.search_query.is_empty()
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
    // Search
    // ─────────────────────────────────────────────────────────────────────────

    /// Set search query
    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_lowercase();
        self.search_matches.clear();
        self.current_match = 0;

        if self.search_query.is_empty() {
            return;
        }

        if let Some(root) = self.root.clone() {
            self.search_recursive(&root);
        }
    }

    fn search_recursive(&mut self, node: &JsonNode) {
        // Check key
        if node.key.to_lowercase().contains(&self.search_query) {
            self.search_matches.push(node.path.clone());
        }
        // Check value
        else if let Some(value) = &node.value {
            if value.to_lowercase().contains(&self.search_query) {
                self.search_matches.push(node.path.clone());
            }
        }

        for child in &node.children {
            self.search_recursive(child);
        }
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_matches.clear();
        self.current_match = 0;
    }

    /// Go to next match
    pub fn next_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.search_matches.len();
            self.go_to_match();
        }
    }

    /// Go to previous match
    pub fn prev_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.current_match = self
                .current_match
                .checked_sub(1)
                .unwrap_or(self.search_matches.len() - 1);
            self.go_to_match();
        }
    }

    fn go_to_match(&mut self) {
        if let Some(path) = self.search_matches.get(self.current_match) {
            // Expand all ancestors
            let parts: Vec<&str> = path.split('.').collect();
            let mut current_path = String::new();
            for (i, part) in parts.iter().enumerate() {
                if i > 0 {
                    current_path.push('.');
                }
                current_path.push_str(part);
                self.collapsed.remove(&current_path);
            }

            // Find and select the node
            let visible = self.get_visible_nodes();
            for (idx, node) in visible.iter().enumerate() {
                if &node.path == path {
                    self.selected = idx;
                    break;
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Get flattened list of visible nodes
    fn get_visible_nodes(&self) -> Vec<JsonNode> {
        let mut nodes = Vec::new();
        if let Some(root) = &self.root {
            self.flatten_node(root, &mut nodes);
        }
        nodes
    }

    fn flatten_node(&self, node: &JsonNode, nodes: &mut Vec<JsonNode>) {
        let mut node_clone = node.clone();
        node_clone.index = nodes.len();
        nodes.push(node_clone);

        if node.is_container() && !self.collapsed.contains(&node.path) {
            for child in &node.children {
                self.flatten_node(child, nodes);
            }
        }
    }

    /// Get line number width
    fn line_number_width(&self, total_lines: usize) -> u16 {
        if self.show_line_numbers {
            let digits = (total_lines as f64).log10().floor() as u16 + 1;
            digits.max(2) + 1
        } else {
            0
        }
    }
}

impl Default for JsonViewer {
    fn default() -> Self {
        Self::new()
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
        let line_num_width = self.line_number_width(total_lines);
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
            let is_match = self.search_matches.contains(&node.path);

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

/// Simple JSON parser (handles basic JSON)
fn parse_json(json: &str) -> Option<JsonNode> {
    let json = json.trim();
    if json.is_empty() {
        return None;
    }

    parse_value(json, "", 0).map(|(node, _)| node)
}

fn parse_value(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    let json = json.trim_start();
    if json.is_empty() {
        return None;
    }

    let first = json.chars().next()?;

    match first {
        '{' => parse_object(json, path, depth),
        '[' => parse_array(json, path, depth),
        '"' => parse_string(json, path, depth),
        't' | 'f' => parse_bool(json, path, depth),
        'n' => parse_null(json, path, depth),
        c if c.is_ascii_digit() || c == '-' => parse_number(json, path, depth),
        _ => None,
    }
}

fn parse_object(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if !json.starts_with('{') {
        return None;
    }

    let node_path = if path.is_empty() {
        "$".to_string()
    } else {
        path.to_string()
    };

    let mut node = JsonNode::new("", &node_path, JsonType::Object, depth);
    let mut children = Vec::new();
    let mut idx = 1; // Skip '{'
    let chars: Vec<char> = json.chars().collect();

    loop {
        // Skip whitespace
        while idx < chars.len() && chars[idx].is_whitespace() {
            idx += 1;
        }

        if idx >= chars.len() {
            return None;
        }

        if chars[idx] == '}' {
            idx += 1;
            break;
        }

        // Skip comma
        if chars[idx] == ',' {
            idx += 1;
            continue;
        }

        // Parse key
        if chars[idx] != '"' {
            return None;
        }
        let key_start = idx + 1;
        idx += 1;
        while idx < chars.len() && chars[idx] != '"' {
            if chars[idx] == '\\' {
                idx += 1;
            }
            idx += 1;
        }
        if idx >= chars.len() {
            return None;
        }
        let key: String = chars[key_start..idx].iter().collect();
        idx += 1; // Skip closing quote

        // Skip whitespace and colon
        while idx < chars.len() && (chars[idx].is_whitespace() || chars[idx] == ':') {
            idx += 1;
        }

        // Parse value
        let child_path = if node_path == "$" {
            format!("$.{}", key)
        } else {
            format!("{}.{}", node_path, key)
        };

        let remaining: String = chars[idx..].iter().collect();
        if let Some((mut child, consumed)) = parse_value(&remaining, &child_path, depth + 1) {
            child.key = key;
            children.push(child);
            idx += consumed;
        } else {
            return None;
        }
    }

    node.children = children;
    Some((node, idx))
}

fn parse_array(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if !json.starts_with('[') {
        return None;
    }

    let node_path = if path.is_empty() {
        "$".to_string()
    } else {
        path.to_string()
    };

    let mut node = JsonNode::new("", &node_path, JsonType::Array, depth);
    let mut children = Vec::new();
    let mut idx = 1; // Skip '['
    let mut array_idx = 0;
    let chars: Vec<char> = json.chars().collect();

    loop {
        // Skip whitespace
        while idx < chars.len() && chars[idx].is_whitespace() {
            idx += 1;
        }

        if idx >= chars.len() {
            return None;
        }

        if chars[idx] == ']' {
            idx += 1;
            break;
        }

        // Skip comma
        if chars[idx] == ',' {
            idx += 1;
            continue;
        }

        // Parse value
        let child_path = format!("{}[{}]", node_path, array_idx);
        let remaining: String = chars[idx..].iter().collect();
        if let Some((mut child, consumed)) = parse_value(&remaining, &child_path, depth + 1) {
            child.key = format!("[{}]", array_idx);
            children.push(child);
            idx += consumed;
            array_idx += 1;
        } else {
            return None;
        }
    }

    node.children = children;
    Some((node, idx))
}

fn parse_string(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if !json.starts_with('"') {
        return None;
    }

    let chars: Vec<char> = json.chars().collect();
    let mut idx = 1;
    let mut value = String::new();

    while idx < chars.len() {
        let c = chars[idx];
        if c == '"' {
            idx += 1;
            break;
        }
        if c == '\\' && idx + 1 < chars.len() {
            idx += 1;
            match chars[idx] {
                'n' => value.push('\n'),
                'r' => value.push('\r'),
                't' => value.push('\t'),
                '"' => value.push('"'),
                '\\' => value.push('\\'),
                _ => value.push(chars[idx]),
            }
        } else {
            value.push(c);
        }
        idx += 1;
    }

    let node = JsonNode::new("", path, JsonType::String, depth).with_value(value);
    Some((node, idx))
}

fn parse_number(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    let chars: Vec<char> = json.chars().collect();
    let mut idx = 0;

    // Optional minus
    if idx < chars.len() && chars[idx] == '-' {
        idx += 1;
    }

    // Digits
    while idx < chars.len()
        && (chars[idx].is_ascii_digit()
            || chars[idx] == '.'
            || chars[idx] == 'e'
            || chars[idx] == 'E'
            || chars[idx] == '+'
            || chars[idx] == '-')
    {
        idx += 1;
    }

    let value: String = chars[..idx].iter().collect();
    let node = JsonNode::new("", path, JsonType::Number, depth).with_value(value);
    Some((node, idx))
}

fn parse_bool(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if json.starts_with("true") {
        let node = JsonNode::new("", path, JsonType::Boolean, depth).with_value("true");
        Some((node, 4))
    } else if json.starts_with("false") {
        let node = JsonNode::new("", path, JsonType::Boolean, depth).with_value("false");
        Some((node, 5))
    } else {
        None
    }
}

fn parse_null(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if json.starts_with("null") {
        let node = JsonNode::new("", path, JsonType::Null, depth);
        Some((node, 4))
    } else {
        None
    }
}

/// Helper function to create a JSON viewer
pub fn json_viewer() -> JsonViewer {
    JsonViewer::new()
}
