//! Popover widget tests
//!
//! Tests for the Popover widget extracted from source files.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::feedback::popover;
use revue::widget::feedback::Popover;
use revue::widget::traits::View;

// =========================================================================
// Popover basic tests
// =========================================================================

#[test]
fn test_popover_new() {
    let p = Popover::new("Test content");
    assert!(!p.is_open());
    // Note: accessing pub(crate) fields directly
}

#[test]
fn test_popover_builder() {
    let p = Popover::new("Content")
        .anchor(10, 5)
        .position(revue::widget::feedback::PopoverPosition::Top)
        .trigger(revue::widget::feedback::PopoverTrigger::Hover);

    assert_eq!(p.anchor, (10, 5));
    assert_eq!(p.position, revue::widget::feedback::PopoverPosition::Top);
    assert_eq!(p.trigger, revue::widget::feedback::PopoverTrigger::Hover);
}

#[test]
fn test_popover_visibility() {
    let mut p = Popover::new("Test");
    assert!(!p.is_open());

    p.show();
    assert!(p.is_open());

    p.hide();
    assert!(!p.is_open());

    p.toggle();
    assert!(p.is_open());

    p.toggle();
    assert!(!p.is_open());
}

#[test]
fn test_popover_handle_escape() {
    let mut p = Popover::new("Test").open(true);
    assert!(p.is_open());

    assert!(p.handle_key(&Key::Escape));
    assert!(!p.is_open());
}

#[test]
fn test_popover_handle_escape_disabled() {
    let mut p = Popover::new("Test").open(true).close_on_escape(false);
    assert!(!p.handle_key(&Key::Escape));
    assert!(p.is_open());
}

#[test]
fn test_popover_handle_key_closed() {
    let mut p = Popover::new("Test");
    assert!(!p.handle_key(&Key::Escape));
}

#[test]
fn test_popover_helper() {
    let p = popover("Quick popover");
    assert_eq!(p.content, "Quick popover");
}

#[test]
fn test_popover_default() {
    let p = Popover::default();
    assert_eq!(p.content, "");
    assert!(!p.is_open());
    assert_eq!(p.position, revue::widget::feedback::PopoverPosition::Bottom);
}

#[test]
fn test_popover_set_anchor() {
    let mut p = Popover::new("Test");
    p.set_anchor(15, 25);
    assert_eq!(p.anchor, (15, 25));
}

#[test]
fn test_popover_trigger_types() {
    use revue::widget::feedback::PopoverTrigger;

    let p_click = Popover::new("Test").trigger(PopoverTrigger::Click);
    let p_hover = Popover::new("Test").trigger(PopoverTrigger::Hover);
    let p_focus = Popover::new("Test").trigger(PopoverTrigger::Focus);
    let p_manual = Popover::new("Test").trigger(PopoverTrigger::Manual);

    assert_eq!(p_click.trigger, PopoverTrigger::Click);
    assert_eq!(p_hover.trigger, PopoverTrigger::Hover);
    assert_eq!(p_focus.trigger, PopoverTrigger::Focus);
    assert_eq!(p_manual.trigger, PopoverTrigger::Manual);
}

#[test]
fn test_popover_custom_colors() {
    use revue::style::Color;

    let p = Popover::new("Test")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .border_color(Color::GREEN);

    assert_eq!(p.state.fg, Some(Color::RED));
    assert_eq!(p.state.bg, Some(Color::BLUE));
    assert_eq!(p.border_color, Some(Color::GREEN));
}

#[test]
fn test_popover_handle_click_inside() {
    let mut p = Popover::new("Test").anchor(20, 10).open(true);

    // Click inside the popover area
    let handled = p.handle_click(20, 12, 40, 20);
    assert!(handled);
    assert!(p.is_open()); // Should stay open
}

#[test]
fn test_popover_handle_click_outside() {
    let mut p = Popover::new("Test").anchor(20, 10).open(true);

    // Click outside the popover
    let handled = p.handle_click(0, 0, 40, 20);
    assert!(handled);
    assert!(!p.is_open()); // Should close
}

