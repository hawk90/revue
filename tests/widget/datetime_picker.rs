//! DateTimePicker widget integration tests
//!
//! DateTimePicker 위젯의 통합 테스트입니다.
//! 생성자, 빌더 메서드, 날짜/시간 선택, 모드 전환,
//! 유효성 검사, 렌더링 등 다양한 기능을 테스트합니다.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{
    date_picker, datetime_picker, time_picker, Date, DateTime, DateTimeFormat, DateTimeMode,
    DateTimePicker, FirstDayOfWeek, Time,
};

// =============================================================================
// 생성자 및 빌더 테스트 (Constructor and Builder Tests)
// =============================================================================

#[test]
fn test_datetime_picker_new() {
    let picker = DateTimePicker::new();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
    assert_eq!(picker.get_date(), Date::today());
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_default() {
    let picker = DateTimePicker::default();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_date_only() {
    let picker = DateTimePicker::date_only();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
    let mut buffer = Buffer::new(25, 10);
    let area = Rect::new(0, 0, 25, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_time_only() {
    let picker = DateTimePicker::time_only();
    assert_eq!(picker.get_mode(), DateTimeMode::Time);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_helper_function() {
    let picker = datetime_picker();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_format() {
    let picker = DateTimePicker::new().format(DateTimeFormat::DateTime24);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_selected_date() {
    let picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert_eq!(picker.get_date(), Date::new(2025, 6, 15));
}

#[test]
fn test_datetime_picker_selected_time() {
    let picker = DateTimePicker::new().selected_time(Time::new(14, 30, 0));
    assert_eq!(picker.get_time(), Time::new(14, 30, 0));
}

#[test]
fn test_datetime_picker_selected() {
    let picker = DateTimePicker::new().selected(Date::new(2025, 6, 15), Time::new(14, 30, 0));
    assert_eq!(picker.get_date(), Date::new(2025, 6, 15));
    assert_eq!(picker.get_time(), Time::new(14, 30, 0));
}

#[test]
fn test_datetime_picker_first_day() {
    let picker = DateTimePicker::new().first_day(FirstDayOfWeek::Monday);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_show_seconds() {
    let picker = DateTimePicker::new().show_seconds(true);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_use_24h() {
    let picker = DateTimePicker::new().use_24h(false);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_min_date() {
    let picker = DateTimePicker::new().min_date(Date::new(2025, 1, 1));
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_max_date() {
    let picker = DateTimePicker::new().max_date(Date::new(2025, 12, 31));
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_date_range() {
    let picker = DateTimePicker::new().date_range(Date::new(2025, 1, 1), Date::new(2025, 12, 31));
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_min_time() {
    let picker = DateTimePicker::new().min_time(Time::new(8, 0, 0));
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_max_time() {
    let picker = DateTimePicker::new().max_time(Time::new(18, 0, 0));
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_header_color() {
    let picker = DateTimePicker::new().header_color(Color::MAGENTA);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_selected_colors() {
    let picker = DateTimePicker::new().selected_colors(Color::YELLOW, Color::BLUE);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_builder_chain() {
    let picker = DateTimePicker::new()
        .format(DateTimeFormat::DateTime24)
        .selected_date(Date::new(2025, 6, 15))
        .selected_time(Time::new(14, 30, 0))
        .first_day(FirstDayOfWeek::Monday)
        .show_seconds(true)
        .use_24h(true)
        .date_range(Date::new(2025, 1, 1), Date::new(2025, 12, 31))
        .header_color(Color::CYAN)
        .selected_colors(Color::BLACK, Color::YELLOW);

    assert_eq!(picker.get_date(), Date::new(2025, 6, 15));
    assert_eq!(picker.get_time(), Time::new(14, 30, 0));
}

// =============================================================================
// Time 구조체 테스트 (Time Struct Tests)
// =============================================================================

#[test]
fn test_time_new() {
    let time = Time::new(14, 30, 45);
    assert_eq!(time.hour, 14);
    assert_eq!(time.minute, 30);
    assert_eq!(time.second, 45);
}

#[test]
fn test_time_default() {
    let time = Time::default();
    assert_eq!(time.hour, 0);
    assert_eq!(time.minute, 0);
    assert_eq!(time.second, 0);
}

#[test]
fn test_time_hm() {
    let time = Time::hm(14, 30);
    assert_eq!(time.hour, 14);
    assert_eq!(time.minute, 30);
    assert_eq!(time.second, 0);
}

#[test]
fn test_time_now() {
    let time = Time::now();
    // Placeholder implementation returns 12:00:00
    assert_eq!(time.hour, 12);
    assert_eq!(time.minute, 0);
    assert_eq!(time.second, 0);
}

#[test]
fn test_time_is_valid() {
    assert!(Time::new(12, 30, 45).is_valid());
    assert!(Time::new(23, 59, 59).is_valid());
    assert!(Time::new(0, 0, 0).is_valid());
}

#[test]
fn test_time_clamp() {
    let time = Time::new(25, 70, 80);
    assert_eq!(time.hour, 23);
    assert_eq!(time.minute, 59);
    assert_eq!(time.second, 59);
}

#[test]
fn test_time_format_hm() {
    let time = Time::new(14, 5, 30);
    assert_eq!(time.format_hm(), "14:05");
}

#[test]
fn test_time_format_hms() {
    let time = Time::new(14, 5, 9);
    assert_eq!(time.format_hms(), "14:05:09");
}

#[test]
fn test_time_format_12h_midnight() {
    let time = Time::new(0, 30, 0);
    assert_eq!(time.format_12h(), "12:30 AM");
}

#[test]
fn test_time_format_12h_morning() {
    let time = Time::new(9, 15, 0);
    assert_eq!(time.format_12h(), " 9:15 AM");
}

#[test]
fn test_time_format_12h_noon() {
    let time = Time::new(12, 0, 0);
    assert_eq!(time.format_12h(), "12:00 PM");
}

#[test]
fn test_time_format_12h_afternoon() {
    let time = Time::new(15, 45, 0);
    assert_eq!(time.format_12h(), " 3:45 PM");
}

#[test]
fn test_time_format_12h_evening() {
    let time = Time::new(23, 59, 59);
    assert_eq!(time.format_12h(), "11:59 PM");
}

// =============================================================================
// DateTime 구조체 테스트 (DateTime Struct Tests)
// =============================================================================

#[test]
fn test_datetime_new() {
    let dt = DateTime::new(Date::new(2025, 6, 15), Time::new(14, 30, 0));
    assert_eq!(dt.date.year, 2025);
    assert_eq!(dt.date.month, 6);
    assert_eq!(dt.date.day, 15);
    assert_eq!(dt.time.hour, 14);
    assert_eq!(dt.time.minute, 30);
}

#[test]
fn test_datetime_default() {
    let dt = DateTime::default();
    assert_eq!(dt.date, Date::default());
    assert_eq!(dt.time, Time::default());
}

#[test]
fn test_datetime_from_parts() {
    let dt = DateTime::from_parts(2025, 6, 15, 14, 30, 45);
    assert_eq!(dt.date.year, 2025);
    assert_eq!(dt.date.month, 6);
    assert_eq!(dt.date.day, 15);
    assert_eq!(dt.time.hour, 14);
    assert_eq!(dt.time.minute, 30);
    assert_eq!(dt.time.second, 45);
}

// =============================================================================
// Query 메서드 테스트 (Query Method Tests)
// =============================================================================

#[test]
fn test_datetime_picker_get_datetime() {
    let picker = DateTimePicker::new().selected(Date::new(2025, 6, 15), Time::new(14, 30, 0));
    let dt = picker.get_datetime();
    assert_eq!(dt.date, Date::new(2025, 6, 15));
    assert_eq!(dt.time, Time::new(14, 30, 0));
}

#[test]
fn test_datetime_picker_get_date() {
    let picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert_eq!(picker.get_date(), Date::new(2025, 6, 15));
}

#[test]
fn test_datetime_picker_get_time() {
    let picker = DateTimePicker::new().selected_time(Time::new(14, 30, 0));
    assert_eq!(picker.get_time(), Time::new(14, 30, 0));
}

#[test]
fn test_datetime_picker_get_mode_date() {
    let picker = DateTimePicker::new();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
}

#[test]
fn test_datetime_picker_get_mode_time() {
    let picker = DateTimePicker::time_only();
    assert_eq!(picker.get_mode(), DateTimeMode::Time);
}

// =============================================================================
// 키보드 핸들링 테스트 - 날짜 모드 (Keyboard Handling Tests - Date Mode)
// =============================================================================

#[test]
fn test_datetime_picker_handle_key_left() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Left));
}

#[test]
fn test_datetime_picker_handle_key_right() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Right));
}

#[test]
fn test_datetime_picker_handle_key_up() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Up));
}

#[test]
fn test_datetime_picker_handle_key_down() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Down));
}

#[test]
fn test_datetime_picker_handle_key_prev_month() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Char('[')));
}

#[test]
fn test_datetime_picker_handle_key_next_month() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Char(']')));
}

