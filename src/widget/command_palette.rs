//! Command Palette widget for quick command access
//!
//! Provides a searchable command interface similar to VSCode's Ctrl+P
//! or Sublime Text's Command Palette.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::{fuzzy_match, FuzzyMatch, Selection};
use crate::{impl_props_builders, impl_styled_view};

/// Command item
#[derive(Clone, Debug)]
pub struct Command {
    /// Command ID
    pub id: String,
    /// Display label
    pub label: String,
    /// Description
    pub description: Option<String>,
    /// Keyboard shortcut
    pub shortcut: Option<String>,
    /// Category/group
    pub category: Option<String>,
    /// Icon character
    pub icon: Option<char>,
    /// Is recently used
    pub recent: bool,
    /// Is pinned
    pub pinned: bool,
}

impl Command {
    /// Create a new command
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            shortcut: None,
            category: None,
            icon: None,
            recent: false,
            pinned: false,
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set shortcut
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set category
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Mark as recent
    pub fn recent(mut self) -> Self {
        self.recent = true;
        self
    }

    /// Mark as pinned
    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    /// Check if command matches query using fuzzy matching
    pub fn matches(&self, query: &str) -> bool {
        self.fuzzy_match(query).is_some()
    }

    /// Get fuzzy match result for label
    pub fn fuzzy_match(&self, query: &str) -> Option<FuzzyMatch> {
        if query.is_empty() {
            return Some(FuzzyMatch::new(0, vec![]));
        }

        // Try fuzzy match on label
        if let Some(m) = fuzzy_match(query, &self.label) {
            return Some(m);
        }

        // Try match on description
        if let Some(ref desc) = self.description {
            if let Some(m) = fuzzy_match(query, desc) {
                return Some(m);
            }
        }

        // Try match on category
        if let Some(ref cat) = self.category {
            if let Some(m) = fuzzy_match(query, cat) {
                return Some(m);
            }
        }

        None
    }

    /// Get match score (higher = better match)
    pub fn match_score(&self, query: &str) -> i32 {
        if query.is_empty() {
            return if self.pinned {
                100
            } else if self.recent {
                50
            } else {
                0
            };
        }

        let mut score = 0;

        // Use fuzzy match score
        if let Some(m) = fuzzy_match(query, &self.label) {
            score += m.score;
        }

        // Bonus for pinned/recent
        if self.pinned {
            score += 50;
        }
        if self.recent {
            score += 25;
        }

        score
    }
}

