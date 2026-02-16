use super::{command::Command, core::CommandPalette};
use crate::style::Color;
use crate::utils::{fuzzy_match, Selection};
use crate::widget::traits::WidgetProps;

impl CommandPalette {
    /// Create a new command palette
    pub fn new() -> Self {
        let selection = Selection::new(0);
        selection.set_visible(10);
        Self {
            commands: Vec::new(),
            query: String::new(),
            filtered: Vec::new(),
            selection,
            visible: false,
            width: 60,
            max_visible: 10,
            placeholder: "Type to search...".to_string(),
            title: None,
            show_descriptions: true,
            show_shortcuts: true,
            show_icons: true,
            bg_color: Color::rgb(30, 30, 30),
            border_color: Color::rgb(80, 80, 80),
            selected_bg: Color::rgb(50, 80, 120),
            match_color: Color::YELLOW,
            props: WidgetProps::new(),
        }
    }

    /// Add a command
    pub fn command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self.update_filter();
        self
    }

    /// Add multiple commands
    pub fn commands(mut self, commands: Vec<Command>) -> Self {
        self.commands.extend(commands);
        self.update_filter();
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(30);
        self
    }

    /// Set max visible items
    pub fn max_visible(mut self, max: u16) -> Self {
        self.max_visible = max.max(3);
        self.selection.set_visible(self.max_visible as usize);
        self
    }

    /// Set placeholder
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Show/hide descriptions
    pub fn show_descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Show/hide shortcuts
    pub fn show_shortcuts(mut self, show: bool) -> Self {
        self.show_shortcuts = show;
        self
    }

    /// Show/hide icons
    pub fn show_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Set colors
    pub fn colors(mut self, bg: Color, border: Color, selected: Color) -> Self {
        self.bg_color = bg;
        self.border_color = border;
        self.selected_bg = selected;
        self
    }

    /// Show the palette
    pub fn show(&mut self) {
        self.visible = true;
        self.query.clear();
        self.selection.first();
        self.selection.reset_offset();
        self.update_filter();
    }

    /// Hide the palette
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        if self.visible {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get current query
    pub fn get_query(&self) -> &str {
        &self.query
    }

    /// Set query
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_filter();
    }

    /// Update filtered list
    fn update_filter(&mut self) {
        self.filtered = self
            .commands
            .iter()
            .enumerate()
            .filter(|(_, cmd)| cmd.matches(&self.query))
            .map(|(idx, cmd)| (idx, cmd.match_score(&self.query)))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(idx, _score)| idx)
            .collect();

        // Sort by score
        self.filtered.sort_by(|&a, &b| {
            let score_a = self.commands[a].match_score(&self.query);
            let score_b = self.commands[b].match_score(&self.query);
            score_b.cmp(&score_a)
        });

        // Reset selection
        self.selection.set_len(self.filtered.len());
        self.selection.first();
    }

    /// Select next item
    pub fn select_next(&mut self) {
        self.selection.next();
    }

    /// Select previous item
    pub fn select_prev(&mut self) {
        self.selection.prev();
    }

    /// Get selected command
    pub fn selected_command(&self) -> Option<&Command> {
        self.filtered
            .get(self.selection.index)
            .map(|&idx| &self.commands[idx])
    }

    /// Get selected command ID
    pub fn selected_id(&self) -> Option<&str> {
        self.selected_command().map(|c| c.id.as_str())
    }

    /// Execute selected (returns command ID)
    pub fn execute(&mut self) -> Option<String> {
        let id = self.selected_id().map(|s| s.to_string());
        if id.is_some() {
            self.hide();
        }
        id
    }

    /// Handle text input
    pub fn input(&mut self, ch: char) {
        self.query.push(ch);
        self.update_filter();
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        self.query.pop();
        self.update_filter();
    }

    /// Clear query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.update_filter();
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if !self.visible {
            return false;
        }

        match key {
            Key::Escape => {
                self.hide();
                true
            }
            Key::Enter => {
                self.execute();
                true
            }
            Key::Up | Key::Char('k') if self.query.is_empty() => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') if self.query.is_empty() => {
                self.select_next();
                true
            }
            Key::Up => {
                self.select_prev();
                true
            }
            Key::Down => {
                self.select_next();
                true
            }
            Key::Backspace => {
                self.backspace();
                true
            }
            Key::Char(ch) => {
                self.input(*ch);
                true
            }
            _ => false,
        }
    }

    /// Add command dynamically
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
        self.update_filter();
    }

    /// Remove command by ID
    pub fn remove_command(&mut self, id: &str) {
        self.commands.retain(|c| c.id != id);
        self.update_filter();
    }

    /// Clear all commands
    pub fn clear_commands(&mut self) {
        self.commands.clear();
        self.filtered.clear();
    }

    /// Mark command as recent
    pub fn mark_recent(&mut self, id: &str) {
        if let Some(cmd) = self.commands.iter_mut().find(|c| c.id == id) {
            cmd.recent = true;
        }
    }

    /// Highlight matching characters in label using fuzzy match indices
    pub fn highlight_match(&self, label: &str) -> Vec<(char, bool)> {
        if self.query.is_empty() {
            return label.chars().map(|c| (c, false)).collect();
        }

        // Get fuzzy match to find matched character indices
        if let Some(m) = fuzzy_match(&self.query, label) {
            label
                .chars()
                .enumerate()
                .map(|(i, c)| (c, m.indices.contains(&i)))
                .collect()
        } else {
            label.chars().map(|c| (c, false)).collect()
        }
    }
}
