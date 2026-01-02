//! CSS parser for TUI styling

use std::collections::HashMap;
use super::{
    ParseError, ErrorCode, Style, Display, FlexDirection, JustifyContent, AlignItems,
    Size, Spacing, BorderStyle, Color, Position, GridTemplate, GridTrack, GridPlacement,
};

/// Create a ParseError at the given position
fn make_error(css: &str, pos: usize, message: &str, code: ErrorCode) -> ParseError {
    ParseError::at_offset(message, css, pos).with_code(code)
}

/// Create a ParseError for missing brace
fn missing_brace_error(css: &str, pos: usize, expected: char) -> ParseError {
    make_error(
        css,
        pos,
        &format!("expected '{}' but found end of input", expected),
        ErrorCode::MissingBrace,
    )
}

/// A parsed stylesheet
#[derive(Debug, Default, Clone)]
pub struct StyleSheet {
    /// CSS rules
    pub rules: Vec<Rule>,
    /// CSS variables
    pub variables: HashMap<String, String>,
}

/// A CSS rule (selector + declarations)
#[derive(Debug, Clone)]
pub struct Rule {
    /// Selector string (e.g., ".class", "#id", "widget")
    pub selector: String,
    /// Declarations in this rule
    pub declarations: Vec<Declaration>,
}

/// A CSS declaration (property: value)
#[derive(Debug, Clone)]
pub struct Declaration {
    /// Property name
    pub property: String,
    /// Property value
    pub value: String,
}

impl StyleSheet {
    /// Create a new empty stylesheet
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge another stylesheet into this one
    pub fn merge(&mut self, other: StyleSheet) {
        self.rules.extend(other.rules);
        self.variables.extend(other.variables);
    }

    /// Get a CSS variable value
    pub fn variable(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }

    /// Get rules matching a selector
    pub fn rules(&self, selector: &str) -> Vec<&Rule> {
        self.rules.iter().filter(|r| r.selector == selector).collect()
    }

    /// Apply stylesheet to a base style for a given selector
    pub fn apply(&self, selector: &str, base: &Style) -> Style {
        let mut style = base.clone();

        for rule in self.rules(selector) {
            for decl in &rule.declarations {
                apply_declaration(&mut style, &decl.property, &decl.value, &self.variables);
            }
        }

        style
    }
}

/// Parse CSS text into a StyleSheet
///
/// This parser uses zero-copy `&str` slicing for efficiency, avoiding `Vec<char>` allocation.
pub fn parse(css: &str) -> Result<StyleSheet, ParseError> {
    let mut sheet = StyleSheet::new();
    let bytes = css.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        // Skip whitespace and comments
        pos = skip_whitespace_bytes(bytes, pos);
        if pos >= bytes.len() {
            break;
        }

        // Check for CSS variable definition (in :root)
        if bytes[pos..].starts_with(b":root") {
            pos = parse_root_variables_str(css, pos, &mut sheet)?;
            continue;
        }

        // Parse selector
        let (selector, new_pos) = parse_selector_str(css, pos)?;
        pos = new_pos;

        // Skip whitespace
        pos = skip_whitespace_bytes(bytes, pos);

        // Expect '{'
        if pos >= bytes.len() || bytes[pos] != b'{' {
            return Err(make_error(
                css, pos,
                &format!("expected '{{' after selector '{}', found '{}'", selector,
                    if pos < bytes.len() { bytes[pos] as char } else { ' ' }),
                ErrorCode::MissingBrace,
            ));
        }
        pos += 1;

        // Parse declarations
        let (declarations, new_pos) = parse_declarations_str(css, pos)?;
        pos = new_pos;

        // Expect '}'
        if pos >= bytes.len() || bytes[pos] != b'}' {
            return Err(missing_brace_error(css, pos, '}'));
        }
        pos += 1;

        sheet.rules.push(Rule {
            selector,
            declarations,
        });
    }

    Ok(sheet)
}

// ─────────────────────────────────────────────────────────────────────────────
// Zero-copy helper functions (using &str/bytes slicing)
// ─────────────────────────────────────────────────────────────────────────────

