//! Tests for Tooltip widget
//!
//! Extracted from src/widget/feedback/tooltip.rs

use revue::prelude::*;

// =========================================================================
// TooltipPosition enum tests
// =========================================================================

#[test]
fn test_tooltip_position_default() {
    assert_eq!(TooltipPosition::default(), TooltipPosition::Top);
}

#[test]
fn test_tooltip_position_clone() {
    let pos1 = TooltipPosition::Bottom;
    let pos2 = pos1.clone();
    assert_eq!(pos1, pos2);
}

#[test]
fn test_tooltip_position_copy() {
    let pos1 = TooltipPosition::Left;
    let pos2 = pos1;
    assert_eq!(pos2, TooltipPosition::Left);
    // pos1 is still valid because of Copy
    assert_eq!(pos1, TooltipPosition::Left);
}

#[test]
fn test_tooltip_position_partial_eq() {
    assert_eq!(TooltipPosition::Top, TooltipPosition::Top);
    assert_eq!(TooltipPosition::Bottom, TooltipPosition::Bottom);
    assert_eq!(TooltipPosition::Left, TooltipPosition::Left);
    assert_eq!(TooltipPosition::Right, TooltipPosition::Right);
    assert_eq!(TooltipPosition::Auto, TooltipPosition::Auto);

    assert_ne!(TooltipPosition::Top, TooltipPosition::Bottom);
    assert_ne!(TooltipPosition::Left, TooltipPosition::Right);
    assert_ne!(TooltipPosition::Auto, TooltipPosition::Top);
}

#[test]
fn test_tooltip_position_all_variants() {
    let positions = [
        TooltipPosition::Top,
        TooltipPosition::Bottom,
        TooltipPosition::Left,
        TooltipPosition::Right,
        TooltipPosition::Auto,
    ];

    // Verify all variants are distinct
    for (i, pos1) in positions.iter().enumerate() {
        for (j, pos2) in positions.iter().enumerate() {
            if i == j {
                assert_eq!(pos1, pos2);
            } else {
                assert_ne!(pos1, pos2);
            }
        }
    }
}

// =========================================================================
// TooltipArrow enum tests
// =========================================================================

#[test]
fn test_tooltip_arrow_default() {
    assert_eq!(TooltipArrow::default(), TooltipArrow::None);
}

#[test]
fn test_tooltip_arrow_clone() {
    let arrow1 = TooltipArrow::Unicode;
    let arrow2 = arrow1.clone();
    assert_eq!(arrow1, arrow2);
}

#[test]
fn test_tooltip_arrow_copy() {
    let arrow1 = TooltipArrow::Simple;
    let arrow2 = arrow1;
    assert_eq!(arrow2, TooltipArrow::Simple);
    // arrow1 is still valid because of Copy
    assert_eq!(arrow1, TooltipArrow::Simple);
}

#[test]
fn test_tooltip_arrow_partial_eq() {
    assert_eq!(TooltipArrow::None, TooltipArrow::None);
    assert_eq!(TooltipArrow::Simple, TooltipArrow::Simple);
    assert_eq!(TooltipArrow::Unicode, TooltipArrow::Unicode);

    assert_ne!(TooltipArrow::None, TooltipArrow::Simple);
    assert_ne!(TooltipArrow::Simple, TooltipArrow::Unicode);
    assert_ne!(TooltipArrow::Unicode, TooltipArrow::None);
}

#[test]
fn test_tooltip_arrow_all_variants() {
    let arrows = [
        TooltipArrow::None,
        TooltipArrow::Simple,
        TooltipArrow::Unicode,
    ];

    // Verify all variants are distinct
    for (i, arrow1) in arrows.iter().enumerate() {
        for (j, arrow2) in arrows.iter().enumerate() {
            if i == j {
                assert_eq!(arrow1, arrow2);
            } else {
                assert_ne!(arrow1, arrow2);
            }
        }
    }
}

