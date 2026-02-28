//! Tab content modules

pub mod chart;
pub mod data;
pub mod developer;
pub mod display;
pub mod feedback;
pub mod input;
pub mod layout;

use crate::SubTab;
use revue::prelude::*;

pub fn render_content(
    sub_tab: SubTab,
    frame: u64,
    cpu: f64,
    memory: f64,
    net_in: &[f64],
    net_out: &[f64],
    wave_data: &[f64],
    checkbox_a: bool,
    switch_a: bool,
    slider_val: f64,
    rating_val: u8,
    radio_selected: usize,
) -> Box<dyn View> {
    match sub_tab {
        // Input
        SubTab::Button => Box::new(input::render_buttons()),
        SubTab::InputField => Box::new(input::render_input_fields()),
        SubTab::Toggle => Box::new(input::render_toggles(checkbox_a, switch_a)),
        SubTab::Select => Box::new(input::render_select(radio_selected)),
        SubTab::Slider => Box::new(input::render_sliders(slider_val, rating_val)),
        SubTab::Form => Box::new(input::render_forms()),
        SubTab::Picker => Box::new(input::render_pickers()),
        SubTab::Autocomplete => Box::new(input::render_autocomplete()),
        SubTab::Number => Box::new(input::render_number()),
        SubTab::Masked => Box::new(input::render_masked()),

        // Display
        SubTab::Text => Box::new(display::render_text()),
        SubTab::Status => Box::new(display::render_status(frame)),
        SubTab::Badge => Box::new(display::render_badges()),
        SubTab::Alert => Box::new(display::render_alerts()),
        SubTab::Progress => Box::new(display::render_progress(cpu, memory)),
        SubTab::Media => Box::new(display::render_media()),
        SubTab::Skeleton => Box::new(display::render_skeleton()),
        SubTab::Typography => Box::new(display::render_typography()),

        // Chart
        SubTab::Bar => Box::new(chart::render_bar(frame)),
        SubTab::Line => Box::new(chart::render_line(frame)),
        SubTab::Pie => Box::new(chart::render_pie()),
        SubTab::Spark => Box::new(chart::render_spark(net_in, net_out)),
        SubTab::Time => Box::new(chart::render_time()),
        SubTab::Special => Box::new(chart::render_special(wave_data)),

        // Data
        SubTab::Table => Box::new(data::render_table()),
        SubTab::Tree => Box::new(data::render_tree()),
        SubTab::List => Box::new(data::render_list()),
        SubTab::Calendar => Box::new(data::render_calendar()),
        SubTab::Timeline => Box::new(data::render_timeline()),
        SubTab::Viewer => Box::new(data::render_viewer(frame)),

        // Layout
        SubTab::Border => Box::new(layout::render_borders()),
        SubTab::Stack => Box::new(layout::render_stacks()),
        SubTab::Grid => Box::new(layout::render_grids()),
        SubTab::Split => Box::new(layout::render_splits()),
        SubTab::Container => Box::new(layout::render_containers()),
        SubTab::Nav => Box::new(layout::render_nav()),

        // Feedback
        SubTab::Modal => Box::new(feedback::render_modals()),
        SubTab::Toast => Box::new(feedback::render_toasts()),
        SubTab::Menu => Box::new(feedback::render_menus()),
        SubTab::Tooltip => Box::new(feedback::render_tooltips()),
        SubTab::Overlay => Box::new(feedback::render_overlays()),

        // Developer
        SubTab::Code => Box::new(developer::render_code()),
        SubTab::Terminal => Box::new(developer::render_terminal()),
        SubTab::Http => Box::new(developer::render_http()),
        SubTab::Ai => Box::new(developer::render_ai()),
        SubTab::Diff => Box::new(developer::render_diff()),
        SubTab::Monitor => Box::new(developer::render_monitor()),
    }
}