/// Command Palette widget
pub struct CommandPalette {
    /// All commands
    commands: Vec<Command>,
    /// Current search query
    query: String,
    /// Filtered command indices
    filtered: Vec<usize>,
    /// Selection state for filtered list (uses Selection utility)
    selection: Selection,
    /// Visible state
    visible: bool,
    /// Width
    width: u16,
    /// Max visible items
    max_visible: u16,
    /// Placeholder text
    placeholder: String,
    /// Title
    title: Option<String>,
    /// Show descriptions
    show_descriptions: bool,
    /// Show shortcuts
    show_shortcuts: bool,
    /// Show icons
    show_icons: bool,
    /// Colors
    bg_color: Color,
    border_color: Color,
    selected_bg: Color,
    match_color: Color,
    /// Widget properties
    props: WidgetProps,
}

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
    fn highlight_match(&self, label: &str) -> Vec<(char, bool)> {
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

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl View for CommandPalette {
    crate::impl_view_meta!("CommandPalette");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible {
            return;
        }

        let area = ctx.area;
        let width = self.width.min(area.width);
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let has_title = self.title.is_some();

        // Calculate height
        let visible_count = self.filtered.len().min(self.max_visible as usize);
        let height = 3 + visible_count as u16 + if has_title { 1 } else { 0 }; // border + input + items
        let y = area.y + 2; // Offset from top

        if y + height > area.y + area.height {
            return;
        }

        // Draw background
        for dy in 0..height {
            for dx in 0..width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.bg_color);
                ctx.buffer.set(x + dx, y + dy, cell);
            }
        }

        // Draw border
        let border_chars = ['╭', '╮', '╰', '╯', '─', '│'];
        let mut current_y = y;

        // Top border
        let mut tl = Cell::new(border_chars[0]);
        tl.fg = Some(self.border_color);
        ctx.buffer.set(x, current_y, tl);

        for dx in 1..width - 1 {
            let mut h = Cell::new(border_chars[4]);
            h.fg = Some(self.border_color);
            ctx.buffer.set(x + dx, current_y, h);
        }

        let mut tr = Cell::new(border_chars[1]);
        tr.fg = Some(self.border_color);
        ctx.buffer.set(x + width - 1, current_y, tr);

        current_y += 1;

        // Title (if present)
        if let Some(ref title) = self.title {
            let mut left = Cell::new(border_chars[5]);
            left.fg = Some(self.border_color);
            ctx.buffer.set(x, current_y, left);

            let title_x = x + 2;
            for (i, ch) in title.chars().enumerate() {
                if title_x + i as u16 >= x + width - 2 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::CYAN);
                cell.bg = Some(self.bg_color);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(title_x + i as u16, current_y, cell);
            }

            let mut right = Cell::new(border_chars[5]);
            right.fg = Some(self.border_color);
            ctx.buffer.set(x + width - 1, current_y, right);

            current_y += 1;
        }

        // Search input line
        let mut left = Cell::new(border_chars[5]);
        left.fg = Some(self.border_color);
        ctx.buffer.set(x, current_y, left);

        // Search icon
        let search_icon = Cell::new('🔍');
        ctx.buffer.set(x + 2, current_y, search_icon);

        // Query or placeholder
        let input_x = x + 4;
        let display_text = if self.query.is_empty() {
            &self.placeholder
        } else {
            &self.query
        };
        let text_color = if self.query.is_empty() {
            Color::rgb(100, 100, 100)
        } else {
            Color::WHITE
        };

        for (i, ch) in display_text.chars().enumerate() {
            if input_x + i as u16 >= x + width - 2 {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(text_color);
            cell.bg = Some(self.bg_color);
            ctx.buffer.set(input_x + i as u16, current_y, cell);
        }

        // Cursor
        if !self.query.is_empty() || display_text == &self.placeholder {
            let cursor_x = input_x + self.query.len() as u16;
            if cursor_x < x + width - 2 {
                let mut cursor = Cell::new('▏');
                cursor.fg = Some(Color::WHITE);
                cursor.bg = Some(self.bg_color);
                ctx.buffer.set(cursor_x, current_y, cursor);
            }
        }

        let mut right = Cell::new(border_chars[5]);
        right.fg = Some(self.border_color);
        ctx.buffer.set(x + width - 1, current_y, right);

        current_y += 1;

        // Separator
        let mut sl = Cell::new('├');
        sl.fg = Some(self.border_color);
        ctx.buffer.set(x, current_y, sl);

        for dx in 1..width - 1 {
            let mut h = Cell::new('─');
            h.fg = Some(self.border_color);
            ctx.buffer.set(x + dx, current_y, h);
        }

        let mut sr = Cell::new('┤');
        sr.fg = Some(self.border_color);
        ctx.buffer.set(x + width - 1, current_y, sr);

        current_y += 1;

        // Command items
        let scroll_offset = self.selection.offset();
        let visible_items: Vec<_> = self
            .filtered
            .iter()
            .skip(scroll_offset)
            .take(self.max_visible as usize)
            .collect();

        for (i, &cmd_idx) in visible_items.iter().enumerate() {
            let cmd = &self.commands[*cmd_idx];
            let is_selected = self.selection.is_selected(scroll_offset + i);
            let item_y = current_y + i as u16;

            // Left border
            let mut left = Cell::new(border_chars[5]);
            left.fg = Some(self.border_color);
            ctx.buffer.set(x, item_y, left);

            // Background for selected
            let row_bg = if is_selected {
                self.selected_bg
            } else {
                self.bg_color
            };
            for dx in 1..width - 1 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(row_bg);
                ctx.buffer.set(x + dx, item_y, cell);
            }

            let mut content_x = x + 2;

            // Icon
            if self.show_icons {
                if let Some(icon) = cmd.icon {
                    let mut cell = Cell::new(icon);
                    cell.fg = Some(Color::CYAN);
                    cell.bg = Some(row_bg);
                    ctx.buffer.set(content_x, item_y, cell);
                }
                content_x += 2;
            }

            // Label with highlighting
            let highlighted = self.highlight_match(&cmd.label);
            for (ch, is_match) in highlighted {
                if content_x >= x + width - 15 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(if is_match {
                    self.match_color
                } else {
                    Color::WHITE
                });
                cell.bg = Some(row_bg);
                if is_match {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(content_x, item_y, cell);
                content_x += 1;
            }

            // Shortcut (right-aligned)
            if self.show_shortcuts {
                if let Some(ref shortcut) = cmd.shortcut {
                    let shortcut_x = x + width - 2 - shortcut.len() as u16;
                    for (i, ch) in shortcut.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::rgb(120, 120, 120));
                        cell.bg = Some(row_bg);
                        ctx.buffer.set(shortcut_x + i as u16, item_y, cell);
                    }
                }
            }

            // Right border
            let mut right = Cell::new(border_chars[5]);
            right.fg = Some(self.border_color);
            ctx.buffer.set(x + width - 1, item_y, right);
        }

        // Fill remaining space if fewer items than max_visible
        for i in visible_items.len()..self.max_visible as usize {
            let item_y = current_y + i as u16;
            if item_y >= y + height - 1 {
                break;
            }

            let mut left = Cell::new(border_chars[5]);
            left.fg = Some(self.border_color);
            ctx.buffer.set(x, item_y, left);

            let mut right = Cell::new(border_chars[5]);
            right.fg = Some(self.border_color);
            ctx.buffer.set(x + width - 1, item_y, right);
        }

        // Bottom border
        let bottom_y = y + height - 1;
        let mut bl = Cell::new(border_chars[2]);
        bl.fg = Some(self.border_color);
        ctx.buffer.set(x, bottom_y, bl);

        for dx in 1..width - 1 {
            let mut h = Cell::new(border_chars[4]);
            h.fg = Some(self.border_color);
            ctx.buffer.set(x + dx, bottom_y, h);
        }

        let mut br = Cell::new(border_chars[3]);
        br.fg = Some(self.border_color);
        ctx.buffer.set(x + width - 1, bottom_y, br);

        // Results count
        let count_str = format!("{}/{}", self.filtered.len(), self.commands.len());
        let count_x = x + width - 2 - count_str.len() as u16;
        for (i, ch) in count_str.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(80, 80, 80));
            cell.bg = Some(self.bg_color);
            ctx.buffer.set(count_x + i as u16, bottom_y, cell);
        }
    }
}

