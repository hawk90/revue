//! Syntax highlighting for code blocks
//!
//! Provides simple keyword-based syntax highlighting for common programming languages.

use crate::style::Color;

/// Token type for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Normal text
    Normal,
    /// Language keywords (fn, let, if, etc.)
    Keyword,
    /// Built-in types (int, str, bool, etc.)
    Type,
    /// String literals
    String,
    /// Number literals
    Number,
    /// Comments
    Comment,
    /// Function names
    Function,
    /// Operators and punctuation
    Operator,
    /// Macros (Rust)
    Macro,
    /// Attributes/decorators
    Attribute,
}

/// A highlighted token
#[derive(Debug, Clone)]
pub struct Token {
    /// The text content of the token
    pub text: String,
    /// The type of the token (keyword, string, etc.)
    pub token_type: TokenType,
}

impl Token {
    /// Create a new token
    pub fn new(text: impl Into<String>, token_type: TokenType) -> Self {
        Self {
            text: text.into(),
            token_type,
        }
    }
}

/// Theme for syntax highlighting
#[derive(Debug, Clone)]
pub struct SyntaxTheme {
    /// Color for keywords (fn, let, if, etc.)
    pub keyword: Color,
    /// Color for types (int, String, etc.)
    pub type_: Color,
    /// Color for string literals
    pub string: Color,
    /// Color for number literals
    pub number: Color,
    /// Color for comments
    pub comment: Color,
    /// Color for function names
    pub function: Color,
    /// Color for operators and punctuation
    pub operator: Color,
    /// Color for macros (Rust)
    pub macro_: Color,
    /// Color for attributes/decorators
    pub attribute: Color,
    /// Color for normal text
    pub normal: Color,
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        Self::monokai()
    }
}

impl SyntaxTheme {
    /// Monokai-inspired theme
    pub fn monokai() -> Self {
        Self {
            keyword: Color::rgb(249, 38, 114),    // Pink
            type_: Color::rgb(102, 217, 239),     // Cyan
            string: Color::rgb(230, 219, 116),    // Yellow
            number: Color::rgb(174, 129, 255),    // Purple
            comment: Color::rgb(117, 113, 94),    // Gray
            function: Color::rgb(166, 226, 46),   // Green
            operator: Color::rgb(249, 38, 114),   // Pink
            macro_: Color::rgb(102, 217, 239),    // Cyan
            attribute: Color::rgb(174, 129, 255), // Purple
            normal: Color::rgb(248, 248, 242),    // White
        }
    }

    /// Nord-inspired theme
    pub fn nord() -> Self {
        Self {
            keyword: Color::rgb(129, 161, 193),   // Blue
            type_: Color::rgb(143, 188, 187),     // Teal
            string: Color::rgb(163, 190, 140),    // Green
            number: Color::rgb(180, 142, 173),    // Purple
            comment: Color::rgb(76, 86, 106),     // Gray
            function: Color::rgb(136, 192, 208),  // Light Blue
            operator: Color::rgb(129, 161, 193),  // Blue
            macro_: Color::rgb(208, 135, 112),    // Orange
            attribute: Color::rgb(180, 142, 173), // Purple
            normal: Color::rgb(236, 239, 244),    // White
        }
    }

    /// Dracula-inspired theme
    pub fn dracula() -> Self {
        Self {
            keyword: Color::rgb(255, 121, 198),   // Pink
            type_: Color::rgb(139, 233, 253),     // Cyan
            string: Color::rgb(241, 250, 140),    // Yellow
            number: Color::rgb(189, 147, 249),    // Purple
            comment: Color::rgb(98, 114, 164),    // Gray
            function: Color::rgb(80, 250, 123),   // Green
            operator: Color::rgb(255, 121, 198),  // Pink
            macro_: Color::rgb(255, 184, 108),    // Orange
            attribute: Color::rgb(189, 147, 249), // Purple
            normal: Color::rgb(248, 248, 242),    // White
        }
    }

