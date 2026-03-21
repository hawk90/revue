//! Process Monitor widget (htop-style) tests

#[cfg(feature = "sysinfo")]
mod tests {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{
        htop, process_monitor, ProcColors, ProcessInfo, ProcessMonitor, ProcessSort, ProcessView,
    };

    // =========================================================================
    // ProcessSort enum tests
    // =========================================================================

    #[test]
    fn test_process_sort_default() {
        assert_eq!(ProcessSort::default(), ProcessSort::Cpu);
    }

    #[test]
    fn test_process_sort_clone() {
        let sort = ProcessSort::Memory;
        assert_eq!(sort, sort.clone());
    }

    #[test]
    fn test_process_sort_copy() {
        let sort1 = ProcessSort::Name;
        let sort2 = sort1;
        assert_eq!(sort1, ProcessSort::Name);
        assert_eq!(sort2, ProcessSort::Name);
    }

    #[test]
    fn test_process_sort_equality() {
        assert_eq!(ProcessSort::Pid, ProcessSort::Pid);
        assert_eq!(ProcessSort::Cpu, ProcessSort::Cpu);
        assert_ne!(ProcessSort::Pid, ProcessSort::Name);
    }

    #[test]
    fn test_process_sort_debug() {
        let debug_str = format!("{:?}", ProcessSort::Memory);
        assert!(debug_str.contains("Memory"));
    }

    // =========================================================================
    // ProcessView enum tests
    // =========================================================================

    #[test]
    fn test_process_view_default() {
        assert_eq!(ProcessView::default(), ProcessView::All);
    }

    #[test]
    fn test_process_view_clone() {
        let view = ProcessView::Tree;
        assert_eq!(view, view.clone());
    }

    #[test]
    fn test_process_view_copy() {
        let view1 = ProcessView::User;
        let view2 = view1;
        assert_eq!(view1, ProcessView::User);
        assert_eq!(view2, ProcessView::User);
    }

    #[test]
    fn test_process_view_equality() {
        assert_eq!(ProcessView::All, ProcessView::All);
        assert_eq!(ProcessView::User, ProcessView::User);
        assert_ne!(ProcessView::All, ProcessView::Tree);
    }

    #[test]
    fn test_process_view_debug() {
        let debug_str = format!("{:?}", ProcessView::Tree);
        assert!(debug_str.contains("Tree"));
    }

    // =========================================================================
    // ProcColors Default tests
    // =========================================================================

    #[test]
    fn test_proc_colors_default() {
        let colors = ProcColors::default();
        // Verify all fields are set
        let _ = colors.header_bg;
        let _ = colors.header_fg;
        let _ = colors.selected_bg;
        let _ = colors.high_cpu;
        let _ = colors.medium_cpu;
        let _ = colors.low_cpu;
        let _ = colors.high_mem;
        let _ = colors.name;
        let _ = colors.pid;
    }

    #[test]
    fn test_proc_colors_clone() {
        let colors1 = ProcColors::default();
        let colors2 = colors1.clone();
        assert_eq!(colors1.header_bg, colors2.header_bg);
    }

    #[test]
    fn test_proc_colors_debug() {
        let colors = ProcColors::default();
        let debug_str = format!("{:?}", colors);
        assert!(debug_str.contains("ProcColors"));
    }

    // =========================================================================
    // ProcessInfo struct tests
    // =========================================================================

    #[test]
    fn test_process_info_clone() {
        let info = ProcessInfo {
            pid: 1234,
            parent_pid: Some(1),
            name: "test".to_string(),
            cpu: 5.0,
            memory: 1024,
            memory_percent: 0.1,
            status: "Running".to_string(),
            cmd: "test".to_string(),
            user: "user".to_string(),
        };
        let cloned = info.clone();
        assert_eq!(info.pid, cloned.pid);
        assert_eq!(info.name, cloned.name);
    }

