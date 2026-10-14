//! The overlay's panel against a plain model: where it goes, what each row
//! holds and in which color, and the wrapped view around it. Frame timing
//! needs a clock, so the model has no frames (FPS 0.0) and records layout
//! and render times directly.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{DebugConfig, DebugEvent, DebugOverlay, DebugPosition, Text, WidgetInfo};
use std::time::Duration;

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const POSITIONS: [DebugPosition; 4] = [
    DebugPosition::TopLeft,
    DebugPosition::TopRight,
    DebugPosition::BottomLeft,
    DebugPosition::BottomRight,
];

/// The red FPS is drawn in below 15 frames per second (here: no frames).
const SLOW: Color = Color::rgb(255, 100, 100);

fn render_at<W: View>(view: &W, buffer_w: u16, buffer_h: u16, area: Rect) -> Buffer {
    let mut buffer = Buffer::new(buffer_w, buffer_h);
    view.render(&mut RenderContext::new(&mut buffer, area));
    buffer
}

fn full_overlay(position: DebugPosition) -> DebugOverlay<Text> {
    let mut overlay = DebugOverlay::wrap(Text::new("inner view text"))
        .position(position)
        .width(24)
        .show_tree(true)
        .show_events(true);
    overlay.record_widget(WidgetInfo::new("Root"));
    overlay.log_event(DebugEvent::KeyPress("Enter".into()));
    overlay
        .metrics_mut()
        .record_layout(Duration::from_millis(2));
    overlay
}

#[test]
fn the_overlay_draws_nothing_into_an_empty_area() {
    for position in POSITIONS {
        assert_zero_areas_draw_nothing(&full_overlay(position));
        assert_zero_areas_draw_nothing(&full_overlay(position).visible(false));
    }
}

#[test]
fn the_overlay_draws_the_same_wherever_it_is_placed() {
    for position in POSITIONS {
        assert_same_when_offset(full_overlay(position), 40, 22);
        assert_same_when_offset(full_overlay(position), 30, 8);
        assert_same_when_offset(full_overlay(position).visible(false), 30, 8);
    }
}

/// The panel's colors are `DebugConfig::default()`'s: its background under
/// every cell, the accent on the border, title and section headers, the
/// text color on the lines; the focused widget in the accent.
#[test]
fn the_panel_draws_in_the_default_colors() {
    let config = DebugConfig::default();
    let (bg, fg, accent) = (
        Some(config.bg_color),
        Some(config.fg_color),
        Some(config.accent_color),
    );
    let mut overlay = DebugOverlay::wrap(Text::new(""))
        .position(DebugPosition::TopLeft)
        .width(20)
        .show_tree(true)
        .show_events(true);
    let mut focused = WidgetInfo::new("Focused");
    focused.focused = true;
    overlay.record_widget(WidgetInfo::new("Plain"));
    overlay.record_widget(focused);
    overlay.log_event(DebugEvent::Custom("event".into()));
    let buffer = render_at(&overlay, 30, 20, Rect::new(0, 0, 30, 20));

    let cell = |x: u16, y: u16| *buffer.get(x, y).unwrap();
    for y in 0..20 {
        for x in 0..20 {
            assert_eq!(cell(x, y).bg, bg, "({x}, {y})");
        }
        // Nothing right of the panel
        assert_eq!(cell(20, y), Cell::default(), "(20, {y})");
    }
    // Border and title
    for (x, y) in [
        (0, 0),
        (19, 0),
        (0, 19),
        (19, 19),
        (0, 5),
        (19, 5),
        (10, 19),
    ] {
        assert_eq!(cell(x, y).fg, accent, "border ({x}, {y})");
    }
    assert_eq!(cell(2, 0).symbol, 'D');
    assert_eq!(cell(2, 0).fg, accent);
    // Rows: separator, FPS, Frame, Layout, Render, blank, Widgets:, Plain,
    // Focused, blank, Events:, event
    let line = |y: u16| -> (String, Option<Color>) {
        let text: String = (1..19).map(|x| cell(x, y).symbol).collect();
        (text.trim_end().to_string(), cell(1, y).fg)
    };
    assert_eq!(line(1), ("-".repeat(18), fg));
    assert_eq!(line(2), ("FPS: 0.0".to_string(), Some(SLOW)));
    assert_eq!(line(3), ("Frame: 0.00ms".to_string(), fg));
    assert_eq!(line(7), ("Widgets:".to_string(), accent));
    assert_eq!(line(8), ("Plain".to_string(), fg));
    assert_eq!(line(9), ("Focused [focused]".to_string(), accent));
    assert_eq!(line(11), ("Events:".to_string(), accent));
    assert_eq!(line(12), ("event".to_string(), fg));
    // The blank cells of a line keep the text color
    assert_eq!(cell(15, 8).fg, fg);
}

