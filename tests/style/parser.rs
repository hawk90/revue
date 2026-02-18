//! Parser tests

#![allow(unused_imports)]

use revue::style::{
    easing, lerp_f32, lerp_u8, parse_css, shared_theme, theme_manager, ActiveTransition,
    AnimationDirection, AnimationFillMode, AnimationGroup, AnimationState, Color, ComputedStyle,
    CssKeyframe, Display, Easing, ErrorCode, FlexDirection, KeyframeAnimation,
    KeyframeBlock, KeyframesDefinition, Palette,
    ParseErrors, Position, RichParseError, Severity, SharedTheme, Size, SourceLocation, Spacing,
    Stagger, Style, Suggestion, Theme, ThemeColors, ThemeManager, ThemeVariant, Themes, Transition,
    TransitionManager, Transitions, Tween, KNOWN_PROPERTIES,
};
use std::time::Duration;

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
    // Grid template parsing is now implemented
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

// =====================================================
// @keyframes parsing tests
// =====================================================

#[test]
fn test_parse_keyframes_from_to() {
    let css = r#"
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
    "#;
    let sheet = parse_css(css).unwrap();

    assert!(sheet.keyframes.contains_key("fadeIn"));
    let def = sheet.keyframes_definition("fadeIn").unwrap();
    assert_eq!(def.name, "fadeIn");
    assert_eq!(def.keyframes.len(), 2);
    assert_eq!(def.keyframes[0].percent, 0);
    assert_eq!(def.keyframes[0].declarations[0].property, "opacity");
    assert_eq!(def.keyframes[0].declarations[0].value, "0");
    assert_eq!(def.keyframes[1].percent, 100);
    assert_eq!(def.keyframes[1].declarations[0].property, "opacity");
    assert_eq!(def.keyframes[1].declarations[0].value, "1");
}

#[test]
fn test_parse_keyframes_percentages() {
    let css = r#"
        @keyframes slideIn {
            0% { opacity: 0; }
            50% { opacity: 0.5; }
            100% { opacity: 1; }
        }
    "#;
    let sheet = parse_css(css).unwrap();

    let def = sheet.keyframes_definition("slideIn").unwrap();
    assert_eq!(def.keyframes.len(), 3);
    assert_eq!(def.keyframes[0].percent, 0);
    assert_eq!(def.keyframes[1].percent, 50);
    assert_eq!(def.keyframes[2].percent, 100);
}

#[test]
fn test_parse_keyframes_multiple_declarations() {
    let css = r#"
        @keyframes complex {
            from { opacity: 0; x: -20; }
            to { opacity: 1; x: 0; }
        }
    "#;
    let sheet = parse_css(css).unwrap();

    let def = sheet.keyframes_definition("complex").unwrap();
    assert_eq!(def.keyframes[0].declarations.len(), 2);
    assert_eq!(def.keyframes[0].declarations[0].property, "opacity");
    assert_eq!(def.keyframes[0].declarations[1].property, "x");
}

#[test]
fn test_parse_keyframes_empty_body() {
    let css = r#"
        @keyframes empty {
        }
    "#;
    let sheet = parse_css(css).unwrap();

    let def = sheet.keyframes_definition("empty").unwrap();
    assert_eq!(def.keyframes.len(), 0);
}

#[test]
fn test_parse_keyframes_missing_name() {
    let css = "@keyframes { from { opacity: 0; } }";
    let result = parse_css(css);
    assert!(result.is_err());
}

#[test]
fn test_parse_keyframes_with_rules() {
    let css = r#"
        .button { color: red; }

        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }

        .text { color: blue; }
    "#;
    let sheet = parse_css(css).unwrap();

    assert_eq!(sheet.rules.len(), 2);
    assert_eq!(sheet.rules[0].selector, ".button");
    assert_eq!(sheet.rules[1].selector, ".text");
    assert!(sheet.keyframes.contains_key("fadeIn"));
}

#[test]
fn test_parse_multiple_keyframes() {
    let css = r#"
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        @keyframes slideUp {
            from { y: 100; }
            to { y: 0; }
        }
    "#;
    let sheet = parse_css(css).unwrap();

    assert_eq!(sheet.keyframes.len(), 2);
    assert!(sheet.keyframes.contains_key("fadeIn"));
    assert!(sheet.keyframes.contains_key("slideUp"));
}

// =====================================================
// animation shorthand/longhand parsing tests
// =====================================================

