//! Syntax highlighting widget tests

use revue::style::Color;
use revue::widget::{HighlightSpan, Language, SyntaxHighlighter, SyntaxTheme};

// HighlightSpan builder method tests

#[test]
fn test_highlight_span_new() {
    let span = HighlightSpan::new(0, 5, Color::RED);
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 5);
    assert_eq!(span.fg, Color::RED);
    assert!(!span.bold);
    assert!(!span.italic);
}

#[test]
fn test_highlight_span_bold() {
    let span = HighlightSpan::new(0, 5, Color::BLUE).bold();
    assert!(span.bold);
    assert!(!span.italic);
}

#[test]
fn test_highlight_span_italic() {
    let span = HighlightSpan::new(0, 5, Color::GREEN).italic();
    assert!(!span.bold);
    assert!(span.italic);
}

#[test]
fn test_highlight_span_bold_and_italic() {
    let span = HighlightSpan::new(0, 5, Color::YELLOW).bold().italic();
    assert!(span.bold);
    assert!(span.italic);
}

// Language detection tests

#[test]
fn test_language_from_extension_rust() {
    assert_eq!(Language::from_extension("rs"), Language::Rust);
}

#[test]
fn test_language_from_extension_python() {
    assert_eq!(Language::from_extension("py"), Language::Python);
    assert_eq!(Language::from_extension("pyw"), Language::Python);
}

#[test]
fn test_language_from_extension_javascript() {
    assert_eq!(Language::from_extension("js"), Language::JavaScript);
    assert_eq!(Language::from_extension("jsx"), Language::JavaScript);
    assert_eq!(Language::from_extension("ts"), Language::JavaScript);
    assert_eq!(Language::from_extension("tsx"), Language::JavaScript);
    assert_eq!(Language::from_extension("mjs"), Language::JavaScript);
}

#[test]
fn test_language_from_extension_json() {
    assert_eq!(Language::from_extension("json"), Language::Json);
}

#[test]
fn test_language_from_extension_toml() {
    assert_eq!(Language::from_extension("toml"), Language::Toml);
}

#[test]
fn test_language_from_extension_yaml() {
    assert_eq!(Language::from_extension("yml"), Language::Yaml);
    assert_eq!(Language::from_extension("yaml"), Language::Yaml);
}

#[test]
fn test_language_from_extension_markdown() {
    assert_eq!(Language::from_extension("md"), Language::Markdown);
    assert_eq!(Language::from_extension("markdown"), Language::Markdown);
}

#[test]
fn test_language_from_extension_shell() {
    assert_eq!(Language::from_extension("sh"), Language::Shell);
    assert_eq!(Language::from_extension("bash"), Language::Shell);
    assert_eq!(Language::from_extension("zsh"), Language::Shell);
}

#[test]
fn test_language_from_extension_sql() {
    assert_eq!(Language::from_extension("sql"), Language::Sql);
}

#[test]
fn test_language_from_extension_html() {
    assert_eq!(Language::from_extension("html"), Language::Html);
    assert_eq!(Language::from_extension("htm"), Language::Html);
}

#[test]
fn test_language_from_extension_css() {
    assert_eq!(Language::from_extension("css"), Language::Css);
    assert_eq!(Language::from_extension("scss"), Language::Css);
    assert_eq!(Language::from_extension("sass"), Language::Css);
}

#[test]
fn test_language_from_extension_go() {
    assert_eq!(Language::from_extension("go"), Language::Go);
}

#[test]
fn test_language_from_extension_unknown() {
    assert_eq!(Language::from_extension("unknown"), Language::None);
    assert_eq!(Language::from_extension("xyz"), Language::None);
}

#[test]
fn test_language_from_extension_case_insensitive() {
    assert_eq!(Language::from_extension("RS"), Language::Rust);
    assert_eq!(Language::from_extension("PY"), Language::Python);
    assert_eq!(Language::from_extension("JSON"), Language::Json);
}

// SyntaxTheme tests

#[test]
fn test_syntax_theme_default() {
    let theme = SyntaxTheme::default();
    // Default should be dark theme
    let dark = SyntaxTheme::dark();
    assert_eq!(theme.keyword.r, dark.keyword.r);
    assert_eq!(theme.keyword.g, dark.keyword.g);
    assert_eq!(theme.keyword.b, dark.keyword.b);
}

