//! CSS custom property (`var()`) substitution

use std::borrow::Cow;
use std::collections::HashMap;

/// How deep a variable may refer to other variables before it is treated as
/// a cycle.
const MAX_VAR_DEPTH: usize = 16;

/// Replace every `var(--name)` / `var(--name, fallback)` in `value`.
///
/// Wherever it appears, not only as the whole value, so `border: rounded
/// var(--accent)` and `margin: var(--y) var(--x)` reach their shorthand
/// parsers as plain tokens. A variable's own value and a fallback may use
/// `var()` in turn. A reference that resolves to nothing - undefined with no
/// fallback, or part of a cycle - is left as written, so the value fails to
/// parse and the declaration does nothing, as it always has.
pub(super) fn substitute_vars<'v>(value: &'v str, vars: &HashMap<String, String>) -> Cow<'v, str> {
    if value.contains("var(") {
        Cow::Owned(substitute(value, vars, 0))
    } else {
        Cow::Borrowed(value)
    }
}

fn substitute(value: &str, vars: &HashMap<String, String>, depth: usize) -> String {
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(start) = rest.find("var(") {
        out.push_str(&rest[..start]);
        let args = &rest[start + "var(".len()..];
        let Some(close) = top_level(args, ')') else {
            // Unbalanced: nothing to substitute.
            out.push_str(&rest[start..]);
            return out;
        };
        let reference = &rest[start..start + "var(".len() + close + 1];
        match resolve_var(&args[..close], vars, depth) {
            Some(resolved) => out.push_str(&resolved),
            None => out.push_str(reference),
        }
        rest = &args[close + 1..];
    }
    out.push_str(rest);
    out
}

/// The value of one `var()` reference, given what is inside its parentheses.
fn resolve_var(args: &str, vars: &HashMap<String, String>, depth: usize) -> Option<String> {
    if depth >= MAX_VAR_DEPTH {
        return None;
    }
    let (name, fallback) = match top_level(args, ',') {
        Some(comma) => (&args[..comma], Some(args[comma + 1..].trim())),
        None => (args, None),
    };
    let raw = vars.get(name.trim()).map(String::as_str).or(fallback)?;
    Some(substitute(raw, vars, depth + 1))
}

/// Byte index of the first `target` in `s` outside any parentheses `s`
/// opens itself.
fn top_level(s: &str, target: char) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            _ if c == target && depth == 0 => return Some(i),
            '(' => depth += 1,
            ')' => depth = depth.checked_sub(1)?,
            _ => {}
        }
    }
    None
}
