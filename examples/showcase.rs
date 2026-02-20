//! Revue Showcase — The flagship demo app
//!
//! A visually rich, interactive demo that showcases 25+ widget types across 4 tabs:
//! Dashboard, Charts, Widgets, and Data. Features real-time simulated data,
//! theme switching, and animated updates.
//!
//! Run with: cargo run --example showcase --features full

use revue::prelude::*;
use revue::utils::Ticker;
use revue::widget::EventType as TlEventType;
use revue::widget::*;
use std::cell::RefCell;

// ─── Color palette (Catppuccin-inspired defaults) ────────────────────────────

const CYAN: Color = Color::rgb(137, 220, 235);
const GREEN: Color = Color::rgb(166, 227, 161);
const YELLOW: Color = Color::rgb(249, 226, 175);
const RED: Color = Color::rgb(243, 139, 168);
const PINK: Color = Color::rgb(245, 194, 231);
const BLUE: Color = Color::rgb(137, 180, 250);
const PEACH: Color = Color::rgb(250, 179, 135);
const MAUVE: Color = Color::rgb(203, 166, 247);
const DIM: Color = Color::rgb(88, 91, 112);

// ─── Tab enum ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Dashboard,
    Charts,
    Widgets,
    Data,
}

impl Tab {
    const ALL: [Tab; 4] = [Tab::Dashboard, Tab::Charts, Tab::Widgets, Tab::Data];

    fn name(&self) -> &'static str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Charts => "Charts",
            Tab::Widgets => "Widgets",
            Tab::Data => "Data",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Tab::Dashboard => "◈",
            Tab::Charts => "▤",
            Tab::Widgets => "◉",
            Tab::Data => "⊞",
        }
    }
}

// ─── Main state ──────────────────────────────────────────────────────────────

struct Showcase {
    active_tab: usize,
    frame: u64,
    ticker: RefCell<Ticker>,

    // Dashboard
    cpu: f64,
    memory: f64,
    disk: f64,
    net_in: Vec<f64>,
    net_out: Vec<f64>,
    task_progress: f32,
    events: Vec<(&'static str, &'static str, TlEventType)>,

    // Widgets tab
    checkbox_val: bool,
    switch_val: bool,
    slider_val: f64,
    rating_val: u8,

    // Data tab
    table_selected: usize,
}

impl Showcase {
    fn new() -> Self {
        Self {
            active_tab: 0,
            frame: 0,
            ticker: RefCell::new(Ticker::new()),

            cpu: 0.42,
            memory: 0.67,
            disk: 0.38,
            net_in: vec![
                10.0, 25.0, 18.0, 30.0, 22.0, 45.0, 38.0, 55.0, 42.0, 60.0, 35.0, 50.0, 28.0, 40.0,
                32.0, 58.0, 44.0, 62.0, 48.0, 55.0, 30.0, 42.0, 56.0, 38.0, 65.0, 50.0, 40.0, 52.0,
                35.0, 45.0, 60.0, 48.0, 55.0, 42.0, 38.0, 50.0, 62.0, 45.0, 52.0, 40.0,
            ],
            net_out: vec![
                5.0, 12.0, 8.0, 15.0, 10.0, 22.0, 18.0, 28.0, 20.0, 32.0, 15.0, 25.0, 12.0, 20.0,
                16.0, 30.0, 22.0, 35.0, 24.0, 28.0, 14.0, 20.0, 30.0, 18.0, 34.0, 26.0, 20.0, 28.0,
                16.0, 22.0, 32.0, 24.0, 28.0, 20.0, 18.0, 26.0, 34.0, 22.0, 28.0, 20.0,
            ],
            task_progress: 0.62,
            events: vec![
                ("12:34:50", "Service alpha deployed", TlEventType::Success),
                ("12:34:42", "High memory on node-3", TlEventType::Warning),
                ("12:34:35", "User john.doe logged in", TlEventType::Info),
                (
                    "12:34:28",
                    "Database backup completed",
                    TlEventType::Success,
                ),
                ("12:34:20", "Request timeout on /api/v2", TlEventType::Error),
            ],

            checkbox_val: true,
            switch_val: true,
            slider_val: 65.0,
            rating_val: 4,

            table_selected: 0,
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('q') | Key::Escape => return false,
            Key::Char('1') => self.active_tab = 0,
            Key::Char('2') => self.active_tab = 1,
            Key::Char('3') => self.active_tab = 2,
            Key::Char('4') => self.active_tab = 3,
            Key::Char('t') => {
                cycle_theme();
            }
            Key::Tab => {
                self.active_tab = (self.active_tab + 1) % Tab::ALL.len();
            }
            Key::BackTab => {
                self.active_tab = (self.active_tab + Tab::ALL.len() - 1) % Tab::ALL.len();
            }
            // Tab-specific keys
            _ => match Tab::ALL[self.active_tab] {
                Tab::Widgets => match key {
                    Key::Char('c') => self.checkbox_val = !self.checkbox_val,
                    Key::Char('s') => self.switch_val = !self.switch_val,
                    Key::Left => self.slider_val = (self.slider_val - 5.0).max(0.0),
                    Key::Right => self.slider_val = (self.slider_val + 5.0).min(100.0),
                    Key::Char('-') => self.rating_val = self.rating_val.saturating_sub(1),
                    Key::Char('+') | Key::Char('=') => {
                        self.rating_val = (self.rating_val + 1).min(5);
                    }
                    _ => {}
                },
                Tab::Data => match key {
                    Key::Up => self.table_selected = self.table_selected.saturating_sub(1),
                    Key::Down => self.table_selected = (self.table_selected + 1).min(5),
                    _ => {}
                },
                _ => {}
            },
        }
        true
    }