#[test]
fn test_syntax_theme_dark() {
    let theme = SyntaxTheme::dark();
    assert_eq!(theme.keyword, Color::rgb(198, 120, 221));
    assert_eq!(theme.type_name, Color::rgb(229, 192, 123));
    assert_eq!(theme.function, Color::rgb(97, 175, 239));
    assert_eq!(theme.string, Color::rgb(152, 195, 121));
    assert_eq!(theme.number, Color::rgb(209, 154, 102));
    assert_eq!(theme.comment, Color::rgb(92, 99, 112));
    assert_eq!(theme.operator, Color::rgb(171, 178, 191));
    assert_eq!(theme.punctuation, Color::rgb(171, 178, 191));
    assert_eq!(theme.constant, Color::rgb(209, 154, 102));
    assert_eq!(theme.macro_call, Color::rgb(86, 182, 194));
    assert_eq!(theme.attribute, Color::rgb(229, 192, 123));
    assert_eq!(theme.variable, Color::rgb(224, 108, 117));
}

#[test]
fn test_syntax_theme_light() {
    let theme = SyntaxTheme::light();
    assert_eq!(theme.keyword, Color::rgb(166, 38, 164));
    assert_eq!(theme.type_name, Color::rgb(193, 132, 1));
    assert_eq!(theme.function, Color::rgb(64, 120, 242));
    assert_eq!(theme.string, Color::rgb(80, 161, 79));
    assert_eq!(theme.number, Color::rgb(152, 104, 1));
    assert_eq!(theme.comment, Color::rgb(160, 161, 167));
    assert_eq!(theme.operator, Color::rgb(56, 58, 66));
    assert_eq!(theme.punctuation, Color::rgb(56, 58, 66));
    assert_eq!(theme.constant, Color::rgb(152, 104, 1));
    assert_eq!(theme.macro_call, Color::rgb(1, 132, 188));
    assert_eq!(theme.attribute, Color::rgb(193, 132, 1));
    assert_eq!(theme.variable, Color::rgb(228, 86, 73));
}

#[test]
fn test_syntax_theme_monokai() {
    let theme = SyntaxTheme::monokai();
    assert_eq!(theme.keyword, Color::rgb(249, 38, 114));
    assert_eq!(theme.type_name, Color::rgb(102, 217, 239));
    assert_eq!(theme.function, Color::rgb(166, 226, 46));
    assert_eq!(theme.string, Color::rgb(230, 219, 116));
    assert_eq!(theme.number, Color::rgb(174, 129, 255));
    assert_eq!(theme.comment, Color::rgb(117, 113, 94));
    assert_eq!(theme.operator, Color::rgb(249, 38, 114));
    assert_eq!(theme.punctuation, Color::rgb(248, 248, 242));
    assert_eq!(theme.constant, Color::rgb(174, 129, 255));
    assert_eq!(theme.macro_call, Color::rgb(102, 217, 239));
    assert_eq!(theme.attribute, Color::rgb(166, 226, 46));
    assert_eq!(theme.variable, Color::rgb(248, 248, 242));
}

#[test]
fn test_syntax_theme_dark_vs_light() {
    let dark = SyntaxTheme::dark();
    let light = SyntaxTheme::light();
    // Themes should have different colors
    assert_ne!(dark.keyword, light.keyword);
    assert_ne!(dark.string, light.string);
    assert_ne!(dark.comment, light.comment);
}

// SyntaxHighlighter builder method tests

#[test]
fn test_syntax_highlighter_new() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    assert_eq!(hl.get_language(), Language::Rust);
}

#[test]
fn test_syntax_highlighter_default() {
    let hl = SyntaxHighlighter::default();
    assert_eq!(hl.get_language(), Language::None);
}

#[test]
fn test_syntax_highlighter_with_theme() {
    let custom_theme = SyntaxTheme::monokai();
    let hl = SyntaxHighlighter::with_theme(Language::Python, custom_theme.clone());
    assert_eq!(hl.get_language(), Language::Python);
}

#[test]
fn test_syntax_highlighter_theme_builder() {
    let hl = SyntaxHighlighter::new(Language::JavaScript).theme(SyntaxTheme::light());
    assert_eq!(hl.get_language(), Language::JavaScript);
}

