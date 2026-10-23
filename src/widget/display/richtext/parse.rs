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
    /// - `[reverse]`, `[rev]` - Reverse video
    /// - `[red]`, `[#ff8800]`, `[orange]` - Text color: a CSS color name or
    ///   hex code
    /// - `[on blue]`, `[on_blue]`, `[on #003366]` - Background color
    /// - `[link=URL]` - Hyperlink
    /// - `[/bold]`, `[/b]`, `[/red]`, `[/on blue]`, `[/link]` - Turn off what
    ///   the most recent open tag with it turned on; the other tags stay
    /// - `[/]` - Reset to default
    ///
    /// Tags can be combined: `[bold red]text[/]`, and closed a part at a
    /// time: `[bold red]a[/red]b[/bold]`. A closing tag with nothing open to
    /// close does nothing.
    ///
    /// Brackets that are not a tag are shown as they are: `"[x] done"` shows
    /// `[x] done`, and so does `[orange-ish]` (every word in a tag has to be
    /// one of the above). `[[` is a literal `[`: `"[[bold]"` shows `[bold]`,
    /// and a `[` with no `]` after it is shown as it is. Use
    /// [`escape`](Self::escape) on text that must not be read as markup,
    /// such as user input.
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
    /// let name = "[bold] notes.txt";
    /// assert_eq!(RichText::escape(name), "[[bold] notes.txt");
    /// // Shows "[bold] notes.txt" in bold; without `escape`, `[bold]` would be read as a tag.
    /// let rich = RichText::markup(&format!("[bold]{}[/]", RichText::escape(name)));
    /// ```
    pub fn escape(text: &str) -> String {
        text.replace('[', "[[")
    }

    /// Parse markup string
    fn parse_markup(&mut self, text: &str) {
        // The open tags, oldest first; the style and link in effect are
        // what they turn on, in order
        let mut open: Vec<Vec<Attr>> = Vec::new();
        let mut buffer = String::new();
        let mut rest = text;

        while let Some(i) = rest.find('[') {
            buffer.push_str(&rest[..i]);
            let after = &rest[i + 1..];
            // `[[` is a literal `[`.
            if let Some(after) = after.strip_prefix('[') {
                buffer.push('[');
                rest = after;
                continue;
            }
            // A tag runs to the next `]`, with no `[` in it; otherwise this
            // `[` is text
            let Some(end) = after
                .find(['[', ']'])
                .filter(|&e| after[e..].starts_with(']'))
            else {
                buffer.push('[');
                rest = after;
                continue;
            };
            let tag = &after[..end];
            rest = &after[end + 1..];
            let Some(tag) = parse_tag(tag) else {
                // Not a tag: the brackets and all are text
                buffer.push('[');
                buffer.push_str(tag);
                buffer.push(']');
                continue;
            };
            let (style, link) = style_of(&open);
            self.push_run(std::mem::take(&mut buffer), &style, &link);
            match tag {
                Tag::Reset => open.clear(),
                Tag::Open(attrs) => open.push(attrs),
                Tag::Close(attrs) => close(&mut open, &attrs),
            }
        }
        buffer.push_str(rest);

        // Flush remaining buffer
        let (style, link) = style_of(&open);
        self.push_run(buffer, &style, &link);
    }

    /// Add `text` as a span with the style and link in effect, unless it is
    /// empty
    fn push_run(&mut self, text: String, style: &Style, link: &Option<String>) {
        if !text.is_empty() {
            let mut span = Span::styled(text, style.clone());
            if let Some(url) = link {
                span.link = Some(url.clone());
            }
            self.spans.push(span);
        }
    }
}

/// One thing a tag turns on
#[derive(Clone, Debug, PartialEq)]
enum Attr {
    Bold,
    Italic,
    Underline,
    Dim,
    Strike,
    Reverse,
    Fg(Color),
    Bg(Color),
    Link(String),
}

impl Attr {
    /// Turn this on in `style` and `link`
    fn apply(&self, style: &mut Style, link: &mut Option<String>) {
        match self {
            Attr::Bold => style.bold = true,
            Attr::Italic => style.italic = true,
            Attr::Underline => style.underline = true,
            Attr::Dim => style.dim = true,
            Attr::Strike => style.strikethrough = true,
            Attr::Reverse => style.reverse = true,
            Attr::Fg(color) => style.fg = Some(*color),
            Attr::Bg(color) => style.bg = Some(*color),
            // A link is underlined, and cyan unless a color is already on
            Attr::Link(url) => {
                *link = Some(url.clone());
                style.underline = true;
                style.fg.get_or_insert(Color::CYAN);
            }
        }
    }

    /// Whether `[/self]` closes `open`: the same attribute and color; any
    /// link
    fn closes(&self, open: &Attr) -> bool {
        match (self, open) {
            (Attr::Link(_), Attr::Link(_)) => true,
            _ => self == open,
        }
    }
}

/// What the text between `[` and `]` does
enum Tag {
    /// `[/]`
    Reset,
    Open(Vec<Attr>),
    Close(Vec<Attr>),
}

/// The tag `text` is, or `None` when it is not one: empty, or with a word
/// that is not an attribute
fn parse_tag(text: &str) -> Option<Tag> {
    match text.strip_prefix('/') {
        Some(rest) if rest.trim().is_empty() => Some(Tag::Reset),
        Some(rest) => parse_attrs(rest, true).map(Tag::Close),
        None => parse_attrs(text, false).map(Tag::Open),
    }
}

/// The attributes the words of a tag name, or `None` if any word is not
/// one (or there are none). A closing tag names a link as `link`.
fn parse_attrs(words: &str, closing: bool) -> Option<Vec<Attr>> {
    let mut attrs = Vec::new();
    let mut words = words.split_whitespace();
    while let Some(word) = words.next() {
        let attr = if let Some(url) = word.strip_prefix("link=") {
            Attr::Link(url.to_string())
        } else {
            match word.to_lowercase().as_str() {
                "link" if closing => Attr::Link(String::new()),
                "bold" | "b" => Attr::Bold,
                "italic" | "i" => Attr::Italic,
                "underline" | "u" => Attr::Underline,
                "dim" => Attr::Dim,
                "strike" | "s" => Attr::Strike,
                "reverse" | "rev" => Attr::Reverse,
                "on" => Attr::Bg(color(words.next()?)?),
                name => match name.strip_prefix("on_") {
                    Some(bg) => Attr::Bg(color(bg)?),
                    None => Attr::Fg(color(name)?),
                },
            }
        };
        attrs.push(attr);
    }
    (!attrs.is_empty()).then_some(attrs)
}

/// A color word: a CSS color name or hex code
fn color(word: &str) -> Option<Color> {
    // Only a name or `#hex`: CSS functions such as `rgb(..)` are not words
    // of a tag
    if word.contains('(') {
        return None;
    }
    crate::style::parse_color(word)
}

/// Turn off each attribute in `attrs` in the most recent open tag that
/// turned it on; a tag with nothing left on is closed
fn close(open: &mut Vec<Vec<Attr>>, attrs: &[Attr]) {
    for attr in attrs {
        let found = open.iter_mut().rev().find_map(|tag| {
            let at = tag.iter().position(|on| attr.closes(on))?;
            tag.remove(at);
            Some(())
        });
        if found.is_some() {
            open.retain(|tag| !tag.is_empty());
        }
    }
}

/// The style and link the open tags turn on, oldest first
fn style_of(open: &[Vec<Attr>]) -> (Style, Option<String>) {
    let mut style = Style::default();
    let mut link = None;
    for attr in open.iter().flatten() {
        attr.apply(&mut style, &mut link);
    }
    (style, link)
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
