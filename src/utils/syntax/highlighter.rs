//! Syntax highlighter implementation

use super::language::Language;
use super::theme::SyntaxTheme;
use super::types::{Token, TokenType};

/// Syntax highlighter
pub struct SyntaxHighlighter {
    theme: SyntaxTheme,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default theme
    pub fn new() -> Self {
        Self {
            theme: SyntaxTheme::default(),
        }
    }

    /// Create with a specific theme
    pub fn with_theme(theme: SyntaxTheme) -> Self {
        Self { theme }
    }

    /// Get the theme
    pub fn theme(&self) -> &SyntaxTheme {
        &self.theme
    }

    /// Set the theme
    pub fn set_theme(&mut self, theme: SyntaxTheme) {
        self.theme = theme;
    }

    /// Highlight a line of code
    pub fn highlight_line(&self, line: &str, lang: Language) -> Vec<Token> {
        self.highlight_line_with_state(line, lang, false).0
    }

    /// Highlight a line of code with block comment state
    /// Returns (tokens, in_block_comment_at_end)
    pub fn highlight_line_with_state(
        &self,
        line: &str,
        lang: Language,
        in_block_comment: bool,
    ) -> (Vec<Token>, bool) {
        let mut tokens = Vec::new();
        let keywords = lang.keywords();
        let types = lang.types();
        let (line_comment, block_comment) = lang.comment_patterns();
        let mut in_block = in_block_comment;

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        // If we're continuing from a block comment
        if in_block {
            if let Some((_, end)) = block_comment {
                // Look for end of block comment
                if let Some(end_pos) = line.find(end) {
                    tokens.push(Token::new(&line[..end_pos + end.len()], TokenType::Comment));
                    i = end_pos + end.len();
                    in_block = false;
                } else {
                    // Entire line is a comment
                    tokens.push(Token::new(line, TokenType::Comment));
                    return (tokens, true);
                }
            }
        }

        while i < chars.len() {
            // Check for block comment start
            if let Some((start, end)) = block_comment {
                if line[i..].starts_with(start) {
                    let comment_start = i;
                    i += start.len();
                    // Look for end of block comment on same line
                    if let Some(rel_end) = line[i..].find(end) {
                        let comment_end = i + rel_end + end.len();
                        tokens.push(Token::new(
                            &line[comment_start..comment_end],
                            TokenType::Comment,
                        ));
                        i = comment_end;
                        continue;
                    } else {
                        // Block comment continues to next line
                        tokens.push(Token::new(&line[comment_start..], TokenType::Comment));
                        return (tokens, true);
                    }
                }
            }

            // Check for line comment
            if !line_comment.is_empty() && line[i..].starts_with(line_comment) {
                tokens.push(Token::new(&line[i..], TokenType::Comment));
                break;
            }

            // Check for string (double quote)
            if chars[i] == '"' {
                let start = i;
                i += 1;
                while i < chars.len() {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        i += 2;
                    } else if chars[i] == '"' {
                        i += 1;
                        break;
                    } else {
                        i += 1;
                    }
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::new(s, TokenType::String));
                continue;
            }

            // Check for string (single quote)
            if chars[i] == '\'' && lang != Language::Rust {
                let start = i;
                i += 1;
                while i < chars.len() {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        i += 2;
                    } else if chars[i] == '\'' {
                        i += 1;
                        break;
                    } else {
                        i += 1;
                    }
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::new(s, TokenType::String));
                continue;
            }

            // Check for Rust char literal or lifetime
            if chars[i] == '\'' && lang == Language::Rust {
                let start = i;
                i += 1;
                // Lifetime or char
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                if i < chars.len() && chars[i] == '\'' {
                    i += 1; // char literal
                    let s: String = chars[start..i].iter().collect();
                    tokens.push(Token::new(s, TokenType::String));
                } else {
                    // Lifetime
                    let s: String = chars[start..i].iter().collect();
                    tokens.push(Token::new(s, TokenType::Type));
                }
                continue;
            }

            // Check for number
            if chars[i].is_ascii_digit()
                || (chars[i] == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
            {
                let start = i;
                // Hex
                if chars[i] == '0'
                    && i + 1 < chars.len()
                    && (chars[i + 1] == 'x' || chars[i + 1] == 'X')
                {
                    i += 2;
                    while i < chars.len() && chars[i].is_ascii_hexdigit() {
                        i += 1;
                    }
                } else {
                    // Decimal or float
                    while i < chars.len()
                        && (chars[i].is_ascii_digit()
                            || chars[i] == '.'
                            || chars[i] == 'e'
                            || chars[i] == 'E')
                    {
                        i += 1;
                    }
                }
                // Suffix (f32, u64, etc.)
                while i < chars.len() && chars[i].is_alphabetic() {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::new(s, TokenType::Number));
                continue;
            }

            // Check for Rust macro
            if lang == Language::Rust && chars[i].is_alphabetic() {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                if i < chars.len() && chars[i] == '!' {
                    i += 1;
                    let s: String = chars[start..i].iter().collect();
                    tokens.push(Token::new(s, TokenType::Macro));
                    continue;
                }
                // Reset and handle as identifier
                i = start;
            }

            // Check for identifier/keyword
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                // Check if it's a keyword
                if keywords.contains(&word.as_str()) {
                    tokens.push(Token::new(word, TokenType::Keyword));
                } else if types.contains(&word.as_str()) {
                    tokens.push(Token::new(word, TokenType::Type));
                } else if i < chars.len() && chars[i] == '(' {
                    // Function call
                    tokens.push(Token::new(word, TokenType::Function));
                } else {
                    tokens.push(Token::new(word, TokenType::Normal));
                }
                continue;
            }

            // Check for Rust attribute
            if lang == Language::Rust
                && chars[i] == '#'
                && i + 1 < chars.len()
                && chars[i + 1] == '['
            {
                let start = i;
                let mut depth = 0;
                while i < chars.len() {
                    if chars[i] == '[' {
                        depth += 1;
                    } else if chars[i] == ']' {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::new(s, TokenType::Attribute));
                continue;
            }

            // Check for Python/shell comment
            if (lang == Language::Python
                || lang == Language::Shell
                || lang == Language::Ruby
                || lang == Language::Yaml
                || lang == Language::Toml)
                && chars[i] == '#'
            {
                tokens.push(Token::new(&line[i..], TokenType::Comment));
                break;
            }

            // Python decorator
            if lang == Language::Python && chars[i] == '@' {
                let start = i;
                i += 1;
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '.')
                {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::new(s, TokenType::Attribute));
                continue;
            }

            // Operators and punctuation
            if "+-*/%=<>!&|^~?:;,.()[]{}@".contains(chars[i]) {
                tokens.push(Token::new(chars[i].to_string(), TokenType::Operator));
                i += 1;
                continue;
            }

            // Whitespace and other
            tokens.push(Token::new(chars[i].to_string(), TokenType::Normal));
            i += 1;
        }

        (tokens, in_block)
    }

    /// Highlight multiple lines of code with block comment tracking
    pub fn highlight(&self, code: &str, lang: Language) -> Vec<Vec<Token>> {
        let mut in_block_comment = false;
        code.lines()
            .map(|line| {
                let (tokens, still_in_block) =
                    self.highlight_line_with_state(line, lang, in_block_comment);
                in_block_comment = still_in_block;
                tokens
            })
            .collect()
    }

    /// Get color for a token
    pub fn token_color(&self, token_type: TokenType) -> crate::style::Color {
        self.theme.color(token_type)
    }
}
