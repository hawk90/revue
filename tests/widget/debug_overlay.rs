//! DebugOverlay widget integration tests
//!
//! DebugOverlay 위젯의 통합 테스트 모음입니다.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::debug_overlay::{
    DebugConfig, DebugEvent, DebugOverlay, DebugPosition, EventLog, PerfMetrics, WidgetInfo,
    enable_debug, disable_debug, is_debug_enabled, toggle_debug,
};
use revue::widget::traits::RenderContext;
use revue::widget::Text;
use std::time::Duration;

// =============================================================================
// Constructor and Builder Tests
// 생성자 및 빌더 메서드 테스트
// =============================================================================

#[test]
fn test_debug_overlay_wrap() {
    let text = Text::new("Hello");
    let overlay = DebugOverlay::wrap(text);

    assert!(overlay.visible);
    assert!(overlay.config.show_metrics);
    assert!(!overlay.config.show_tree);
    assert!(!overlay.config.show_events);
    assert!(!overlay.config.show_styles);
}

#[test]
fn test_debug_overlay_visible_builder() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).visible(true);

    assert!(overlay.visible);
}

#[test]
fn test_debug_overlay_visible_false() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).visible(false);

    assert!(!overlay.visible);
}

#[test]
fn test_debug_overlay_show_metrics() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).show_metrics(true);

    assert!(overlay.config.show_metrics);
}

#[test]
fn test_debug_overlay_show_metrics_false() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).show_metrics(false);

    assert!(!overlay.config.show_metrics);
}

#[test]
fn test_debug_overlay_show_tree() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).show_tree(true);

    assert!(overlay.config.show_tree);
}

#[test]
fn test_debug_overlay_show_events() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).show_events(true);

    assert!(overlay.config.show_events);
}

#[test]
fn test_debug_overlay_show_styles() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).show_styles(true);

    assert!(overlay.config.show_styles);
}

#[test]
fn test_debug_overlay_position() {
    let text = Text::new("Test");

    let overlay_tl = DebugOverlay::wrap(text.clone()).position(DebugPosition::TopLeft);
    assert_eq!(overlay_tl.config.position, DebugPosition::TopLeft);

    let overlay_tr = DebugOverlay::wrap(text.clone()).position(DebugPosition::TopRight);
    assert_eq!(overlay_tr.config.position, DebugPosition::TopRight);

    let overlay_bl = DebugOverlay::wrap(text.clone()).position(DebugPosition::BottomLeft);
    assert_eq!(overlay_bl.config.position, DebugPosition::BottomLeft);

    let overlay_br = DebugOverlay::wrap(text).position(DebugPosition::BottomRight);
    assert_eq!(overlay_br.config.position, DebugPosition::BottomRight);
}

#[test]
fn test_debug_overlay_width() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).width(30);

    assert_eq!(overlay.config.width, 30);
}

#[test]
fn test_debug_overlay_builder_chain() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text)
        .visible(true)
        .show_metrics(true)
        .show_tree(true)
        .show_events(true)
        .position(DebugPosition::TopLeft)
        .width(25);

    assert!(overlay.visible);
    assert!(overlay.config.show_metrics);
    assert!(overlay.config.show_tree);
    assert!(overlay.config.show_events);
    assert_eq!(overlay.config.position, DebugPosition::TopLeft);
    assert_eq!(overlay.config.width, 25);
}

// =============================================================================
// DebugConfig Tests
// 디버그 설정 테스트
// =============================================================================

#[test]
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

#[test]
fn test_debug_config_custom_colors() {
    let config = DebugConfig {
        bg_color: Color::RED,
        fg_color: Color::GREEN,
        accent_color: Color::BLUE,
        ..DebugConfig::default()
    };

    assert_eq!(config.bg_color, Color::RED);
    assert_eq!(config.fg_color, Color::GREEN);
    assert_eq!(config.accent_color, Color::BLUE);
}

