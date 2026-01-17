//! Integration tests for style module

use revue::style::{
    // Animation
    easing,
    // Transition
    lerp_f32,
    lerp_u8,
    // Parser
    parse_css,
    // Theme
    shared_theme,
    theme_manager,
    ActiveTransition,
    AnimationDirection,
    AnimationFillMode,
    AnimationGroup,
    AnimationState,
    // Properties
    Color,
    ComputedStyle,
    CssKeyframe,
    Display,
    Easing,
    // Error
    ErrorCode,
    FlexDirection,
    KeyframeAnimation,
    Palette,
    ParseErrors,
    Position,
    RichParseError,
    Severity,
    SharedTheme,
    Size,
    SourceLocation,
    Spacing,
    Stagger,
    Style,
    Suggestion,
    Theme,
    ThemeColors,
    ThemeManager,
    ThemeVariant,
    Themes,
    Transition,
    TransitionManager,
    Transitions,
    Tween,
    KNOWN_PROPERTIES,
};
use std::time::Duration;

// =============================================================================
// Properties tests (from properties.rs)
// =============================================================================

#[test]
fn test_color_rgb() {
    let c = Color::rgb(255, 128, 64);
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 128);
    assert_eq!(c.b, 64);
}

#[test]
fn test_color_hex() {
    let c = Color::hex(0xFF8040);
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 128);
    assert_eq!(c.b, 64);
}

#[test]
fn test_color_constants() {
    assert_eq!(Color::WHITE, Color::rgb(255, 255, 255));
    assert_eq!(Color::BLACK, Color::rgb(0, 0, 0));
    assert_eq!(Color::RED, Color::rgb(255, 0, 0));
}

#[test]
fn test_spacing_all() {
    let s = Spacing::all(5);
    assert_eq!(s.top, 5);
    assert_eq!(s.right, 5);
    assert_eq!(s.bottom, 5);
    assert_eq!(s.left, 5);
}

#[test]
fn test_spacing_vertical() {
    let s = Spacing::vertical(3);
    assert_eq!(s.top, 3);
    assert_eq!(s.bottom, 3);
    assert_eq!(s.left, 0);
    assert_eq!(s.right, 0);
}

#[test]
fn test_spacing_horizontal() {
    let s = Spacing::horizontal(2);
    assert_eq!(s.left, 2);
    assert_eq!(s.right, 2);
    assert_eq!(s.top, 0);
    assert_eq!(s.bottom, 0);
}

#[test]
fn test_style_default() {
    let style = Style::default();
    assert_eq!(style.layout.display, Display::Flex);
    assert_eq!(style.layout.flex_direction, FlexDirection::Row);
    assert_eq!(style.visual.opacity, 1.0);
    assert!(style.visual.visible);
}

#[test]
fn test_size_variants() {
    assert_eq!(Size::Auto, Size::default());

    let fixed = Size::Fixed(100);
    match fixed {
        Size::Fixed(v) => assert_eq!(v, 100),
        _ => panic!("Expected Fixed"),
    }

    let percent = Size::Percent(50.0);
    match percent {
        Size::Percent(v) => assert!((v - 50.0).abs() < f32::EPSILON),
        _ => panic!("Expected Percent"),
    }
}

#[test]
fn test_style_inherit() {
    let mut parent = Style::default();
    parent.visual.color = Color::RED;
    parent.visual.opacity = 0.8;
    parent.visual.visible = false;
    parent.visual.background = Color::BLUE;
    parent.spacing.padding = Spacing::all(10);

    let inherited = Style::inherit(&parent);

    assert_eq!(inherited.visual.color, Color::RED);
    assert_eq!(inherited.visual.opacity, 0.8);
    assert!(!inherited.visual.visible);

    assert_eq!(inherited.visual.background, Color::default());
    assert_eq!(inherited.spacing.padding, Spacing::default());
}

#[test]
fn test_style_with_inheritance() {
    let mut parent = Style::default();
    parent.visual.color = Color::RED;
    parent.visual.opacity = 0.8;

    let mut child = Style::default();
    child.visual.color = Color::GREEN;
    child.spacing.padding = Spacing::all(5);

    let result = child.with_inheritance(&parent);

    assert_eq!(result.visual.color, Color::GREEN);
    assert_eq!(result.visual.opacity, 0.8);
    assert_eq!(result.spacing.padding, Spacing::all(5));
}

#[test]
fn test_style_inherit_chain() {
    let mut grandparent = Style::default();
    grandparent.visual.color = Color::RED;

    let parent = Style::inherit(&grandparent);
    assert_eq!(parent.visual.color, Color::RED);

    let child = Style::inherit(&parent);
    assert_eq!(child.visual.color, Color::RED);
}

#[test]
fn test_color_darken() {
    let base = Color::rgb(100, 150, 200);
    let darker = base.darken(30);
    assert_eq!(darker.r, 70);
    assert_eq!(darker.g, 120);
    assert_eq!(darker.b, 170);
    assert_eq!(darker.a, 255);
}

#[test]
fn test_color_darken_saturates() {
    let base = Color::rgb(10, 20, 30);
    let darker = base.darken(50);
    assert_eq!(darker.r, 0);
    assert_eq!(darker.g, 0);
    assert_eq!(darker.b, 0);
}

