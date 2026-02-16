//! Theme picker widget for runtime theme switching
//!
//! Provides a dropdown-style widget for selecting themes from the registered
//! theme list. Integrates with the reactive theme system for automatic UI updates.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::theme_picker;
//!
//! // Basic theme picker
//! theme_picker().render(ctx);
//!
//! // With specific themes only
//! theme_picker()
//!     .themes(["dark", "light", "dracula", "nord"])
//!     .render(ctx);
//!
//! // Compact mode (shows only color swatches)
//! theme_picker()
//!     .compact(true)
//!     .render(ctx);
//! ```

use crate::event::{Key, KeyEvent};
use crate::render::Cell;
use crate::style::{get_theme, set_theme_by_id, theme_ids, use_theme, Color, Theme};
use crate::widget::traits::{EventResult, Interactive, RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Theme picker widget for selecting themes
#[derive(Clone, Debug)]
pub struct ThemePicker {
    /// Available theme IDs
    themes: Vec<String>,
    /// Currently selected index in themes list
    selected_index: usize,
    /// Whether dropdown is open
    open: bool,
    /// Show compact color swatch mode
    compact: bool,
    /// Show theme preview
    show_preview: bool,
    /// Widget width
    width: Option<u16>,
    /// Foreground color override
    fg: Option<Color>,
    /// Background color override
    bg: Option<Color>,
    /// CSS styling properties
    props: WidgetProps,
}

impl ThemePicker {
    /// Create a new theme picker with all registered themes
    pub fn new() -> Self {
        let all_themes = theme_ids();
        let current = use_theme().get();
        let selected_index = all_themes
            .iter()
            .position(|id| {
                get_theme(id)
                    .map(|t| t.name == current.name)
                    .unwrap_or(false)
            })
            .unwrap_or(0);

        Self {
            themes: all_themes,
            selected_index,
            open: false,
            compact: false,
            show_preview: true,
            width: None,
            fg: None,
            bg: None,
            props: WidgetProps::new(),
        }
    }

    /// Set specific themes to show (by ID)
    pub fn themes<I, S>(mut self, theme_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.themes = theme_ids.into_iter().map(|s| s.into()).collect();
        self.selected_index = 0;
        self
    }

    /// Enable compact mode (color swatches only)
    pub fn compact(mut self, enable: bool) -> Self {
        self.compact = enable;
        self
    }

    /// Show theme preview (default: true)
    pub fn show_preview(mut self, show: bool) -> Self {
        self.show_preview = show;
        self
    }

    /// Set widget width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
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

    /// Toggle dropdown open/closed
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    /// Open the dropdown
    pub fn open(&mut self) {
        self.open = true;
    }

    /// Close the dropdown
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Check if dropdown is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Move selection up
    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if self.selected_index < self.themes.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Apply the selected theme
    pub fn apply_selected(&self) {
        if let Some(id) = self.themes.get(self.selected_index) {
            set_theme_by_id(id);
        }
    }

    /// Get currently selected theme ID
    pub fn selected_id(&self) -> Option<&str> {
        self.themes.get(self.selected_index).map(|s| s.as_str())
    }

    /// Get selected theme
    pub fn selected_theme(&self) -> Option<Theme> {
        self.selected_id().and_then(get_theme)
    }

    /// Draw color swatch at position, returns width used
    fn draw_swatch(&self, ctx: &mut RenderContext, x: u16, y: u16, theme: &Theme) -> u16 {
        let swatch_colors = [
            theme.colors.background,
            theme.palette.primary,
            theme.palette.success,
            theme.palette.error,
        ];

        for (i, color) in swatch_colors.iter().enumerate() {
            let mut cell = Cell::new(' ');
            cell.bg = Some(*color);
            ctx.buffer.set(x + i as u16, y, cell);
        }

        swatch_colors.len() as u16
    }
}

impl Default for ThemePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ThemePicker {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 1 {
            return;
        }

        let current_theme = use_theme().get();
        let width = self.width.unwrap_or(area.width.min(35));

        let fg = self.fg.unwrap_or(current_theme.colors.text);
        let bg = self.bg.unwrap_or(current_theme.colors.surface);

        if self.compact {
            // Compact mode: just show current theme swatch
            self.draw_swatch(ctx, area.x, area.y, &current_theme);

            if self.open && !self.themes.is_empty() {
                let mut y = area.y + 1;
                for (i, theme_id) in self.themes.iter().enumerate() {
                    if y >= area.y + area.height {
                        break;
                    }
                    if let Some(theme) = get_theme(theme_id) {
                        let selected = i == self.selected_index;

                        // Selection indicator
                        let indicator = if selected { '>' } else { ' ' };
                        let mut cell = Cell::new(indicator);
                        cell.fg = Some(theme.palette.primary);
                        ctx.buffer.set(area.x, y, cell);

                        // Swatch
                        self.draw_swatch(ctx, area.x + 1, y, &theme);
                        y += 1;
                    }
                }
            }
        } else {
            // Full mode: show current theme with name
            let mut x = area.x;

            // "Theme: " label
            let label = "Theme: ";
            for ch in label.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }

            // Theme name
            for ch in current_theme.name.chars() {
                if x >= area.x + width - 6 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }

            // Space
            let mut cell = Cell::new(' ');
            cell.bg = Some(bg);
            ctx.buffer.set(x, area.y, cell);
            x += 1;

            // Swatch
            x += self.draw_swatch(ctx, x, area.y, &current_theme);

            // Dropdown indicator
            let indicator = if self.open { " ▲" } else { " ▼" };
            for ch in indicator.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }

            // Fill remaining header
            while x < area.x + width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }

            // Dropdown content
            if self.open && !self.themes.is_empty() {
                let border_color = current_theme.colors.border;
                let mut y = area.y + 1;

                // Border top
                if y < area.y + area.height {
                    let mut cell = Cell::new('┌');
                    cell.fg = Some(border_color);
                    ctx.buffer.set(area.x, y, cell);

                    for i in 1..width.saturating_sub(1) {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(border_color);
                        ctx.buffer.set(area.x + i, y, cell);
                    }

                    let mut cell = Cell::new('┐');
                    cell.fg = Some(border_color);
                    ctx.buffer.set(area.x + width - 1, y, cell);
                    y += 1;
                }

                // Theme list
                for (i, theme_id) in self.themes.iter().enumerate() {
                    if y >= area.y + area.height - 1 {
                        break;
                    }
                    if let Some(theme) = get_theme(theme_id) {
                        let selected = i == self.selected_index;
                        let item_bg = if selected {
                            current_theme.colors.selection
                        } else {
                            current_theme.colors.surface
                        };
                        let item_fg = if selected {
                            current_theme.colors.selection_text
                        } else {
                            current_theme.colors.text
                        };

                        // Left border
                        let mut cell = Cell::new('│');
                        cell.fg = Some(border_color);
                        ctx.buffer.set(area.x, y, cell);

                        let mut cx = area.x + 1;

                        // Selection indicator
                        let indicator = if selected { '▶' } else { ' ' };
                        let mut cell = Cell::new(indicator);
                        cell.fg = Some(theme.palette.primary);
                        cell.bg = Some(item_bg);
                        ctx.buffer.set(cx, y, cell);
                        cx += 1;

                        // Swatch
                        cx += self.draw_swatch(ctx, cx, y, &theme);

                        // Space
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(item_bg);
                        ctx.buffer.set(cx, y, cell);
                        cx += 1;

                        // Name
                        let max_name_len = (width as usize).saturating_sub(9);
                        for (j, ch) in theme.name.chars().enumerate() {
                            if j >= max_name_len {
                                break;
                            }
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(item_fg);
                            cell.bg = Some(item_bg);
                            ctx.buffer.set(cx, y, cell);
                            cx += 1;
                        }

                        // Padding
                        while cx < area.x + width - 1 {
                            let mut cell = Cell::new(' ');
                            cell.bg = Some(item_bg);
                            ctx.buffer.set(cx, y, cell);
                            cx += 1;
                        }

                        // Right border
                        let mut cell = Cell::new('│');
                        cell.fg = Some(border_color);
                        ctx.buffer.set(area.x + width - 1, y, cell);

                        y += 1;
                    }
                }

                // Border bottom
                if y < area.y + area.height {
                    let mut cell = Cell::new('└');
                    cell.fg = Some(border_color);
                    ctx.buffer.set(area.x, y, cell);

                    for i in 1..width.saturating_sub(1) {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(border_color);
                        ctx.buffer.set(area.x + i, y, cell);
                    }

                    let mut cell = Cell::new('┘');
                    cell.fg = Some(border_color);
                    ctx.buffer.set(area.x + width - 1, y, cell);
                }
            }
        }
    }
}

impl Interactive for ThemePicker {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        match event.key {
            Key::Enter | Key::Char(' ') => {
                if self.open {
                    self.apply_selected();
                    self.close();
                } else {
                    self.open();
                }
                EventResult::Consumed
            }
            Key::Up | Key::Char('k') if self.open => {
                self.select_prev();
                EventResult::Consumed
            }
            Key::Down | Key::Char('j') if self.open => {
                self.select_next();
                EventResult::Consumed
            }
            Key::Escape if self.open => {
                self.close();
                EventResult::Consumed
            }
            Key::Tab => {
                // Apply next theme without opening dropdown
                self.select_next();
                if self.selected_index == 0 && !self.themes.is_empty() {
                    // Wrapped around, go to first
                }
                self.apply_selected();
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}

// Implement styled view macros
impl_styled_view!(ThemePicker);
impl_props_builders!(ThemePicker);

/// Create a new theme picker
pub fn theme_picker() -> ThemePicker {
    ThemePicker::new()
}