/// Skip ASCII whitespace using byte slice (no allocation)
#[inline]
fn skip_whitespace_bytes(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

/// Skip whitespace and block comments using byte slice (no allocation)
fn skip_whitespace_and_comments_bytes(bytes: &[u8], mut pos: usize) -> usize {
    loop {
        pos = skip_whitespace_bytes(bytes, pos);
        if pos + 1 < bytes.len() && bytes[pos] == b'/' && bytes[pos + 1] == b'*' {
            // Skip block comment
            pos += 2;
            while pos + 1 < bytes.len() && !(bytes[pos] == b'*' && bytes[pos + 1] == b'/') {
                pos += 1;
            }
            if pos + 1 < bytes.len() {
                pos += 2;
            }
        } else {
            break;
        }
    }
    pos
}

/// Parse :root variables block using zero-copy str slicing
fn parse_root_variables_str(
    css: &str,
    mut pos: usize,
    sheet: &mut StyleSheet,
) -> Result<usize, ParseError> {
    let bytes = css.as_bytes();

    // Skip ":root"
    pos += 5;
    pos = skip_whitespace_bytes(bytes, pos);

    // Expect '{'
    if pos >= bytes.len() || bytes[pos] != b'{' {
        return Err(make_error(css, pos, "expected '{' after :root", ErrorCode::MissingBrace));
    }
    pos += 1;

    // Parse variable declarations
    loop {
        pos = skip_whitespace_and_comments_bytes(bytes, pos);

        if pos >= bytes.len() {
            return Err(missing_brace_error(css, pos, '}'));
        }

        if bytes[pos] == b'}' {
            pos += 1;
            break;
        }

        // Variable name starts with --
        if !bytes[pos..].starts_with(b"--") {
            return Err(make_error(
                css, pos,
                "CSS variables must start with '--' (e.g., --primary-color)",
                ErrorCode::InvalidSyntax,
            ).suggest("use '--variable-name: value;' format"));
        }

        // Read variable name (ASCII only, safe to use byte indexing)
        let start = pos;
        while pos < bytes.len() && bytes[pos] != b':' && !bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        let name = css[start..pos].to_string();

        pos = skip_whitespace_bytes(bytes, pos);

        // Expect ':'
        if pos >= bytes.len() || bytes[pos] != b':' {
            return Err(make_error(
                css, pos,
                "expected ':' after variable name",
                ErrorCode::InvalidSyntax,
            ).suggest("format: --variable-name: value;"));
        }
        pos += 1;

        pos = skip_whitespace_bytes(bytes, pos);

        // Read value until ';' or '}'
        let start = pos;
        while pos < bytes.len() && bytes[pos] != b';' && bytes[pos] != b'}' {
            pos += 1;
        }
        let value = css[start..pos].trim().to_string();

        sheet.variables.insert(name, value);

        if pos < bytes.len() && bytes[pos] == b';' {
            pos += 1;
        }
    }

    Ok(pos)
}

/// Parse selector using zero-copy str slicing
fn parse_selector_str(css: &str, mut pos: usize) -> Result<(String, usize), ParseError> {
    let bytes = css.as_bytes();
    let start = pos;
    while pos < bytes.len() && bytes[pos] != b'{' {
        pos += 1;
    }
    Ok((css[start..pos].trim().to_string(), pos))
}

/// Parse declarations block using zero-copy str slicing
fn parse_declarations_str(
    css: &str,
    mut pos: usize,
) -> Result<(Vec<Declaration>, usize), ParseError> {
    let bytes = css.as_bytes();
    let mut declarations = Vec::new();

    loop {
        pos = skip_whitespace_and_comments_bytes(bytes, pos);

        if pos >= bytes.len() || bytes[pos] == b'}' {
            break;
        }

        // Read property name
        let start = pos;
        while pos < bytes.len() && bytes[pos] != b':' && bytes[pos] != b'}' {
            pos += 1;
        }
        let property = css[start..pos].trim().to_string();

        if pos >= bytes.len() || bytes[pos] == b'}' {
            break;
        }

        // Skip ':'
        pos += 1;
        pos = skip_whitespace_bytes(bytes, pos);

        // Read value until ';' or '}'
        let start = pos;
        let mut paren_depth: i32 = 0;
        while pos < bytes.len() {
            match bytes[pos] {
                b'(' => paren_depth += 1,
                b')' => paren_depth = paren_depth.saturating_sub(1),
                b';' | b'}' if paren_depth == 0 => break,
                _ => {}
            }
            pos += 1;
        }
        let value = css[start..pos].trim().to_string();

        if !property.is_empty() {
            declarations.push(Declaration { property, value });
        }

        if pos < bytes.len() && bytes[pos] == b';' {
            pos += 1;
        }
    }

    Ok((declarations, pos))
}

/// Apply a declaration to a style
pub fn apply_declaration(style: &mut Style, property: &str, value: &str, vars: &HashMap<String, String>) {
    // Resolve CSS variable if needed
    let value = if value.starts_with("var(") && value.ends_with(')') {
        let var_name = &value[4..value.len() - 1];
        vars.get(var_name).map(|s| s.as_str()).unwrap_or(value)
    } else {
        value
    };

    // Try each category of properties
    if apply_display_layout(style, property, value) { return; }
    if apply_grid_properties(style, property, value) { return; }
    if apply_position_offsets(style, property, value) { return; }
    if apply_sizing(style, property, value) { return; }
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
        "gap" => {
            if let Some(v) = parse_length(value) {
                style.layout.gap = v;
                return true;
            }
            false
        }
        "column-gap" => {
            if let Some(v) = parse_length(value) {
                style.layout.column_gap = Some(v);
                return true;
            }
            false
        }
        "row-gap" => {
            if let Some(v) = parse_length(value) {
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

/// Apply sizing properties (width, height, padding, margin)
fn apply_sizing(style: &mut Style, property: &str, value: &str) -> bool {
    match property {
        "padding" => {
            if let Some(v) = parse_length(value) {
                style.spacing.padding = Spacing::all(v);
                return true;
            }
            false
        }
        "margin" => {
            if let Some(v) = parse_length(value) {
                style.spacing.margin = Spacing::all(v);
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

fn parse_length(value: &str) -> Option<u16> {
    let value = value.trim();
    if let Some(stripped) = value.strip_suffix("px") {
        stripped.trim().parse().ok()
    } else {
        value.parse().ok()
    }
}

fn parse_signed_length(value: &str) -> Option<i16> {
    let value = value.trim();
    if let Some(stripped) = value.strip_suffix("px") {
        stripped.trim().parse().ok()
    } else {
        value.parse().ok()
    }
}

/// Parse a grid template like "1fr 2fr 1fr", "repeat(3, 1fr)", or "minmax(100px, 1fr)"
fn parse_grid_template(value: &str) -> GridTemplate {
    let value = value.trim();
    let mut tracks: Vec<GridTrack> = Vec::new();
    let mut pos = 0;
    let chars: Vec<char> = value.chars().collect();

    while pos < chars.len() {
        // Skip whitespace
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }
        if pos >= chars.len() {
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

        // Parse regular token (no spaces)
        let start = pos;
        while pos < chars.len() && !chars[pos].is_whitespace() {
            pos += 1;
        }

        if pos > start {
            let token: String = chars[start..pos].iter().collect();
            if let Some(track) = parse_grid_track(&token) {
                tracks.push(track);
            }
        }
    }

    GridTemplate::new(tracks)
}

/// Parse repeat(count, track) function
/// Returns (expanded tracks, chars consumed)
fn parse_repeat_function(value: &str) -> Option<(Vec<GridTrack>, usize)> {
    if !value.starts_with("repeat(") {
        return None;
    }

    // Find matching closing paren
    let mut paren_depth = 0;
    let mut end_pos = 0;
    for (i, ch) in value.chars().enumerate() {
        match ch {
            '(' => paren_depth += 1,
            ')' => {
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

    let count: usize = parts[0].trim().parse().ok()?;
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
/// Returns (GridTrack, chars consumed)
fn parse_minmax_function(value: &str) -> Option<(GridTrack, usize)> {
    if !value.starts_with("minmax(") {
        return None;
    }

    // Find matching closing paren
    let mut paren_depth = 0;
    let mut end_pos = 0;
    for (i, ch) in value.chars().enumerate() {
        match ch {
            '(' => paren_depth += 1,
            ')' => {
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
fn parse_grid_placement(value: &str) -> GridPlacement {
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

fn parse_size(value: &str) -> Size {
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

fn parse_color(value: &str) -> Option<Color> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let sheet = parse("").unwrap();
        assert!(sheet.rules.is_empty());
        assert!(sheet.variables.is_empty());
    }

    #[test]
    fn test_parse_simple_rule() {
        let css = ".button { color: red; }";
        let sheet = parse(css).unwrap();

        assert_eq!(sheet.rules.len(), 1);
        assert_eq!(sheet.rules[0].selector, ".button");
        assert_eq!(sheet.rules[0].declarations.len(), 1);
        assert_eq!(sheet.rules[0].declarations[0].property, "color");
        assert_eq!(sheet.rules[0].declarations[0].value, "red");
    }

    #[test]
    fn test_parse_multiple_declarations() {
        let css = ".box { width: 100; height: 50; padding: 4; }";
        let sheet = parse(css).unwrap();

        assert_eq!(sheet.rules[0].declarations.len(), 3);
    }

    #[test]
    fn test_parse_css_variables() {
        let css = r#"
            :root {
                --primary: #ff0000;
                --spacing: 8;
            }
            .button { color: var(--primary); }
        "#;
        let sheet = parse(css).unwrap();

        assert_eq!(sheet.variables.get("--primary"), Some(&"#ff0000".to_string()));
        assert_eq!(sheet.variables.get("--spacing"), Some(&"8".to_string()));
        assert_eq!(sheet.rules.len(), 1);
    }

    #[test]
    fn test_parse_comments() {
        let css = r#"
            /* This is a comment */
            .box {
                /* Another comment */
                width: 100;
            }
        "#;
        let sheet = parse(css).unwrap();
        assert_eq!(sheet.rules.len(), 1);
        assert_eq!(sheet.rules[0].declarations.len(), 1);
    }

    #[test]
    fn test_apply_stylesheet() {
        let css = r#"
            .container {
                display: flex;
                flex-direction: column;
                width: 200;
                padding: 10;
            }
        "#;
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".container", &Style::default());

        assert_eq!(style.layout.display, Display::Flex);
        assert_eq!(style.layout.flex_direction, FlexDirection::Column);
        assert_eq!(style.sizing.width, Size::Fixed(200));
        assert_eq!(style.spacing.padding, Spacing::all(10));
    }

    #[test]
    fn test_parse_color_hex() {
        assert_eq!(parse_color("#ff0000"), Some(Color::RED));
        assert_eq!(parse_color("#00ff00"), Some(Color::GREEN));
        assert_eq!(parse_color("#f00"), Some(Color::RED));
    }

    #[test]
    fn test_parse_color_rgb() {
        assert_eq!(parse_color("rgb(255, 0, 0)"), Some(Color::RED));
        assert_eq!(parse_color("rgb(0, 255, 0)"), Some(Color::GREEN));
    }

    #[test]
    fn test_parse_color_named() {
        assert_eq!(parse_color("red"), Some(Color::RED));
        assert_eq!(parse_color("WHITE"), Some(Color::WHITE));
    }

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("auto"), Size::Auto);
        assert_eq!(parse_size("100"), Size::Fixed(100));
        assert_eq!(parse_size("100px"), Size::Fixed(100));
        assert_eq!(parse_size("50%"), Size::Percent(50.0));
    }

    #[test]
    fn test_apply_with_variables() {
        let css = r#"
            :root {
                --primary: #ff0000;
            }
            .text { color: var(--primary); }
        "#;
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".text", &Style::default());

        assert_eq!(style.visual.color, Color::RED);
    }

    #[test]
    fn test_parse_grid_template() {
        let template = parse_grid_template("1fr 2fr 1fr");
        assert_eq!(template.tracks.len(), 3);
        assert!(matches!(template.tracks[0], GridTrack::Fr(v) if (v - 1.0).abs() < 0.01));
        assert!(matches!(template.tracks[1], GridTrack::Fr(v) if (v - 2.0).abs() < 0.01));
    }

    #[test]
    fn test_parse_grid_template_mixed() {
        let template = parse_grid_template("100px auto 1fr");
        assert_eq!(template.tracks.len(), 3);
        assert!(matches!(template.tracks[0], GridTrack::Fixed(100)));
        assert!(matches!(template.tracks[1], GridTrack::Auto));
        assert!(matches!(template.tracks[2], GridTrack::Fr(_)));
    }

    #[test]
    fn test_parse_grid_placement_line() {
        let placement = parse_grid_placement("2");
        assert_eq!(placement.start, 2);
        assert_eq!(placement.end, 0);
    }

    #[test]
    fn test_parse_grid_placement_span() {
        let placement = parse_grid_placement("span 3");
        assert_eq!(placement.start, 0);
        assert_eq!(placement.end, -3);
    }

    #[test]
    fn test_parse_grid_placement_range() {
        let placement = parse_grid_placement("1 / 4");
        assert_eq!(placement.start, 1);
        assert_eq!(placement.end, 4);
    }

    #[test]
    fn test_apply_grid_properties() {
        let css = r#"
            .grid {
                display: grid;
                grid-template-columns: 1fr 2fr;
                grid-template-rows: auto 100px;
            }
        "#;
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".grid", &Style::default());

        assert_eq!(style.layout.display, Display::Grid);
        assert_eq!(style.layout.grid_template_columns.tracks.len(), 2);
        assert_eq!(style.layout.grid_template_rows.tracks.len(), 2);
    }

    #[test]
    fn test_apply_position_properties() {
        let css = r#"
            .modal {
                position: absolute;
                top: 10;
                left: 20;
                z-index: 100;
            }
        "#;
        let sheet = parse(css).unwrap();
        let style = sheet.apply(".modal", &Style::default());

        assert_eq!(style.layout.position, Position::Absolute);
        assert_eq!(style.spacing.top, Some(10));
        assert_eq!(style.spacing.left, Some(20));
        assert_eq!(style.visual.z_index, 100);
    }
}