#[test]
fn test_color_lighten() {
    let base = Color::rgb(100, 150, 200);
    let lighter = base.lighten(30);
    assert_eq!(lighter.r, 130);
    assert_eq!(lighter.g, 180);
    assert_eq!(lighter.b, 230);
    assert_eq!(lighter.a, 255);
}

#[test]
fn test_color_lighten_saturates() {
    let base = Color::rgb(240, 250, 255);
    let lighter = base.lighten(50);
    assert_eq!(lighter.r, 255);
    assert_eq!(lighter.g, 255);
    assert_eq!(lighter.b, 255);
}

#[test]
fn test_color_darken_pct() {
    let base = Color::rgb(100, 100, 100);
    let darker = base.darken_pct(0.2);
    assert_eq!(darker.r, 80);
    assert_eq!(darker.g, 80);
    assert_eq!(darker.b, 80);
}

#[test]
fn test_color_lighten_pct() {
    let base = Color::rgb(100, 100, 100);
    let lighter = base.lighten_pct(0.2);
    assert_eq!(lighter.r, 131);
    assert_eq!(lighter.g, 131);
    assert_eq!(lighter.b, 131);
}

#[test]
fn test_color_pressed() {
    let base = Color::rgb(100, 100, 100);
    let pressed = base.pressed();
    assert_eq!(pressed, base.darken(30));
}

#[test]
fn test_color_hover() {
    let base = Color::rgb(100, 100, 100);
    let hover = base.hover();
    assert_eq!(hover, base.lighten(40));
}

#[test]
fn test_color_focus() {
    let base = Color::rgb(100, 100, 100);
    let focus = base.focus();
    assert_eq!(focus, base.lighten(40));
}

#[test]
fn test_color_blend() {
    let red = Color::RED;
    let blue = Color::BLUE;
    let purple = red.blend(blue, 0.5);
    assert_eq!(purple.r, 127);
    assert_eq!(purple.g, 0);
    assert_eq!(purple.b, 127);
}

#[test]
fn test_color_blend_edge_cases() {
    let red = Color::RED;
    let blue = Color::BLUE;

    let result = red.blend(blue, 0.0);
    assert_eq!(result, red);

    let result = red.blend(blue, 1.0);
    assert_eq!(result, blue);
}

#[test]
fn test_color_with_interaction() {
    let base = Color::rgb(100, 100, 100);

    let result = base.with_interaction(true, true, true);
    assert_eq!(result, base.pressed());

    let result = base.with_interaction(false, true, false);
    assert_eq!(result, base.hover());

    let result = base.with_interaction(false, false, true);
    assert_eq!(result, base.focus());

    let result = base.with_interaction(false, false, false);
    assert_eq!(result, base);
}

#[test]
fn test_color_preserves_alpha() {
    let base = Color::rgba(100, 100, 100, 128);

    assert_eq!(base.darken(30).a, 128);
    assert_eq!(base.lighten(30).a, 128);
    assert_eq!(base.pressed().a, 128);
    assert_eq!(base.hover().a, 128);
}

// =============================================================================
// Computed style tests (from computed.rs)
// =============================================================================

#[test]
fn test_computed_style_new() {
    let computed = ComputedStyle::new();
    let _ = computed.style.color();
}

#[test]
fn test_computed_style_default() {
    let computed = ComputedStyle::default();
    let _ = computed.style.color();
}

#[test]
fn test_computed_style_compute_no_parent() {
    let style = Style::default();
    let computed = ComputedStyle::compute(style, None);
    let _ = computed.style.color();
}

#[test]
fn test_computed_style_compute_with_parent() {
    let mut parent_style = Style::default();
    parent_style.visual.color = Color::RED;
    let parent = ComputedStyle {
        style: parent_style,
    };

    let child_style = Style::default();
    let computed = ComputedStyle::compute(child_style, Some(&parent));

    assert_eq!(computed.style.color(), Color::RED);
}

#[test]
fn test_computed_style_compute_child_overrides_parent() {
    let mut parent_style = Style::default();
    parent_style.visual.color = Color::RED;
    let parent = ComputedStyle {
        style: parent_style,
    };

    let mut child_style = Style::default();
    child_style.visual.color = Color::BLUE;
    let computed = ComputedStyle::compute(child_style, Some(&parent));

    assert_eq!(computed.style.color(), Color::BLUE);
}

#[test]
fn test_computed_style_clone() {
    let computed = ComputedStyle::new();
    let cloned = computed.clone();
    let _ = cloned.style.color();
}

#[test]
fn test_computed_style_debug() {
    let computed = ComputedStyle::new();
    let debug = format!("{:?}", computed);
    assert!(debug.contains("ComputedStyle"));
}

#[test]
fn test_computed_style_inheritance_chain() {
    let mut grandparent_style = Style::default();
    grandparent_style.visual.color = Color::RED;
    let grandparent = ComputedStyle {
        style: grandparent_style,
    };

    let parent_style = Style::default();
    let parent = ComputedStyle::compute(parent_style, Some(&grandparent));

    let child_style = Style::default();
    let child = ComputedStyle::compute(child_style, Some(&parent));

    assert_eq!(child.style.color(), Color::RED);
}

