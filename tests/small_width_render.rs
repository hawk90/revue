//! Widgets render in areas, or with configured widths, smaller than their
//! chrome needs (#880).
//!
//! A narrow pane or a `.width(1)` builder call is a layout outcome, not a
//! programming error: the widget must draw what fits and never panic on a
//! `u16` subtraction that goes below zero.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    CodeEditor, Combobox, Divider, FilePicker, MultiSelect, Select, StreamLayer, Streamline,
    ThemePicker,
};

fn render(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    view.render(&mut RenderContext::new(&mut buffer, area));
    buffer
}

fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect()
}

#[test]
fn a_divider_whose_margins_meet_draws_nothing() {
    let buffer = render(&Divider::new().margin(5), 8, 1);
    assert_eq!(row_text(&buffer, 0), " ".repeat(8));
}

#[test]
fn a_labelled_divider_whose_margins_meet_draws_nothing() {
    let buffer = render(&Divider::new().margin(5).label("x"), 8, 1);
    assert_eq!(row_text(&buffer, 0), " ".repeat(8));
}

#[test]
fn a_vertical_divider_whose_margins_meet_draws_nothing() {
    let buffer = render(&Divider::vertical().margin(3), 1, 4);
    for y in 0..4 {
        assert_eq!(row_text(&buffer, y), " ");
    }
}

#[test]
fn a_divider_margin_past_the_area_draws_nothing() {
    let buffer = render(&Divider::new().margin(20).length(3), 8, 1);
    assert_eq!(row_text(&buffer, 0), " ".repeat(8));
}

fn streamline() -> Streamline {
    Streamline::new()
        .layer(StreamLayer::new("alpha").data(vec![1.0, 3.0, 2.0]))
        .layer(StreamLayer::new("beta").data(vec![2.0, 1.0, 4.0]))
}

#[test]
fn a_streamline_legend_fits_a_narrow_area() {
    for width in 5..10 {
        render(&streamline().show_legend(true), width, 6);
    }
}

#[test]
fn a_streamline_with_an_empty_palette_still_renders() {
    let chart = streamline().palette(vec![]);
    render(&chart, 30, 8);
    // Layers without their own color fall back to a visible default.
    assert_ne!(chart.get_layer_color(0), chart.get_layer_color(1));
}

#[test]
fn a_streamline_keeps_a_custom_palette() {
    let chart = streamline().palette(vec![Color::RED]);
    assert_eq!(chart.get_layer_color(0), Color::RED);
    assert_eq!(chart.get_layer_color(1), Color::RED);
}

#[test]
fn dropdowns_with_a_tiny_configured_width_still_render() {
    for width in 0..3 {
        render(
            &Select::new().options(vec!["one", "two"]).width(width),
            20,
            3,
        );
        render(&Combobox::new().options(["one", "two"]).width(width), 20, 3);
        render(
            &MultiSelect::new().options(vec!["one", "two"]).width(width),
            20,
            3,
        );
    }
}

#[test]
fn a_select_with_a_tiny_width_still_shows_its_arrow() {
    let buffer = render(&Select::new().options(vec!["one"]).width(1), 20, 1);
    assert!(row_text(&buffer, 0).contains('▼'));
}

#[test]
fn a_file_picker_with_a_tiny_width_still_renders() {
    for width in 0..4 {
        render(&FilePicker::new().width(width), 30, 10);
    }
}

#[test]
fn a_theme_picker_with_a_tiny_width_still_renders() {
    for width in 0..10 {
        render(&ThemePicker::new().width(width), 30, 6);
        render(&ThemePicker::new().width(width).show_preview(false), 30, 6);
    }
}

#[test]
fn a_code_editor_minimap_wider_than_the_area_still_renders() {
    let editor = CodeEditor::new().content("fn main() {}").minimap(true);
    for width in 1..10 {
        render(&editor, width, 3);
    }
}
