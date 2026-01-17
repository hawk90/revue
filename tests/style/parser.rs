//! Parser tests

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