// =============================================================================
// Parser tests (from parser.rs)
// =============================================================================

#[test]
fn test_parse_empty() {
    let sheet = parse_css("").unwrap();
    assert!(sheet.rules.is_empty());
    assert!(sheet.variables.is_empty());
}

#[test]
fn test_parse_simple_rule() {
    let css = ".button { color: red; }";
    let sheet = parse_css(css).unwrap();

    assert_eq!(sheet.rules.len(), 1);
    assert_eq!(sheet.rules[0].selector, ".button");
    assert_eq!(sheet.rules[0].declarations.len(), 1);
    assert_eq!(sheet.rules[0].declarations[0].property, "color");
    assert_eq!(sheet.rules[0].declarations[0].value, "red");
}

#[test]
fn test_parse_multiple_declarations() {
    let css = ".box { width: 100; height: 50; padding: 4; }";
    let sheet = parse_css(css).unwrap();

    assert_eq!(sheet.rules[0].declarations.len(), 3);
}

#[test]
fn test_parse_css_variables() {
    let css = r#"
        :root {
            --primary: #ff0000;
            --spacing: 8;
        }
        .button { color: var(--primary); }
    "#;
    let sheet = parse_css(css).unwrap();

    assert_eq!(
        sheet.variables.get("--primary"),
        Some(&"#ff0000".to_string())
    );
    assert_eq!(sheet.variables.get("--spacing"), Some(&"8".to_string()));
    assert_eq!(sheet.rules.len(), 1);
}