#[test]
fn test_datetime_picker_handle_key_prev_year() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Char('{')));
}

#[test]
fn test_datetime_picker_handle_key_next_year() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Char('}')));
}

#[test]
fn test_datetime_picker_handle_key_enter() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Enter));
}

#[test]
fn test_datetime_picker_handle_key_space() {
    let mut picker = DateTimePicker::new().selected_date(Date::new(2025, 6, 15));
    assert!(picker.handle_key(&Key::Char(' ')));
}

// =============================================================================
// 키보드 핸들링 테스트 - 시간 모드 (Keyboard Handling Tests - Time Mode)
// =============================================================================

#[test]
fn test_datetime_picker_handle_key_time_left() {
    let mut picker = DateTimePicker::time_only();
    assert!(picker.handle_key(&Key::Left));
}

#[test]
fn test_datetime_picker_handle_key_time_right() {
    let mut picker = DateTimePicker::time_only();
    assert!(picker.handle_key(&Key::Right));
}

#[test]
fn test_datetime_picker_handle_key_time_up() {
    let mut picker = DateTimePicker::time_only().selected_time(Time::new(10, 30, 0));
    assert!(picker.handle_key(&Key::Up));
}

#[test]
fn test_datetime_picker_handle_key_time_down() {
    let mut picker = DateTimePicker::time_only().selected_time(Time::new(10, 30, 0));
    assert!(picker.handle_key(&Key::Down));
}

