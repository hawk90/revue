//! Widget trees with stylesheets, read from bytes: nested stacks, borders,
//! text, lists, progress bars and scroll views with ids and classes, and a
//! stylesheet of rules over those ids, classes and types. Shared with the
//! `fuzz` crate (`fuzz/src/lib.rs` includes this file), so it uses only
//! `revue` and `std`.
//!
//! The check is differential: drawing tree `a` and then tree `b` in one app
//! (so the DOM is reconciled from `a` to `b`) gives the same screen, text
//! and colors, as drawing `b` in a fresh app. The rest of the bytes are
//! then events sent through the app's dispatch (Tab, keys, clicks, scrolls,
//! resizes) with a draw after each. Nothing may panic.

use revue::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::style::parse_css;
use revue::testing::PipelineHarness;
use revue::widget::{border, hstack, list, progress, scroll_view, text, vstack, View};

const WORDS: &[&str] = &[
    "",
    "a",
    "hello",
    "한글",
    "e\u{301}",
    "👩\u{200D}💻",
    "wide text that wraps",
    "\t",
    "x\ny",
];
const CLASSES: &[&str] = &["a", "b", "c"];
const IDS: &[&str] = &["one", "two", "three"];
const PROPERTIES: &[(&str, &[&str])] = &[
    (
        "color",
        &["red", "#00ff00", "rgb(1,2,3)", "var(--c)", "inherit"],
    ),
    ("background", &["blue", "transparent", "#123"]),
    ("width", &["10", "50%", "auto", "0"]),
    ("height", &["1", "3", "100%", "auto"]),
    ("padding", &["0", "1", "1 2", "2 1 0 3"]),
    ("margin", &["0", "1", "1 2"]),
    ("gap", &["0", "1", "3"]),
    ("border", &["single", "double", "rounded", "none"]),
    ("display", &["none", "flex", "block"]),
    ("flex-direction", &["row", "column"]),
    ("text-align", &["left", "center", "right"]),
    ("min-width", &["0", "5"]),
    ("max-height", &["2", "10"]),
    ("--c", &["yellow", "#fff"]),
];

