//! Theme system tests

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::style::Color;

    // ThemeVariant tests
    #[test]
    fn test_theme_variant_default() {
        assert_eq!(ThemeVariant::default(), ThemeVariant::Dark);
    }

    #[test]
    fn test_theme_variant_variants() {
        assert_eq!(ThemeVariant::Dark, ThemeVariant::Dark);
        assert_eq!(ThemeVariant::Light, ThemeVariant::Light);
        assert_eq!(ThemeVariant::HighContrast, ThemeVariant::HighContrast);
    }

    // Palette tests
    #[test]
    fn test_palette_default() {
        let palette = Palette::default();
        assert_eq!(palette, Palette::dark());
    }

    #[test]
    fn test_palette_dark() {
        let palette = Palette::dark();
        assert_eq!(palette.primary, Color::rgb(66, 133, 244));
        assert_eq!(palette.secondary, Color::rgb(156, 39, 176));
        assert_eq!(palette.success, Color::rgb(76, 175, 80));
        assert_eq!(palette.warning, Color::rgb(255, 193, 7));
        assert_eq!(palette.error, Color::rgb(244, 67, 54));
        assert_eq!(palette.info, Color::rgb(33, 150, 243));
    }

    #[test]
    fn test_palette_light() {
        let palette = Palette::light();
        assert_eq!(palette.primary, Color::rgb(25, 118, 210));
        assert_eq!(palette.secondary, Color::rgb(123, 31, 162));
        assert_eq!(palette.success, Color::rgb(56, 142, 60));
        assert_eq!(palette.warning, Color::rgb(255, 160, 0));
        assert_eq!(palette.error, Color::rgb(211, 47, 47));
        assert_eq!(palette.info, Color::rgb(2, 136, 209));
    }

    #[test]
    fn test_palette_high_contrast() {
        let palette = Palette::high_contrast();
        assert_eq!(palette.primary, Color::CYAN);
        assert_eq!(palette.secondary, Color::MAGENTA);
        assert_eq!(palette.success, Color::GREEN);
        assert_eq!(palette.warning, Color::YELLOW);
        assert_eq!(palette.error, Color::RED);
        assert_eq!(palette.info, Color::BLUE);
    }

    // ThemeColors tests
    #[test]
    fn test_theme_colors_default() {
        let colors = ThemeColors::default();
        assert_eq!(colors, ThemeColors::dark());
    }

    #[test]
    fn test_theme_colors_dark() {
        let colors = ThemeColors::dark();
        assert_eq!(colors.background, Color::rgb(18, 18, 18));
        assert_eq!(colors.surface, Color::rgb(30, 30, 30));
        assert_eq!(colors.text, Color::rgb(255, 255, 255));
        assert_eq!(colors.text_muted, Color::rgb(158, 158, 158));
        assert_eq!(colors.border, Color::rgb(66, 66, 66));
        assert_eq!(colors.divider, Color::rgb(48, 48, 48));
        assert_eq!(colors.selection, Color::rgb(66, 133, 244));
        assert_eq!(colors.selection_text, Color::WHITE);
        assert_eq!(colors.focus, Color::rgb(66, 133, 244));
    }

    #[test]
    fn test_theme_colors_light() {
        let colors = ThemeColors::light();
        assert_eq!(colors.background, Color::rgb(255, 255, 255));
        assert_eq!(colors.surface, Color::rgb(250, 250, 250));
        assert_eq!(colors.text, Color::rgb(33, 33, 33));
        assert_eq!(colors.text_muted, Color::rgb(117, 117, 117));
        assert_eq!(colors.border, Color::rgb(224, 224, 224));
        assert_eq!(colors.divider, Color::rgb(238, 238, 238));
        assert_eq!(colors.selection, Color::rgb(25, 118, 210));
        assert_eq!(colors.selection_text, Color::WHITE);
        assert_eq!(colors.focus, Color::rgb(25, 118, 210));
    }

    #[test]
    fn test_theme_colors_high_contrast() {
        let colors = ThemeColors::high_contrast();
        assert_eq!(colors.background, Color::BLACK);
        assert_eq!(colors.surface, Color::BLACK);
        assert_eq!(colors.text, Color::WHITE);
        assert_eq!(colors.border, Color::WHITE);
        assert_eq!(colors.divider, Color::WHITE);
        assert_eq!(colors.selection, Color::YELLOW);
        assert_eq!(colors.selection_text, Color::BLACK);
        assert_eq!(colors.focus, Color::CYAN);
    }

    // Theme tests
    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme, Theme::dark());
    }

    #[test]
    fn test_theme_dark() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "Dark");
        assert_eq!(theme.variant, ThemeVariant::Dark);
        assert_eq!(theme.palette, Palette::dark());
        assert_eq!(theme.colors, ThemeColors::dark());
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.name, "Light");
        assert_eq!(theme.variant, ThemeVariant::Light);
        assert_eq!(theme.palette, Palette::light());
        assert_eq!(theme.colors, ThemeColors::light());
    }

    #[test]
    fn test_theme_high_contrast() {
        let theme = Theme::high_contrast();
        assert_eq!(theme.name, "High Contrast");
        assert_eq!(theme.variant, ThemeVariant::HighContrast);
        assert_eq!(theme.palette, Palette::high_contrast());
        assert_eq!(theme.colors, ThemeColors::high_contrast());
    }

    #[test]
    fn test_theme_is_dark() {
        let dark = Theme::dark();
        assert!(dark.is_dark());
        assert!(!dark.is_light());
    }

    #[test]
    fn test_theme_is_light() {
        let light = Theme::light();
        assert!(light.is_light());
        assert!(!light.is_dark());
    }

    #[test]
    fn test_theme_clone() {
        let theme = Theme::dark();
        let cloned = theme.clone();
        assert_eq!(theme.name, cloned.name);
        assert_eq!(theme.variant, cloned.variant);
    }

    // ThemeBuilder tests
    #[test]
    fn test_theme_builder_new() {
        let builder = ThemeBuilder::new("Custom");
        assert_eq!(builder.theme.name, "Custom");
        // Should start with dark theme as base
        assert_eq!(builder.theme.variant, ThemeVariant::Dark);
    }

    #[test]
    fn test_theme_builder_variant() {
        let builder = ThemeBuilder::new("Custom").variant(ThemeVariant::Light);
        assert_eq!(builder.theme.variant, ThemeVariant::Light);
    }

    #[test]
    fn test_theme_builder_palette() {
        let palette = Palette::light();
        let builder = ThemeBuilder::new("Custom").palette(palette.clone());
        assert_eq!(builder.theme.palette, palette);
    }

    #[test]
    fn test_theme_builder_colors() {
        let colors = ThemeColors::light();
        let builder = ThemeBuilder::new("Custom").colors(colors.clone());
        assert_eq!(builder.theme.colors, colors);
    }

    #[test]
    fn test_theme_builder_primary() {
        let builder = ThemeBuilder::new("Custom").primary(Color::RED);
        assert_eq!(builder.theme.palette.primary, Color::RED);
    }

    #[test]
    fn test_theme_builder_background() {
        let builder = ThemeBuilder::new("Custom").background(Color::BLACK);
        assert_eq!(builder.theme.colors.background, Color::BLACK);
    }

    #[test]
    fn test_theme_builder_text() {
        let builder = ThemeBuilder::new("Custom").text(Color::WHITE);
        assert_eq!(builder.theme.colors.text, Color::WHITE);
    }

    #[test]
    fn test_theme_builder_build() {
        let theme = ThemeBuilder::new("MyTheme")
            .variant(ThemeVariant::Light)
            .primary(Color::rgb(100, 150, 200))
            .build();

        assert_eq!(theme.name, "MyTheme");
        assert_eq!(theme.variant, ThemeVariant::Light);
        assert_eq!(theme.palette.primary, Color::rgb(100, 150, 200));
    }

    #[test]
    fn test_theme_builder_chain() {
        let theme = ThemeBuilder::new("Chained")
            .variant(ThemeVariant::HighContrast)
            .primary(Color::CYAN)
            .background(Color::BLACK)
            .text(Color::WHITE)
            .build();

        assert_eq!(theme.name, "Chained");
        assert_eq!(theme.variant, ThemeVariant::HighContrast);
        assert_eq!(theme.palette.primary, Color::CYAN);
        assert_eq!(theme.colors.background, Color::BLACK);
        assert_eq!(theme.colors.text, Color::WHITE);
    }

    // Themes (predefined) tests
    #[test]
    fn test_themes_dracula() {
        let theme = Themes::dracula();
        assert_eq!(theme.name, "Dracula");
        assert_eq!(theme.variant, ThemeVariant::Dark);
        assert_eq!(theme.palette.primary, Color::rgb(139, 233, 253));
    }

    #[test]
    fn test_themes_nord() {
        let theme = Themes::nord();
        assert_eq!(theme.name, "Nord");
        assert_eq!(theme.variant, ThemeVariant::Dark);
        assert_eq!(theme.palette.primary, Color::rgb(136, 192, 208));
    }

    #[test]
    fn test_themes_monokai() {
        let theme = Themes::monokai();
        assert_eq!(theme.name, "Monokai");
        assert_eq!(theme.variant, ThemeVariant::Dark);
        assert_eq!(theme.palette.primary, Color::rgb(102, 217, 239));
    }

    #[test]
    fn test_themes_solarized_dark() {
        let theme = Themes::solarized_dark();
        assert_eq!(theme.name, "Solarized Dark");
        assert_eq!(theme.variant, ThemeVariant::Dark);
        assert_eq!(theme.palette.primary, Color::rgb(38, 139, 210));
    }

    #[test]
    fn test_themes_solarized_light() {
        let theme = Themes::solarized_light();
        assert_eq!(theme.name, "Solarized Light");
        assert_eq!(theme.variant, ThemeVariant::Light);
        assert_eq!(theme.palette.primary, Color::rgb(38, 139, 210));
    }

    // ThemeManager tests
    #[test]
    fn test_theme_manager_new() {
        let manager = ThemeManager::new();
        assert_eq!(manager.current_id(), "dark");
        assert!(manager.current().is_dark());
    }

    #[test]
    fn test_theme_manager_default() {
        let manager = ThemeManager::default();
        assert_eq!(manager.current_id(), "dark");
    }

    #[test]
    fn test_theme_manager_with_theme() {
        let manager = ThemeManager::with_theme("light");
        assert_eq!(manager.current_id(), "light");
        assert!(manager.current().is_light());
    }

    #[test]
    fn test_theme_manager_with_theme_invalid() {
        let manager = ThemeManager::with_theme("nonexistent");
        // Should fall back to default
        assert_eq!(manager.current_id(), "dark");
    }

    #[test]
    fn test_theme_manager_register() {
        let mut manager = ThemeManager::new();
        manager.register("custom", Theme::dark());
        assert!(manager.has_theme("custom"));
    }

    #[test]
    fn test_theme_manager_unregister() {
        let mut manager = ThemeManager::new();
        manager.register("custom", Theme::dark());
        let removed = manager.unregister("custom");
        assert!(removed.is_some());
        assert!(!manager.has_theme("custom"));
    }

    #[test]
    fn test_theme_manager_unregister_current_returns_none() {
        let mut manager = ThemeManager::new();
        let result = manager.unregister("dark");
        assert!(result.is_none());
        assert!(manager.has_theme("dark"));
    }

    #[test]
    fn test_theme_manager_set_theme() {
        let mut manager = ThemeManager::new();
        assert!(manager.set_theme("light"));
        assert_eq!(manager.current_id(), "light");
    }

    #[test]
    fn test_theme_manager_set_theme_invalid() {
        let mut manager = ThemeManager::new();
        assert!(!manager.set_theme("nonexistent"));
        assert_eq!(manager.current_id(), "dark");
    }

    #[test]
    fn test_theme_manager_current() {
        let manager = ThemeManager::new();
        let current = manager.current();
        assert_eq!(current.name, "Dark");
    }

    #[test]
    fn test_theme_manager_get() {
        let manager = ThemeManager::new();
        assert!(manager.get("dark").is_some());
        assert!(manager.get("light").is_some());
        assert!(manager.get("nonexistent").is_none());
    }

    #[test]
    fn test_theme_manager_theme_ids() {
        let manager = ThemeManager::new();
        let ids = manager.theme_ids();
        assert!(ids.contains(&"dark"));
        assert!(ids.contains(&"light"));
        assert!(ids.contains(&"high_contrast"));
    }

    #[test]
    fn test_theme_manager_themes_iterator() {
        let manager = ThemeManager::new();
        let count = manager.themes().count();
        assert!(count >= 3); // At least dark, light, high_contrast
    }

    #[test]
    fn test_theme_manager_len() {
        let manager = ThemeManager::new();
        assert!(manager.len() >= 3);
    }

    #[test]
    fn test_theme_manager_is_empty() {
        let manager = ThemeManager::new();
        assert!(!manager.is_empty());
    }

    #[test]
    fn test_theme_manager_has_theme() {
        let manager = ThemeManager::new();
        assert!(manager.has_theme("dark"));
        assert!(!manager.has_theme("nonexistent"));
    }

    #[test]
    fn test_theme_manager_toggle_dark_light() {
        let mut manager = ThemeManager::new();
        assert_eq!(manager.current_id(), "dark");
        manager.toggle_dark_light();
        assert_eq!(manager.current_id(), "light");
        manager.toggle_dark_light();
        assert_eq!(manager.current_id(), "dark");
    }

    #[test]
    fn test_theme_manager_set_dark_light_theme() {
        let mut manager = ThemeManager::new();
        manager.register("custom_dark", Theme::dark());
        manager.register("custom_light", Theme::light());
        manager.set_dark_theme("custom_dark");
        manager.set_light_theme("custom_light");

        manager.toggle_dark_light();
        assert_eq!(manager.current_id(), "custom_light");
        manager.toggle_dark_light();
        assert_eq!(manager.current_id(), "custom_dark");
    }

    #[test]
    fn test_theme_manager_cycle() {
        let mut manager = ThemeManager::new();
        let first_id = manager.current_id().to_string();
        manager.cycle();
        assert_ne!(manager.current_id(), first_id);
    }

    #[test]
    fn test_theme_manager_cycle_wraps() {
        let mut manager = ThemeManager::new();
        let first = manager.current_id().to_string();
        // Cycle through all themes
        for _ in 0..manager.len() {
            manager.cycle();
        }
        // Should be back to the first theme
        assert_eq!(manager.current_id(), first);
    }

    #[test]
    fn test_theme_manager_cycle_dark() {
        let mut manager = ThemeManager::new();
        manager.set_theme("dracula");
        let first = manager.current_id().to_string();
        manager.cycle_dark();
        assert_ne!(manager.current_id(), first);
        // Should still be a dark theme
        assert!(manager.current().is_dark());
    }

    #[test]
    fn test_theme_manager_cycle_light() {
        let mut manager = ThemeManager::new();
        manager.set_theme("light");
        let first = manager.current_id().to_string();
        manager.cycle_light();
        assert_ne!(manager.current_id(), first);
        // Should still be a light theme
        assert!(manager.current().is_light());
    }

    // SharedTheme tests
    #[test]
    fn test_shared_theme_new() {
        let shared = SharedTheme::new();
        assert_eq!(shared.current_id(), "dark");
    }

    #[test]
    fn test_shared_theme_default() {
        let shared = SharedTheme::default();
        assert_eq!(shared.current_id(), "dark");
    }

    #[test]
    fn test_shared_theme_with_theme() {
        let shared = SharedTheme::with_theme("light");
        assert_eq!(shared.current_id(), "light");
    }

    #[test]
    fn test_shared_theme_current() {
        let shared = SharedTheme::new();
        let current = shared.current();
        assert_eq!(current.name, "Dark");
    }

    #[test]
    fn test_shared_theme_set_theme() {
        let shared = SharedTheme::new();
        assert!(shared.set_theme("light"));
        assert_eq!(shared.current_id(), "light");
    }

    #[test]
    fn test_shared_theme_set_theme_invalid() {
        let shared = SharedTheme::new();
        assert!(!shared.set_theme("nonexistent"));
    }

    #[test]
    fn test_shared_theme_toggle_dark_light() {
        let shared = SharedTheme::new();
        assert_eq!(shared.current_id(), "dark");
        shared.toggle_dark_light();
        assert_eq!(shared.current_id(), "light");
    }

    #[test]
    fn test_shared_theme_cycle() {
        let shared = SharedTheme::new();
        let first = shared.current_id().to_string();
        shared.cycle();
        assert_ne!(shared.current_id(), first);
    }

    #[test]
    fn test_shared_theme_register() {
        let shared = SharedTheme::new();
        shared.register("custom", Theme::dark());
        assert!(shared.theme_ids().contains(&"custom".to_string()));
    }

    #[test]
    fn test_shared_theme_ids() {
        let shared = SharedTheme::new();
        let ids = shared.theme_ids();
        assert!(ids.contains(&"dark".to_string()));
        assert!(ids.contains(&"light".to_string()));
    }

    #[test]
    fn test_shared_theme_clone() {
        let shared = SharedTheme::new();
        let cloned = shared.clone();
        assert_eq!(shared.current_id(), cloned.current_id());
    }

    // Helper function tests
    #[test]
    fn test_theme_manager_helper() {
        let manager = theme_manager();
        assert_eq!(manager.current_id(), "dark");
    }

    #[test]
    fn test_shared_theme_helper() {
        let shared = shared_theme();
        assert_eq!(shared.current_id(), "dark");
    }
}