#[test]
fn test_popover_handle_click_outside_disabled() {
    let mut p = Popover::new("Test")
        .anchor(20, 10)
        .open(true)
        .close_on_click_outside(false);

    let handled = p.handle_click(0, 0, 40, 20);
    assert!(!handled);
    assert!(p.is_open()); // Should stay open
}

#[test]
fn test_popover_handle_click_on_anchor() {
    use revue::widget::feedback::PopoverTrigger;

    let mut p = Popover::new("Test")
        .anchor(10, 5)
        .trigger(PopoverTrigger::Click);
    assert!(!p.is_open());

    // Click on anchor should open
    let handled = p.handle_click(10, 5, 40, 20);
    assert!(handled);
    assert!(p.is_open());
}

#[test]
fn test_popover_handle_click_on_anchor_hover_trigger() {
    use revue::widget::feedback::PopoverTrigger;

    let mut p = Popover::new("Test")
        .anchor(10, 5)
        .trigger(PopoverTrigger::Hover);

    // Hover trigger shouldn't toggle on click
    let handled = p.handle_click(10, 5, 40, 20);
    assert!(!handled);
    assert!(!p.is_open());
}

// =========================================================================
// PopoverPosition enum tests
// =========================================================================

#[test]
fn test_popover_position_default() {
    use revue::widget::feedback::PopoverPosition;
    assert_eq!(PopoverPosition::default(), PopoverPosition::Bottom);
}

#[test]
fn test_popover_position_clone() {
    use revue::widget::feedback::PopoverPosition;
    let pos1 = PopoverPosition::Top;
    let pos2 = pos1.clone();
    assert_eq!(pos1, pos2);
}

#[test]
fn test_popover_position_copy() {
    use revue::widget::feedback::PopoverPosition;
    let pos1 = PopoverPosition::Left;
    let pos2 = pos1;
    assert_eq!(pos2, PopoverPosition::Left);
    assert_eq!(pos1, PopoverPosition::Left);
}

#[test]
fn test_popover_position_partial_eq() {
    use revue::widget::feedback::PopoverPosition;
    assert_eq!(PopoverPosition::Top, PopoverPosition::Top);
    assert_eq!(PopoverPosition::Bottom, PopoverPosition::Bottom);
    assert_eq!(PopoverPosition::Left, PopoverPosition::Left);
    assert_eq!(PopoverPosition::Right, PopoverPosition::Right);
    assert_eq!(PopoverPosition::Auto, PopoverPosition::Auto);

    assert_ne!(PopoverPosition::Top, PopoverPosition::Bottom);
    assert_ne!(PopoverPosition::Left, PopoverPosition::Right);
    assert_ne!(PopoverPosition::Auto, PopoverPosition::Top);
}