    /// One Dark-inspired theme
    pub fn one_dark() -> Self {
        Self {
            keyword: Color::rgb(198, 120, 221),   // Purple
            type_: Color::rgb(229, 192, 123),     // Yellow
            string: Color::rgb(152, 195, 121),    // Green
            number: Color::rgb(209, 154, 102),    // Orange
            comment: Color::rgb(92, 99, 112),     // Gray
            function: Color::rgb(97, 175, 239),   // Blue
            operator: Color::rgb(171, 178, 191),  // Light gray
            macro_: Color::rgb(86, 182, 194),     // Cyan
            attribute: Color::rgb(209, 154, 102), // Orange
            normal: Color::rgb(171, 178, 191),    // Light gray
        }
    }

    /// Get color for token type
    pub fn color(&self, token_type: TokenType) -> Color {
        match token_type {
            TokenType::Normal => self.normal,
            TokenType::Keyword => self.keyword,
            TokenType::Type => self.type_,
            TokenType::String => self.string,
            TokenType::Number => self.number,
            TokenType::Comment => self.comment,
            TokenType::Function => self.function,
            TokenType::Operator => self.operator,
            TokenType::Macro => self.macro_,
            TokenType::Attribute => self.attribute,
        }
    }
}

/// Supported languages for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// Rust programming language
    Rust,
    /// Python programming language
    Python,
    /// JavaScript programming language
    JavaScript,
    /// TypeScript programming language
    TypeScript,
    /// Go programming language
    Go,
    /// C programming language
    C,
    /// C++ programming language
    Cpp,
    /// Java programming language
    Java,
    /// Ruby programming language
    Ruby,
    /// Shell/Bash scripting
    Shell,
    /// JSON data format
    Json,
    /// YAML data format
    Yaml,
    /// TOML configuration format
    Toml,
    /// Markdown markup language
    Markdown,
    /// SQL query language
    Sql,
    /// HTML markup language
    Html,
    /// CSS stylesheet language
    Css,
    /// Unknown or unsupported language
    Unknown,
}

impl Language {
    /// Detect language from code fence string
    pub fn from_fence(fence: &str) -> Self {
        let fence = fence.to_lowercase();
        match fence.as_str() {
            "rust" | "rs" => Self::Rust,
            "python" | "py" => Self::Python,
            "javascript" | "js" => Self::JavaScript,
            "typescript" | "ts" => Self::TypeScript,
            "go" | "golang" => Self::Go,
            "c" => Self::C,
            "c++" | "cpp" | "cxx" => Self::Cpp,
            "java" => Self::Java,
            "ruby" | "rb" => Self::Ruby,
            "shell" | "bash" | "sh" | "zsh" => Self::Shell,
            "json" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "toml" => Self::Toml,
            "markdown" | "md" => Self::Markdown,
            "sql" => Self::Sql,
            "html" | "htm" => Self::Html,
            "css" => Self::Css,
            _ => Self::Unknown,
        }
    }

