//! CSS custom property (`var()`) substitution

use std::borrow::Cow;
use std::collections::HashMap;

/// How deep a variable may refer to other variables before it is treated as
/// a cycle.
const MAX_VAR_DEPTH: usize = 16;

/// How much work one value's substitution may do: every `var()` resolved costs
/// its raw value's length plus one, and every substituted byte costs one.
///
/// The depth limit alone does not bound it. A variable that refers to another
/// several times - `--a: var(--b) var(--b) var(--b) var(--b)`, sixteen levels
/// down - expands to 4^16 copies, which never finishes. A value that runs out
/// is left as written, like a cycle.
const MAX_VAR_WORK: usize = 1 << 20;

/// The substitution ran out of [`MAX_VAR_WORK`].
struct Exhausted;

/// Replace every `var(--name)` / `var(--name, fallback)` in `value`.
///
/// Wherever it appears, not only as the whole value, so `border: rounded
/// var(--accent)` and `margin: var(--y) var(--x)` reach their shorthand
/// parsers as plain tokens. A variable's own value and a fallback may use
/// `var()` in turn. A reference that resolves to nothing - undefined with no
/// fallback, or part of a cycle - is left as written, so the value fails to
/// parse and the declaration does nothing, as it always has. So is a whole
/// value whose expansion grows past [`MAX_VAR_WORK`].
pub(super) fn substitute_vars<'v>(value: &'v str, vars: &HashMap<String, String>) -> Cow<'v, str> {
    if value.contains("var(") {
        let mut budget = MAX_VAR_WORK;
        match substitute(value, vars, 0, &mut budget) {
            Ok(substituted) => Cow::Owned(substituted),
            Err(Exhausted) => Cow::Borrowed(value),
        }
    } else {
        Cow::Borrowed(value)
    }
}

/// Take `cost` from `budget`, or fail if it is not there.
fn spend(budget: &mut usize, cost: usize) -> Result<(), Exhausted> {
    *budget = budget.checked_sub(cost).ok_or(Exhausted)?;
    Ok(())
}

fn substitute(
    value: &str,
    vars: &HashMap<String, String>,
    depth: usize,
    budget: &mut usize,
) -> Result<String, Exhausted> {
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(start) = rest.find("var(") {
        out.push_str(&rest[..start]);
        let args = &rest[start + "var(".len()..];
        let Some(close) = top_level(args, ')') else {
            // Unbalanced: nothing to substitute.
            out.push_str(&rest[start..]);
            return Ok(out);
        };
        let reference = &rest[start..start + "var(".len() + close + 1];
        match resolve_var(&args[..close], vars, depth, budget)? {
            Some(resolved) => {
                spend(budget, resolved.len())?;
                out.push_str(&resolved);
            }
            None => out.push_str(reference),
        }
        rest = &args[close + 1..];
    }
    out.push_str(rest);
    Ok(out)
}

/// The value of one `var()` reference, given what is inside its parentheses.
fn resolve_var(
    args: &str,
    vars: &HashMap<String, String>,
    depth: usize,
    budget: &mut usize,
) -> Result<Option<String>, Exhausted> {
    if depth >= MAX_VAR_DEPTH {
        return Ok(None);
    }
    let (name, fallback) = match top_level(args, ',') {
        Some(comma) => (&args[..comma], Some(args[comma + 1..].trim())),
        None => (args, None),
    };
    let Some(raw) = vars.get(name.trim()).map(String::as_str).or(fallback) else {
        return Ok(None);
    };
    spend(budget, raw.len() + 1)?;
    substitute(raw, vars, depth + 1, budget).map(Some)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn substitutes_nested_references() {
        let vars = vars(&[("--a", "var(--b) 2"), ("--b", "1")]);
        assert_eq!(substitute_vars("var(--a) 3", &vars), "1 2 3");
    }

    /// Four references per level, sixteen levels deep: 4^16 copies. The
    /// expansion must give up - leaving the value as written - not run forever.
    #[test]
    fn an_exponential_expansion_is_left_as_written() {
        let mut pairs: Vec<(String, String)> = (0..16)
            .map(|i| {
                let next = format!("var(--v{})", i + 1);
                (format!("--v{i}"), [next.as_str(); 4].join(" "))
            })
            .collect();
        pairs.push(("--v16".into(), "x".into()));
        let vars: HashMap<String, String> = pairs.into_iter().collect();

        let start = std::time::Instant::now();
        assert_eq!(substitute_vars("var(--v0)", &vars), "var(--v0)");
        assert!(start.elapsed() < std::time::Duration::from_secs(2));
    }

    /// The same, with every leaf empty: no bytes are produced, so only
    /// counting the references themselves bounds it.
    #[test]
    fn an_exponential_expansion_to_nothing_is_bounded_too() {
        let mut pairs: Vec<(String, String)> = (0..16)
            .map(|i| {
                let next = format!("var(--v{})", i + 1);
                (format!("--v{i}"), [next.as_str(); 4].concat())
            })
            .collect();
        pairs.push(("--v16".into(), String::new()));
        let vars: HashMap<String, String> = pairs.into_iter().collect();

        let start = std::time::Instant::now();
        assert_eq!(substitute_vars("var(--v0)", &vars), "var(--v0)");
        assert!(start.elapsed() < std::time::Duration::from_secs(2));
    }
}
