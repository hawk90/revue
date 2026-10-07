//! The runtime-configuration matrix: composite screens rendered through the
//! real `App::draw` (`PipelineHarness`) under combinations of the switches
//! that change how a frame is built.
//!
//! The factors:
//!
//! - the screen (a form, a dashboard, lists in a scroll view, nested stacks
//!   with CSS spacing, a wrapping row, ...);
//! - the terminal size;
//! - `dom_from_render`, `css_layout`, `tab_navigation` (app switches);
//! - `Stack::content_sized` (a view switch, applied to every stack the screen
//!   builds);
//! - the stylesheet: none, the screen's own, or the screen's own plus
//!   `overflow: hidden` on the frame the screen is drawn in.
//!
//! The case set is the full product of those factors - 2112 cases, a few
//! seconds in a debug build. (An all-pairs subset, as the widget matrix uses,
//! was tried first: it caught 3 of the 52 escaping cases the full product
//! found. At this size the product is cheap enough to run.)
//!
//! Every case draws two frames, sends Tab twice and draws a third, and is
//! checked for:
//!
//! 1. no panic;
//! 2. nothing written outside the screen's area - the screen is drawn one
//!    cell inside a ring of sentinel cells, which must survive every frame.
//!    Checked without a stylesheet and with `overflow: hidden`, not with the
//!    plain stylesheet: there a CSS `width`/`height`/`min-*` larger than the
//!    slot a container offers lets the widget paint past it - CSS
//!    `overflow: visible`, the default, pinned by
//!    `tests/css_card_and_overflow.rs`. What this checks instead is that
//!    every such escape stays inside a box that says `overflow: hidden`;
//! 3. a redraw of an unchanged view paints the same screen (frame 2 = frame 1);
//! 4. switches that must not change the output do not: the same case with
//!    `incremental_dom(true)`, and with `layout_engine(false)`, paints the
//!    same three frames cell for cell.
//!
//! The ratchet: the test fails on a failure not listed in [`KNOWN`], and on a
//! listed failure that no longer happens. The list only shrinks. See
//! `docs/refactor/findings-config-matrix.md`.

#[path = "matrix/sentinel.rs"]
#[allow(dead_code)]
mod sentinel;

use std::collections::BTreeSet;
use std::fmt;

use revue::event::Key;
use revue::layout::Rect;
use revue::prelude::*;
use revue::render::Cell;
use revue::testing::PipelineHarness;
use revue::widget::{Accordion, AccordionSection, Collapsible, Grid, Slider, VirtualList};

use sentinel::catch;

// ─── Screens ────────────────────────────────────────────────────────────────

/// A composite screen: its stylesheet and a builder that takes the
/// `content_sized` setting for every stack it makes.
struct Screen {
    name: &'static str,
    css: &'static str,
    build: fn(bool) -> Box<dyn View>,
}

fn v(sized: bool) -> Stack {
    vstack().content_sized(sized)
}

fn h(sized: bool) -> Stack {
    hstack().content_sized(sized)
}

fn items(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("item {i}")).collect()
}

