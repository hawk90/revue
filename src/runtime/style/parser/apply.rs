//! CSS property application functions

use super::vars::substitute_vars;
use crate::style::parser::parse_spacing;
use crate::style::parser::value_parsers::{
    parse_calc, parse_color, parse_grid_placement, parse_grid_template, parse_length,
    parse_signed_length, parse_size,
};
use crate::style::Style;
use crate::style::{
    AlignItems, AlignSelf, BorderStyle, Display, FlexDirection, FlexWrap, FontWeight,
    JustifyContent, Position, TextAlign, TextDecoration,
};
use std::collections::HashMap;

/// Apply a declaration to a style
pub fn apply_declaration(
    style: &mut Style,
    property: &str,
    value: &str,
    vars: &HashMap<String, String>,
) {
    // Variables first, so a shorthand parses the substituted tokens.
    let value = substitute_vars(value, vars);
    let value = value.as_ref();

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
                "sticky" => Position::Sticky,
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
        "flex-grow" => {
            if let Ok(v) = value.parse::<f32>() {
                style.layout.flex_grow = v.max(0.0);
                return true;
            }
            false
        }
        "flex" => {
            // Shorthand: flex: <grow> or flex: <grow> <shrink> <basis>
            // We only support flex-grow for now
            let first = value.split_whitespace().next().unwrap_or("0");
            if let Ok(v) = first.parse::<f32>() {
                style.layout.flex_grow = v.max(0.0);
                return true;
            }
            false
        }
        "flex-wrap" => {
            style.layout.flex_wrap = match value {
                "nowrap" => FlexWrap::NoWrap,
                "wrap" => FlexWrap::Wrap,
                "wrap-reverse" => FlexWrap::WrapReverse,
                _ => return false,
            };
            true
        }
        "align-self" => {
            style.layout.align_self = match value {
                "auto" => AlignSelf::Auto,
                "start" | "flex-start" => AlignSelf::Start,
                "center" => AlignSelf::Center,
                "end" | "flex-end" => AlignSelf::End,
                "stretch" => AlignSelf::Stretch,
                _ => return false,
            };
            true
        }
        "order" => {
            if let Ok(v) = value.parse::<i16>() {
                style.layout.order = v;
                return true;
            }
            false
        }
        "gap" => {
            if let Ok(v) = value.trim().parse::<u16>() {
                style.layout.gap = Some(v);
                return true;
            }
            false
        }
        "column-gap" => {
            if let Ok(v) = value.trim().parse::<u16>() {
                style.layout.column_gap = Some(v);
                return true;
            }
            false
        }
        "row-gap" => {
            if let Ok(v) = value.trim().parse::<u16>() {
                style.layout.row_gap = Some(v);
                return true;
            }
            false
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

/// Parse a size value, resolving calc() expressions immediately to a fixed value.
///
/// If the value starts with "calc(", it is parsed as a calc expression and
/// resolved against a default parent size of 80 (standard terminal width).
/// Otherwise, it falls back to the normal `parse_size` behavior.
fn parse_size_or_calc(value: &str) -> crate::style::Size {
    if value.trim().starts_with("calc(") {
        if let Some(expr) = parse_calc(value) {
            // Resolve immediately with a default parent size of 80 columns
            return expr.to_size(80);
        }
    }
    parse_size(value)
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
        "padding-top" | "padding-right" | "padding-bottom" | "padding-left" | "margin-top"
        | "margin-right" | "margin-bottom" | "margin-left" => {
            let Some(v) = parse_length(value) else {
                return false;
            };
            let (side, spacing) = match property.split_once('-') {
                Some(("padding", side)) => (side, &mut style.spacing.padding),
                Some((_, side)) => (side, &mut style.spacing.margin),
                None => return false,
            };
            match side {
                "top" => spacing.top = v,
                "right" => spacing.right = v,
                "bottom" => spacing.bottom = v,
                _ => spacing.left = v,
            }
            true
        }
        "width" => {
            style.sizing.width = parse_size_or_calc(value);
            true
        }
        "height" => {
            style.sizing.height = parse_size_or_calc(value);
            true
        }
        "min-width" => {
            style.sizing.min_width = parse_size_or_calc(value);
            true
        }
        "max-width" => {
            style.sizing.max_width = parse_size_or_calc(value);
            true
        }
        "min-height" => {
            style.sizing.min_height = parse_size_or_calc(value);
            true
        }
        "max-height" => {
            style.sizing.max_height = parse_size_or_calc(value);
            true
        }
        _ => false,
    }
}

/// Apply visual properties (colors, border, opacity, visibility)
fn apply_visual(style: &mut Style, property: &str, value: &str) {
    match property {
        "border-style" => {
            style.visual.border_style = Some(match value {
                "none" => BorderStyle::None,
                "solid" => BorderStyle::Solid,
                "dashed" => BorderStyle::Dashed,
                "double" => BorderStyle::Double,
                "rounded" => BorderStyle::Rounded,
                _ => return,
            });
        }
        "border" => {
            // Shorthand: border: <style> [color] or border: <style>
            let parts: Vec<&str> = value.split_whitespace().collect();
            for part in &parts {
                match *part {
                    "none" => style.visual.border_style = Some(BorderStyle::None),
                    "solid" => style.visual.border_style = Some(BorderStyle::Solid),
                    "dashed" => style.visual.border_style = Some(BorderStyle::Dashed),
                    "double" => style.visual.border_style = Some(BorderStyle::Double),
                    "rounded" => style.visual.border_style = Some(BorderStyle::Rounded),
                    _ => {
                        if let Some(c) = parse_color(part) {
                            style.visual.border_color = c;
                        }
                    }
                }
            }
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
        "text-align" => {
            style.visual.text_align = match value {
                "left" | "start" => TextAlign::Left,
                "center" => TextAlign::Center,
                "right" | "end" => TextAlign::Right,
                _ => return,
            };
        }
        "font-weight" => {
            style.visual.font_weight = match value {
                "bold" | "700" | "800" | "900" => FontWeight::Bold,
                "normal" | "400" => FontWeight::Normal,
                _ => return,
            };
        }
        "text-decoration" | "text-decoration-line" => {
            let mut decoration = TextDecoration::default();
            for part in value.split_whitespace() {
                match part {
                    "underline" => decoration.underline = true,
                    "line-through" => decoration.line_through = true,
                    "none" => {
                        decoration = TextDecoration::default();
                        break;
                    }
                    _ => {}
                }
            }
            style.visual.text_decoration = decoration;
        }
        "overflow" | "overflow-x" | "overflow-y" => {
            style.visual.overflow = match value {
                "visible" => crate::style::Overflow::Visible,
                "hidden" => crate::style::Overflow::Hidden,
                "scroll" => crate::style::Overflow::Scroll,
                "auto" => crate::style::Overflow::Auto,
                _ => return,
            };
        }
        _ => {} // Unknown property, ignore
    }
}
