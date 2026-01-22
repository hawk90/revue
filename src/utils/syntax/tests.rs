//! Tests for syntax highlighting

use super::*;
use crate::style::Color;

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

#[test]
fn test_token_type_clone() {
    let tt = TokenType::Keyword;
    let cloned = tt.clone();
    assert_eq!(tt, cloned);
}

#[test]
fn test_token_type_eq() {
    assert_eq!(TokenType::Keyword, TokenType::Keyword);
    assert_ne!(TokenType::Keyword, TokenType::String);
}

#[test]
fn test_token_type_debug() {
    let tt = TokenType::Function;
    let debug = format!("{:?}", tt);
    assert!(debug.contains("Function"));
}

#[test]
fn test_token_type_copy() {
    let tt = TokenType::Number;
    let copied = tt; // Copy, not move
    assert_eq!(tt, copied);
}

#[test]
fn test_token_new() {
    let token = Token::new("hello", TokenType::String);
    assert_eq!(token.text, "hello");
    assert_eq!(token.token_type, TokenType::String);
}

#[test]
fn test_token_clone() {
    let token = Token::new("test", TokenType::Keyword);
    let cloned = token.clone();
    assert_eq!(cloned.text, "test");
    assert_eq!(cloned.token_type, TokenType::Keyword);
}

#[test]
fn test_token_debug() {
    let token = Token::new("x", TokenType::Normal);
    let debug = format!("{:?}", token);
    assert!(debug.contains("x"));
    assert!(debug.contains("Normal"));
}

#[test]
fn test_syntax_theme_default() {
    let theme = SyntaxTheme::default();
    // Default is monokai
    assert_eq!(theme.keyword, Color::rgb(249, 38, 114));
}

#[test]
fn test_syntax_theme_color_all_types() {
    let theme = SyntaxTheme::monokai();

    assert_eq!(theme.color(TokenType::Normal), theme.normal);
    assert_eq!(theme.color(TokenType::Keyword), theme.keyword);
    assert_eq!(theme.color(TokenType::Type), theme.type_);
    assert_eq!(theme.color(TokenType::String), theme.string);
    assert_eq!(theme.color(TokenType::Number), theme.number);
    assert_eq!(theme.color(TokenType::Comment), theme.comment);
    assert_eq!(theme.color(TokenType::Function), theme.function);
    assert_eq!(theme.color(TokenType::Operator), theme.operator);
    assert_eq!(theme.color(TokenType::Macro), theme.macro_);
    assert_eq!(theme.color(TokenType::Attribute), theme.attribute);
}

#[test]
fn test_syntax_theme_clone() {
    let theme = SyntaxTheme::nord();
    let cloned = theme.clone();
    assert_eq!(cloned.keyword, Color::rgb(129, 161, 193));
}

#[test]
fn test_language_from_fence_all() {
    // Test all supported language fence strings
    assert_eq!(Language::from_fence("rust"), Language::Rust);
    assert_eq!(Language::from_fence("rs"), Language::Rust);
    assert_eq!(Language::from_fence("python"), Language::Python);
    assert_eq!(Language::from_fence("py"), Language::Python);
    assert_eq!(Language::from_fence("javascript"), Language::JavaScript);
    assert_eq!(Language::from_fence("js"), Language::JavaScript);
    assert_eq!(Language::from_fence("typescript"), Language::TypeScript);
    assert_eq!(Language::from_fence("ts"), Language::TypeScript);
    assert_eq!(Language::from_fence("go"), Language::Go);
    assert_eq!(Language::from_fence("golang"), Language::Go);
    assert_eq!(Language::from_fence("c"), Language::C);
    assert_eq!(Language::from_fence("c++"), Language::Cpp);
    assert_eq!(Language::from_fence("cpp"), Language::Cpp);
    assert_eq!(Language::from_fence("cxx"), Language::Cpp);
    assert_eq!(Language::from_fence("java"), Language::Java);
    assert_eq!(Language::from_fence("ruby"), Language::Ruby);
    assert_eq!(Language::from_fence("rb"), Language::Ruby);
    assert_eq!(Language::from_fence("shell"), Language::Shell);
    assert_eq!(Language::from_fence("bash"), Language::Shell);
    assert_eq!(Language::from_fence("sh"), Language::Shell);
    assert_eq!(Language::from_fence("zsh"), Language::Shell);
    assert_eq!(Language::from_fence("json"), Language::Json);
    assert_eq!(Language::from_fence("yaml"), Language::Yaml);
    assert_eq!(Language::from_fence("yml"), Language::Yaml);
    assert_eq!(Language::from_fence("toml"), Language::Toml);
    assert_eq!(Language::from_fence("markdown"), Language::Markdown);
    assert_eq!(Language::from_fence("md"), Language::Markdown);
    assert_eq!(Language::from_fence("sql"), Language::Sql);
    assert_eq!(Language::from_fence("html"), Language::Html);
    assert_eq!(Language::from_fence("htm"), Language::Html);
    assert_eq!(Language::from_fence("css"), Language::Css);
}

