//! Revue Showcase — Complete Widget Gallery
//!
//! A comprehensive demo showcasing 92+ widgets across 7 tabs with two-level navigation.
//!
//! Run with: cargo run --example showcase --features full
//!
//! Key Bindings:
//! - [1-7] Switch main tabs
//! - [←/→] Cycle sub-tabs
//! - [t] Cycle themes
//! - [q/Esc] Quit

mod footer;
mod header;
mod tab;
mod tabs;
mod theme;

use revue::prelude::*;
use revue::utils::Ticker;
use std::cell::RefCell;

use footer::render_footer;
use header::{render_header, render_main_tabs, render_sub_tabs};
use tab::{MainTab, SubTab};
use theme::{theme_colors, themed_gauge, threshold_gauge};

// ─── State ────────────────────────────────────────────────────────────────────

struct Showcase {
    main_tab: MainTab,
    sub_tab_index: usize,
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
            main_tab: MainTab::Input,
            sub_tab_index: 0,
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
            Key::Char('1') => {
                self.main_tab = MainTab::Input;
                self.sub_tab_index = 0;
            }
            Key::Char('2') => {
                self.main_tab = MainTab::Display;
                self.sub_tab_index = 0;
            }
            Key::Char('3') => {
                self.main_tab = MainTab::Chart;
                self.sub_tab_index = 0;
            }
            Key::Char('4') => {
                self.main_tab = MainTab::Data;
                self.sub_tab_index = 0;
            }
            Key::Char('5') => {
                self.main_tab = MainTab::Layout;
                self.sub_tab_index = 0;
            }
            Key::Char('6') => {
                self.main_tab = MainTab::Feedback;
                self.sub_tab_index = 0;
            }
            Key::Char('7') => {
                self.main_tab = MainTab::Developer;
                self.sub_tab_index = 0;
            }
            // Input tab controls - slider controls (must come before general navigation)
            Key::Left if self.main_tab == MainTab::Input && self.sub_tab_index == 4 => {
                self.slider_val = (self.slider_val - 5.0).max(0.0);
            }
            Key::Right if self.main_tab == MainTab::Input && self.sub_tab_index == 4 => {
                self.slider_val = (self.slider_val + 5.0).min(100.0);
            }
            // General sub-tab navigation
            Key::Left => {
                let sub_tabs = self.main_tab.sub_tabs();
                if sub_tabs.len() > 0 {
                    self.sub_tab_index = if self.sub_tab_index == 0 {
                        sub_tabs.len() - 1
                    } else {
                        self.sub_tab_index - 1
                    };
                }
            }
            Key::Right => {
                let sub_tabs = self.main_tab.sub_tabs();
                if sub_tabs.len() > 0 {
                    self.sub_tab_index = (self.sub_tab_index + 1) % sub_tabs.len();
                }
            }
            // Input tab controls
            Key::Char('c') if self.main_tab == MainTab::Input => {
                self.checkbox_a = !self.checkbox_a;
            }
            Key::Char('s') if self.main_tab == MainTab::Input => {
                self.switch_a = !self.switch_a;
            }
            Key::Char('-') if self.main_tab == MainTab::Input && self.sub_tab_index == 4 => {
                self.rating_val = self.rating_val.saturating_sub(1);
            }
            Key::Char('+') | Key::Char('=')
                if self.main_tab == MainTab::Input && self.sub_tab_index == 4 =>
            {
                self.rating_val = (self.rating_val + 1).min(5);
            }
            Key::Up if self.main_tab == MainTab::Input && self.sub_tab_index == 3 => {
                self.radio_selected = self.radio_selected.saturating_sub(1);
            }
            Key::Down if self.main_tab == MainTab::Input && self.sub_tab_index == 3 => {
                self.radio_selected = (self.radio_selected + 1).min(2);
            }
            _ => {}
        }
        true
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
}

// ─── View Implementation ───────────────────────────────────────────────────────

impl View for Showcase {
    fn render(&self, ctx: &mut RenderContext) {
        let sub_tabs = self.main_tab.sub_tabs();
        let current_sub_tab = sub_tabs
            .get(self.sub_tab_index)
            .copied()
            .unwrap_or(SubTab::Button);

        let main_content = tabs::render_content(
            current_sub_tab,
            self.frame,
            self.cpu,
            self.memory,
            &self.net_in,
            &self.net_out,
            &self.wave_data,
            self.checkbox_a,
            self.switch_a,
            self.slider_val,
            self.rating_val,
            self.radio_selected,
        );

        vstack()
            .gap(0)
            .child(render_header(self.frame, self.main_tab))
            .child(Text::new(""))
            .child(render_main_tabs(self.main_tab))
            .child(Text::new(""))
            .child(render_sub_tabs(sub_tabs, self.sub_tab_index))
            .child(Text::new(""))
            .child(main_content)
            .child(Text::new(""))
            .child(render_footer(sub_tabs, self.sub_tab_index))
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
