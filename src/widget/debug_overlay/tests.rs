#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::Text;

    #[test]
    fn test_perf_metrics() {
        let mut metrics = PerfMetrics::new();
        metrics.start_frame();
        metrics.record_layout(Duration::from_millis(1));
        metrics.record_render(Duration::from_millis(2));

        assert!(metrics.avg_layout_time_ms() > 0.0);
        assert!(metrics.avg_render_time_ms() > 0.0);
    }

    #[test]
    fn test_event_log() {
        let mut log = EventLog::new();
        log.log(DebugEvent::KeyPress("a".to_string()));
        log.log(DebugEvent::Mouse("click".to_string()));

        assert_eq!(log.recent(10).count(), 2);
    }

    #[test]
    fn test_widget_info() {
        let info = WidgetInfo::new("Button")
            .id("submit")
            .class("primary")
            .depth(1);

        let line = info.tree_line();
        assert!(line.contains("Button"));
        assert!(line.contains("#submit"));
        assert!(line.contains(".primary"));
    }

    #[test]
    fn test_debug_overlay() {
        let text = Text::new("Hello");
        let overlay = DebugOverlay::wrap(text)
            .show_metrics(true)
            .show_tree(true)
            .position(DebugPosition::TopRight)
            .width(30);

        assert!(overlay.visible);
        assert!(overlay.config.show_metrics);
        assert!(overlay.config.show_tree);
    }

    #[test]
    fn test_debug_config_default() {
        let config = DebugConfig::default();
        assert!(config.show_metrics);
        assert!(!config.show_tree);
        assert!(!config.show_events);
        assert_eq!(config.width, 40);
    }

    #[test]
    fn test_global_debug_state() {
        disable_debug();
        assert!(!is_debug_enabled());

        enable_debug();
        assert!(is_debug_enabled());

        toggle_debug();
        assert!(!is_debug_enabled());
    }

    #[test]
    fn test_panel_rect_positions() {
        let text = Text::new("test");
        let overlay = DebugOverlay::wrap(text)
            .width(20)
            .position(DebugPosition::TopLeft);

        let area = Rect::new(0, 0, 80, 24);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 0);
        assert_eq!(panel.y, 0);
        assert_eq!(panel.width, 20);
    }
}
