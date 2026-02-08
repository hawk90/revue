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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Key;

    // =========================================================================
    // CommandPalette::new tests
    // =========================================================================

    #[test]
    fn test_command_palette_new() {
        let palette = CommandPalette::new();
        assert!(palette.commands.is_empty());
        assert!(palette.query.is_empty());
        assert!(!palette.is_visible());
        assert_eq!(palette.width, 60);
        assert_eq!(palette.max_visible, 10);
    }

    // =========================================================================
    // command(s) builder tests
    // =========================================================================

    #[test]
    fn test_command_palette_command() {
        let palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
        assert_eq!(palette.commands.len(), 1);
    }

    #[test]
    fn test_command_palette_commands() {
        let cmds = vec![
            Command::new("cmd1", "Command 1"),
            Command::new("cmd2", "Command 2"),
        ];
        let palette = CommandPalette::new().commands(cmds);
        assert_eq!(palette.commands.len(), 2);
    }

    // =========================================================================
    // Builder methods tests
    // =========================================================================

    #[test]
    fn test_command_palette_width() {
        let palette = CommandPalette::new().width(40);
        assert_eq!(palette.width, 40);
    }

    #[test]
    fn test_command_palette_width_minimum() {
        let palette = CommandPalette::new().width(10);
        assert_eq!(palette.width, 30); // minimum enforced
    }

    #[test]
    fn test_command_palette_max_visible() {
        let palette = CommandPalette::new().max_visible(5);
        assert_eq!(palette.max_visible, 5);
    }

    #[test]
    fn test_command_palette_max_visible_minimum() {
        let palette = CommandPalette::new().max_visible(1);
        assert_eq!(palette.max_visible, 3); // minimum enforced
    }

    #[test]
    fn test_command_palette_placeholder() {
        let palette = CommandPalette::new().placeholder("Search...");
        assert_eq!(palette.placeholder, "Search...");
    }

    #[test]
    fn test_command_palette_title() {
        let palette = CommandPalette::new().title("Commands");
        assert_eq!(palette.title, Some("Commands".to_string()));
    }

    #[test]
    fn test_command_palette_show_descriptions() {
        let palette = CommandPalette::new().show_descriptions(false);
        assert!(!palette.show_descriptions);
    }

    #[test]
    fn test_command_palette_show_shortcuts() {
        let palette = CommandPalette::new().show_shortcuts(false);
        assert!(!palette.show_shortcuts);
    }

    #[test]
    fn test_command_palette_show_icons() {
        let palette = CommandPalette::new().show_icons(false);
        assert!(!palette.show_icons);
    }

    #[test]
    fn test_command_palette_colors() {
        let palette = CommandPalette::new().colors(Color::BLACK, Color::WHITE, Color::RED);
        assert_eq!(palette.bg_color, Color::BLACK);
        assert_eq!(palette.border_color, Color::WHITE);
        assert_eq!(palette.selected_bg, Color::RED);
    }

    // =========================================================================
    // Visibility tests
    // =========================================================================

    #[test]
    fn test_command_palette_show() {
        let mut palette = CommandPalette::new();
        palette.show();
        assert!(palette.is_visible());
        assert!(palette.query.is_empty());
    }

    #[test]
    fn test_command_palette_hide() {
        let mut palette = CommandPalette::new();
        palette.show();
        palette.hide();
        assert!(!palette.is_visible());
    }

    #[test]
    fn test_command_palette_toggle_hidden_to_visible() {
        let mut palette = CommandPalette::new();
        palette.toggle();
        assert!(palette.is_visible());
    }

    #[test]
    fn test_command_palette_toggle_visible_to_hidden() {
        let mut palette = CommandPalette::new();
        palette.show();
        palette.toggle();
        assert!(!palette.is_visible());
    }

    // =========================================================================
    // Query tests
    // =========================================================================

    #[test]
    fn test_command_palette_get_query() {
        let palette = CommandPalette::new();
        assert_eq!(palette.get_query(), "");
    }

    #[test]
    fn test_command_palette_set_query() {
        let mut palette = CommandPalette::new();
        palette.set_query("test");
        assert_eq!(palette.get_query(), "test");
    }

    #[test]
    fn test_command_palette_clear_query() {
        let mut palette = CommandPalette::new();
        palette.set_query("test");
        palette.clear_query();
        assert_eq!(palette.get_query(), "");
    }

    // =========================================================================
    // Selection tests
    // =========================================================================

    #[test]
    fn test_command_palette_select_next() {
        let mut palette = CommandPalette::new()
            .commands(vec![Command::new("cmd1", "A"), Command::new("cmd2", "B")]);
        palette.select_next();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_command_palette_select_prev() {
        let mut palette = CommandPalette::new()
            .commands(vec![Command::new("cmd1", "A"), Command::new("cmd2", "B")]);
        palette.select_prev();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_command_palette_selected_command_empty() {
        let palette = CommandPalette::new();
        assert!(palette.selected_command().is_none());
    }

    #[test]
    fn test_command_palette_selected_command() {
        let palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
        let cmd = palette.selected_command();
        assert!(cmd.is_some());
    }

    #[test]
    fn test_command_palette_selected_id_empty() {
        let palette = CommandPalette::new();
        assert!(palette.selected_id().is_none());
    }

    #[test]
    fn test_command_palette_selected_id() {
        let palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
        let id = palette.selected_id();
        assert_eq!(id, Some("cmd1"));
    }

    // =========================================================================
    // Execute tests
    // =========================================================================

    #[test]
    fn test_command_palette_execute_empty() {
        let mut palette = CommandPalette::new();
        let result = palette.execute();
        assert!(result.is_none());
    }

    #[test]
    fn test_command_palette_execute_with_selection() {
        let mut palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
        let result = palette.execute();
        assert_eq!(result, Some("cmd1".to_string()));
        assert!(!palette.is_visible());
    }

    // =========================================================================
    // Input tests
    // =========================================================================

    #[test]
    fn test_command_palette_input() {
        let mut palette = CommandPalette::new();
        palette.input('a');
        assert_eq!(palette.get_query(), "a");
    }

    #[test]
    fn test_command_palette_input_multiple() {
        let mut palette = CommandPalette::new();
        palette.input('a');
        palette.input('b');
        palette.input('c');
        assert_eq!(palette.get_query(), "abc");
    }

    #[test]
    fn test_command_palette_backspace() {
        let mut palette = CommandPalette::new();
        palette.input('a');
        palette.backspace();
        assert_eq!(palette.get_query(), "");
    }

    #[test]
    fn test_command_palette_backspace_empty() {
        let mut palette = CommandPalette::new();
        palette.backspace();
        assert_eq!(palette.get_query(), "");
    }

    // =========================================================================
    // Key handling tests
    // =========================================================================

    #[test]
    fn test_handle_key_escape_hides() {
        let mut palette = CommandPalette::new();
        palette.show();
        palette.handle_key(&Key::Escape);
        assert!(!palette.is_visible());
    }

    #[test]
    fn test_handle_key_enter_executes() {
        let mut palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
        palette.show();
        let result = palette.handle_key(&Key::Enter);
        assert!(result);
        assert!(!palette.is_visible());
    }

    #[test]
    fn test_handle_key_up() {
        let mut palette = CommandPalette::new()
            .commands(vec![Command::new("cmd1", "A"), Command::new("cmd2", "B")]);
        palette.show();
        let handled = palette.handle_key(&Key::Up);
        assert!(handled);
    }

    #[test]
    fn test_handle_key_down() {
        let mut palette = CommandPalette::new()
            .commands(vec![Command::new("cmd1", "A"), Command::new("cmd2", "B")]);
        palette.show();
        let handled = palette.handle_key(&Key::Down);
        assert!(handled);
    }

    #[test]
    fn test_handle_key_char() {
        let mut palette = CommandPalette::new();
        palette.show();
        let handled = palette.handle_key(&Key::Char('a'));
        assert!(handled);
        assert_eq!(palette.get_query(), "a");
    }

    #[test]
    fn test_handle_key_backspace() {
        let mut palette = CommandPalette::new();
        palette.show();
        palette.input('a');
        let handled = palette.handle_key(&Key::Backspace);
        assert!(handled);
        assert_eq!(palette.get_query(), "");
    }

    #[test]
    fn test_handle_key_when_hidden() {
        let mut palette = CommandPalette::new();
        let handled = palette.handle_key(&Key::Char('a'));
        assert!(!handled);
    }

    #[test]
    fn test_handle_key_unknown() {
        let mut palette = CommandPalette::new();
        palette.show();
        let handled = palette.handle_key(&Key::PageUp);
        assert!(!handled);
    }

    // =========================================================================
    // Command management tests
    // =========================================================================

    #[test]
    fn test_add_command() {
        let mut palette = CommandPalette::new();
        palette.add_command(Command::new("cmd1", "Command 1"));
        assert_eq!(palette.commands.len(), 1);
    }

    #[test]
    fn test_remove_command() {
        let mut palette = CommandPalette::new();
        palette.add_command(Command::new("cmd1", "Command 1"));
        palette.remove_command("cmd1");
        assert_eq!(palette.commands.len(), 0);
    }

    #[test]
    fn test_remove_command_nonexistent() {
        let mut palette = CommandPalette::new();
        palette.remove_command("nonexistent");
        assert_eq!(palette.commands.len(), 0);
    }

    #[test]
    fn test_clear_commands() {
        let mut palette = CommandPalette::new();
        palette.add_command(Command::new("cmd1", "Command 1"));
        palette.add_command(Command::new("cmd2", "Command 2"));
        palette.clear_commands();
        assert_eq!(palette.commands.len(), 0);
        assert!(palette.filtered.is_empty());
    }

    #[test]
    fn test_mark_recent() {
        let mut palette = CommandPalette::new();
        palette.add_command(Command::new("cmd1", "Command 1"));
        palette.mark_recent("cmd1");
        assert!(palette.commands[0].recent);
    }

    #[test]
    fn test_mark_recent_nonexistent() {
        let mut palette = CommandPalette::new();
        palette.mark_recent("nonexistent");
        // Should not panic
    }

    // =========================================================================
    // highlight_match tests
    // =========================================================================

    #[test]
    fn test_highlight_match_empty_query() {
        let palette = CommandPalette::new();
        let result = palette.highlight_match("Save");
        assert_eq!(result.len(), 4);
        // All chars should not be highlighted
        assert!(!result[0].1);
        assert!(!result[1].1);
        assert!(!result[2].1);
        assert!(!result[3].1);
    }

    #[test]
    fn test_highlight_match_with_query() {
        let mut palette = CommandPalette::new();
        palette.set_query("sv");
        let result = palette.highlight_match("Save");
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_highlight_match_empty_label() {
        let palette = CommandPalette::new();
        let result = palette.highlight_match("");
        assert!(result.is_empty());
    }

    // =========================================================================
    // Filter tests
    // =========================================================================

    #[test]
    fn test_filter_with_query() {
        let mut palette = CommandPalette::new().commands(vec![
            Command::new("cmd1", "Save File"),
            Command::new("cmd2", "Open File"),
            Command::new("cmd3", "Exit"),
        ]);
        palette.set_query("save");
        assert!(palette.filtered.len() > 0);
    }

    #[test]
    fn test_filter_empty_query() {
        let mut palette = CommandPalette::new().commands(vec![
            Command::new("cmd1", "Save"),
            Command::new("cmd2", "Open"),
        ]);
        palette.set_query("");
        assert_eq!(palette.filtered.len(), 2);
    }

    #[test]
    fn test_filter_no_match() {
        let mut palette = CommandPalette::new().commands(vec![Command::new("cmd1", "Save")]);
        palette.set_query("xyz");
        assert_eq!(palette.filtered.len(), 0);
    }
}