#[test]
fn test_language_from_fence_case_insensitive() {
    assert_eq!(Language::from_fence("RUST"), Language::Rust);
    assert_eq!(Language::from_fence("Python"), Language::Python);
    assert_eq!(Language::from_fence("JavaScript"), Language::JavaScript);
}

#[test]
fn test_language_clone() {
    let lang = Language::Rust;
    let cloned = lang.clone();
    assert_eq!(lang, cloned);
}

#[test]
fn test_language_copy() {
    let lang = Language::Python;
    let copied = lang; // Copy, not move
    assert_eq!(lang, copied);
}

#[test]
fn test_language_keywords() {
    // Rust keywords should include fn, let, etc.
    let keywords = Language::Rust.keywords();
    assert!(keywords.contains(&"fn"));
    assert!(keywords.contains(&"let"));
    assert!(keywords.contains(&"mut"));

    // Python keywords
    let keywords = Language::Python.keywords();
    assert!(keywords.contains(&"def"));
    assert!(keywords.contains(&"class"));
}

#[test]
fn test_language_types() {
    // Rust types should include i32, String, etc.
    let types = Language::Rust.types();
    assert!(types.contains(&"i32"));
    assert!(types.contains(&"String"));
    assert!(types.contains(&"Vec"));

    // Python types
    let types = Language::Python.types();
    assert!(types.contains(&"int"));
    assert!(types.contains(&"str"));
}

#[test]
fn test_language_unknown_has_no_keywords() {
    let keywords = Language::Unknown.keywords();
    assert!(keywords.is_empty());
}

#[test]
fn test_syntax_highlighter_default() {
    let highlighter = SyntaxHighlighter::default();
    assert_eq!(highlighter.theme().keyword, Color::rgb(249, 38, 114));
}

#[test]
fn test_syntax_highlighter_with_theme() {
    let highlighter = SyntaxHighlighter::with_theme(SyntaxTheme::nord());
    assert_eq!(highlighter.theme().keyword, Color::rgb(129, 161, 193));
}

#[test]
fn test_syntax_highlighter_set_theme() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_theme(SyntaxTheme::dracula());
    assert_eq!(highlighter.theme().keyword, Color::rgb(255, 121, 198));
}

#[test]
fn test_syntax_highlighter_token_color() {
    let highlighter = SyntaxHighlighter::new();
    let color = highlighter.token_color(TokenType::Keyword);
    assert_eq!(color, Color::rgb(249, 38, 114));
}

#[test]
fn test_highlight_hex_number() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("let x = 0xFF;", Language::Rust);

    let hex_token = tokens.iter().find(|t| t.text == "0xFF");
    assert!(hex_token.is_some());
    assert_eq!(hex_token.unwrap().token_type, TokenType::Number);
}

#[test]
fn test_highlight_float_number() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("let x = 3.14;", Language::Rust);

    let float_token = tokens.iter().find(|t| t.text.contains("3.14"));
    assert!(float_token.is_some());
    assert_eq!(float_token.unwrap().token_type, TokenType::Number);
}

#[test]
fn test_highlight_scientific_notation() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("let x = 1e10;", Language::Rust);

    let sci_token = tokens.iter().find(|t| t.text.contains("1e10"));
    assert!(sci_token.is_some());
    assert_eq!(sci_token.unwrap().token_type, TokenType::Number);
}

#[test]
fn test_highlight_escaped_string() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line(r#"let s = "hello \"world\"";"#, Language::Rust);

    // Should have a string token containing the escaped quotes
    let string_token = tokens.iter().find(|t| t.text.contains("hello"));
    assert!(string_token.is_some());
    assert_eq!(string_token.unwrap().token_type, TokenType::String);
}

#[test]
fn test_highlight_rust_type() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("let v: Vec<i32> = Vec::new();", Language::Rust);

    // Vec should be a Type
    let vec_token = tokens.iter().find(|t| t.text == "Vec");
    assert!(vec_token.is_some());
    assert_eq!(vec_token.unwrap().token_type, TokenType::Type);

    // i32 should be a Type
    let i32_token = tokens.iter().find(|t| t.text == "i32");
    assert!(i32_token.is_some());
    assert_eq!(i32_token.unwrap().token_type, TokenType::Type);
}

#[test]
fn test_highlight_rust_lifetime() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("fn foo<'a>(x: &'a str) {}", Language::Rust);

    // 'a should be a Type (lifetime)
    let lifetime_token = tokens.iter().find(|t| t.text.starts_with('\''));
    assert!(lifetime_token.is_some());
}

#[test]
fn test_highlight_python_decorator() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("@property", Language::Python);

    assert_eq!(tokens[0].text, "@property");
    assert_eq!(tokens[0].token_type, TokenType::Attribute);
}

