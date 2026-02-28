//! Revue Showcase — Complete Widget Gallery
//!
//! A comprehensive demo showcasing 90+ widgets across 6 tabs:
//! Overview, Inputs, Display, Charts, Data, and Layout.
//!
//! Run with: cargo run --example showcase --features full
//!
//! Key Bindings:
//! - [1-6] Switch tabs
//! - [Tab/Shift+Tab] Cycle tabs
//! - [t] Cycle themes
//! - [q/Esc] Quit

use revue::prelude::*;
use revue::utils::Ticker;
use revue::widget::*;
use std::cell::RefCell;

// ─── Theme Helpers ─────────────────────────────────────────────────────────────

fn theme_colors() -> (Color, Color, Color, Color, Color, Color, Color, Color) {
    let t = use_theme().get();
    (
        t.palette.primary,
        t.palette.success,
        t.palette.warning,
        t.palette.error,
        t.palette.info,
        t.colors.text_muted,
        t.colors.text,
        t.colors.surface, // background for empty areas
    )
}

/// Create a consistently styled gauge
/// - fill_color: theme color for the filled portion
/// - fill_background: darkened version (40% darker)
/// - empty_background: surface color from theme
fn themed_gauge(value: f64, label: &str, fill_color: Color, empty_bg: Color) -> Gauge {
    Gauge::new()
        .value(value)
        .label(label)
        .fill_color(fill_color)
        .fill_background(fill_color.darken_pct(0.4))
        .empty_background(empty_bg)
        .width(20)
}

/// Create a gauge that changes color based on threshold (e.g., CPU > 80% turns red)
fn threshold_gauge(
    value: f64,
    label: &str,
    normal_color: Color,
    warning_color: Color,
    empty_bg: Color,
    threshold: f64,
) -> Gauge {
    let fill_color = if value > threshold {
        warning_color
    } else {
        normal_color
    };
    themed_gauge(value, label, fill_color, empty_bg)
}

// ─── Tab Definition ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Overview,
    Inputs,
    Display,
    Charts,
    Data,
    Layout,
}

impl Tab {
    const ALL: [Tab; 6] = [
        Tab::Overview,
        Tab::Inputs,
        Tab::Display,
        Tab::Charts,
        Tab::Data,
        Tab::Layout,
    ];

    fn name(&self) -> &'static str {
        match self {
            Tab::Overview => "Overview",
            Tab::Inputs => "Inputs",
            Tab::Display => "Display",
            Tab::Charts => "Charts",
            Tab::Data => "Data",
            Tab::Layout => "Layout",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Tab::Overview => "◉",
            Tab::Inputs => "◈",
            Tab::Display => "◆",
            Tab::Charts => "▤",
            Tab::Data => "⊞",
            Tab::Layout => "▥",
        }
    }

    fn key(&self) -> char {
        match self {
            Tab::Overview => '1',
            Tab::Inputs => '2',
            Tab::Display => '3',
            Tab::Charts => '4',
            Tab::Data => '5',
            Tab::Layout => '6',
        }
    }
}

// ─── State ────────────────────────────────────────────────────────────────────

struct Showcase {
    active_tab: usize,
    frame: u64,
    ticker: RefCell<Ticker>,

    // Animation data
    cpu: f64,
    memory: f64,
    net_in: Vec<f64>,
    net_out: Vec<f64>,
    wave_data: Vec<f64>,

    // Input states
    checkbox_a: bool,
    switch_a: bool,
    slider_val: f64,
    rating_val: u8,
    radio_selected: usize,
}