    fn tick(&mut self) {
        self.frame += 1;
        let _ = self.ticker.borrow_mut().tick();

        // Simulate CPU/memory changes (random walk)
        let wobble = ((self.frame as f64) * 0.1).sin() * 0.05;
        self.cpu = (self.cpu + wobble + 0.01 * ((self.frame % 7) as f64 - 3.0)).clamp(0.05, 0.95);
        self.memory =
            (self.memory + wobble * 0.5 + 0.005 * ((self.frame % 5) as f64 - 2.0)).clamp(0.3, 0.9);
        self.disk = (self.disk + 0.001).min(0.85);
        self.task_progress = (self.task_progress + 0.002).min(1.0);

        // Shift sparkline data
        let new_in = (self.net_in.last().unwrap_or(&30.0) + ((self.frame % 11) as f64 - 5.0) * 2.0)
            .clamp(5.0, 80.0);
        let new_out = (self.net_out.last().unwrap_or(&15.0)
            + ((self.frame % 9) as f64 - 4.0) * 1.5)
            .clamp(2.0, 50.0);
        self.net_in.push(new_in);
        self.net_out.push(new_out);
        if self.net_in.len() > 40 {
            self.net_in.remove(0);
        }
        if self.net_out.len() > 40 {
            self.net_out.remove(0);
        }
    }

    // ── Header ───────────────────────────────────────────────────────────────

    fn render_header(&self) -> impl View {
        let theme = use_theme().get();
        let time = format!(
            "{:02}:{:02}:{:02}",
            (self.frame / 3600) % 24,
            (self.frame / 60) % 60,
            self.frame % 60
        );

        hstack()
            .gap(1)
            .child(Text::new(" ⚡ REVUE SHOWCASE ").bold().fg(CYAN))
            .child(Text::new(format!("│ Theme: {}", theme.name)).fg(DIM))
            .child(Text::new(format!("│ {}", time)).fg(DIM))
    }

    // ── Sidebar ──────────────────────────────────────────────────────────────

    fn render_sidebar(&self) -> impl View {
        let mut list = vstack().gap(0);
        for (i, tab) in Tab::ALL.iter().enumerate() {
            let is_active = i == self.active_tab;
            let label = format!(" {} {} ", tab.icon(), tab.name());
            let text = if is_active {
                Text::new(format!("▸{}", label)).bold().fg(CYAN)
            } else {
                Text::new(format!(" {}", label)).fg(DIM)
            };
            list = list.child(text);
        }
        Border::rounded().title(" Nav ").child(list)
    }

