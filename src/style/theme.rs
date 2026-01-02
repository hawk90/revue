//! Theme system for consistent styling
//!
//! Provides theme support for TUI applications including:
//! - Built-in themes (Dark, Light, High Contrast)
//! - Popular themes (Dracula, Nord, Monokai, Solarized)
//! - Theme switching at runtime
//! - Theme persistence
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::style::theme::{ThemeManager, Theme, Themes};
//!
//! let mut manager = ThemeManager::new();
//! manager.register("dracula", Themes::dracula());
//! manager.register("nord", Themes::nord());
//!
//! manager.set_theme("dracula");
//! println!("Current: {}", manager.current().name);
//!
//! manager.toggle_dark_light();
//! ```

use super::properties::Color;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Theme variant (dark, light, high contrast)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeVariant {
    /// Dark theme variant (default)
    #[default]
    Dark,
    /// Light theme variant
    Light,
    /// High contrast accessibility theme
    HighContrast,
}

/// Color palette
#[derive(Clone, Debug)]
pub struct Palette {
    /// Primary brand color
    pub primary: Color,
    /// Secondary accent color
    pub secondary: Color,
    /// Success/positive color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error/danger color
    pub error: Color,
    /// Info color
    pub info: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self::dark()
    }
}

impl Palette {
    /// Dark theme palette
    pub fn dark() -> Self {
        Self {
            primary: Color::rgb(66, 133, 244),   // Blue
            secondary: Color::rgb(156, 39, 176), // Purple
            success: Color::rgb(76, 175, 80),    // Green
            warning: Color::rgb(255, 193, 7),    // Amber
            error: Color::rgb(244, 67, 54),      // Red
            info: Color::rgb(33, 150, 243),      // Light Blue
        }
    }

    /// Light theme palette
    pub fn light() -> Self {
        Self {
            primary: Color::rgb(25, 118, 210),   // Darker blue
            secondary: Color::rgb(123, 31, 162), // Darker purple
            success: Color::rgb(56, 142, 60),    // Darker green
            warning: Color::rgb(255, 160, 0),    // Darker amber
            error: Color::rgb(211, 47, 47),      // Darker red
            info: Color::rgb(2, 136, 209),       // Darker light blue
        }
    }

    /// High contrast palette
    pub fn high_contrast() -> Self {
        Self {
            primary: Color::CYAN,
            secondary: Color::MAGENTA,
            success: Color::GREEN,
            warning: Color::YELLOW,
            error: Color::RED,
            info: Color::BLUE,
        }
    }
}

/// Theme colors
#[derive(Clone, Debug)]
pub struct ThemeColors {
    /// Background color
    pub background: Color,
    /// Surface color (cards, dialogs)
    pub surface: Color,
    /// Primary text color
    pub text: Color,
    /// Secondary/muted text color
    pub text_muted: Color,
    /// Border color
    pub border: Color,
    /// Divider color
    pub divider: Color,
    /// Selection background
    pub selection: Color,
    /// Selection text
    pub selection_text: Color,
    /// Focus ring color
    pub focus: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::dark()
    }
}

impl ThemeColors {
    /// Dark theme colors
    pub fn dark() -> Self {
        Self {
            background: Color::rgb(18, 18, 18),
            surface: Color::rgb(30, 30, 30),
            text: Color::rgb(255, 255, 255),
            text_muted: Color::rgb(158, 158, 158),
            border: Color::rgb(66, 66, 66),
            divider: Color::rgb(48, 48, 48),
            selection: Color::rgb(66, 133, 244),
            selection_text: Color::WHITE,
            focus: Color::rgb(66, 133, 244),
        }
    }

    /// Light theme colors
    pub fn light() -> Self {
        Self {
            background: Color::rgb(255, 255, 255),
            surface: Color::rgb(250, 250, 250),
            text: Color::rgb(33, 33, 33),
            text_muted: Color::rgb(117, 117, 117),
            border: Color::rgb(224, 224, 224),
            divider: Color::rgb(238, 238, 238),
            selection: Color::rgb(25, 118, 210),
            selection_text: Color::WHITE,
            focus: Color::rgb(25, 118, 210),
        }
    }

