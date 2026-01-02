//! Syntax highlighting for text widgets
//!
//! Provides simple regex-based syntax highlighting for common programming languages.
//! For more advanced highlighting, consider integrating with syntect or tree-sitter.

use crate::style::Color;

/// A highlighted span of text
#[derive(Clone, Debug)]
pub struct HighlightSpan {
    /// Start column (character index)
    pub start: usize,
    /// End column (exclusive)
    pub end: usize,
    /// Foreground color
    pub fg: Color,
    /// Whether to apply bold
    pub bold: bool,
    /// Whether to apply italic
    pub italic: bool,
}

impl HighlightSpan {
    /// Create a new highlight span
    pub fn new(start: usize, end: usize, fg: Color) -> Self {
        Self {
            start,
            end,
            fg,
            bold: false,
            italic: false,
        }
    }

    /// Set bold style
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set italic style
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
}

/// Syntax highlighting language
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Language {
    /// No syntax highlighting
    #[default]
    None,
    /// Rust
    Rust,
    /// Python
    Python,
    /// JavaScript/TypeScript
    JavaScript,
    /// JSON
    Json,
    /// TOML
    Toml,
    /// YAML
    Yaml,
    /// Markdown
    Markdown,
    /// Shell/Bash
    Shell,
    /// SQL
    Sql,
    /// HTML
    Html,
    /// CSS
    Css,
    /// Go
    Go,
}

impl Language {
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "py" | "pyw" => Language::Python,
            "js" | "jsx" | "ts" | "tsx" | "mjs" => Language::JavaScript,
            "json" => Language::Json,
            "toml" => Language::Toml,
            "yml" | "yaml" => Language::Yaml,
            "md" | "markdown" => Language::Markdown,
            "sh" | "bash" | "zsh" => Language::Shell,
            "sql" => Language::Sql,
            "html" | "htm" => Language::Html,
            "css" | "scss" | "sass" => Language::Css,
            "go" => Language::Go,
            _ => Language::None,
        }
    }
}

/// Color theme for syntax highlighting
#[derive(Clone, Debug)]
pub struct SyntaxTheme {
    /// Keywords (if, else, fn, etc.)
    pub keyword: Color,
    /// Types (String, i32, etc.)
    pub type_name: Color,
    /// Functions
    pub function: Color,
    /// Strings
    pub string: Color,
    /// Numbers
    pub number: Color,
    /// Comments
    pub comment: Color,
    /// Operators
    pub operator: Color,
    /// Punctuation
    pub punctuation: Color,
    /// Constants/booleans
    pub constant: Color,
    /// Macros
    pub macro_call: Color,
    /// Attributes/decorators
    pub attribute: Color,
    /// Variables
    pub variable: Color,
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl SyntaxTheme {
    /// Dark theme (default)
    pub fn dark() -> Self {
        Self {
            keyword: Color::rgb(198, 120, 221),   // Purple
            type_name: Color::rgb(229, 192, 123), // Yellow
            function: Color::rgb(97, 175, 239),   // Blue
            string: Color::rgb(152, 195, 121),    // Green
            number: Color::rgb(209, 154, 102),    // Orange
            comment: Color::rgb(92, 99, 112),     // Gray
            operator: Color::rgb(171, 178, 191),  // Light gray
            punctuation: Color::rgb(171, 178, 191),
            constant: Color::rgb(209, 154, 102),  // Orange
            macro_call: Color::rgb(86, 182, 194), // Cyan
            attribute: Color::rgb(229, 192, 123), // Yellow
            variable: Color::rgb(224, 108, 117),  // Red
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            keyword: Color::rgb(166, 38, 164),  // Purple
            type_name: Color::rgb(193, 132, 1), // Yellow/Brown
            function: Color::rgb(64, 120, 242), // Blue
            string: Color::rgb(80, 161, 79),    // Green
            number: Color::rgb(152, 104, 1),    // Brown
            comment: Color::rgb(160, 161, 167), // Gray
            operator: Color::rgb(56, 58, 66),   // Dark gray
            punctuation: Color::rgb(56, 58, 66),
            constant: Color::rgb(152, 104, 1),   // Brown
            macro_call: Color::rgb(1, 132, 188), // Cyan
            attribute: Color::rgb(193, 132, 1),  // Yellow/Brown
            variable: Color::rgb(228, 86, 73),   // Red
        }
    }

    /// Monokai theme
    pub fn monokai() -> Self {
        Self {
            keyword: Color::rgb(249, 38, 114),    // Pink
            type_name: Color::rgb(102, 217, 239), // Cyan
            function: Color::rgb(166, 226, 46),   // Green
            string: Color::rgb(230, 219, 116),    // Yellow
            number: Color::rgb(174, 129, 255),    // Purple
            comment: Color::rgb(117, 113, 94),    // Gray
            operator: Color::rgb(249, 38, 114),   // Pink
            punctuation: Color::rgb(248, 248, 242),
            constant: Color::rgb(174, 129, 255),   // Purple
            macro_call: Color::rgb(102, 217, 239), // Cyan
            attribute: Color::rgb(166, 226, 46),   // Green
            variable: Color::rgb(248, 248, 242),   // White
        }
    }
}

/// Syntax highlighter
#[derive(Clone, Debug)]
pub struct SyntaxHighlighter {
    /// Language to highlight
    language: Language,
    /// Color theme
    theme: SyntaxTheme,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self {
            language: Language::None,
            theme: SyntaxTheme::default(),
        }
    }
}