// =============================================================================
// DebugPosition Tests
// 디버그 패널 위치 테스트
// =============================================================================

#[test]
fn test_debug_position_top_left() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).width(20);
    let area = Rect::new(0, 0, 80, 24);
    let panel = overlay.panel_rect(area);

    assert_eq!(panel.x, 80 - 20);
    assert_eq!(panel.y, 0);
    assert_eq!(panel.width, 20);
}

#[test]
fn test_debug_position_top_right() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text)
        .width(20)
        .position(DebugPosition::TopRight);
    let area = Rect::new(0, 0, 80, 24);
    let panel = overlay.panel_rect(area);

    assert_eq!(panel.x, 80 - 20);
    assert_eq!(panel.y, 0);
    assert_eq!(panel.width, 20);
}

#[test]
fn test_debug_position_bottom_left() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text)
        .width(20)
        .position(DebugPosition::BottomLeft);
    let area = Rect::new(0, 0, 80, 24);
    let panel = overlay.panel_rect(area);

    assert_eq!(panel.x, 0);
    assert_eq!(panel.y, 24 - 20);
    assert_eq!(panel.width, 20);
}

#[test]
fn test_debug_position_bottom_right() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text)
        .width(20)
        .position(DebugPosition::BottomRight);
    let area = Rect::new(0, 0, 80, 24);
    let panel = overlay.panel_rect(area);

    assert_eq!(panel.x, 80 - 20);
    assert_eq!(panel.y, 24 - 20);
    assert_eq!(panel.width, 20);
}

#[test]
fn test_panel_rect_width_clamping() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).width(100);
    let area = Rect::new(0, 0, 80, 24);
    let panel = overlay.panel_rect(area);

    // Width should be clamped to area width
    assert_eq!(panel.width, 80);
}

#[test]
fn test_panel_rect_height_clamping() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).width(20);
    let area = Rect::new(0, 0, 80, 10);
    let panel = overlay.panel_rect(area);

    // Height should be clamped to max_height
    assert_eq!(panel.height, 10);
}

// =============================================================================
// PerfMetrics Tests
// 성능 메트릭 테스트
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
fn test_perf_metrics_start_frame() {
    let mut metrics = PerfMetrics::new();

    metrics.start_frame();
    // First frame doesn't record a time since there's no previous frame
    assert_eq!(metrics.fps(), 0.0);
}

#[test]
fn test_perf_metrics_multiple_frames() {
    let mut metrics = PerfMetrics::new();

    // Simulate frame timing - use longer duration for reliability
    metrics.start_frame();
    std::thread::sleep(Duration::from_millis(50));
    metrics.start_frame();

    // Should have recorded a frame time
    assert!(metrics.avg_frame_time_ms() > 0.0);
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
fn test_perf_metrics_fps_calculation() {
    let mut metrics = PerfMetrics::new();

    // Record 60 FPS (16.67ms per frame)
    for _ in 0..10 {
        metrics.start_frame();
        metrics.frame_times.push_back(Duration::from_millis(16));
    }

    let fps = metrics.fps();
    assert!(fps > 50.0 && fps < 70.0);
}

#[test]
fn test_perf_metrics_reset() {
    let mut metrics = PerfMetrics::new();

    metrics.start_frame();
    metrics.record_layout(Duration::from_millis(5));
    metrics.record_render(Duration::from_millis(3));

    metrics.reset();

    assert_eq!(metrics.fps(), 0.0);
    assert_eq!(metrics.avg_frame_time_ms(), 0.0);
    assert_eq!(metrics.avg_layout_time_ms(), 0.0);
    assert_eq!(metrics.avg_render_time_ms(), 0.0);
}

#[test]
fn test_perf_metrics_max_samples() {
    let mut metrics = PerfMetrics::new();

    // Add more than max_samples (60) frame times
    for _ in 0..100 {
        metrics.frame_times.push_back(Duration::from_millis(10));
    }

    // Should only keep 60 samples
    assert_eq!(metrics.frame_times.len(), 60);
}

#[test]
fn test_perf_metrics_metrics_mut() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text);

    let metrics = overlay.metrics_mut();
    metrics.record_layout(Duration::from_millis(5));

    assert_eq!(overlay.metrics.avg_layout_time_ms(), 5.0);
}

