//! Dashboard example - System monitoring demo
//!
//! Run with: cargo run --example dashboard

use revue::prelude::*;

/// System metrics (simulated)
struct Metrics {
    cpu: Vec<f32>,
    memory: f32,
    disk: f32,
    network_in: Vec<f32>,
    network_out: Vec<f32>,
    processes: Vec<Process>,
    uptime: u64,
}

struct Process {
    name: String,
    cpu: f32,
    memory: f32,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            cpu: vec![0.3, 0.5, 0.4, 0.6, 0.8, 0.7, 0.5, 0.4, 0.6, 0.5],
            memory: 0.65,
            disk: 0.42,
            network_in: vec![0.2, 0.4, 0.3, 0.5, 0.7, 0.6, 0.4, 0.3],
            network_out: vec![0.1, 0.2, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1],
            processes: vec![
                Process {
                    name: "revue".into(),
                    cpu: 12.5,
                    memory: 45.2,
                },
                Process {
                    name: "cargo".into(),
                    cpu: 8.3,
                    memory: 128.0,
                },
                Process {
                    name: "rust-analyzer".into(),
                    cpu: 15.2,
                    memory: 256.4,
                },
                Process {
                    name: "terminal".into(),
                    cpu: 1.2,
                    memory: 32.0,
                },
                Process {
                    name: "nvim".into(),
                    cpu: 3.4,
                    memory: 64.8,
                },
            ],
            uptime: 86400 * 3 + 3600 * 7 + 60 * 23,
        }
    }
}

struct Dashboard {
    metrics: Metrics,
    selected_tab: usize,
    process_selected: usize,
}

impl Dashboard {
    fn new() -> Self {
        Self {
            metrics: Metrics::default(),
            selected_tab: 0,
            process_selected: 0,
        }
    }

    fn format_uptime(&self) -> String {
        let days = self.metrics.uptime / 86400;
        let hours = (self.metrics.uptime % 86400) / 3600;
        let mins = (self.metrics.uptime % 3600) / 60;
        format!("{}d {}h {}m", days, hours, mins)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Tab => {
                self.selected_tab = (self.selected_tab + 1) % 3;
                true
            }
            Key::BackTab => {
                self.selected_tab = if self.selected_tab == 0 {
                    2
                } else {
                    self.selected_tab - 1
                };
                true
            }
            Key::Up | Key::Char('k') => {
                if self.process_selected > 0 {
                    self.process_selected -= 1;
                }
                true
            }
            Key::Down | Key::Char('j') => {
                if self.process_selected < self.metrics.processes.len() - 1 {
                    self.process_selected += 1;
                }
                true
            }
            Key::Char('r') => {
                // Simulate refresh with random values
                self.metrics.cpu.rotate_left(1);
                self.metrics.memory = (self.metrics.memory + 0.05) % 1.0;
                true
            }
            _ => false,
        }
    }

    fn render_sparkline_text(&self, data: &[f32]) -> String {
        let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        data.iter()
            .map(|&v| {
                let idx = ((v.clamp(0.0, 1.0) * 7.0) as usize).min(7);
                blocks[idx]
            })
            .collect()
    }

    fn render_overview(&self, ctx: &mut RenderContext) {
        let cpu_spark = Text::new(self.render_sparkline_text(&self.metrics.cpu)).fg(Color::GREEN);
        let cpu_panel = Border::rounded()
            .title("CPU")
            .child(vstack().child(cpu_spark).child(Text::new(format!(
                "{:.1}%",
                self.metrics.cpu.last().unwrap_or(&0.0) * 100.0
            ))));

        let mem_panel = Border::rounded().title("Memory").child(
            vstack()
                .child(Progress::new(self.metrics.memory).filled_color(Color::YELLOW))
                .child(Text::new(format!("{:.1}%", self.metrics.memory * 100.0))),
        );

        let disk_panel = Border::rounded().title("Disk").child(
            vstack()
                .child(Progress::new(self.metrics.disk).filled_color(Color::BLUE))
                .child(Text::new(format!("{:.1}%", self.metrics.disk * 100.0))),
        );

        hstack()
            .gap(1)
            .child(cpu_panel)
            .child(mem_panel)
            .child(disk_panel)
            .render(ctx);
    }

    fn render_processes(&self, ctx: &mut RenderContext) {
        let header_row = hstack()
            .child(Text::new(format!("{:<20}", "Process")).bold())
            .child(Text::new(format!("{:>8}", "CPU %")).bold())
            .child(Text::new(format!("{:>10}", "Mem (MB)")).bold());

        let mut process_list = vstack().child(header_row);
        for (i, p) in self.metrics.processes.iter().enumerate() {
            let row_text = format!("{:<20}{:>8.1}{:>10.1}", p.name, p.cpu, p.memory);
            let row = if i == self.process_selected {
                Text::new(row_text).fg(Color::CYAN).bold()
            } else {
                Text::new(row_text)
            };
            process_list = process_list.child(row);
        }

        Border::rounded()
            .title("Processes")
            .child(process_list)
            .render(ctx);
    }

    fn render_network(&self, ctx: &mut RenderContext) {
        let in_spark =
            Text::new(self.render_sparkline_text(&self.metrics.network_in)).fg(Color::GREEN);
        let out_spark =
            Text::new(self.render_sparkline_text(&self.metrics.network_out)).fg(Color::RED);

        let net_in = Border::rounded()
            .title("Network In")
            .child(vstack().child(in_spark).child(Text::new("125.4 KB/s")));

        let net_out = Border::rounded()
            .title("Network Out")
            .child(vstack().child(out_spark).child(Text::new("42.1 KB/s")));

        hstack().gap(1).child(net_in).child(net_out).render(ctx);
    }
}

impl View for Dashboard {
    fn render(&self, ctx: &mut RenderContext) {
        // Header
        let header = hstack()
            .child(Text::new("System Dashboard").fg(Color::CYAN).bold())
            .child(
                Text::new(format!("  Uptime: {}", self.format_uptime()))
                    .fg(Color::rgb(128, 128, 128)),
            );
        header.render(ctx);

        // Tab bar
        let tabs = ["Overview", "Processes", "Network"];
        let mut tab_bar = hstack().gap(2);
        for (i, &name) in tabs.iter().enumerate() {
            let tab_text = if i == self.selected_tab {
                Text::new(format!("[{}]", name)).fg(Color::CYAN).bold()
            } else {
                Text::new(format!(" {} ", name)).fg(Color::rgb(128, 128, 128))
            };
            tab_bar = tab_bar.child(tab_text);
        }
        tab_bar.render(ctx);

        // Content based on selected tab
        match self.selected_tab {
            0 => self.render_overview(ctx),
            1 => self.render_processes(ctx),
            2 => self.render_network(ctx),
            _ => {}
        }

        // Help bar
        Text::new("Tab: Switch tabs | j/k: Navigate | r: Refresh | q: Quit")
            .fg(Color::rgb(100, 100, 100))
            .render(ctx);
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let dashboard = Dashboard::new();

    app.run_with_handler(dashboard, |key_event, dashboard| {
        dashboard.handle_key(&key_event.key)
    })
}
