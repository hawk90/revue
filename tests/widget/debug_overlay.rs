//! DebugOverlay widget integration tests
//!
//! The overlay keeps its configuration private, so builder options are
//! checked through what the panel draws.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
#[allow(deprecated)] // the global debug flag is deprecated (#799)
use revue::widget::{disable_debug, enable_debug, is_debug_enabled};
use revue::widget::{
    DebugConfig, DebugEvent, DebugOverlay, DebugPosition, EventLog, PerfMetrics, Text, WidgetInfo,
};
use serial_test::serial;
use std::time::Duration;

// =============================================================================
// Helpers
// =============================================================================

fn render_in(overlay: &DebugOverlay<Text>, buffer: &mut Buffer, area: Rect) {
    let mut ctx = RenderContext::new(buffer, area);
    overlay.render(&mut ctx);
}

/// Render into an 80x24 buffer.
fn render(overlay: &DebugOverlay<Text>) -> Buffer {
    let mut buffer = Buffer::new(80, 24);
    render_in(overlay, &mut buffer, Rect::new(0, 0, 80, 24));
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

fn text(buffer: &Buffer) -> String {
    (0..buffer.height())
        .map(|y| row(buffer, y))
        .collect::<Vec<_>>()
        .join("\n")
}

fn symbol(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

fn has_panel(buffer: &Buffer) -> bool {
    text(buffer).contains('┌')
}

fn event_text(event: &DebugEvent) -> &str {
    match event {
        DebugEvent::KeyPress(s)
        | DebugEvent::Mouse(s)
        | DebugEvent::StateChange(s)
        | DebugEvent::Custom(s) => s,
    }
}

// =============================================================================
// Builder options, observed through the rendered panel
// =============================================================================

#[test]
fn test_debug_overlay_wrap_defaults() {
    let overlay = DebugOverlay::wrap(Text::new("Hello"));
    let buffer = render(&overlay);

    // Visible by default, top-right, 40 columns wide, at most 20 rows
    assert_eq!(symbol(&buffer, 40, 0), '┌');
    assert_eq!(symbol(&buffer, 79, 0), '┐');
    assert_eq!(symbol(&buffer, 40, 19), '└');
    // Metrics on; tree and events off
    let all = text(&buffer);
    assert!(all.contains("FPS: 0.0"));
    assert!(!all.contains("Widgets:"));
    assert!(!all.contains("Events:"));
    // The wrapped view is still drawn
    assert!(row(&buffer, 0).starts_with("Hello"));
}

#[test]
fn test_debug_overlay_title() {
    let buffer = render(&DebugOverlay::wrap(Text::new("Hello")));
    assert!(row(&buffer, 0).contains(" Debug "));
}

#[test]
fn test_debug_overlay_visible_false() {
    let overlay = DebugOverlay::wrap(Text::new("Hello")).visible(false);
    let buffer = render(&overlay);
    assert!(!has_panel(&buffer));
    assert!(row(&buffer, 0).starts_with("Hello"));
}

#[test]
fn test_debug_overlay_toggle() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test")).visible(true);
    assert!(has_panel(&render(&overlay)));

    overlay.toggle();
    assert!(!has_panel(&render(&overlay)));

    overlay.toggle();
    assert!(has_panel(&render(&overlay)));
}

#[test]
fn test_debug_overlay_toggle_from_hidden() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test")).visible(false);
    overlay.toggle();
    assert!(has_panel(&render(&overlay)));
}

#[test]
fn test_debug_overlay_show_metrics_false() {
    let overlay = DebugOverlay::wrap(Text::new("Test")).show_metrics(false);
    let all = text(&render(&overlay));
    assert!(has_panel(&render(&overlay)));
    assert!(!all.contains("FPS:"));
    assert!(!all.contains("Layout:"));
}

#[test]
fn test_debug_overlay_show_tree() {
    let overlay = DebugOverlay::wrap(Text::new("Test")).show_tree(true);
    assert!(text(&render(&overlay)).contains("Widgets:"));
}

#[test]
fn test_debug_overlay_show_events() {
    let overlay = DebugOverlay::wrap(Text::new("Test")).show_events(true);
    assert!(text(&render(&overlay)).contains("Events:"));
}

