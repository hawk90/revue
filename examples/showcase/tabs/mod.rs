//! Tab content modules

pub mod chart;
pub mod data;
pub mod developer;
pub mod display;
pub mod feedback;
pub mod input;
pub mod layout;

use crate::example::Example;
use crate::SubTab;

pub fn get_examples(
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
) -> Vec<Example> {
    match sub_tab {
        // Input
        SubTab::Button => input::button_examples(),
        SubTab::InputField => input::input_field_examples(),
        SubTab::Toggle => input::toggle_examples(checkbox_a, switch_a),
        SubTab::Select => input::select_examples(radio_selected),
        SubTab::Slider => input::slider_examples(slider_val, rating_val),
        SubTab::Form => input::form_examples(),
        SubTab::Picker => input::picker_examples(),
        SubTab::Autocomplete => input::autocomplete_examples(),
        SubTab::Number => input::number_examples(),
        SubTab::Masked => input::masked_examples(),

        // Display
        SubTab::Text => display::text_examples(),
        SubTab::Status => display::status_examples(frame),
        SubTab::Badge => display::badge_examples(),
        SubTab::Alert => display::alert_examples(),
        SubTab::Progress => display::progress_examples(cpu, memory),
        SubTab::Media => display::media_examples(),
        SubTab::Skeleton => display::skeleton_examples(),
        SubTab::Typography => display::typography_examples(),

        // Chart
        SubTab::Bar => chart::bar_examples(frame),
        SubTab::Line => chart::line_examples(frame),
        SubTab::Pie => chart::pie_examples(),
        SubTab::Spark => chart::spark_examples(net_in, net_out),
        SubTab::Time => chart::time_examples(),
        SubTab::Special => chart::special_examples(wave_data),

        // Data
        SubTab::Table => data::table_examples(),
        SubTab::Tree => data::tree_examples(),
        SubTab::List => data::list_examples(),
        SubTab::Calendar => data::calendar_examples(),
        SubTab::Timeline => data::timeline_examples(),
        SubTab::Viewer => data::viewer_examples(frame),

        // Layout
        SubTab::Border => layout::border_examples(),
        SubTab::Stack => layout::stack_examples(),
        SubTab::Grid => layout::grid_examples(),
        SubTab::Split => layout::split_examples(),
        SubTab::Container => layout::container_examples(),
        SubTab::Nav => layout::nav_examples(),

        // Feedback
        SubTab::Modal => feedback::modal_examples(),
        SubTab::Toast => feedback::toast_examples(),
        SubTab::Menu => feedback::menu_examples(),
        SubTab::Tooltip => feedback::tooltip_examples(),
        SubTab::Overlay => feedback::overlay_examples(),

        // Developer
        SubTab::Code => developer::code_examples(),
        SubTab::Terminal => developer::terminal_examples(),
        SubTab::Http => developer::http_examples(),
        SubTab::Ai => developer::ai_examples(),
        SubTab::Diff => developer::diff_examples(),
        SubTab::Monitor => developer::monitor_examples(),
    }
}
