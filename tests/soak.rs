//! Memory stays flat over many frames (#1170).
//!
//! An app draws frame after frame while random changes hit it: tree edits,
//! keys, mouse moves and clicks, resizes, stylesheet reloads and selector
//! queries. Every `CHECKPOINT` frames it is brought back to the same state and
//! the live heap is measured. A cache that never forgets, or anything else that
//! keeps a little per frame, shows up as a heap that grows from one checkpoint
//! to the next.
//!
//! `SOAK_FRAMES` sets the length and `SOAK_SEED` the random changes; a failure
//! prints both. PR CI runs the default 3,000 frames, which only catches a leak
//! of tens of bytes a frame; the nightly run (200,000 frames) catches one of a
//! few bytes, and is the real check. A binary of its own because it installs
//! its own allocator.

use revue::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::prelude::*;
use revue::testing::PipelineHarness;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicIsize, Ordering};

/// Bytes allocated and not yet freed, over all threads.
static LIVE: AtomicIsize = AtomicIsize::new(0);

struct Counting;

// SAFETY: every call forwards to `System` with the caller's layout; the counter
// does not touch the memory.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            LIVE.fetch_add(layout.size() as isize, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            LIVE.fetch_add(layout.size() as isize, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        LIVE.fetch_sub(layout.size() as isize, Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new = unsafe { System.realloc(ptr, layout, new_size) };
        if !new.is_null() {
            LIVE.fetch_add(
                new_size as isize - layout.size() as isize,
                Ordering::Relaxed,
            );
        }
        new
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Frames between two measurements.
const CHECKPOINT: usize = 500;
/// How much the live heap may grow over the second half of the run. Bounded
/// caches fill during the first half; after that the heap moves by a few
/// hundred bytes (a map growing its capacity). Over 25,000 frames this catches
/// a leak of 3 bytes a frame.
const SLACK_BYTES: isize = 64 << 10;

/// xorshift64*: small, and the same sequence for the same seed everywhere.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    /// Text that is new most of the time, so a cache keyed by text keeps
    /// seeing new keys.
    fn text(&mut self) -> String {
        let len = self.below(12);
        (0..len)
            .map(|_| match self.below(10) {
                0 => '한',
                1 => '界',
                2 => ' ',
                _ => (b'a' + self.below(26) as u8) as char,
            })
            .collect()
    }
}

#[derive(Clone)]
struct Item {
    id: u32,
    class: usize,
    text: String,
    button: bool,
}

#[derive(Default)]
struct Page {
    items: Vec<Item>,
    next_id: u32,
}

impl Page {
    fn edit(&mut self, rng: &mut Rng) {
        match rng.below(5) {
            0 | 1 if self.items.len() < 40 => {
                let at = rng.below(self.items.len() + 1);
                self.next_id += 1;
                let item = Item {
                    id: self.next_id,
                    class: rng.below(CLASSES),
                    text: rng.text(),
                    button: rng.below(3) == 0,
                };
                self.items.insert(at, item);
            }
            2 if !self.items.is_empty() => {
                let at = rng.below(self.items.len());
                self.items.remove(at);
            }
            3 if self.items.len() > 1 => {
                let a = rng.below(self.items.len());
                let b = rng.below(self.items.len());
                self.items.swap(a, b);
            }
            _ if !self.items.is_empty() => {
                let at = rng.below(self.items.len());
                self.items[at].text = rng.text();
                self.items[at].class = rng.below(CLASSES);
            }
            _ => {}
        }
    }
}

const CLASSES: usize = 6;

impl View for Page {
    fn render(&self, ctx: &mut RenderContext) {
        let mut stack = vstack().element_id("page");
        for item in &self.items {
            let id = format!("n{}", item.id);
            let class = format!("c{}", item.class);
            if item.button {
                stack = stack.child(Button::new(item.text.as_str()).element_id(id).class(class));
            } else {
                stack = stack.child(Text::new(item.text.as_str()).element_id(id).class(class));
            }
        }
        stack.render(ctx);
    }
}

const BASE_CSS: &str = "
    .c0 { color: red; }
    .c1 + .c2 { background: blue; }
    Button:focus { color: yellow; }
    Text:hover { color: green; }
";