fn screens() -> Vec<Screen> {
    vec![
        Screen {
            name: "form",
            css: ".form { gap: 1; } .field { margin-top: 1; } \
                  .buttons { gap: 2; } Button:focus { background: red; }",
            build: |s| {
                Box::new(
                    v(s).class("form")
                        .child(Text::new("Sign in").class("title"))
                        .child(Input::new().placeholder("name").class("field"))
                        .child(Input::new().placeholder("password").class("field"))
                        .child(Checkbox::new("remember me").class("field"))
                        .child(
                            h(s).class("buttons")
                                .child(Button::new("OK").element_id("ok"))
                                .child(Button::new("Cancel").element_id("cancel")),
                        ),
                )
            },
        },
        Screen {
            name: "dashboard",
            css: ".title { margin-bottom: 1; color: cyan; } .cards { gap: 2; } \
                  .card { width: 20; min-height: 3; } .status { background: blue; }",
            build: |s| {
                Box::new(
                    v(s).child(Text::new("Dashboard").class("title"))
                        .child(
                            h(s).class("cards")
                                .child(
                                    Border::rounded()
                                        .title("CPU")
                                        .child(Progress::new(0.42))
                                        .class("card"),
                                )
                                .child(
                                    Border::rounded()
                                        .title("Mem")
                                        .child(Gauge::new().percent(73.0).label("73%"))
                                        .class("card"),
                                )
                                .child(
                                    Border::rounded()
                                        .title("Net")
                                        .child(Sparkline::new(vec![1.0, 5.0, 2.0, 8.0, 3.0]))
                                        .class("card"),
                                ),
                        )
                        .child_flex(
                            Table::new(vec![Column::new("Host"), Column::new("Load")])
                                .rows(
                                    items(8)
                                        .into_iter()
                                        .map(|i| vec![i, "0.5".into()])
                                        .collect(),
                                )
                                .selected(1),
                            1.0,
                        )
                        .child(Text::new("ok").class("status")),
                )
            },
        },
        Screen {
            name: "lists",
            css: ".nav { width: 14; } .main { gap: 1; } .scroll { height: 4; }",
            build: |s| {
                Box::new(
                    h(s).child(List::new(items(50)).selected(3).class("nav"))
                        .child(
                            v(s).class("main")
                                .child(
                                    ScrollView::new()
                                        .content_height(100)
                                        .scroll_offset(10)
                                        .class("scroll"),
                                )
                                .child(VirtualList::new(items(200)).selected(150)),
                        ),
                )
            },
        },
        Screen {
            name: "nested",
            css: ".outer { gap: 1; } .inner { margin-top: 2; margin-left: 3; gap: 1; } \
                  .deeper { margin-left: 2; margin-bottom: 1; } .last { margin-top: 1; }",
            build: |s| {
                Box::new(
                    v(s).class("outer")
                        .child(Text::new("top"))
                        .child(
                            v(s).class("inner")
                                .child(Text::new("a"))
                                .child(
                                    v(s).class("deeper")
                                        .child(Text::new("b1"))
                                        .child(h(s).child(Text::new("b2")).child(Text::new("b3"))),
                                )
                                .child(Text::new("c").class("last")),
                        )
                        .child(Text::new("bottom")),
                )
            },
        },
        Screen {
            name: "tags",
            css: ".tags { flex-wrap: wrap; gap: 1; } .after { margin-top: 1; }",
            build: |s| {
                let mut row = h(s).class("tags");
                for i in 0..12 {
                    row = row.child(Tag::new(format!("tag{i}")));
                }
                Box::new(v(s).child(row).child(Text::new("after").class("after")))
            },
        },
        Screen {
            name: "hidden",
            css: ".hidden { display: none; } .gone { visibility: hidden; } \
                  .tall { height: 3; } .wide { width: 6; }",
            build: |s| {
                Box::new(
                    v(s).child(Text::new("shown"))
                        .child(Text::new("not shown").class("hidden"))
                        .child(Button::new("Press").class("tall"))
                        .child(Text::new("invisible").class("gone"))
                        .child(
                            h(s).child(Text::new("x").class("wide"))
                                .child(Text::new("y")),
                        ),
                )
            },
        },
        Screen {
            name: "oversize",
            css: ".huge { width: 500; height: 300; } .far { margin-left: 200; margin-top: 100; } \
                  .neg { min-width: 300; min-height: 90; }",
            build: |s| {
                Box::new(
                    v(s).child(
                        Border::single()
                            .title("huge")
                            .child(Text::new("x"))
                            .class("huge"),
                    )
                    .child(Text::new("far away").class("far"))
                    .child(
                        h(s).child(Button::new("min").class("neg"))
                            .child(Text::new("z")),
                    ),
                )
            },
        },
        Screen {
            name: "wide",
            css: ".card { margin: 1; } .wide { width: 9; }",
            build: |s| {
                Box::new(
                    v(s).child(
                        Card::new()
                            .title("한글 제목")
                            .subtitle("👍🏽 emoji 👨‍👩‍👧")
                            .body(Text::new("가나다라마바사아자차카타파하"))
                            .footer(Button::new("확인"))
                            .class("card"),
                    )
                    .child(
                        h(s).child(Text::new("漢字漢字漢字").class("wide"))
                            .child(Text::new("終")),
                    )
                    .child(Accordion::new().sections(vec![
                        AccordionSection::new("섹션").content("내용").expanded(true),
                        AccordionSection::new("two").content("🙂🙂").expanded(true),
                    ])),
                )
            },
        },
        Screen {
            name: "grid",
            css: ".grid { margin-left: 2; margin-top: 1; } .tabs { margin-bottom: 1; }",
            build: |s| {
                let mut g = Grid::new().cols(3).gap(1).class("grid");
                for i in 0..9 {
                    g = g.child(Button::new(format!("b{i}")));
                }
                Box::new(
                    v(s).child(Tabs::new().tabs(items(4)).class("tabs"))
                        .child_flex(g, 1.0)
                        .child(Text::new("footer")),
                )
            },
        },
        Screen {
            name: "overlay",
            css: ".panel { width: 30; height: 8; } .note { margin-top: 1; }",
            build: |s| {
                Box::new(
                    Layers::new()
                        .child(
                            v(s).child(Border::double().title("panel").class("panel"))
                                .child(Text::new("note").class("note")),
                        )
                        .child(Positioned::new(Text::new("popup text")).at(30, 10))
                        .child(Positioned::new(Text::new("edge")).at(118, 39)),
                )
            },
        },
        Screen {
            name: "controls",
            css: ".row { gap: 1; } Slider { width: 12; } .box { margin-top: 1; }",
            build: |s| {
                Box::new(
                    v(s).child(
                        h(s).class("row")
                            .child(Slider::new().value(40.0).label("vol"))
                            .child(Select::new().options(items(3)).placeholder("pick")),
                    )
                    .child(RadioGroup::new(items(3)).class("box"))
                    .child(
                        Collapsible::new("more")
                            .content("hidden body")
                            .expanded(true),
                    )
                    .child(Divider::new().label("end")),
                )
            },
        },
    ]
}

