//! JSON for time-travel sessions: writing a [`SnapshotValue`] and reading one
//! back.
//!
//! A small strict parser rather than the `JsonViewer` one, which is lenient
//! by design (it shows whatever it can of a broken document). An import must
//! either reproduce the export exactly or fail: `\u` escapes (with surrogate
//! pairs) are decoded, integers stay [`SnapshotValue::Int`], numbers with a
//! fraction or exponent are [`SnapshotValue::Float`], and trailing garbage,
//! unterminated strings or nesting deeper than [`MAX_DEPTH`] are errors. It
//! runs in linear time.

use std::collections::HashMap;
use std::fmt::Write;

use super::SnapshotValue;

/// Deepest nesting an import accepts, so a hostile file cannot overflow the
/// stack.
pub(super) const MAX_DEPTH: usize = 128;

/// `s` as a quoted JSON string: quotes, backslashes and control characters
/// are escaped.
pub(super) fn string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// `value` as JSON. A float that is NaN or infinite has no JSON form and is
/// written as `null`; every other float keeps a `.` or exponent, so it reads
/// back as a float.
pub(super) fn value(value: &SnapshotValue) -> String {
    let mut out = String::new();
    write_value(&mut out, value);
    out
}

fn write_value(out: &mut String, value: &SnapshotValue) {
    match value {
        SnapshotValue::Null => out.push_str("null"),
        SnapshotValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        SnapshotValue::Int(n) => {
            let _ = write!(out, "{n}");
        }
        SnapshotValue::Float(f) if f.is_finite() => {
            // `{:?}` keeps the `.0` of a whole number: `1.0`, not `1`.
            let _ = write!(out, "{f:?}");
        }
        SnapshotValue::Float(_) => out.push_str("null"),
        SnapshotValue::String(s) => out.push_str(&string(s)),
        SnapshotValue::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(out, item);
            }
            out.push(']');
        }
        SnapshotValue::Object(map) => {
            // Sorted, so the same state always exports the same text.
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (i, key) in keys.into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str(&string(key));
                out.push(':');
                write_value(out, &map[key]);
            }
            out.push('}');
        }
    }
}

/// Parse `text` as exactly one JSON value.
///
/// The error says what was wrong and at which byte.
pub(super) fn parse(text: &str) -> Result<SnapshotValue, String> {
    let mut p = Parser {
        bytes: text.as_bytes(),
        text,
        pos: 0,
    };
    p.skip_ws();
    let value = p.value(0)?;
    p.skip_ws();
    if p.pos != p.bytes.len() {
        return Err(p.error("unexpected text after the JSON value"));
    }
    Ok(value)
}

struct Parser<'a> {
    bytes: &'a [u8],
    text: &'a str,
    pos: usize,
}