#[test]
fn test_popover_position_all_variants() {
    use revue::widget::feedback::PopoverPosition;
    let positions = [
        PopoverPosition::Top,
        PopoverPosition::Bottom,
        PopoverPosition::Left,
        PopoverPosition::Right,
        PopoverPosition::Auto,
    ];

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
// PopoverTrigger enum tests
// =========================================================================

#[test]
fn test_popover_trigger_default() {
    use revue::widget::feedback::PopoverTrigger;
    assert_eq!(PopoverTrigger::default(), PopoverTrigger::Click);
}

#[test]
fn test_popover_trigger_clone() {
    use revue::widget::feedback::PopoverTrigger;
    let trigger1 = PopoverTrigger::Hover;
    let trigger2 = trigger1.clone();
    assert_eq!(trigger1, trigger2);
}

#[test]
fn test_popover_trigger_copy() {
    use revue::widget::feedback::PopoverTrigger;
    let trigger1 = PopoverTrigger::Focus;
    let trigger2 = trigger1;
    assert_eq!(trigger2, PopoverTrigger::Focus);
    assert_eq!(trigger1, PopoverTrigger::Focus);
}

#[test]
fn test_popover_trigger_partial_eq() {
    use revue::widget::feedback::PopoverTrigger;
    assert_eq!(PopoverTrigger::Click, PopoverTrigger::Click);
    assert_eq!(PopoverTrigger::Hover, PopoverTrigger::Hover);
    assert_eq!(PopoverTrigger::Focus, PopoverTrigger::Focus);
    assert_eq!(PopoverTrigger::Manual, PopoverTrigger::Manual);

    assert_ne!(PopoverTrigger::Click, PopoverTrigger::Hover);
    assert_ne!(PopoverTrigger::Focus, PopoverTrigger::Manual);
}

#[test]
fn test_popover_trigger_all_variants() {
    use revue::widget::feedback::PopoverTrigger;
    let triggers = [
        PopoverTrigger::Click,
        PopoverTrigger::Hover,
        PopoverTrigger::Focus,
        PopoverTrigger::Manual,
    ];

    for (i, trigger1) in triggers.iter().enumerate() {
        for (j, trigger2) in triggers.iter().enumerate() {
            if i == j {
                assert_eq!(trigger1, trigger2);
            } else {
                assert_ne!(trigger1, trigger2);
            }
        }
    }
}

// =========================================================================
// PopoverArrow enum tests
// =========================================================================

#[test]
fn test_popover_arrow_default() {
    use revue::widget::feedback::PopoverArrow;
    assert_eq!(PopoverArrow::default(), PopoverArrow::None);
}

#[test]
fn test_popover_arrow_clone() {
    use revue::widget::feedback::PopoverArrow;
    let arrow1 = PopoverArrow::Unicode;
    let arrow2 = arrow1.clone();
    assert_eq!(arrow1, arrow2);
}

#[test]
fn test_popover_arrow_copy() {
    use revue::widget::feedback::PopoverArrow;
    let arrow1 = PopoverArrow::Simple;
    let arrow2 = arrow1;
    assert_eq!(arrow2, PopoverArrow::Simple);
    assert_eq!(arrow1, PopoverArrow::Simple);
}

#[test]
fn test_popover_arrow_partial_eq() {
    use revue::widget::feedback::PopoverArrow;
    assert_eq!(PopoverArrow::None, PopoverArrow::None);
    assert_eq!(PopoverArrow::Simple, PopoverArrow::Simple);
    assert_eq!(PopoverArrow::Unicode, PopoverArrow::Unicode);

    assert_ne!(PopoverArrow::None, PopoverArrow::Simple);
    assert_ne!(PopoverArrow::Simple, PopoverArrow::Unicode);
    assert_ne!(PopoverArrow::Unicode, PopoverArrow::None);
}

#[test]
fn test_popover_arrow_all_variants() {
    use revue::widget::feedback::PopoverArrow;
    let arrows = [
        PopoverArrow::None,
        PopoverArrow::Simple,
        PopoverArrow::Unicode,
    ];

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
fn test_popover_arrow_chars_all_combinations() {
    use revue::widget::feedback::{PopoverArrow, PopoverPosition};
    let arrows = [
        PopoverArrow::None,
        PopoverArrow::Simple,
        PopoverArrow::Unicode,
    ];
    let positions = [
        PopoverPosition::Top,
        PopoverPosition::Bottom,
        PopoverPosition::Left,
        PopoverPosition::Right,
        PopoverPosition::Auto,
    ];

    for arrow in arrows {
        for pos in positions {
            let ch = arrow.chars(pos);
            assert!(ch.len_utf8() >= 1);
        }
    }
}

// =========================================================================
// PopoverStyle enum tests
// =========================================================================

#[test]
fn test_popover_style_default() {
    use revue::widget::feedback::PopoverStyle;
    assert_eq!(PopoverStyle::default(), PopoverStyle::Default);
}

#[test]
fn test_popover_style_clone() {
    use revue::widget::feedback::PopoverStyle;
    let style1 = PopoverStyle::Rounded;
    let style2 = style1.clone();
    assert_eq!(style1, style2);
}

#[test]
fn test_popover_style_copy() {
    use revue::widget::feedback::PopoverStyle;
    let style1 = PopoverStyle::Minimal;
    let style2 = style1;
    assert_eq!(style2, PopoverStyle::Minimal);
    assert_eq!(style1, PopoverStyle::Minimal);
}

#[test]
fn test_popover_style_partial_eq() {
    use revue::widget::feedback::PopoverStyle;
    assert_eq!(PopoverStyle::Default, PopoverStyle::Default);
    assert_eq!(PopoverStyle::Rounded, PopoverStyle::Rounded);
    assert_eq!(PopoverStyle::Minimal, PopoverStyle::Minimal);
    assert_eq!(PopoverStyle::Elevated, PopoverStyle::Elevated);

    assert_ne!(PopoverStyle::Default, PopoverStyle::Rounded);
    assert_ne!(PopoverStyle::Minimal, PopoverStyle::Elevated);
}

#[test]
fn test_popover_style_all_variants() {
    use revue::widget::feedback::PopoverStyle;
    let styles = [
        PopoverStyle::Default,
        PopoverStyle::Rounded,
        PopoverStyle::Minimal,
        PopoverStyle::Elevated,
    ];

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
fn test_popover_style_colors_all_variants() {
    use revue::widget::feedback::PopoverStyle;
    let styles = [
        PopoverStyle::Default,
        PopoverStyle::Rounded,
        PopoverStyle::Minimal,
        PopoverStyle::Elevated,
    ];

    for style in styles {
        let (fg, bg, border) = style.colors();
        // u8 values are always valid 0-255, just verify colors exist
        let _ = (
            fg.r, fg.g, fg.b, bg.r, bg.g, bg.b, border.r, border.g, border.b,
        );
    }
}

#[test]
fn test_popover_style_border_chars_all_variants() {
    use revue::widget::feedback::PopoverStyle;
    let styles = [
        PopoverStyle::Default,
        PopoverStyle::Rounded,
        PopoverStyle::Minimal,
        PopoverStyle::Elevated,
    ];

    for style in styles {
        let border = style.border_chars();
        if matches!(style, PopoverStyle::Minimal) {
            assert!(border.is_none());
        } else {
            assert!(border.is_some());
        }
    }
}

// =========================================================================
// Popover builder method tests
// =========================================================================

#[test]
fn test_popover_content() {
    let p = Popover::new("Original").content("Updated");
    assert_eq!(p.content, "Updated");
}

#[test]
fn test_popover_anchor_builder() {
    let p = Popover::new("Test").anchor(100, 200);
    assert_eq!(p.anchor, (100, 200));
}

#[test]
fn test_popover_position_builder() {
    use revue::widget::feedback::PopoverPosition;
    let p = Popover::new("Test").position(PopoverPosition::Top);
    assert_eq!(p.position, PopoverPosition::Top);
}

#[test]
fn test_popover_trigger_builder() {
    use revue::widget::feedback::PopoverTrigger;
    let p = Popover::new("Test").trigger(PopoverTrigger::Manual);
    assert_eq!(p.trigger, PopoverTrigger::Manual);
}

#[test]
fn test_popover_style_builder() {
    use revue::widget::feedback::PopoverStyle;
    let p = Popover::new("Test").popover_style(PopoverStyle::Elevated);
    assert_eq!(p.popover_style, PopoverStyle::Elevated);
}

#[test]
fn test_popover_arrow_builder() {
    use revue::widget::feedback::PopoverArrow;
    let p = Popover::new("Test").arrow(PopoverArrow::Simple);
    assert_eq!(p.arrow, PopoverArrow::Simple);
}

#[test]
fn test_popover_open_builder() {
    let p = Popover::new("Test").open(true);
    assert!(p.open);
}

#[test]
fn test_popover_close_on_escape_builder() {
    let p = Popover::new("Test").close_on_escape(false);
    assert!(!p.close_on_escape);
}

#[test]
fn test_popover_close_on_click_outside_builder() {
    let p = Popover::new("Test").close_on_click_outside(false);
    assert!(!p.close_on_click_outside);
}

#[test]
fn test_popover_title_builder() {
    let p = Popover::new("Test").title("My Title");
    assert_eq!(p.title, Some("My Title".to_string()));
}

#[test]
fn test_popover_max_width_builder() {
    let p = Popover::new("Test").max_width(100);
    assert_eq!(p.max_width, 100);
}

#[test]
fn test_popover_border_color_builder() {
    use revue::style::Color;
    let p = Popover::new("Test").border_color(Color::YELLOW);
    assert_eq!(p.border_color, Some(Color::YELLOW));
}

// =========================================================================
// Popover builder chain tests
// =========================================================================

#[test]
fn test_popover_builder_chain() {
    use revue::widget::feedback::{PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};
    let p = Popover::new("Chain test")
        .anchor(50, 25)
        .position(PopoverPosition::Right)
        .trigger(PopoverTrigger::Hover)
        .popover_style(PopoverStyle::Rounded)
        .arrow(PopoverArrow::Unicode)
        .title("Chain Title")
        .max_width(60)
        .close_on_escape(true)
        .close_on_click_outside(true);

    assert_eq!(p.content, "Chain test");
    assert_eq!(p.anchor, (50, 25));
    assert_eq!(p.position, PopoverPosition::Right);
    assert_eq!(p.trigger, PopoverTrigger::Hover);
    assert_eq!(p.popover_style, PopoverStyle::Rounded);
    assert_eq!(p.arrow, PopoverArrow::Unicode);
    assert_eq!(p.title, Some("Chain Title".to_string()));
    assert_eq!(p.max_width, 60);
    assert!(p.close_on_escape);
    assert!(p.close_on_click_outside);
}

// =========================================================================
// Popover render tests
// =========================================================================

#[test]
fn test_popover_render_hidden() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let p = Popover::new("Test").anchor(10, 5);
    p.render(&mut ctx);
    // Hidden popover shouldn't render anything special
}

#[test]
fn test_popover_render_visible() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    use revue::widget::feedback::PopoverStyle;
    let p = Popover::new("Visible content")
        .anchor(10, 5)
        .open(true)
        .popover_style(PopoverStyle::Default);

    p.render(&mut ctx);
    // Smoke test - should render without panic
}

#[test]
fn test_popover_render_with_title() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

    let p = Popover::new("Content")
        .title("Title")
        .anchor(10, 5)
        .open(true);

    p.render(&mut ctx);
}

