//! JSON parsing for JsonViewer

use super::types::{JsonNode, JsonType};

/// Maximum JSON nesting depth to prevent stack overflow
const MAX_JSON_DEPTH: usize = 256;

/// Simple JSON parser (handles basic JSON)
pub fn parse_json(json: &str) -> Option<JsonNode> {
    let json = json.trim();
    if json.is_empty() {
        return None;
    }

    parse_value(json, "", 0).map(|(node, _)| node)
}

fn parse_value(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    // Prevent stack overflow from deeply nested JSON
    if depth > MAX_JSON_DEPTH {
        return None;
    }

    let json = json.trim_start();
    if json.is_empty() {
        return None;
    }

    let first = json.chars().next()?;

    match first {
        '{' => parse_object(json, path, depth),
        '[' => parse_array(json, path, depth),
        '"' => parse_string(json, path, depth),
        't' | 'f' => parse_bool(json, path, depth),
        'n' => parse_null(json, path, depth),
        c if c.is_ascii_digit() || c == '-' => parse_number(json, path, depth),
        _ => None,
    }
}

fn parse_object(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if !json.starts_with('{') {
        return None;
    }

    let node_path = if path.is_empty() {
        "$".to_string()
    } else {
        path.to_string()
    };

    let mut node = JsonNode::new("", &node_path, JsonType::Object, depth);
    let mut children = Vec::new();
    let mut idx = 1; // Skip '{'
    let chars: Vec<char> = json.chars().collect();

    loop {
        // Skip whitespace
        while idx < chars.len() && chars[idx].is_whitespace() {
            idx += 1;
        }

        if idx >= chars.len() {
            return None;
        }

        if chars[idx] == '}' {
            idx += 1;
            break;
        }

        // Skip comma
        if chars[idx] == ',' {
            idx += 1;
            continue;
        }

        // Parse key
        if chars[idx] != '"' {
            return None;
        }
        let key_start = idx + 1;
        idx += 1;
        while idx < chars.len() && chars[idx] != '"' {
            if chars[idx] == '\\' {
                idx += 1;
            }
            idx += 1;
        }
        if idx >= chars.len() {
            return None;
        }
        let key: String = chars[key_start..idx].iter().collect();
        idx += 1; // Skip closing quote

        // Skip whitespace and colon
        while idx < chars.len() && (chars[idx].is_whitespace() || chars[idx] == ':') {
            idx += 1;
        }

        // Parse value
        let child_path = if node_path == "$" {
            format!("$.{}", key)
        } else {
            format!("{}.{}", node_path, key)
        };

        let remaining: String = chars[idx..].iter().collect();
        if let Some((mut child, consumed)) = parse_value(&remaining, &child_path, depth + 1) {
            child.key = key;
            children.push(child);
            idx += consumed;
        } else {
            return None;
        }
    }

    node.children = children;
    Some((node, idx))
}

fn parse_array(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if !json.starts_with('[') {
        return None;
    }

    let node_path = if path.is_empty() {
        "$".to_string()
    } else {
        path.to_string()
    };

    let mut node = JsonNode::new("", &node_path, JsonType::Array, depth);
    let mut children = Vec::new();
    let mut idx = 1; // Skip '['
    let mut array_idx = 0;
    let chars: Vec<char> = json.chars().collect();

    loop {
        // Skip whitespace
        while idx < chars.len() && chars[idx].is_whitespace() {
            idx += 1;
        }

        if idx >= chars.len() {
            return None;
        }

        if chars[idx] == ']' {
            idx += 1;
            break;
        }

        // Skip comma
        if chars[idx] == ',' {
            idx += 1;
            continue;
        }

        // Parse value
        let child_path = format!("{}[{}]", node_path, array_idx);
        let remaining: String = chars[idx..].iter().collect();
        if let Some((mut child, consumed)) = parse_value(&remaining, &child_path, depth + 1) {
            child.key = format!("[{}]", array_idx);
            children.push(child);
            idx += consumed;
            array_idx += 1;
        } else {
            return None;
        }
    }

    node.children = children;
    Some((node, idx))
}

fn parse_string(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if !json.starts_with('"') {
        return None;
    }

    let chars: Vec<char> = json.chars().collect();
    let mut idx = 1;
    let mut value = String::new();

    while idx < chars.len() {
        let c = chars[idx];
        if c == '"' {
            idx += 1;
            break;
        }
        if c == '\\' && idx + 1 < chars.len() {
            idx += 1;
            match chars[idx] {
                'n' => value.push('\n'),
                'r' => value.push('\r'),
                't' => value.push('\t'),
                '"' => value.push('"'),
                '\\' => value.push('\\'),
                _ => value.push(chars[idx]),
            }
        } else {
            value.push(c);
        }
        idx += 1;
    }

    let node = JsonNode::new("", path, JsonType::String, depth).with_value(value);
    Some((node, idx))
}

fn parse_number(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    let chars: Vec<char> = json.chars().collect();
    let mut idx = 0;

    // Optional minus
    if idx < chars.len() && chars[idx] == '-' {
        idx += 1;
    }

    // Digits
    while idx < chars.len()
        && (chars[idx].is_ascii_digit()
            || chars[idx] == '.'
            || chars[idx] == 'e'
            || chars[idx] == 'E'
            || chars[idx] == '+'
            || chars[idx] == '-')
    {
        idx += 1;
    }

    let value: String = chars[..idx].iter().collect();
    let node = JsonNode::new("", path, JsonType::Number, depth).with_value(value);
    Some((node, idx))
}

fn parse_bool(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if json.starts_with("true") {
        let node = JsonNode::new("", path, JsonType::Boolean, depth).with_value("true");
        Some((node, 4))
    } else if json.starts_with("false") {
        let node = JsonNode::new("", path, JsonType::Boolean, depth).with_value("false");
        Some((node, 5))
    } else {
        None
    }
}

fn parse_null(json: &str, path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if json.starts_with("null") {
        let node = JsonNode::new("", path, JsonType::Null, depth);
        Some((node, 4))
    } else {
        None
    }
}