#[test]
fn test_arrow_chars() {
    let arrow = TooltipArrow::Unicode;
    let (top, _) = arrow.chars(TooltipPosition::Top);
    assert_eq!(top, '▼');

    let (bottom, _) = arrow.chars(TooltipPosition::Bottom);
    assert_eq!(bottom, '▲');
}

#[test]
fn test_tooltip_arrow_chars_all_combinations() {
    let arrows = [
        TooltipArrow::None,
        TooltipArrow::Simple,
        TooltipArrow::Unicode,
    ];
    let positions = [
        TooltipPosition::Top,
        TooltipPosition::Bottom,
        TooltipPosition::Left,
        TooltipPosition::Right,
        TooltipPosition::Auto,
    ];

    for arrow in arrows {
        for pos in positions {
            let (char1, char2) = arrow.chars(pos);
            // Verify chars are valid (no panics) and are valid Unicode chars
            assert!(char1.len_utf8() >= 1);
            assert!(char2.len_utf8() >= 1);
        }
    }
}

// =========================================================================
// TooltipStyle enum tests
// =========================================================================

#[test]
fn test_tooltip_style_default() {
    assert_eq!(TooltipStyle::default(), TooltipStyle::Plain);
}

#[test]
fn test_tooltip_style_clone() {
    let style1 = TooltipStyle::Info;
    let style2 = style1.clone();
    assert_eq!(style1, style2);
}

#[test]
fn test_tooltip_style_copy() {
    let style1 = TooltipStyle::Warning;
    let style2 = style1;
    assert_eq!(style2, TooltipStyle::Warning);
    // style1 is still valid because of Copy
    assert_eq!(style1, TooltipStyle::Warning);
}

#[test]
fn test_tooltip_style_partial_eq() {
    assert_eq!(TooltipStyle::Plain, TooltipStyle::Plain);
    assert_eq!(TooltipStyle::Bordered, TooltipStyle::Bordered);
    assert_eq!(TooltipStyle::Rounded, TooltipStyle::Rounded);
    assert_eq!(TooltipStyle::Info, TooltipStyle::Info);
    assert_eq!(TooltipStyle::Warning, TooltipStyle::Warning);
    assert_eq!(TooltipStyle::Error, TooltipStyle::Error);
    assert_eq!(TooltipStyle::Success, TooltipStyle::Success);

    assert_ne!(TooltipStyle::Plain, TooltipStyle::Bordered);
    assert_ne!(TooltipStyle::Info, TooltipStyle::Warning);
    assert_ne!(TooltipStyle::Error, TooltipStyle::Success);
}

#[test]
fn test_tooltip_style_all_variants() {
    let styles = [
        TooltipStyle::Plain,
        TooltipStyle::Bordered,
        TooltipStyle::Rounded,
        TooltipStyle::Info,
        TooltipStyle::Warning,
        TooltipStyle::Error,
        TooltipStyle::Success,
    ];

    // Verify all variants are distinct
    for (i, style1) in styles.iter().enumerate() {
        for (j, style2) in styles.iter().enumerate() {
            if i == j {
                assert_eq!(style1, style2);
            } else {
                assert_ne!(style1, style2);
            }
        }
    }
}

#[test]
fn test_tooltip_style_colors_all_variants() {
    let styles = [
        TooltipStyle::Plain,
        TooltipStyle::Bordered,
        TooltipStyle::Rounded,
        TooltipStyle::Info,
        TooltipStyle::Warning,
        TooltipStyle::Error,
        TooltipStyle::Success,
    ];

    for style in styles {
        let (fg, bg) = style.colors();
        // u8 values are always valid 0-255, just verify colors exist
        let _ = (fg.r, fg.g, fg.b, bg.r, bg.g, bg.b);
    }
}

