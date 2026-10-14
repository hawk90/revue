//! The modal's unit tests, moved out of `mod.rs`, `types.rs`, `render.rs` and
//! `key_handling.rs` with their bodies unchanged. The first group reads the
//! private fields.

use super::*;
use crate::event::FocusManager;

#[test]
fn test_modal_new() {
    let m = Modal::new();
    assert!(!m.is_visible());
    assert!(m.title.is_empty());
    assert!(m.content.is_empty());
    assert!(m.buttons.is_empty());
}

#[test]
fn test_modal_builder() {
    let m = Modal::new()
        .title("Test")
        .content(
            "Hello
World",
        )
        .ok_cancel();

    assert_eq!(m.title, "Test");
    assert_eq!(m.content.len(), 2);
    assert_eq!(m.buttons.len(), 2);
}

#[test]
fn test_modal_visibility() {
    let mut m = Modal::new();
    assert!(!m.is_visible());

    m.show();
    assert!(m.is_visible());

    m.hide();
    assert!(!m.is_visible());

    m.toggle();
    assert!(m.is_visible());
}

#[test]
fn test_modal_presets() {
    let alert = Modal::alert("Title", "Message");
    assert_eq!(alert.title, "Title");
    assert_eq!(alert.buttons.len(), 1);

    let confirm = Modal::confirm("Title", "Question?");
    assert_eq!(confirm.buttons.len(), 2);

    let error = Modal::error("Something went wrong");
    assert_eq!(error.title, "Error");
}

#[test]
fn test_modal_helper() {
    let m = modal().title("Quick").ok();

    assert_eq!(m.title, "Quick");
}

#[test]
fn test_modal_with_body() {
    use crate::widget::Text;

    let m = Modal::new()
        .title("Form")
        .body(Text::new("Custom content"))
        .height(10)
        .ok();

    assert!(m.body.is_some());
    assert_eq!(m.height, Some(10));
}

// =========================================================================
// Modal builder method tests for edge cases
// =========================================================================

#[test]
fn test_modal_empty_title() {
    let m = Modal::new().title("");
    assert_eq!(m.title, "");
}

#[test]
fn test_modal_empty_content() {
    let m = Modal::new().content("");
    assert!(m.content.is_empty());
}

#[test]
fn test_modal_content_with_multiline() {
    let m = Modal::new().content("Line 1\nLine 2\nLine 3");
    assert_eq!(m.content.len(), 3);
    assert_eq!(m.content[0], "Line 1");
    assert_eq!(m.content[1], "Line 2");
    assert_eq!(m.content[2], "Line 3");
}

#[test]
fn test_modal_line_multiple() {
    let m = Modal::new().line("Line 1").line("Line 2").line("Line 3");
    assert_eq!(m.content.len(), 3);
}

#[test]
fn test_modal_buttons_empty() {
    let m = Modal::new().buttons(vec![]);
    assert!(m.buttons.is_empty());
}

#[test]
fn test_modal_width_zero() {
    let m = Modal::new().width(0);
    assert_eq!(m.width, 0);
}

#[test]
fn test_modal_height_zero() {
    let m = Modal::new().height(0);
    assert_eq!(m.height, Some(0));
}

#[test]
fn test_modal_title_colors() {
    let m = Modal::new().title_fg(Color::CYAN);
    assert_eq!(m.title_fg, Some(Color::CYAN));
}

#[test]
fn test_modal_border_colors() {
    let m = Modal::new().border_fg(Color::MAGENTA);
    assert_eq!(m.border_fg, Some(Color::MAGENTA));
}

// =========================================================================
// Modal builder chain tests
// =========================================================================

#[test]
fn test_modal_builder_chain_full() {
    let m = Modal::new()
        .title("Chain Title")
        .content("Chain content")
        .width(60)
        .height(10)
        .title_fg(Color::YELLOW)
        .border_fg(Color::GREEN);

    assert_eq!(m.title, "Chain Title");
    assert_eq!(m.content.len(), 1);
    assert_eq!(m.content[0], "Chain content");
    assert_eq!(m.width, 60);
    assert_eq!(m.height, Some(10));
    assert_eq!(m.title_fg, Some(Color::YELLOW));
    assert_eq!(m.border_fg, Some(Color::GREEN));
}

#[test]
fn test_modal_buttons_builder_chain() {
    let buttons = vec![
        ModalButton::new("One"),
        ModalButton::primary("Two"),
        ModalButton::danger("Three"),
    ];
    let m = Modal::new().buttons(buttons.clone());

    assert_eq!(m.buttons.len(), 3);
    assert_eq!(m.buttons[0].label, "One");
    assert_eq!(m.buttons[1].label, "Two");
    assert_eq!(m.buttons[2].label, "Three");
}

