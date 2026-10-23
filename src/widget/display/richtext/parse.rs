//! Building rich text from markup tags

use std::collections::{HashMap, HashSet};

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
        let mut open = OpenTags::default();
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
            self.push_run(std::mem::take(&mut buffer), &open.style, &open.link);
            match tag {
                Tag::Reset => open = OpenTags::default(),
                Tag::Open(attrs) => open.open(attrs),
                Tag::Close(attrs) => open.close(&attrs),
            }
        }
        buffer.push_str(rest);

        // Flush remaining buffer
        self.push_run(buffer, &open.style, &open.link);
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
    /// What a closing tag matches this by: the attribute, the exact color
    /// for a color, and any link for a link
    fn key(&self) -> Key {
        let rgba = |c: &Color| u32::from_be_bytes([c.r, c.g, c.b, c.a]);
        match self {
            Attr::Bold => Key::Modifier(0),
            Attr::Italic => Key::Modifier(1),
            Attr::Underline => Key::Modifier(2),
            Attr::Dim => Key::Modifier(3),
            Attr::Strike => Key::Modifier(4),
            Attr::Reverse => Key::Modifier(5),
            Attr::Fg(c) => Key::Fg(rgba(c)),
            Attr::Bg(c) => Key::Bg(rgba(c)),
            Attr::Link(_) => Key::Link,
        }
    }
}

/// See [`Attr::key`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Key {
    Modifier(usize),
    Fg(u32),
    Bg(u32),
    Link,
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
        let attr = if word.len() >= 5
            && word.is_char_boundary(5)
            && word[..5].eq_ignore_ascii_case("link=")
        {
            // The word is case-insensitive, the URL is not
            Attr::Link(word[5..].to_string())
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
        // A word given twice counts once: `[bold bold]` is closed by one
        // `[/bold]`
        if !attrs.contains(&attr) {
            attrs.push(attr);
        }
    }
    (!attrs.is_empty()).then_some(attrs)
}

/// A color word: a CSS color name or hex code
fn color(word: &str) -> Option<Color> {
    // Only a name or `#hex`: CSS functions such as `rgb(..)` are not words
    // of a tag
    // `transparent` is not a color here: there is no see-through text
    if word.contains('(') || word.eq_ignore_ascii_case("transparent") {
        return None;
    }
    crate::style::parse_color(word)
}

/// What the open tags turn on, kept per attribute so that opening or
/// closing a tag takes the same time however many others are open, in
/// whatever order they close: the modifiers as counts, the colors and the
/// link as stacks (the newest one wins), and for each attribute the entries
/// that turned it on, newest last. An entry closed under newer ones is
/// marked dead and dropped once it reaches the top of its stack.
#[derive(Default)]
struct OpenTags {
    /// The next entry's number
    next: u64,
    /// Bold, italic, underline, dim, strike, reverse: how many open tags
    /// turn each on
    counts: [usize; 6],
    fg: Vec<(u64, Color)>,
    bg: Vec<(u64, Color)>,
    links: Vec<(u64, String)>,
    by_attr: HashMap<Key, Vec<u64>>,
    /// Entries closed while newer ones were above them
    dead: HashSet<u64>,
    /// The style and link in effect
    style: Style,
    link: Option<String>,
}

impl OpenTags {
    fn open(&mut self, attrs: Vec<Attr>) {
        for attr in attrs {
            let entry = self.next;
            self.next += 1;
            let key = attr.key();
            self.by_attr.entry(key).or_default().push(entry);
            match (key, attr) {
                (Key::Modifier(i), _) => self.counts[i] += 1,
                (_, Attr::Fg(c)) => self.fg.push((entry, c)),
                (_, Attr::Bg(c)) => self.bg.push((entry, c)),
                (_, Attr::Link(url)) => self.links.push((entry, url)),
                _ => {}
            }
        }
        self.restyle();
    }

    /// Turn off each attribute in `attrs` in the most recent open tag that
    /// turned it on; one nothing turned on does nothing
    fn close(&mut self, attrs: &[Attr]) {
        for attr in attrs {
            let key = attr.key();
            let Some(entry) = self.by_attr.get_mut(&key).and_then(Vec::pop) else {
                continue;
            };
            match key {
                Key::Modifier(i) => self.counts[i] -= 1,
                _ => {
                    self.dead.insert(entry);
                }
            }
        }
        self.restyle();
    }

    /// The style and link the open tags give: a modifier any of them turns
    /// on, the newest color and link; a link is underlined, and cyan when no
    /// color is on
    fn restyle(&mut self) {
        fn drop_dead<T>(stack: &mut Vec<(u64, T)>, dead: &mut HashSet<u64>) {
            while stack.last().is_some_and(|(entry, _)| dead.remove(entry)) {
                stack.pop();
            }
        }
        drop_dead(&mut self.fg, &mut self.dead);
        drop_dead(&mut self.bg, &mut self.dead);
        drop_dead(&mut self.links, &mut self.dead);
        let on = |i: usize| self.counts[i] > 0;
        self.style = Style {
            bold: on(0),
            italic: on(1),
            underline: on(2) || !self.links.is_empty(),
            dim: on(3),
            strikethrough: on(4),
            reverse: on(5),
            fg: self
                .fg
                .last()
                .map(|(_, c)| *c)
                .or_else(|| (!self.links.is_empty()).then_some(Color::CYAN)),
            bg: self.bg.last().map(|(_, c)| *c),
        };
        self.link = self.links.last().map(|(_, url)| url.clone());
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