/// What a panel row shows: its text and its color, before truncation.
#[derive(Debug, Clone, PartialEq)]
struct Line {
    text: String,
    color: Option<Color>,
}

/// `truncate_with_suffix` for ASCII text.
fn cut(text: &str, width: usize) -> String {
    if text.len() <= width {
        text.to_string()
    } else if width <= 3 {
        "...".chars().take(width).collect()
    } else {
        format!("{}...", &text[..width - 3])
    }
}

#[derive(Debug, Clone)]
struct Scene {
    area: (u16, u16, u16, u16),
    width: u16,
    position: usize,
    sections: u8,
    widgets: Vec<(String, usize, bool)>,
    events: Vec<(u8, String)>,
    layout_ms: Vec<u16>,
    render_ms: Vec<u16>,
}

fn scene() -> impl Strategy<Value = Scene> {
    (
        (0u16..6, 0u16..4, 0u16..70, 0u16..30),
        0u16..60,
        0usize..4,
        0u8..8,
        prop::collection::vec(("[A-Za-z]{1,14}", 0usize..4, any::<bool>()), 0..12),
        prop::collection::vec((0u8..4, "[a-z ]{0,16}"), 0..9),
        prop::collection::vec(0u16..5000, 0..4),
        prop::collection::vec(0u16..5000, 0..4),
    )
        .prop_map(
            |(area, width, position, sections, widgets, events, layout_ms, render_ms)| Scene {
                area,
                width,
                position,
                sections,
                widgets,
                events,
                layout_ms,
                render_ms,
            },
        )
}

fn build(s: &Scene) -> DebugOverlay<Text> {
    let mut overlay = DebugOverlay::wrap(Text::new("wrapped view, drawn first"))
        .position(POSITIONS[s.position])
        .width(s.width)
        .show_metrics(s.sections & 1 != 0)
        .show_tree(s.sections & 2 != 0)
        .show_events(s.sections & 4 != 0);
    for (name, depth, focused) in &s.widgets {
        let mut info = WidgetInfo::new(name.clone()).depth(*depth);
        info.focused = *focused;
        overlay.record_widget(info);
    }
    for (kind, text) in &s.events {
        overlay.log_event(match kind {
            0 => DebugEvent::KeyPress(text.clone()),
            1 => DebugEvent::Mouse(text.clone()),
            2 => DebugEvent::StateChange(text.clone()),
            _ => DebugEvent::Custom(text.clone()),
        });
    }
    for ms in &s.layout_ms {
        overlay
            .metrics_mut()
            .record_layout(Duration::from_millis(*ms as u64));
    }
    for ms in &s.render_ms {
        overlay
            .metrics_mut()
            .record_render(Duration::from_millis(*ms as u64));
    }
    overlay
}

fn average(ms: &[u16]) -> f64 {
    if ms.is_empty() {
        0.0
    } else {
        ms.iter().map(|&m| m as f64).sum::<f64>() / ms.len() as f64
    }
}

/// Every row the panel would show with unlimited height, blank rows
/// standing for the spacing after a section.
fn model_lines(s: &Scene, inner: usize) -> Vec<Line> {
    let c = DebugConfig::default();
    let (fg, accent) = (Some(c.fg_color), Some(c.accent_color));
    let line = |text: String, color| Line { text, color };
    let blank = || line(String::new(), fg);
    let mut lines = vec![line("-".repeat(inner), fg)];
    if s.sections & 1 != 0 {
        lines.push(line("FPS: 0.0".into(), Some(SLOW)));
        lines.push(line("Frame: 0.00ms".into(), fg));
        lines.push(line(format!("Layout: {:.2}ms", average(&s.layout_ms)), fg));
        lines.push(line(format!("Render: {:.2}ms", average(&s.render_ms)), fg));
        lines.push(blank());
    }
    if s.sections & 2 != 0 {
        lines.push(line("Widgets:".into(), accent));
        for (name, depth, focused) in &s.widgets {
            let mut tree = format!("{}{}", "  ".repeat(*depth), name);
            if *focused {
                tree.push_str(" [focused]");
            }
            lines.push(Line {
                text: cut(&tree, inner),
                color: if *focused { accent } else { fg },
            });
        }
        lines.push(blank());
    }
    if s.sections & 4 != 0 {
        lines.push(line("Events:".into(), accent));
        for (kind, text) in s.events.iter().rev().take(5) {
            let shown = match kind {
                0 => format!("Key: {text}"),
                1 => format!("Mouse: {text}"),
                2 => format!("State: {text}"),
                _ => text.clone(),
            };
            lines.push(line(cut(&shown, inner), fg));
        }
    }
    for l in &mut lines {
        l.text = l.text.chars().take(inner).collect();
    }
    lines
}

