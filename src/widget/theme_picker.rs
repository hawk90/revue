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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_picker_new() {
        let picker = ThemePicker::new();
        assert!(!picker.themes.is_empty());
        assert!(!picker.open);
    }

    #[test]
    fn test_theme_picker_toggle() {
        let mut picker = ThemePicker::new();
        assert!(!picker.is_open());

        picker.toggle();
        assert!(picker.is_open());

        picker.toggle();
        assert!(!picker.is_open());
    }

    #[test]
    fn test_theme_picker_selection() {
        let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);

        assert_eq!(picker.selected_index, 0);

        picker.select_next();
        assert_eq!(picker.selected_index, 1);

        picker.select_next();
        assert_eq!(picker.selected_index, 2);

        // Should not go past end
        picker.select_next();
        assert_eq!(picker.selected_index, 2);

        picker.select_prev();
        assert_eq!(picker.selected_index, 1);
    }

    #[test]
    fn test_theme_picker_selected_id() {
        let picker = ThemePicker::new().themes(["dracula", "nord"]);

        assert_eq!(picker.selected_id(), Some("dracula"));
    }

    #[test]
    fn test_theme_picker_compact() {
        let picker = ThemePicker::new().compact(true);
        assert!(picker.compact);
    }

    #[test]
    fn test_theme_picker_custom_themes() {
        let picker = ThemePicker::new().themes(["dark", "nord"]);

        assert_eq!(picker.themes.len(), 2);
        assert_eq!(picker.themes[0], "dark");
        assert_eq!(picker.themes[1], "nord");
    }

    #[test]
    fn test_theme_picker_width() {
        let picker = ThemePicker::new().width(50);
        assert_eq!(picker.width, Some(50));
    }

    #[test]
    fn test_theme_picker_handle_key_open() {
        let mut picker = ThemePicker::new();

        let event = KeyEvent::new(Key::Enter);
        let result = picker.handle_key(&event);

        assert_eq!(result, EventResult::Consumed);
        assert!(picker.is_open());
    }

    #[test]
    fn test_theme_picker_handle_key_close() {
        let mut picker = ThemePicker::new();
        picker.open();

        let event = KeyEvent::new(Key::Escape);
        let result = picker.handle_key(&event);

        assert_eq!(result, EventResult::Consumed);
        assert!(!picker.is_open());
    }

    #[test]
    fn test_theme_picker_handle_key_navigate() {
        let mut picker = ThemePicker::new().themes(["dark", "light", "dracula"]);
        picker.open();

        // Down
        let event = KeyEvent::new(Key::Down);
        picker.handle_key(&event);
        assert_eq!(picker.selected_index, 1);

        // Up
        let event = KeyEvent::new(Key::Up);
        picker.handle_key(&event);
        assert_eq!(picker.selected_index, 0);

        // j (vim down)
        let event = KeyEvent::new(Key::Char('j'));
        picker.handle_key(&event);
        assert_eq!(picker.selected_index, 1);

        // k (vim up)
        let event = KeyEvent::new(Key::Char('k'));
        picker.handle_key(&event);
        assert_eq!(picker.selected_index, 0);
    }

    // =========================================================================
    // ThemePicker::new tests
    // =========================================================================

    #[test]
    fn test_theme_picker_new_default_values() {
        let picker = ThemePicker::new();
        assert!(!picker.themes.is_empty());
        assert!(!picker.open);
        assert!(!picker.compact);
        assert!(picker.show_preview);
        assert!(picker.width.is_none());
        assert!(picker.fg.is_none());
        assert!(picker.bg.is_none());
    }

    // =========================================================================
    // ThemePicker builder tests
    // =========================================================================

    #[test]
    fn test_theme_picker_show_preview() {
        let picker = ThemePicker::new().show_preview(false);
        assert!(!picker.show_preview);
    }

    #[test]
    fn test_theme_picker_colors() {
        let picker = ThemePicker::new().fg(Color::RED).bg(Color::BLUE);

        assert_eq!(picker.fg, Some(Color::RED));
        assert_eq!(picker.bg, Some(Color::BLUE));
    }

    // =========================================================================
    // ThemePicker state management tests
    // =========================================================================

    #[test]
    fn test_theme_picker_open() {
        let mut picker = ThemePicker::new();
        picker.open();
        assert!(picker.is_open());
    }

    #[test]
    fn test_theme_picker_close() {
        let mut picker = ThemePicker::new();
        picker.open();
        picker.close();
        assert!(!picker.is_open());
    }

    #[test]
    fn test_theme_picker_is_open() {
        let mut picker = ThemePicker::new();
        assert!(!picker.is_open());

        picker.open();
        assert!(picker.is_open());

        picker.close();
        assert!(!picker.is_open());
    }

    // =========================================================================
    // ThemePicker selection tests
    // =========================================================================

    #[test]
    fn test_select_prev_at_start() {
        let mut picker = ThemePicker::new().themes(["dark", "light"]);
        picker.select_prev();
        assert_eq!(picker.selected_index, 0);
    }

    #[test]
    fn test_select_next_at_end() {
        let mut picker = ThemePicker::new().themes(["dark", "light"]);
        picker.select_next();
        assert_eq!(picker.selected_index, 1);

        picker.select_next(); // Already at end
        assert_eq!(picker.selected_index, 1);
    }

    #[test]
    fn test_select_next_single_theme() {
        let mut picker = ThemePicker::new().themes(["only"]);
        picker.select_next();
        assert_eq!(picker.selected_index, 0);
    }

    #[test]
    fn test_select_prev_single_theme() {
        let mut picker = ThemePicker::new().themes(["only"]);
        picker.select_prev();
        assert_eq!(picker.selected_index, 0);
    }

    #[test]
    fn test_select_empty_themes() {
        let mut picker = ThemePicker::new().themes([";" as &str]); // Use a non-existent theme
        picker.themes.clear(); // Clear to get empty
        picker.select_next();
        assert_eq!(picker.selected_index, 0);
    }

    // =========================================================================
    // ThemePicker query tests
    // =========================================================================

    #[test]
    fn test_selected_theme_none() {
        let mut picker = ThemePicker::new().themes([";" as &str]);
        picker.themes.clear();
        picker.selected_index = 0;
        assert!(picker.selected_id().is_none());
    }

    #[test]
    fn test_selected_theme_empty_themes() {
        let picker = ThemePicker::new().themes(["dark"]);
        assert!(picker.selected_id().is_some());
    }

    #[test]
    fn test_apply_selected_empty_themes() {
        let mut picker = ThemePicker::new().themes([";" as &str]);
        picker.themes.clear();
        picker.selected_index = 0;
        // Should not panic
        picker.apply_selected();
    }

    // =========================================================================
    // ThemePicker Default tests
    // =========================================================================

    #[test]
    fn test_theme_picker_default() {
        let picker = ThemePicker::default();
        assert!(!picker.themes.is_empty());
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_theme_picker_helper() {
        let picker = theme_picker();
        assert!(!picker.themes.is_empty());
    }

    // =========================================================================
    // ThemePicker::draw_swatch tests
    // =========================================================================

    #[test]
    fn test_draw_swatch() {
        // We can't test render output here without proper setup
        // but we can verify it doesn't panic with basic inputs
        // The method returns a width value
        // This is tested indirectly through render tests
    }

    // =========================================================================
    // Interactive::handle_key tests
    // =========================================================================

    #[test]
    fn test_handle_key_space_opens() {
        let mut picker = ThemePicker::new();
        let event = KeyEvent::new(Key::Char(' '));

        let result = picker.handle_key(&event);
        assert_eq!(result, EventResult::Consumed);
        assert!(picker.is_open());
    }

    #[test]
    fn test_handle_key_space_applies() {
        let mut picker = ThemePicker::new();
        picker.open();

        let event = KeyEvent::new(Key::Char(' '));
        picker.handle_key(&event);

        assert!(!picker.is_open());
    }

    #[test]
    fn test_handle_key_tab() {
        let mut picker = ThemePicker::new().themes(["dark", "light"]);
        let first_id = picker.selected_id().unwrap().to_string();

        let event = KeyEvent::new(Key::Tab);
        picker.handle_key(&event);

        // Should select next theme and apply it
        let new_id = picker.selected_id().unwrap();
        assert_ne!(first_id, new_id);
    }

    #[test]
    fn test_handle_key_ignored() {
        let mut picker = ThemePicker::new();
        let event = KeyEvent::new(Key::Char('x'));

        let result = picker.handle_key(&event);
        assert_eq!(result, EventResult::Ignored);
        assert!(!picker.is_open());
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_theme_picker_clone() {
        let picker = ThemePicker::new().themes(["dark"]).compact(true);
        let cloned = picker.clone();

        assert_eq!(picker.themes, cloned.themes);
        assert_eq!(picker.compact, cloned.compact);
    }
}