/// The screen, drawn one cell inside a ring of sentinel cells, in a `.frame`
/// stack the stylesheet can clip.
struct Guarded(Stack);

impl Guarded {
    fn new(screen: Box<dyn View>) -> Self {
        Self(vstack().class("frame").child_flex(screen, 1.0))
    }
}

const GUARD: char = '\u{E000}';

impl View for Guarded {
    fn render(&self, ctx: &mut RenderContext) {
        let a = ctx.area;
        for x in 0..a.width {
            ctx.draw_char(x, 0, GUARD, Color::WHITE);
            ctx.draw_char(x, a.height - 1, GUARD, Color::WHITE);
        }
        for y in 0..a.height {
            ctx.draw_char(0, y, GUARD, Color::WHITE);
            ctx.draw_char(a.width - 1, y, GUARD, Color::WHITE);
        }
        let inner = Rect::new(a.x + 1, a.y + 1, a.width - 2, a.height - 2);
        ctx.render_child(&self.0, inner);
    }
    fn widget_type(&self) -> &'static str {
        "Guarded"
    }
}

// ─── Cases ──────────────────────────────────────────────────────────────────

const SIZES: [(u16, u16); 4] = [(1, 1), (7, 3), (40, 12), (120, 40)];

/// Which stylesheet a case loads.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Sheet {
    None,
    /// The screen's own.
    Screen,
    /// The screen's own, and `.frame { overflow: hidden; }`.
    Clipped,
}

/// One combination of the switches.
#[derive(Clone, Copy)]
struct Config {
    dom_from_render: bool,
    css_layout: bool,
    content_sized: bool,
    tab_navigation: bool,
    sheet: Sheet,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = |b: bool| if b { '+' } else { '-' };
        write!(
            f,
            "dom{} layout{} sized{} tab{} css{}",
            s(self.dom_from_render),
            s(self.css_layout),
            s(self.content_sized),
            s(self.tab_navigation),
            match self.sheet {
                Sheet::None => "-",
                Sheet::Screen => "+",
                Sheet::Clipped => "+clip",
            }
        )
    }
}