#[test]
fn test_syntax_highlighter_language_builder() {
    let hl = SyntaxHighlighter::new(Language::None).language(Language::Go);
    assert_eq!(hl.get_language(), Language::Go);
}

#[test]
fn test_syntax_highlighter_get_language() {
    let hl = SyntaxHighlighter::new(Language::Sql);
    assert_eq!(hl.get_language(), Language::Sql);
}

// No highlighting tests

#[test]
fn test_no_highlighting_empty_line() {
    let hl = SyntaxHighlighter::new(Language::None);
    let spans = hl.highlight_line("");
    assert!(spans.is_empty());
}

#[test]
fn test_no_highlighting_with_code() {
    let hl = SyntaxHighlighter::new(Language::None);
    let spans = hl.highlight_line("fn main() { println!(\"Hello\"); }");
    assert!(spans.is_empty());
}

// Rust syntax highlighting tests

#[test]
fn test_rust_keyword_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("fn main() {");
    assert!(!spans.is_empty());
    // Check for keyword span (bold)
    let keyword_span = spans.iter().find(|s| s.bold);
    assert!(keyword_span.is_some());
}

#[test]
fn test_rust_keywords() {
    let hl = SyntaxHighlighter::new(Language::Rust);

    let keywords = vec![
        "fn", "let", "mut", "const", "static", "if", "else", "for", "while", "loop", "match",
        "struct", "enum", "impl", "trait", "type", "use", "mod", "crate", "pub", "async", "await",
        "return", "break", "continue",
    ];

    for keyword in keywords {
        let line = format!("{} x;", keyword);
        let spans = hl.highlight_line(&line);
        assert!(
            !spans.is_empty(),
            "Keyword '{}' should produce spans",
            keyword
        );
        assert!(
            spans.iter().any(|s| s.bold),
            "Keyword '{}' should be bold",
            keyword
        );
    }
}

#[test]
fn test_rust_type_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let x: String = \"hello\";");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_string_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let s = \"hello world\";");
    assert!(!spans.is_empty());
    // Check for string span
    let string_span = spans.iter().find(|s| {
        let line = "let s = \"hello world\";";
        &line[s.start..s.end] == "\"hello world\""
    });
    assert!(string_span.is_some());
}

#[test]
fn test_rust_string_with_escape() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let s = \"hello\\nworld\";");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_char_literal() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let c = 'a';");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_char_literal_escape() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let c = '\\n';");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_comment_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("// This is a comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
    assert_eq!(spans[0].start, 0);
}

#[test]
fn test_rust_code_with_comment() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let x = 5; // comment");
    assert!(!spans.is_empty());
    // Check for italic comment span
    assert!(spans.iter().any(|s| s.italic));
}

#[test]
fn test_rust_number_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let x = 42;");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_float_number() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let x = 3.14;");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_macro_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("println!(\"Hello\");");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_attribute_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("#[derive(Debug)]");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_constant_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let x = true;");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_none_some() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("let x = None;");
    assert!(!spans.is_empty());
}

#[test]
fn test_rust_empty_line() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("");
    assert!(spans.is_empty());
}

#[test]
fn test_rust_whitespace_only() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("   \t\t   ");
    assert!(spans.is_empty());
}

// Python syntax highlighting tests

#[test]
fn test_python_keyword_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("def main():");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_python_keywords() {
    let hl = SyntaxHighlighter::new(Language::Python);

    let keywords = vec![
        "def", "class", "if", "else", "elif", "for", "while", "return", "import", "from", "as",
        "try", "except", "finally", "with", "lambda",
    ];

    for keyword in keywords {
        let line = format!("{} x", keyword);
        let spans = hl.highlight_line(&line);
        assert!(
            !spans.is_empty(),
            "Keyword '{}' should produce spans",
            keyword
        );
    }
}

#[test]
fn test_python_comment_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("# This is a comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
}

#[test]
fn test_python_string_double_quote() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("s = \"hello\"");
    assert!(!spans.is_empty());
}

#[test]
fn test_python_string_single_quote() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("s = 'hello'");
    assert!(!spans.is_empty());
}

#[test]
fn test_python_function_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("print(\"hello\")");
    assert!(!spans.is_empty());
}

#[test]
fn test_python_decorator() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("@staticmethod");
    assert!(!spans.is_empty());
}