impl_styled_view!(CommandPalette);
impl_props_builders!(CommandPalette);

/// Helper to create a command palette
pub fn command_palette() -> CommandPalette {
    CommandPalette::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_command_new() {
        let cmd = Command::new("test", "Test Command");
        assert_eq!(cmd.id, "test");
        assert_eq!(cmd.label, "Test Command");
    }

    #[test]
    fn test_command_builder() {
        let cmd = Command::new("save", "Save File")
            .description("Save the current file")
            .shortcut("Ctrl+S")
            .category("File")
            .icon('💾')
            .recent()
            .pinned();

        assert_eq!(cmd.description, Some("Save the current file".to_string()));
        assert_eq!(cmd.shortcut, Some("Ctrl+S".to_string()));
        assert!(cmd.recent);
        assert!(cmd.pinned);
    }

    #[test]
    fn test_command_matches() {
        let cmd = Command::new("save_file", "Save File").description("Save to disk");

        assert!(cmd.matches("save"));
        assert!(cmd.matches("file"));
        assert!(cmd.matches("sf")); // fuzzy: S_ave F_ile
        assert!(cmd.matches("svfl")); // fuzzy: S_a_V_e F_i_L_e
        assert!(cmd.matches("disk")); // description
        assert!(!cmd.matches("xyz"));
    }

    #[test]
    fn test_command_score() {
        let cmd = Command::new("save", "Save").pinned();

        assert!(cmd.match_score("save") > cmd.match_score("sav"));
        assert!(cmd.match_score("") > 0); // Pinned bonus
    }

    #[test]
    fn test_palette_new() {
        let p = CommandPalette::new();
        assert!(!p.is_visible());
        assert!(p.commands.is_empty());
    }

    #[test]
    fn test_palette_show_hide() {
        let mut p = CommandPalette::new();

        p.show();
        assert!(p.is_visible());

        p.hide();
        assert!(!p.is_visible());

        p.toggle();
        assert!(p.is_visible());
    }

    #[test]
    fn test_palette_add_commands() {
        let p = CommandPalette::new()
            .command(Command::new("a", "Alpha"))
            .command(Command::new("b", "Beta"));

        assert_eq!(p.commands.len(), 2);
    }

    #[test]
    fn test_palette_filter() {
        let mut p = CommandPalette::new()
            .command(Command::new("save", "Save File"))
            .command(Command::new("open", "Open File"))
            .command(Command::new("close", "Close File"));

        p.show();
        assert_eq!(p.filtered.len(), 3);

        p.set_query("save");
        assert_eq!(p.filtered.len(), 1);

        p.clear_query();
        assert_eq!(p.filtered.len(), 3);
    }

    #[test]
    fn test_palette_selection() {
        let mut p = CommandPalette::new()
            .command(Command::new("a", "A"))
            .command(Command::new("b", "B"))
            .command(Command::new("c", "C"));

        p.show();

        assert_eq!(p.selection.index, 0);

        p.select_next();
        assert_eq!(p.selection.index, 1);

        p.select_next();
        assert_eq!(p.selection.index, 2);

        p.select_next();
        assert_eq!(p.selection.index, 0); // Wrap

        p.select_prev();
        assert_eq!(p.selection.index, 2); // Wrap back
    }

    #[test]
    fn test_palette_execute() {
        let mut p = CommandPalette::new().command(Command::new("test", "Test"));

        p.show();
        let result = p.execute();

        assert_eq!(result, Some("test".to_string()));
        assert!(!p.is_visible());
    }

    #[test]
    fn test_palette_input() {
        let mut p = CommandPalette::new();
        p.show();

        p.input('t');
        p.input('e');
        p.input('s');
        p.input('t');

        assert_eq!(p.query, "test");

        p.backspace();
        assert_eq!(p.query, "tes");
    }

    #[test]
    fn test_palette_render() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut p = CommandPalette::new()
            .title("Commands")
            .command(Command::new("test", "Test Command").shortcut("Ctrl+T"));

        p.show();
        p.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_palette_helper() {
        let p = command_palette().width(50);
        assert_eq!(p.width, 50);
    }

    #[test]
    fn test_highlight_match() {
        let p = CommandPalette::new();
        let result = p.highlight_match("Hello");
        assert_eq!(result.len(), 5);
        assert!(result.iter().all(|(_, m)| !m));

        let mut p = CommandPalette::new();
        p.query = "ell".to_string();
        let result = p.highlight_match("Hello");
        assert!(result[1].1); // 'e' matched
        assert!(result[2].1); // 'l' matched
        assert!(result[3].1); // 'l' matched
    }
}