    /// High contrast colors
    pub fn high_contrast() -> Self {
        Self {
            background: Color::BLACK,
            surface: Color::BLACK,
            text: Color::WHITE,
            text_muted: Color::rgb(200, 200, 200),
            border: Color::WHITE,
            divider: Color::WHITE,
            selection: Color::YELLOW,
            selection_text: Color::BLACK,
            focus: Color::CYAN,
        }
    }
}

/// Complete theme
#[derive(Clone, Debug)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme variant
    pub variant: ThemeVariant,
    /// Color palette
    pub palette: Palette,
    /// Theme colors
    pub colors: ThemeColors,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Create dark theme
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette::dark(),
            colors: ThemeColors::dark(),
        }
    }

    /// Create light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            variant: ThemeVariant::Light,
            palette: Palette::light(),
            colors: ThemeColors::light(),
        }
    }

    /// Create high contrast theme
    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),
            variant: ThemeVariant::HighContrast,
            palette: Palette::high_contrast(),
            colors: ThemeColors::high_contrast(),
        }
    }

    /// Create a custom theme
    pub fn custom(name: impl Into<String>) -> ThemeBuilder {
        ThemeBuilder::new(name)
    }

    /// Check if theme is dark
    pub fn is_dark(&self) -> bool {
        self.variant == ThemeVariant::Dark
    }

    /// Check if theme is light
    pub fn is_light(&self) -> bool {
        self.variant == ThemeVariant::Light
    }
}

/// Theme builder
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    /// Create a new theme builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            theme: Theme {
                name: name.into(),
                ..Theme::dark()
            },
        }
    }

    /// Set theme variant
    pub fn variant(mut self, variant: ThemeVariant) -> Self {
        self.theme.variant = variant;
        self
    }

    /// Set palette
    pub fn palette(mut self, palette: Palette) -> Self {
        self.theme.palette = palette;
        self
    }

    /// Set colors
    pub fn colors(mut self, colors: ThemeColors) -> Self {
        self.theme.colors = colors;
        self
    }

    /// Set primary color
    pub fn primary(mut self, color: Color) -> Self {
        self.theme.palette.primary = color;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.theme.colors.background = color;
        self
    }

    /// Set text color
    pub fn text(mut self, color: Color) -> Self {
        self.theme.colors.text = color;
        self
    }

    /// Build the theme
    pub fn build(self) -> Theme {
        self.theme
    }
}

/// Predefined themes
pub struct Themes;

impl Themes {
    /// Dracula theme
    pub fn dracula() -> Theme {
        Theme {
            name: "Dracula".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette {
                primary: Color::rgb(139, 233, 253),   // Cyan
                secondary: Color::rgb(255, 121, 198), // Pink
                success: Color::rgb(80, 250, 123),    // Green
                warning: Color::rgb(241, 250, 140),   // Yellow
                error: Color::rgb(255, 85, 85),       // Red
                info: Color::rgb(189, 147, 249),      // Purple
            },
            colors: ThemeColors {
                background: Color::rgb(40, 42, 54),
                surface: Color::rgb(68, 71, 90),
                text: Color::rgb(248, 248, 242),
                text_muted: Color::rgb(98, 114, 164),
                border: Color::rgb(68, 71, 90),
                divider: Color::rgb(68, 71, 90),
                selection: Color::rgb(68, 71, 90),
                selection_text: Color::rgb(248, 248, 242),
                focus: Color::rgb(139, 233, 253),
            },
        }
    }