#[test]
fn test_highlight_python_comment() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("x = 1 # comment", Language::Python);

    let comment_token = tokens.iter().find(|t| t.text.contains("comment"));
    assert!(comment_token.is_some());
    assert_eq!(comment_token.unwrap().token_type, TokenType::Comment);
}

#[test]
fn test_highlight_shell_comment() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("echo hello # comment", Language::Shell);

    let comment_token = tokens.iter().find(|t| t.text.contains("comment"));
    assert!(comment_token.is_some());
    assert_eq!(comment_token.unwrap().token_type, TokenType::Comment);
}

#[test]
fn test_highlight_operators() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("x + y * z", Language::Rust);

    let plus_token = tokens.iter().find(|t| t.text == "+");
    assert!(plus_token.is_some());
    assert_eq!(plus_token.unwrap().token_type, TokenType::Operator);
}

#[test]
fn test_highlight_single_quote_string() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("x = 'hello'", Language::Python);

    let string_token = tokens.iter().find(|t| t.text.contains("hello"));
    assert!(string_token.is_some());
    assert_eq!(string_token.unwrap().token_type, TokenType::String);
}

#[test]
fn test_highlight_go_code() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("func main() {", Language::Go);

    assert_eq!(tokens[0].text, "func");
    assert_eq!(tokens[0].token_type, TokenType::Keyword);
}

#[test]
fn test_highlight_java_code() {
    let highlighter = SyntaxHighlighter::new();
    let tokens =
        highlighter.highlight_line("public static void main(String[] args) {", Language::Java);

    let public_token = tokens.iter().find(|t| t.text == "public");
    assert!(public_token.is_some());
    assert_eq!(public_token.unwrap().token_type, TokenType::Keyword);

    let string_type = tokens.iter().find(|t| t.text == "String");
    assert!(string_type.is_some());
    assert_eq!(string_type.unwrap().token_type, TokenType::Type);
}

#[test]
fn test_highlight_ruby_code() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("def foo; end", Language::Ruby);

    let def_token = tokens.iter().find(|t| t.text == "def");
    assert!(def_token.is_some());
    assert_eq!(def_token.unwrap().token_type, TokenType::Keyword);

    let end_token = tokens.iter().find(|t| t.text == "end");
    assert!(end_token.is_some());
    assert_eq!(end_token.unwrap().token_type, TokenType::Keyword);
}

#[test]
fn test_highlight_sql_code() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("SELECT * FROM users WHERE id = 1", Language::Sql);

    let select_token = tokens.iter().find(|t| t.text == "SELECT");
    assert!(select_token.is_some());
    assert_eq!(select_token.unwrap().token_type, TokenType::Keyword);
}

#[test]
fn test_highlight_cpp_code() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("class Foo : public Bar {", Language::Cpp);

    let class_token = tokens.iter().find(|t| t.text == "class");
    assert!(class_token.is_some());
    assert_eq!(class_token.unwrap().token_type, TokenType::Keyword);
}

#[test]
fn test_highlight_empty_line() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("", Language::Rust);
    assert!(tokens.is_empty());
}

#[test]
fn test_highlight_whitespace_only() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("   ", Language::Rust);

    // Should have tokens for whitespace
    assert!(tokens.iter().all(|t| t.token_type == TokenType::Normal));
}

#[test]
fn test_highlight_rust_number_suffix() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("let x = 42u64;", Language::Rust);

    let num_token = tokens.iter().find(|t| t.text.contains("42"));
    assert!(num_token.is_some());
    assert_eq!(num_token.unwrap().token_type, TokenType::Number);
}

#[test]
fn test_highlight_multiline() {
    let highlighter = SyntaxHighlighter::new();
    let code = "fn foo() {\n    let x = 1;\n}";
    let lines = highlighter.highlight(code, Language::Rust);

    assert_eq!(lines.len(), 3);
    assert!(!lines[0].is_empty());
    assert!(!lines[1].is_empty());
    assert!(!lines[2].is_empty());
}

#[test]
fn test_highlight_nested_block_comment() {
    let highlighter = SyntaxHighlighter::new();
    let tokens = highlighter.highlight_line("/* outer /* inner */ */", Language::Rust);

    // The outer comment should start as a comment
    assert!(tokens.iter().any(|t| t.token_type == TokenType::Comment));
}

#[test]
fn test_language_comment_patterns() {
    // Rust uses // and /* */
    let (line, block) = Language::Rust.comment_patterns();
    assert_eq!(line, "//");
    assert_eq!(block, Some(("/*", "*/")));

    // Python uses # only
    let (line, block) = Language::Python.comment_patterns();
    assert_eq!(line, "#");
    assert_eq!(block, None);

    // SQL uses -- and /* */
    let (line, block) = Language::Sql.comment_patterns();
    assert_eq!(line, "--");
    assert_eq!(block, Some(("/*", "*/")));

    // HTML uses <!-- --> only
    let (line, block) = Language::Html.comment_patterns();
    assert_eq!(line, "");
    assert_eq!(block, Some(("<!--", "-->")));
}