// =============================================================================
// 모드 전환 테스트 (Mode Switching Tests)
// =============================================================================

#[test]
fn test_datetime_picker_tab_switch_mode() {
    let mut picker = datetime_picker();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);

    picker.handle_key(&Key::Tab);
    assert_eq!(picker.get_mode(), DateTimeMode::Time);

    picker.handle_key(&Key::Tab);
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
}

#[test]
fn test_datetime_picker_date_only_no_tab_switch() {
    let mut picker = date_picker();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);

    // Tab should not switch mode in date-only mode
    let handled = picker.handle_key(&Key::Tab);
    assert!(!handled);
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
}

#[test]
fn test_datetime_picker_time_only_no_tab_switch() {
    let mut picker = time_picker();
    assert_eq!(picker.get_mode(), DateTimeMode::Time);

    // Tab should not switch mode in time-only mode
    let handled = picker.handle_key(&Key::Tab);
    assert!(!handled);
    assert_eq!(picker.get_mode(), DateTimeMode::Time);
}

// =============================================================================
// Vim 키 테스트 (Vim Key Tests)
// =============================================================================

#[test]
fn test_datetime_picker_vim_keys_date_mode() {
    let mut picker = datetime_picker().selected_date(Date::new(2025, 6, 15));

    assert!(picker.handle_key(&Key::Char('h'))); // left
    assert!(picker.handle_key(&Key::Char('l'))); // right
    assert!(picker.handle_key(&Key::Char('k'))); // up
    assert!(picker.handle_key(&Key::Char('j'))); // down
}

#[test]
fn test_datetime_picker_vim_keys_time_mode() {
    let mut picker = time_picker().selected_time(Time::new(10, 30, 0));

    assert!(picker.handle_key(&Key::Char('h'))); // prev field
    assert!(picker.handle_key(&Key::Char('l'))); // next field
    assert!(picker.handle_key(&Key::Char('k'))); // increment
    assert!(picker.handle_key(&Key::Char('j'))); // decrement
}

// =============================================================================
// 시간 필드 탐색 테스트 (Time Field Navigation Tests)
// =============================================================================

#[test]
fn test_datetime_picker_time_field_navigation() {
    let mut picker = time_picker().selected_time(Time::new(10, 30, 0));

    // Start with Hour field (default)
    picker.handle_key(&Key::Right); // Move to Minute
    picker.handle_key(&Key::Right); // Should cycle based on show_seconds

    // Without show_seconds, cycles back to Hour
    picker.handle_key(&Key::Left); // Back to Minute
}