    // ── Dashboard tab ────────────────────────────────────────────────────────

    fn render_dashboard(&self) -> impl View {
        vstack()
            .gap(1)
            .child(self.render_gauges())
            .child(self.render_sparklines())
            .child(
                hstack()
                    .gap(1)
                    .child(self.render_task_progress())
                    .child(self.render_services()),
            )
            .child(self.render_timeline())
    }

    fn render_gauges(&self) -> impl View {
        hstack()
            .gap(2)
            .child(
                Gauge::new()
                    .value(self.cpu)
                    .label("CPU")
                    .fill_color(if self.cpu > 0.8 { RED } else { GREEN })
                    .width(20),
            )
            .child(
                Gauge::new()
                    .value(self.memory)
                    .label("MEM")
                    .fill_color(if self.memory > 0.8 { RED } else { BLUE })
                    .width(20),
            )
            .child(
                Gauge::new()
                    .value(self.disk)
                    .label("DSK")
                    .fill_color(PEACH)
                    .width(20),
            )
    }

    fn render_sparklines(&self) -> impl View {
        hstack()
            .gap(2)
            .child(
                vstack()
                    .child(Text::new(" Network IN (Mbps)").fg(GREEN))
                    .child(Sparkline::new(self.net_in.clone()).fg(GREEN)),
            )
            .child(
                vstack()
                    .child(Text::new(" Network OUT (Mbps)").fg(BLUE))
                    .child(Sparkline::new(self.net_out.clone()).fg(BLUE)),
            )
    }

    fn render_task_progress(&self) -> impl View {
        Border::rounded().title(" Tasks ").child(
            vstack()
                .gap(1)
                .child(
                    Text::new(format!(" Completion: {:.0}%", self.task_progress * 100.0))
                        .fg(YELLOW),
                )
                .child(Progress::new(self.task_progress).filled_color(MAUVE)),
        )
    }

    fn render_services(&self) -> impl View {
        Border::rounded().title(" Services ").child(
            vstack()
                .child(
                    hstack()
                        .gap(1)
                        .child(StatusIndicator::new(Status::Online).label("API"))
                        .child(StatusIndicator::new(Status::Online).label("DB")),
                )
                .child(
                    hstack()
                        .gap(1)
                        .child(StatusIndicator::new(Status::Busy).label("Cache"))
                        .child(StatusIndicator::new(Status::Offline).label("Worker")),
                ),
        )
    }

    fn render_timeline(&self) -> impl View {
        let mut tl = Timeline::new();
        for &(ts, desc, ref etype) in &self.events {
            tl = tl.event(TimelineEvent::new(desc).timestamp(ts).event_type(*etype));
        }
        Border::rounded().title(" Events ").child(tl)
    }

    // ── Charts tab ───────────────────────────────────────────────────────────

    fn render_charts(&self) -> impl View {
        vstack()
            .gap(1)
            .child(
                hstack()
                    .gap(1)
                    .child(self.render_barchart())
                    .child(self.render_piechart()),
            )
            .child(
                hstack()
                    .gap(1)
                    .child(self.render_linechart())
                    .child(self.render_waveline()),
            )
    }

    fn render_barchart(&self) -> impl View {
        let offset = (self.frame as f64 * 0.05).sin() * 20.0;
        Border::rounded().title(" Revenue ").child(
            BarChart::new()
                .bar("Jan", 120.0 + offset)
                .bar("Feb", 180.0 - offset * 0.5)
                .bar("Mar", 150.0 + offset * 0.3)
                .bar("Apr", 210.0)
                .bar("May", 190.0 + offset * 0.7)
                .bar("Jun", 240.0 - offset * 0.2)
                .show_values(true)
                .fg(CYAN),
        )
    }

    fn render_piechart(&self) -> impl View {
        Border::rounded().title(" Categories ").child(
            PieChart::new()
                .slice("Rust", 45.0)
                .slice("Go", 25.0)
                .slice("Python", 20.0)
                .slice("Other", 10.0)
                .title("Languages"),
        )
    }