// =========================================================================
// Focus trap integration tests
// =========================================================================

#[test]
fn test_modal_show_with_focus_trap() {
    let mut fm = FocusManager::new();
    fm.register(1); // background widget
    fm.register(2); // background widget
    fm.register(10); // modal button 1
    fm.register(11); // modal button 2
    fm.focus(1);

    let mut m = Modal::new().ok_cancel();
    m.show_with_focus_trap(&mut fm, 100, &[10, 11]);

    assert!(m.is_visible());
    assert!(m.has_focus_trap());
    assert!(fm.is_trapped());
    // Focus should be on first trapped child
    assert_eq!(fm.current(), Some(10));
}

#[test]
fn test_modal_hide_with_focus_restore() {
    let mut fm = FocusManager::new();
    fm.register(1);
    fm.register(2);
    fm.register(10);
    fm.register(11);
    fm.focus(1);

    let mut m = Modal::new().ok_cancel();
    m.show_with_focus_trap(&mut fm, 100, &[10, 11]);
    assert_eq!(fm.current(), Some(10));

    m.hide_with_focus_restore(&mut fm);
    assert!(!m.is_visible());
    assert!(!m.has_focus_trap());
    assert!(!fm.is_trapped());
    // Focus should be restored to widget 1
    assert_eq!(fm.current(), Some(1));
}

#[test]
fn test_modal_no_focus_trap_by_default() {
    let m = Modal::new().ok();
    assert!(!m.has_focus_trap());
}

mod types {
    use super::super::{ModalButton, ModalButtonStyle};