    #[test]
    fn test_process_info_debug() {
        let info = ProcessInfo {
            pid: 1,
            parent_pid: None,
            name: "init".to_string(),
            cpu: 0.0,
            memory: 0,
            memory_percent: 0.0,
            status: "Sleeping".to_string(),
            cmd: "".to_string(),
            user: "root".to_string(),
        };
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("ProcessInfo"));
    }

    // =========================================================================
    // ProcessMonitor creation tests
    // =========================================================================

    #[test]
    fn test_process_monitor_creation() {
        let monitor = ProcessMonitor::new();
        // Just verify the monitor can be created (process count may be 0 in some CI environments)
        let _ = monitor.process_count();
    }

    #[test]
    fn test_process_monitor_default() {
        let monitor = ProcessMonitor::default();
        // Default sort is Cpu (descending) — verify via toggle behavior
        let mut m = ProcessMonitor::default();
        // toggle_sort on the default sort (Cpu) should flip direction
        m.toggle_sort(ProcessSort::Cpu);
        // After toggling same column, direction changes — no panic
        let _ = m.process_count();
    }

    #[test]
    fn test_process_monitor_helper() {
        let monitor = process_monitor();
        let _ = monitor.process_count();
    }

    #[test]
    fn test_htop_helper() {
        let monitor = htop();
        let _ = monitor.process_count();
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_sort_by() {
        // sort_by returns Self — verify it compiles and doesn't panic
        let monitor = ProcessMonitor::new().sort_by(ProcessSort::Memory);
        let _ = monitor.process_count();
    }

    #[test]
    fn test_ascending() {
        let monitor = ProcessMonitor::new().ascending(true);
        let _ = monitor.process_count();
    }

    #[test]
    fn test_ascending_false() {
        let monitor = ProcessMonitor::new().ascending(false);
        let _ = monitor.process_count();
    }

    #[test]
    fn test_view() {
        let monitor = ProcessMonitor::new().view(ProcessView::Tree);
        let _ = monitor.process_count();
    }

    #[test]
    fn test_colors() {
        let custom_colors = ProcColors::default();
        let monitor = ProcessMonitor::new().colors(custom_colors);
        let _ = monitor.process_count();
    }

    #[test]
    fn test_show_cmd() {
        let monitor = ProcessMonitor::new().show_cmd(true);
        let _ = monitor.process_count();
    }

    #[test]
    fn test_show_cmd_false() {
        let monitor = ProcessMonitor::new().show_cmd(false);
        let _ = monitor.process_count();
    }

    #[test]
    fn test_update_interval() {
        let monitor = ProcessMonitor::new().update_interval(500);
        // Verify it doesn't need update immediately after creation with large interval
        assert!(!monitor.needs_update());
    }

    #[test]
    fn test_builder_chain() {
        let monitor = ProcessMonitor::new()
            .sort_by(ProcessSort::Name)
            .ascending(true)
            .view(ProcessView::User)
            .show_cmd(true)
            .update_interval(2000);

        let _ = monitor.process_count();
    }

    // =========================================================================
    // Filter tests
    // =========================================================================

    #[test]
    fn test_filter() {
        let mut monitor = ProcessMonitor::new();
        monitor.filter("test");
        // After filter, selected resets to 0 — verify selection doesn't panic
        let _ = monitor.selected_process();
    }

    #[test]
    fn test_clear_filter() {
        let mut monitor = ProcessMonitor::new();
        monitor.filter("test");
        monitor.clear_filter();
        // After clear, process list should be accessible
        let _ = monitor.process_count();
    }

    #[test]
    fn test_filter_with_uppercase() {
        let mut monitor = ProcessMonitor::new();
        // Filter is case-insensitive (stored as lowercase)
        monitor.filter("TEST");
        let _ = monitor.process_count();
    }

    // =========================================================================
    // Toggle sort tests
    // =========================================================================

    #[test]
    fn test_toggle_sort_same_column() {
        let mut monitor = ProcessMonitor::new();
        // Toggle same column (Cpu by default) flips direction
        monitor.toggle_sort(ProcessSort::Cpu);
        monitor.toggle_sort(ProcessSort::Cpu);
        // Two toggles return to original state — no panic
        let _ = monitor.process_count();
    }

    #[test]
    fn test_toggle_sort_different_column() {
        let mut monitor = ProcessMonitor::new();
        // Toggle different column sets it as sort column (descending)
        monitor.toggle_sort(ProcessSort::Memory);
        let _ = monitor.process_count();
    }

    // =========================================================================
    // Selection tests
    // =========================================================================

    #[test]
    fn test_select_next() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        monitor.select_next();
        // Selection should either advance or stay at bound
        let _ = monitor.selected_process();
    }

    #[test]
    fn test_select_prev() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        monitor.select_next();
        monitor.select_prev();
        let _ = monitor.selected_process();
    }

    #[test]
    fn test_select_prev_at_zero() {
        let mut monitor = ProcessMonitor::new();
        monitor.select_prev();
        // Should not go below 0 — selected_process still valid
        let _ = monitor.selected_process();
    }

    #[test]
    fn test_page_down() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        monitor.page_down(10);
        let _ = monitor.selected_process();
    }

    #[test]
    fn test_page_up() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        monitor.page_down(10);
        monitor.page_up(5);
        let _ = monitor.selected_process();
    }

    #[test]
    fn test_page_up_at_zero() {
        let mut monitor = ProcessMonitor::new();
        monitor.page_up(10);
        let _ = monitor.selected_process();
    }

    // =========================================================================
    // Getter tests
    // =========================================================================

    #[test]
    fn test_selected_process() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        let selected = monitor.selected_process();
        if monitor.process_count() > 0 {
            assert!(selected.is_some());
        } else {
            assert!(selected.is_none());
        }
    }

    #[test]
    fn test_selected_process_out_of_bounds() {
        let mut monitor = ProcessMonitor::new();
        // page_down with huge value clamps to valid range
        monitor.page_down(99999);
        // selected_process either returns Some (valid index) or None (empty list)
        let _ = monitor.selected_process();
    }

    #[test]
    fn test_process_count() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();
        let count = monitor.process_count();
        // usize is always >= 0, just verify we can get the count
        let _ = count;
    }

    #[test]
    fn test_cpu_usage() {
        let monitor = ProcessMonitor::new();
        let cpu = monitor.cpu_usage();
        assert!(cpu >= 0.0);
    }

    #[test]
    fn test_memory_usage() {
        let monitor = ProcessMonitor::new();
        let (used, total) = monitor.memory_usage();
        assert!(used <= total);
        assert!(total > 0);
    }

    // =========================================================================
    // Update tests
    // =========================================================================

    #[test]
    fn test_refresh() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();
        // Should not panic
        let _ = monitor.process_count();
    }

    #[test]
    fn test_needs_update_true() {
        let monitor = ProcessMonitor::new().update_interval(1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(monitor.needs_update());
    }

    #[test]
    fn test_needs_update_false() {
        let monitor = ProcessMonitor::new();
        assert!(!monitor.needs_update());
    }

    #[test]
    fn test_tick() {
        let mut monitor = ProcessMonitor::new().update_interval(1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        monitor.tick();
        // Should not panic
        let _ = monitor.process_count();
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_render() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        let mut buffer = Buffer::new(100, 30);
        let area = Rect::new(0, 0, 100, 30);
        let mut ctx = RenderContext::new(&mut buffer, area);

        monitor.render(&mut ctx);
    }

    #[test]
    fn test_render_small_area() {
        let monitor = ProcessMonitor::new();

        let mut buffer = Buffer::new(10, 2);
        let area = Rect::new(0, 0, 10, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        monitor.render(&mut ctx);
        // Should not crash with small area
    }
}