#[test]
fn test_animation_shorthand_resolves_to_keyframe_animation() {
    let css = r#"
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        .box {
            animation: fadeIn 0.3s ease-in-out;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    let anim = sheet.animation(".box");

    assert!(anim.is_some());
    let anim = anim.unwrap();
    assert_eq!(anim.name(), "fadeIn");
    assert_eq!(anim.duration, Duration::from_millis(300));
}

#[test]
fn test_animation_shorthand_with_iterations() {
    let css = r#"
        @keyframes pulse {
            from { opacity: 1; }
            to { opacity: 0.5; }
        }
        .pulse {
            animation: pulse 1s ease-in infinite alternate;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    let anim = sheet.animation(".pulse").unwrap();

    assert_eq!(anim.name(), "pulse");
    assert_eq!(anim.duration, Duration::from_secs(1));
    assert_eq!(anim.iterations, 0); // 0 = infinite
    assert_eq!(anim.direction, AnimationDirection::Alternate);
}

#[test]
fn test_animation_longhand_properties() {
    let css = r#"
        @keyframes slideIn {
            0% { x: -100; }
            100% { x: 0; }
        }
        .slide {
            animation-name: slideIn;
            animation-duration: 500ms;
            animation-timing-function: ease-out;
            animation-delay: 100ms;
            animation-iteration-count: 3;
            animation-direction: reverse;
            animation-fill-mode: forwards;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    let anim = sheet.animation(".slide").unwrap();

    assert_eq!(anim.name(), "slideIn");
    assert_eq!(anim.duration, Duration::from_millis(500));
    assert_eq!(anim.delay, Duration::from_millis(100));
    assert_eq!(anim.iterations, 3);
    assert_eq!(anim.direction, AnimationDirection::Reverse);
    assert_eq!(anim.fill_mode, AnimationFillMode::Forwards);
}

#[test]
fn test_animation_nonexistent_keyframes_returns_none() {
    let css = r#"
        .box {
            animation: nonexistent 1s;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    assert!(sheet.animation(".box").is_none());
}

#[test]
fn test_animation_no_animation_property_returns_none() {
    let css = r#"
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        .box { color: red; }
    "#;
    let sheet = parse_css(css).unwrap();
    assert!(sheet.animation(".box").is_none());
}

// =====================================================
// StyleSheet::animation() resolution tests
// =====================================================

#[test]
fn test_stylesheet_animation_builds_keyframes() {
    let css = r#"
        @keyframes fadeSlide {
            0% { opacity: 0; x: -20; }
            50% { opacity: 1; x: 10; }
            100% { opacity: 1; x: 0; }
        }
        .animated {
            animation: fadeSlide 500ms ease-out;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    let mut anim = sheet.animation(".animated").unwrap();

    anim.start();
    // Animation should start (or complete instantly if reduced motion)
    assert!(anim.is_running() || anim.is_completed());
}

#[test]
fn test_stylesheet_animation_fill_mode_both() {
    let css = r#"
        @keyframes grow {
            from { scale: 0; }
            to { scale: 1; }
        }
        .grow {
            animation: grow 1s both;
        }
    "#;
    let sheet = parse_css(css).unwrap();
    let anim = sheet.animation(".grow").unwrap();

    assert_eq!(anim.fill_mode, AnimationFillMode::Both);
}

// =====================================================
// merge() with keyframes tests
// =====================================================

#[test]
fn test_merge_stylesheets_with_keyframes() {
    let css1 = r#"
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
    "#;
    let css2 = r#"
        @keyframes slideUp {
            from { y: 100; }
            to { y: 0; }
        }
    "#;
    let mut sheet1 = parse_css(css1).unwrap();
    let sheet2 = parse_css(css2).unwrap();

    sheet1.merge(sheet2);

    assert_eq!(sheet1.keyframes.len(), 2);
    assert!(sheet1.keyframes.contains_key("fadeIn"));
    assert!(sheet1.keyframes.contains_key("slideUp"));
}

#[test]
fn test_keyframes_definition_accessor() {
    let css = r#"
        @keyframes bounce {
            0% { y: 0; }
            50% { y: -20; }
            100% { y: 0; }
        }
    "#;
    let sheet = parse_css(css).unwrap();

    assert!(sheet.keyframes_definition("bounce").is_some());
    assert!(sheet.keyframes_definition("nonexistent").is_none());
}