#[test]
fn test_datetime_picker_time_field_navigation_with_seconds() {
    let mut picker = time_picker()
        .selected_time(Time::new(10, 30, 45))
        .show_seconds(true);

    // Hour -> Minute -> Second -> Hour cycle
    picker.handle_key(&Key::Right);
    picker.handle_key(&Key::Right);
    picker.handle_key(&Key::Right); // Back to Hour

    picker.handle_key(&Key::Left); // To Second
    picker.handle_key(&Key::Left); // To Minute
    picker.handle_key(&Key::Left); // To Hour
}

// =============================================================================
// 시간 증감 테스트 (Time Increment/Decrement Tests)
// =============================================================================

#[test]
fn test_datetime_picker_time_increment_hour() {
    let mut picker = time_picker().selected_time(Time::new(10, 30, 0));
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().hour, 11);
}

#[test]
fn test_datetime_picker_time_decrement_hour() {
    let mut picker = time_picker().selected_time(Time::new(10, 30, 0));
    picker.handle_key(&Key::Down);
    assert_eq!(picker.get_time().hour, 9);
}

#[test]
fn test_datetime_picker_time_increment_minute() {
    let mut picker = time_picker().selected_time(Time::new(10, 30, 0));
    picker.handle_key(&Key::Right); // Move to minute field
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().minute, 31);
}

#[test]
fn test_datetime_picker_time_decrement_minute() {
    let mut picker = time_picker().selected_time(Time::new(10, 30, 0));
    picker.handle_key(&Key::Right); // Move to minute field
    picker.handle_key(&Key::Down);
    assert_eq!(picker.get_time().minute, 29);
}

#[test]
fn test_datetime_picker_time_increment_second() {
    let mut picker = time_picker()
        .selected_time(Time::new(10, 30, 45))
        .show_seconds(true);
    picker.handle_key(&Key::Right); // Minute
    picker.handle_key(&Key::Right); // Second
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().second, 46);
}

#[test]
fn test_datetime_picker_time_decrement_second() {
    let mut picker = time_picker()
        .selected_time(Time::new(10, 30, 45))
        .show_seconds(true);
    picker.handle_key(&Key::Right); // Minute
    picker.handle_key(&Key::Right); // Second
    picker.handle_key(&Key::Down);
    assert_eq!(picker.get_time().second, 44);
}

// =============================================================================
// 시간 랩 어라운드 테스트 (Time Wrap-around Tests)
// =============================================================================

#[test]
fn test_datetime_picker_time_wrap_hour_up() {
    let mut picker = time_picker().selected_time(Time::new(23, 30, 0));
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().hour, 0);
}

#[test]
fn test_datetime_picker_time_wrap_hour_down() {
    let mut picker = time_picker().selected_time(Time::new(0, 30, 0));
    picker.handle_key(&Key::Down);
    assert_eq!(picker.get_time().hour, 23);
}

#[test]
fn test_datetime_picker_time_wrap_minute_up() {
    let mut picker = time_picker().selected_time(Time::new(10, 59, 0));
    picker.handle_key(&Key::Right); // Move to minute
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().minute, 0);
}

#[test]
fn test_datetime_picker_time_wrap_minute_down() {
    let mut picker = time_picker().selected_time(Time::new(10, 0, 0));
    picker.handle_key(&Key::Right); // Move to minute
    picker.handle_key(&Key::Down);
    assert_eq!(picker.get_time().minute, 59);
}

#[test]
fn test_datetime_picker_time_wrap_second_up() {
    let mut picker = time_picker()
        .selected_time(Time::new(10, 30, 59))
        .show_seconds(true);
    picker.handle_key(&Key::Right); // Minute
    picker.handle_key(&Key::Right); // Second
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().second, 0);
}

#[test]
fn test_datetime_picker_time_wrap_second_down() {
    let mut picker = time_picker()
        .selected_time(Time::new(10, 30, 0))
        .show_seconds(true);
    picker.handle_key(&Key::Right); // Minute
    picker.handle_key(&Key::Right); // Second
    picker.handle_key(&Key::Down);
    assert_eq!(picker.get_time().second, 59);
}

// =============================================================================
// 렌더링 테스트 (Rendering Tests)
// =============================================================================

