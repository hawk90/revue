//! Construction and builder methods.

use super::{command::Command, CommandPalette};
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::theme::{DARK_GRAY, EDITOR_BG};
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
            bg_color: EDITOR_BG,
            border_color: DARK_GRAY,
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
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}