/// What a case failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    /// Building or drawing panicked.
    Panic,
    /// A sentinel cell around the screen changed.
    OutOfArea,
    /// Redrawing the unchanged view changed the screen.
    Unstable,
    /// `incremental_dom(true)` painted something else.
    IncrementalDom,
    /// `layout_engine(false)` painted something else.
    LayoutEngine,
}

/// Known failures - the ratchet. Case keys are
/// `"<screen> <w>x<h> dom± layout± sized± tab± css(-|+|+clip)"`.
struct Known {
    kind: Kind,
    /// One-line root cause, or the design decision it waits on.
    reason: &'static str,
    cases: &'static [&'static str],
}

const KNOWN: &[Known] = &[];

/// The cells of the three frames a case paints.
type Frames = Vec<Vec<Cell>>;

/// Draw a case: frame 1, frame 2 (same view), Tab Tab, frame 3.
fn paint(
    screen: &Screen,
    size: (u16, u16),
    cfg: Config,
    incremental: bool,
    engine: bool,
) -> std::result::Result<Frames, String> {
    catch(|| {
        let (w, h) = (size.0 + 2, size.1 + 2);
        let harness = match cfg.sheet {
            Sheet::None => PipelineHarness::new(w, h),
            Sheet::Screen => PipelineHarness::with_css(screen.css, w, h),
            Sheet::Clipped => PipelineHarness::with_css(
                &format!("{} .frame {{ overflow: hidden; }}", screen.css),
                w,
                h,
            ),
        };
        let mut harness = harness
            .dom_from_render(cfg.dom_from_render)
            .css_layout(cfg.css_layout)
            .tab_navigation(cfg.tab_navigation)
            .incremental_dom(incremental)
            .layout_engine(engine);
        let mut view = Guarded::new((screen.build)(cfg.content_sized));
        let snapshot = |h: &PipelineHarness| {
            let b = h.buffer();
            (0..b.height())
                .flat_map(|y| (0..b.width()).map(move |x| (x, y)))
                .map(|(x, y)| *b.get(x, y).expect("in bounds"))
                .collect::<Vec<Cell>>()
        };
        let mut frames = Vec::new();
        harness.draw(&view);
        frames.push(snapshot(&harness));
        harness.draw(&view);
        frames.push(snapshot(&harness));
        for _ in 0..2 {
            harness.send(Event::Key(KeyEvent::new(Key::Tab)), &mut view);
        }
        harness.draw(&view);
        frames.push(snapshot(&harness));
        frames
    })
}

/// The first broken sentinel in a frame of a `w`×`h` screen, if any.
fn broken_guard(frame: &[Cell], size: (u16, u16)) -> Option<String> {
    let (w, h) = (size.0 + 2, size.1 + 2);
    for y in 0..h {
        for x in 0..w {
            let edge = x == 0 || y == 0 || x == w - 1 || y == h - 1;
            let cell = frame[(y * w + x) as usize];
            if edge && cell.symbol != GUARD {
                return Some(format!("wrote {:?} at ({x},{y})", cell.symbol));
            }
        }
    }
    None
}

/// The first cell where two paintings differ.
fn first_difference(a: &Frames, b: &Frames, size: (u16, u16)) -> Option<String> {
    let w = size.0 + 2;
    for (f, (fa, fb)) in a.iter().zip(b).enumerate() {
        if let Some(i) = fa.iter().zip(fb).position(|(x, y)| x != y) {
            let (x, y) = (i as u16 % w, i as u16 / w);
            return Some(format!(
                "frame {}: ({x},{y}) {:?} vs {:?}",
                f + 1,
                fa[i].symbol,
                fb[i].symbol
            ));
        }
    }
    None
}