#[test]
fn test_parse_comments() {
    let css = r#"
        /* This is a comment */
        .box {
            /* Another comment */
            width: 100;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    assert_eq!(sheet.rules.len(), 1);
    assert_eq!(sheet.rules[0].declarations.len(), 1);
}

#[test]
fn test_apply_stylesheet() {
    let css = r#"
        .container {
            display: flex;
            flex-direction: column;
            width: 200;
            padding: 10;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    let style = sheet.apply(".container", &Style::default());

    assert_eq!(style.layout.display, Display::Flex);
    assert_eq!(style.layout.flex_direction, FlexDirection::Column);
    assert_eq!(style.sizing.width, Size::Fixed(200));
    assert_eq!(style.spacing.padding, Spacing::all(10));
}

#[test]
fn test_apply_with_variables() {
    let css = r#"
        :root {
            --primary: #ff0000;
        }
        .text { color: var(--primary); }
    "#;
    let sheet = parse_css(css).unwrap();
    let style = sheet.apply(".text", &Style::default());

    assert_eq!(style.visual.color, Color::RED);
}

#[test]
fn test_apply_grid_properties() {
    let css = r#"
        .grid {
            display: grid;
            grid-template-columns: 1fr 2fr;
            grid-template-rows: auto 100px;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    let style = sheet.apply(".grid", &Style::default());

    assert_eq!(style.layout.display, Display::Grid);
    assert_eq!(style.layout.grid_template_columns.tracks.len(), 2);
    assert_eq!(style.layout.grid_template_rows.tracks.len(), 2);
}

#[test]
fn test_apply_position_properties() {
    let css = r#"
        .modal {
            position: absolute;
            top: 10;
            left: 20;
            z-index: 100;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    let style = sheet.apply(".modal", &Style::default());

    assert_eq!(style.layout.position, Position::Absolute);
    assert_eq!(style.spacing.top, Some(10));
    assert_eq!(style.spacing.left, Some(20));
    assert_eq!(style.visual.z_index, 100);
}

// =============================================================================
// Theme tests (from theme.rs)
// =============================================================================

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

// =============================================================================
// Easing tests (from animation/easing.rs)
// =============================================================================

#[test]
fn test_easing_linear() {
    assert_eq!(easing::linear(0.0), 0.0);
    assert_eq!(easing::linear(0.5), 0.5);
    assert_eq!(easing::linear(1.0), 1.0);
}

#[test]
fn test_easing_ease_in() {
    assert_eq!(easing::ease_in(0.0), 0.0);
    assert_eq!(easing::ease_in(1.0), 1.0);
    assert!(easing::ease_in(0.5) < 0.5);
}

#[test]
fn test_easing_ease_out() {
    assert_eq!(easing::ease_out(0.0), 0.0);
    assert_eq!(easing::ease_out(1.0), 1.0);
    assert!(easing::ease_out(0.5) > 0.5);
}

#[test]
fn test_easing_ease_in_out() {
    assert_eq!(easing::ease_in_out(0.0), 0.0);
    assert_eq!(easing::ease_in_out(1.0), 1.0);
    assert_eq!(easing::ease_in_out(0.5), 0.5);
}

#[test]
fn test_easing_ease_in_cubic() {
    assert_eq!(easing::ease_in_cubic(0.0), 0.0);
    assert_eq!(easing::ease_in_cubic(1.0), 1.0);
    assert!(easing::ease_in_cubic(0.5) < 0.5);
}

#[test]
fn test_easing_ease_out_cubic() {
    assert_eq!(easing::ease_out_cubic(0.0), 0.0);
    assert_eq!(easing::ease_out_cubic(1.0), 1.0);
    assert!(easing::ease_out_cubic(0.5) > 0.5);
}

#[test]
fn test_easing_ease_in_out_cubic() {
    assert_eq!(easing::ease_in_out_cubic(0.0), 0.0);
    assert_eq!(easing::ease_in_out_cubic(1.0), 1.0);
    assert_eq!(easing::ease_in_out_cubic(0.5), 0.5);
}

#[test]
fn test_easing_bounce_out() {
    assert_eq!(easing::bounce_out(0.0), 0.0);
    assert!((easing::bounce_out(1.0) - 1.0).abs() < 0.001);
    assert!(easing::bounce_out(0.5) > 0.0);
}

#[test]
fn test_easing_elastic_out() {
    assert_eq!(easing::elastic_out(0.0), 0.0);
    assert_eq!(easing::elastic_out(1.0), 1.0);
    assert!(easing::elastic_out(0.5) > 0.0);
}

#[test]
fn test_easing_back_out() {
    assert_eq!(easing::back_out(0.0), 0.0);
    assert!((easing::back_out(1.0) - 1.0).abs() < 0.001);
    assert!(easing::back_out(0.5) > 0.0);
}

#[test]
fn test_all_easing_bounds() {
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        assert!(easing::linear(t) >= 0.0 && easing::linear(t) <= 1.0);
        assert!(easing::ease_in(t) >= 0.0 && easing::ease_in(t) <= 1.0);
        assert!(easing::ease_out(t) >= 0.0 && easing::ease_out(t) <= 1.0);
        assert!(easing::ease_in_out(t) >= 0.0 && easing::ease_in_out(t) <= 1.0);
        assert!(easing::ease_in_cubic(t) >= 0.0 && easing::ease_in_cubic(t) <= 1.0);
        assert!(easing::ease_out_cubic(t) >= 0.0 && easing::ease_out_cubic(t) <= 1.0);
        assert!(easing::ease_in_out_cubic(t) >= 0.0 && easing::ease_in_out_cubic(t) <= 1.0);
        assert!(easing::bounce_out(t) >= 0.0 && easing::bounce_out(t) <= 1.1);
        assert!(easing::elastic_out(t) >= -0.5 && easing::elastic_out(t) <= 1.5);
        assert!(easing::back_out(t) >= -0.5 && easing::back_out(t) <= 1.5);
    }
}

// =============================================================================
// Animation tests (from animation/mod.rs) - public API only
// =============================================================================

#[test]
fn test_animation_state_eq() {
    assert_eq!(AnimationState::Pending, AnimationState::Pending);
    assert_eq!(AnimationState::Running, AnimationState::Running);
    assert_eq!(AnimationState::Paused, AnimationState::Paused);
    assert_eq!(AnimationState::Completed, AnimationState::Completed);
}

#[test]
fn test_animation_state_clone() {
    let state = AnimationState::Running;
    let cloned = state.clone();
    assert_eq!(state, cloned);
}

#[test]
fn test_tween_new() {
    let tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    assert_eq!(tween.state(), AnimationState::Pending);
}

#[test]
fn test_tween_start() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_millis(100));
    tween.start();
    assert_eq!(tween.state(), AnimationState::Running);
    assert!(tween.is_running());
}

#[test]
fn test_tween_pause() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    tween.start();
    tween.pause();
    assert_eq!(tween.state(), AnimationState::Paused);
    assert!(!tween.is_running());
}

#[test]
fn test_tween_pause_not_running() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    tween.pause();
    assert_eq!(tween.state(), AnimationState::Pending);
}

#[test]
fn test_tween_resume() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    tween.start();
    tween.pause();
    tween.resume();
    assert_eq!(tween.state(), AnimationState::Running);
}

#[test]
fn test_tween_resume_not_paused() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    tween.start();
    tween.resume();
    assert_eq!(tween.state(), AnimationState::Running);
}

#[test]
fn test_tween_reset() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_millis(100));
    tween.start();
    tween.reset();
    assert_eq!(tween.state(), AnimationState::Pending);
    assert!(!tween.is_running());
    assert!(!tween.is_completed());
}

#[test]
fn test_tween_value_pending() {
    let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    assert_eq!(tween.value(), 0.0);
}

#[test]
fn test_tween_progress_pending() {
    let tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
    assert_eq!(tween.progress(), 0.0);
}

#[test]
fn test_tween_is_completed() {
    let tween = Tween::new(0.0, 100.0, Duration::from_millis(1));
    assert!(!tween.is_completed());
}

#[test]
fn test_keyframe_animation_new() {
    let anim = KeyframeAnimation::new("test");
    assert_eq!(anim.name(), "test");
    assert_eq!(anim.state(), AnimationState::Pending);
}

#[test]
fn test_keyframe_animation_keyframe() {
    let anim = KeyframeAnimation::new("test")
        .keyframe(0, |kf| kf.set("opacity", 0.0))
        .keyframe(100, |kf| kf.set("opacity", 1.0));
    assert!(!anim.is_running());
}