#[test]
fn test_popover_render_all_styles() {
    use revue::widget::feedback::PopoverStyle;
    let styles = [
        PopoverStyle::Default,
        PopoverStyle::Rounded,
        PopoverStyle::Minimal,
        PopoverStyle::Elevated,
    ];

    for style in styles {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

        let p = Popover::new("Test")
            .anchor(10, 5)
            .open(true)
            .popover_style(style);

        p.render(&mut ctx);
    }
}

#[test]
fn test_popover_render_all_positions() {
    use revue::widget::feedback::PopoverPosition;
    let positions = [
        PopoverPosition::Top,
        PopoverPosition::Bottom,
        PopoverPosition::Left,
        PopoverPosition::Right,
        PopoverPosition::Auto,
    ];

    for position in positions {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

        let p = Popover::new("Test")
            .anchor(40, 12)
            .open(true)
            .position(position);

        p.render(&mut ctx);
    }
}

#[test]
fn test_popover_render_with_arrow() {
    use revue::widget::feedback::PopoverArrow;
    let arrows = [
        PopoverArrow::None,
        PopoverArrow::Simple,
        PopoverArrow::Unicode,
    ];

    for arrow in arrows {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);

        use revue::widget::feedback::PopoverPosition;
        let p = Popover::new("Test")
            .anchor(40, 12)
            .open(true)
            .arrow(arrow)
            .position(PopoverPosition::Top);

        p.render(&mut ctx);
    }
}

// =========================================================================
// Popover Default trait tests
// =========================================================================

#[test]
fn test_popover_default_trait() {
    use revue::widget::feedback::{PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};
    let p = Popover::default();
    assert_eq!(p.content, "");
    assert_eq!(p.anchor, (0, 0));
    assert_eq!(p.position, PopoverPosition::Bottom);
    assert_eq!(p.trigger, PopoverTrigger::Click);
    assert_eq!(p.popover_style, PopoverStyle::Default);
    assert_eq!(p.arrow, PopoverArrow::None);
    assert!(!p.open);
    assert!(p.close_on_escape);
    assert!(p.close_on_click_outside);
    assert!(p.title.is_none());
    assert_eq!(p.max_width, 40);
    assert!(p.border_color.is_none());
}

#[test]
fn test_popover_default_vs_new_empty() {
    let default_p = Popover::default();
    let new_p = Popover::new("");

    assert_eq!(default_p.content, new_p.content);
    assert_eq!(default_p.position, new_p.position);
    assert_eq!(default_p.open, new_p.open);
}
