//! Tests for rich_text_editor text_format module

use revue::widget::form::rich_text_editor::text_format::TextFormat;

// =========================================================================
// TextFormat struct tests
// =========================================================================

#[test]
fn test_text_format_default() {
    let fmt = TextFormat::default();
    assert!(!fmt.bold);
    assert!(!fmt.italic);
    assert!(!fmt.underline);
    assert!(!fmt.strikethrough);
    assert!(!fmt.code);
}

#[test]
fn test_text_format_new() {
    let fmt = TextFormat::new();
    assert!(!fmt.bold);
    assert!(!fmt.italic);
    assert!(!fmt.underline);
    assert!(!fmt.strikethrough);
    assert!(!fmt.code);
}

#[test]
fn test_text_format_clone() {
    let fmt1 = TextFormat {
        bold: true,
        ..Default::default()
    };
    let fmt2 = fmt1.clone();
    assert_eq!(fmt1, fmt2);
}

#[test]
fn test_text_format_copy() {
    let fmt1 = TextFormat {
        italic: true,
        ..Default::default()
    };
    let fmt2 = fmt1;
    assert_eq!(
        fmt1,
        TextFormat {
            italic: true,
            ..Default::default()
        }
    );
    assert_eq!(
        fmt2,
        TextFormat {
            italic: true,
            ..Default::default()
        }
    );
}

#[test]
fn test_text_format_equality() {
    let fmt1 = TextFormat {
        bold: true,
        ..Default::default()
    };
    let fmt2 = TextFormat {
        bold: true,
        ..Default::default()
    };
    assert_eq!(fmt1, fmt2);

    let fmt3 = TextFormat {
        italic: true,
        ..Default::default()
    };
    assert_ne!(fmt1, fmt3);
}

#[test]
fn test_text_format_debug() {
    let fmt = TextFormat {
        bold: true,
        ..Default::default()
    };
    let debug_str = format!("{:?}", fmt);
    assert!(debug_str.contains("TextFormat"));
}

// =========================================================================
// TextFormat field tests
// =========================================================================

#[test]
fn test_text_format_bold_field() {
    let mut fmt = TextFormat::default();
    fmt.bold = true;
    assert!(fmt.bold);
    assert!(!fmt.italic);
}

#[test]
fn test_text_format_italic_field() {
    let mut fmt = TextFormat::default();
    fmt.italic = true;
    assert!(fmt.italic);
    assert!(!fmt.bold);
}

#[test]
fn test_text_format_underline_field() {
    let mut fmt = TextFormat::default();
    fmt.underline = true;
    assert!(fmt.underline);
}

#[test]
fn test_text_format_strikethrough_field() {
    let mut fmt = TextFormat::default();
    fmt.strikethrough = true;
    assert!(fmt.strikethrough);
}

#[test]
fn test_text_format_code_field() {
    let mut fmt = TextFormat::default();
    fmt.code = true;
    assert!(fmt.code);
}

// =========================================================================
// TextFormat::toggle_bold tests
// =========================================================================

#[test]
fn test_toggle_bold_from_false() {
    let fmt = TextFormat::default().toggle_bold();
    assert!(fmt.bold);
    assert!(!fmt.italic);
}

#[test]
fn test_toggle_bold_from_true() {
    let fmt = TextFormat {
        bold: true,
        ..Default::default()
    }
    .toggle_bold();
    assert!(!fmt.bold);
}

#[test]
fn test_toggle_bold_chainable() {
    let fmt = TextFormat::default().toggle_bold().toggle_bold();
    assert!(!fmt.bold);
}

// =========================================================================
// TextFormat::toggle_italic tests
// =========================================================================

#[test]
fn test_toggle_italic_from_false() {
    let fmt = TextFormat::default().toggle_italic();
    assert!(fmt.italic);
    assert!(!fmt.bold);
}

#[test]
fn test_toggle_italic_from_true() {
    let fmt = TextFormat {
        italic: true,
        ..Default::default()
    }
    .toggle_italic();
    assert!(!fmt.italic);
}

