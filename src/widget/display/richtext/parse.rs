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
    ///
    /// `[[` is a literal `[`: `"[[x] done"` shows `[x] done`. A `[` with no
    /// `]` after it is shown as it is. Use [`escape`](Self::escape) on text
    /// that must not be read as markup, such as user input.
    pub fn markup(text: &str) -> Self {
        let mut rich = Self::new();
        rich.parse_markup(text);
        rich
    }

    /// Escape `text` so that [`markup`](Self::markup) shows it as it is.
    ///
    /// Every `[` becomes `[[`. Use it for text from outside the program -
    /// a file name, a message - placed inside markup, so a `[` in it cannot
    /// open a tag.
    ///
    /// ```
    /// use revue::widget::RichText;
    ///
    /// let name = "[draft] notes.txt";
    /// assert_eq!(RichText::escape(name), "[[draft] notes.txt");
    /// // Shows "[draft] notes.txt" in bold; without `escape`, `[draft]` would be read as a tag.
    /// let rich = RichText::markup(&format!("[bold]{}[/]", RichText::escape(name)));
    /// ```
    pub fn escape(text: &str) -> String {
        text.replace('[', "[[")
    }

    /// Parse markup string
    fn parse_markup(&mut self, text: &str) {
        let mut current_style = Style::default();
        let mut current_link: Option<String> = None;
        let mut buffer = String::new();
        let mut chars = text.char_indices();
        let last_close = text.rfind(']');

        while let Some((i, ch)) = chars.next() {
            // `[[` is a literal `[`.
            if ch == '[' && text[i + 1..].starts_with('[') {
                chars.next();
                buffer.push('[');
                continue;
            }
            // A `[` that is never closed is plain text, as is everything after
            // it (no later `[` can be closed either).
            if ch == '[' && last_close.is_none_or(|close| close < i) {
                buffer.push_str(&text[i..]);
                break;
            }
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
                for (_, c) in chars.by_ref() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::{Buffer, Modifier};
    use crate::widget::traits::{RenderContext, View};

    fn render(markup: &str) -> Buffer {
        let mut buf = Buffer::new(20, 1);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, 20, 1));
        RichText::markup(markup).render(&mut ctx);
        buf
    }

    fn text(buf: &Buffer) -> String {
        (0..20)
            .map(|x| buf.get(x, 0).unwrap().symbol)
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    #[test]
    fn a_doubled_bracket_is_a_literal_bracket() {
        assert_eq!(text(&render("[[x] done")), "[x] done");
        assert_eq!(text(&render("a[[b")), "a[b");
        assert_eq!(text(&render("[[[bold]x[/]")), "[x");
        let buf = render("[[[bold]x");
        assert!(buf.get(1, 0).unwrap().modifier.contains(Modifier::BOLD));
        assert!(!buf.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
        // An escaped tag is text, not a tag.
        assert_eq!(text(&render("[[bold]")), "[bold]");
    }

    #[test]
    fn escape_makes_any_text_render_as_itself() {
        for raw in [
            "[x] done",
            "a[b",
            "[bold]",
            "[[",
            "]",
            "[/]",
            "plain",
            "[link=u]x[/]",
        ] {
            let escaped = RichText::escape(raw);
            assert_eq!(
                text(&render(&escaped)),
                raw,
                "escape({raw:?}) = {escaped:?}"
            );
        }
    }

    #[test]
    fn unclosed_bracket_is_literal_text() {
        assert_eq!(text(&render("a [b c")), "a [b c");
        // Tags before it still apply, and the bracket keeps their style.
        let buf = render("[bold]x [y");
        assert_eq!(text(&buf), "x [y");
        assert!(buf.get(2, 0).unwrap().modifier.contains(Modifier::BOLD));
    }
}