    /// Get keywords for this language
    fn keywords(&self) -> &'static [&'static str] {
        match self {
            Self::Rust => &[
                "fn", "let", "mut", "const", "static", "if", "else", "match", "for", "while",
                "loop", "break", "continue", "return", "pub", "use", "mod", "struct", "enum",
                "impl", "trait", "type", "where", "as", "in", "ref", "self", "Self", "super",
                "crate", "async", "await", "dyn", "move", "unsafe", "extern", "true", "false",
            ],
            Self::Python => &[
                "def", "class", "if", "elif", "else", "for", "while", "try", "except", "finally",
                "with", "as", "import", "from", "return", "yield", "raise", "pass", "break",
                "continue", "and", "or", "not", "in", "is", "lambda", "True", "False", "None",
                "global", "nonlocal", "assert", "async", "await",
            ],
            Self::JavaScript | Self::TypeScript => &[
                "function",
                "const",
                "let",
                "var",
                "if",
                "else",
                "for",
                "while",
                "do",
                "switch",
                "case",
                "default",
                "break",
                "continue",
                "return",
                "try",
                "catch",
                "finally",
                "throw",
                "new",
                "delete",
                "typeof",
                "instanceof",
                "in",
                "of",
                "class",
                "extends",
                "super",
                "this",
                "import",
                "export",
                "from",
                "as",
                "async",
                "await",
                "yield",
                "true",
                "false",
                "null",
                "undefined",
            ],
            Self::Go => &[
                "func",
                "var",
                "const",
                "type",
                "struct",
                "interface",
                "map",
                "chan",
                "if",
                "else",
                "for",
                "range",
                "switch",
                "case",
                "default",
                "break",
                "continue",
                "return",
                "go",
                "defer",
                "select",
                "package",
                "import",
                "true",
                "false",
                "nil",
            ],
            Self::C | Self::Cpp => &[
                "if",
                "else",
                "for",
                "while",
                "do",
                "switch",
                "case",
                "default",
                "break",
                "continue",
                "return",
                "goto",
                "sizeof",
                "typedef",
                "struct",
                "union",
                "enum",
                "static",
                "extern",
                "const",
                "volatile",
                "register",
                "auto",
                "inline",
                "true",
                "false",
                "NULL",
                "nullptr",
                // C++ specific
                "class",
                "public",
                "private",
                "protected",
                "virtual",
                "override",
                "final",
                "new",
                "delete",
                "this",
                "namespace",
                "using",
                "template",
                "typename",
                "try",
                "catch",
                "throw",
                "noexcept",
                "constexpr",
            ],
            Self::Java => &[
                "class",
                "interface",
                "extends",
                "implements",
                "public",
                "private",
                "protected",
                "static",
                "final",
                "abstract",
                "synchronized",
                "volatile",
                "transient",
                "if",
                "else",
                "for",
                "while",
                "do",
                "switch",
                "case",
                "default",
                "break",
                "continue",
                "return",
                "try",
                "catch",
                "finally",
                "throw",
                "throws",
                "new",
                "this",
                "super",
                "instanceof",
                "import",
                "package",
                "true",
                "false",
                "null",
            ],
            Self::Ruby => &[
                "def", "class", "module", "if", "elsif", "else", "unless", "case", "when", "while",
                "until", "for", "do", "end", "begin", "rescue", "ensure", "raise", "return",
                "yield", "break", "next", "redo", "retry", "self", "super", "true", "false", "nil",
                "and", "or", "not", "in", "then",
            ],
            Self::Shell => &[
                "if", "then", "else", "elif", "fi", "case", "esac", "for", "while", "until", "do",
                "done", "in", "function", "return", "exit", "break", "continue", "export", "local",
                "readonly", "unset", "true", "false",
            ],
            Self::Sql => &[
                "SELECT",
                "FROM",
                "WHERE",
                "AND",
                "OR",
                "NOT",
                "IN",
                "LIKE",
                "BETWEEN",
                "JOIN",
                "LEFT",
                "RIGHT",
                "INNER",
                "OUTER",
                "ON",
                "GROUP",
                "BY",
                "HAVING",
                "ORDER",
                "ASC",
                "DESC",
                "LIMIT",
                "OFFSET",
                "INSERT",
                "INTO",
                "VALUES",
                "UPDATE",
                "SET",
                "DELETE",
                "CREATE",
                "TABLE",
                "DROP",
                "ALTER",
                "INDEX",
                "PRIMARY",
                "KEY",
                "FOREIGN",
                "REFERENCES",
                "NULL",
                "DEFAULT",
                "UNIQUE",
                "AS",
                "DISTINCT",
                "COUNT",
                "SUM",
                "AVG",
                "MIN",
                "MAX",
                "CASE",
                "WHEN",
                "THEN",
                "ELSE",
                "END",
                "UNION",
                "ALL",
                "EXISTS",
            ],
            _ => &[],
        }
    }

    /// Get type keywords for this language
    fn types(&self) -> &'static [&'static str] {
        match self {
            Self::Rust => &[
                "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
                "usize", "f32", "f64", "bool", "char", "str", "String", "Vec", "Box", "Rc", "Arc",
                "Option", "Result", "HashMap", "HashSet", "BTreeMap", "BTreeSet",
            ],
            Self::Python => &[
                "int",
                "float",
                "str",
                "bool",
                "list",
                "dict",
                "set",
                "tuple",
                "bytes",
                "bytearray",
                "object",
                "type",
            ],
            Self::JavaScript | Self::TypeScript => &[
                "number", "string", "boolean", "object", "Array", "Map", "Set", "Promise", "Date",
                "RegExp", "Error", "Symbol", "BigInt", "void", "any", "unknown", "never",
            ],
            Self::Go => &[
                "int",
                "int8",
                "int16",
                "int32",
                "int64",
                "uint",
                "uint8",
                "uint16",
                "uint32",
                "uint64",
                "float32",
                "float64",
                "complex64",
                "complex128",
                "bool",
                "byte",
                "rune",
                "string",
                "error",
            ],
            Self::C | Self::Cpp => &[
                "int",
                "long",
                "short",
                "char",
                "float",
                "double",
                "void",
                "signed",
                "unsigned",
                "bool",
                "size_t",
                "ptrdiff_t",
                "int8_t",
                "int16_t",
                "int32_t",
                "int64_t",
                "uint8_t",
                "uint16_t",
                "uint32_t",
                "uint64_t",
                "string",
                "vector",
                "map",
                "set",
                "array",
                "deque",
                "list",
            ],
            Self::Java => &[
                "int", "long", "short", "byte", "float", "double", "char", "boolean", "void",
                "String", "Integer", "Long", "Double", "Boolean", "Object", "List", "Map", "Set",
                "Array",
            ],
            Self::Ruby => &[
                "Integer",
                "Float",
                "String",
                "Array",
                "Hash",
                "Symbol",
                "TrueClass",
                "FalseClass",
                "NilClass",
                "Object",
                "Class",
            ],
            Self::Sql => &[
                "INT",
                "INTEGER",
                "BIGINT",
                "SMALLINT",
                "TINYINT",
                "FLOAT",
                "DOUBLE",
                "DECIMAL",
                "NUMERIC",
                "VARCHAR",
                "CHAR",
                "TEXT",
                "BLOB",
                "DATE",
                "TIME",
                "DATETIME",
                "TIMESTAMP",
                "BOOLEAN",
                "BOOL",
            ],
            _ => &[],
        }
    }

    /// Get comment patterns for this language
    fn comment_patterns(&self) -> (&'static str, Option<(&'static str, &'static str)>) {
        match self {
            Self::Rust
            | Self::Go
            | Self::C
            | Self::Cpp
            | Self::Java
            | Self::JavaScript
            | Self::TypeScript
            | Self::Css => ("//", Some(("/*", "*/"))),
            Self::Python | Self::Ruby | Self::Shell | Self::Yaml | Self::Toml => ("#", None),
            Self::Sql => ("--", Some(("/*", "*/"))),
            Self::Html => ("", Some(("<!--", "-->"))),
            _ => ("//", None),
        }
    }
}

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
    pub fn token_color(&self, token_type: TokenType) -> Color {
        self.theme.color(token_type)
    }
}

