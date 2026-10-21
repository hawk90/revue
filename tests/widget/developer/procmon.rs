//! Process Monitor widget (htop-style) tests

#[cfg(feature = "sysinfo")]
mod tests {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::Color;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{
        htop, process_monitor, ProcColors, ProcessInfo, ProcessMonitor, ProcessSort, ProcessView,
    };

    /// Draw `monitor` into a `w`x`h` buffer
    fn screen(monitor: &ProcessMonitor, w: u16, h: u16) -> Buffer {
        let mut buffer = Buffer::new(w, h);
        monitor.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, w, h)));
        buffer
    }

    /// Row `y` of `buffer` as text
    fn row_text(buffer: &Buffer, y: u16) -> String {
        (0..buffer.width())
            .map(|x| buffer.get(x, y).unwrap().symbol)
            .collect()
    }

    /// The listed processes in order, read by moving the selection through
    /// them; the selection ends on the first
    fn listed(monitor: &mut ProcessMonitor) -> Vec<ProcessInfo> {
        monitor.page_up(usize::MAX);
        let mut list = Vec::new();
        for _ in 0..monitor.process_count() {
            list.push(monitor.selected_process().unwrap().clone());
            monitor.select_next();
        }
        monitor.page_up(usize::MAX);
        list
    }

    /// The position of the highlighted row when every process fits on screen
    fn selected_index(monitor: &ProcessMonitor) -> Option<usize> {
        let height = monitor.process_count() as u16 + 2;
        let buffer = screen(monitor, 60, height.max(5));
        (2..height)
            .find(|&y| buffer.get(0, y).unwrap().bg == Some(Color::rgb(60, 80, 120)))
            .map(|y| y as usize - 2)
    }

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
    #[allow(deprecated)] // ProcessView::Tree
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
    #[allow(deprecated)] // ProcessView::Tree
    fn test_process_view_equality() {
        assert_eq!(ProcessView::All, ProcessView::All);
        assert_eq!(ProcessView::User, ProcessView::User);
        assert_ne!(ProcessView::All, ProcessView::Tree);
    }

    #[test]
    #[allow(deprecated)] // ProcessView::Tree
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
        assert_eq!(colors.header_bg, Color::rgb(40, 40, 60));
        assert_eq!(colors.header_fg, Color::WHITE);
        assert_eq!(colors.selected_bg, Color::rgb(60, 80, 120));
        assert_eq!(colors.high_cpu, Color::RED);
        assert_eq!(colors.medium_cpu, Color::YELLOW);
        assert_eq!(colors.low_cpu, Color::GREEN);
        assert_eq!(colors.high_mem, Color::MAGENTA);
        // No name color: the stylesheet decides, then white
        assert_eq!(colors.name, None);
        assert_eq!(colors.pid, Color::CYAN);
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
        // Nothing is listed before the first refresh
        assert_eq!(monitor.process_count(), 0);
        assert!(monitor.selected_process().is_none());
        let buffer = screen(&monitor, 120, 6);
        let stats = row_text(&buffer, 0);
        assert!(stats.starts_with("CPU:"), "{stats}");
        assert!(stats.trim_end().ends_with("Processes: 0"), "{stats}");
        assert!(row_text(&buffer, 1).starts_with("PID    NAME"));
        assert!((2..6).all(|y| row_text(&buffer, y).trim().is_empty()));
    }

    #[test]
    fn test_process_monitor_default() {
        // Default sorts by CPU, descending
        let mut m = ProcessMonitor::default();
        assert!(row_text(&screen(&m, 70, 5), 1).contains("CPU%▼"));
        assert_eq!(m.process_count(), 0);
        // toggle_sort on the default sort (Cpu) flips the direction
        m.toggle_sort(ProcessSort::Cpu);
        assert!(row_text(&screen(&m, 70, 5), 1).contains("CPU%▲"));
    }

    #[test]
    fn test_process_monitor_helper() {
        let monitor = process_monitor();
        let header = row_text(&screen(&monitor, 70, 5), 1);
        assert!(
            header.contains("CPU%▼") && !header.contains("COMMAND"),
            "{header}"
        );
        assert_eq!(monitor.process_count(), 0);
    }

    #[test]
    fn test_htop_helper() {
        let monitor = htop();
        let header = row_text(&screen(&monitor, 70, 5), 1);
        assert!(
            header.contains("CPU%▼") && !header.contains("COMMAND"),
            "{header}"
        );
        assert_eq!(monitor.process_count(), 0);
    }

    // =========================================================================
    // Builder method tests
    // =========================================================================

    #[test]
    fn test_sort_by() {
        let mut monitor = ProcessMonitor::new().sort_by(ProcessSort::Memory);
        // Both memory columns carry the arrow; CPU does not
        let header = row_text(&screen(&monitor, 70, 5), 1);
        assert!(
            header.contains("MEM%▼") && header.contains("MEM▼"),
            "{header}"
        );
        assert!(!header.contains("CPU%▼"), "{header}");
        monitor.refresh();
        let list = listed(&mut monitor);
        assert!(list.windows(2).all(|w| w[0].memory >= w[1].memory));
    }

    #[test]
    fn test_ascending() {
        let mut monitor = ProcessMonitor::new().ascending(true);
        assert!(row_text(&screen(&monitor, 70, 5), 1).contains("CPU%▲"));
        monitor.refresh();
        let list = listed(&mut monitor);
        assert!(list.windows(2).all(|w| w[0].cpu <= w[1].cpu));
    }

    #[test]
    fn test_ascending_false() {
        let mut monitor = ProcessMonitor::new().ascending(false);
        assert!(row_text(&screen(&monitor, 70, 5), 1).contains("CPU%▼"));
        monitor.refresh();
        let list = listed(&mut monitor);
        assert!(list.windows(2).all(|w| w[0].cpu >= w[1].cpu));
    }

    #[test]
    #[allow(deprecated)] // ProcessView::Tree
    fn test_view() {
        // Tree has no layout of its own: the flat list, sorted by CPU
        let mut monitor = ProcessMonitor::new().view(ProcessView::Tree);
        monitor.refresh();
        let list = listed(&mut monitor);
        assert!(list.iter().any(|p| p.pid == std::process::id()));
        assert!(list.windows(2).all(|w| w[0].cpu >= w[1].cpu));
        let mut pids: Vec<u32> = list.iter().map(|p| p.pid).collect();
        pids.sort_unstable();
        pids.dedup();
        assert_eq!(pids.len(), list.len(), "each process once");
    }

    #[test]
    fn test_colors() {
        let custom_colors = ProcColors {
            header_bg: Color::rgb(1, 2, 3),
            header_fg: Color::rgb(4, 5, 6),
            ..ProcColors::default()
        };
        let monitor = ProcessMonitor::new().colors(custom_colors);
        let buffer = screen(&monitor, 60, 5);
        let cell = buffer.get(0, 1).unwrap();
        assert_eq!(
            (cell.symbol, cell.fg, cell.bg),
            ('P', Some(Color::rgb(4, 5, 6)), Some(Color::rgb(1, 2, 3)))
        );
        assert_eq!(buffer.get(59, 1).unwrap().bg, Some(Color::rgb(1, 2, 3)));
    }

    #[test]
    fn test_show_cmd() {
        let mut monitor = ProcessMonitor::new().show_cmd(true);
        monitor.refresh();
        let buffer = screen(&monitor, 100, 5);
        let header: String = row_text(&buffer, 1).chars().skip(59).collect();
        assert_eq!(header.trim_end(), "COMMAND");
        // The first process's command (or its name when it has none)
        assert_ne!(buffer.get(59, 2).unwrap().symbol, ' ');
    }

    #[test]
    fn test_show_cmd_false() {
        let mut monitor = ProcessMonitor::new().show_cmd(false);
        monitor.refresh();
        let buffer = screen(&monitor, 100, 8);
        for y in 1..8 {
            let tail: String = row_text(&buffer, y).chars().skip(59).collect();
            assert!(tail.trim().is_empty(), "row {y}: {tail:?}");
        }
    }

    #[test]
    fn test_update_interval() {
        let monitor = ProcessMonitor::new().update_interval(500);
        // Verify it doesn't need update immediately after creation with large interval
        assert!(!monitor.needs_update());
    }

    #[test]
    fn test_builder_chain() {
        let mut monitor = ProcessMonitor::new()
            .sort_by(ProcessSort::Name)
            .ascending(true)
            .view(ProcessView::User)
            .show_cmd(true)
            .update_interval(2000);

        let header = row_text(&screen(&monitor, 100, 5), 1);
        assert!(
            header.contains("NAME▲") && header.contains("COMMAND"),
            "{header}"
        );
        monitor.refresh();
        let list = listed(&mut monitor);
        assert!(list
            .windows(2)
            .all(|w| w[0].name.to_lowercase() <= w[1].name.to_lowercase()));
        // The User view keeps the processes of whoever runs this one
        if let Some(me) = list.iter().find(|p| p.pid == std::process::id()) {
            assert!(list.iter().all(|p| p.user == me.user));
        }
    }

    // =========================================================================
    // Filter tests
    // =========================================================================

    #[test]
    fn test_filter() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();
        monitor.select_next();
        let before = selected_index(&monitor);
        monitor.filter("zz-no-such-process-zz");
        // The filter applies at the next refresh; until then the selection
        // stays on its process (#975)
        assert_eq!(selected_index(&monitor), before);
        // The next refresh lists only matches: none
        monitor.refresh();
        assert_eq!(monitor.process_count(), 0);
        assert!(monitor.selected_process().is_none());
    }

    #[test]
    fn test_clear_filter() {
        let mut monitor = ProcessMonitor::new();
        monitor.filter("zz-no-such-process-zz");
        monitor.refresh();
        assert_eq!(monitor.process_count(), 0);
        monitor.clear_filter();
        monitor.refresh();
        let list = listed(&mut monitor);
        assert!(list.iter().any(|p| p.pid == std::process::id()));
    }

    #[test]
    fn test_filter_with_uppercase() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();
        let me = listed(&mut monitor)
            .into_iter()
            .find(|p| p.pid == std::process::id())
            .expect("this test process is listed");
        // Filter is case-insensitive (stored as lowercase)
        monitor.filter(me.name.to_uppercase());
        monitor.refresh();
        let needle = me.name.to_lowercase();
        let list = listed(&mut monitor);
        assert!(list.iter().any(|p| p.pid == me.pid));
        assert!(list
            .iter()
            .all(|p| p.name.to_lowercase().contains(&needle)
                || p.cmd.to_lowercase().contains(&needle)));
    }

    // =========================================================================
    // Toggle sort tests
    // =========================================================================

    #[test]
    fn test_toggle_sort_same_column() {
        let mut monitor = ProcessMonitor::new();
        // Toggle same column (Cpu by default) flips direction
        monitor.toggle_sort(ProcessSort::Cpu);
        assert!(row_text(&screen(&monitor, 70, 5), 1).contains("CPU%▲"));
        monitor.toggle_sort(ProcessSort::Cpu);
        // Two toggles return to the original direction
        assert!(row_text(&screen(&monitor, 70, 5), 1).contains("CPU%▼"));
    }

    #[test]
    fn test_toggle_sort_different_column() {
        let mut monitor = ProcessMonitor::new().ascending(true);
        // Toggle different column sets it as sort column (descending)
        monitor.toggle_sort(ProcessSort::Memory);
        let header = row_text(&screen(&monitor, 70, 5), 1);
        assert!(
            header.contains("MEM%▼") && !header.contains("CPU%▲"),
            "{header}"
        );
        monitor.refresh();
        let list = listed(&mut monitor);
        assert!(list.windows(2).all(|w| w[0].memory >= w[1].memory));
    }

    // =========================================================================
    // Selection tests
    // =========================================================================

    #[test]
    fn test_select_next() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        monitor.select_next();
        // Selection advances, or stays on the last process
        let last = monitor.process_count() - 1;
        assert_eq!(selected_index(&monitor), Some(1.min(last)));
    }

    #[test]
    fn test_select_prev() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        monitor.select_next();
        monitor.select_prev();
        assert_eq!(selected_index(&monitor), Some(0));
    }

    #[test]
    fn test_select_prev_at_zero() {
        let mut monitor = ProcessMonitor::new();
        monitor.select_prev();
        // Nothing listed yet: nothing selected
        assert!(monitor.selected_process().is_none());
        monitor.refresh();
        monitor.select_prev();
        // Does not go below 0
        assert_eq!(selected_index(&monitor), Some(0));
    }

    #[test]
    fn test_page_down() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        monitor.page_down(10);
        let last = monitor.process_count() - 1;
        assert_eq!(selected_index(&monitor), Some(10.min(last)));
    }

    #[test]
    fn test_page_up() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        monitor.page_down(10);
        monitor.page_up(5);
        let last = monitor.process_count() - 1;
        assert_eq!(
            selected_index(&monitor),
            Some(10.min(last).saturating_sub(5))
        );
    }

    #[test]
    fn test_page_up_at_zero() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();
        monitor.page_up(10);
        assert_eq!(selected_index(&monitor), Some(0));
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
        monitor.refresh();
        let list = listed(&mut monitor);
        // page_down with huge value clamps to the last process
        monitor.page_down(99999);
        assert_eq!(
            monitor.selected_process().map(|p| p.pid),
            list.last().map(|p| p.pid)
        );
        monitor.select_next();
        assert_eq!(selected_index(&monitor), Some(list.len() - 1));
    }

    #[test]
    fn test_process_count() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();
        let count = monitor.process_count();
        // One row per process, and the stats bar's count
        let buffer = screen(&monitor, 120, count as u16 + 3);
        let rows = (2..count as u16 + 3)
            .filter(|&y| !row_text(&buffer, y).trim().is_empty())
            .count();
        assert_eq!(rows, count);
        let stats = row_text(&buffer, 0);
        assert!(
            stats.trim_end().ends_with(&format!("Processes: {count}")),
            "{stats}"
        );
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
        // This test process is among them
        let list = listed(&mut monitor);
        assert_eq!(list.len(), monitor.process_count());
        assert!(list.iter().any(|p| p.pid == std::process::id()));
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
        // Not due yet: tick reads nothing
        let mut idle = ProcessMonitor::new().update_interval(3_600_000);
        idle.tick();
        assert_eq!(idle.process_count(), 0);

        let mut monitor = ProcessMonitor::new().update_interval(1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        monitor.tick();
        // Due: tick refreshed the list and restarted the interval
        assert!(listed(&mut monitor)
            .iter()
            .any(|p| p.pid == std::process::id()));
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_render() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();
        let first = monitor.selected_process().unwrap().pid;

        let mut buffer = Buffer::new(100, 30);
        let area = Rect::new(0, 0, 100, 30);
        let mut ctx = RenderContext::new(&mut buffer, area);

        monitor.render(&mut ctx);
        assert!(row_text(&buffer, 0).starts_with("CPU:"));
        assert!(row_text(&buffer, 1).starts_with("PID    NAME                CPU%▼"));
        // The first process, selected
        assert_eq!(row_text(&buffer, 2)[..6].trim(), first.to_string());
        assert_eq!(buffer.get(99, 2).unwrap().bg, Some(Color::rgb(60, 80, 120)));
    }

    #[test]
    fn test_render_small_area() {
        let mut monitor = ProcessMonitor::new();
        monitor.refresh();

        // Under 40 columns or 5 rows nothing is drawn
        for (w, h) in [(10, 2), (39, 30), (100, 4)] {
            let buffer = screen(&monitor, w, h);
            let blank = (0..h).all(|y| {
                (0..w).all(|x| {
                    let cell = buffer.get(x, y).unwrap();
                    cell.symbol == ' ' && cell.fg.is_none() && cell.bg.is_none()
                })
            });
            assert!(blank, "{w}x{h}");
        }
    }
}