/// Run one case; every failure it shows, with a detail.
fn run_case(screen: &Screen, size: (u16, u16), cfg: Config) -> Vec<(Kind, String)> {
    let base = match paint(screen, size, cfg, false, true) {
        Ok(frames) => frames,
        Err(msg) => return vec![(Kind::Panic, msg)],
    };
    let mut out = Vec::new();
    if cfg.sheet != Sheet::Screen {
        if let Some(detail) = base.iter().find_map(|f| broken_guard(f, size)) {
            out.push((Kind::OutOfArea, detail));
        }
    }
    if base[0] != base[1] {
        let detail = first_difference(&base[..1].to_vec(), &base[1..2].to_vec(), size);
        out.push((Kind::Unstable, detail.unwrap_or_default()));
    }
    for (kind, incremental, engine) in [
        (Kind::IncrementalDom, true, true),
        (Kind::LayoutEngine, false, false),
    ] {
        match paint(screen, size, cfg, incremental, engine) {
            Err(msg) => out.push((kind, format!("panicked: {msg}"))),
            Ok(twin) => {
                if let Some(detail) = first_difference(&base, &twin, size) {
                    out.push((kind, detail));
                }
            }
        }
    }
    out
}

// ─── The matrix ─────────────────────────────────────────────────────────────

/// Every combination, as index tuples: (screen, size, dom, layout, sized,
/// tab, css).
fn case_set(n_screens: usize) -> Vec<Vec<usize>> {
    let mut all: Vec<Vec<usize>> = vec![vec![]];
    for n in [n_screens, SIZES.len(), 2, 2, 2, 2, 3] {
        all = all
            .into_iter()
            .flat_map(|c| {
                (0..n).map(move |v| {
                    let mut c = c.clone();
                    c.push(v);
                    c
                })
            })
            .collect();
    }
    all
}

#[test]
fn every_screen_survives_every_combination_of_switches() {
    let screens = screens();
    let set = case_set(screens.len());

    let mut seen: BTreeSet<(Kind, String)> = BTreeSet::new();
    let mut details = Vec::new();
    for c in &set {
        let screen = &screens[c[0]];
        let size = SIZES[c[1]];
        let cfg = Config {
            dom_from_render: c[2] == 1,
            css_layout: c[3] == 1,
            content_sized: c[4] == 1,
            tab_navigation: c[5] == 1,
            sheet: [Sheet::None, Sheet::Screen, Sheet::Clipped][c[6]],
        };
        let key = format!("{} {}x{} {cfg}", screen.name, size.0, size.1);
        for (kind, detail) in run_case(screen, size, cfg) {
            seen.insert((kind, key.clone()));
            details.push((kind, key.clone(), detail));
        }
    }

    let known: BTreeSet<(Kind, String)> = KNOWN
        .iter()
        .flat_map(|k| k.cases.iter().map(move |c| (k.kind, c.to_string())))
        .collect();
    let new: Vec<_> = details
        .iter()
        .filter(|(k, key, _)| !known.contains(&(*k, key.clone())))
        .collect();
    let fixed: Vec<_> = known.difference(&seen).collect();

    eprintln!(
        "config matrix: {} cases, {} failing ({} known)",
        set.len(),
        details.len(),
        details.len() - new.len()
    );
    if new.is_empty() && fixed.is_empty() {
        return;
    }
    let mut report = String::new();
    if !new.is_empty() {
        report.push_str(&format!(
            "\n{} NEW failure(s) (fix the library, or list them in KNOWN in tests/config_matrix.rs):\n\n",
            new.len()
        ));
        for (kind, key, detail) in &new {
            report.push_str(&format!(
                "  {:<15} {key:<48} {detail}\n",
                format!("{kind:?}")
            ));
        }
    }
    if !fixed.is_empty() {
        report.push_str(&format!(
            "\n{} known failure(s) no longer happen - remove them from KNOWN:\n\n",
            fixed.len()
        ));
        for (kind, key) in fixed {
            let reason = KNOWN
                .iter()
                .find(|k| k.kind == *kind && k.cases.contains(&key.as_str()))
                .map_or("", |k| k.reason);
            report.push_str(&format!(
                "  {:<15} {key:<48} ({reason})\n",
                format!("{kind:?}")
            ));
        }
    }
    panic!("{report}");
}