#[test]
fn test_debug_overlay_all_sections_enabled() {
    let overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_metrics(true)
        .show_tree(true)
        .show_events(true);
    let all = text(&render(&overlay));
    let fps = all.find("FPS:").unwrap();
    let widgets = all.find("Widgets:").unwrap();
    let events = all.find("Events:").unwrap();
    // Sections are stacked in this order
    assert!(fps < widgets && widgets < events);
}

#[test]
fn test_debug_overlay_all_sections_disabled() {
    let overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_metrics(false)
        .show_tree(false)
        .show_events(false);
    let buffer = render(&overlay);
    let all = text(&buffer);
    assert!(has_panel(&buffer));
    assert!(!all.contains("FPS:"));
    assert!(!all.contains("Widgets:"));
    assert!(!all.contains("Events:"));
}

#[test]
fn test_debug_overlay_width() {
    let overlay = DebugOverlay::wrap(Text::new("Test")).width(30);
    let buffer = render(&overlay);
    assert_eq!(symbol(&buffer, 50, 0), '┌');
    assert_eq!(symbol(&buffer, 79, 0), '┐');
    assert_ne!(symbol(&buffer, 49, 0), '─');
}

#[test]
fn test_debug_overlay_builder_chain() {
    let overlay = DebugOverlay::wrap(Text::new("Test"))
        .visible(true)
        .show_metrics(true)
        .show_tree(true)
        .show_events(true)
        .position(DebugPosition::TopLeft)
        .width(25);
    let buffer = render(&overlay);
    assert_eq!(symbol(&buffer, 0, 0), '┌');
    assert_eq!(symbol(&buffer, 24, 0), '┐');
    let all = text(&buffer);
    assert!(all.contains("FPS:"));
    assert!(all.contains("Widgets:"));
    assert!(all.contains("Events:"));
}

// =============================================================================
// Panel placement
// =============================================================================

#[test]
fn test_debug_position_top_left() {
    let overlay = DebugOverlay::wrap(Text::new("Test"))
        .width(20)
        .position(DebugPosition::TopLeft);
    let buffer = render(&overlay);
    assert_eq!(symbol(&buffer, 0, 0), '┌');
    assert_eq!(symbol(&buffer, 19, 0), '┐');
    assert_eq!(symbol(&buffer, 0, 19), '└');
}

#[test]
fn test_debug_position_top_right() {
    let overlay = DebugOverlay::wrap(Text::new("Test"))
        .width(20)
        .position(DebugPosition::TopRight);
    let buffer = render(&overlay);
    assert_eq!(symbol(&buffer, 60, 0), '┌');
    assert_eq!(symbol(&buffer, 79, 19), '┘');
}

#[test]
fn test_debug_position_bottom_left() {
    let overlay = DebugOverlay::wrap(Text::new("Test"))
        .width(20)
        .position(DebugPosition::BottomLeft);
    let buffer = render(&overlay);
    // 20 rows high, flush with the bottom of a 24-row area
    assert_eq!(symbol(&buffer, 0, 4), '┌');
    assert_eq!(symbol(&buffer, 0, 23), '└');
    assert_eq!(symbol(&buffer, 19, 23), '┘');
}

#[test]
fn test_debug_position_bottom_right() {
    let overlay = DebugOverlay::wrap(Text::new("Test"))
        .width(20)
        .position(DebugPosition::BottomRight);
    let buffer = render(&overlay);
    assert_eq!(symbol(&buffer, 60, 4), '┌');
    assert_eq!(symbol(&buffer, 79, 23), '┘');
}

#[test]
fn test_panel_rect_width_clamping() {
    let overlay = DebugOverlay::wrap(Text::new("Test")).width(100);
    let buffer = render(&overlay);
    // Clamped to the 80-column area
    assert_eq!(symbol(&buffer, 0, 0), '┌');
    assert_eq!(symbol(&buffer, 79, 0), '┐');
}

#[test]
fn test_panel_rect_height_clamping() {
    let overlay = DebugOverlay::wrap(Text::new("Test")).width(20);
    let mut buffer = Buffer::new(80, 10);
    render_in(&overlay, &mut buffer, Rect::new(0, 0, 80, 10));
    // Clamped to the 10-row area
    assert_eq!(symbol(&buffer, 60, 9), '└');
}