impl Parser<'_> {
    fn error(&self, what: &str) -> String {
        format!("{what} at byte {}", self.pos)
    }

    fn skip_ws(&mut self) {
        while let Some(b' ' | b'\t' | b'\n' | b'\r') = self.bytes.get(self.pos) {
            self.pos += 1;
        }
    }

    fn expect(&mut self, byte: u8) -> Result<(), String> {
        if self.bytes.get(self.pos) == Some(&byte) {
            self.pos += 1;
            Ok(())
        } else {
            Err(self.error(&format!("expected `{}`", byte as char)))
        }
    }

    fn literal(&mut self, word: &str, value: SnapshotValue) -> Result<SnapshotValue, String> {
        if self.bytes[self.pos..].starts_with(word.as_bytes()) {
            self.pos += word.len();
            Ok(value)
        } else {
            Err(self.error("invalid literal"))
        }
    }

    fn value(&mut self, depth: usize) -> Result<SnapshotValue, String> {
        if depth > MAX_DEPTH {
            return Err(self.error("nested too deeply"));
        }
        match self.bytes.get(self.pos) {
            None => Err(self.error("unexpected end of input")),
            Some(b'n') => self.literal("null", SnapshotValue::Null),
            Some(b't') => self.literal("true", SnapshotValue::Bool(true)),
            Some(b'f') => self.literal("false", SnapshotValue::Bool(false)),
            Some(b'"') => self.string().map(SnapshotValue::String),
            Some(b'[') => self.array(depth),
            Some(b'{') => self.object(depth),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(_) => Err(self.error("unexpected character")),
        }
    }

    fn array(&mut self, depth: usize) -> Result<SnapshotValue, String> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if self.bytes.get(self.pos) == Some(&b']') {
            self.pos += 1;
            return Ok(SnapshotValue::Array(items));
        }
        loop {
            self.skip_ws();
            items.push(self.value(depth + 1)?);
            self.skip_ws();
            match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(SnapshotValue::Array(items));
                }
                _ => return Err(self.error("expected `,` or `]`")),
            }
        }
    }

    fn object(&mut self, depth: usize) -> Result<SnapshotValue, String> {
        self.expect(b'{')?;
        let mut map = HashMap::new();
        self.skip_ws();
        if self.bytes.get(self.pos) == Some(&b'}') {
            self.pos += 1;
            return Ok(SnapshotValue::Object(map));
        }
        loop {
            self.skip_ws();
            if self.bytes.get(self.pos) != Some(&b'"') {
                return Err(self.error("expected a string key"));
            }
            let key = self.string()?;
            self.skip_ws();
            self.expect(b':')?;
            self.skip_ws();
            let value = self.value(depth + 1)?;
            map.insert(key, value);
            self.skip_ws();
            match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(SnapshotValue::Object(map));
                }
                _ => return Err(self.error("expected `,` or `}`")),
            }
        }
    }

    fn hex4(&mut self) -> Result<u32, String> {
        let digits = self
            .text
            .get(self.pos..self.pos + 4)
            .filter(|d| d.bytes().all(|b| b.is_ascii_hexdigit()))
            .ok_or_else(|| self.error("expected four hex digits after `\\u`"))?;
        self.pos += 4;
        Ok(u32::from_str_radix(digits, 16).expect("checked hex digits"))
    }

    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            // Copy the run up to the next quote or backslash in one go.
            let start = self.pos;
            while let Some(&b) = self.bytes.get(self.pos) {
                if b == b'"' || b == b'\\' {
                    break;
                }
                if b < 0x20 {
                    return Err(self.error("unescaped control character in a string"));
                }
                self.pos += 1;
            }
            out.push_str(&self.text[start..self.pos]);
            match self.bytes.get(self.pos) {
                None => return Err(self.error("unterminated string")),
                Some(b'"') => {
                    self.pos += 1;
                    return Ok(out);
                }
                Some(_) => {
                    self.pos += 1; // the backslash
                    let escape = *self
                        .bytes
                        .get(self.pos)
                        .ok_or_else(|| self.error("unterminated string"))?;
                    self.pos += 1;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let first = self.hex4()?;
                            let code = if (0xD800..0xDC00).contains(&first) {
                                // A high surrogate must be followed by `\u` + a low one.
                                if !self.bytes[self.pos..].starts_with(b"\\u") {
                                    return Err(self.error("unpaired surrogate"));
                                }
                                self.pos += 2;
                                let second = self.hex4()?;
                                if !(0xDC00..0xE000).contains(&second) {
                                    return Err(self.error("unpaired surrogate"));
                                }
                                0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00)
                            } else {
                                first
                            };
                            out.push(
                                char::from_u32(code)
                                    .ok_or_else(|| self.error("unpaired surrogate"))?,
                            );
                        }
                        _ => return Err(self.error("invalid escape")),
                    }
                }
            }
        }
    }

    fn number(&mut self) -> Result<SnapshotValue, String> {
        let start = self.pos;
        let mut float = false;
        if self.bytes.get(self.pos) == Some(&b'-') {
            self.pos += 1;
        }
        let digits = |p: &mut Self| {
            let s = p.pos;
            while let Some(b'0'..=b'9') = p.bytes.get(p.pos) {
                p.pos += 1;
            }
            p.pos > s
        };
        if !digits(self) {
            return Err(self.error("expected a digit"));
        }
        if self.bytes.get(self.pos) == Some(&b'.') {
            float = true;
            self.pos += 1;
            if !digits(self) {
                return Err(self.error("expected a digit after `.`"));
            }
        }
        if let Some(b'e' | b'E') = self.bytes.get(self.pos) {
            float = true;
            self.pos += 1;
            if let Some(b'+' | b'-') = self.bytes.get(self.pos) {
                self.pos += 1;
            }
            if !digits(self) {
                return Err(self.error("expected a digit in the exponent"));
            }
        }
        let text = &self.text[start..self.pos];
        if !float {
            if let Ok(n) = text.parse::<i64>() {
                return Ok(SnapshotValue::Int(n));
            }
        }
        // Too large for an i64, or written as a float.
        text.parse::<f64>()
            .map(SnapshotValue::Float)
            .map_err(|_| self.error("invalid number"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(pairs: &[(&str, SnapshotValue)]) -> SnapshotValue {
        SnapshotValue::Object(
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect(),
        )
    }

    #[test]
    fn every_value_round_trips() {
        let values = [
            SnapshotValue::Null,
            SnapshotValue::Bool(true),
            SnapshotValue::Bool(false),
            SnapshotValue::Int(0),
            SnapshotValue::Int(-42),
            SnapshotValue::Int(i64::MAX),
            SnapshotValue::Int(i64::MIN),
            SnapshotValue::Float(1.0),
            SnapshotValue::Float(-0.5),
            SnapshotValue::Float(1e300),
            SnapshotValue::Float(f64::MIN_POSITIVE),
            SnapshotValue::String(String::new()),
            SnapshotValue::String("quote \" back \\ nl \n tab \t bell \u{7} 한글 🦀".into()),
            SnapshotValue::Array(vec![]),
            SnapshotValue::Array(vec![SnapshotValue::Int(1), SnapshotValue::Null]),
            obj(&[]),
            obj(&[
                ("a", SnapshotValue::Int(1)),
                ("nested", obj(&[("x", SnapshotValue::Array(vec![]))])),
                ("", SnapshotValue::String("empty key".into())),
            ]),
        ];
        for v in values {
            let text = value(&v);
            assert_eq!(parse(&text), Ok(v.clone()), "{v:?} -> {text}");
        }
    }

    #[test]
    fn a_whole_float_stays_a_float_and_nan_becomes_null() {
        assert_eq!(value(&SnapshotValue::Float(3.0)), "3.0");
        assert_eq!(value(&SnapshotValue::Float(f64::NAN)), "null");
        assert_eq!(value(&SnapshotValue::Float(f64::INFINITY)), "null");
        assert_eq!(parse("3"), Ok(SnapshotValue::Int(3)));
        assert_eq!(parse("3e2"), Ok(SnapshotValue::Float(300.0)));
        // Past i64: kept as a float rather than refused.
        assert_eq!(
            parse("99999999999999999999"),
            Ok(SnapshotValue::Float(1e20))
        );
    }

    #[test]
    fn escapes_are_decoded() {
        assert_eq!(
            parse(r#""\u0041\u00e9\ud83e\udd80\/\b\f""#),
            Ok(SnapshotValue::String("Aé🦀/\u{8}\u{c}".into()))
        );
    }

    #[test]
    fn malformed_input_is_an_error_not_a_guess() {
        for bad in [
            "",
            "   ",
            "nul",
            "t",
            "[1,]",
            "[1 2]",
            "{\"a\" 1}",
            "{a:1}",
            "{\"a\":1,}",
            "\"unterminated",
            "\"bad \\x escape\"",
            "\"\\u12\"",
            "\"\\ud83e\"",
            "\"\\ud83e\\u0041\"",
            "\"\\udd80\"",
            "\"raw\ncontrol\"",
            "01x",
            "-",
            "1.",
            "1e",
            "1 2",
            "{} x",
            "@",
        ] {
            assert!(parse(bad).is_err(), "{bad:?} parsed");
        }
    }

    #[test]
    fn deep_nesting_is_refused_without_overflowing() {
        let ok = "[".repeat(MAX_DEPTH) + &"]".repeat(MAX_DEPTH);
        assert!(parse(&ok).is_ok());
        let deep = "[".repeat(100_000) + &"]".repeat(100_000);
        assert!(parse(&deep).is_err());
    }
}
