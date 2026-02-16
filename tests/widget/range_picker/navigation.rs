//! Range picker navigation and key handling tests
//! Extracted from src/widget/range_picker/navigation.rs

use revue::event::Key;
use revue::widget::data::calendar::{Date, days_in_month};
use revue::widget::range_picker::{PresetRange, RangeFocus, RangePicker};

// =========================================================================
// Focus navigation tests
// =========================================================================

#[test]
fn test_next_focus_start_to_end() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Start;
    picker.next_focus();
    assert_eq!(picker.focus, RangeFocus::End);
}

#[test]
fn test_next_focus_end_to_presets() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::End;
    picker.show_presets = true;
    picker.next_focus();
    assert_eq!(picker.focus, RangeFocus::Presets);
}

#[test]
fn test_next_focus_end_to_start_no_presets() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::End;
    picker.show_presets = false;
    picker.next_focus();
    assert_eq!(picker.focus, RangeFocus::Start);
}

#[test]
fn test_next_focus_presets_to_start() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Presets;
    picker.next_focus();
    assert_eq!(picker.focus, RangeFocus::Start);
}

#[test]
fn test_prev_focus_start_to_presets() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Start;
    picker.show_presets = true;
    picker.prev_focus();
    assert_eq!(picker.focus, RangeFocus::Presets);
}

#[test]
fn test_prev_focus_start_to_end_no_presets() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Start;
    picker.show_presets = false;
    picker.prev_focus();
    assert_eq!(picker.focus, RangeFocus::End);
}

#[test]
fn test_prev_focus_end_to_start() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::End;
    picker.prev_focus();
    assert_eq!(picker.focus, RangeFocus::Start);
}

#[test]
fn test_prev_focus_presets_to_end() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Presets;
    picker.prev_focus();
    assert_eq!(picker.focus, RangeFocus::End);
}

// =========================================================================
// Day movement tests - Start focus
// =========================================================================

#[test]
fn test_move_day_left_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.move_day_left();
    assert_eq!(picker.start_cursor_day, 14);
    assert_eq!(picker.start.date.day, 15); // Selected date unchanged
}

#[test]
fn test_move_day_left_wrap_to_previous_month_start() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.start_cursor_day = 1;
    picker.move_day_left();
    assert_eq!(picker.start_cursor_day, 31);
    assert_eq!(picker.start.date.month, 12);
    assert_eq!(picker.start.date.year, 2023);
}

#[test]
fn test_move_day_right_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.move_day_right();
    assert_eq!(picker.start_cursor_day, 16);
}

#[test]
fn test_move_day_right_wrap_to_next_month_start() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.start_cursor_day = 31;
    picker.move_day_right();
    assert_eq!(picker.start_cursor_day, 1);
    assert_eq!(picker.start.date.month, 2);
}

#[test]
fn test_move_week_up_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.move_week_up();
    assert_eq!(picker.start_cursor_day, 8);
}

#[test]
fn test_move_week_up_wrap_to_previous_month_start() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.start_cursor_day = 5;
    picker.move_week_up();
    assert_eq!(picker.start_cursor_day, 29);
    assert_eq!(picker.start.date.month, 12);
    assert_eq!(picker.start.date.year, 2023);
}

#[test]
fn test_move_week_down_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.move_week_down();
    assert_eq!(picker.start_cursor_day, 22);
}

#[test]
fn test_move_week_down_wrap_to_next_month_start() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.start_cursor_day = 28;
    picker.move_week_down();
    assert_eq!(picker.start_cursor_day, 4);
    assert_eq!(picker.start.date.month, 2);
}

// =========================================================================
// Day movement tests - End focus
// =========================================================================

#[test]
fn test_move_day_left_end_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::End;
    picker.move_day_left();
    assert_eq!(picker.end_cursor_day, 19);
}

#[test]
fn test_move_day_right_end_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::End;
    picker.move_day_right();
    assert_eq!(picker.end_cursor_day, 21);
}

