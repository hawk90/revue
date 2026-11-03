//! JSON parsing for JsonViewer: the grammar of RFC 8259, strictly

use super::types::{JsonNode, JsonType};

/// Maximum JSON nesting depth to prevent stack overflow
const MAX_JSON_DEPTH: usize = 256;

/// JSON whitespace: space, tab, line feed and carriage return (RFC 8259 §2)
fn is_ws(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

/// The tree of `json`, one complete JSON value with only whitespace around
/// it, or `None` when it is not valid JSON
pub fn parse_json(json: &str) -> Option<JsonNode> {
    let chars: Vec<char> = json.chars().collect();
    let (node, taken) = parse_value(&chars, "", 0)?;
    chars[taken..].iter().all(|&c| is_ws(c)).then_some(node)
}

/// Whether `json` is one complete JSON value (surrounding whitespace allowed)
pub(crate) fn is_json(json: &str) -> bool {
    parse_json(json).is_some()
}

// Each parser takes the chars from where its value starts to the end of the
// input (a slice of one shared buffer, so nothing is copied per value) and
// returns the node and the chars it took.

/// The chars of whitespace at the start of `json`
fn skip_ws(json: &[char]) -> usize {
    json.iter().take_while(|&&c| is_ws(c)).count()
}

fn parse_value(json: &[char], path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    // Prevent stack overflow from deeply nested JSON
    if depth > MAX_JSON_DEPTH {
        return None;
    }

    let start = skip_ws(json);
    let json = &json[start..];
    let first = *json.first()?;
    // The root is `$`, whatever its type
    let path = &node_path(path);

    let parsed = match first {
        '{' => parse_object(json, path, depth),
        '[' => parse_array(json, path, depth),
        '"' => {
            let (value, taken) = parse_string(json)?;
            let node = JsonNode::new("", path, JsonType::String, depth).with_value(value);
            Some((node, taken))
        }
        't' | 'f' => parse_bool(json, path, depth),
        'n' => parse_null(json, path, depth),
        c if c.is_ascii_digit() || c == '-' => parse_number(json, path, depth),
        _ => None,
    };
    // The whitespace before the value counts as taken too
    parsed.map(|(node, taken)| (node, start + taken))
}

/// The path of member `key` of the node at `parent`, as JSONPath writes it
/// (RFC 9535): `.key` when the key is a member-name shorthand (a letter,
/// `_` or a non-ASCII char, then those or digits), else `['key']` with the
/// escapes of a normalized path. Every member gets a path of its own, so
/// `{"a.b": ..}` (`$['a.b']`) and `{"a": {"b": ..}}` (`$.a.b`) differ (#983).
fn member_path(parent: &str, key: &str) -> String {
    let mut chars = key.chars();
    let first_ok = |c: char| c.is_ascii_alphabetic() || c == '_' || !c.is_ascii();
    let shorthand =
        chars.next().is_some_and(first_ok) && chars.all(|c| first_ok(c) || c.is_ascii_digit());
    if shorthand {
        return format!("{parent}.{key}");
    }
    let mut path = format!("{parent}['");
    for c in key.chars() {
        match c {
            '\'' => path.push_str("\\'"),
            '\\' => path.push_str("\\\\"),
            '\u{8}' => path.push_str("\\b"),
            '\u{c}' => path.push_str("\\f"),
            '\n' => path.push_str("\\n"),
            '\r' => path.push_str("\\r"),
            '\t' => path.push_str("\\t"),
            c if (c as u32) < 0x20 => path.push_str(&format!("\\u{:04x}", c as u32)),
            c => path.push(c),
        }
    }
    path.push_str("']");
    path
}

/// `$` for the root, else the path as given
fn node_path(path: &str) -> String {
    if path.is_empty() {
        "$".to_string()
    } else {
        path.to_string()
    }
}

fn parse_object(json: &[char], path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if json.first() != Some(&'{') {
        return None;
    }
    let node_path = node_path(path);
    let mut node = JsonNode::new("", &node_path, JsonType::Object, depth);
    let mut idx = 1 + skip_ws(&json[1..]);

    if json.get(idx) == Some(&'}') {
        return Some((node, idx + 1));
    }

    loop {
        // A member: a key, a colon and a value
        let (key, taken) = parse_string(&json[idx..])?;
        idx += taken;
        idx += skip_ws(&json[idx..]);
        if json.get(idx) != Some(&':') {
            return None;
        }
        idx += 1;

        let child_path = member_path(&node_path, &key);
        let (mut child, consumed) = parse_value(&json[idx..], &child_path, depth + 1)?;
        child.key = key;
        node.children.push(child);
        idx += consumed;

        // Then a comma and another member, or the end
        idx += skip_ws(&json[idx..]);
        match json.get(idx) {
            Some(',') => {
                idx += 1;
                idx += skip_ws(&json[idx..]);
            }
            Some('}') => return Some((node, idx + 1)),
            _ => return None,
        }
    }
}

fn parse_array(json: &[char], path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if json.first() != Some(&'[') {
        return None;
    }
    let node_path = node_path(path);
    let mut node = JsonNode::new("", &node_path, JsonType::Array, depth);
    let mut idx = 1 + skip_ws(&json[1..]);

    if json.get(idx) == Some(&']') {
        return Some((node, idx + 1));
    }

    loop {
        let array_idx = node.children.len();
        let child_path = format!("{}[{}]", node_path, array_idx);
        let (mut child, consumed) = parse_value(&json[idx..], &child_path, depth + 1)?;
        child.key = format!("[{}]", array_idx);
        node.children.push(child);
        idx += consumed;

        // Then a comma and another value, or the end
        idx += skip_ws(&json[idx..]);
        match json.get(idx) {
            Some(',') => idx += 1,
            Some(']') => return Some((node, idx + 1)),
            _ => return None,
        }
    }
}

/// A string from its opening quote: the decoded text and the chars taken,
/// or `None` when it is not closed, holds a control character or a bad
/// escape
fn parse_string(json: &[char]) -> Option<(String, usize)> {
    if json.first() != Some(&'"') {
        return None;
    }

    let mut idx = 1;
    let mut value = String::new();

    loop {
        let c = *json.get(idx)?;
        idx += 1;
        match c {
            '"' => return Some((value, idx)),
            '\\' => {
                let escape = *json.get(idx)?;
                idx += 1;
                match escape {
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    '/' => value.push('/'),
                    'b' => value.push('\u{8}'),
                    'f' => value.push('\u{c}'),
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    'u' => {
                        let (ch, taken) = parse_unicode_escape(&json[idx..])?;
                        value.push(ch);
                        idx += taken;
                    }
                    _ => return None,
                }
            }
            c if (c as u32) < 0x20 => return None,
            c => value.push(c),
        }
    }
}

/// The four hex digits after `\u` as a code unit
fn hex4(json: &[char]) -> Option<u32> {
    let digits = json.get(..4)?;
    digits
        .iter()
        .try_fold(0u32, |n, c| Some(n * 16 + c.to_digit(16)?))
}

/// The char of a `\u` escape from after its `u`, joining a surrogate pair
/// written as two escapes; a lone surrogate, which JSON's grammar allows
/// but no char can hold, becomes U+FFFD. Returns the char and the chars
/// taken.
fn parse_unicode_escape(json: &[char]) -> Option<(char, usize)> {
    let unit = hex4(json)?;
    if (0xD800..0xDC00).contains(&unit) {
        // A high surrogate: a low one may follow as `\uXXXX`
        if json.get(4) == Some(&'\\') && json.get(5) == Some(&'u') {
            if let Some(low) = hex4(&json[6..]) {
                if (0xDC00..0xE000).contains(&low) {
                    let code = 0x10000 + ((unit - 0xD800) << 10) + (low - 0xDC00);
                    return Some((char::from_u32(code)?, 10));
                }
            }
        }
        return Some((char::REPLACEMENT_CHARACTER, 4));
    }
    Some((
        char::from_u32(unit).unwrap_or(char::REPLACEMENT_CHARACTER),
        4,
    ))
}

/// A number: `-`, then `0` or digits not starting with 0, then optionally
/// a fraction and an exponent (RFC 8259 §6)
fn parse_number(json: &[char], path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    let digits_from = |idx: usize| {
        json[idx.min(json.len())..]
            .iter()
            .take_while(|c| c.is_ascii_digit())
            .count()
    };
    let mut idx = 0;

    if json.first() == Some(&'-') {
        idx += 1;
    }

    // The integer part
    match json.get(idx) {
        Some('0') => idx += 1,
        Some(c) if c.is_ascii_digit() => idx += digits_from(idx),
        _ => return None,
    }

    // A fraction: a dot and at least one digit
    if json.get(idx) == Some(&'.') {
        let n = digits_from(idx + 1);
        if n == 0 {
            return None;
        }
        idx += 1 + n;
    }

    // An exponent: e or E, a sign maybe, and at least one digit
    if matches!(json.get(idx), Some('e' | 'E')) {
        idx += 1;
        if matches!(json.get(idx), Some('+' | '-')) {
            idx += 1;
        }
        let n = digits_from(idx);
        if n == 0 {
            return None;
        }
        idx += n;
    }

    let value: String = json[..idx].iter().collect();
    let node = JsonNode::new("", path, JsonType::Number, depth).with_value(value);
    Some((node, idx))
}

fn parse_bool(json: &[char], path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if starts_with(json, "true") {
        let node = JsonNode::new("", path, JsonType::Boolean, depth).with_value("true");
        Some((node, 4))
    } else if starts_with(json, "false") {
        let node = JsonNode::new("", path, JsonType::Boolean, depth).with_value("false");
        Some((node, 5))
    } else {
        None
    }
}

fn parse_null(json: &[char], path: &str, depth: usize) -> Option<(JsonNode, usize)> {
    if starts_with(json, "null") {
        let node = JsonNode::new("", path, JsonType::Null, depth);
        Some((node, 4))
    } else {
        None
    }
}

/// Whether `chars` starts with `word`
fn starts_with(chars: &[char], word: &str) -> bool {
    let mut it = chars.iter();
    word.chars().all(|w| it.next() == Some(&w))
}