impl Showcase {
    fn new() -> Self {
        Self {
            active_tab: 0,
            frame: 0,
            ticker: RefCell::new(Ticker::new()),

            cpu: 0.42,
            memory: 0.67,
            net_in: vec![
                10.0, 25.0, 18.0, 30.0, 22.0, 45.0, 38.0, 55.0, 42.0, 60.0, 35.0, 50.0, 28.0, 40.0,
                32.0, 58.0, 44.0, 62.0, 48.0, 55.0,
            ],
            net_out: vec![
                5.0, 12.0, 8.0, 15.0, 10.0, 22.0, 18.0, 28.0, 20.0, 32.0, 15.0, 25.0, 12.0, 20.0,
                16.0, 30.0, 22.0, 35.0, 24.0, 28.0,
            ],
            wave_data: (0..40)
                .map(|i| (i as f64 * 0.3).sin() * 0.5 + 0.5)
                .collect(),

            checkbox_a: true,
            switch_a: true,
            slider_val: 65.0,
            rating_val: 4,
            radio_selected: 0,
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('q') | Key::Escape => return false,
            Key::Char('t') => cycle_theme(),
            Key::Char('1') => self.active_tab = 0,
            Key::Char('2') => self.active_tab = 1,
            Key::Char('3') => self.active_tab = 2,
            Key::Char('4') => self.active_tab = 3,
            Key::Char('5') => self.active_tab = 4,
            Key::Char('6') => self.active_tab = 5,
            Key::Tab => self.active_tab = (self.active_tab + 1) % Tab::ALL.len(),
            Key::BackTab => {
                self.active_tab = (self.active_tab + Tab::ALL.len() - 1) % Tab::ALL.len()
            }
            _ => self.handle_tab_key(key),
        }
        true
    }

    fn handle_tab_key(&mut self, key: &Key) {
        if Tab::ALL[self.active_tab] == Tab::Inputs {
            match key {
                Key::Char('c') => self.checkbox_a = !self.checkbox_a,
                Key::Char('s') => self.switch_a = !self.switch_a,
                Key::Left => self.slider_val = (self.slider_val - 5.0).max(0.0),
                Key::Right => self.slider_val = (self.slider_val + 5.0).min(100.0),
                Key::Char('-') => self.rating_val = self.rating_val.saturating_sub(1),
                Key::Char('+') | Key::Char('=') => self.rating_val = (self.rating_val + 1).min(5),
                Key::Up => self.radio_selected = self.radio_selected.saturating_sub(1),
                Key::Down => self.radio_selected = (self.radio_selected + 1).min(2),
                _ => {}
            }
        }
    }

    fn tick(&mut self) {
        self.frame += 1;
        let _ = self.ticker.borrow_mut().tick();

        // Animate data
        let wobble = (self.frame as f64 * 0.1).sin() * 0.05;
        self.cpu = (self.cpu + wobble + 0.01 * ((self.frame % 7) as f64 - 3.0)).clamp(0.05, 0.95);
        self.memory = (self.memory + wobble * 0.5).clamp(0.3, 0.9);

        // Update sparklines
        let new_in = (self.net_in.last().unwrap_or(&30.0) + ((self.frame % 11) as f64 - 5.0) * 2.0)
            .clamp(5.0, 80.0);
        let new_out = (self.net_out.last().unwrap_or(&15.0)
            + ((self.frame % 9) as f64 - 4.0) * 1.5)
            .clamp(2.0, 50.0);
        self.net_in.push(new_in);
        self.net_out.push(new_out);
        if self.net_in.len() > 20 {
            self.net_in.remove(0);
        }
        if self.net_out.len() > 20 {
            self.net_out.remove(0);
        }

        // Update wave
        self.wave_data = (0..40)
            .map(|i| {
                let x = i as f64 * 0.15 + self.frame as f64 * 0.05;
                (x.sin() + (x * 1.5).sin() * 0.5) * 0.5 + 0.5
            })
            .collect();
    }

    // ─── Header ───────────────────────────────────────────────────────────────

    fn render_header(&self) -> impl View {
        let (primary, _, _, _, _, muted, _, _) = theme_colors();
        let theme = use_theme().get();
        let time = format!(
            "{:02}:{:02}:{:02}",
            (self.frame / 3600) % 24,
            (self.frame / 60) % 60,
            self.frame % 60
        );

        hstack()
            .gap(2)
            .child(Text::new(" REVUE SHOWCASE ").bold().fg(primary))
            .child(Text::new(format!("│ Theme: {}", theme.name)).fg(muted))
            .child(Text::new(format!("│ {}", time)).fg(muted))
    }

    // ─── Navigation Tabs ─────────────────────────────────────────────────────

    fn render_tabs(&self) -> impl View {
        let (primary, _, _, _, _, muted, _, _) = theme_colors();

        let mut tabs = hstack().gap(1);
        for (i, tab) in Tab::ALL.iter().enumerate() {
            let is_active = i == self.active_tab;
            let label = format!("[{}] {} {}", tab.key(), tab.icon(), tab.name());

            tabs = tabs.child(if is_active {
                Text::new(label).bold().fg(primary)
            } else {
                Text::new(label).fg(muted)
            });
        }
        tabs
    }