#[test]
fn test_panel_rect_with_offset_area() {
    let overlay = DebugOverlay::wrap(Text::new("Test"))
        .width(20)
        .position(DebugPosition::TopLeft);
    let mut buffer = Buffer::new(100, 40);
    render_in(&overlay, &mut buffer, Rect::new(10, 5, 80, 24));
    assert_eq!(symbol(&buffer, 10, 5), '┌');
    assert_eq!(symbol(&buffer, 29, 5), '┐');
    assert_eq!(symbol(&buffer, 9, 5), ' ');
}

#[test]
fn test_debug_overlay_render_small_buffer() {
    let overlay = DebugOverlay::wrap(Text::new("Test")).width(10);
    let mut buffer = Buffer::new(20, 10);
    render_in(&overlay, &mut buffer, Rect::new(0, 0, 20, 10));
    assert_eq!(symbol(&buffer, 10, 0), '┌');
    assert_eq!(symbol(&buffer, 19, 9), '┘');
    assert!(row(&buffer, 0).starts_with("Test"));
}

#[test]
fn test_debug_overlay_render_zero_width() {
    let overlay = DebugOverlay::wrap(Text::new("Test")).width(0);
    let buffer = render(&overlay);
    // No room for a panel: only the wrapped view is drawn
    assert!(!has_panel(&buffer));
    assert!(row(&buffer, 0).starts_with("Test"));
}

#[test]
fn test_debug_overlay_render_one_row_area() {
    let overlay = DebugOverlay::wrap(Text::new("Test"));
    let mut buffer = Buffer::new(80, 1);
    render_in(&overlay, &mut buffer, Rect::new(0, 0, 80, 1));
    // No room for a bordered panel
    assert!(!has_panel(&buffer));
    assert!(row(&buffer, 0).starts_with("Test"));
}

#[test]
fn test_debug_overlay_narrow_panel_truncates_lines() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_metrics(false)
        .show_tree(true)
        .width(4);
    overlay.record_widget(WidgetInfo::new("VeryLongWidgetName"));
    let buffer = render(&overlay);
    // Nothing is drawn outside the 4-column panel
    assert_eq!(symbol(&buffer, 76, 0), '┌');
    assert_eq!(symbol(&buffer, 79, 0), '┐');
    assert!(!text(&buffer).contains("Very"));
}

#[test]
fn test_debug_overlay_metrics_stay_inside_narrow_panel() {
    let mut overlay = DebugOverlay::wrap(Text::new(""))
        .position(DebugPosition::TopLeft)
        .width(12);
    overlay.metrics_mut().record_layout(Duration::from_secs(10));
    let buffer = render(&overlay);
    let layout_row = (0..24)
        .find(|&y| row(&buffer, y).contains("Layout"))
        .unwrap();
    // "Layout: 10000.00ms" is cut at the right border, not drawn past it
    assert_eq!(symbol(&buffer, 11, layout_row), '│');
    let past_panel: String = row(&buffer, layout_row).chars().skip(12).collect();
    assert_eq!(past_panel.trim(), "");
}

#[test]
fn test_debug_overlay_truncates_wide_chars() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_metrics(false)
        .show_events(true)
        .width(12);
    overlay.log_event(DebugEvent::Custom("한글로 된 아주 긴 이벤트".to_string()));
    let buffer = render(&overlay);
    let all = text(&buffer);
    assert!(all.contains("Events:"));
    assert!(all.contains("..."));
    // The right border is intact
    assert_eq!(symbol(&buffer, 79, 3), '│');
}

// =============================================================================
// Panel contents
// =============================================================================

#[test]
fn test_debug_overlay_render_with_metrics() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_metrics(true)
        .width(30);
    overlay.metrics_mut().start_frame();
    overlay
        .metrics_mut()
        .record_layout(Duration::from_millis(5));
    overlay
        .metrics_mut()
        .record_render(Duration::from_millis(3));

    let all = text(&render(&overlay));
    assert!(all.contains("FPS: 0.0"));
    assert!(all.contains("Frame: 0.00ms"));
    assert!(all.contains("Layout: 5.00ms"));
    assert!(all.contains("Render: 3.00ms"));
}

