//! Value parsing functions for CSS property values

use crate::style::{CalcExpr, Color, GridPlacement, GridTemplate, GridTrack, Size, Spacing};

/// Parse a length value (e.g., "100", "100px")
pub fn parse_length(value: &str) -> Option<u16> {
    let value = value.trim();
    if let Some(stripped) = value.strip_suffix("px") {
        stripped.trim().parse().ok()
    } else {
        value.parse().ok()
    }
}

/// Parse a signed length value (e.g., "-10", "10px")
pub fn parse_signed_length(value: &str) -> Option<i16> {
    let value = value.trim();
    if let Some(stripped) = value.strip_suffix("px") {
        stripped.trim().parse().ok()
    } else {
        value.parse().ok()
    }
}

/// Parse a size value (e.g., "auto", "100", "100px", "50%")
pub fn parse_size(value: &str) -> Size {
    let value = value.trim();
    if value == "auto" {
        Size::Auto
    } else if let Some(stripped) = value.strip_suffix('%') {
        stripped
            .trim()
            .parse()
            .map(Size::Percent)
            .unwrap_or(Size::Auto)
    } else {
        parse_length(value).map(Size::Fixed).unwrap_or(Size::Auto)
    }
}

/// Parse a color value (hex, rgb(), or named color)
pub fn parse_color(value: &str) -> Option<Color> {
    let value = value.trim();

    // Named colors
    match value.to_lowercase().as_str() {
        "white" => return Some(Color::WHITE),
        "black" => return Some(Color::BLACK),
        "red" => return Some(Color::RED),
        "green" => return Some(Color::GREEN),
        "blue" => return Some(Color::BLUE),
        "cyan" => return Some(Color::CYAN),
        "yellow" => return Some(Color::YELLOW),
        "magenta" => return Some(Color::MAGENTA),
        _ => {}
    }

    // Hex color
    if let Some(hex) = value.strip_prefix('#') {
        if hex.len() == 6 {
            if let Ok(v) = u32::from_str_radix(hex, 16) {
                return Some(Color::hex(v));
            }
        } else if hex.len() == 3 {
            // Short hex (#RGB -> #RRGGBB)
            let chars: Vec<char> = hex.chars().collect();
            let r = u8::from_str_radix(&format!("{}{}", chars[0], chars[0]), 16).ok()?;
            let g = u8::from_str_radix(&format!("{}{}", chars[1], chars[1]), 16).ok()?;
            let b = u8::from_str_radix(&format!("{}{}", chars[2], chars[2]), 16).ok()?;
            return Some(Color::rgb(r, g, b));
        }
    }

    // rgb(r, g, b)
    if value.starts_with("rgb(") && value.ends_with(')') {
        let inner = &value[4..value.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 3 {
            let r: u8 = parts[0].trim().parse().ok()?;
            let g: u8 = parts[1].trim().parse().ok()?;
            let b: u8 = parts[2].trim().parse().ok()?;
            return Some(Color::rgb(r, g, b));
        }
    }

    None
}

/// Parse a grid template like "1fr 2fr 1fr", "repeat(3, 1fr)", or "minmax(100px, 1fr)"
pub fn parse_grid_template(value: &str) -> GridTemplate {
    let value = value.trim();
    let mut tracks: Vec<GridTrack> = Vec::new();
    let bytes = value.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        // Skip whitespace (CSS grid values are ASCII-only)
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= bytes.len() {
            break;
        }

        // Check for repeat() function
        if value[pos..].starts_with("repeat(") {
            if let Some((repeat_tracks, new_pos)) = parse_repeat_function(&value[pos..]) {
                tracks.extend(repeat_tracks);
                pos += new_pos;
                continue;
            }
        }

        // Check for minmax() function
        if value[pos..].starts_with("minmax(") {
            if let Some((track, new_pos)) = parse_minmax_function(&value[pos..]) {
                tracks.push(track);
                pos += new_pos;
                continue;
            }
        }

        // Parse regular token (no spaces) - use byte slicing directly
        let start = pos;
        while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }

        if pos > start {
            // Direct slice - no allocation needed
            let token = &value[start..pos];
            if let Some(track) = parse_grid_track(token) {
                tracks.push(track);
            }
        }
    }

    GridTemplate::new(tracks)
}