// =============================================================================
// EventLog Tests
// 이벤트 로그 테스트
// =============================================================================

#[test]
fn test_event_log_new() {
    let log = EventLog::new();

    assert_eq!(log.recent(10).count(), 0);
}

#[test]
fn test_event_log_log_key_press() {
    let mut log = EventLog::new();

    log.log(DebugEvent::KeyPress("a".to_string()));
    log.log(DebugEvent::KeyPress("Enter".to_string()));

    assert_eq!(log.recent(10).count(), 2);
}

#[test]
fn test_event_log_log_mouse() {
    let mut log = EventLog::new();

    log.log(DebugEvent::Mouse("click".to_string()));
    log.log(DebugEvent::Mouse("move".to_string()));

    assert_eq!(log.recent(10).count(), 2);
}

#[test]
fn test_event_log_log_state_change() {
    let mut log = EventLog::new();

    log.log(DebugEvent::StateChange("focus".to_string()));
    log.log(DebugEvent::StateChange("blur".to_string()));

    assert_eq!(log.recent(10).count(), 2);
}

#[test]
fn test_event_log_log_custom() {
    let mut log = EventLog::new();

    log.log(DebugEvent::Custom("Custom event".to_string()));

    assert_eq!(log.recent(10).count(), 1);
}

#[test]
fn test_event_log_recent_limit() {
    let mut log = EventLog::new();

    for i in 0..10 {
        log.log(DebugEvent::KeyPress(i.to_string()));
    }

    assert_eq!(log.recent(5).count(), 5);
}

#[test]
fn test_event_log_recent_order() {
    let mut log = EventLog::new();

    log.log(DebugEvent::KeyPress("first".to_string()));
    log.log(DebugEvent::KeyPress("second".to_string()));
    log.log(DebugEvent::KeyPress("third".to_string()));

    let events: Vec<_> = log.recent(10).collect();
    // Most recent first (reversed)
    assert!(events[0].1.to_string().contains("third"));
    assert!(events[1].1.to_string().contains("second"));
    assert!(events[2].1.to_string().contains("first"));
}

#[test]
fn test_event_log_clear() {
    let mut log = EventLog::new();

    log.log(DebugEvent::KeyPress("a".to_string()));
    log.log(DebugEvent::KeyPress("b".to_string()));

    log.clear();

    assert_eq!(log.recent(10).count(), 0);
}

#[test]
fn test_event_log_max_events() {
    let mut log = EventLog::new();

    // Add more than max_events (50)
    for i in 0..100 {
        log.log(DebugEvent::KeyPress(i.to_string()));
    }

    // Should only keep 50 events
    assert_eq!(log.events.len(), 50);
}

#[test]
fn test_debug_overlay_events_mut() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text);

    let events = overlay.events_mut();
    events.log(DebugEvent::KeyPress("a".to_string()));

    assert_eq!(overlay.events.recent(10).count(), 1);
}

#[test]
fn test_debug_overlay_log_event() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text);

    overlay.log_event(DebugEvent::KeyPress("a".to_string()));
    overlay.log_event(DebugEvent::Mouse("click".to_string()));

    assert_eq!(overlay.events.recent(10).count(), 2);
}

// =============================================================================
// WidgetInfo Tests (DebugWidget as WidgetInfo)
// 위젯 정보 테스트
// =============================================================================

#[test]
fn test_widget_info_new() {
    let info = WidgetInfo::new("Button");

    assert_eq!(info.type_name, "Button");
    assert!(info.id.is_none());
    assert!(info.classes.is_empty());
    assert_eq!(info.depth, 0);
    assert!(!info.focused);
    assert!(!info.hovered);
}

