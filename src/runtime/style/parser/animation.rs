//! CSS animation declarations resolved against `@keyframes`

/// `animation-iteration-count`
///
/// Its own type because `KeyframeAnimation` spells "infinite" as 0, and in CSS
/// a count of 0 means the animation does not run at all.
#[derive(Clone, Copy)]
enum Iterations {
    Count(u32),
    Infinite,
}

/// CSS animation properties parsed from declarations
#[derive(Default)]
struct CssAnimationProperties {
    name: Option<String>,
    duration: Option<std::time::Duration>,
    easing: Option<crate::style::Easing>,
    delay: Option<std::time::Duration>,
    iterations: Option<Iterations>,
    direction: Option<crate::style::AnimationDirection>,
    fill_mode: Option<crate::style::AnimationFillMode>,
}

/// Parse an animation shorthand value like "fadeIn 0.3s ease-in-out infinite alternate"
fn parse_animation_shorthand(value: &str) -> CssAnimationProperties {
    let mut props = CssAnimationProperties::default();
    let mut found_duration = false;

    for part in crate::style::transition::split_outside_parens(value, char::is_whitespace) {
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
            props.iterations = Some(Iterations::Infinite);
            continue;
        }
        if let Ok(n) = part.parse::<u32>() {
            props.iterations = Some(Iterations::Count(n));
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

    for rule in sheet.rules(selector) {
        for decl in &rule.declarations {
            match decl.property.as_str() {
                "animation" => {
                    anim_props = parse_animation_shorthand(&decl.value);
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
                    anim_props.iterations = match decl.value.trim() {
                        "infinite" => Some(Iterations::Infinite),
                        count => count.parse().ok().map(Iterations::Count),
                    };
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

    // A count of 0 runs nothing, as in CSS
    if matches!(anim_props.iterations, Some(Iterations::Count(0))) {
        return None;
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
        anim = match easing {
            crate::style::Easing::Linear => anim.easing(easing_fns::linear),
            crate::style::Easing::EaseIn => anim.easing(easing_fns::ease_in),
            crate::style::Easing::EaseOut => anim.easing(easing_fns::ease_out),
            crate::style::Easing::EaseInOut => anim.easing(easing_fns::ease_in_out),
            crate::style::Easing::CubicBezier(x1, y1, x2, y2) => anim.cubic_bezier(x1, y1, x2, y2),
        };
    }
    match anim_props.iterations {
        Some(Iterations::Infinite) => anim = anim.infinite(),
        Some(Iterations::Count(count)) => anim = anim.iterations(count),
        None => {}
    }
    if let Some(direction) = anim_props.direction {
        anim = anim.direction(direction);
    }
    if let Some(fill_mode) = anim_props.fill_mode {
        anim = anim.fill_mode(fill_mode);
    }

    Some(anim)
}