// =========================================================================
// Month navigation tests
// =========================================================================

#[test]
fn test_prev_month_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 3, 15))
        .end_date(Date::new(2024, 3, 20));
    picker.focus = RangeFocus::Start;
    picker.prev_month();
    assert_eq!(picker.start.date.month, 2);
    assert_eq!(picker.start.date.year, 2024);
    assert_eq!(picker.start_cursor_day, 15);
}

#[test]
fn test_prev_month_clamps_cursor_start() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 3, 31))
        .end_date(Date::new(2024, 3, 31));
    picker.focus = RangeFocus::Start;
    picker.prev_month();
    // February 2024 has 29 days
    assert_eq!(picker.start.date.month, 2);
    assert_eq!(picker.start_cursor_day, 29);
}

#[test]
fn test_next_month_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.next_month();
    assert_eq!(picker.start.date.month, 2);
    assert_eq!(picker.start.date.year, 2024);
}

#[test]
fn test_next_month_clamps_cursor_start() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 31))
        .end_date(Date::new(2024, 1, 31));
    picker.focus = RangeFocus::Start;
    picker.next_month();
    // February 2024 has 29 days
    assert_eq!(picker.start.date.month, 2);
    assert_eq!(picker.start_cursor_day, 29);
}

// =========================================================================
// Year navigation tests
// =========================================================================

#[test]
fn test_prev_year_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 15))
        .end_date(Date::new(2024, 6, 20));
    picker.focus = RangeFocus::Start;
    picker.prev_year();
    assert_eq!(picker.start.date.year, 2023);
    assert_eq!(picker.start.date.month, 6);
}

#[test]
fn test_prev_year_handles_feb_29_start() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 2, 29))
        .end_date(Date::new(2024, 3, 1));
    picker.focus = RangeFocus::Start;
    picker.prev_year();
    // 2023 is not a leap year
    assert_eq!(picker.start.date.year, 2023);
    assert_eq!(picker.start_cursor_day, 28);
}

#[test]
fn test_next_year_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 15))
        .end_date(Date::new(2024, 6, 20));
    picker.focus = RangeFocus::Start;
    picker.next_year();
    assert_eq!(picker.start.date.year, 2025);
}

#[test]
fn test_next_year_handles_feb_29_start() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 2, 29))
        .end_date(Date::new(2024, 3, 1));
    picker.focus = RangeFocus::Start;
    picker.next_year();
    // 2025 is not a leap year
    assert_eq!(picker.start.date.year, 2025);
    assert_eq!(picker.start_cursor_day, 28);
}

// =========================================================================
// Date selection tests
// =========================================================================

#[test]
fn test_select_date_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.start_cursor_day = 10;
    picker.select_date();
    assert_eq!(picker.start.date.day, 10);
    assert_eq!(picker.active_preset, Some(PresetRange::Custom));
}

#[test]
fn test_select_date_end_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::End;
    picker.end_cursor_day = 25;
    picker.select_date();
    assert_eq!(picker.end.date.day, 25);
    assert_eq!(picker.active_preset, Some(PresetRange::Custom));
}

#[test]
fn test_select_date_presets_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Presets;
    picker.preset_cursor = 1; // Should be Yesterday
    picker.select_date();
    // Should apply the preset
    assert!(picker.active_preset.is_some());
}

#[test]
fn test_select_date_swaps_if_needed() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 20))
        .end_date(Date::new(2024, 1, 10));
    picker.focus = RangeFocus::End;
    picker.end_cursor_day = 5;
    picker.select_date();
    // Should swap to ensure start <= end
    assert!(picker.start.date <= picker.end.date);
}

// =========================================================================
// Preset navigation tests
// =========================================================================

#[test]
fn test_preset_up() {
    let mut picker = RangePicker::new();
    picker.preset_cursor = 5;
    preset_up(&mut picker);
    assert_eq!(picker.preset_cursor, 4);
}