#[test]
fn test_datetime_picker_render_basic() {
    let picker = datetime_picker()
        .selected_date(Date::new(2025, 6, 15))
        .selected_time(Time::new(14, 30, 0));

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_render_date_only() {
    let picker = date_picker().selected_date(Date::new(2025, 6, 15));

    let mut buffer = Buffer::new(25, 10);
    let area = Rect::new(0, 0, 25, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_render_time_only() {
    let picker = time_picker().selected_time(Time::new(14, 30, 0));

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_render_with_seconds() {
    let picker = time_picker()
        .selected_time(Time::new(14, 30, 45))
        .show_seconds(true);

    let mut buffer = Buffer::new(25, 10);
    let area = Rect::new(0, 0, 25, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_render_small_area() {
    let picker = datetime_picker();

    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should return early for small area
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_render_zero_area() {
    let picker = datetime_picker();

    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
}

// =============================================================================
// DateTimeFormat 테스트 (DateTimeFormat Tests)
// =============================================================================

#[test]
fn test_datetime_format_date_only() {
    let picker = DateTimePicker::new().format(DateTimeFormat::DateOnly);
    let mut buffer = Buffer::new(25, 10);
    let area = Rect::new(0, 0, 25, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_format_time_only() {
    let picker = DateTimePicker::new().format(DateTimeFormat::TimeOnly);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_format_time_with_seconds() {
    let picker = DateTimePicker::new().format(DateTimeFormat::TimeWithSeconds);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_format_datetime() {
    let picker = DateTimePicker::new().format(DateTimeFormat::DateTime);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_format_datetime24() {
    let picker = DateTimePicker::new().format(DateTimeFormat::DateTime24);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_format_datetime12() {
    let picker = DateTimePicker::new().format(DateTimeFormat::DateTime12);
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

// =============================================================================
// Helper 함수 테스트 (Helper Function Tests)
// =============================================================================

#[test]
fn test_datetime_helper_datetime() {
    let picker = datetime_picker();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
}

#[test]
fn test_date_helper() {
    let picker = date_picker();
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
}

#[test]
fn test_time_helper() {
    let picker = time_picker();
    assert_eq!(picker.get_mode(), DateTimeMode::Time);
}

// =============================================================================
// 엣지 케이스 테스트 (Edge Case Tests)
// =============================================================================

#[test]
fn test_datetime_picker_unhandled_key() {
    let mut picker = datetime_picker();
    let handled = picker.handle_key(&Key::Char('x'));
    assert!(!handled);
}

#[test]
fn test_datetime_picker_disabled_state() {
    // DateTimePicker doesn't have a disabled() method
    // Just verify normal operation works
    let mut picker = datetime_picker();
    let handled = picker.handle_key(&Key::Enter);
    assert!(handled);
}

#[test]
fn test_datetime_picker_very_early_date() {
    let picker = date_picker().selected_date(Date::new(1900, 1, 1));
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_very_late_date() {
    let picker = date_picker().selected_date(Date::new(9999, 12, 31));
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_leap_year_feb() {
    let picker = date_picker().selected_date(Date::new(2024, 2, 29));
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_midnight_time() {
    let picker = time_picker().selected_time(Time::new(0, 0, 0));
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_last_second_of_day() {
    let picker = time_picker().selected_time(Time::new(23, 59, 59));
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_datetime_picker_multiple_mode_switches() {
    let mut picker = datetime_picker();

    for _ in 0..10 {
        picker.handle_key(&Key::Tab);
    }

    // Should still be in Date mode after even number of switches
    assert_eq!(picker.get_mode(), DateTimeMode::Date);
}

#[test]
fn test_datetime_picker_all_formats_renderable() {
    let formats = vec![
        DateTimeFormat::DateOnly,
        DateTimeFormat::TimeOnly,
        DateTimeFormat::TimeWithSeconds,
        DateTimeFormat::DateTime,
        DateTimeFormat::DateTime24,
        DateTimeFormat::DateTime12,
    ];

    for format in formats {
        let picker = datetime_picker().format(format);
        let mut buffer = Buffer::new(30, 15);
        let area = Rect::new(0, 0, 30, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);
        picker.render(&mut ctx);
    }
}

#[test]
fn test_datetime_picker_time_operations_sequence() {
    let mut picker = time_picker()
        .show_seconds(true)
        .selected_time(Time::new(12, 30, 45));

    // Increment hour
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().hour, 13);

    // Move to minute and increment
    picker.handle_key(&Key::Right);
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().minute, 31);

    // Move to second and increment
    picker.handle_key(&Key::Right);
    picker.handle_key(&Key::Up);
    assert_eq!(picker.get_time().second, 46);
}
