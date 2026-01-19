//! Error tests

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