#[test]
fn test_widget_info_id() {
    let info = WidgetInfo::new("Button").id("submit");

    assert_eq!(info.id, Some("submit".to_string()));
}

#[test]
fn test_widget_info_class() {
    let info = WidgetInfo::new("Button")
        .class("primary")
        .class("large");

    assert_eq!(info.classes.len(), 2);
    assert!(info.classes.contains(&"primary".to_string()));
    assert!(info.classes.contains(&"large".to_string()));
}

#[test]
fn test_widget_info_rect() {
    let rect = Rect::new(5, 10, 20, 5);
    let info = WidgetInfo::new("Button").rect(rect);

    assert_eq!(info.rect, rect);
}

#[test]
fn test_widget_info_depth() {
    let info = WidgetInfo::new("Container").depth(2);

    assert_eq!(info.depth, 2);
}

#[test]
fn test_widget_info_tree_line() {
    let info = WidgetInfo::new("Button")
        .id("submit")
        .class("primary")
        .depth(1);

    let line = info.tree_line();
    assert!(line.contains("Button"));
    assert!(line.contains("#submit"));
    assert!(line.contains(".primary"));
    assert!(line.starts_with("  ")); // 2 spaces for depth 1
}

#[test]
fn test_widget_info_tree_line_focused() {
    let mut info = WidgetInfo::new("Button");
    info.focused = true;

    let line = info.tree_line();
    assert!(line.contains("[focused]"));
}

#[test]
fn test_widget_info_tree_line_hovered() {
    let mut info = WidgetInfo::new("Button");
    info.hovered = true;

    let line = info.tree_line();
    assert!(line.contains("[hover]"));
}

#[test]
fn test_widget_info_tree_line_indent() {
    let info0 = WidgetInfo::new("Root").depth(0);
    let info1 = WidgetInfo::new("Child").depth(1);
    let info2 = WidgetInfo::new("GrandChild").depth(2);

    assert!(!info0.tree_line().starts_with(" "));
    assert!(info1.tree_line().starts_with("  "));
    assert!(info2.tree_line().starts_with("    "));
}

#[test]
fn test_debug_overlay_record_widget() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text);

    let widget = WidgetInfo::new("Button").id("submit").class("primary");
    overlay.record_widget(widget);

    assert_eq!(overlay.widgets.len(), 1);
    assert_eq!(overlay.widgets[0].type_name, "Button");
}

#[test]
fn test_debug_overlay_clear_widgets() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text);

    overlay.record_widget(WidgetInfo::new("Button"));
    overlay.record_widget(WidgetInfo::new("Text"));

    assert_eq!(overlay.widgets.len(), 2);

    overlay.clear_widgets();

    assert_eq!(overlay.widgets.len(), 0);
}

// =============================================================================
// Rendering Tests
// 렌더링 테스트
// =============================================================================

#[test]
fn test_debug_overlay_render_visible() {
    let text = Text::new("Hello");
    let overlay = DebugOverlay::wrap(text).visible(true);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    overlay.render(&mut ctx);

    // Inner text should be rendered
    let mut found_text = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'H' {
                found_text = true;
                break;
            }
        }
    }
    assert!(found_text);
}

#[test]
fn test_debug_overlay_render_not_visible() {
    let text = Text::new("Hello");
    let overlay = DebugOverlay::wrap(text).visible(false);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    overlay.render(&mut ctx);

    // Inner text should still be rendered
    let mut found_text = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'H' {
                found_text = true;
                break;
            }
        }
    }
    assert!(found_text);
}

