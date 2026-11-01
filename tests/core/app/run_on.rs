//! `App::run_on`: a terminal and events the caller owns (#1153), and `Tick`
//! under input that never stops (#1195)

use revue::core::app::App;
use revue::event::{Event, EventSource, Key, KeyEvent, Polled};
use revue::render::Terminal;
use revue::testing::MockTime;
use revue::widget::{RenderContext, Text, View};
use std::collections::VecDeque;
use std::io::Write;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const MS: fn(u64) -> Duration = Duration::from_millis;

/// A writer the test reads while the terminal holds it
#[derive(Clone, Default)]
struct Shared(Arc<Mutex<Vec<u8>>>);

impl Write for Shared {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Shared {
    /// The screen a terminal shows after these bytes
    fn screen(&self, width: u16, height: u16) -> Vec<String> {
        let nz = |n: u16| std::num::NonZeroU16::new(n).unwrap();
        let mut parser = vt100::Parser::new(nz(height), nz(width), 0);
        parser.process(&self.0.lock().unwrap());
        let screen = parser.screen();
        screen.rows(0, width).collect()
    }
}

/// Events from a script, then closed
struct Script(VecDeque<Polled>);

impl EventSource for Script {
    fn poll_event(&mut self, _: Duration) -> revue::Result<Polled> {
        Ok(self.0.pop_front().unwrap_or(Polled::Closed))
    }
}

fn app() -> App {
    App::builder().build()
}

fn key(c: char) -> Event {
    Event::Key(KeyEvent::new(Key::Char(c)))
}

#[test]
fn it_draws_on_the_terminal_it_is_given() {
    let out = Shared::default();
    let mut terminal = Terminal::with_size(out.clone(), 12, 3);
    app()
        .run_on(
            Text::new("hello"),
            &mut terminal,
            Script(VecDeque::new()),
            |_, _, _| false,
        )
        .unwrap();
    assert_eq!(out.screen(12, 3)[0].trim_end(), "hello");
}

/// Draws its own size, so a test sees the area the app gave it
struct SizeView;

impl View for SizeView {
    fn render(&self, ctx: &mut RenderContext) {
        Text::new(format!("{}x{}", ctx.area.width, ctx.area.height)).render(ctx);
    }
}

#[test]
fn it_takes_the_terminals_size_and_follows_a_resize() {
    let out = Shared::default();
    let mut terminal = Terminal::with_size(out.clone(), 20, 4);
    let script = Script(VecDeque::from([Polled::Event(Event::Resize(30, 6))]));
    app()
        .run_on(SizeView, &mut terminal, script, |_, _, _| false)
        .unwrap();
    assert_eq!(terminal.size(), (30, 6));
    assert_eq!(out.screen(30, 6)[0].trim_end(), "30x6");
}

#[test]
fn the_handler_gets_each_event_and_can_redraw() {
    let out = Shared::default();
    let mut terminal = Terminal::with_size(out.clone(), 12, 3);
    let script = Script(VecDeque::from([
        Polled::Event(key('a')),
        Polled::Event(key('b')),
    ]));
    let mut seen = Vec::new();
    app()
        .run_on(
            Text::new("x"),
            &mut terminal,
            script,
            |event, view: &mut Text, _| {
                if let Event::Key(KeyEvent {
                    key: Key::Char(c), ..
                }) = event
                {
                    seen.push(*c);
                    *view = Text::new(c.to_string());
                    return true;
                }
                false
            },
        )
        .unwrap();
    assert_eq!(seen, ['a', 'b']);
    assert_eq!(out.screen(12, 3)[0].trim_end(), "b");
}

#[test]
fn the_app_quitting_ends_it_before_the_source_does() {
    let mut terminal = Terminal::with_size(Vec::new(), 12, 3);
    let script = Script(VecDeque::from([
        Polled::Event(key('a')),
        Polled::Event(key('b')),
    ]));
    let mut seen = 0;
    app()
        .run_on(Text::new("x"), &mut terminal, script, |event, _, app| {
            if matches!(event, Event::Key(_)) {
                seen += 1;
                app.quit();
            }
            false
        })
        .unwrap();
    assert_eq!(seen, 1);
}

#[test]
fn a_source_error_comes_back() {
    struct Broken;
    impl EventSource for Broken {
        fn poll_event(&mut self, _: Duration) -> revue::Result<Polled> {
            Err(std::io::Error::other("gone").into())
        }
    }
    let mut terminal = Terminal::with_size(Vec::new(), 12, 3);
    let result = app().run_on(Text::new("x"), &mut terminal, Broken, |_, _, _| false);
    assert!(result.is_err());
}

#[test]
fn a_channel_ends_when_every_sender_is_gone() {
    let (input, events) = mpsc::channel();
    let session = std::thread::spawn(move || {
        let mut terminal = Terminal::with_size(Vec::new(), 12, 3);
        let mut keys = 0;
        App::builder()
            .build()
            .run_on(Text::new("x"), &mut terminal, events, |event, _, _| {
                keys += usize::from(matches!(event, Event::Key(_)));
                false
            })
            .unwrap();
        keys
    });
    input.send(key('a')).unwrap();
    input.send(key('b')).unwrap();
    drop(input);
    assert_eq!(session.join().unwrap(), 2);
}

#[test]
fn a_channel_with_nothing_waiting_times_out() {
    let (_input, mut events) = mpsc::channel::<Event>();
    assert_eq!(events.poll_event(MS(1)).unwrap(), Polled::Timeout);
}

/// Input that never stops: a key every `gap`, with the clock moving
struct Flood {
    time: MockTime,
    gap: u64,
    left: u32,
}

impl EventSource for Flood {
    fn poll_event(&mut self, timeout: Duration) -> revue::Result<Polled> {
        assert!(!timeout.is_zero(), "a due Tick does not wait for input");
        if self.left == 0 {
            return Ok(Polled::Closed);
        }
        self.left -= 1;
        self.time.advance_ms(self.gap);
        Ok(Polled::Event(key('x')))
    }
}

fn ticks_during_flood(gap: u64, keys: u32) -> (u32, u32) {
    let time = MockTime::new();
    let _clock = time.install();
    let mut terminal = Terminal::with_size(Vec::new(), 12, 3);
    let source = Flood {
        time: time.clone(),
        gap,
        left: keys,
    };
    let (mut ticks, mut seen) = (0, 0);
    app()
        .run_on(Text::new("x"), &mut terminal, source, |event, _, _| {
            match event {
                Event::Tick => ticks += 1,
                Event::Key(_) => seen += 1,
                _ => {}
            }
            false
        })
        .unwrap();
    (ticks, seen)
}

#[test]
fn ticks_keep_coming_under_input_that_never_stops() {
    // 16 ms per tick, 1 000 keys 5 ms apart: 5 s of input with no idle moment.
    let (ticks, keys) = ticks_during_flood(5, 1000);
    assert_eq!(keys, 1000, "input is not dropped for ticks");
    // Keys land at 5, 10, 15, 20 ms: a Tick is due at the first check at least
    // 16 ms on, so one per four keys
    assert_eq!(ticks, 250);
}

#[test]
fn input_slower_than_the_tick_rate_gets_a_tick_per_key() {
    let (ticks, keys) = ticks_during_flood(100, 10);
    assert_eq!((ticks, keys), (10, 10));
}

#[test]
fn no_input_at_all_gives_ticks_at_once() {
    let time = MockTime::new();
    let _clock = time.install();
    let mut terminal = Terminal::with_size(Vec::new(), 12, 3);
    let script = Script(VecDeque::from([
        Polled::Timeout,
        Polled::Timeout,
        Polled::Closed,
    ]));
    let mut ticks = 0;
    app()
        .run_on(Text::new("x"), &mut terminal, script, |event, _, _| {
            ticks += u32::from(matches!(event, Event::Tick));
            false
        })
        .unwrap();
    assert_eq!(ticks, 2);
}
