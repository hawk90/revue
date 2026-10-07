//! Runtime theme switching: registry, listeners and shared access

use super::{Theme, Themes};
use crate::utils::lock::{read_or_recover, write_or_recover};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
    ///
    /// Switches between `dark_theme` (default: "dark") and `light_theme` (default: "light").
    /// Use [`Self::set_dark_theme`] and [`Self::set_light_theme`] to customize toggle targets.
    pub fn toggle_dark_light(&mut self) {
        // Clone is necessary here due to Rust borrowing rules - we can't hold
        // a reference to self.light_theme/self.dark_theme while calling set_theme(&mut self)
        let new_id = if self.current().is_dark() {
            self.light_theme.clone()
        } else {
            self.dark_theme.clone()
        };
        self.set_theme(new_id);
    }

    /// Cycle through all themes
    pub fn cycle(&mut self) {
        let ids: Vec<String> = self.themes.keys().cloned().collect();
        if ids.is_empty() {
            return;
        }

        let current_idx = ids
            .iter()
            .position(|id| id == &self.current_id)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % ids.len();
        self.set_theme(&ids[next_idx]);
    }

    /// Cycle through dark themes only
    pub fn cycle_dark(&mut self) {
        let dark_ids: Vec<String> = self
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
        self.set_theme(&dark_ids[next_idx]);
    }

    /// Cycle through light themes only
    pub fn cycle_light(&mut self) {
        let light_ids: Vec<String> = self
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
        self.set_theme(&light_ids[next_idx]);
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
        read_or_recover(&self.inner).current().clone()
    }

    /// Get current theme ID
    pub fn current_id(&self) -> String {
        read_or_recover(&self.inner).current_id().to_string()
    }

    /// Set theme
    pub fn set_theme(&self, id: impl Into<String>) -> bool {
        write_or_recover(&self.inner).set_theme(id)
    }

    /// Toggle dark/light
    pub fn toggle_dark_light(&self) {
        write_or_recover(&self.inner).toggle_dark_light();
    }

    /// Cycle themes
    pub fn cycle(&self) {
        write_or_recover(&self.inner).cycle();
    }

    /// Register theme
    pub fn register(&self, id: impl Into<String>, theme: Theme) {
        write_or_recover(&self.inner).register(id, theme);
    }

    /// Get theme IDs
    pub fn theme_ids(&self) -> Vec<String> {
        read_or_recover(&self.inner)
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