#[test]
fn test_keyframe_animation_pause_resume() {
    let mut anim = KeyframeAnimation::new("test").duration(Duration::from_secs(1));
    anim.start();
    anim.pause();
    assert_eq!(anim.state(), AnimationState::Paused);
    anim.resume();
    assert!(anim.is_running());
}

#[test]
fn test_keyframe_animation_reset() {
    let mut anim = KeyframeAnimation::new("test");
    anim.start();
    anim.reset();
    assert_eq!(anim.state(), AnimationState::Pending);
}

#[test]
fn test_keyframe_animation_progress_pending() {
    let anim = KeyframeAnimation::new("test");
    assert_eq!(anim.progress(), 0.0);
}

#[test]
fn test_animation_direction_default() {
    let dir = AnimationDirection::default();
    assert_eq!(dir, AnimationDirection::Normal);
}

#[test]
fn test_animation_direction_variants() {
    assert_eq!(AnimationDirection::Normal, AnimationDirection::Normal);
    assert_eq!(AnimationDirection::Reverse, AnimationDirection::Reverse);
    assert_eq!(AnimationDirection::Alternate, AnimationDirection::Alternate);
    assert_eq!(
        AnimationDirection::AlternateReverse,
        AnimationDirection::AlternateReverse
    );
}

#[test]
fn test_animation_fill_mode_default() {
    let fill = AnimationFillMode::default();
    assert_eq!(fill, AnimationFillMode::None);
}

#[test]
fn test_animation_fill_mode_variants() {
    assert_eq!(AnimationFillMode::None, AnimationFillMode::None);
    assert_eq!(AnimationFillMode::Forwards, AnimationFillMode::Forwards);
    assert_eq!(AnimationFillMode::Backwards, AnimationFillMode::Backwards);
    assert_eq!(AnimationFillMode::Both, AnimationFillMode::Both);
}

#[test]
fn test_css_keyframe_new() {
    let kf = CssKeyframe::new(50);
    assert_eq!(kf.percent, 50);
    assert!(kf.properties.is_empty());
}

#[test]
fn test_css_keyframe_new_clamped() {
    let kf = CssKeyframe::new(150);
    assert_eq!(kf.percent, 100);
}

#[test]
fn test_css_keyframe_set() {
    let kf = CssKeyframe::new(0).set("opacity", 0.5);
    assert_eq!(kf.get("opacity"), Some(0.5));
}

#[test]
fn test_css_keyframe_get_missing() {
    let kf = CssKeyframe::new(0);
    assert_eq!(kf.get("opacity"), None);
}

#[test]
fn test_css_keyframe_multiple_properties() {
    let kf = CssKeyframe::new(50)
        .set("opacity", 1.0)
        .set("scale", 1.5)
        .set("x", 100.0);
    assert_eq!(kf.get("opacity"), Some(1.0));
    assert_eq!(kf.get("scale"), Some(1.5));
    assert_eq!(kf.get("x"), Some(100.0));
}

#[test]
fn test_stagger() {
    let stagger = Stagger::new(5, Duration::from_millis(50));
    assert_eq!(stagger.delay_for(0), Duration::ZERO);
    assert_eq!(stagger.delay_for(1), Duration::from_millis(50));
    assert_eq!(stagger.delay_for(4), Duration::from_millis(200));
}

#[test]
fn test_animation_group_parallel() {
    let group = AnimationGroup::parallel()
        .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(100)))
        .with_animation(KeyframeAnimation::new("b").duration(Duration::from_millis(200)));

    assert_eq!(group.total_duration(), Duration::from_millis(200));
}

#[test]
fn test_animation_group_sequential() {
    let group = AnimationGroup::sequential()
        .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(100)))
        .with_animation(KeyframeAnimation::new("b").duration(Duration::from_millis(200)));

    assert_eq!(group.total_duration(), Duration::from_millis(300));
}

// =============================================================================
// Transition tests (from transition.rs) - public API only
// =============================================================================

#[test]
fn test_transition_easing_linear() {
    assert_eq!(Easing::Linear.apply(0.0), 0.0);
    assert_eq!(Easing::Linear.apply(0.5), 0.5);
    assert_eq!(Easing::Linear.apply(1.0), 1.0);
}

#[test]
fn test_transition_easing_ease_in() {
    let e = Easing::EaseIn;
    assert_eq!(e.apply(0.0), 0.0);
    assert!(e.apply(0.5) < 0.5);
    assert_eq!(e.apply(1.0), 1.0);
}

#[test]
fn test_transition_easing_ease_out() {
    let e = Easing::EaseOut;
    assert_eq!(e.apply(0.0), 0.0);
    assert!(e.apply(0.5) > 0.5);
    assert_eq!(e.apply(1.0), 1.0);
}

#[test]
fn test_transition_easing_parse() {
    assert_eq!(Easing::parse("linear"), Some(Easing::Linear));
    assert_eq!(Easing::parse("ease-in"), Some(Easing::EaseIn));
    assert_eq!(Easing::parse("ease-out"), Some(Easing::EaseOut));
    assert_eq!(Easing::parse("ease-in-out"), Some(Easing::EaseInOut));
}