    fn render_linechart(&self) -> impl View {
        let data: Vec<f64> = (0..20)
            .map(|i| {
                let x = i as f64 * 0.3 + self.frame as f64 * 0.02;
                50.0 + 30.0 * x.sin() + 10.0 * (x * 2.3).cos()
            })
            .collect();

        Border::rounded()
            .title(" Trend ")
            .child(line_chart(&data).title("Metrics"))
    }

    fn render_waveline(&self) -> impl View {
        let data: Vec<f64> = (0..40)
            .map(|i| {
                let x = i as f64 * 0.15 + self.frame as f64 * 0.05;
                (x.sin() + (x * 1.5).sin() * 0.5) * 0.5 + 0.5
            })
            .collect();

        Border::rounded()
            .title(" Audio ")
            .child(waveline(data).color(PINK))
    }

    // ── Widgets tab ──────────────────────────────────────────────────────────

    fn render_widgets(&self) -> impl View {
        vstack()
            .gap(1)
            .child(self.render_buttons())
            .child(self.render_toggles())
            .child(self.render_inputs())
            .child(self.render_feedback())
            .child(self.render_indicators())
    }

    fn render_buttons(&self) -> impl View {
        Border::rounded().title(" Buttons ").child(
            hstack()
                .gap(2)
                .child(Button::primary("Primary"))
                .child(Button::new("Secondary"))
                .child(Button::new("Danger").variant(ButtonVariant::Danger))
                .child(Button::new("Disabled").disabled(true)),
        )
    }

    fn render_toggles(&self) -> impl View {
        Border::rounded().title(" Toggles ").child(
            hstack()
                .gap(3)
                .child(
                    vstack()
                        .child(Checkbox::new("Feature A").checked(self.checkbox_val))
                        .child(Text::muted("  [c] toggle")),
                )
                .child(
                    vstack()
                        .child(Switch::new().on(self.switch_val).label("Dark mode"))
                        .child(Text::muted("  [s] toggle")),
                )
                .child(
                    vstack()
                        .child(RadioGroup::new(["Small", "Medium", "Large"]).selected(1))
                        .child(Text::muted("  (display only)")),
                ),
        )
    }

    fn render_inputs(&self) -> impl View {
        Border::rounded().title(" Inputs ").child(
            hstack()
                .gap(3)
                .child(
                    vstack()
                        .child(Text::new(format!(" Slider: {:.0}", self.slider_val)))
                        .child(
                            Slider::new()
                                .value(self.slider_val)
                                .range(0.0, 100.0)
                                .label("Volume"),
                        )
                        .child(Text::muted("  [←/→] adjust")),
                )
                .child(
                    vstack()
                        .child(Rating::new().value(self.rating_val as f32).max_value(5))
                        .child(Text::muted("  [-/+] rate")),
                ),
        )
    }

    fn render_feedback(&self) -> impl View {
        Border::rounded().title(" Alerts ").child(
            vstack()
                .gap(0)
                .child(Alert::info("System update available"))
                .child(Alert::success("Deployment successful"))
                .child(Alert::warning("Disk space running low"))
                .child(Alert::error("Connection to node-5 lost")),
        )
    }

    fn render_indicators(&self) -> impl View {
        Border::rounded().title(" Display ").child(
            hstack()
                .gap(2)
                .child(
                    vstack()
                        .child(
                            hstack()
                                .gap(1)
                                .child(Badge::new("v2.50"))
                                .child(Badge::new("stable").variant(BadgeVariant::Success))
                                .child(Badge::new("hot").variant(BadgeVariant::Warning)),
                        )
                        .child(
                            hstack()
                                .gap(1)
                                .child(Tag::new("rust"))
                                .child(Tag::new("tui"))
                                .child(Tag::new("css")),
                        ),
                )
                .child(
                    vstack()
                        .child(Spinner::new().label("Loading..."))
                        .child(Progress::new(0.45)),
                )
                .child(
                    Stepper::new()
                        .step(Step::new("Init").status(StepStatus::Completed))
                        .step(Step::new("Build").status(StepStatus::Active))
                        .step(Step::new("Deploy").status(StepStatus::Pending)),
                ),
        )
    }

