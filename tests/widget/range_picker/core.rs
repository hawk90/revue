//! RangePicker default state, observed through the public API

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{Date, PresetRange, RangePicker, Time, View};

fn render_rows(picker: &RangePicker) -> Vec<String> {
    let (w, h) = (80, 12);
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    picker.render(&mut ctx);
    (0..h)
        .map(|y| {
            (0..w)
                .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
                .collect()
        })
        .collect()
}

#[test]
fn test_range_picker_new_covers_all_of_today() {
    let picker = RangePicker::new();
    let today = Date::today();
    let (start, end) = picker.get_datetime_range();
    assert_eq!(start.date, today);
    assert_eq!(end.date, today);
    assert_eq!(start.time, Time::new(0, 0, 0));
    assert_eq!(end.time, Time::new(23, 59, 59));
}

#[test]
fn test_range_picker_new_active_preset_is_today() {
    assert_eq!(
        RangePicker::new().get_active_preset(),
        Some(PresetRange::Today)
    );
}

#[test]
fn test_range_picker_default_equals_new() {
    let new = RangePicker::new();
    let default = RangePicker::default();
    assert_eq!(new.get_datetime_range(), default.get_datetime_range());
    assert_eq!(new.get_active_preset(), default.get_active_preset());
    assert_eq!(new.get_focus(), default.get_focus());
}

#[test]
fn test_range_picker_new_shows_the_common_presets() {
    let rows = render_rows(&RangePicker::new());
    let panel: Vec<&str> = rows
        .iter()
        .map(|r| r[r.char_indices().nth(48).unwrap().0..].trim_end())
        .collect();
    assert_eq!(panel[0], "Presets");
    let common = PresetRange::common();
    for (i, preset) in common.iter().enumerate() {
        let marker = if *preset == PresetRange::Today {
            "● "
        } else {
            "  "
        };
        assert_eq!(panel[1 + i], format!("{marker}{}", preset.name()));
    }
    // ThisYear and Custom are not offered by default
    assert!(!rows
        .iter()
        .any(|r| r.contains("This Year") || r.contains("Custom")));
}

#[test]
fn test_range_picker_new_weeks_start_on_sunday() {
    let rows = render_rows(&RangePicker::new());
    assert!(rows[1].starts_with("Su Mo Tu We Th Fr Sa"), "{:?}", rows[1]);
}

#[test]
fn test_range_picker_new_preset_cursor_starts_at_first_preset() {
    let mut picker = RangePicker::new();
    picker.apply_preset(PresetRange::Yesterday);
    // Start -> End -> Presets, then select the preset under the cursor
    picker.handle_key(&Key::Tab);
    picker.handle_key(&Key::Tab);
    picker.handle_key(&Key::Enter);
    assert_eq!(picker.get_active_preset(), Some(PresetRange::Today));
}