/// Parse repeat(count, track) function
/// Returns (expanded tracks, bytes consumed)
fn parse_repeat_function(value: &str) -> Option<(Vec<GridTrack>, usize)> {
    const MAX_REPEAT_COUNT: usize = 10_000;

    let bytes = value.as_bytes();
    if !bytes.starts_with(b"repeat(") {
        return None;
    }

    // Find matching closing paren using byte iteration (ASCII-only)
    let mut paren_depth = 0;
    let mut end_pos = 0;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => paren_depth += 1,
            b')' => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    end_pos = i;
                    break;
                }
            }
            _ => {}
        }
    }

    if end_pos == 0 {
        return None;
    }

    // Extract content inside repeat()
    let inner = &value[7..end_pos]; // Skip "repeat("
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    if parts.len() != 2 {
        return None;
    }

    let count: usize = parts[0]
        .trim()
        .parse()
        .ok()
        .filter(|&c| c > 0 && c <= MAX_REPEAT_COUNT)?;
    let track_def = parts[1].trim();

    // Parse the track definition (could be a single track or minmax)
    let track = if track_def.starts_with("minmax(") {
        parse_minmax_function(track_def).map(|(t, _)| t)?
    } else {
        parse_grid_track(track_def)?
    };

    // Expand the repeat
    let tracks = vec![track; count];
    Some((tracks, end_pos + 1))
}

/// Parse minmax(min, max) function
/// Returns (GridTrack, bytes consumed)
fn parse_minmax_function(value: &str) -> Option<(GridTrack, usize)> {
    let bytes = value.as_bytes();
    if !bytes.starts_with(b"minmax(") {
        return None;
    }

    // Find matching closing paren using byte iteration (ASCII-only)
    let mut paren_depth = 0;
    let mut end_pos = 0;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => paren_depth += 1,
            b')' => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    end_pos = i;
                    break;
                }
            }
            _ => {}
        }
    }

    if end_pos == 0 {
        return None;
    }

    // Extract content inside minmax()
    let inner = &value[7..end_pos]; // Skip "minmax("
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    if parts.len() != 2 {
        return None;
    }

    let _min_track = parse_grid_track(parts[0].trim())?;
    let max_track = parse_grid_track(parts[1].trim())?;

    // For now, we use the max track value (simplified)
    // A full implementation would need MinMax variant in GridTrack
    Some((max_track, end_pos + 1))
}

/// Parse a single grid track value
fn parse_grid_track(value: &str) -> Option<GridTrack> {
    let value = value.trim();

    if value == "auto" {
        return Some(GridTrack::Auto);
    }
    if value == "min-content" {
        return Some(GridTrack::MinContent);
    }
    if value == "max-content" {
        return Some(GridTrack::MaxContent);
    }

    // Check for fr unit (e.g., "1fr", "2.5fr")
    if let Some(stripped) = value.strip_suffix("fr") {
        if let Ok(fr_val) = stripped.trim().parse::<f32>() {
            if fr_val > 0.0 {
                return Some(GridTrack::Fr(fr_val));
            }
        }
    }

    // Check for px unit
    if let Some(stripped) = value.strip_suffix("px") {
        if let Ok(px) = stripped.trim().parse::<u16>() {
            return Some(GridTrack::Fixed(px));
        }
    }

    // Try plain number
    if let Ok(n) = value.parse::<u16>() {
        return Some(GridTrack::Fixed(n));
    }

    None
}

/// Parse grid placement like "1", "1 / 3", or "span 2"
pub fn parse_grid_placement(value: &str) -> GridPlacement {
    let value = value.trim();

    // Check for "span N"
    if let Some(span_str) = value.strip_prefix("span") {
        if let Ok(n) = span_str.trim().parse::<i16>() {
            return GridPlacement::span(n);
        }
    }

    // Check for "start / end"
    if value.contains('/') {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() == 2 {
            let start = parts[0].trim().parse::<i16>().unwrap_or(0);
            let end_str = parts[1].trim();

            // Check if end is "span N"
            if let Some(span_str) = end_str.strip_prefix("span") {
                if let Ok(n) = span_str.trim().parse::<i16>() {
                    return GridPlacement { start, end: -n };
                }
            }

            let end = end_str.parse::<i16>().unwrap_or(0);
            return GridPlacement::from_to(start, end);
        }
    }

    // Single number - line position
    if let Ok(n) = value.parse::<i16>() {
        return GridPlacement::line(n);
    }

    GridPlacement::auto()
}