impl SyntaxHighlighter {
    /// Create a new highlighter for the given language
    pub fn new(language: Language) -> Self {
        Self {
            language,
            theme: SyntaxTheme::default(),
        }
    }

    /// Create a new highlighter with a custom theme
    pub fn with_theme(language: Language, theme: SyntaxTheme) -> Self {
        Self { language, theme }
    }

    /// Set the color theme
    pub fn theme(mut self, theme: SyntaxTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Set the language
    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    /// Get the current language
    pub fn get_language(&self) -> Language {
        self.language
    }

    /// Highlight a line of code
    pub fn highlight_line(&self, line: &str) -> Vec<HighlightSpan> {
        match self.language {
            Language::None => Vec::new(),
            Language::Rust => self.highlight_rust(line),
            Language::Python => self.highlight_python(line),
            Language::JavaScript => self.highlight_javascript(line),
            Language::Json => self.highlight_json(line),
            Language::Toml => self.highlight_toml(line),
            Language::Yaml => self.highlight_yaml(line),
            Language::Markdown => self.highlight_markdown(line),
            Language::Shell => self.highlight_shell(line),
            Language::Sql => self.highlight_sql(line),
            Language::Html => self.highlight_html(line),
            Language::Css => self.highlight_css(line),
            Language::Go => self.highlight_go(line),
        }
    }

    /// Highlight Rust code
    fn highlight_rust(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            // Skip whitespace
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
                spans.push(HighlightSpan::new(i, len, self.theme.comment).italic());
                break;
            }

            // Strings
            if chars[i] == '"' {
                let start = i;
                i += 1;
                while i < len && (chars[i] != '"' || (i > 0 && chars[i - 1] == '\\')) {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Character literals
            if chars[i] == '\'' && i + 2 < len {
                let start = i;
                i += 1;
                if chars[i] == '\\' {
                    i += 2;
                } else {
                    i += 1;
                }
                if i < len && chars[i] == '\'' {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Macros
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if i < len && chars[i] == '!' {
                    spans.push(HighlightSpan::new(start, i + 1, self.theme.macro_call));
                    i += 1;
                    continue;
                }

                // Keywords
                if is_rust_keyword(&word) {
                    spans.push(HighlightSpan::new(start, i, self.theme.keyword).bold());
                }
                // Types (starts with uppercase)
                else if word
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
                {
                    spans.push(HighlightSpan::new(start, i, self.theme.type_name));
                }
                // Constants
                else if word == "true" || word == "false" || word == "None" || word == "Some" {
                    spans.push(HighlightSpan::new(start, i, self.theme.constant));
                }
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit() {
                let start = i;
                while i < len
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '.')
                {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            // Attributes
            if chars[i] == '#' && i + 1 < len && chars[i + 1] == '[' {
                let start = i;
                while i < len && chars[i] != ']' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.attribute));
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight Python code
    fn highlight_python(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if chars[i] == '#' {
                spans.push(HighlightSpan::new(i, len, self.theme.comment).italic());
                break;
            }

            // Strings
            if chars[i] == '"' || chars[i] == '\'' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != quote {
                    if chars[i] == '\\' && i + 1 < len {
                        i += 1;
                    }
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Identifiers and keywords
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if is_python_keyword(&word) {
                    spans.push(HighlightSpan::new(start, i, self.theme.keyword).bold());
                } else if word == "True" || word == "False" || word == "None" {
                    spans.push(HighlightSpan::new(start, i, self.theme.constant));
                } else if i < len && chars[i] == '(' {
                    spans.push(HighlightSpan::new(start, i, self.theme.function));
                }
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit() {
                let start = i;
                while i < len
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '.')
                {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            // Decorators
            if chars[i] == '@' {
                let start = i;
                i += 1;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.attribute));
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight JavaScript code
    fn highlight_javascript(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
                spans.push(HighlightSpan::new(i, len, self.theme.comment).italic());
                break;
            }

            // Strings
            if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != quote {
                    if chars[i] == '\\' && i + 1 < len {
                        i += 1;
                    }
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Identifiers and keywords
            if chars[i].is_alphabetic() || chars[i] == '_' || chars[i] == '$' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '$')
                {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if is_javascript_keyword(&word) {
                    spans.push(HighlightSpan::new(start, i, self.theme.keyword).bold());
                } else if word == "true" || word == "false" || word == "null" || word == "undefined"
                {
                    spans.push(HighlightSpan::new(start, i, self.theme.constant));
                } else if i < len && chars[i] == '(' {
                    spans.push(HighlightSpan::new(start, i, self.theme.function));
                }
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit() {
                let start = i;
                while i < len
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '.')
                {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight JSON
    fn highlight_json(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;
        let mut in_key = true;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Strings
            if chars[i] == '"' {
                let start = i;
                i += 1;
                while i < len && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < len {
                        i += 1;
                    }
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                let color = if in_key {
                    self.theme.variable
                } else {
                    self.theme.string
                };
                spans.push(HighlightSpan::new(start, i, color));

                // Check for colon after key
                let mut j = i;
                while j < len && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < len && chars[j] == ':' {
                    in_key = false;
                }
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit() || chars[i] == '-' {
                let start = i;
                if chars[i] == '-' {
                    i += 1;
                }
                while i < len
                    && (chars[i].is_ascii_digit()
                        || chars[i] == '.'
                        || chars[i] == 'e'
                        || chars[i] == 'E'
                        || chars[i] == '+'
                        || chars[i] == '-')
                {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            // Boolean and null
            if chars[i].is_alphabetic() {
                let start = i;
                while i < len && chars[i].is_alphabetic() {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                if word == "true" || word == "false" || word == "null" {
                    spans.push(HighlightSpan::new(start, i, self.theme.constant));
                }
                continue;
            }

            // Reset key detection on comma or opening brace/bracket
            if chars[i] == ',' || chars[i] == '{' || chars[i] == '[' {
                in_key = true;
            }

            i += 1;
        }

        spans
    }

    /// Highlight TOML
    fn highlight_toml(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if chars[i] == '#' {
                spans.push(HighlightSpan::new(i, len, self.theme.comment).italic());
                break;
            }

            // Section headers
            if chars[i] == '[' {
                let start = i;
                while i < len && chars[i] != ']' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.keyword).bold());
                continue;
            }

            // Strings
            if chars[i] == '"' || chars[i] == '\'' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != quote {
                    if chars[i] == '\\' && i + 1 < len {
                        i += 1;
                    }
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Keys (before =)
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-')
                {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                // Check if followed by =
                let mut j = i;
                while j < len && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < len && chars[j] == '=' {
                    spans.push(HighlightSpan::new(start, i, self.theme.variable));
                } else if word == "true" || word == "false" {
                    spans.push(HighlightSpan::new(start, i, self.theme.constant));
                }
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit() || chars[i] == '-' || chars[i] == '+' {
                let start = i;
                while i < len
                    && (chars[i].is_ascii_alphanumeric()
                        || chars[i] == '_'
                        || chars[i] == '.'
                        || chars[i] == '-'
                        || chars[i] == '+'
                        || chars[i] == ':')
                {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight YAML
    fn highlight_yaml(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if chars[i] == '#' {
                spans.push(HighlightSpan::new(i, len, self.theme.comment).italic());
                break;
            }

            // Keys (word followed by colon)
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-')
                {
                    i += 1;
                }

                // Check if followed by :
                if i < len && chars[i] == ':' {
                    spans.push(HighlightSpan::new(start, i, self.theme.variable));
                    i += 1;
                    continue;
                }

                let word: String = chars[start..i].iter().collect();
                if word == "true"
                    || word == "false"
                    || word == "null"
                    || word == "yes"
                    || word == "no"
                {
                    spans.push(HighlightSpan::new(start, i, self.theme.constant));
                }
                continue;
            }

            // Strings
            if chars[i] == '"' || chars[i] == '\'' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != quote {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit() || chars[i] == '-' {
                let start = i;
                while i < len
                    && (chars[i].is_ascii_alphanumeric()
                        || chars[i] == '.'
                        || chars[i] == '-'
                        || chars[i] == '+')
                {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight Markdown
    fn highlight_markdown(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();

        // Headers
        if !chars.is_empty() && chars[0] == '#' {
            let mut i = 0;
            while i < len && chars[i] == '#' {
                i += 1;
            }
            spans.push(HighlightSpan::new(0, len, self.theme.keyword).bold());
            return spans;
        }

        let mut i = 0;
        while i < len {
            // Bold **text** or __text__
            if i + 1 < len
                && ((chars[i] == '*' && chars[i + 1] == '*')
                    || (chars[i] == '_' && chars[i + 1] == '_'))
            {
                let marker = chars[i];
                let start = i;
                i += 2;
                while i + 1 < len && !(chars[i] == marker && chars[i + 1] == marker) {
                    i += 1;
                }
                if i + 1 < len {
                    i += 2;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.constant).bold());
                continue;
            }

            // Italic *text* or _text_
            if chars[i] == '*' || chars[i] == '_' {
                let marker = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != marker {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string).italic());
                continue;
            }

            // Code `text`
            if chars[i] == '`' {
                let start = i;
                i += 1;
                while i < len && chars[i] != '`' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.function));
                continue;
            }

            // Links [text](url)
            if chars[i] == '[' {
                let start = i;
                while i < len && chars[i] != ']' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                if i < len && chars[i] == '(' {
                    while i < len && chars[i] != ')' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                }
                spans.push(HighlightSpan::new(start, i, self.theme.type_name));
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight Shell/Bash
    fn highlight_shell(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if chars[i] == '#' {
                spans.push(HighlightSpan::new(i, len, self.theme.comment).italic());
                break;
            }

            // Strings
            if chars[i] == '"' || chars[i] == '\'' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != quote {
                    if chars[i] == '\\' && i + 1 < len {
                        i += 1;
                    }
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Variables $VAR or ${VAR}
            if chars[i] == '$' {
                let start = i;
                i += 1;
                if i < len && chars[i] == '{' {
                    while i < len && chars[i] != '}' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                } else {
                    while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                        i += 1;
                    }
                }
                spans.push(HighlightSpan::new(start, i, self.theme.variable));
                continue;
            }

            // Commands/keywords
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-')
                {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if is_shell_keyword(&word) {
                    spans.push(HighlightSpan::new(start, i, self.theme.keyword).bold());
                }
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight SQL
    fn highlight_sql(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if i + 1 < len && chars[i] == '-' && chars[i + 1] == '-' {
                spans.push(HighlightSpan::new(i, len, self.theme.comment).italic());
                break;
            }

            // Strings
            if chars[i] == '\'' {
                let start = i;
                i += 1;
                while i < len && chars[i] != '\'' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Keywords
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if is_sql_keyword(&word) {
                    spans.push(HighlightSpan::new(start, i, self.theme.keyword).bold());
                } else if word.to_uppercase() == "NULL"
                    || word.to_uppercase() == "TRUE"
                    || word.to_uppercase() == "FALSE"
                {
                    spans.push(HighlightSpan::new(start, i, self.theme.constant));
                }
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit() {
                let start = i;
                while i < len && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight HTML
    fn highlight_html(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            // Tags
            if chars[i] == '<' {
                let start = i;
                i += 1;
                // Tag name
                while i < len && !chars[i].is_whitespace() && chars[i] != '>' && chars[i] != '/' {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.keyword));

                // Attributes
                while i < len && chars[i] != '>' {
                    if chars[i].is_alphabetic() {
                        let attr_start = i;
                        while i < len && (chars[i].is_alphanumeric() || chars[i] == '-') {
                            i += 1;
                        }
                        spans.push(HighlightSpan::new(attr_start, i, self.theme.variable));
                    } else if chars[i] == '"' || chars[i] == '\'' {
                        let quote = chars[i];
                        let str_start = i;
                        i += 1;
                        while i < len && chars[i] != quote {
                            i += 1;
                        }
                        if i < len {
                            i += 1;
                        }
                        spans.push(HighlightSpan::new(str_start, i, self.theme.string));
                    } else {
                        i += 1;
                    }
                }
                if i < len && chars[i] == '>' {
                    i += 1;
                }
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight CSS
    fn highlight_css(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
                let start = i;
                while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                if i + 1 < len {
                    i += 2;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.comment).italic());
                continue;
            }

            // Selectors (class, id, element)
            if chars[i] == '.' || chars[i] == '#' || chars[i].is_alphabetic() {
                let start = i;
                while i < len
                    && (chars[i].is_alphanumeric()
                        || chars[i] == '-'
                        || chars[i] == '_'
                        || chars[i] == '.'
                        || chars[i] == '#')
                {
                    i += 1;
                }
                // Check if it's before a { (selector) or : (property)
                let mut j = i;
                while j < len && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < len && chars[j] == '{' {
                    spans.push(HighlightSpan::new(start, i, self.theme.type_name));
                } else if j < len && chars[j] == ':' {
                    spans.push(HighlightSpan::new(start, i, self.theme.variable));
                }
                continue;
            }

            // Strings
            if chars[i] == '"' || chars[i] == '\'' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != quote {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Numbers and units
            if chars[i].is_ascii_digit() || chars[i] == '-' {
                let start = i;
                if chars[i] == '-' {
                    i += 1;
                }
                while i < len
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '%')
                {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            // Colors #hex
            if chars[i] == '#' {
                let start = i;
                i += 1;
                while i < len && chars[i].is_ascii_hexdigit() {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.constant));
                continue;
            }

            i += 1;
        }

        spans
    }

    /// Highlight Go
    fn highlight_go(&self, line: &str) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Comments
            if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
                spans.push(HighlightSpan::new(i, len, self.theme.comment).italic());
                break;
            }

            // Strings
            if chars[i] == '"' || chars[i] == '`' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < len && chars[i] != quote {
                    if chars[i] == '\\' && i + 1 < len && quote == '"' {
                        i += 1;
                    }
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.string));
                continue;
            }

            // Identifiers and keywords
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if is_go_keyword(&word) {
                    spans.push(HighlightSpan::new(start, i, self.theme.keyword).bold());
                } else if word == "true" || word == "false" || word == "nil" {
                    spans.push(HighlightSpan::new(start, i, self.theme.constant));
                } else if word
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
                {
                    spans.push(HighlightSpan::new(start, i, self.theme.type_name));
                } else if i < len && chars[i] == '(' {
                    spans.push(HighlightSpan::new(start, i, self.theme.function));
                }
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit() {
                let start = i;
                while i < len
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_')
                {
                    i += 1;
                }
                spans.push(HighlightSpan::new(start, i, self.theme.number));
                continue;
            }

            i += 1;
        }

        spans
    }
}

// Keyword lists

fn is_rust_keyword(word: &str) -> bool {
    matches!(
        word,
        "as" | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
    )
}

fn is_python_keyword(word: &str) -> bool {
    matches!(
        word,
        "and"
            | "as"
            | "assert"
            | "async"
            | "await"
            | "break"
            | "class"
            | "continue"
            | "def"
            | "del"
            | "elif"
            | "else"
            | "except"
            | "finally"
            | "for"
            | "from"
            | "global"
            | "if"
            | "import"
            | "in"
            | "is"
            | "lambda"
            | "nonlocal"
            | "not"
            | "or"
            | "pass"
            | "raise"
            | "return"
            | "try"
            | "while"
            | "with"
            | "yield"
    )
}

fn is_javascript_keyword(word: &str) -> bool {
    matches!(
        word,
        "async"
            | "await"
            | "break"
            | "case"
            | "catch"
            | "class"
            | "const"
            | "continue"
            | "debugger"
            | "default"
            | "delete"
            | "do"
            | "else"
            | "export"
            | "extends"
            | "finally"
            | "for"
            | "function"
            | "if"
            | "import"
            | "in"
            | "instanceof"
            | "let"
            | "new"
            | "of"
            | "return"
            | "static"
            | "super"
            | "switch"
            | "this"
            | "throw"
            | "try"
            | "typeof"
            | "var"
            | "void"
            | "while"
            | "with"
            | "yield"
    )
}

fn is_shell_keyword(word: &str) -> bool {
    matches!(
        word,
        "if" | "then"
            | "else"
            | "elif"
            | "fi"
            | "for"
            | "while"
            | "do"
            | "done"
            | "case"
            | "esac"
            | "in"
            | "function"
            | "select"
            | "until"
            | "return"
            | "exit"
            | "export"
            | "local"
            | "readonly"
            | "declare"
            | "typeset"
            | "unset"
            | "shift"
            | "cd"
            | "echo"
            | "read"
            | "source"
            | "alias"
            | "eval"
            | "exec"
            | "set"
    )
}

fn is_sql_keyword(word: &str) -> bool {
    let upper = word.to_uppercase();
    matches!(
        upper.as_str(),
        "SELECT"
            | "FROM"
            | "WHERE"
            | "AND"
            | "OR"
            | "NOT"
            | "IN"
            | "LIKE"
            | "BETWEEN"
            | "JOIN"
            | "LEFT"
            | "RIGHT"
            | "INNER"
            | "OUTER"
            | "ON"
            | "AS"
            | "ORDER"
            | "BY"
            | "GROUP"
            | "HAVING"
            | "LIMIT"
            | "OFFSET"
            | "INSERT"
            | "INTO"
            | "VALUES"
            | "UPDATE"
            | "SET"
            | "DELETE"
            | "CREATE"
            | "TABLE"
            | "DROP"
            | "ALTER"
            | "INDEX"
            | "PRIMARY"
            | "KEY"
            | "FOREIGN"
            | "REFERENCES"
            | "CONSTRAINT"
            | "UNIQUE"
            | "DEFAULT"
            | "NULL"
            | "IS"
            | "ASC"
            | "DESC"
            | "DISTINCT"
            | "COUNT"
            | "SUM"
            | "AVG"
            | "MIN"
            | "MAX"
            | "CASE"
            | "WHEN"
            | "THEN"
            | "ELSE"
            | "END"
            | "UNION"
            | "ALL"
            | "EXISTS"
    )
}

fn is_go_keyword(word: &str) -> bool {
    matches!(
        word,
        "break"
            | "case"
            | "chan"
            | "const"
            | "continue"
            | "default"
            | "defer"
            | "else"
            | "fallthrough"
            | "for"
            | "func"
            | "go"
            | "goto"
            | "if"
            | "import"
            | "interface"
            | "map"
            | "package"
            | "range"
            | "return"
            | "select"
            | "struct"
            | "switch"
            | "type"
            | "var"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(Language::from_extension("js"), Language::JavaScript);
        assert_eq!(Language::from_extension("json"), Language::Json);
        assert_eq!(Language::from_extension("unknown"), Language::None);
    }

    #[test]
    fn test_rust_highlighting() {
        let hl = SyntaxHighlighter::new(Language::Rust);
        let spans = hl.highlight_line("fn main() {");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_rust_comment_highlighting() {
        let hl = SyntaxHighlighter::new(Language::Rust);
        let spans = hl.highlight_line("// This is a comment");
        assert_eq!(spans.len(), 1);
        assert!(spans[0].italic);
    }

    #[test]
    fn test_rust_string_highlighting() {
        let hl = SyntaxHighlighter::new(Language::Rust);
        let spans = hl.highlight_line("let s = \"hello\";");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_python_highlighting() {
        let hl = SyntaxHighlighter::new(Language::Python);
        let spans = hl.highlight_line("def hello():");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_json_highlighting() {
        let hl = SyntaxHighlighter::new(Language::Json);
        let spans = hl.highlight_line("{\"key\": \"value\"}");
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_no_highlighting() {
        let hl = SyntaxHighlighter::new(Language::None);
        let spans = hl.highlight_line("fn main() {}");
        assert!(spans.is_empty());
    }

    #[test]
    fn test_syntax_theme() {
        let dark = SyntaxTheme::dark();
        let light = SyntaxTheme::light();
        let monokai = SyntaxTheme::monokai();

        // Just check they're different
        assert_ne!(dark.keyword.r, light.keyword.r);
        assert_ne!(dark.keyword.r, monokai.keyword.r);
    }
}
