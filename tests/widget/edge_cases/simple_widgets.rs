//! Edge case tests for simple widgets
//!
//! Tests edge cases for basic interactive widgets:
//! - Button: empty label, very long labels, special characters
//! - Checkbox: toggle edge cases, state transitions
//! - RadioGroup: selection edge cases, empty groups
//! - Switch: toggle edge cases
//! - Badge: empty text, very long text
//! - Divider: zero width, various orientations

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Badge, Button, Checkbox, Divider, RadioGroup, Switch};

/// Test Button edge cases
mod button_edge_cases {
    use super::*;

    #[test]
    fn test_button_with_empty_label() {
        let button = Button::new("");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic with empty label
        button.render(&mut ctx);
    }

    #[test]
    fn test_button_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let button = Button::new(&long_label);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should clip to buffer width
        button.render(&mut ctx);
    }

    #[test]
    fn test_button_with_newlines_in_label() {
        let button = Button::new("Line 1\nLine 2");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        button.render(&mut ctx);
    }

    #[test]
    fn test_button_with_special_chars() {
        let button = Button::new("Test\t\r\n");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        button.render(&mut ctx);
    }

    #[test]
    fn test_button_with_unicode_emoji() {
        let button = Button::new("🎉 Click Me! 🎉");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        button.render(&mut ctx);
    }

    #[test]
    fn test_button_all_variants() {
        let variants = [
            Button::new("Test"),
            Button::primary("Test"),
            Button::danger("Test"),
            Button::ghost("Test"),
            Button::success("Test"),
        ];

        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);

        for button in variants {
            let mut ctx = RenderContext::new(&mut buffer, area);
            button.render(&mut ctx);
        }
    }

    #[test]
    fn test_button_with_icon() {
        let button = Button::new("Save").icon('💾');
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        button.render(&mut ctx);
    }

    #[test]
    fn test_button_with_zero_width_buffer() {
        let button = Button::new("Test");
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        button.render(&mut ctx);
    }

    #[test]
    fn test_button_disabled_with_empty_label() {
        let button = Button::new("").disabled(true);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        button.render(&mut ctx);
    }
}

/// Test Checkbox edge cases
mod checkbox_edge_cases {
    use super::*;

    #[test]
    fn test_checkbox_with_empty_label() {
        let checkbox = Checkbox::new("");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
    }

    #[test]
    fn test_checkbox_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let checkbox = Checkbox::new(&long_label);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
    }

    #[test]
    fn test_checkbox_toggle_multiple_times() {
        let mut checkbox = Checkbox::new("Test");

        // Toggle many times using builder pattern
        for i in 0..100 {
            checkbox = checkbox.checked(i % 2 == 1);
        }

        // 99 is odd, so checked(true)
        assert_eq!(checkbox.is_checked(), true);
    }

    #[test]
    fn test_checkbox_with_unicode_label() {
        let checkbox = Checkbox::new("✅ 체크박스");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
    }

    #[test]
    fn test_checkbox_disabled_checked() {
        let checkbox = Checkbox::new("Test").checked(true).disabled(true);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        checkbox.render(&mut ctx);
    }
}

/// Test RadioGroup edge cases
mod radio_group_edge_cases {
    use super::*;

    #[test]
    fn test_radio_group_empty() {
        let radio = RadioGroup::new(std::iter::empty::<&str>());
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should handle empty options gracefully
        radio.render(&mut ctx);
    }

    #[test]
    fn test_radio_group_with_empty_option() {
        let radio = RadioGroup::new([""]);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        radio.render(&mut ctx);
    }

    #[test]
    fn test_radio_group_with_very_long_options() {
        let long_option = "A".repeat(1000);
        let radio = RadioGroup::new([&long_option, "Short", &long_option]);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        radio.render(&mut ctx);
    }

    #[test]
    fn test_radio_group_with_unicode() {
        let radio = RadioGroup::new(["옵션 1", "옵션 2", "옵션 3"]);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        radio.render(&mut ctx);
    }

    #[test]
    fn test_radio_group_selection_edge_cases() {
        let radio = RadioGroup::new(["A", "B", "C"]).selected(100); // Out of bounds
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should handle out of bounds selection gracefully
        radio.render(&mut ctx);
    }

