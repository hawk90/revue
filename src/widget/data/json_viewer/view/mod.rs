//! JsonViewer widget implementation

mod navigation;
mod render;
mod search_impl;

use crate::style::Color;
use crate::widget::data::json_viewer::helpers::flatten_tree;
use crate::widget::data::json_viewer::parser::parse_json;
use crate::widget::data::json_viewer::search::{Search, SearchState};
use crate::widget::data::json_viewer::types::{JsonNode, JsonType};
use crate::widget::theme::{DISABLED_FG, PLACEHOLDER_FG};
use crate::widget::traits::WidgetProps;
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
            null_fg: Some(PLACEHOLDER_FG),
            bracket_fg: Some(Color::WHITE),
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            match_fg: Some(Color::BLACK),
            match_bg: Some(Color::YELLOW),
            line_number_fg: Some(DISABLED_FG),
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
}

impl Default for JsonViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(JsonViewer);
impl_props_builders!(JsonViewer);