    // ── Data tab ─────────────────────────────────────────────────────────────

    fn render_data(&self) -> impl View {
        vstack()
            .gap(1)
            .child(
                hstack()
                    .gap(1)
                    .child(self.render_table())
                    .child(self.render_tree()),
            )
            .child(
                hstack()
                    .gap(1)
                    .child(self.render_calendar())
                    .child(self.render_digits()),
            )
    }

    fn render_table(&self) -> impl View {
        Border::rounded().title(" Servers ").child(
            Table::new(vec![
                Column::new("Host"),
                Column::new("Status"),
                Column::new("CPU"),
                Column::new("Uptime"),
            ])
            .row(vec!["node-1", "online", "42%", "14d 3h"])
            .row(vec!["node-2", "online", "67%", "7d 12h"])
            .row(vec!["node-3", "warning", "89%", "2d 5h"])
            .row(vec!["node-4", "online", "23%", "30d 1h"])
            .row(vec!["node-5", "offline", "0%", "—"])
            .row(vec!["node-6", "online", "55%", "21d 8h"]),
        )
    }

    fn render_tree(&self) -> impl View {
        Border::rounded().title(" Project ").child(
            Tree::new()
                .node(
                    TreeNode::new("src")
                        .expanded(true)
                        .child(
                            TreeNode::new("widget")
                                .expanded(true)
                                .child(TreeNode::new("display.rs"))
                                .child(TreeNode::new("layout.rs"))
                                .child(TreeNode::new("input.rs")),
                        )
                        .child(
                            TreeNode::new("style")
                                .child(TreeNode::new("theme.rs"))
                                .child(TreeNode::new("css.rs")),
                        )
                        .child(TreeNode::new("lib.rs")),
                )
                .node(TreeNode::new("Cargo.toml"))
                .node(TreeNode::new("README.md")),
        )
    }

    fn render_calendar(&self) -> impl View {
        Border::rounded()
            .title(" Calendar ")
            .child(Calendar::new(2026, 2))
    }

    fn render_digits(&self) -> impl View {
        let secs = self.frame;
        Border::rounded()
            .title(" Uptime ")
            .child(Digits::timer(secs).fg(CYAN))
    }

    // ── Footer ───────────────────────────────────────────────────────────────

    fn render_footer(&self) -> impl View {
        hstack()
            .gap(2)
            .child(Text::new(" [1-4] Tab").fg(DIM))
            .child(Text::new("[Tab] Next").fg(DIM))
            .child(Text::new("[t] Theme").fg(DIM))
            .child(Text::new("[q] Quit").fg(DIM))
            .child(match Tab::ALL[self.active_tab] {
                Tab::Widgets => Text::new("│ [c] Check [s] Switch [←/→] Slider [-/+] Rate").fg(DIM),
                Tab::Data => Text::new("│ [↑/↓] Table").fg(DIM),
                _ => Text::new(""),
            })
    }
}

// ─── View impl ───────────────────────────────────────────────────────────────

impl View for Showcase {
    fn render(&self, ctx: &mut RenderContext) {
        let main_content: Box<dyn View> = match Tab::ALL[self.active_tab] {
            Tab::Dashboard => Box::new(self.render_dashboard()),
            Tab::Charts => Box::new(self.render_charts()),
            Tab::Widgets => Box::new(self.render_widgets()),
            Tab::Data => Box::new(self.render_data()),
        };

        vstack()
            .gap(0)
            .child(self.render_header())
            .child(
                hstack().gap(1).child(self.render_sidebar()).child(
                    Border::rounded()
                        .title(format!(
                            " {} {} ",
                            Tab::ALL[self.active_tab].icon(),
                            Tab::ALL[self.active_tab].name()
                        ))
                        .child(main_content),
                ),
            )
            .child(self.render_footer())
            .render(ctx);
    }
}

// ─── Main ────────────────────────────────────────────────────────────────────

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let showcase = Showcase::new();

    app.run(showcase, |event, showcase, _app| match event {
        Event::Key(key_event) => {
            showcase.tick();
            showcase.handle_key(&key_event.key)
        }
        _ => {
            showcase.tick();
            true
        }
    })
}
