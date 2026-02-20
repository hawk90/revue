//! CSS property application functions

use crate::style::parser::parse_spacing;
use crate::style::parser::value_parsers::{
    parse_calc, parse_color, parse_grid_placement, parse_grid_template, parse_signed_length,
    parse_size,
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

/// CSS animation properties parsed from declarations
#[derive(Default)]
struct CssAnimationProperties {
    name: Option<String>,
    duration: Option<std::time::Duration>,
    easing: Option<crate::style::Easing>,
    delay: Option<std::time::Duration>,
    iteration_count: Option<u32>,
    direction: Option<crate::style::AnimationDirection>,
    fill_mode: Option<crate::style::AnimationFillMode>,
}

/// Parse an animation shorthand value like "fadeIn 0.3s ease-in-out infinite alternate"
fn parse_animation_shorthand(value: &str) -> CssAnimationProperties {
    let mut props = CssAnimationProperties::default();
    let mut found_duration = false;

    for part in value.split_whitespace() {
        // Try parsing as duration
        if let Some(dur) = crate::style::transition::parse_duration(part) {
            if !found_duration {
                props.duration = Some(dur);
                found_duration = true;
            } else {
                props.delay = Some(dur);
            }
            continue;
        }

        // Try parsing as easing
        if let Some(easing) = crate::style::Easing::parse(part) {
            props.easing = Some(easing);
            continue;
        }

        // Try parsing as iteration count
        if part == "infinite" {
            props.iteration_count = Some(0); // 0 = infinite in KeyframeAnimation
            continue;
        }
        if let Ok(n) = part.parse::<u32>() {
            props.iteration_count = Some(n);
            continue;
        }

        // Try parsing as direction
        match part {
            "normal" => {
                props.direction = Some(crate::style::AnimationDirection::Normal);
                continue;
            }
            "reverse" => {
                props.direction = Some(crate::style::AnimationDirection::Reverse);
                continue;
            }
            "alternate" => {
                props.direction = Some(crate::style::AnimationDirection::Alternate);
                continue;
            }
            "alternate-reverse" => {
                props.direction = Some(crate::style::AnimationDirection::AlternateReverse);
                continue;
            }
            _ => {}
        }

        // Try parsing as fill mode
        match part {
            "forwards" => {
                props.fill_mode = Some(crate::style::AnimationFillMode::Forwards);
                continue;
            }
            "backwards" => {
                props.fill_mode = Some(crate::style::AnimationFillMode::Backwards);
                continue;
            }
            "both" => {
                props.fill_mode = Some(crate::style::AnimationFillMode::Both);
                continue;
            }
            "none" if props.name.is_some() => {
                // "none" as fill mode only if we already have a name
                props.fill_mode = Some(crate::style::AnimationFillMode::None);
                continue;
            }
            _ => {}
        }

        // Otherwise treat as animation name (first unrecognized token)
        if props.name.is_none() {
            props.name = Some(part.to_string());
        }
    }

    props
}

/// Convert a CSS value to f32 for keyframe interpolation
fn css_value_to_f32(value: &str) -> Option<f32> {
    let value = value.trim();
    // Direct number
    if let Ok(v) = value.parse::<f32>() {
        return Some(v);
    }
    // px values
    if let Some(num) = value.strip_suffix("px") {
        return num.trim().parse::<f32>().ok();
    }
    // percentage
    if let Some(num) = value.strip_suffix('%') {
        return num.trim().parse::<f32>().ok().map(|v| v / 100.0);
    }
    None
}

/// Resolve animation for a selector by looking up declarations and @keyframes
pub(super) fn resolve_animation(
    sheet: &super::types::StyleSheet,
    selector: &str,
) -> Option<crate::style::animation::KeyframeAnimation> {
    use crate::style::animation::{easing as easing_fns, KeyframeAnimation};

    // Collect animation properties from selector's declarations
    let mut anim_props = CssAnimationProperties::default();
    let mut found_shorthand = false;

    for rule in sheet.rules(selector) {
        for decl in &rule.declarations {
            match decl.property.as_str() {
                "animation" => {
                    anim_props = parse_animation_shorthand(&decl.value);
                    found_shorthand = true;
                }
                "animation-name" => anim_props.name = Some(decl.value.clone()),
                "animation-duration" => {
                    anim_props.duration = crate::style::transition::parse_duration(&decl.value);
                }
                "animation-timing-function" => {
                    anim_props.easing = crate::style::Easing::parse(&decl.value);
                }
                "animation-delay" => {
                    anim_props.delay = crate::style::transition::parse_duration(&decl.value);
                }
                "animation-iteration-count" => {
                    if decl.value.trim() == "infinite" {
                        anim_props.iteration_count = Some(0);
                    } else {
                        anim_props.iteration_count = decl.value.trim().parse::<u32>().ok();
                    }
                }
                "animation-direction" => {
                    anim_props.direction = match decl.value.trim() {
                        "normal" => Some(crate::style::AnimationDirection::Normal),
                        "reverse" => Some(crate::style::AnimationDirection::Reverse),
                        "alternate" => Some(crate::style::AnimationDirection::Alternate),
                        "alternate-reverse" => {
                            Some(crate::style::AnimationDirection::AlternateReverse)
                        }
                        _ => None,
                    };
                }
                "animation-fill-mode" => {
                    anim_props.fill_mode = match decl.value.trim() {
                        "none" => Some(crate::style::AnimationFillMode::None),
                        "forwards" => Some(crate::style::AnimationFillMode::Forwards),
                        "backwards" => Some(crate::style::AnimationFillMode::Backwards),
                        "both" => Some(crate::style::AnimationFillMode::Both),
                        _ => None,
                    };
                }
                _ => {}
            }
        }
    }

    // Need at least an animation name
    let anim_name = anim_props.name.as_ref()?;

    // Look up @keyframes definition
    let keyframes_def = sheet.keyframes_definition(anim_name)?;

    // Build KeyframeAnimation from the definition
    let mut anim = KeyframeAnimation::new(anim_name);

    // Add keyframes
    for block in &keyframes_def.keyframes {
        anim = anim.keyframe(block.percent, |mut kf| {
            for decl in &block.declarations {
                if let Some(val) = css_value_to_f32(&decl.value) {
                    kf = kf.set(&decl.property, val);
                }
            }
            kf
        });
    }

    // Apply animation properties
    if let Some(duration) = anim_props.duration {
        anim = anim.duration(duration);
    }
    if let Some(delay) = anim_props.delay {
        anim = anim.delay(delay);
    }
    if let Some(easing) = anim_props.easing {
        let easing_fn = match easing {
            crate::style::Easing::Linear => easing_fns::linear,
            crate::style::Easing::EaseIn => easing_fns::ease_in,
            crate::style::Easing::EaseOut => easing_fns::ease_out,
            crate::style::Easing::EaseInOut => easing_fns::ease_in_out,
            // CubicBezier falls back to ease_in_out for now
            crate::style::Easing::CubicBezier(..) => easing_fns::ease_in_out,
        };
        anim = anim.easing(easing_fn);
    }
    if let Some(count) = anim_props.iteration_count {
        if count == 0 {
            anim = anim.infinite();
        } else {
            anim = anim.iterations(count);
        }
    }
    if let Some(direction) = anim_props.direction {
        anim = anim.direction(direction);
    }
    if let Some(fill_mode) = anim_props.fill_mode {
        anim = anim.fill_mode(fill_mode);
    }

    // Suppress unused warning for found_shorthand
    let _ = found_shorthand;

    Some(anim)
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