#[test]
fn test_debug_overlay_render_with_tree() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_tree(true)
        .width(30);
    overlay.record_widget(WidgetInfo::new("Button").id("submit").class("primary"));

    let all = text(&render(&overlay));
    assert!(all.contains("Widgets:"));
    assert!(all.contains("Button #submit .primary"));
}

#[test]
fn test_debug_overlay_clear_widgets() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_tree(true)
        .width(30);
    overlay.record_widget(WidgetInfo::new("Button"));
    overlay.record_widget(WidgetInfo::new("Label"));
    let all = text(&render(&overlay));
    assert!(all.contains("Button"));
    assert!(all.contains("Label"));

    overlay.clear_widgets();
    let all = text(&render(&overlay));
    assert!(all.contains("Widgets:"));
    assert!(!all.contains("Button"));
    assert!(!all.contains("Label"));
}

#[test]
fn test_debug_overlay_many_widgets() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_metrics(false)
        .show_tree(true)
        .width(30);
    for i in 0..100 {
        overlay.record_widget(WidgetInfo::new(format!("Widget{}", i)));
    }
    let buffer = render(&overlay);
    let all = text(&buffer);
    assert!(all.contains("Widget0"));
    // Only what fits in the 20-row panel is listed, and the bottom border
    // stays intact
    assert!(!all.contains("Widget99"));
    assert_eq!(symbol(&buffer, 50, 19), '└');
    assert_eq!(symbol(&buffer, 79, 19), '┘');
    assert_eq!(row(&buffer, 20).trim(), "");
}

#[test]
fn test_debug_overlay_render_with_events() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_events(true)
        .width(30);
    overlay.log_event(DebugEvent::KeyPress("a".to_string()));
    overlay.log_event(DebugEvent::Mouse("click".to_string()));
    overlay.log_event(DebugEvent::StateChange("focus".to_string()));
    overlay.log_event(DebugEvent::Custom("hello".to_string()));

    let all = text(&render(&overlay));
    assert!(all.contains("Key: a"));
    assert!(all.contains("Mouse: click"));
    assert!(all.contains("State: focus"));
    assert!(all.contains("hello"));
    // Most recent first
    assert!(all.find("hello").unwrap() < all.find("Key: a").unwrap());
}

#[test]
fn test_debug_overlay_events_mut() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test")).show_events(true);
    overlay
        .events_mut()
        .log(DebugEvent::KeyPress("x".to_string()));
    assert!(text(&render(&overlay)).contains("Key: x"));

    overlay.events_mut().clear();
    assert!(!text(&render(&overlay)).contains("Key: x"));
}

#[test]
fn test_debug_overlay_shows_five_recent_events() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"))
        .show_metrics(false)
        .show_events(true);
    for i in 0..8 {
        overlay.log_event(DebugEvent::Custom(format!("event-{}", i)));
    }
    let all = text(&render(&overlay));
    for i in 3..8 {
        assert!(all.contains(&format!("event-{}", i)));
    }
    for i in 0..3 {
        assert!(!all.contains(&format!("event-{}", i)));
    }
}

#[test]
fn test_perf_metrics_metrics_mut() {
    let mut overlay = DebugOverlay::wrap(Text::new("Test"));
    overlay
        .metrics_mut()
        .record_layout(Duration::from_millis(5));
    assert_eq!(overlay.metrics_mut().avg_layout_time_ms(), 5.0);
    assert!(text(&render(&overlay)).contains("Layout: 5.00ms"));
}

// =============================================================================
// DebugConfig
// =============================================================================

#[test]
#[allow(deprecated)]
fn test_debug_config_default() {
    let config = DebugConfig::default();

    assert!(config.show_metrics);
    assert!(!config.show_tree);
    assert!(!config.show_events);
    assert!(!config.show_styles);
    assert_eq!(config.position, DebugPosition::TopRight);
    assert_eq!(config.width, 40);
    assert_eq!(config.max_height, 20);
    assert_eq!(config.opacity, 220);
}

// =============================================================================
// PerfMetrics
// =============================================================================

#[test]
fn test_perf_metrics_new() {
    let metrics = PerfMetrics::new();
    assert_eq!(metrics.fps(), 0.0);
    assert_eq!(metrics.avg_frame_time_ms(), 0.0);
    assert_eq!(metrics.avg_layout_time_ms(), 0.0);
    assert_eq!(metrics.avg_render_time_ms(), 0.0);
}