/// Highlight code and return tokens
pub fn highlight(code: &str, lang: &str) -> Vec<Vec<Token>> {
    let highlighter = SyntaxHighlighter::new();
    let language = Language::from_fence(lang);
    highlighter.highlight(code, language)
}

/// Highlight a single line and return tokens
pub fn highlight_line(line: &str, lang: &str) -> Vec<Token> {
    let highlighter = SyntaxHighlighter::new();
    let language = Language::from_fence(lang);
    highlighter.highlight_line(line, language)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_fence() {
        assert_eq!(Language::from_fence("rust"), Language::Rust);
        assert_eq!(Language::from_fence("rs"), Language::Rust);
        assert_eq!(Language::from_fence("python"), Language::Python);
        assert_eq!(Language::from_fence("py"), Language::Python);
        assert_eq!(Language::from_fence("javascript"), Language::JavaScript);
        assert_eq!(Language::from_fence("js"), Language::JavaScript);
        assert_eq!(Language::from_fence("unknown"), Language::Unknown);
    }

    #[test]
    fn test_highlight_rust() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("fn main() {", Language::Rust);

        assert!(!tokens.is_empty());
        // "fn" should be a keyword
        assert_eq!(tokens[0].text, "fn");
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        // "main" should be a function
        assert_eq!(tokens[2].text, "main");
        assert_eq!(tokens[2].token_type, TokenType::Function);
    }

    #[test]
    fn test_highlight_string() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("let s = \"hello\";", Language::Rust);

        // Find string token
        let string_token = tokens.iter().find(|t| t.text == "\"hello\"");
        assert!(string_token.is_some());
        assert_eq!(string_token.unwrap().token_type, TokenType::String);
    }

    #[test]
    fn test_highlight_number() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("let x = 42;", Language::Rust);

        // Find number token
        let num_token = tokens.iter().find(|t| t.text == "42");
        assert!(num_token.is_some());
        assert_eq!(num_token.unwrap().token_type, TokenType::Number);
    }

    #[test]
    fn test_highlight_comment() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("let x = 1; // comment", Language::Rust);

        // Find comment token
        let comment_token = tokens.iter().find(|t| t.text.contains("comment"));
        assert!(comment_token.is_some());
        assert_eq!(comment_token.unwrap().token_type, TokenType::Comment);
    }

    #[test]
    fn test_highlight_python() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("def foo():", Language::Python);

        assert_eq!(tokens[0].text, "def");
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
    }

    #[test]
    fn test_highlight_rust_macro() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("println!(\"hello\");", Language::Rust);

        assert_eq!(tokens[0].text, "println!");
        assert_eq!(tokens[0].token_type, TokenType::Macro);
    }

    #[test]
    fn test_highlight_rust_attribute() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("#[derive(Debug)]", Language::Rust);

        assert_eq!(tokens[0].token_type, TokenType::Attribute);
    }

    #[test]
    fn test_syntax_theme() {
        let theme = SyntaxTheme::monokai();
        assert_eq!(theme.keyword, Color::rgb(249, 38, 114));

        let theme = SyntaxTheme::nord();
        assert_eq!(theme.keyword, Color::rgb(129, 161, 193));

        let theme = SyntaxTheme::dracula();
        assert_eq!(theme.keyword, Color::rgb(255, 121, 198));

        let theme = SyntaxTheme::one_dark();
        assert_eq!(theme.keyword, Color::rgb(198, 120, 221));
    }

    #[test]
    fn test_highlight_function() {
        let tokens = highlight("fn main() { println!(\"test\"); }", "rust");
        assert!(!tokens.is_empty());
        assert!(!tokens[0].is_empty());
    }

    #[test]
    fn test_highlight_line_function() {
        let tokens = highlight_line("for i in range(10):", "python");
        assert!(!tokens.is_empty());
        // "for" should be keyword
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
    }

    #[test]
    fn test_block_comment_single_line() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.highlight_line("let x /* comment */ = 1;", Language::Rust);

        // Find block comment token
        let comment_token = tokens.iter().find(|t| t.text.contains("comment"));
        assert!(comment_token.is_some());
        assert_eq!(comment_token.unwrap().token_type, TokenType::Comment);
    }

    #[test]
    fn test_block_comment_multiline() {
        let highlighter = SyntaxHighlighter::new();
        let code = "/* start\nmiddle\nend */\nlet x = 1;";
        let tokens = highlighter.highlight(code, Language::Rust);

        // First 3 lines should be comments
        assert!(tokens[0].iter().all(|t| t.token_type == TokenType::Comment));
        assert!(tokens[1].iter().all(|t| t.token_type == TokenType::Comment));
        assert!(tokens[2].iter().all(|t| t.token_type == TokenType::Comment));
        // Fourth line should have non-comment tokens
        assert!(tokens[3].iter().any(|t| t.token_type == TokenType::Keyword));
    }
}