proptest! {
    /// The panel is `min(width, area width)` by `min(20, area height)` in the
    /// chosen corner; it shows the model's rows from the top until the
    /// bottom border, with the border and title around them; everything
    /// outside it is the wrapped view alone.
    #[test]
    fn the_panel_matches_the_model(s in scene()) {
        let (ax, ay, aw, ah) = s.area;
        let (bw, bh) = (ax + aw + 2, ay + ah + 2);
        let area = Rect::new(ax, ay, aw, ah);
        let overlay = build(&s);
        let buffer = render_at(&overlay, bw, bh, area);
        let alone = render_at(&Text::new("wrapped view, drawn first"), bw, bh, area);

        let (pw, ph) = (s.width.min(aw), 20.min(ah));
        let (px, py) = match POSITIONS[s.position] {
            DebugPosition::TopLeft => (ax, ay),
            DebugPosition::TopRight => (ax + aw - pw, ay),
            DebugPosition::BottomLeft => (ax, ay + ah - ph),
            DebugPosition::BottomRight => (ax + aw - pw, ay + ah - ph),
        };
        let drawn = pw >= 2 && ph >= 2;
        let inside = |x: u16, y: u16| drawn && x >= px && x < px + pw && y >= py && y < py + ph;
        for y in 0..bh {
            for x in 0..bw {
                if !inside(x, y) {
                    prop_assert_eq!(buffer.get(x, y), alone.get(x, y), "({}, {}) outside", x, y);
                }
            }
        }
        if !drawn {
            return Ok(());
        }

        let c = DebugConfig::default();
        let accent = Some(c.accent_color);
        let inner = (pw - 2) as usize;
        let at = |x: u16, y: u16| *buffer.get(x, y).unwrap();
        // Border, with the title over the top edge
        let title: Vec<char> = " Debug ".chars().take(inner).collect();
        for x in px..px + pw {
            let top = if x == px { '┌' } else if x == px + pw - 1 { '┐' } else {
                title.get((x - px - 1) as usize).copied().unwrap_or('─')
            };
            let bottom = if x == px { '└' } else if x == px + pw - 1 { '┘' } else { '─' };
            prop_assert_eq!((at(x, py).symbol, at(x, py).fg), (top, accent), "top ({}, {})", x, py);
            prop_assert_eq!((at(x, py + ph - 1).symbol, at(x, py + ph - 1).fg), (bottom, accent));
        }
        for y in py + 1..py + ph - 1 {
            prop_assert_eq!((at(px, y).symbol, at(px, y).fg), ('│', accent));
            prop_assert_eq!((at(px + pw - 1, y).symbol, at(px + pw - 1, y).fg), ('│', accent));
        }
        // Rows between the borders
        let lines = model_lines(&s, inner);
        for (k, y) in (py + 1..py + ph - 1).enumerate() {
            let want = lines.get(k).cloned().unwrap_or(Line { text: String::new(), color: Some(c.fg_color) });
            let text: String = (px + 1..px + pw - 1).map(|x| at(x, y).symbol).collect();
            prop_assert_eq!(text.trim_end(), want.text.trim_end(), "row {}", k);
            for (i, x) in (px + 1..px + pw - 1).enumerate() {
                let fg = if i < want.text.chars().count() { want.color } else { Some(c.fg_color) };
                prop_assert_eq!(at(x, y).fg, fg, "row {} column {}", k, i);
                prop_assert_eq!(at(x, y).bg, Some(c.bg_color));
            }
        }
    }
}
