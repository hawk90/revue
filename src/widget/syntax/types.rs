//! Syntax highlighting types

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
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