    /// Nord theme
    pub fn nord() -> Theme {
        Theme {
            name: "Nord".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette {
                primary: Color::rgb(136, 192, 208),  // Frost
                secondary: Color::rgb(180, 142, 173), // Aurora purple
                success: Color::rgb(163, 190, 140),  // Aurora green
                warning: Color::rgb(235, 203, 139),  // Aurora yellow
                error: Color::rgb(191, 97, 106),     // Aurora red
                info: Color::rgb(129, 161, 193),     // Frost blue
            },
            colors: ThemeColors {
                background: Color::rgb(46, 52, 64),
                surface: Color::rgb(59, 66, 82),
                text: Color::rgb(236, 239, 244),
                text_muted: Color::rgb(147, 161, 161),
                border: Color::rgb(76, 86, 106),
                divider: Color::rgb(67, 76, 94),
                selection: Color::rgb(76, 86, 106),
                selection_text: Color::rgb(236, 239, 244),
                focus: Color::rgb(136, 192, 208),
            },
        }
    }

    /// Monokai theme
    pub fn monokai() -> Theme {
        Theme {
            name: "Monokai".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette {
                primary: Color::rgb(102, 217, 239),  // Cyan
                secondary: Color::rgb(174, 129, 255), // Purple
                success: Color::rgb(166, 226, 46),   // Green
                warning: Color::rgb(253, 151, 31),   // Orange
                error: Color::rgb(249, 38, 114),     // Red/Pink
                info: Color::rgb(102, 217, 239),     // Cyan
            },
            colors: ThemeColors {
                background: Color::rgb(39, 40, 34),
                surface: Color::rgb(49, 50, 44),
                text: Color::rgb(248, 248, 242),
                text_muted: Color::rgb(117, 113, 94),
                border: Color::rgb(73, 72, 62),
                divider: Color::rgb(73, 72, 62),
                selection: Color::rgb(73, 72, 62),
                selection_text: Color::rgb(248, 248, 242),
                focus: Color::rgb(166, 226, 46),
            },
        }
    }

    /// Solarized Dark theme
    pub fn solarized_dark() -> Theme {
        Theme {
            name: "Solarized Dark".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette {
                primary: Color::rgb(38, 139, 210),   // Blue
                secondary: Color::rgb(108, 113, 196), // Violet
                success: Color::rgb(133, 153, 0),    // Green
                warning: Color::rgb(181, 137, 0),    // Yellow
                error: Color::rgb(220, 50, 47),      // Red
                info: Color::rgb(42, 161, 152),      // Cyan
            },
            colors: ThemeColors {
                background: Color::rgb(0, 43, 54),
                surface: Color::rgb(7, 54, 66),
                text: Color::rgb(131, 148, 150),
                text_muted: Color::rgb(88, 110, 117),
                border: Color::rgb(7, 54, 66),
                divider: Color::rgb(7, 54, 66),
                selection: Color::rgb(7, 54, 66),
                selection_text: Color::rgb(147, 161, 161),
                focus: Color::rgb(38, 139, 210),
            },
        }
    }

    /// Solarized Light theme
    pub fn solarized_light() -> Theme {
        Theme {
            name: "Solarized Light".to_string(),
            variant: ThemeVariant::Light,
            palette: Palette {
                primary: Color::rgb(38, 139, 210),   // Blue
                secondary: Color::rgb(108, 113, 196), // Violet
                success: Color::rgb(133, 153, 0),    // Green
                warning: Color::rgb(181, 137, 0),    // Yellow
                error: Color::rgb(220, 50, 47),      // Red
                info: Color::rgb(42, 161, 152),      // Cyan
            },
            colors: ThemeColors {
                background: Color::rgb(253, 246, 227),
                surface: Color::rgb(238, 232, 213),
                text: Color::rgb(101, 123, 131),
                text_muted: Color::rgb(147, 161, 161),
                border: Color::rgb(238, 232, 213),
                divider: Color::rgb(238, 232, 213),
                selection: Color::rgb(238, 232, 213),
                selection_text: Color::rgb(88, 110, 117),
                focus: Color::rgb(38, 139, 210),
            },
        }
    }
}

/// Theme change listener type
pub type ThemeChangeListener = Box<dyn Fn(&Theme) + Send + Sync>;