/// A stylesheet with selectors that change from one reload to the next.
fn random_css(rng: &mut Rng) -> String {
    let mut css = String::from(BASE_CSS);
    for _ in 0..rng.below(6) {
        let a = rng.below(CLASSES);
        let b = rng.below(CLASSES);
        let shade = rng.below(256);
        let rule = match rng.below(3) {
            0 => format!(".c{a} {{ color: rgb({shade}, 0, 0); }}"),
            1 => format!(".c{a} ~ .c{b} {{ background: rgb(0, {shade}, 0); }}"),
            _ => format!("#n{shade} {{ padding: {}; }}", rng.below(3)),
        };
        css.push_str(&rule);
    }
    css
}

/// One random thing happening to the app between two frames.
fn act(harness: &mut PipelineHarness, page: &mut Page, rng: &mut Rng) {
    let (width, height) = harness.size();
    match rng.below(10) {
        0..=3 => page.edit(rng),
        4 => {
            let key = match rng.below(5) {
                0 => Key::Tab,
                1 => Key::Down,
                2 => Key::Enter,
                3 => Key::Backspace,
                _ => Key::Char((b'a' + rng.below(26) as u8) as char),
            };
            harness.send(Event::Key(KeyEvent::new(key)), page);
        }
        5 | 6 => {
            let x = rng.below(width as usize) as u16;
            let y = rng.below(height as usize) as u16;
            let kind = if rng.below(4) == 0 {
                MouseEventKind::Down(MouseButton::Left)
            } else {
                MouseEventKind::Move
            };
            harness.send(Event::Mouse(MouseEvent::new(x, y, kind)), page);
        }
        7 => {
            let width = 10 + rng.below(110) as u16;
            let height = 3 + rng.below(47) as u16;
            harness.resize(width, height, page);
        }
        8 => {
            harness.set_css(&random_css(rng));
        }
        _ => {
            // Fewer distinct selectors than the selector cache holds (1024),
            // so it fills before the first checkpoint and then stays. Past its
            // limit it empties itself, a sawtooth as large as a leak of
            // thousands of frames; that limit has unit tests of its own.
            let selector = match rng.below(3) {
                0 => format!(".c{}", rng.below(CLASSES)),
                1 => format!("#n{}", rng.below(50)),
                _ => format!("Text.c{} + .x{}", rng.below(CLASSES), rng.below(20)),
            };
            harness.query_count(&selector);
        }
    }
}

/// The same state at every checkpoint, so the heap is comparable.
fn settle(harness: &mut PipelineHarness, page: &mut Page) -> isize {
    *page = Page {
        next_id: page.next_id,
        ..Page::default()
    };
    harness.resize(80, 24, page);
    harness.set_css(BASE_CSS);
    harness.focus(None).hover(None);
    harness.draw(page).draw(page);
    harness.clear_terminal_output();
    LIVE.load(Ordering::Relaxed)
}

fn env_or(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[test]
fn the_heap_stays_flat_over_many_frames() {
    let frames = env_or("SOAK_FRAMES", 3_000) as usize;
    let seed = env_or("SOAK_SEED", 0x5eed_1170);
    let mut rng = Rng(seed.max(1));

    let mut harness = PipelineHarness::new(80, 24).tab_navigation(true);
    let mut page = Page::default();

    let mut heap = Vec::new();
    for frame in 1..=frames {
        act(&mut harness, &mut page, &mut rng);
        harness.draw(&page);
        harness.clear_terminal_output();
        if frame % CHECKPOINT == 0 {
            heap.push(settle(&mut harness, &mut page));
        }
    }

    eprintln!("live heap per checkpoint (SOAK_SEED={seed}): {heap:?}");
    assert!(
        heap.len() >= 4,
        "SOAK_FRAMES={frames} is under four checkpoints"
    );
    // Caches that fill up to a bound fill during the first half.
    let half = heap.len() / 2;
    let start = heap[half];
    let growth = heap[half..]
        .iter()
        .map(|&live| live - start)
        .max()
        .unwrap_or(0);
    assert!(
        growth <= SLACK_BYTES,
        "the live heap grew by {growth} bytes over the second half of {frames} frames \
         (SOAK_FRAMES={frames} SOAK_SEED={seed}); per checkpoint: {heap:?}"
    );
}