#[test]
fn test_python_true_false_none() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("x = True");
    assert!(!spans.is_empty());
}

#[test]
fn test_python_number() {
    let hl = SyntaxHighlighter::new(Language::Python);
    let spans = hl.highlight_line("x = 42");
    assert!(!spans.is_empty());
}

// JavaScript syntax highlighting tests

#[test]
fn test_javascript_keyword_highlighting() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);
    let spans = hl.highlight_line("function test() {");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_javascript_keywords() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);

    let keywords = vec![
        "const", "let", "var", "function", "if", "else", "for", "while", "return", "class",
        "extends", "import", "export", "async", "await",
    ];

    for keyword in keywords {
        let line = format!("{} x", keyword);
        let spans = hl.highlight_line(&line);
        assert!(
            !spans.is_empty(),
            "Keyword '{}' should produce spans",
            keyword
        );
    }
}

#[test]
fn test_javascript_comment() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);
    let spans = hl.highlight_line("// comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
}

#[test]
fn test_javascript_string_double_quote() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);
    let spans = hl.highlight_line("const s = \"hello\";");
    assert!(!spans.is_empty());
}

#[test]
fn test_javascript_string_single_quote() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);
    let spans = hl.highlight_line("const s = 'hello';");
    assert!(!spans.is_empty());
}

#[test]
fn test_javascript_template_literal() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);
    let spans = hl.highlight_line("const s = `hello`;");
    assert!(!spans.is_empty());
}

#[test]
fn test_javascript_function() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);
    let spans = hl.highlight_line("console.log(\"test\");");
    assert!(!spans.is_empty());
}

#[test]
fn test_javascript_true_false_null_undefined() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);
    let spans = hl.highlight_line("const x = true;");
    assert!(!spans.is_empty());
}

#[test]
fn test_javascript_dollar_identifier() {
    let hl = SyntaxHighlighter::new(Language::JavaScript);
    let spans = hl.highlight_line("const $elem = document;");
    assert!(!spans.is_empty());
}

// JSON syntax highlighting tests

#[test]
fn test_json_key_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"key\": \"value\"}");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_string_value() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"name\": \"John\"}");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_number() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"count\": 42}");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_negative_number() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"value\": -10}");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_float() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"price\": 19.99}");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_boolean() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"active\": true}");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_null() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"value\": null}");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_nested_object() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"user\": {\"name\": \"John\"}}");
    assert!(!spans.is_empty());
}

#[test]
fn test_json_array() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"items\": [1, 2, 3]}");
    assert!(!spans.is_empty());
}

// TOML syntax highlighting tests