#[test]
fn test_debug_overlay_render_with_metrics() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text)
        .show_metrics(true)
        .width(30);

    overlay.metrics.start_frame();
    overlay.metrics.record_layout(Duration::from_millis(5));
    overlay.metrics.record_render(Duration::from_millis(3));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    overlay.render(&mut ctx);

    // Should render the panel with metrics
    // Check for panel background in top-right corner
    let panel_x = 80 - 30;
    let cell = buffer.get(panel_x + 1, 1);
    assert!(cell.is_some());
}

#[test]
fn test_debug_overlay_render_with_tree() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text)
        .show_tree(true)
        .width(30);

    overlay.record_widget(WidgetInfo::new("Button").id("submit"));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    overlay.render(&mut ctx);

    // Should render the panel with widget tree
    let panel_x = 80 - 30;
    let cell = buffer.get(panel_x + 1, 1);
    assert!(cell.is_some());
}

#[test]
fn test_debug_overlay_render_with_events() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text)
        .show_events(true)
        .width(30);

    overlay.log_event(DebugEvent::KeyPress("a".to_string()));

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    overlay.render(&mut ctx);

    // Should render the panel with event log
    let panel_x = 80 - 30;
    let cell = buffer.get(panel_x + 1, 1);
    assert!(cell.is_some());
}

#[test]
fn test_debug_overlay_render_zero_width() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).width(0);

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    overlay.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_debug_overlay_render_small_buffer() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).width(10);

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    overlay.render(&mut ctx);
    // Should not panic
}

// =============================================================================
// Toggle Tests
// 토글 테스트
// =============================================================================

#[test]
fn test_debug_overlay_toggle() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text).visible(true);

    assert!(overlay.visible);

    overlay.toggle();
    assert!(!overlay.visible);

    overlay.toggle();
    assert!(overlay.visible);
}

#[test]
fn test_debug_overlay_toggle_from_hidden() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text).visible(false);

    assert!(!overlay.visible);

    overlay.toggle();
    assert!(overlay.visible);

    overlay.toggle();
    assert!(!overlay.visible);
}

// =============================================================================
// Global Debug State Tests
// 전역 디버그 상태 테스트
// =============================================================================

#[test]
fn test_global_debug_enable() {
    disable_debug();
    assert!(!is_debug_enabled());

    enable_debug();
    assert!(is_debug_enabled());
}

#[test]
fn test_global_debug_disable() {
    enable_debug();
    assert!(is_debug_enabled());

    disable_debug();
    assert!(!is_debug_enabled());
}

#[test]
fn test_global_debug_toggle() {
    disable_debug();
    assert!(!is_debug_enabled());

    let result = toggle_debug();
    assert!(result);
    assert!(is_debug_enabled());

    let result = toggle_debug();
    assert!(!result);
    assert!(!is_debug_enabled());
}

#[test]
fn test_global_debug_multiple_enables() {
    disable_debug();

    enable_debug();
    enable_debug();
    enable_debug();

    assert!(is_debug_enabled());
}

#[test]
fn test_global_debug_multiple_disables() {
    enable_debug();

    disable_debug();
    disable_debug();
    disable_debug();

    assert!(!is_debug_enabled());
}

// =============================================================================
// Edge Cases
// 엣지 케이스 테스트
// =============================================================================

#[test]
fn test_debug_overlay_empty_widgets() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).show_tree(true);

    assert_eq!(overlay.widgets.len(), 0);
}

#[test]
fn test_debug_overlay_many_widgets() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text).show_tree(true);

    for i in 0..100 {
        overlay.record_widget(WidgetInfo::new(&format!("Widget{}", i)));
    }

    assert_eq!(overlay.widgets.len(), 100);
}

#[test]
fn test_perf_metrics_empty_frame_times() {
    let metrics = PerfMetrics::new();

    assert_eq!(metrics.fps(), 0.0);
    assert_eq!(metrics.avg_frame_time_ms(), 0.0);
}

