//! The typing animation and the stream state, against a plain model: how
//! far each tick reveals the text in each typing style, what each call does
//! to the status, and what the screen shows after each step
//!
//! Every tick runs with `typing_speed(0)`, so each one takes a step whatever
//! the wall clock says.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::utils::char_width;
use revue::widget::traits::RenderContext;
use revue::widget::{AiStream, StreamStatus, TypingStyle, View};

const STYLES: [TypingStyle; 5] = [
    TypingStyle::None,
    TypingStyle::Character,
    TypingStyle::Word,
    TypingStyle::Line,
    TypingStyle::Chunk,
];

const SPINNER: [char; 4] = ['⠋', '⠙', '⠹', '⠸'];

#[derive(Clone, Debug)]
enum Op {
    Append(String),
    Tick,
    Pause,
    Resume,
    Complete,
    Error,
    Clear,
    SetContent(String),
}

/// What the stream should hold: the text, how many chars of it are shown,
/// the status and the animation frame
struct Model {
    style: TypingStyle,
    text: Vec<char>,
    shown: usize,
    status: StreamStatus,
    frame: usize,
}

impl Model {
    /// The number of chars shown after the next typing step
    fn next_stop(&self) -> usize {
        let total = self.text.len();
        let mut after = self.shown + 1..=total;
        match self.style {
            TypingStyle::None => total,
            TypingStyle::Character => (self.shown + 1).min(total),
            // up to the start of the next word, or the end
            TypingStyle::Word => after
                .find(|&i| {
                    i == total
                        || (self.text[i - 1].is_whitespace() && !self.text[i].is_whitespace())
                })
                .unwrap_or(total),
            // up to just after the next newline, or the end
            TypingStyle::Line => after
                .find(|&i| i == total || self.text[i - 1] == '\n')
                .unwrap_or(total),
            TypingStyle::Chunk => (self.shown + 5).min(total),
        }
    }

    fn tick(&mut self) {
        self.frame = (self.frame + 1) % 4;
        if self.status != StreamStatus::Streaming {
            return;
        }
        let total = self.text.len();
        // Character style takes one more tick to notice it has caught up
        if self.style == TypingStyle::Character && self.shown < total {
            self.shown += 1;
            return;
        }
        self.shown = self.next_stop();
        if self.shown >= total {
            self.status = StreamStatus::Complete;
        }
    }

    fn apply(&mut self, op: &Op) {
        match op {
            Op::Append(text) => {
                self.text.extend(text.chars());
                if self.status == StreamStatus::Idle {
                    self.status = StreamStatus::Streaming;
                }
            }
            Op::Tick => self.tick(),
            Op::Pause if self.status == StreamStatus::Streaming => {
                self.status = StreamStatus::Paused
            }
            Op::Resume if self.status == StreamStatus::Paused => {
                self.status = StreamStatus::Streaming
            }
            Op::Pause | Op::Resume => {}
            Op::Complete => {
                self.status = StreamStatus::Complete;
                self.shown = self.text.len();
            }
            Op::Error => self.status = StreamStatus::Error,
            Op::Clear => {
                self.text.clear();
                self.shown = 0;
                self.status = StreamStatus::Idle;
            }
            Op::SetContent(text) => {
                self.text = text.chars().collect();
                self.shown = self.text.len();
                self.status = StreamStatus::Complete;
            }
        }
    }

    fn progress(&self) -> f32 {
        if self.text.is_empty() {
            1.0
        } else {
            self.shown as f32 / self.text.len() as f32
        }
    }

    /// The rows the shown text takes (no wrap) and where the cursor goes
    fn lines(&self) -> Vec<String> {
        let shown: String = self.text[..self.shown].iter().collect();
        shown.split('\n').map(str::to_string).collect()
    }
}

fn columns(line: &str) -> usize {
    line.chars().map(char_width).sum()
}

