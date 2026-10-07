//! Building rich text from markup tags

use super::{RichText, Span, Style};
use crate::style::Color;

impl RichText {
    /// Create from markup string
    ///
    /// Supported tags:
    /// - `[bold]`, `[b]` - Bold text
    /// - `[italic]`, `[i]` - Italic text
    /// - `[underline]`, `[u]` - Underlined text
    /// - `[dim]` - Dimmed text
    /// - `[strike]`, `[s]` - Strikethrough
    /// - `[red]`, `[green]`, `[blue]`, `[yellow]`, `[cyan]`, `[magenta]`, `[white]` - Colors
    /// - `[link=URL]` - Hyperlink
    /// - `[/]` - Reset to default
    ///
    /// Tags can be combined: `[bold red]text[/]`
    pub fn markup(text: &str) -> Self {
        let mut rich = Self::new();
        rich.parse_markup(text);
        rich
    }

    /// Parse markup string
    fn parse_markup(&mut self, text: &str) {
        let mut current_style = Style::default();
        let mut current_link: Option<String> = None;
        let mut buffer = String::new();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '[' {
                // Flush buffer with current style
                if !buffer.is_empty() {
                    let mut span = Span::styled(buffer.clone(), current_style.clone());
                    if let Some(ref url) = current_link {
                        span.link = Some(url.clone());
                    }
                    self.spans.push(span);
                    buffer.clear();
                }

                // Parse tag
                let mut tag = String::new();
                for c in chars.by_ref() {
                    if c == ']' {
                        break;
                    }
                    tag.push(c);
                }

                // Handle reset tag
                if tag == "/" {
                    current_style = Style::default();
                    current_link = None;
                    continue;
                }

                // Parse tag attributes
                for part in tag.split_whitespace() {
                    if let Some(link) = part.strip_prefix("link=") {
                        current_link = Some(link.to_string());
                        current_style.underline = true;
                        if current_style.fg.is_none() {
                            current_style.fg = Some(Color::CYAN);
                        }
                    } else {
                        match part.to_lowercase().as_str() {
                            "bold" | "b" => current_style.bold = true,
                            "italic" | "i" => current_style.italic = true,
                            "underline" | "u" => current_style.underline = true,
                            "dim" => current_style.dim = true,
                            "strike" | "s" => current_style.strikethrough = true,
                            "reverse" | "rev" => current_style.reverse = true,
                            "red" => current_style.fg = Some(Color::RED),
                            "green" => current_style.fg = Some(Color::GREEN),
                            "blue" => current_style.fg = Some(Color::BLUE),
                            "yellow" => current_style.fg = Some(Color::YELLOW),
                            "cyan" => current_style.fg = Some(Color::CYAN),
                            "magenta" => current_style.fg = Some(Color::MAGENTA),
                            "white" => current_style.fg = Some(Color::WHITE),
                            "black" => current_style.fg = Some(Color::BLACK),
                            // Background colors with "on_" prefix
                            "on_red" => current_style.bg = Some(Color::RED),
                            "on_green" => current_style.bg = Some(Color::GREEN),
                            "on_blue" => current_style.bg = Some(Color::BLUE),
                            "on_yellow" => current_style.bg = Some(Color::YELLOW),
                            "on_cyan" => current_style.bg = Some(Color::CYAN),
                            "on_magenta" => current_style.bg = Some(Color::MAGENTA),
                            "on_white" => current_style.bg = Some(Color::WHITE),
                            "on_black" => current_style.bg = Some(Color::BLACK),
                            _ => {}
                        }
                    }
                }
            } else {
                buffer.push(ch);
            }
        }

        // Flush remaining buffer
        if !buffer.is_empty() {
            let mut span = Span::styled(buffer, current_style);
            if let Some(url) = current_link {
                span.link = Some(url);
            }
            self.spans.push(span);
        }
    }
}