#[test]
fn test_toml_section_header() {
    let hl = SyntaxHighlighter::new(Language::Toml);
    let spans = hl.highlight_line("[section]");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_toml_key_value() {
    let hl = SyntaxHighlighter::new(Language::Toml);
    let spans = hl.highlight_line("name = \"value\"");
    assert!(!spans.is_empty());
}

#[test]
fn test_toml_comment() {
    let hl = SyntaxHighlighter::new(Language::Toml);
    let spans = hl.highlight_line("# comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
}

#[test]
fn test_toml_string() {
    let hl = SyntaxHighlighter::new(Language::Toml);
    let spans = hl.highlight_line("name = 'Tom'");
    assert!(!spans.is_empty());
}

#[test]
fn test_toml_number() {
    let hl = SyntaxHighlighter::new(Language::Toml);
    let spans = hl.highlight_line("port = 8080");
    assert!(!spans.is_empty());
}

#[test]
fn test_toml_boolean() {
    let hl = SyntaxHighlighter::new(Language::Toml);
    let spans = hl.highlight_line("debug = true");
    assert!(!spans.is_empty());
}

// YAML syntax highlighting tests

#[test]
fn test_yaml_key_value() {
    let hl = SyntaxHighlighter::new(Language::Yaml);
    let spans = hl.highlight_line("name: value");
    assert!(!spans.is_empty());
}

#[test]
fn test_yaml_comment() {
    let hl = SyntaxHighlighter::new(Language::Yaml);
    let spans = hl.highlight_line("# comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
}

#[test]
fn test_yaml_string() {
    let hl = SyntaxHighlighter::new(Language::Yaml);
    let spans = hl.highlight_line("name: \"value\"");
    assert!(!spans.is_empty());
}

#[test]
fn test_yaml_number() {
    let hl = SyntaxHighlighter::new(Language::Yaml);
    let spans = hl.highlight_line("port: 8080");
    assert!(!spans.is_empty());
}

#[test]
fn test_yaml_boolean() {
    let hl = SyntaxHighlighter::new(Language::Yaml);
    let spans = hl.highlight_line("enabled: true");
    assert!(!spans.is_empty());
}

#[test]
fn test_yaml_null() {
    let hl = SyntaxHighlighter::new(Language::Yaml);
    let spans = hl.highlight_line("value: null");
    assert!(!spans.is_empty());
}

#[test]
fn test_yaml_yes_no() {
    let hl = SyntaxHighlighter::new(Language::Yaml);
    let spans = hl.highlight_line("enabled: yes");
    assert!(!spans.is_empty());
}

// Markdown syntax highlighting tests

#[test]
fn test_markdown_header() {
    let hl = SyntaxHighlighter::new(Language::Markdown);
    let spans = hl.highlight_line("# Header");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_markdown_header_multiple_levels() {
    let hl = SyntaxHighlighter::new(Language::Markdown);

    let levels = vec![
        "# H1",
        "## H2",
        "### H3",
        "#### H4",
        "##### H5",
        "###### H6",
    ];
    for header in levels {
        let spans = hl.highlight_line(header);
        assert!(
            !spans.is_empty(),
            "Header '{}' should produce spans",
            header
        );
        assert!(
            spans.iter().any(|s| s.bold),
            "Header '{}' should be bold",
            header
        );
    }
}

#[test]
fn test_markdown_bold() {
    let hl = SyntaxHighlighter::new(Language::Markdown);
    let spans = hl.highlight_line("**bold text**");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_markdown_bold_underscore() {
    let hl = SyntaxHighlighter::new(Language::Markdown);
    let spans = hl.highlight_line("__bold text__");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_markdown_italic() {
    let hl = SyntaxHighlighter::new(Language::Markdown);
    let spans = hl.highlight_line("*italic text*");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.italic));
}

#[test]
fn test_markdown_italic_underscore() {
    let hl = SyntaxHighlighter::new(Language::Markdown);
    let spans = hl.highlight_line("_italic text_");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.italic));
}

#[test]
fn test_markdown_inline_code() {
    let hl = SyntaxHighlighter::new(Language::Markdown);
    let spans = hl.highlight_line("`code`");
    assert!(!spans.is_empty());
}

#[test]
fn test_markdown_link() {
    let hl = SyntaxHighlighter::new(Language::Markdown);
    let spans = hl.highlight_line("[text](url)");
    assert!(!spans.is_empty());
}

#[test]
fn test_markdown_plain_text() {
    let hl = SyntaxHighlighter::new(Language::Markdown);
    let spans = hl.highlight_line("Just plain text");
    // Plain text without markdown syntax should have no spans
    assert!(spans.is_empty());
}

// Shell syntax highlighting tests

#[test]
fn test_shell_comment() {
    let hl = SyntaxHighlighter::new(Language::Shell);
    let spans = hl.highlight_line("# comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
}

#[test]
fn test_shell_string_double_quote() {
    let hl = SyntaxHighlighter::new(Language::Shell);
    let spans = hl.highlight_line("echo \"hello\"");
    assert!(!spans.is_empty());
}

#[test]
fn test_shell_string_single_quote() {
    let hl = SyntaxHighlighter::new(Language::Shell);
    let spans = hl.highlight_line("echo 'hello'");
    assert!(!spans.is_empty());
}

#[test]
fn test_shell_variable() {
    let hl = SyntaxHighlighter::new(Language::Shell);
    let spans = hl.highlight_line("echo $HOME");
    assert!(!spans.is_empty());
}

#[test]
fn test_shell_variable_braced() {
    let hl = SyntaxHighlighter::new(Language::Shell);
    let spans = hl.highlight_line("echo ${PATH}");
    assert!(!spans.is_empty());
}

#[test]
fn test_shell_keyword() {
    let hl = SyntaxHighlighter::new(Language::Shell);
    let spans = hl.highlight_line("if true; then");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_shell_command() {
    let hl = SyntaxHighlighter::new(Language::Shell);
    let spans = hl.highlight_line("echo hello");
    // `echo` is a shell keyword, so it should be highlighted
    assert!(!spans.is_empty());
}

// SQL syntax highlighting tests

#[test]
fn test_sql_keyword_uppercase() {
    let hl = SyntaxHighlighter::new(Language::Sql);
    let spans = hl.highlight_line("SELECT * FROM users");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_sql_keyword_lowercase() {
    let hl = SyntaxHighlighter::new(Language::Sql);
    let spans = hl.highlight_line("select * from users");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_sql_comment() {
    let hl = SyntaxHighlighter::new(Language::Sql);
    let spans = hl.highlight_line("-- comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
}

#[test]
fn test_sql_string() {
    let hl = SyntaxHighlighter::new(Language::Sql);
    let spans = hl.highlight_line("SELECT 'hello'");
    assert!(!spans.is_empty());
}

#[test]
fn test_sql_number() {
    let hl = SyntaxHighlighter::new(Language::Sql);
    let spans = hl.highlight_line("SELECT 1, 2.5");
    assert!(!spans.is_empty());
}

#[test]
fn test_sql_null() {
    let hl = SyntaxHighlighter::new(Language::Sql);
    let spans = hl.highlight_line("WHERE value IS NULL");
    assert!(!spans.is_empty());
}

#[test]
fn test_sql_common_keywords() {
    let hl = SyntaxHighlighter::new(Language::Sql);

    let keywords = vec![
        "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "JOIN", "LEFT", "RIGHT", "INNER",
        "OUTER", "ORDER", "BY", "GROUP", "HAVING", "LIMIT", "AND", "OR", "NOT",
    ];

    for keyword in keywords {
        let line = format!("{} *", keyword);
        let spans = hl.highlight_line(&line);
        assert!(
            !spans.is_empty(),
            "Keyword '{}' should produce spans",
            keyword
        );
    }
}

// HTML syntax highlighting tests

#[test]
fn test_html_tag() {
    let hl = SyntaxHighlighter::new(Language::Html);
    let spans = hl.highlight_line("<div>");
    assert!(!spans.is_empty());
}

#[test]
fn test_html_tag_with_attributes() {
    let hl = SyntaxHighlighter::new(Language::Html);
    let spans = hl.highlight_line("<div class=\"container\">");
    assert!(!spans.is_empty());
}

#[test]
fn test_html_closing_tag() {
    let hl = SyntaxHighlighter::new(Language::Html);
    let spans = hl.highlight_line("</div>");
    assert!(!spans.is_empty());
}

#[test]
fn test_html_self_closing_tag() {
    let hl = SyntaxHighlighter::new(Language::Html);
    let spans = hl.highlight_line("<img src=\"test.jpg\" />");
    assert!(!spans.is_empty());
}

#[test]
fn test_html_multiple_attributes() {
    let hl = SyntaxHighlighter::new(Language::Html);
    let spans = hl.highlight_line("<input type=\"text\" class=\"form-control\" />");
    assert!(!spans.is_empty());
}

#[test]
fn test_html_plain_text() {
    let hl = SyntaxHighlighter::new(Language::Html);
    let spans = hl.highlight_line("Just text without tags");
    assert!(spans.is_empty());
}

// CSS syntax highlighting tests

#[test]
fn test_css_class_selector() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line(".container {");
    assert!(!spans.is_empty());
}

#[test]
fn test_css_id_selector() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line("#header {");
    assert!(!spans.is_empty());
}

#[test]
fn test_css_element_selector() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line("div {");
    assert!(!spans.is_empty());
}

#[test]
fn test_css_property() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line("color: red;");
    assert!(!spans.is_empty());
}

#[test]
fn test_css_string_value() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line("content: \"hello\";");
    assert!(!spans.is_empty());
}

#[test]
fn test_css_number_with_unit() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line("width: 100px;");
    assert!(!spans.is_empty());
}

#[test]
fn test_css_color_hex() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line("color: #FF0000;");
    assert!(!spans.is_empty());
}