/// Parse spacing values with shorthand support
///
/// Supports CSS shorthand syntax for padding and margin:
/// - 1 value: all sides (e.g., `padding: 10px`)
/// - 2 values: vertical | horizontal (e.g., `padding: 10px 20px`)
/// - 3 values: top | horizontal | bottom (e.g., `padding: 10px 20px 5px`)
/// - 4 values: top | right | bottom | left (e.g., `padding: 10px 20px 15px 25px`)
///
/// # Examples
///
/// ```ignore
/// use revue::style::parser::parse_spacing;
///
/// // 1 value: all sides
/// let spacing = parse_spacing("10px");  // top:10, right:10, bottom:10, left:10
///
/// // 2 values: vertical | horizontal
/// let spacing = parse_spacing("10px 20px");  // top:10, right:20, bottom:10, left:20
///
/// // 3 values: top | horizontal | bottom
/// let spacing = parse_spacing("10px 20px 5px");  // top:10, right:20, bottom:5, left:20
///
/// // 4 values: top | right | bottom | left
/// let spacing = parse_spacing("10px 20px 15px 25px");  // top:10, right:20, bottom:15, left:25
/// ```
pub fn parse_spacing(value: &str) -> Option<Spacing> {
    let values: Vec<&str> = value.split_whitespace().collect();

    match values.len() {
        0 => None,
        1 => {
            // padding: 10px -> all sides
            let v = parse_length(values[0])?;
            Some(Spacing::all(v))
        }
        2 => {
            // padding: 10px 20px -> top/bottom: 10, left/right: 20
            let v = parse_length(values[0])?;
            let h = parse_length(values[1])?;
            Some(Spacing {
                top: v,
                right: h,
                bottom: v,
                left: h,
            })
        }
        3 => {
            // padding: 10px 20px 5px -> top: 10, left/right: 20, bottom: 5
            let top = parse_length(values[0])?;
            let h = parse_length(values[1])?;
            let bottom = parse_length(values[2])?;
            Some(Spacing {
                top,
                right: h,
                bottom,
                left: h,
            })
        }
        4 => {
            // padding: 10px 20px 15px 25px -> top: 10, right: 20, bottom: 15, left: 25
            let top = parse_length(values[0])?;
            let right = parse_length(values[1])?;
            let bottom = parse_length(values[2])?;
            let left = parse_length(values[3])?;
            Some(Spacing {
                top,
                right,
                bottom,
                left,
            })
        }
        _ => None,
    }
}

/// Parse a CSS calc() expression
///
/// Supports simplified two-operand expressions:
/// - `calc(100% - 20)` or `calc(100% - 20px)`
/// - `calc(50% + 10)`
/// - `calc(80 - 20)`
/// - `calc(100% * 0.5)`
/// - `calc(200 / 2)`
///
/// Returns `None` if parsing fails.
pub fn parse_calc(value: &str) -> Option<CalcExpr> {
    let value = value.trim();
    let inner = value.strip_prefix("calc(")?.strip_suffix(')')?.trim();

    // Find the operator by scanning for ` + `, ` - `, ` * `, ` / ` (with spaces)
    let (left_str, op, right_str) = find_operator(inner)?;

    let left = parse_calc_operand(left_str.trim())?;
    let right_trimmed = right_str.trim();

    match op {
        '+' => {
            let right = parse_calc_operand(right_trimmed)?;
            Some(CalcExpr::Add(Box::new(left), Box::new(right)))
        }
        '-' => {
            let right = parse_calc_operand(right_trimmed)?;
            Some(CalcExpr::Sub(Box::new(left), Box::new(right)))
        }
        '*' => {
            let scalar: f32 = right_trimmed.parse().ok()?;
            Some(CalcExpr::Mul(Box::new(left), scalar))
        }
        '/' => {
            let scalar: f32 = right_trimmed.parse().ok()?;
            Some(CalcExpr::Div(Box::new(left), scalar))
        }
        _ => None,
    }
}

/// Find the binary operator in a calc expression, returning (left, op, right)
fn find_operator(expr: &str) -> Option<(&str, char, &str)> {
    for op in ['+', '-', '*', '/'] {
        let pattern = format!(" {} ", op);
        if let Some(pos) = expr.find(&pattern) {
            let left = &expr[..pos];
            let right = &expr[pos + pattern.len()..];
            return Some((left, op, right));
        }
    }
    None
}

/// Parse a single operand in a calc expression (percentage or fixed value)
fn parse_calc_operand(value: &str) -> Option<CalcExpr> {
    let value = value.trim();
    if let Some(pct) = value.strip_suffix('%') {
        let v: f32 = pct.trim().parse().ok()?;
        Some(CalcExpr::Percent(v))
    } else if let Some(px) = value.strip_suffix("px") {
        let v: u16 = px.trim().parse().ok()?;
        Some(CalcExpr::Fixed(v))
    } else if let Ok(v) = value.parse::<u16>() {
        Some(CalcExpr::Fixed(v))
    } else {
        None
    }
}
