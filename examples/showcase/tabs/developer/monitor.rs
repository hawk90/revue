//! Monitor widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{braille_canvas, canvas, sparkline, ProcessMonitor};

pub fn examples() -> Vec<Example> {
    let (primary, success, _warning, _error, info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Process Monitor",
            "System process list with CPU and memory usage",
            render_process_monitor(&primary, &muted),
        ),
        Example::new(
            "System Stats",
            "Real-time CPU, memory, and disk resource usage",
            Border::rounded().title(" System Stats ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("CPU: 42.5%").fg(primary))
                    .child(Progress::new(0.425))
                    .child(Text::new(""))
                    .child(Text::new("Memory: 8.2 / 16.0 GB").fg(primary))
                    .child(Progress::new(0.51))
                    .child(Text::new(""))
                    .child(Text::new("Disk: 256 / 512 GB").fg(primary))
                    .child(Progress::new(0.50))
                    .child(Text::new(""))
                    .child(Text::new("Uptime: 5d 12h 30m").fg(info))
                    .child(Text::new(""))
                    .child(Text::new("• Real-time stats").fg(muted))
                    .child(Text::new("• Resource usage").fg(muted))
                    .child(Text::new("• Uptime display").fg(muted)),
            ),
        ),
        Example::new(
            "Network Monitor",
            "Network I/O with bandwidth and connection tracking",
            Border::rounded().title(" Network Monitor ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Interface: eth0").fg(primary))
                    .child(Text::new("RX: 1024 KB/s").fg(success))
                    .child(Text::new("TX: 512 KB/s").fg(info))
                    .child(Text::new("Connections: 24").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• Network I/O").fg(muted))
                    .child(Text::new("• Bandwidth usage").fg(muted))
                    .child(Text::new("• Connection count").fg(muted)),
            ),
        ),
        Example::new(
            "Canvas",
            "2D drawing canvas with points, lines, and shapes",
            Border::rounded().title(" Canvas ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Drawing canvas:").fg(primary))
                    .child(Text::new(""))
                    .child(canvas(|_ctx| {
                        // Canvas drawing placeholder
                    }))
                    .child(Text::new(""))
                    .child(Text::new("• 2D drawing").fg(muted))
                    .child(Text::new("• Points, lines, shapes").fg(muted))
                    .child(Text::new("• Custom dimensions").fg(muted)),
            ),
        ),
        Example::new(
            "Braille Canvas",
            "High-resolution drawing using braille patterns",
            Border::rounded().title(" Braille Canvas ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("High-resolution:").fg(primary))
                    .child(Text::new(""))
                    .child(braille_canvas(|_ctx| {
                        // Braille canvas drawing placeholder
                    }))
                    .child(Text::new(""))
                    .child(Text::new("• Braille patterns").fg(muted))
                    .child(Text::new("• Higher resolution").fg(muted))
                    .child(Text::new("• Smooth curves").fg(muted)),
            ),
        ),
        Example::new(
            "Activity Monitor",
            "Real-time sparkline graphs with multiple data series",
            Border::rounded().title(" Activity Monitor ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Real-time graph:").fg(primary))
                    .child(Text::new(""))
                    .child(sparkline(vec![
                        0.2, 0.4, 0.6, 0.8, 0.5, 0.3, 0.7, 0.9, 0.4, 0.6,
                    ]))
                    .child(Text::new(""))
                    .child(sparkline(vec![
                        0.5, 0.3, 0.7, 0.4, 0.6, 0.8, 0.5, 0.3, 0.7, 0.5,
                    ]))
                    .child(Text::new(""))
                    .child(Text::new("• Live updates").fg(muted))
                    .child(Text::new("• Multiple series").fg(muted))
                    .child(Text::new("• Auto-scrolling").fg(muted)),
            ),
        ),
    ]
}

#[cfg(feature = "sysinfo")]
fn render_process_monitor(_primary: &Color, muted: &Color) -> Border {
    Border::rounded().title(" Process Monitor ").child(
        vstack()
            .gap(1)
            .child(ProcessMonitor::new())
            .child(Text::new(""))
            .child(Text::new("• Process list").fg(*muted))
            .child(Text::new("• CPU/Memory").fg(*muted))
            .child(Text::new("• Status display").fg(*muted)),
    )
}

#[cfg(not(feature = "sysinfo"))]
fn render_process_monitor(primary: &Color, muted: &Color) -> Border {
    Border::rounded().title(" Process Monitor ").child(
        vstack()
            .gap(1)
            .child(Text::new("Enable 'sysinfo' feature").fg(*muted))
            .child(Text::new("to see process monitor.").fg(*muted))
            .child(Text::new(""))
            .child(Text::new("• Process list").fg(*muted))
            .child(Text::new("• CPU/Memory").fg(*muted))
            .child(Text::new("• Status display").fg(*muted)),
    )
}
