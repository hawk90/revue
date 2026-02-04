//! Signal-based theme system for reactive theme switching
//!
//! Provides a reactive theme system where theme changes automatically
//! trigger UI updates through the Signal system.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::style::{use_theme, set_theme, Theme, Themes};
//!
//! // Get the current theme as a reactive signal
//! let theme = use_theme();
//!
//! // Theme changes trigger reactive updates
//! effect(move || {
//!     let current = theme.get();
//!     println!("Theme changed to: {}", current.name);
//! });
//!
//! // Change theme (triggers all effects)
//! set_theme(Themes::nord());
//! ```

use super::theme::{Theme, ThemeManager};
use crate::reactive::{signal, Signal};
use crate::utils::lock::lock_or_recover;
use std::cell::RefCell;
use std::sync::{Arc, Mutex, OnceLock};

// ─────────────────────────────────────────────────────────────────────────────
// Thread-local Theme Signal
// ─────────────────────────────────────────────────────────────────────────────

thread_local! {
    /// Thread-local theme signal (Signal uses Rc internally, so not Send+Sync)
    static THEME_SIGNAL: RefCell<Option<Signal<Theme>>> = const { RefCell::new(None) };
}

/// Global theme manager for registration (this IS Send+Sync safe)
static THEME_MANAGER: OnceLock<Arc<Mutex<ThemeManager>>> = OnceLock::new();

/// Get or create the thread-local theme signal
fn get_theme_signal() -> Signal<Theme> {
    THEME_SIGNAL.with(|cell| {
        let mut opt = cell.borrow_mut();
        if opt.is_none() {
            *opt = Some(signal(Theme::dark()));
        }
        opt.as_ref().unwrap().clone()
    })
}

/// Get or create the global theme manager
fn get_theme_manager() -> Arc<Mutex<ThemeManager>> {
    THEME_MANAGER
        .get_or_init(|| Arc::new(Mutex::new(ThemeManager::new())))
        .clone()
}

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

/// Get the current theme as a reactive signal
///
/// The returned signal automatically updates when the theme changes,
/// triggering any effects or computed values that depend on it.
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::use_theme;
///
/// let theme = use_theme();
///
/// // Read current theme
/// let current = theme.get();
/// println!("Using theme: {}", current.name);
///
/// // React to theme changes
/// effect(move || {
///     let t = theme.get();
///     apply_colors(&t.colors);
/// });
/// ```
pub fn use_theme() -> Signal<Theme> {
    get_theme_signal()
}

/// Set the current theme
///
/// This triggers reactive updates to all components using `use_theme()`.
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::{set_theme, Themes};
///
/// set_theme(Themes::dracula());
/// ```
pub fn set_theme(theme: Theme) {
    get_theme_signal().set(theme);
}

/// Set theme by ID
///
/// Uses the theme manager to look up a registered theme by ID.
/// Returns `true` if the theme was found and set.
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::set_theme_by_id;
///
/// set_theme_by_id("nord");
/// ```
pub fn set_theme_by_id(id: &str) -> bool {
    let manager = get_theme_manager();
    let guard = lock_or_recover(&manager);

    if let Some(theme) = guard.get(id) {
        let theme = theme.clone();
        drop(guard); // Release lock before setting signal
        get_theme_signal().set(theme);
        true
    } else {
        false
    }
}

/// Toggle between dark and light themes
///
/// Switches to light theme if currently dark, and vice versa.
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::toggle_theme;
///
/// toggle_theme(); // Dark -> Light
/// toggle_theme(); // Light -> Dark
/// ```
pub fn toggle_theme() {
    let signal = get_theme_signal();
    let current = signal.get();

    let new_theme = if current.is_dark() {
        Theme::light()
    } else {
        Theme::dark()
    };

    signal.set(new_theme);
}

/// Cycle through available themes
///
/// Cycles to the next registered theme. Note that the order is not guaranteed
/// as themes are stored in a HashMap.
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::cycle_theme;
///
/// cycle_theme(); // Switches to next registered theme
/// ```
pub fn cycle_theme() {
    let manager = get_theme_manager();
    let mut guard = lock_or_recover(&manager);

    guard.cycle();
    let theme = guard.current().clone();
    drop(guard);

    get_theme_signal().set(theme);
}

/// Get list of available theme IDs
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::theme_ids;
///
/// for id in theme_ids() {
///     println!("Available: {}", id);
/// }
/// ```
pub fn theme_ids() -> Vec<String> {
    let manager = get_theme_manager();
    let guard = lock_or_recover(&manager);
    guard
        .theme_ids()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Register a custom theme
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::{register_theme, Theme};
///
/// let custom = Theme::custom("my-theme")
///     .primary(Color::rgb(255, 100, 100))
///     .build();
///
/// register_theme("my-theme", custom);
/// set_theme_by_id("my-theme");
/// ```
pub fn register_theme(id: impl Into<String>, theme: Theme) {
    let manager = get_theme_manager();
    let mut guard = lock_or_recover(&manager);
    guard.register(id, theme);
}

/// Get a theme by ID without setting it
pub fn get_theme(id: &str) -> Option<Theme> {
    let manager = get_theme_manager();
    let guard = lock_or_recover(&manager);
    guard.get(id).cloned()
}

// ─────────────────────────────────────────────────────────────────────────────
// Theme CSS Variables
// ─────────────────────────────────────────────────────────────────────────────

/// Generate CSS variables for a theme
///
/// Returns a CSS string with `:root` variables that can be merged
/// into a stylesheet for theming support.
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::{theme_to_css_variables, Themes};
///
/// let css = theme_to_css_variables(&Themes::dracula());
/// // Returns:
/// // :root {
/// //   --theme-bg: #282a36;
/// //   --theme-surface: #44475a;
/// //   ...
/// // }
/// ```
pub fn theme_to_css_variables(theme: &Theme) -> String {
    use crate::style::Color;

    fn color_to_css(color: &Color) -> String {
        format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
    }

    format!(
        r#":root {{
  --theme-name: "{}";
  --theme-bg: {};
  --theme-surface: {};
  --theme-text: {};
  --theme-text-muted: {};
  --theme-border: {};
  --theme-divider: {};
  --theme-selection: {};
  --theme-selection-text: {};
  --theme-focus: {};
  --theme-primary: {};
  --theme-secondary: {};
  --theme-success: {};
  --theme-warning: {};
  --theme-error: {};
  --theme-info: {};
}}"#,
        theme.name,
        color_to_css(&theme.colors.background),
        color_to_css(&theme.colors.surface),
        color_to_css(&theme.colors.text),
        color_to_css(&theme.colors.text_muted),
        color_to_css(&theme.colors.border),
        color_to_css(&theme.colors.divider),
        color_to_css(&theme.colors.selection),
        color_to_css(&theme.colors.selection_text),
        color_to_css(&theme.colors.focus),
        color_to_css(&theme.palette.primary),
        color_to_css(&theme.palette.secondary),
        color_to_css(&theme.palette.success),
        color_to_css(&theme.palette.warning),
        color_to_css(&theme.palette.error),
        color_to_css(&theme.palette.info),
    )
}

// Tests moved to tests/style_tests.rs
