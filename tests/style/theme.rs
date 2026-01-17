//! Theme tests

#![allow(unused_imports)]

use revue::style::{
    easing, lerp_f32, lerp_u8, parse_css, shared_theme, theme_manager, ActiveTransition,
    AnimationDirection, AnimationFillMode, AnimationGroup, AnimationState, Color, ComputedStyle,
    CssKeyframe, Display, Easing, ErrorCode, FlexDirection, KeyframeAnimation, Palette,
    ParseErrors, Position, RichParseError, Severity, SharedTheme, Size, SourceLocation, Spacing,
    Stagger, Style, Suggestion, Theme, ThemeColors, ThemeManager, ThemeVariant, Themes, Transition,
    TransitionManager, Transitions, Tween, KNOWN_PROPERTIES,
};
use std::time::Duration;

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
    let custom = Theme::custom("Custom").primary(Color::MAGENTA).build();

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
    assert!(manager.unregister("dark").is_none());
    assert!(manager.has_theme("dark"));
}

#[test]
fn test_theme_manager_cycle() {
    let mut manager = ThemeManager::new();
    let initial = manager.current_id().to_string();

    manager.cycle();
    let after_cycle = manager.current_id().to_string();

    assert_ne!(initial, after_cycle);
}

#[test]
fn test_theme_manager_len() {
    let manager = ThemeManager::new();
    assert_eq!(manager.len(), 8);
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