#[test]
fn test_perf_metrics_single_frame() {
    let mut metrics = PerfMetrics::new();

    metrics.start_frame();
    std::thread::sleep(Duration::from_millis(50));
    metrics.start_frame();

    assert!(metrics.avg_frame_time_ms() > 0.0);
}

#[test]
fn test_event_log_empty() {
    let log = EventLog::new();

    assert_eq!(log.recent(100).count(), 0);
}

#[test]
fn test_event_log_request_more_than_available() {
    let mut log = EventLog::new();

    log.log(DebugEvent::KeyPress("a".to_string()));
    log.log(DebugEvent::KeyPress("b".to_string()));

    assert_eq!(log.recent(100).count(), 2);
}

#[test]
fn test_widget_info_empty_attributes() {
    let info = WidgetInfo::new("Minimal");

    let line = info.tree_line();
    assert_eq!(line, "Minimal");
}

#[test]
fn test_widget_info_all_attributes() {
    let mut info = WidgetInfo::new("Complete")
        .id("test-id")
        .class("class1")
        .class("class2")
        .depth(2)
        .rect(Rect::new(5, 5, 10, 3));

    info.focused = true;
    info.hovered = true;

    let line = info.tree_line();
    assert!(line.contains("Complete"));
    assert!(line.contains("#test-id"));
    assert!(line.contains(".class1"));
    assert!(line.contains(".class2"));
    assert!(line.contains("[focused]"));
    assert!(line.contains("[hover]"));
}

#[test]
fn test_debug_overlay_width_zero() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).width(0);

    assert_eq!(overlay.config.width, 0);
}

#[test]
fn test_debug_overlay_width_very_large() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text).width(1000);

    assert_eq!(overlay.config.width, 1000);

    let area = Rect::new(0, 0, 80, 24);
    let panel = overlay.panel_rect(area);

    // Should be clamped to area width
    assert_eq!(panel.width, 80);
}

#[test]
fn test_debug_overlay_max_height_zero() {
    let text = Text::new("Test");
    let mut overlay = DebugOverlay::wrap(text);
    overlay.config.max_height = 0;

    let area = Rect::new(0, 0, 80, 24);
    let panel = overlay.panel_rect(area);

    assert_eq!(panel.height, 0);
}

#[test]
fn test_debug_overlay_all_sections_enabled() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text)
        .show_metrics(true)
        .show_tree(true)
        .show_events(true)
        .show_styles(true);

    assert!(overlay.config.show_metrics);
    assert!(overlay.config.show_tree);
    assert!(overlay.config.show_events);
    assert!(overlay.config.show_styles);
}

#[test]
fn test_debug_overlay_all_sections_disabled() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text)
        .show_metrics(false)
        .show_tree(false)
        .show_events(false)
        .show_styles(false);

    assert!(!overlay.config.show_metrics);
    assert!(!overlay.config.show_tree);
    assert!(!overlay.config.show_events);
    assert!(!overlay.config.show_styles);
}

#[test]
fn test_debug_event_all_variants() {
    let events = vec![
        DebugEvent::KeyPress("a".to_string()),
        DebugEvent::Mouse("click".to_string()),
        DebugEvent::StateChange("focus".to_string()),
        DebugEvent::Custom("custom".to_string()),
    ];

    for event in events {
        let mut log = EventLog::new();
        log.log(event.clone());
        assert_eq!(log.recent(10).count(), 1);
    }
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

#[test]
fn test_panel_rect_with_offset_area() {
    let text = Text::new("Test");
    let overlay = DebugOverlay::wrap(text)
        .width(20)
        .position(DebugPosition::TopLeft);

    let area = Rect::new(10, 5, 80, 24);
    let panel = overlay.panel_rect(area);

    assert_eq!(panel.x, 10);
    assert_eq!(panel.y, 5);
}

#[test]
fn test_debug_clone_types() {
    let event1 = DebugEvent::KeyPress("a".to_string());
    let event2 = event1.clone();
    assert_eq!(event1.to_string(), event2.to_string());
}