#[test]
fn test_transition_parse() {
    let t = Transition::parse("opacity 0.3s ease-in").unwrap();
    assert_eq!(t.property, "opacity");
    assert_eq!(t.duration, Duration::from_millis(300));
    assert_eq!(t.easing, Easing::EaseIn);
}

#[test]
fn test_transition_parse_with_delay() {
    let t = Transition::parse("transform 0.5s 0.1s ease-out").unwrap();
    assert_eq!(t.property, "transform");
    assert_eq!(t.duration, Duration::from_millis(500));
    assert_eq!(t.delay, Duration::from_millis(100));
    assert_eq!(t.easing, Easing::EaseOut);
}

#[test]
fn test_transitions_parse() {
    let ts = Transitions::parse("opacity 0.3s, background 0.5s ease-out");
    assert_eq!(ts.items.len(), 2);
    assert!(ts.has("opacity"));
    assert!(ts.has("background"));
}

#[test]
fn test_active_transition() {
    let t = Transition::new("opacity", Duration::from_secs(1));
    let mut active = ActiveTransition::new("opacity", 0.0, 1.0, &t);

    assert_eq!(active.current(), 0.0);

    active.update(Duration::from_millis(500));
    let mid = active.current();
    assert!(mid > 0.0 && mid < 1.0);

    active.update(Duration::from_millis(500));
    assert_eq!(active.current(), 1.0);
    assert!(active.is_complete());
}

#[test]
fn test_transition_manager() {
    let mut manager = TransitionManager::new();
    let t = Transition::new("opacity", Duration::from_secs(1));

    manager.start("opacity", 0.0, 1.0, &t);
    assert!(manager.has_active());

    manager.update(Duration::from_secs(2));
    assert!(!manager.has_active());
}

#[test]
fn test_lerp() {
    assert_eq!(lerp_u8(0, 100, 0.5), 50);
    assert_eq!(lerp_f32(0.0, 1.0, 0.5), 0.5);
}

#[test]
fn test_node_aware_transition() {
    let mut manager = TransitionManager::new();
    let t = Transition::new("opacity", Duration::from_secs(1));

    manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);

    assert!(manager.has_active());
    assert!(manager.node_has_active("btn1"));
    assert!(!manager.node_has_active("btn2"));

    let active_ids: Vec<&str> = manager.active_node_ids().collect();
    assert_eq!(active_ids, vec!["btn1"]);
}

#[test]
fn test_node_transition_update() {
    let mut manager = TransitionManager::new();
    let t = Transition::new("opacity", Duration::from_secs(1));

    manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);

    manager.update_nodes(Duration::from_millis(500));

    let value = manager.get_for_node("btn1", "opacity").unwrap();
    assert!(value > 0.0 && value < 1.0);

    manager.update_nodes(Duration::from_millis(600));

    assert!(!manager.node_has_active("btn1"));
}

#[test]
fn test_multiple_node_transitions() {
    let mut manager = TransitionManager::new();
    let t1 = Transition::new("opacity", Duration::from_secs(1));
    let t2 = Transition::new("scale", Duration::from_millis(500));

    manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t1);
    manager.start_for_node("btn1", "scale", 1.0, 1.5, &t2);
    manager.start_for_node("btn2", "opacity", 1.0, 0.0, &t1);

    assert!(manager.node_has_active("btn1"));
    assert!(manager.node_has_active("btn2"));

    let values = manager.current_values_for_node("btn1");
    assert_eq!(values.len(), 2);
    assert!(values.contains_key("opacity"));
    assert!(values.contains_key("scale"));
}

#[test]
fn test_clear_clears_node_transitions() {
    let mut manager = TransitionManager::new();
    let t = Transition::new("opacity", Duration::from_secs(1));

    manager.start_for_node("btn1", "opacity", 0.0, 1.0, &t);
    assert!(manager.has_active());

    manager.clear();

    assert!(!manager.has_active());
    assert!(!manager.node_has_active("btn1"));
}

// =============================================================================
// Error tests (from error.rs)
// =============================================================================

#[test]
fn test_error_code_display() {
    assert_eq!(ErrorCode::InvalidSyntax.code(), "E001");
    assert_eq!(ErrorCode::UnknownProperty.code(), "E002");
}

#[test]
fn test_source_location_from_offset() {
    let source = "line1\nline2\nline3";
    let loc = SourceLocation::from_offset(source, 6);
    assert_eq!(loc.line, 2);
    assert_eq!(loc.column, 1);
}

#[test]
fn test_error_code_all_codes() {
    assert_eq!(ErrorCode::InvalidSyntax.code(), "E001");
    assert_eq!(ErrorCode::UnknownProperty.code(), "E002");
    assert_eq!(ErrorCode::InvalidValue.code(), "E003");
    assert_eq!(ErrorCode::MissingBrace.code(), "E004");
    assert_eq!(ErrorCode::MissingSemicolon.code(), "E005");
    assert_eq!(ErrorCode::InvalidSelector.code(), "E006");
    assert_eq!(ErrorCode::UndefinedVariable.code(), "E007");
    assert_eq!(ErrorCode::InvalidColor.code(), "E008");
    assert_eq!(ErrorCode::InvalidNumber.code(), "E009");
    assert_eq!(ErrorCode::EmptyRule.code(), "E010");
}