#[test]
fn test_css_comment() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line("/* comment */");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.italic));
}

#[test]
fn test_css_percentage() {
    let hl = SyntaxHighlighter::new(Language::Css);
    let spans = hl.highlight_line("width: 100%;");
    assert!(!spans.is_empty());
}

// Go syntax highlighting tests

#[test]
fn test_go_keyword_highlighting() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("func main() {");
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|s| s.bold));
}

#[test]
fn test_go_keywords() {
    let hl = SyntaxHighlighter::new(Language::Go);

    let keywords = vec![
        "func",
        "var",
        "const",
        "type",
        "struct",
        "interface",
        "if",
        "else",
        "for",
        "return",
        "go",
        "defer",
        "select",
        "chan",
    ];

    for keyword in keywords {
        let line = format!("{} x", keyword);
        let spans = hl.highlight_line(&line);
        assert!(
            !spans.is_empty(),
            "Keyword '{}' should produce spans",
            keyword
        );
        assert!(
            spans.iter().any(|s| s.bold),
            "Keyword '{}' should be bold",
            keyword
        );
    }
}

#[test]
fn test_go_comment() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("// comment");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].italic);
}

#[test]
fn test_go_string_double_quote() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("s := \"hello\"");
    assert!(!spans.is_empty());
}

#[test]
fn test_go_raw_string() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("s := `hello`");
    assert!(!spans.is_empty());
}

