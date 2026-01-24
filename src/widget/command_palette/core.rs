use super::command::Command;
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::WidgetProps;

/// Command palette widget for quick command access
///
/// Provides a searchable command interface similar to VSCode's Ctrl+P
/// or Sublime Text's Command Palette.
pub struct CommandPalette {
    /// All commands
    pub commands: Vec<Command>,
    /// Current search query
    pub query: String,
    /// Filtered command indices
    pub filtered: Vec<usize>,
    /// Selection state for filtered list (uses Selection utility)
    pub selection: Selection,
    /// Visible state
    pub visible: bool,
    /// Width
    pub width: u16,
    /// Max visible items
    pub max_visible: u16,
    /// Placeholder text
    pub placeholder: String,
    /// Title
    pub title: Option<String>,
    /// Show descriptions
    pub show_descriptions: bool,
    /// Show shortcuts
    pub show_shortcuts: bool,
    /// Show icons
    pub show_icons: bool,
    /// Colors
    pub bg_color: Color,
    /// Border color
    pub border_color: Color,
    /// Selected item background
    pub selected_bg: Color,
    /// Match highlight color
    pub match_color: Color,
    /// Widget properties
    pub props: WidgetProps,
}
