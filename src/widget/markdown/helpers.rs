//! Markdown widget helper functions

use super::Markdown;

/// Create a new markdown widget
pub fn markdown(source: impl Into<String>) -> Markdown {
    Markdown::new(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_function() {
        let md = markdown("# Hello");
        let _ = md;
    }

    #[test]
    fn test_markdown_function_with_string() {
        let md = markdown("**bold** text".to_string());
        let _ = md;
    }

    #[test]
    fn test_markdown_function_with_str() {
        let md = markdown("*italic* text");
        let _ = md;
    }
}