    #[test]
    fn test_modal_button_styles() {
        let btn = ModalButton::new("Test");
        assert!(matches!(btn.style, ModalButtonStyle::Default));

        let btn = ModalButton::primary("OK");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));

        let btn = ModalButton::danger("Delete");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    // =========================================================================
    // ModalButtonStyle enum tests
    // =========================================================================

    #[test]
    fn test_modal_button_style_default() {
        let style = ModalButtonStyle::default();
        assert!(matches!(style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_style_clone() {
        let style1 = ModalButtonStyle::Primary;
        let style2 = style1;
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_modal_button_style_copy() {
        let style1 = ModalButtonStyle::Danger;
        let style2 = style1;
        assert_eq!(style2, ModalButtonStyle::Danger);
        // style1 is still valid because of Copy
        assert_eq!(style1, ModalButtonStyle::Danger);
    }

    #[test]
    fn test_modal_button_style_partial_eq() {
        assert_eq!(ModalButtonStyle::Default, ModalButtonStyle::Default);
        assert_eq!(ModalButtonStyle::Primary, ModalButtonStyle::Primary);
        assert_eq!(ModalButtonStyle::Danger, ModalButtonStyle::Danger);

        assert_ne!(ModalButtonStyle::Default, ModalButtonStyle::Primary);
        assert_ne!(ModalButtonStyle::Primary, ModalButtonStyle::Danger);
        assert_ne!(ModalButtonStyle::Danger, ModalButtonStyle::Default);
    }

    #[test]
    fn test_modal_button_style_all_variants() {
        let styles = [
            ModalButtonStyle::Default,
            ModalButtonStyle::Primary,
            ModalButtonStyle::Danger,
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

    // =========================================================================
    // ModalButton Clone trait tests
    // =========================================================================

    #[test]
    fn test_modal_button_clone() {
        let btn1 = ModalButton::new("Test").style(ModalButtonStyle::Primary);
        let btn2 = btn1.clone();

        assert_eq!(btn1.label, btn2.label);
        assert_eq!(btn1.style, btn2.style);
    }

    // =========================================================================
    // ModalButton builder method tests
    // =========================================================================

    #[test]
    fn test_modal_button_new_with_string() {
        let label = String::from("Owned Label");
        let btn = ModalButton::new(label);
        assert_eq!(btn.label, "Owned Label");
        assert!(matches!(btn.style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_new_with_str() {
        let btn = ModalButton::new("Test Label");
        assert_eq!(btn.label, "Test Label");
        assert!(matches!(btn.style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_empty_label() {
        let btn = ModalButton::new("");
        assert_eq!(btn.label, "");
    }

    #[test]
    fn test_modal_button_primary_with_string() {
        let label = String::from("Submit");
        let btn = ModalButton::primary(label);
        assert_eq!(btn.label, "Submit");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));
    }

    #[test]
    fn test_modal_button_primary_with_str() {
        let btn = ModalButton::primary("OK");
        assert_eq!(btn.label, "OK");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));
    }

    #[test]
    fn test_modal_button_danger_with_string() {
        let label = String::from("Delete");
        let btn = ModalButton::danger(label);
        assert_eq!(btn.label, "Delete");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    #[test]
    fn test_modal_button_danger_with_str() {
        let btn = ModalButton::danger("Cancel");
        assert_eq!(btn.label, "Cancel");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    #[test]
    fn test_modal_button_all_distinct() {
        let default_btn = ModalButton::new("Default");
        let primary_btn = ModalButton::primary("Primary");
        let danger_btn = ModalButton::danger("Danger");

        assert!(matches!(default_btn.style, ModalButtonStyle::Default));
        assert!(matches!(primary_btn.style, ModalButtonStyle::Primary));
        assert!(matches!(danger_btn.style, ModalButtonStyle::Danger));
    }
}

mod render {
    use super::super::Modal;
    use crate::widget::traits::{RenderContext, View};

    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_modal_render_hidden() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let m = Modal::new().title("Test");
        m.render(&mut ctx);

        // Hidden modal shouldn't render anything special
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_modal_render_visible() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new().title("Test Dialog").content("Hello").ok();
        m.show();
        m.render(&mut ctx);

        // Modal should render centered - check for border characters
        // The exact position depends on centering calculation
        let center_x = (80 - 40) / 2;
        let center_y = (24 - m.required_height()) / 2;

        assert_eq!(buffer.get(center_x, center_y).unwrap().symbol, '┌');
    }

    #[test]
    fn test_modal_body_render() {
        use crate::widget::Text;

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new()
            .title("Body Test")
            .body(Text::new("Widget content"))
            .width(50)
            .height(12)
            .ok();
        m.show();
        m.render(&mut ctx);

        // Modal with body should render
        let center_x = (80 - 50) / 2;
        let center_y = (24 - 12) / 2;
        assert_eq!(buffer.get(center_x, center_y).unwrap().symbol, '┌');
    }

    #[test]
    fn test_modal_render_small_area_no_panic() {
        // Rendering in very small areas doesn't panic (issue #154), and a
        // modal with no room draws nothing: under six columns there is no
        // box, and under three rows no height
        for (w, h) in [
            (0, 24),
            (1, 24),
            (2, 24),
            (5, 24),
            (80, 0),
            (80, 1),
            (80, 2),
            (0, 0),
        ] {
            let mut buffer = Buffer::new(80, 24);
            let area = Rect::new(0, 0, w, h);
            let mut ctx = RenderContext::new(&mut buffer, area);
            let mut m = Modal::new().title("Test").ok();
            m.show();
            m.render(&mut ctx);
            let blank = (0..24).all(|y| (0..80).all(|x| buffer.get(x, y).unwrap().symbol == ' '));
            assert!(blank, "{w}x{h}");
        }
    }

    #[test]
    fn test_modal_render_text_content_in_short_area() {
        // With text content the line loop compares against the bottom of
        // the modal, which used to underflow for modals under 2 rows
        let text = |buffer: &Buffer| -> String {
            (0..24)
                .map(|y| {
                    (0..80)
                        .map(|x| buffer.get(x, y).unwrap().symbol)
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        for height in 0..=4 {
            let mut buffer = Buffer::new(80, 24);
            let area = Rect::new(0, 0, 80, height);
            let mut ctx = RenderContext::new(&mut buffer, area);
            let mut m = Modal::new().content("Line one\nLine two").ok();
            m.show();
            m.render(&mut ctx);
            // At four rows only an empty two-row box fits; below that, nothing
            let drawn = text(&buffer);
            assert!(!drawn.contains("Line"), "{height}");
            assert_eq!(drawn.contains('┌'), height == 4, "{height}");
            assert!((height..24).all(|y| (0..80).all(|x| buffer.get(x, y).unwrap().symbol == ' ')));

            let mut buffer = Buffer::new(80, 24);
            let mut ctx = RenderContext::new(&mut buffer, area);
            let mut m = Modal::new().title("Test").content("Body").ok();
            m.show();
            m.render(&mut ctx);
            // At three and four rows the title is drawn outside its box (the
            // modal issue from the #930 audit), so only the smaller areas are
            // checked
            if height <= 2 {
                assert!(text(&buffer).trim().is_empty(), "{height}");
            }
        }
    }

    #[test]
    fn test_modal_render_width_2_border() {
        // A 2-wide modal needs a 6-wide area (two columns of margin each
        // side); in a 4-wide one there is no room and nothing is drawn
        let rows = |area_w: u16| -> Vec<String> {
            let mut buffer = Buffer::new(10, 10);
            let area = Rect::new(0, 0, area_w, 10);
            let mut ctx = RenderContext::new(&mut buffer, area);
            let mut m = Modal::new().title("X").width(2).height(4);
            m.show();
            m.render(&mut ctx);
            (0..10)
                .map(|y| {
                    (0..10)
                        .map(|x| buffer.get(x, y).unwrap().symbol)
                        .collect::<String>()
                })
                .collect()
        };
        assert!(rows(4).iter().all(|r| r.trim().is_empty()));
        let six = rows(6);
        assert_eq!(
            &six[3..7],
            ["  ┌┐      ", "  ││      ", "  ││      ", "  └┘      "]
        );
        assert!(six[..3]
            .iter()
            .chain(&six[7..])
            .all(|r| r.trim().is_empty()));
    }

    #[test]
    fn buttons_center_by_display_width() {
        // A 20-wide modal in a 24-wide area starts at x = 2; the 8-column
        // button "[ 확인 ]" centers at 2 + (20 - 8) / 2 = 8.
        let mut m = Modal::new()
            .width(20)
            .buttons(vec![super::super::ModalButton::new("확인")]);
        m.show();
        let mut buffer = Buffer::new(24, 10);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 24, 10));
        m.render(&mut ctx);
        let start = (0..10)
            .find_map(|y| (0..24).find(|&x| buffer.get(x, y).unwrap().symbol == '['))
            .expect("no button drawn");
        assert_eq!(start, 8);
    }
}

mod key_handling {
    use super::super::Modal;
    use crate::event::FocusManager;

    #[test]
    fn test_modal_button_navigation() {
        let mut m = Modal::new().ok_cancel();

        assert_eq!(m.selected_button(), 0);

        m.next_button();
        assert_eq!(m.selected_button(), 1);

        m.next_button(); // Wraps around
        assert_eq!(m.selected_button(), 0);

        m.prev_button(); // Wraps around
        assert_eq!(m.selected_button(), 1);
    }

    #[test]
    fn test_modal_handle_key() {
        use crate::event::Key;

        let mut m = Modal::new().yes_no();
        m.show();

        // Navigate buttons
        m.handle_key(&Key::Right);
        assert_eq!(m.selected_button(), 1);

        m.handle_key(&Key::Left);
        assert_eq!(m.selected_button(), 0);

        // Confirm selection
        let result = m.handle_key(&Key::Enter);
        assert_eq!(result, Some(0));

        // Escape closes
        m.handle_key(&Key::Escape);
        assert!(!m.is_visible());
    }

    #[test]
    fn test_modal_selected_button_initial() {
        let m = Modal::new();
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_next_button_empty() {
        let mut m = Modal::new();
        m.next_button(); // Should not panic
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_prev_button_empty() {
        let mut m = Modal::new();
        m.prev_button(); // Should not panic
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_handle_key_no_buttons() {
        use crate::event::Key;

        let mut m = Modal::new();
        let result = m.handle_key(&Key::Enter);
        assert_eq!(result, None);
    }

    #[test]
    fn test_modal_handle_key_unknown() {
        use crate::event::Key;

        let mut m = Modal::new().ok();
        let result = m.handle_key(&Key::Char('x'));
        assert_eq!(result, None);
    }

    #[test]
    fn test_modal_handle_key_with_focus_escape() {
        use crate::event::Key;

        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(10);
        fm.register(11);
        fm.focus(1);

        let mut m = Modal::new().ok_cancel();
        m.show_with_focus_trap(&mut fm, 100, &[10, 11]);

        // Escape should hide modal and release trap
        m.handle_key_with_focus(&Key::Escape, &mut fm);
        assert!(!m.is_visible());
        assert!(!fm.is_trapped());
        assert_eq!(fm.current(), Some(1));
    }

    #[test]
    fn test_modal_handle_key_with_focus_confirm() {
        use crate::event::Key;

        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(10);
        fm.register(11);
        fm.focus(1);

        let mut m = Modal::new().ok_cancel();
        m.show_with_focus_trap(&mut fm, 100, &[10, 11]);

        // Enter confirms and releases trap
        let result = m.handle_key_with_focus(&Key::Enter, &mut fm);
        assert_eq!(result, Some(0));
        assert!(!fm.is_trapped());
    }

    #[test]
    fn test_modal_focus_trap_tab_does_not_release() {
        use crate::event::Key;

        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(10);
        fm.register(11);
        fm.focus(1);

        let mut m = Modal::new().ok_cancel();
        m.show_with_focus_trap(&mut fm, 100, &[10, 11]);

        // Tab should navigate buttons but keep trap active
        m.handle_key_with_focus(&Key::Tab, &mut fm);
        assert!(m.has_focus_trap());
        assert!(fm.is_trapped());
    }
}