#[test]
fn test_perf_metrics_default_keeps_samples() {
    let mut metrics = PerfMetrics::default();
    metrics.record_layout(Duration::from_millis(4));
    metrics.record_render(Duration::from_millis(2));
    assert_eq!(metrics.avg_layout_time_ms(), 4.0);
    assert_eq!(metrics.avg_render_time_ms(), 2.0);
}

#[test]
fn test_perf_metrics_start_frame() {
    let mut metrics = PerfMetrics::new();
    // A single frame start has no completed frame to measure yet
    metrics.start_frame();
    assert_eq!(metrics.fps(), 0.0);
    assert_eq!(metrics.avg_frame_time_ms(), 0.0);
}

#[test]
fn test_perf_metrics_multiple_frames() {
    let mut metrics = PerfMetrics::new();
    metrics.start_frame();
    std::thread::sleep(Duration::from_millis(20));
    metrics.start_frame();

    let frame_ms = metrics.avg_frame_time_ms();
    assert!(frame_ms >= 20.0);
    // fps is the inverse of the average frame time
    let fps = metrics.fps();
    assert!(fps > 0.0 && fps <= 50.0);
    assert!((fps - 1000.0 / frame_ms).abs() < 1e-6);
}

#[test]
fn test_perf_metrics_record_layout() {
    let mut metrics = PerfMetrics::new();
    metrics.record_layout(Duration::from_millis(5));
    assert_eq!(metrics.avg_layout_time_ms(), 5.0);
    metrics.record_layout(Duration::from_millis(10));
    assert_eq!(metrics.avg_layout_time_ms(), 7.5);
}

#[test]
fn test_perf_metrics_record_render() {
    let mut metrics = PerfMetrics::new();
    metrics.record_render(Duration::from_millis(3));
    assert_eq!(metrics.avg_render_time_ms(), 3.0);
    metrics.record_render(Duration::from_millis(7));
    assert_eq!(metrics.avg_render_time_ms(), 5.0);
}

#[test]
fn test_perf_metrics_reset() {
    let mut metrics = PerfMetrics::new();
    metrics.start_frame();
    std::thread::sleep(Duration::from_millis(1));
    metrics.start_frame();
    metrics.record_layout(Duration::from_millis(5));
    metrics.record_render(Duration::from_millis(3));
    assert!(metrics.fps() > 0.0);

    metrics.reset();

    assert_eq!(metrics.fps(), 0.0);
    assert_eq!(metrics.avg_frame_time_ms(), 0.0);
    assert_eq!(metrics.avg_layout_time_ms(), 0.0);
    assert_eq!(metrics.avg_render_time_ms(), 0.0);

    // reset also forgets the last frame start
    metrics.start_frame();
    assert_eq!(metrics.avg_frame_time_ms(), 0.0);
}

#[test]
fn test_perf_metrics_max_samples() {
    let mut metrics = PerfMetrics::new();
    for _ in 0..40 {
        metrics.record_layout(Duration::from_millis(100));
    }
    for _ in 0..60 {
        metrics.record_layout(Duration::from_millis(10));
    }
    // Only the last 60 samples count
    assert_eq!(metrics.avg_layout_time_ms(), 10.0);
}

#[test]
fn test_perf_metrics_zero_duration() {
    let mut metrics = PerfMetrics::new();
    metrics.record_layout(Duration::ZERO);
    metrics.record_render(Duration::ZERO);
    assert_eq!(metrics.avg_layout_time_ms(), 0.0);
    assert_eq!(metrics.avg_render_time_ms(), 0.0);
}

#[test]
fn test_perf_metrics_very_long_duration() {
    let mut metrics = PerfMetrics::new();
    metrics.record_layout(Duration::from_secs(10));
    metrics.record_render(Duration::from_secs(5));
    assert_eq!(metrics.avg_layout_time_ms(), 10000.0);
    assert_eq!(metrics.avg_render_time_ms(), 5000.0);
}

// =============================================================================
// EventLog
// =============================================================================

#[test]
fn test_event_log_default_keeps_events() {
    let mut log = EventLog::default();
    log.log(DebugEvent::KeyPress("a".to_string()));
    assert_eq!(log.recent(10).count(), 1);
}