    // ─── Overview Tab ────────────────────────────────────────────────────────

    fn render_overview(&self) -> impl View {
        let (_primary, success, warning, error, info, muted, text, surface) = theme_colors();

        vstack()
            .gap(1)
            // Title Section
            .child(
                Border::rounded().title(" Welcome to Revue ").child(
                    vstack()
                        .gap(1)
                        .child(
                            Text::new("Revue is a Rust TUI framework with 92+ widgets.").fg(text),
                        )
                        .child(Text::new("Build beautiful terminal interfaces with ease!").fg(text))
                        .child(Text::new(""))
                        .child(
                            Text::new("Press [1-6] to explore different widget categories.")
                                .fg(muted),
                        )
                        .child(Text::new("Press [t] to cycle through themes.").fg(muted)),
                ),
            )
            // Widget Categories
            .child(
                hstack()
                    .gap(2)
                    .child(
                        Border::rounded().title(" Input Widgets ").child(
                            vstack()
                                .gap(0)
                                .child(Text::new("• Button, Checkbox, Switch").fg(info))
                                .child(Text::new("• Input, TextArea, Select").fg(info))
                                .child(Text::new("• Slider, Rating, ColorPicker").fg(info))
                                .child(Text::new("• RadioGroup, Combobox").fg(info)),
                        ),
                    )
                    .child(
                        Border::rounded().title(" Display Widgets ").child(
                            vstack()
                                .gap(0)
                                .child(Text::new("• Text, RichText, BigText").fg(success))
                                .child(Text::new("• Progress, Gauge, Spinner").fg(success))
                                .child(Text::new("• Badge, Tag, Avatar").fg(success))
                                .child(Text::new("• Alert, Callout, Tooltip").fg(success)),
                        ),
                    ),
            )
            .child(
                hstack()
                    .gap(2)
                    .child(
                        Border::rounded().title(" Chart Widgets ").child(
                            vstack()
                                .gap(0)
                                .child(Text::new("• BarChart, LineChart, PieChart").fg(warning))
                                .child(Text::new("• Sparkline, Waveline").fg(warning))
                                .child(Text::new("• Histogram, HeatMap").fg(warning))
                                .child(Text::new("• CandleChart, ScatterChart").fg(warning)),
                        ),
                    )
                    .child(
                        Border::rounded().title(" Data Widgets ").child(
                            vstack()
                                .gap(0)
                                .child(Text::new("• Table, DataGrid, List").fg(error))
                                .child(Text::new("• Tree, FileTree").fg(error))
                                .child(Text::new("• Calendar, Timeline").fg(error))
                                .child(Text::new("• JsonViewer, CsvViewer").fg(error)),
                        ),
                    ),
            )
            // Live Stats
            .child(
                Border::rounded().title(" Live Stats ").child(
                    hstack()
                        .gap(4)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("CPU Usage:").fg(muted))
                                .child(threshold_gauge(
                                    self.cpu, "CPU", success, error, surface, 0.8,
                                )),
                        )
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Memory:").fg(muted))
                                .child(themed_gauge(self.memory, "MEM", info, surface)),
                        )
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Network I/O:").fg(muted))
                                .child(
                                    hstack()
                                        .gap(2)
                                        .child(Sparkline::new(self.net_in.clone()).fg(success))
                                        .child(Sparkline::new(self.net_out.clone()).fg(info)),
                                ),
                        ),
                ),
            )
    }

    // ─── Inputs Tab ───────────────────────────────────────────────────────────

    fn render_inputs(&self) -> impl View {
        let (primary, _, _, _, _, muted, _, _) = theme_colors();

        vstack()
            .gap(1)
            .child(
                hstack()
                    .gap(2)
                    // Buttons
                    .child(
                        Border::rounded().title(" Buttons ").child(
                            vstack()
                                .gap(1)
                                .child(
                                    hstack()
                                        .gap(2)
                                        .child(Button::primary("Primary"))
                                        .child(Button::new("Default"))
                                        .child(
                                            Button::new("Danger").variant(ButtonVariant::Danger),
                                        ),
                                )
                                .child(
                                    hstack()
                                        .gap(2)
                                        .child(
                                            Button::new("Success").variant(ButtonVariant::Success),
                                        )
                                        .child(Button::new("Ghost").variant(ButtonVariant::Ghost)),
                                ),
                        ),
                    )
                    // Toggles
                    .child(
                        Border::rounded().title(" Toggles ").child(
                            vstack()
                                .gap(1)
                                .child(Checkbox::new("Enable feature A").checked(self.checkbox_a))
                                .child(Checkbox::new("Enable feature B").checked(!self.checkbox_a))
                                .child(Switch::new().on(self.switch_a).label("Dark mode"))
                                .child(Switch::new().on(!self.switch_a).label("Auto-save"))
                                .child(Text::new("[c] checkbox  [s] switch").fg(muted)),
                        ),
                    ),
            )
            .child(
                hstack()
                    .gap(2)
                    // Slider
                    .child(
                        Border::rounded().title(" Slider & Rating ").child(
                            vstack()
                                .gap(1)
                                .child(
                                    Text::new(format!("Volume: {:.0}%", self.slider_val))
                                        .fg(primary),
                                )
                                .child(
                                    Slider::new()
                                        .value(self.slider_val)
                                        .range(0.0, 100.0)
                                        .label(""),
                                )
                                .child(Text::new("[←/→] adjust slider").fg(muted))
                                .child(Text::new(""))
                                .child(
                                    Text::new(format!("Rating: {} / 5", self.rating_val))
                                        .fg(primary),
                                )
                                .child(Rating::new().value(self.rating_val as f32).max_value(5))
                                .child(Text::new("[-/+] change rating").fg(muted)),
                        ),
                    )
                    // Radio
                    .child(
                        Border::rounded().title(" Selection ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Priority:").bold().fg(primary))
                                .child(
                                    RadioGroup::new(["Low", "Medium", "High"])
                                        .selected(self.radio_selected),
                                )
                                .child(Text::new("[↑/↓] change").fg(muted)),
                        ),
                    )
                    // Input Preview
                    .child(
                        Border::rounded().title(" Text Input ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Search:").fg(primary))
                                .child(Input::new().placeholder("Type to search..."))
                                .child(Text::new(""))
                                .child(
                                    Text::new("(Input widgets are non-interactive in demo)")
                                        .fg(muted),
                                ),
                        ),
                    ),
            )
    }

    // ─── Display Tab ──────────────────────────────────────────────────────────

    fn render_display(&self) -> impl View {
        let (primary, success, warning, error, info, muted, text, surface) = theme_colors();

        vstack()
            .gap(1)
            .child(
                hstack()
                    .gap(2)
                    // Progress & Gauge
                    .child(
                        Border::rounded().title(" Progress Indicators ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Progress Bars:").fg(primary))
                                .child(Progress::new(0.75).filled_color(success))
                                .child(Progress::new(0.45).filled_color(info))
                                .child(Progress::new(0.90).filled_color(warning))
                                .child(Text::new(""))
                                .child(Text::new("Gauges:").fg(primary))
                                .child(themed_gauge(0.75, "Storage", success, surface).width(15))
                                .child(themed_gauge(0.35, "Battery", warning, surface).width(15))
                                .child(Text::new(""))
                                .child(Spinner::new().label("Loading...")),
                        ),
                    )
                    // Badges & Tags
                    .child(
                        Border::rounded().title(" Badges & Tags ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Badges:").fg(primary))
                                .child(
                                    hstack()
                                        .gap(1)
                                        .child(Badge::new("v2.52"))
                                        .child(Badge::new("stable").variant(BadgeVariant::Success))
                                        .child(Badge::new("beta").variant(BadgeVariant::Warning))
                                        .child(Badge::new("new").variant(BadgeVariant::Primary)),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Tags:").fg(primary))
                                .child(
                                    hstack()
                                        .gap(1)
                                        .child(Tag::new("rust"))
                                        .child(Tag::new("tui"))
                                        .child(Tag::new("async")),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Status:").fg(primary))
                                .child(
                                    hstack()
                                        .gap(2)
                                        .child(online())
                                        .child(Text::new("Online").fg(text))
                                        .child(offline())
                                        .child(Text::new("Offline").fg(text))
                                        .child(away_indicator())
                                        .child(Text::new("Away").fg(text)),
                                ),
                        ),
                    )
                    // Alerts
                    .child(
                        Border::rounded().title(" Alerts ").child(
                            vstack()
                                .gap(1)
                                .child(Alert::info("Update available"))
                                .child(Alert::success("Operation completed"))
                                .child(Alert::warning("Low disk space"))
                                .child(Alert::error("Connection failed")),
                        ),
                    ),
            )
            .child(
                hstack()
                    .gap(2)
                    // Text Styles
                    .child(
                        Border::rounded().title(" Text Styles ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Normal text").fg(text))
                                .child(Text::new("Bold text").bold().fg(text))
                                .child(Text::new("Italic text").italic().fg(text))
                                .child(Text::new("Underlined text").underline().fg(text))
                                .child(Text::new("Dim text").dim().fg(muted))
                                .child(Text::new(""))
                                .child(Text::new("Colored:").fg(primary))
                                .child(
                                    hstack()
                                        .gap(2)
                                        .child(Text::new("Primary").fg(primary))
                                        .child(Text::new("Success").fg(success))
                                        .child(Text::new("Warning").fg(warning))
                                        .child(Text::new("Error").fg(error))
                                        .child(Text::new("Info").fg(info)),
                                ),
                        ),
                    )
                    // Avatars
                    .child(
                        Border::rounded().title(" Avatars ").child(
                            vstack().gap(1).child(
                                hstack()
                                    .gap(2)
                                    .child(avatar("Alice"))
                                    .child(avatar("Bob"))
                                    .child(avatar("Charlie")),
                            ),
                        ),
                    )
                    // Unicode
                    .child(
                        Border::rounded().title(" Unicode Support ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Korean: 안녕하세요!").fg(text))
                                .child(Text::new("Japanese: こんにちは").fg(text))
                                .child(Text::new("Chinese: 你好世界").fg(text))
                                .child(Text::new("Emoji: 🎉 👍 🚀 ✨ 💻").fg(text))
                                .child(Text::new("Math: ∑ ∞ π √ ∫ ∂").fg(text))
                                .child(Text::new("Arrows: ← → ↑ ↓ ↔ ↕").fg(text)),
                        ),
                    ),
            )
    }

    // ─── Charts Tab ────────────────────────────────────────────────────────────

    fn render_charts(&self) -> impl View {
        let (primary, success, warning, _, info, muted, _, _) = theme_colors();
        let offset = (self.frame as f64 * 0.05).sin() * 20.0;

        let line_data: Vec<f64> = (0..25)
            .map(|i| {
                let x = i as f64 * 0.3 + self.frame as f64 * 0.02;
                50.0 + 30.0 * x.sin() + 10.0 * (x * 2.3).cos()
            })
            .collect();

        vstack()
            .gap(1)
            .child(
                hstack()
                    .gap(2)
                    // Bar Chart
                    .child(
                        Border::rounded().title(" Bar Chart ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Monthly Revenue:").fg(primary))
                                .child(
                                    BarChart::new()
                                        .bar("Jan", 120.0 + offset)
                                        .bar("Feb", 180.0 - offset * 0.5)
                                        .bar("Mar", 150.0 + offset * 0.3)
                                        .bar("Apr", 210.0)
                                        .bar("May", 190.0 + offset * 0.7)
                                        .bar("Jun", 240.0 - offset * 0.2)
                                        .show_values(true)
                                        .fg(success),
                                ),
                        ),
                    )
                    // Sparklines
                    .child(
                        Border::rounded().title(" Sparklines ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Network IN:").fg(success))
                                .child(Sparkline::new(self.net_in.clone()).fg(success))
                                .child(Text::new("Network OUT:").fg(info))
                                .child(Sparkline::new(self.net_out.clone()).fg(info))
                                .child(Text::new(""))
                                .child(Text::new("(Real-time animated data)").fg(muted)),
                        ),
                    ),
            )
            .child(
                hstack()
                    .gap(2)
                    // Line Chart
                    .child(
                        Border::rounded().title(" Line Chart ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Trend Analysis:").fg(primary))
                                .child(Chart::new().series(
                                    Series::new("Trend").data_y(&line_data).line().color(info),
                                )),
                        ),
                    )
                    // Waveline
                    .child(
                        Border::rounded().title(" Waveline ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Audio Waveform:").fg(primary))
                                .child(waveline(self.wave_data.clone()).color(warning))
                                .child(Text::new(""))
                                .child(Text::new("(Animated audio visualization)").fg(muted)),
                        ),
                    )
                    // Pie Chart
                    .child(
                        Border::rounded().title(" Pie Chart ").child(
                            vstack().gap(1).child(
                                PieChart::new()
                                    .slice_colored("Rust", 45.0, primary)
                                    .slice_colored("Python", 25.0, info)
                                    .slice_colored("Go", 15.0, success)
                                    .slice_colored("Other", 15.0, muted),
                            ),
                        ),
                    ),
            )
    }

    // ─── Data Tab ──────────────────────────────────────────────────────────────

    fn render_data(&self) -> impl View {
        let (primary, success, _, _, info, _muted, _text, _) = theme_colors();

        vstack()
            .gap(1)
            .child(
                hstack()
                    .gap(2)
                    // Table
                    .child(
                        Border::rounded().title(" Table ").child(
                            vstack().gap(1).child(
                                Table::new(vec![
                                    Column::new("ID"),
                                    Column::new("Name"),
                                    Column::new("Status"),
                                    Column::new("CPU"),
                                ])
                                .row(vec!["001", "web-server", "●", "42%"])
                                .row(vec!["002", "api-gateway", "●", "67%"])
                                .row(vec!["003", "database", "●", "89%"])
                                .row(vec!["004", "cache", "○", "0%"])
                                .row(vec!["005", "worker", "●", "23%"]),
                            ),
                        ),
                    )
                    // Tree
                    .child(
                        Border::rounded().title(" Tree ").child(
                            Tree::new()
                                .node(
                                    TreeNode::new("src")
                                        .expanded(true)
                                        .child(TreeNode::new("widget"))
                                        .child(TreeNode::new("style"))
                                        .child(TreeNode::new("lib.rs")),
                                )
                                .node(TreeNode::new("Cargo.toml"))
                                .node(TreeNode::new("README.md")),
                        ),
                    )
                    // List
                    .child(Border::rounded().title(" List ").child(List::new(vec![
                        "Task: Write documentation",
                        "Task: Add tests",
                        "Task: Review PR",
                        "Task: Deploy",
                        "Task: Monitor",
                    ]))),
            )
            .child(
                hstack()
                    .gap(2)
                    // Calendar
                    .child(
                        Border::rounded()
                            .title(" Calendar ")
                            .child(Calendar::new(2026, 2)),
                    )
                    // Timeline
                    .child(
                        Border::rounded().title(" Timeline ").child(
                            Timeline::new()
                                .event(
                                    TimelineEvent::new("Meeting started")
                                        .timestamp("10:00")
                                        .event_type(revue::widget::EventType::Info),
                                )
                                .event(
                                    TimelineEvent::new("Decision made")
                                        .timestamp("11:30")
                                        .success(),
                                )
                                .event(
                                    TimelineEvent::new("Warning issued")
                                        .timestamp("14:00")
                                        .warning(),
                                )
                                .event(
                                    TimelineEvent::new("Error detected")
                                        .timestamp("16:00")
                                        .error(),
                                ),
                        ),
                    )
                    // Digits
                    .child(
                        Border::rounded().title(" Digits & Timer ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Counter:").fg(primary))
                                .child(Digits::timer(self.frame).fg(info))
                                .child(Text::new(""))
                                .child(Text::new("Digits:").fg(primary))
                                .child(Digits::new(12345).fg(success)),
                        ),
                    ),
            )
    }

    // ─── Layout Tab ────────────────────────────────────────────────────────────

    fn render_layout(&self) -> impl View {
        let (primary, _, _, _, _, muted, text, _) = theme_colors();

        vstack()
            .gap(1)
            .child(
                hstack()
                    .gap(2)
                    // Border Types
                    .child(
                        Border::rounded()
                            .title(" Border Types ")
                            .child(
                                hstack()
                                    .gap(1)
                                    .child(
                                        Border::single()
                                            .title("Single")
                                            .child(Text::new("Content").fg(text))
                                    )
                                    .child(
                                        Border::double()
                                            .title("Double")
                                            .child(Text::new("Content").fg(text))
                                    )
                                    .child(
                                        Border::thick()
                                            .title("Thick")
                                            .child(Text::new("Content").fg(text))
                                    )
                            )
                    )
                    // Card
                    .child(
                        Border::rounded()
                            .title(" Card Widget ")
                            .child(
                                Card::new()
                                    .title("Card Title")
                                    .body(Text::new(
                                        "Card body content goes here. Cards are useful for grouping.",
                                    ).fg(text))
                                    .footer(Text::new("Footer").fg(muted))
                            )
                    )
            )
            .child(
                hstack()
                    .gap(2)
                    // Collapsible
                    .child(
                        Border::rounded()
                            .title(" Collapsible ")
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(
                                        Collapsible::new("Section 1")
                                            .expanded(true)
                                            .content("This section is expanded by default.")
                                    )
                                    .child(
                                        Collapsible::new("Section 2")
                                            .expanded(false)
                                            .content("This section is collapsed.")
                                    )
                            )
                    )
                    // Tabs Demo
                    .child(
                        Border::rounded()
                            .title(" Tabs Widget ")
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Tabs::new().tab("Tab A").tab("Tab B").tab("Tab C").selected(0))
                                    .child(Text::new("Content of Tab A").fg(text))
                                    .child(Text::new("Tabs provide navigation between views.").fg(muted))
                            )
                    )
                    // Accordion
                    .child(
                        Border::rounded()
                            .title(" Accordion ")
                            .child(
                                Accordion::new()
                                    .section(
                                        revue::widget::AccordionSection::new("FAQ 1: What is Revue?")
                                            .content("A Rust TUI framework."),
                                    )
                                    .section(
                                        revue::widget::AccordionSection::new("FAQ 2: How many widgets?")
                                            .content("92+ widgets available!"),
                                    )
                                    .section(
                                        revue::widget::AccordionSection::new("FAQ 3: Is it fast?")
                                            .content("Yes, extremely fast!"),
                                    )
                            )
                    )
            )
            .child(
                hstack()
                    .gap(2)
                    // Grid
                    .child(
                        Border::rounded()
                            .title(" Grid Layout ")
                            .child(
                                Grid::new()
                                    .cols(3)
                                    .gap(1)
                                    .child(Border::rounded().title("A").child(Text::new("1").fg(text)))
                                    .child(Border::rounded().title("B").child(Text::new("2").fg(text)))
                                    .child(Border::rounded().title("C").child(Text::new("3").fg(text)))
                            )
                    )
                    // Navigation
                    .child(
                        Border::rounded()
                            .title(" Navigation ")
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Text::new("Stepper:").fg(primary))
                                    .child(
                                        Stepper::new().steps(vec![
                                            Step::new("Setup").complete(),
                                            Step::new("Configure").active(),
                                            Step::new("Build"),
                                            Step::new("Deploy"),
                                        ])
                                    )
                            )
                    )
            )
    }

    // ─── Footer ───────────────────────────────────────────────────────────────

    fn render_footer(&self) -> impl View {
        let (_, _, _, _, _, muted, _, _) = theme_colors();

        hstack()
            .gap(3)
            .child(Text::new(" [1-6] Tabs").fg(muted))
            .child(Text::new("[Tab] Next").fg(muted))
            .child(Text::new("[t] Theme").fg(muted))
            .child(Text::new("[q] Quit").fg(muted))
            .child(Text::new("│ 92+ widgets").fg(muted))
    }
}

// ─── View Implementation ───────────────────────────────────────────────────────

impl View for Showcase {
    fn render(&self, ctx: &mut RenderContext) {
        let main_content: Box<dyn View> = match Tab::ALL[self.active_tab] {
            Tab::Overview => Box::new(self.render_overview()),
            Tab::Inputs => Box::new(self.render_inputs()),
            Tab::Display => Box::new(self.render_display()),
            Tab::Charts => Box::new(self.render_charts()),
            Tab::Data => Box::new(self.render_data()),
            Tab::Layout => Box::new(self.render_layout()),
        };

        vstack()
            .gap(0)
            .child(self.render_header())
            .child(self.render_tabs())
            .child(Text::new(""))
            .child(main_content)
            .child(Text::new(""))
            .child(self.render_footer())
            .render(ctx);
    }
}

// ─── Main Entry ────────────────────────────────────────────────────────────────

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
