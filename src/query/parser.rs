//! Query string parser
//!
//! Parses query strings like:
//! - `hello world` - free text search
//! - `author:john` - field equals
//! - `title~rust` - field contains
//! - `age:>18` - greater than
//! - `status:!draft` - not equals
//! - `sort:name:desc` - sorting
//! - `limit:10` - pagination

use super::{Filter, Operator, Query, SortBy, SortDirection};

/// Query parsing error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Error message
    pub message: String,
    /// Position in input where error occurred
    pub position: usize,
}

impl ParseError {
    /// Create a new parse error
    pub fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at {}: {}", self.position, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Parse a query string into a Query
pub fn parse(input: &str) -> Result<Query, ParseError> {
    let mut query = Query::new();
    let input = input.trim();

    if input.is_empty() {
        return Ok(query);
    }

    let tokens = tokenize(input)?;

    for token in tokens {
        match token {
            Token::Text(text) => {
                query.filters.push(Filter::Text(text));
            }
            Token::Field { name, op, value } => {
                // Handle special fields
                match name.to_lowercase().as_str() {
                    "sort" => {
                        // sort:field or sort:field:desc
                        let parts: Vec<&str> = value.split(':').collect();
                        let field = parts[0].to_string();
                        let direction = if parts.len() > 1 && parts[1].to_lowercase() == "desc" {
                            SortDirection::Descending
                        } else {
                            SortDirection::Ascending
                        };
                        query.sort = Some(SortBy { field, direction });
                    }
                    "limit" => {
                        if let Ok(limit) = value.parse() {
                            query.limit = Some(limit);
                        }
                    }
                    "offset" | "skip" => {
                        if let Ok(offset) = value.parse() {
                            query.offset = Some(offset);
                        }
                    }
                    _ => {
                        query.filters.push(Filter::Field {
                            name: name.to_string(),
                            op,
                            value: value.to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok(query)
}

#[derive(Debug)]
enum Token {
    Text(String),
    Field {
        name: String,
        op: Operator,
        value: String,
    },
}

fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        // Skip whitespace
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        // Parse quoted string
        if ch == '"' || ch == '\'' {
            let quote = ch;
            chars.next();

            let mut text = String::new();
            while let Some(&c) = chars.peek() {
                if c == quote {
                    chars.next();
                    break;
                }
                text.push(c);
                chars.next();
            }
            tokens.push(Token::Text(text));
            continue;
        }

        // Parse word or field:value
        let mut word = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                break;
            }
            word.push(c);
            chars.next();
        }

        if word.is_empty() {
            continue;
        }

        // Check for field:value pattern
        if let Some(token) = parse_field_token(&word) {
            tokens.push(token);
        } else {
            tokens.push(Token::Text(word));
        }
    }

    Ok(tokens)
}

fn parse_field_token(word: &str) -> Option<Token> {
    // Try to find operator patterns

    // field~value (contains)
    if let Some(idx) = word.find('~') {
        let name = word[..idx].to_string();
        let value = word[idx + 1..].to_string();
        if !name.is_empty() && !value.is_empty() {
            return Some(Token::Field {
                name,
                op: Operator::Contains,
                value,
            });
        }
    }

    // field:operator value patterns
    if let Some(idx) = word.find(':') {
        let name = word[..idx].to_string();
        let rest = &word[idx + 1..];

        if name.is_empty() || rest.is_empty() {
            return None;
        }

        // Check for operators
        let (op, value) = if let Some(v) = rest.strip_prefix(">=") {
            (Operator::Ge, v)
        } else if let Some(v) = rest.strip_prefix("<=") {
            (Operator::Le, v)
        } else if let Some(v) = rest.strip_prefix('>') {
            (Operator::Gt, v)
        } else if let Some(v) = rest.strip_prefix('<') {
            (Operator::Lt, v)
        } else if let Some(v) = rest.strip_prefix('!') {
            (Operator::Ne, v)
        } else {
            (Operator::Eq, rest)
        };

        if !value.is_empty() {
            return Some(Token::Field {
                name,
                op,
                value: value.to_string(),
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let query = parse("").unwrap();
        assert!(query.is_empty());
    }

    #[test]
    fn test_parse_text() {
        let query = parse("hello world").unwrap();
        assert_eq!(query.filters.len(), 2);
    }

    #[test]
    fn test_parse_quoted() {
        let query = parse("\"hello world\"").unwrap();
        assert_eq!(query.filters.len(), 1);
        match &query.filters[0] {
            Filter::Text(t) => assert_eq!(t, "hello world"),
            _ => panic!("Expected text filter"),
        }
    }

    #[test]
    fn test_parse_field_eq() {
        let query = parse("author:john").unwrap();
        assert_eq!(query.filters.len(), 1);
        match &query.filters[0] {
            Filter::Field { name, op, value } => {
                assert_eq!(name, "author");
                assert_eq!(*op, Operator::Eq);
                assert_eq!(value, "john");
            }
            _ => panic!("Expected field filter"),
        }
    }

    #[test]
    fn test_parse_field_ne() {
        let query = parse("status:!draft").unwrap();
        assert_eq!(query.filters.len(), 1);
        match &query.filters[0] {
            Filter::Field { name, op, value } => {
                assert_eq!(name, "status");
                assert_eq!(*op, Operator::Ne);
                assert_eq!(value, "draft");
            }
            _ => panic!("Expected field filter"),
        }
    }

    #[test]
    fn test_parse_field_gt() {
        let query = parse("age:>18").unwrap();
        match &query.filters[0] {
            Filter::Field { name, op, value } => {
                assert_eq!(name, "age");
                assert_eq!(*op, Operator::Gt);
                assert_eq!(value, "18");
            }
            _ => panic!("Expected field filter"),
        }
    }

    #[test]
    fn test_parse_field_contains() {
        let query = parse("title~rust").unwrap();
        match &query.filters[0] {
            Filter::Field { name, op, value } => {
                assert_eq!(name, "title");
                assert_eq!(*op, Operator::Contains);
                assert_eq!(value, "rust");
            }
            _ => panic!("Expected field filter"),
        }
    }

    #[test]
    fn test_parse_sort() {
        let query = parse("sort:name").unwrap();
        assert!(query.sort.is_some());
        let sort = query.sort.unwrap();
        assert_eq!(sort.field, "name");
        assert_eq!(sort.direction, SortDirection::Ascending);
    }

    #[test]
    fn test_parse_sort_desc() {
        let query = parse("sort:name:desc").unwrap();
        let sort = query.sort.unwrap();
        assert_eq!(sort.direction, SortDirection::Descending);
    }

    #[test]
    fn test_parse_limit() {
        let query = parse("limit:10").unwrap();
        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn test_parse_complex() {
        let query = parse("author:john active:true sort:date:desc limit:20").unwrap();
        assert_eq!(query.filters.len(), 2);
        assert!(query.sort.is_some());
        assert_eq!(query.limit, Some(20));
    }
}