/// Rows of `stream` drawn without wrap, continuation cells left out
fn screen(stream: &AiStream, width: u16, height: u16) -> Vec<String> {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    stream.render(&mut ctx);
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer.get(x, y).unwrap())
                .filter(|c| !c.is_continuation())
                .map(|c| c.symbol)
                .collect()
        })
        .collect()
}

/// The screen the model says the stream shows, in an area that fits it
fn expected_screen(model: &Model, width: u16, height: u16) -> Vec<String> {
    let mut rows = vec![String::new(); height as usize];
    if model.text.is_empty() && model.status == StreamStatus::Streaming {
        rows[0] = format!("{} Thinking...", SPINNER[model.frame]);
    } else {
        let lines = model.lines();
        for (row, line) in rows.iter_mut().zip(&lines) {
            row.push_str(line);
        }
        if model.status == StreamStatus::Streaming && model.frame.is_multiple_of(2) {
            // right after the last shown glyph
            rows[lines.len() - 1].push('█');
        }
    }
    rows.iter()
        .map(|r| {
            let used: usize = r.chars().map(char_width).sum();
            format!("{r}{}", " ".repeat(width as usize - used))
        })
        .collect()
}

fn piece() -> impl Strategy<Value = String> {
    prop::sample::select(vec!["", "a", "bc", " ", "\n", "世", "word ", " \n", "x y"])
        .prop_map(str::to_string)
}

fn text() -> impl Strategy<Value = String> {
    prop::collection::vec(piece(), 0..6).prop_map(|parts| parts.concat())
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        3 => piece().prop_map(Op::Append),
        8 => Just(Op::Tick),
        1 => Just(Op::Pause),
        1 => Just(Op::Resume),
        1 => Just(Op::Complete),
        1 => Just(Op::Error),
        1 => Just(Op::Clear),
        1 => text().prop_map(Op::SetContent),
    ]
}

/// How the stream starts: idle, with `content(..)`, or with a first append
#[derive(Clone, Debug)]
enum Start {
    Idle,
    Content(String),
    Append(String),
}

fn start() -> impl Strategy<Value = Start> {
    prop_oneof![
        Just(Start::Idle),
        text().prop_map(Start::Content),
        text().prop_map(Start::Append),
    ]
}

fn check(stream: &AiStream, model: &Model, step: &str) -> Result<(), TestCaseError> {
    prop_assert_eq!(stream.status(), model.status, "{}", step);
    prop_assert_eq!(stream.is_complete(), model.status == StreamStatus::Complete);
    prop_assert_eq!(stream.progress(), model.progress(), "{}", step);
    let lines = model.lines();
    let width = lines.iter().map(|l| columns(l)).max().unwrap_or(0).max(13) as u16 + 1;
    let height = lines.len() as u16 + 1;
    prop_assert_eq!(
        screen(stream, width, height),
        expected_screen(model, width, height),
        "{}",
        step
    );
    Ok(())
}