#[test]
fn test_tooltip_style_border_chars_all_variants() {
    let styles = [
        TooltipStyle::Plain,
        TooltipStyle::Bordered,
        TooltipStyle::Rounded,
        TooltipStyle::Info,
        TooltipStyle::Warning,
        TooltipStyle::Error,
        TooltipStyle::Success,
    ];

    for style in styles {
        let border = style.border_chars();
        // Plain should have no border
        if matches!(style, TooltipStyle::Plain) {
            assert!(border.is_none());
        } else {
            assert!(border.is_some());
        }
    }
}

// =========================================================================
// Tooltip builder tests
// =========================================================================

#[test]
fn test_tooltip_new() {
    let t = Tooltip::new("Test tooltip");
    assert_eq!(t.get_text(), "Test tooltip");
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_builder() {
    let t = Tooltip::new("Hello")
        .position(TooltipPosition::Bottom)
        .anchor(10, 5)
        .style(TooltipStyle::Info)
        .arrow(TooltipArrow::Unicode)
        .max_width(30);

    assert!(matches!(t.get_position(), TooltipPosition::Bottom));
    assert_eq!(t.get_anchor(), (10, 5));
    assert!(matches!(t.get_style(), TooltipStyle::Info));
    assert_eq!(t.get_max_width(), 30);
}

#[test]
fn test_tooltip_presets() {
    let info = Tooltip::info("Info message");
    assert!(matches!(info.get_style(), TooltipStyle::Info));

    let warning = Tooltip::warning("Warning!");
    assert!(matches!(warning.get_style(), TooltipStyle::Warning));

    let error = Tooltip::error("Error!");
    assert!(matches!(error.get_style(), TooltipStyle::Error));

    let success = Tooltip::success("Success!");
    assert!(matches!(success.get_style(), TooltipStyle::Success));
}

#[test]
fn test_tooltip_helper_text() {
    let t = tooltip("Quick tooltip");
    assert_eq!(t.get_text(), "Quick tooltip");
}

#[test]
fn test_tooltip_visibility() {
    let mut t = Tooltip::new("Test");
    assert!(t.is_visible());

    t.hide();
    assert!(!t.is_visible());

    t.show();
    assert!(t.is_visible());

    t.toggle();
    assert!(!t.is_visible());
}

#[test]
fn test_tooltip_delay_tick() {
    let mut t = Tooltip::new("Test").delay(5);
    assert_eq!(t.get_delay(), 5);
    assert_eq!(t.get_delay_counter(), 0);

    // Initially not visible due to delay
    assert!(!t.is_visible());

    // Tick until visible
    for _ in 0..5 {
        t.tick();
    }
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_set_anchor() {
    let mut t = Tooltip::new("Test");
    t.set_anchor(15, 20);
    assert_eq!(t.get_anchor(), (15, 20));
}

#[test]
fn test_tooltip_title() {
    let t = Tooltip::new("Content").title("My Title");
    assert_eq!(t.get_title(), Some("My Title"));
}

// =========================================================================
// Tooltip Default trait tests
// =========================================================================

#[test]
fn test_tooltip_default_trait() {
    let tooltip = Tooltip::default();
    assert_eq!(tooltip.get_text(), "");
    assert!(tooltip.is_visible());
    assert_eq!(tooltip.get_position(), TooltipPosition::Top);
    assert_eq!(tooltip.get_anchor(), (0, 0));
    assert!(matches!(tooltip.get_style(), TooltipStyle::Bordered));
    assert!(matches!(tooltip.get_arrow(), TooltipArrow::Unicode));
    assert_eq!(tooltip.get_max_width(), 40);
}

#[test]
fn test_tooltip_default_vs_new_empty() {
    let default_tooltip = Tooltip::default();
    let new_tooltip = Tooltip::new("");

    assert_eq!(default_tooltip.get_text(), new_tooltip.get_text());
    assert_eq!(default_tooltip.is_visible(), new_tooltip.is_visible());
}