/// The bytes, read one at a time; zeros once they run out
struct Reader<'a> {
    data: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn done(&self) -> bool {
        self.at >= self.data.len()
    }

    /// An event for the app's dispatch, within a screen `width` by `height`
    /// (and sometimes just outside it)
    fn event(&mut self, width: u16, height: u16) -> Event {
        let x = u16::from(self.byte()) % (width + 2);
        let y = u16::from(self.byte()) % (height + 2);
        match self.byte() % 6 {
            0 => Event::Key(KeyEvent::new(Key::Tab)),
            1 => {
                let keys = [
                    Key::Enter,
                    Key::Up,
                    Key::Down,
                    Key::Left,
                    Key::Right,
                    Key::Char('a'),
                    Key::Escape,
                ];
                Event::Key(KeyEvent::new(keys[self.byte() as usize % keys.len()]))
            }
            2 => Event::Mouse(MouseEvent::new(
                x,
                y,
                MouseEventKind::Down(MouseButton::Left),
            )),
            3 => Event::Mouse(MouseEvent::new(x, y, MouseEventKind::ScrollDown)),
            4 => Event::Mouse(MouseEvent::new(x, y, MouseEventKind::Move)),
            _ => Event::Resize(
                u16::from(self.byte() % 40) + 1,
                u16::from(self.byte() % 12) + 1,
            ),
        }
    }

    fn byte(&mut self) -> u8 {
        let b = self.data.get(self.at).copied().unwrap_or(0);
        self.at += 1;
        b
    }

    fn pick<'s>(&mut self, from: &[&'s str]) -> &'s str {
        from[self.byte() as usize % from.len()]
    }

    /// A selector: a type, `.class`, `#id`, or two joined by a combinator
    fn selector(&mut self) -> String {
        let simple = |r: &mut Self| match r.byte() % 4 {
            0 => r
                .pick(&[
                    "Stack",
                    "Border",
                    "Text",
                    "List",
                    "Progress",
                    "ScrollView",
                    "*",
                ])
                .to_string(),
            1 => format!(".{}", r.pick(CLASSES)),
            2 => format!("#{}", r.pick(IDS)),
            _ => format!(
                "{}:{}",
                r.pick(&["Text", "Border"]),
                r.pick(&["focus", "hover", "first-child"])
            ),
        };
        let first = simple(self);
        match self.byte() % 4 {
            0 => format!("{first} {}", simple(self)),
            1 => format!("{first} > {}", simple(self)),
            _ => first,
        }
    }

    fn stylesheet(&mut self) -> String {
        let rules = self.byte() % 6;
        let mut css = String::new();
        for _ in 0..rules {
            css.push_str(&self.selector());
            css.push_str(" { ");
            for _ in 0..1 + self.byte() % 4 {
                let (name, values) = PROPERTIES[self.byte() as usize % PROPERTIES.len()];
                css.push_str(&format!("{name}: {}; ", self.pick(values)));
            }
            css.push_str("}\n");
        }
        css
    }

    /// A widget, its children read to `depth` levels
    fn tree(&mut self, depth: u8) -> Box<dyn View> {
        let leaf = depth >= 3;
        let kind = self.byte() % if leaf { 3 } else { 7 };
        let class = self.pick(CLASSES);
        let id = self.pick(IDS);
        let tagged = self.byte();
        macro_rules! tag {
            ($w:expr) => {{
                let w = $w;
                let w = if tagged & 1 != 0 { w.class(class) } else { w };
                if tagged & 2 != 0 {
                    w.element_id(id)
                } else {
                    w
                }
            }};
        }
        match kind {
            0 => Box::new(tag!(text(self.pick(WORDS)))),
            1 => Box::new(tag!(progress(f32::from(self.byte()) / 200.0))),
            2 => Box::new(tag!(list(vec![
                self.pick(WORDS).to_string(),
                self.pick(WORDS).to_string()
            ]))),
            3 | 4 => {
                let mut stack = if kind == 3 { vstack() } else { hstack() };
                for _ in 0..self.byte() % 4 {
                    stack = stack.child(self.tree(depth + 1));
                }
                Box::new(tag!(stack))
            }
            5 => Box::new(tag!(border()
                .title(self.pick(WORDS))
                .child(self.tree(depth + 1)))),
            _ => Box::new(tag!(
                scroll_view().content_height(u16::from(self.byte() % 30))
            )),
        }
    }
}

/// Draw tree `a` then `b` in one app, and `b` in a fresh one: the screens
/// must match
pub fn trees(data: &[u8]) {
    let mut r = Reader { data, at: 0 };
    let css = r.stylesheet();
    if parse_css(&css).is_err() {
        // The parser is fuzzed on its own; here only sheets it takes
        return;
    }
    let width = u16::from(r.byte() % 40) + 1;
    let height = u16::from(r.byte() % 12) + 1;
    let a = r.tree(0);
    let mut b = r.tree(0);

    let mut reused = PipelineHarness::with_css(&css, width, height);
    reused.draw(&a);
    reused.draw(&b);
    let mut fresh = PipelineHarness::with_css(&css, width, height);
    fresh.draw(&b);
    let (got, want) = (reused.buffer(), fresh.buffer());
    for y in 0..height {
        for x in 0..width {
            assert!(
                got.get(x, y) == want.get(x, y),
                "drawing another tree first changed cell ({x}, {y}): {:?}, not {:?}\ncss:\n{css}\nreused:\n{}\nfresh:\n{}",
                got.get(x, y),
                want.get(x, y),
                reused.screen_text(),
                fresh.screen_text()
            );
        }
    }

    // Events through the dispatch, a draw after each
    let (mut w, mut h) = (width, height);
    let mut events = 0;
    while !r.done() && events < 64 {
        events += 1;
        let event = r.event(w, h);
        if let Event::Resize(nw, nh) = event {
            (w, h) = (nw, nh);
        }
        reused.send(event, &mut b);
        reused.draw(&b);
    }
}
