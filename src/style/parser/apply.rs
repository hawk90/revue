//! CSS property application functions

use crate::style::parser::parse_spacing;
use crate::style::parser::value_parsers::{
    parse_color, parse_grid_placement, parse_grid_template, parse_signed_length, parse_size,
};
use crate::style::Style;
use crate::style::{AlignItems, BorderStyle, Display, FlexDirection, JustifyContent, Position};
use std::collections::HashMap;

/// Apply a declaration to a style
pub fn apply_declaration(
    style: &mut Style,
    property: &str,
    value: &str,
    vars: &HashMap<String, String>,
) {
    // Resolve CSS variable if needed
    let value = if value.starts_with("var(") && value.ends_with(')') {
        let var_name = &value[4..value.len() - 1];
        vars.get(var_name).map(|s| s.as_str()).unwrap_or(value)
    } else {
        value
    };

    // Try each category of properties
    if apply_display_layout(style, property, value) {
        return;
    }
    if apply_grid_properties(style, property, value) {
        return;
    }
    if apply_position_offsets(style, property, value) {
        return;
    }
    if apply_sizing(style, property, value) {
        return;
    }
    apply_visual(style, property, value);
}

/// Apply display and flexbox layout properties
fn apply_display_layout(style: &mut Style, property: &str, value: &str) -> bool {
    match property {
        "display" => {
            style.layout.display = match value {
                "flex" => Display::Flex,
                "block" => Display::Block,
                "grid" => Display::Grid,
                "none" => Display::None,
                _ => return false,
            };
            true
        }
        "position" => {
            style.layout.position = match value {
                "static" => Position::Static,
                "relative" => Position::Relative,
                "absolute" => Position::Absolute,
                "fixed" => Position::Fixed,
                _ => return false,
            };
            true
        }
        "flex-direction" => {
            style.layout.flex_direction = match value {
                "row" => FlexDirection::Row,
                "column" => FlexDirection::Column,
                _ => return false,
            };
            true
        }
        "justify-content" => {
            style.layout.justify_content = match value {
                "start" | "flex-start" => JustifyContent::Start,
                "center" => JustifyContent::Center,
                "end" | "flex-end" => JustifyContent::End,
                "space-between" => JustifyContent::SpaceBetween,
                "space-around" => JustifyContent::SpaceAround,
                _ => return false,
            };
            true
        }
        "align-items" => {
            style.layout.align_items = match value {
                "start" | "flex-start" => AlignItems::Start,
                "center" => AlignItems::Center,
                "end" | "flex-end" => AlignItems::End,
                "stretch" => AlignItems::Stretch,
                _ => return false,
            };
            true
        }
        _ => false,
    }
}

/// Apply CSS Grid properties
fn apply_grid_properties(style: &mut Style, property: &str, value: &str) -> bool {
    match property {
        "grid-template-columns" => {
            style.layout.grid_template_columns = parse_grid_template(value);
            true
        }
        "grid-template-rows" => {
            style.layout.grid_template_rows = parse_grid_template(value);
            true
        }
        "grid-column" => {
            style.layout.grid_column = parse_grid_placement(value);
            true
        }
        "grid-row" => {
            style.layout.grid_row = parse_grid_placement(value);
            true
        }
        _ => false,
    }
}

/// Apply position offset properties (top, right, bottom, left, z-index)
fn apply_position_offsets(style: &mut Style, property: &str, value: &str) -> bool {
    match property {
        "top" => {
            if let Some(v) = parse_signed_length(value) {
                style.spacing.top = Some(v);
                return true;
            }
            false
        }
        "right" => {
            if let Some(v) = parse_signed_length(value) {
                style.spacing.right = Some(v);
                return true;
            }
            false
        }
        "bottom" => {
            if let Some(v) = parse_signed_length(value) {
                style.spacing.bottom = Some(v);
                return true;
            }
            false
        }
        "left" => {
            if let Some(v) = parse_signed_length(value) {
                style.spacing.left = Some(v);
                return true;
            }
            false
        }
        "z-index" => {
            if let Ok(v) = value.parse::<i16>() {
                style.visual.z_index = v;
                return true;
            }
            false
        }
        _ => false,
    }
}

/// Apply sizing properties (width, height, padding, margin)
fn apply_sizing(style: &mut Style, property: &str, value: &str) -> bool {
    match property {
        "padding" => {
            if let Some(spacing) = parse_spacing(value) {
                style.spacing.padding = spacing;
                return true;
            }
            false
        }
        "margin" => {
            if let Some(spacing) = parse_spacing(value) {
                style.spacing.margin = spacing;
                return true;
            }
            false
        }
        "width" => {
            style.sizing.width = parse_size(value);
            true
        }
        "height" => {
            style.sizing.height = parse_size(value);
            true
        }
        "min-width" => {
            style.sizing.min_width = parse_size(value);
            true
        }
        "max-width" => {
            style.sizing.max_width = parse_size(value);
            true
        }
        "min-height" => {
            style.sizing.min_height = parse_size(value);
            true
        }
        "max-height" => {
            style.sizing.max_height = parse_size(value);
            true
        }
        _ => false,
    }
}

/// Apply visual properties (colors, border, opacity, visibility)
fn apply_visual(style: &mut Style, property: &str, value: &str) {
    match property {
        "border-style" | "border" => {
            style.visual.border_style = match value {
                "none" => BorderStyle::None,
                "solid" => BorderStyle::Solid,
                "dashed" => BorderStyle::Dashed,
                "double" => BorderStyle::Double,
                "rounded" => BorderStyle::Rounded,
                _ => return,
            };
        }
        "border-color" => {
            if let Some(c) = parse_color(value) {
                style.visual.border_color = c;
            }
        }
        "color" => {
            if let Some(c) = parse_color(value) {
                style.visual.color = c;
            }
        }
        "background" | "background-color" => {
            if let Some(c) = parse_color(value) {
                style.visual.background = c;
            }
        }
        "opacity" => {
            if let Ok(v) = value.parse::<f32>() {
                style.visual.opacity = v.clamp(0.0, 1.0);
            }
        }
        "visible" | "visibility" => {
            style.visual.visible = value != "hidden" && value != "false";
        }
        _ => {} // Unknown property, ignore
    }
}