#[test]
fn test_event_log_recent_limit() {
    let mut log = EventLog::new();
    for i in 0..10 {
        log.log(DebugEvent::KeyPress(i.to_string()));
    }
    let recent: Vec<_> = log.recent(5).map(|(_, e)| event_text(e)).collect();
    assert_eq!(recent, ["9", "8", "7", "6", "5"]);
}

#[test]
fn test_event_log_recent_order() {
    let mut log = EventLog::new();
    log.log(DebugEvent::KeyPress("first".to_string()));
    log.log(DebugEvent::Mouse("second".to_string()));
    log.log(DebugEvent::Custom("third".to_string()));

    let events: Vec<_> = log.recent(10).map(|(_, e)| e).collect();
    assert!(matches!(events[0], DebugEvent::Custom(s) if s == "third"));
    assert!(matches!(events[1], DebugEvent::Mouse(s) if s == "second"));
    assert!(matches!(events[2], DebugEvent::KeyPress(s) if s == "first"));
}

#[test]
fn test_event_log_max_events() {
    let mut log = EventLog::new();
    for i in 0..100 {
        log.log(DebugEvent::KeyPress(i.to_string()));
    }
    // Only the last 50 are kept
    let kept: Vec<_> = log.recent(usize::MAX).map(|(_, e)| event_text(e)).collect();
    assert_eq!(kept.len(), 50);
    assert_eq!(kept[0], "99");
    assert_eq!(kept[49], "50");
}

#[test]
fn test_event_log_request_more_than_available() {
    let mut log = EventLog::new();
    log.log(DebugEvent::KeyPress("a".to_string()));
    log.log(DebugEvent::KeyPress("b".to_string()));
    assert_eq!(log.recent(100).count(), 2);
}

#[test]
fn test_debug_event_clone() {
    let event = DebugEvent::StateChange("focus".to_string());
    let cloned = event.clone();
    assert!(matches!(cloned, DebugEvent::StateChange(ref s) if s == "focus"));
    assert_eq!(event_text(&event), event_text(&cloned));
}

// =============================================================================
// WidgetInfo
// =============================================================================

#[test]
fn test_widget_info_id() {
    let info = WidgetInfo::new("Button").id("submit");
    assert_eq!(info.id, Some("submit".to_string()));
    assert_eq!(info.tree_line(), "Button #submit");
}

#[test]
fn test_widget_info_class() {
    let info = WidgetInfo::new("Button").class("primary").class("large");
    assert_eq!(info.classes, ["primary", "large"]);
    assert_eq!(info.tree_line(), "Button .primary .large");
}

#[test]
fn test_widget_info_rect() {
    let rect = Rect::new(5, 10, 20, 5);
    let info = WidgetInfo::new("Button").rect(rect);
    assert_eq!(info.rect, rect);
}

#[test]
fn test_widget_info_tree_line_indent() {
    assert_eq!(WidgetInfo::new("Root").depth(0).tree_line(), "Root");
    assert_eq!(WidgetInfo::new("Child").depth(1).tree_line(), "  Child");
    assert_eq!(
        WidgetInfo::new("GrandChild").depth(2).tree_line(),
        "    GrandChild"
    );
}

#[test]
fn test_widget_info_all_attributes() {
    let mut info = WidgetInfo::new("Complete")
        .id("test-id")
        .class("class1")
        .class("class2")
        .depth(1)
        .rect(Rect::new(5, 5, 10, 3));
    info.focused = true;
    info.hovered = true;

    assert_eq!(
        info.tree_line(),
        "  Complete #test-id .class1 .class2 [focused] [hover]"
    );
}

// =============================================================================
// Global debug state
// =============================================================================

#[test]
#[serial]
#[allow(deprecated)]
fn test_global_debug_multiple_enables() {
    disable_debug();
    enable_debug();
    enable_debug();
    enable_debug();
    assert!(is_debug_enabled());
    disable_debug();
}

#[test]
#[serial]
#[allow(deprecated)]
fn test_global_debug_multiple_disables() {
    enable_debug();
    disable_debug();
    disable_debug();
    disable_debug();
    assert!(!is_debug_enabled());
}