#[test]
fn test_preset_up_wraps() {
    let mut picker = RangePicker::new();
    picker.preset_cursor = 0;
    preset_up(&mut picker);
    // Should wrap to last preset
    assert!(picker.preset_cursor > 0);
}

#[test]
fn test_preset_down() {
    let mut picker = RangePicker::new();
    picker.preset_cursor = 2;
    preset_down(&mut picker);
    assert_eq!(picker.preset_cursor, 3);
}

#[test]
fn test_preset_down_wraps() {
    let mut picker = RangePicker::new();
    let len = picker.presets.len();
    picker.preset_cursor = len - 1;
    preset_down(&mut picker);
    assert_eq!(picker.preset_cursor, 0);
}

#[test]
fn test_preset_up_empty_presets() {
    let mut picker = RangePicker::new();
    picker.presets.clear();
    let original = picker.preset_cursor;
    preset_up(&mut picker);
    // Should not change
    assert_eq!(picker.preset_cursor, original);
}

#[test]
fn test_preset_down_empty_presets() {
    let mut picker = RangePicker::new();
    picker.presets.clear();
    let original = picker.preset_cursor;
    preset_down(&mut picker);
    // Should not change
    assert_eq!(picker.preset_cursor, original);
}

// Helper functions for testing (since they're private)
fn preset_up(picker: &mut RangePicker) {
    if !picker.presets.is_empty() {
        picker.preset_cursor = picker
            .preset_cursor
            .checked_sub(1)
            .unwrap_or(picker.presets.len() - 1);
    }
}

fn preset_down(picker: &mut RangePicker) {
    if !picker.presets.is_empty() {
        picker.preset_cursor = (picker.preset_cursor + 1) % picker.presets.len();
    }
}

// =========================================================================
// Key handling tests - Focus navigation
// =========================================================================

#[test]
fn test_handle_key_tab() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Tab));
    assert_eq!(picker.focus, RangeFocus::End);
}

#[test]
fn test_handle_key_back_tab() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::End;
    assert!(picker.handle_key(&Key::BackTab));
    assert_eq!(picker.focus, RangeFocus::Start);
}

// =========================================================================
// Key handling tests - Arrow keys
// =========================================================================

#[test]
fn test_handle_key_left_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Left));
    assert_eq!(picker.start_cursor_day, 14);
}

#[test]
fn test_handle_key_right_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Right));
    assert_eq!(picker.start_cursor_day, 16);
}

#[test]
fn test_handle_key_up_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Up));
    assert_eq!(picker.start_cursor_day, 8);
}

#[test]
fn test_handle_key_down_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Down));
    assert_eq!(picker.start_cursor_day, 22);
}

#[test]
fn test_handle_key_vim_h_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Char('h')));
    assert_eq!(picker.start_cursor_day, 14);
}

#[test]
fn test_handle_key_vim_l_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Char('l')));
    assert_eq!(picker.start_cursor_day, 16);
}

#[test]
fn test_handle_key_vim_k_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Char('k')));
    assert_eq!(picker.start_cursor_day, 8);
}

#[test]
fn test_handle_key_vim_j_start_focus() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Char('j')));
    assert_eq!(picker.start_cursor_day, 22);
}

// =========================================================================
// Key handling tests - Preset navigation
// =========================================================================

#[test]
fn test_handle_key_up_presets_focus() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Presets;
    picker.preset_cursor = 3;
    assert!(picker.handle_key(&Key::Up));
    assert_eq!(picker.preset_cursor, 2);
}

#[test]
fn test_handle_key_down_presets_focus() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Presets;
    picker.preset_cursor = 3;
    assert!(picker.handle_key(&Key::Down));
    assert_eq!(picker.preset_cursor, 4);
}

#[test]
fn test_handle_key_vim_k_presets_focus() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Presets;
    picker.preset_cursor = 3;
    assert!(picker.handle_key(&Key::Char('k')));
    assert_eq!(picker.preset_cursor, 2);
}