#[test]
fn test_error_code_descriptions() {
    assert!(!ErrorCode::InvalidSyntax.description().is_empty());
    assert!(!ErrorCode::UnknownProperty.description().is_empty());
    assert!(!ErrorCode::InvalidValue.description().is_empty());
}

#[test]
fn test_error_code_help() {
    assert!(!ErrorCode::InvalidSyntax.help().is_empty());
    assert!(!ErrorCode::UnknownProperty.help().is_empty());
    assert!(!ErrorCode::InvalidColor.help().is_empty());
}

#[test]
fn test_error_code_display_format() {
    let code = ErrorCode::InvalidSyntax;
    assert_eq!(format!("{}", code), "E001");
}

#[test]
fn test_error_code_equality() {
    assert_eq!(ErrorCode::InvalidSyntax, ErrorCode::InvalidSyntax);
    assert_ne!(ErrorCode::InvalidSyntax, ErrorCode::UnknownProperty);
}

#[test]
fn test_error_code_copy() {
    let code = ErrorCode::InvalidValue;
    let copied = code;
    assert_eq!(code, copied);
}

#[test]
fn test_severity_labels() {
    assert_eq!(Severity::Error.label(), "error");
    assert_eq!(Severity::Warning.label(), "warning");
    assert_eq!(Severity::Hint.label(), "hint");
}

#[test]
fn test_severity_colors() {
    assert!(Severity::Error.color().contains("\x1b["));
    assert!(Severity::Warning.color().contains("\x1b["));
    assert!(Severity::Hint.color().contains("\x1b["));
}

#[test]
fn test_severity_equality() {
    assert_eq!(Severity::Error, Severity::Error);
    assert_ne!(Severity::Error, Severity::Warning);
}

#[test]
fn test_source_location_default() {
    let loc = SourceLocation::default();
    assert_eq!(loc.line, 0);
    assert_eq!(loc.column, 0);
    assert_eq!(loc.offset, 0);
    assert_eq!(loc.length, 0);
}

#[test]
fn test_source_location_new() {
    let loc = SourceLocation::new(5, 10, 50, 3);
    assert_eq!(loc.line, 5);
    assert_eq!(loc.column, 10);
    assert_eq!(loc.offset, 50);
    assert_eq!(loc.length, 3);
}

#[test]
fn test_source_location_from_offset_first_line() {
    let source = "hello world";
    let loc = SourceLocation::from_offset(source, 6);
    assert_eq!(loc.line, 1);
    assert_eq!(loc.column, 7);
}

#[test]
fn test_source_location_from_offset_multiline() {
    let source = "line1\nline2\nline3";
    let loc = SourceLocation::from_offset(source, 12);
    assert_eq!(loc.line, 3);
    assert_eq!(loc.column, 1);
}

#[test]
fn test_source_location_from_offset_len() {
    let source = "hello world";
    let loc = SourceLocation::from_offset_len(source, 0, 5);
    assert_eq!(loc.length, 5);
}

#[test]
fn test_source_location_from_offset_empty() {
    let source = "";
    let loc = SourceLocation::from_offset(source, 0);
    assert_eq!(loc.line, 1);
    assert_eq!(loc.column, 1);
}

#[test]
fn test_suggestion_new() {
    let suggestion = Suggestion::new("try something else");
    assert_eq!(suggestion.message, "try something else");
    assert!(suggestion.replacement.is_none());
}

#[test]
fn test_suggestion_with_fix() {
    let suggestion = Suggestion::with_fix("did you mean", "color");
    assert_eq!(suggestion.message, "did you mean");
    assert_eq!(suggestion.replacement, Some("color".to_string()));
}

#[test]
fn test_suggestion_clone() {
    let suggestion = Suggestion::with_fix("hint", "fix");
    let cloned = suggestion.clone();
    assert_eq!(cloned.message, "hint");
    assert_eq!(cloned.replacement, Some("fix".to_string()));
}

#[test]
fn test_rich_parse_error_new() {
    let loc = SourceLocation::new(1, 5, 4, 3);
    let error = RichParseError::new(ErrorCode::InvalidValue, "invalid value", loc);

    assert_eq!(error.code, ErrorCode::InvalidValue);
    assert_eq!(error.severity, Severity::Error);
    assert_eq!(error.message, "invalid value");
    assert!(error.suggestions.is_empty());
    assert!(error.notes.is_empty());
}

#[test]
fn test_rich_parse_error_severity() {
    let error = RichParseError::new(
        ErrorCode::EmptyRule,
        "empty rule",
        SourceLocation::default(),
    )
    .severity(Severity::Warning);

    assert_eq!(error.severity, Severity::Warning);
}

#[test]
fn test_rich_parse_error_suggest() {
    let error = RichParseError::new(
        ErrorCode::UnknownProperty,
        "unknown property",
        SourceLocation::default(),
    )
    .suggest(Suggestion::new("check spelling"));

    assert_eq!(error.suggestions.len(), 1);
}