    #[test]
    fn test_radio_group_many_options() {
        let options: Vec<String> = (0..100).map(|i| format!("Option {}", i)).collect();
        let radio = RadioGroup::new(options);
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        radio.render(&mut ctx);
    }
}

/// Test Switch edge cases
mod switch_edge_cases {
    use super::*;

    #[test]
    fn test_switch_no_label() {
        let switch = Switch::new();
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
    }

    #[test]
    fn test_switch_with_empty_label() {
        let switch = Switch::new().label("");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
    }

    #[test]
    fn test_switch_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let switch = Switch::new().label(&long_label);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
    }

    #[test]
    fn test_switch_toggle_multiple_times() {
        let mut switch = Switch::new();

        // Toggle many times
        for i in 0..100 {
            switch = switch.on(i % 2 == 1);
        }

        // 99 is odd, so on(true)
        assert_eq!(switch.is_on(), true);
    }

    #[test]
    fn test_switch_with_unicode() {
        let switch = Switch::new().label("전환 ⚡");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
    }

    #[test]
    fn test_switch_disabled_on() {
        let switch = Switch::new().on(true).disabled(true);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
    }

    #[test]
    fn test_switch_with_zero_width_buffer() {
        let switch = Switch::new();
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        switch.render(&mut ctx);
    }
}

/// Test Badge edge cases
mod badge_edge_cases {
    use super::*;

    #[test]
    fn test_badge_with_empty_content() {
        let badge = Badge::new("");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        badge.render(&mut ctx);
    }

    #[test]
    fn test_badge_with_very_long_content() {
        let long_content = "99".repeat(100);
        let badge = Badge::new(&long_content);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        badge.render(&mut ctx);
    }

    #[test]
    fn test_badge_with_unicode() {
        let badge = Badge::new("🏷️ 뱃지");
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        badge.render(&mut ctx);
    }

    #[test]
    fn test_badge_with_zero_width_buffer() {
        let badge = Badge::new("1");
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        badge.render(&mut ctx);
    }

    #[test]
    fn test_badge_dot_variant() {
        let badge = Badge::dot();
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        badge.render(&mut ctx);
    }
}

/// Test Divider edge cases
mod divider_edge_cases {
    use super::*;

    #[test]
    fn test_divider_horizontal_zero_width() {
        let divider = Divider::new();
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_vertical_zero_height() {
        let divider = Divider::vertical();
        let mut buffer = Buffer::new(10, 0);
        let area = Rect::new(0, 0, 10, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_both_zero() {
        let divider = Divider::new();
        let mut buffer = Buffer::new(0, 0);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_with_label() {
        let divider = Divider::new().label("Section");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_with_empty_label() {
        let divider = Divider::new().label("");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_with_very_long_label() {
        let long_label = "A".repeat(1000);
        let divider = Divider::new().label(&long_label);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }

    #[test]
    fn test_divider_with_unicode_label() {
        let divider = Divider::new().label("구분선 🎨");
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        divider.render(&mut ctx);
    }
}

/// Test widget state edge cases
mod widget_state_edge_cases {
    use super::*;

    #[test]
    fn test_button_all_states() {
        let states = [
            Button::new("Test"),
            Button::new("Test").focused(true),
            Button::new("Test").disabled(true),
        ];

        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);

        for button in states {
            let mut ctx = RenderContext::new(&mut buffer, area);
            button.render(&mut ctx);
        }
    }

    #[test]
    fn test_checkbox_all_states() {
        let states = [
            Checkbox::new("Test"),
            Checkbox::new("Test").checked(true),
            Checkbox::new("Test").disabled(true),
            Checkbox::new("Test").focused(true),
            Checkbox::new("Test").checked(true).disabled(true),
        ];

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);

        for checkbox in states {
            let mut ctx = RenderContext::new(&mut buffer, area);
            checkbox.render(&mut ctx);
        }
    }

    #[test]
    fn test_switch_all_states() {
        let states = [
            Switch::new(),
            Switch::new().on(true),
            Switch::new().disabled(true),
            Switch::new().focused(true),
            Switch::new().on(true).disabled(true),
        ];

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);

        for switch in states {
            let mut ctx = RenderContext::new(&mut buffer, area);
            switch.render(&mut ctx);
        }
    }
}