#[test]
fn test_toggle_italic_chainable() {
    let fmt = TextFormat::default().toggle_italic().toggle_italic();
    assert!(!fmt.italic);
}

// =========================================================================
// TextFormat::toggle_underline tests
// =========================================================================

#[test]
fn test_toggle_underline_from_false() {
    let fmt = TextFormat::default().toggle_underline();
    assert!(fmt.underline);
}

#[test]
fn test_toggle_underline_from_true() {
    let fmt = TextFormat {
        underline: true,
        ..Default::default()
    }
    .toggle_underline();
    assert!(!fmt.underline);
}

#[test]
fn test_toggle_underline_chainable() {
    let fmt = TextFormat::default().toggle_underline().toggle_underline();
    assert!(!fmt.underline);
}

// =========================================================================
// TextFormat::toggle_strikethrough tests
// =========================================================================

#[test]
fn test_toggle_strikethrough_from_false() {
    let fmt = TextFormat::default().toggle_strikethrough();
    assert!(fmt.strikethrough);
}

#[test]
fn test_toggle_strikethrough_from_true() {
    let fmt = TextFormat {
        strikethrough: true,
        ..Default::default()
    }
    .toggle_strikethrough();
    assert!(!fmt.strikethrough);
}

#[test]
fn test_toggle_strikethrough_chainable() {
    let fmt = TextFormat::default()
        .toggle_strikethrough()
        .toggle_strikethrough();
    assert!(!fmt.strikethrough);
}

// =========================================================================
// TextFormat::toggle_code tests
// =========================================================================

#[test]
fn test_toggle_code_from_false() {
    let fmt = TextFormat::default().toggle_code();
    assert!(fmt.code);
}

#[test]
fn test_toggle_code_from_true() {
    let fmt = TextFormat {
        code: true,
        ..Default::default()
    }
    .toggle_code();
    assert!(!fmt.code);
}

#[test]
fn test_toggle_code_chainable() {
    let fmt = TextFormat::default().toggle_code().toggle_code();
    assert!(!fmt.code);
}

// =========================================================================
// Toggle chain tests
// =========================================================================

#[test]
fn test_toggle_bold_italic_chain() {
    let fmt = TextFormat::default().toggle_bold().toggle_italic();
    assert!(fmt.bold);
    assert!(fmt.italic);
    assert!(!fmt.underline);
}

#[test]
fn test_all_toggles_on() {
    let fmt = TextFormat::default()
        .toggle_bold()
        .toggle_italic()
        .toggle_underline()
        .toggle_strikethrough()
        .toggle_code();
    assert!(fmt.bold);
    assert!(fmt.italic);
    assert!(fmt.underline);
    assert!(fmt.strikethrough);
    assert!(fmt.code);
}

#[test]
fn test_all_toggles_on_then_off() {
    let fmt = TextFormat::default()
        .toggle_bold()
        .toggle_italic()
        .toggle_underline()
        .toggle_strikethrough()
        .toggle_code()
        .toggle_bold()
        .toggle_italic()
        .toggle_underline()
        .toggle_strikethrough()
        .toggle_code();
    assert!(!fmt.bold);
    assert!(!fmt.italic);
    assert!(!fmt.underline);
    assert!(!fmt.strikethrough);
    assert!(!fmt.code);
}

// =========================================================================
// Combination tests
// =========================================================================

#[test]
fn test_bold_italic_combination() {
    let fmt = TextFormat {
        bold: true,
        italic: true,
        ..Default::default()
    };
    assert!(fmt.bold);
    assert!(fmt.italic);
    assert!(!fmt.underline);
}

#[test]
fn test_code_bold_combination() {
    let fmt = TextFormat {
        code: true,
        bold: true,
        ..Default::default()
    };
    assert!(fmt.code);
    assert!(fmt.bold);
    assert!(!fmt.italic);
}

#[test]
fn test_all_formats_true() {
    let fmt = TextFormat {
        bold: true,
        italic: true,
        underline: true,
        strikethrough: true,
        code: true,
    };
    assert!(fmt.bold);
    assert!(fmt.italic);
    assert!(fmt.underline);
    assert!(fmt.strikethrough);
    assert!(fmt.code);
}