/// Theme manager for runtime theme switching
pub struct ThemeManager {
    /// Registered themes
    themes: HashMap<String, Theme>,
    /// Current theme ID
    current_id: String,
    /// Theme change listeners
    listeners: Vec<ThemeChangeListener>,
    /// Light theme ID for toggling
    light_theme: String,
    /// Dark theme ID for toggling
    dark_theme: String,
}

impl ThemeManager {
    /// Create a new theme manager with default themes
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_id: "dark".to_string(),
            listeners: Vec::new(),
            light_theme: "light".to_string(),
            dark_theme: "dark".to_string(),
        };

        // Register default themes
        manager.register("dark", Theme::dark());
        manager.register("light", Theme::light());
        manager.register("high_contrast", Theme::high_contrast());
        manager.register("dracula", Themes::dracula());
        manager.register("nord", Themes::nord());
        manager.register("monokai", Themes::monokai());
        manager.register("solarized_dark", Themes::solarized_dark());
        manager.register("solarized_light", Themes::solarized_light());

        manager
    }

    /// Create theme manager with custom initial theme
    pub fn with_theme(theme_id: impl Into<String>) -> Self {
        let mut manager = Self::new();
        let id = theme_id.into();
        if manager.themes.contains_key(&id) {
            manager.current_id = id;
        }
        manager
    }

    /// Register a theme
    pub fn register(&mut self, id: impl Into<String>, theme: Theme) {
        self.themes.insert(id.into(), theme);
    }

    /// Unregister a theme
    pub fn unregister(&mut self, id: &str) -> Option<Theme> {
        // Don't remove current theme
        if id == self.current_id {
            return None;
        }
        self.themes.remove(id)
    }

    /// Set current theme by ID
    pub fn set_theme(&mut self, id: impl Into<String>) -> bool {
        let id = id.into();
        if self.themes.contains_key(&id) {
            self.current_id = id;
            self.notify_change();
            true
        } else {
            false
        }
    }

    /// Get current theme
    pub fn current(&self) -> &Theme {
        self.themes.get(&self.current_id).unwrap_or_else(|| {
            static DEFAULT: std::sync::OnceLock<Theme> = std::sync::OnceLock::new();
            DEFAULT.get_or_init(Theme::dark)
        })
    }

    /// Get current theme ID
    pub fn current_id(&self) -> &str {
        &self.current_id
    }

    /// Get theme by ID
    pub fn get(&self, id: &str) -> Option<&Theme> {
        self.themes.get(id)
    }

    /// Get all registered theme IDs
    pub fn theme_ids(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }

    /// Get all registered themes
    pub fn themes(&self) -> impl Iterator<Item = (&str, &Theme)> {
        self.themes.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Set light theme for toggling
    pub fn set_light_theme(&mut self, id: impl Into<String>) {
        self.light_theme = id.into();
    }

    /// Set dark theme for toggling
    pub fn set_dark_theme(&mut self, id: impl Into<String>) {
        self.dark_theme = id.into();
    }

    /// Toggle between dark and light theme
    pub fn toggle_dark_light(&mut self) {
        let new_id = if self.current().is_dark() {
            self.light_theme.clone()
        } else {
            self.dark_theme.clone()
        };
        self.set_theme(new_id);
    }

    /// Cycle through all themes
    pub fn cycle(&mut self) {
        let ids: Vec<_> = self.themes.keys().cloned().collect();
        if ids.is_empty() {
            return;
        }

        let current_idx = ids.iter().position(|id| id == &self.current_id).unwrap_or(0);
        let next_idx = (current_idx + 1) % ids.len();
        self.set_theme(ids[next_idx].clone());
    }

    /// Cycle through dark themes only
    pub fn cycle_dark(&mut self) {
        let dark_ids: Vec<_> = self
            .themes
            .iter()
            .filter(|(_, t)| t.is_dark())
            .map(|(id, _)| id.clone())
            .collect();

        if dark_ids.is_empty() {
            return;
        }

        let current_idx = dark_ids
            .iter()
            .position(|id| id == &self.current_id)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % dark_ids.len();
        self.set_theme(dark_ids[next_idx].clone());
    }

    /// Cycle through light themes only
    pub fn cycle_light(&mut self) {
        let light_ids: Vec<_> = self
            .themes
            .iter()
            .filter(|(_, t)| t.is_light())
            .map(|(id, _)| id.clone())
            .collect();

        if light_ids.is_empty() {
            return;
        }

        let current_idx = light_ids
            .iter()
            .position(|id| id == &self.current_id)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % light_ids.len();
        self.set_theme(light_ids[next_idx].clone());
    }

    /// Add theme change listener
    pub fn on_change<F>(&mut self, listener: F)
    where
        F: Fn(&Theme) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    /// Notify listeners of theme change
    fn notify_change(&self) {
        let theme = self.current();
        for listener in &self.listeners {
            listener(theme);
        }
    }

    /// Check if a theme is registered
    pub fn has_theme(&self, id: &str) -> bool {
        self.themes.contains_key(id)
    }

    /// Get number of registered themes
    pub fn len(&self) -> usize {
        self.themes.len()
    }

    /// Check if manager has no themes
    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global theme state for shared access
#[derive(Clone)]
pub struct SharedTheme {
    inner: Arc<RwLock<ThemeManager>>,
}

impl SharedTheme {
    /// Create new shared theme
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(ThemeManager::new())),
        }
    }

    /// Create with specific initial theme
    pub fn with_theme(id: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(ThemeManager::with_theme(id))),
        }
    }

    /// Get current theme (cloned)
    pub fn current(&self) -> Theme {
        self.inner.read().unwrap().current().clone()
    }

    /// Get current theme ID
    pub fn current_id(&self) -> String {
        self.inner.read().unwrap().current_id().to_string()
    }

    /// Set theme
    pub fn set_theme(&self, id: impl Into<String>) -> bool {
        self.inner.write().unwrap().set_theme(id)
    }

    /// Toggle dark/light
    pub fn toggle_dark_light(&self) {
        self.inner.write().unwrap().toggle_dark_light();
    }

    /// Cycle themes
    pub fn cycle(&self) {
        self.inner.write().unwrap().cycle();
    }

    /// Register theme
    pub fn register(&self, id: impl Into<String>, theme: Theme) {
        self.inner.write().unwrap().register(id, theme);
    }

    /// Get theme IDs
    pub fn theme_ids(&self) -> Vec<String> {
        self.inner
            .read()
            .unwrap()
            .theme_ids()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }
}

