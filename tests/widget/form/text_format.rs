//! TextFormat value tests
//!
//! Default/new and toggling each flag on (and bold/italic back off) are
//! covered by the in-source text_format tests.

use revue::widget::TextFormat;

#[test]
fn test_text_format_equality() {
    let bold = TextFormat {
        bold: true,
        ..Default::default()
    };
    assert_eq!(bold, TextFormat::new().toggle_bold());
    assert_ne!(bold, TextFormat::new().toggle_italic());
    assert_ne!(bold, TextFormat::new());
}

#[test]
fn test_text_format_toggles_turn_set_flags_off() {
    let all = TextFormat {
        bold: true,
        italic: true,
        underline: true,
        strikethrough: true,
        code: true,
    };

    assert_eq!(
        all.toggle_underline(),
        TextFormat {
            underline: false,
            ..all
        }
    );
    assert_eq!(
        all.toggle_strikethrough(),
        TextFormat {
            strikethrough: false,
            ..all
        }
    );
    assert_eq!(all.toggle_code(), TextFormat { code: false, ..all });
}

#[test]
fn test_text_format_each_toggle_touches_only_its_flag() {
    let none = TextFormat::new();
    assert_eq!(
        none.toggle_underline(),
        TextFormat {
            underline: true,
            ..none
        }
    );
    assert_eq!(
        none.toggle_strikethrough(),
        TextFormat {
            strikethrough: true,
            ..none
        }
    );
    assert_eq!(none.toggle_code(), TextFormat { code: true, ..none });
}

#[test]
fn test_text_format_all_toggles_on_then_off() {
    let on = TextFormat::default()
        .toggle_bold()
        .toggle_italic()
        .toggle_underline()
        .toggle_strikethrough()
        .toggle_code();
    assert_eq!(
        on,
        TextFormat {
            bold: true,
            italic: true,
            underline: true,
            strikethrough: true,
            code: true,
        }
    );

    let off = on
        .toggle_bold()
        .toggle_italic()
        .toggle_underline()
        .toggle_strikethrough()
        .toggle_code();
    assert_eq!(off, TextFormat::default());
}