#[test]
fn test_handle_key_vim_j_presets_focus() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Presets;
    picker.preset_cursor = 3;
    assert!(picker.handle_key(&Key::Char('j')));
    assert_eq!(picker.preset_cursor, 4);
}

// =========================================================================
// Key handling tests - Month/Year navigation
// =========================================================================

#[test]
fn test_handle_key_left_bracket() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 3, 15))
        .end_date(Date::new(2024, 3, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Char('[')));
    assert_eq!(picker.start.date.month, 2);
}

#[test]
fn test_handle_key_right_bracket() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Char(']')));
    assert_eq!(picker.start.date.month, 2);
}

#[test]
fn test_handle_key_left_brace() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 15))
        .end_date(Date::new(2024, 6, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Char('{')));
    assert_eq!(picker.start.date.year, 2023);
}

#[test]
fn test_handle_key_right_brace() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 15))
        .end_date(Date::new(2024, 6, 20));
    picker.focus = RangeFocus::Start;
    assert!(picker.handle_key(&Key::Char('}')));
    assert_eq!(picker.start.date.year, 2025);
}

// =========================================================================
// Key handling tests - Selection
// =========================================================================

#[test]
fn test_handle_key_enter() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.start_cursor_day = 10;
    assert!(picker.handle_key(&Key::Enter));
    assert_eq!(picker.start.date.day, 10);
}

#[test]
fn test_handle_key_space() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Start;
    picker.start_cursor_day = 10;
    assert!(picker.handle_key(&Key::Char(' ')));
    assert_eq!(picker.start.date.day, 10);
}

// =========================================================================
// Key handling tests - Disabled state
// =========================================================================

#[test]
fn test_handle_key_disabled_returns_false() {
    let mut picker = RangePicker::new();
    picker.state.disabled = true;
    assert!(!picker.handle_key(&Key::Tab));
    assert!(!picker.handle_key(&Key::Left));
    assert!(!picker.handle_key(&Key::Enter));
}

// =========================================================================
// Key handling tests - Preset focus ignores navigation keys
// =========================================================================

#[test]
fn test_handle_key_left_presets_focus_ignored() {
    let mut picker = RangePicker::new();
    picker.focus = RangeFocus::Presets;
    let original_cursor = picker.start_cursor_day;
    assert!(!picker.handle_key(&Key::Left));
    assert_eq!(picker.start_cursor_day, original_cursor);
}

#[test]
fn test_handle_key_month_brackets_presets_focus_ignored() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 6, 15))
        .end_date(Date::new(2024, 6, 20));
    picker.focus = RangeFocus::Presets;
    let original_month = picker.start.date.month;
    assert!(!picker.handle_key(&Key::Char('[')));
    assert_eq!(picker.start.date.month, original_month);
}

// =========================================================================
// Key handling tests - Unhandled keys
// =========================================================================

#[test]
fn test_handle_key_unhandled_char() {
    let mut picker = RangePicker::new();
    assert!(!picker.handle_key(&Key::Char('x')));
}

#[test]
fn test_handle_key_unhandled_special_key() {
    let mut picker = RangePicker::new();
    // PageUp/PageDown are not handled
    assert!(!picker.handle_key(&Key::PageUp));
    assert!(!picker.handle_key(&Key::PageDown));
}

// =========================================================================
// Edge cases
// =========================================================================

#[test]
fn test_current_date_mut_presets_focus_uses_end() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::Presets;
    picker.move_day_left(); // Should operate on end date
                               // End date should change
    assert!(picker.end_cursor_day != 20 || picker.end.date.month != 1);
}

#[test]
fn test_current_date_mut_end_focus_uses_end() {
    let mut picker = RangePicker::new()
        .start_date(Date::new(2024, 1, 15))
        .end_date(Date::new(2024, 1, 20));
    picker.focus = RangeFocus::End;
    let original_start_day = picker.start_cursor_day;
    picker.move_day_left();
    // Start date should not change
    assert_eq!(picker.start_cursor_day, original_start_day);
}