impl Default for SharedTheme {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a theme manager
pub fn theme_manager() -> ThemeManager {
    ThemeManager::new()
}

/// Create a shared theme
pub fn shared_theme() -> SharedTheme {
    SharedTheme::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_dark() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "Dark");
        assert!(theme.is_dark());
        assert!(!theme.is_light());
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.name, "Light");
        assert!(theme.is_light());
        assert!(!theme.is_dark());
    }

    #[test]
    fn test_theme_high_contrast() {
        let theme = Theme::high_contrast();
        assert_eq!(theme.name, "High Contrast");
        assert_eq!(theme.variant, ThemeVariant::HighContrast);
    }

    #[test]
    fn test_theme_builder() {
        let theme = Theme::custom("My Theme")
            .variant(ThemeVariant::Dark)
            .primary(Color::RED)
            .build();

        assert_eq!(theme.name, "My Theme");
        assert_eq!(theme.palette.primary, Color::RED);
    }

    #[test]
    fn test_palette_dark() {
        let palette = Palette::dark();
        assert_ne!(palette.primary, Color::BLACK);
    }

    #[test]
    fn test_palette_light() {
        let palette = Palette::light();
        assert_ne!(palette.primary, Color::BLACK);
    }

    #[test]
    fn test_theme_colors_dark() {
        let colors = ThemeColors::dark();
        assert_ne!(colors.background, Color::WHITE);
    }

    #[test]
    fn test_theme_colors_light() {
        let colors = ThemeColors::light();
        assert_ne!(colors.text, Color::WHITE);
    }

    #[test]
    fn test_themes_dracula() {
        let theme = Themes::dracula();
        assert_eq!(theme.name, "Dracula");
    }

    #[test]
    fn test_themes_nord() {
        let theme = Themes::nord();
        assert_eq!(theme.name, "Nord");
    }

    #[test]
    fn test_themes_monokai() {
        let theme = Themes::monokai();
        assert_eq!(theme.name, "Monokai");
    }

    #[test]
    fn test_themes_solarized() {
        let dark = Themes::solarized_dark();
        let light = Themes::solarized_light();

        assert!(dark.is_dark());
        assert!(light.is_light());
    }

    #[test]
    fn test_theme_manager_new() {
        let manager = ThemeManager::new();
        assert_eq!(manager.current_id(), "dark");
        assert!(manager.has_theme("dark"));
        assert!(manager.has_theme("light"));
        assert!(manager.has_theme("dracula"));
    }

    #[test]
    fn test_theme_manager_set_theme() {
        let mut manager = ThemeManager::new();
        assert!(manager.set_theme("nord"));
        assert_eq!(manager.current_id(), "nord");
        assert_eq!(manager.current().name, "Nord");
    }

    #[test]
    fn test_theme_manager_set_invalid() {
        let mut manager = ThemeManager::new();
        assert!(!manager.set_theme("nonexistent"));
        assert_eq!(manager.current_id(), "dark");
    }

    #[test]
    fn test_theme_manager_toggle() {
        let mut manager = ThemeManager::new();
        assert!(manager.current().is_dark());

        manager.toggle_dark_light();
        assert!(manager.current().is_light());

        manager.toggle_dark_light();
        assert!(manager.current().is_dark());
    }

    #[test]
    fn test_theme_manager_register() {
        let mut manager = ThemeManager::new();
        let custom = Theme::custom("Custom")
            .primary(Color::MAGENTA)
            .build();

        manager.register("custom", custom);
        assert!(manager.has_theme("custom"));
        manager.set_theme("custom");
        assert_eq!(manager.current().name, "Custom");
    }

    #[test]
    fn test_theme_manager_unregister() {
        let mut manager = ThemeManager::new();
        assert!(manager.unregister("dracula").is_some());
        assert!(!manager.has_theme("dracula"));
    }

    #[test]
    fn test_theme_manager_unregister_current() {
        let mut manager = ThemeManager::new();
        // Cannot unregister current theme
        assert!(manager.unregister("dark").is_none());
        assert!(manager.has_theme("dark"));
    }

    #[test]
    fn test_theme_manager_cycle() {
        let mut manager = ThemeManager::new();
        let initial = manager.current_id().to_string();

        manager.cycle();
        let after_cycle = manager.current_id().to_string();

        // Should have changed
        assert_ne!(initial, after_cycle);
    }

    #[test]
    fn test_theme_manager_len() {
        let manager = ThemeManager::new();
        assert_eq!(manager.len(), 8); // Default themes count
        assert!(!manager.is_empty());
    }

    #[test]
    fn test_theme_manager_with_theme() {
        let manager = ThemeManager::with_theme("dracula");
        assert_eq!(manager.current_id(), "dracula");
    }

    #[test]
    fn test_shared_theme() {
        let shared = SharedTheme::new();
        assert_eq!(shared.current_id(), "dark");

        shared.set_theme("light");
        assert_eq!(shared.current_id(), "light");
    }

    #[test]
    fn test_shared_theme_toggle() {
        let shared = SharedTheme::new();
        assert!(shared.current().is_dark());

        shared.toggle_dark_light();
        assert!(shared.current().is_light());
    }

    #[test]
    fn test_theme_manager_helper() {
        let manager = theme_manager();
        assert!(!manager.is_empty());
    }

    #[test]
    fn test_shared_theme_helper() {
        let shared = shared_theme();
        assert_eq!(shared.current_id(), "dark");
    }
}