#[test]
fn test_go_function() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("fmt.Println(\"hello\")");
    assert!(!spans.is_empty());
}

#[test]
fn test_go_type_uppercase() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("var s String");
    assert!(!spans.is_empty());
}

#[test]
fn test_go_true_false_nil() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("b := true");
    assert!(!spans.is_empty());
}

#[test]
fn test_go_number() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("x := 42");
    assert!(!spans.is_empty());
}

#[test]
fn test_go_float() {
    let hl = SyntaxHighlighter::new(Language::Go);
    let spans = hl.highlight_line("x := 3.14");
    assert!(!spans.is_empty());
}

// Edge cases and complex scenarios

#[test]
fn test_multiple_same_line_constructs_rust() {
    let hl = SyntaxHighlighter::new(Language::Rust);
    let spans = hl.highlight_line("fn test() -> String { \"hello\" }");
    assert!(!spans.is_empty());
}

#[test]
fn test_complex_json_line() {
    let hl = SyntaxHighlighter::new(Language::Json);
    let spans = hl.highlight_line("{\"name\": \"John\", \"age\": 30, \"active\": true}");
    assert!(!spans.is_empty());
}

#[test]
fn test_empty_lines_all_languages() {
    let languages = vec![
        Language::Rust,
        Language::Python,
        Language::JavaScript,
        Language::Json,
        Language::Toml,
        Language::Yaml,
        Language::Markdown,
        Language::Shell,
        Language::Sql,
        Language::Html,
        Language::Css,
        Language::Go,
    ];

    for lang in languages {
        let hl = SyntaxHighlighter::new(lang);
        let spans = hl.highlight_line("");
        assert!(
            spans.is_empty(),
            "Empty line should have no spans for {:?}",
            lang
        );
    }
}

#[test]
fn test_whitespace_only_lines_all_languages() {
    let languages = vec![
        Language::Rust,
        Language::Python,
        Language::JavaScript,
        Language::Json,
        Language::Toml,
        Language::Yaml,
        Language::Shell,
        Language::Sql,
        Language::Html,
        Language::Css,
        Language::Go,
    ];

    for lang in languages {
        let hl = SyntaxHighlighter::new(lang);
        let spans = hl.highlight_line("   \t  ");
        assert!(
            spans.is_empty(),
            "Whitespace-only line should have no spans for {:?}",
            lang
        );
    }
}

#[test]
fn test_highlighter_chaining_builder_methods() {
    let hl = SyntaxHighlighter::new(Language::Rust)
        .language(Language::Python)
        .theme(SyntaxTheme::light());

    assert_eq!(hl.get_language(), Language::Python);
}

#[test]
fn test_all_languages_are_distinct() {
    let languages = vec![
        Language::None,
        Language::Rust,
        Language::Python,
        Language::JavaScript,
        Language::Json,
        Language::Toml,
        Language::Yaml,
        Language::Markdown,
        Language::Shell,
        Language::Sql,
        Language::Html,
        Language::Css,
        Language::Go,
    ];

    // Ensure all language variants can be created and are distinct
    for i in 0..languages.len() {
        for j in (i + 1)..languages.len() {
            assert_ne!(
                languages[i], languages[j],
                "Languages at indices {} and {:?} should be different",
                i, j
            );
        }
    }
}