#[test]
fn test_rich_parse_error_note() {
    let error = RichParseError::new(
        ErrorCode::InvalidSyntax,
        "syntax error",
        SourceLocation::default(),
    )
    .note("see documentation");

    assert_eq!(error.notes.len(), 1);
    assert_eq!(error.notes[0], "see documentation");
}

#[test]
fn test_rich_parse_error_chained() {
    let error = RichParseError::new(
        ErrorCode::UnknownProperty,
        "unknown 'colr'",
        SourceLocation::new(2, 3, 10, 4),
    )
    .severity(Severity::Error)
    .suggest(Suggestion::with_fix("did you mean", "color"))
    .note("color is a valid CSS property");

    assert_eq!(error.suggestions.len(), 1);
    assert_eq!(error.notes.len(), 1);
}

#[test]
fn test_rich_parse_error_display() {
    let error = RichParseError::new(
        ErrorCode::InvalidColor,
        "invalid color format",
        SourceLocation::new(3, 10, 25, 5),
    );

    let display = format!("{}", error);
    assert!(display.contains("E008"));
    assert!(display.contains("invalid color format"));
    assert!(display.contains("line 3"));
}

#[test]
fn test_rich_parse_error_plain_text() {
    let source = ".button { color: invalid; }";
    let error = RichParseError::new(
        ErrorCode::InvalidValue,
        "invalid color value",
        SourceLocation::new(1, 18, 17, 7),
    );

    let plain = error.plain_text(source);
    assert!(plain.contains("error"));
    assert!(plain.contains("E003"));
    assert!(plain.contains("invalid"));
}

#[test]
fn test_rich_parse_error_pretty_print_contains_code() {
    let source = ".x { y: z; }";
    let error = RichParseError::new(
        ErrorCode::UnknownProperty,
        "unknown property 'y'",
        SourceLocation::new(1, 6, 5, 1),
    );

    let pretty = error.pretty_print(source);
    assert!(pretty.contains("E002"));
}

#[test]
fn test_parse_errors_new() {
    let errors = ParseErrors::new();
    assert!(errors.is_empty());
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_parse_errors_default() {
    let errors = ParseErrors::default();
    assert!(errors.is_empty());
}

#[test]
fn test_parse_errors_max_errors() {
    let errors = ParseErrors::new().max_errors(5);
    assert!(errors.is_empty());
}

#[test]
fn test_parse_errors_is_full() {
    let mut errors = ParseErrors::new().max_errors(2);

    errors.push(RichParseError::new(
        ErrorCode::InvalidSyntax,
        "error 1",
        SourceLocation::default(),
    ));
    assert!(!errors.is_full());

    errors.push(RichParseError::new(
        ErrorCode::InvalidSyntax,
        "error 2",
        SourceLocation::default(),
    ));
    assert!(errors.is_full());
}

#[test]
fn test_parse_errors_has_errors_with_warning() {
    let mut errors = ParseErrors::new();

    errors.push(
        RichParseError::new(ErrorCode::EmptyRule, "warning", SourceLocation::default())
            .severity(Severity::Warning),
    );

    assert!(!errors.has_errors());
}

#[test]
fn test_parse_errors_get_errors() {
    let mut errors = ParseErrors::new();
    errors.push(RichParseError::new(
        ErrorCode::InvalidSyntax,
        "test",
        SourceLocation::default(),
    ));

    let slice = errors.errors();
    assert_eq!(slice.len(), 1);
}

#[test]
fn test_parse_errors_pretty_print() {
    let mut errors = ParseErrors::new();
    errors.push(RichParseError::new(
        ErrorCode::InvalidSyntax,
        "syntax error",
        SourceLocation::new(1, 1, 0, 1),
    ));

    let source = "invalid { }";
    let output = errors.pretty_print(source);
    assert!(output.contains("1 error(s)"));
}

#[test]
fn test_known_properties_contains_common() {
    assert!(KNOWN_PROPERTIES.contains(&"color"));
    assert!(KNOWN_PROPERTIES.contains(&"background"));
    assert!(KNOWN_PROPERTIES.contains(&"padding"));
    assert!(KNOWN_PROPERTIES.contains(&"margin"));
    assert!(KNOWN_PROPERTIES.contains(&"border"));
    assert!(KNOWN_PROPERTIES.contains(&"width"));
    assert!(KNOWN_PROPERTIES.contains(&"height"));
}

#[test]
fn test_known_properties_contains_flex() {
    assert!(KNOWN_PROPERTIES.contains(&"display"));
    assert!(KNOWN_PROPERTIES.contains(&"flex-direction"));
    assert!(KNOWN_PROPERTIES.contains(&"justify-content"));
    assert!(KNOWN_PROPERTIES.contains(&"align-items"));
}

#[test]
fn test_known_properties_contains_grid() {
    assert!(KNOWN_PROPERTIES.contains(&"grid-template-columns"));
    assert!(KNOWN_PROPERTIES.contains(&"grid-template-rows"));
    assert!(KNOWN_PROPERTIES.contains(&"grid-column"));
    assert!(KNOWN_PROPERTIES.contains(&"grid-row"));
}