proptest! {
    #[test]
    fn calls_and_ticks_follow_the_model(
        style in prop::sample::select(STYLES.to_vec()),
        start in start(),
        ops in prop::collection::vec(op(), 0..24),
    ) {
        let mut stream = AiStream::new().typing_style(style).typing_speed(0).wrap(false);
        let mut model = Model {
            style,
            text: Vec::new(),
            shown: 0,
            status: StreamStatus::Idle,
            frame: 0,
        };
        match &start {
            Start::Idle => {}
            Start::Content(text) => {
                stream = stream.content(text.clone());
                model.text = text.chars().collect();
                model.status = StreamStatus::Streaming;
            }
            Start::Append(text) => {
                stream.append(text);
                model.apply(&Op::Append(text.clone()));
            }
        }
        check(&stream, &model, "start")?;
        for (i, op) in ops.iter().enumerate() {
            // Text appended after the stream completed is never typed out
            // (#891), so the model leaves that out
            if matches!(op, Op::Append(t) if !t.is_empty())
                && model.status == StreamStatus::Complete
            {
                continue;
            }
            match op {
                Op::Append(text) => stream.append(text),
                Op::Tick => stream.tick(),
                Op::Pause => stream.pause(),
                Op::Resume => stream.resume(),
                Op::Complete => stream.complete(),
                Op::Error => stream.error(),
                Op::Clear => stream.clear(),
                Op::SetContent(text) => stream.set_content(text.clone()),
            }
            model.apply(op);
            check(&stream, &model, &format!("op {i} {op:?}"))?;
        }
    }

    #[test]
    fn every_style_types_the_whole_text_and_completes(
        style in prop::sample::select(STYLES.to_vec()),
        text in text(),
    ) {
        let mut stream = AiStream::new().typing_style(style).typing_speed(0).content(text.clone());
        let total = text.chars().count();
        let mut last = 0.0;
        let mut ticks = 0;
        while !stream.is_complete() {
            stream.tick();
            ticks += 1;
            // progress never goes back
            prop_assert!(stream.progress() >= last);
            last = stream.progress();
            prop_assert!(ticks <= total + 1, "{:?} still typing after {} ticks", style, ticks);
        }
        prop_assert_eq!(stream.progress(), 1.0);
        let lines: Vec<&str> = text.split('\n').collect();
        let width = lines.iter().map(|l| columns(l)).max().unwrap_or(0) as u16 + 1;
        let drawn = screen(&stream, width, lines.len() as u16);
        for (row, line) in drawn.iter().zip(&lines) {
            prop_assert_eq!(row.trim_end(), line.trim_end());
        }
    }
}

/// The chars shown after each tick until the stream of `total` chars
/// completes; it gives up after `total + 1` ticks, so a stream that never
/// completes fails the test instead of hanging it
fn stops(stream: &mut AiStream, total: usize) -> Vec<usize> {
    let mut stops = Vec::new();
    while !stream.is_complete() && stops.len() <= total {
        stream.tick();
        stops.push((stream.progress() * total as f32).round() as usize);
    }
    stops
}

#[test]
fn ticks_before_typing_speed_has_passed_do_not_type() {
    let mut stream = AiStream::new().typing_speed(u64::MAX).content("abc");
    for _ in 0..5 {
        stream.tick();
    }
    assert_eq!(stream.progress(), 0.0);
    assert_eq!(stream.status(), StreamStatus::Streaming);
}

#[test]
fn character_style_completes_one_tick_after_the_last_char() {
    let mut stream = AiStream::new().typing_speed(0).content("ab");
    stream.tick();
    stream.tick();
    assert_eq!(stream.progress(), 1.0);
    assert_eq!(stream.status(), StreamStatus::Streaming);
    stream.tick();
    assert!(stream.is_complete());
}

#[test]
fn word_style_stops_at_each_word_start() {
    let mut stream = AiStream::new()
        .typing_style(TypingStyle::Word)
        .typing_speed(0)
        .content("  one  two\nthree");
    let stops = stops(&mut stream, 16);
    assert!(stream.is_complete());
    // leading blanks, "one  ", "two\n", "three"
    assert_eq!(stops, [2, 7, 11, 16]);
}

#[test]
fn line_style_stops_after_each_newline() {
    let mut stream = AiStream::new()
        .typing_style(TypingStyle::Line)
        .typing_speed(0)
        .content("ab\n\ncd");
    let stops = stops(&mut stream, 6);
    assert!(stream.is_complete());
    assert_eq!(stops, [3, 4, 6]);
}

#[test]
fn chunk_style_shows_five_chars_a_tick() {
    let mut stream = AiStream::new()
        .typing_style(TypingStyle::Chunk)
        .typing_speed(0)
        .content("0123456789ab");
    let stops = stops(&mut stream, 12);
    assert!(stream.is_complete());
    assert_eq!(stops, [5, 10, 12]);
}
