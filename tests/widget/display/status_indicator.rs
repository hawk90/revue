//! StatusIndicator widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::away_indicator;
use revue::widget::busy_indicator;
use revue::widget::offline;
use revue::widget::online;
use revue::widget::traits::RenderContext;
use revue::widget::traits::View;
use revue::widget::Status;
use revue::widget::StatusIndicator;

/// Foreground colors drawn on the first row.
fn drawn_colors(s: &StatusIndicator) -> Vec<Color> {
    let mut buffer = Buffer::new(12, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 12, 1));
    s.render(&mut ctx);
    (0..12)
        .filter_map(|x| {
            let cell = buffer.get(x, 0).unwrap();
            (cell.symbol != ' ').then_some(cell.fg).flatten()
        })
        .collect()
}

#[test]
fn test_status_indicator_new() {
    let s = StatusIndicator::new(Status::Online);
    assert_eq!(s.get_status(), Status::Online);
    let colors = drawn_colors(&s);
    assert!(!colors.is_empty());
    assert!(colors.iter().all(|&c| c == Status::Online.color()));
}

#[test]
fn test_status_indicator_set_status_changes_color() {
    let mut s = StatusIndicator::new(Status::Online);
    s.set_status(Status::Busy);
    assert_eq!(s.get_status(), Status::Busy);
    assert!(drawn_colors(&s).contains(&Status::Busy.color()));
    assert!(!drawn_colors(&s).contains(&Status::Online.color()));
}

#[test]
fn test_online_helper() {
    assert_eq!(online().get_status(), Status::Online);
    assert_eq!(
        drawn_colors(&online()),
        drawn_colors(&StatusIndicator::online())
    );
}

#[test]
fn test_offline_helper() {
    assert_eq!(offline().get_status(), Status::Offline);
    assert_eq!(
        drawn_colors(&offline()),
        drawn_colors(&StatusIndicator::offline())
    );
}

#[test]
fn test_away_indicator_helper() {
    assert_eq!(away_indicator().get_status(), Status::Away);
    assert_eq!(
        drawn_colors(&away_indicator()),
        drawn_colors(&StatusIndicator::away())
    );
}

#[test]
fn test_busy_indicator_helper() {
    assert_eq!(busy_indicator().get_status(), Status::Busy);
    assert_eq!(
        drawn_colors(&busy_indicator()),
        drawn_colors(&StatusIndicator::busy())
    );
}

#[test]
fn test_status_colors_are_distinct() {
    let all = [Status::Online, Status::Offline, Status::Busy, Status::Away];
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert_ne!(a.color(), b.color(), "{:?} vs {:?}", a, b);
        }
    }
}

/// The character a dot-style indicator draws at the origin.
fn rendered_dot(indicator: &StatusIndicator) -> char {
    let mut buffer = Buffer::new(4, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 4, 1));
    indicator.render(&mut ctx);
    buffer.get(0, 0).map(|c| c.symbol).unwrap_or(' ')
}

#[test]
fn a_stored_indicator_blinks_as_it_is_ticked() {
    let mut indicator = StatusIndicator::online().pulsing(true);
    assert_eq!(rendered_dot(&indicator), '●');
    for _ in 0..6 {
        indicator.tick();
    }
    // Frames 6 and 7 of each 8-frame cycle are the "off" phase.
    assert_eq!(rendered_dot(&indicator), ' ');
}

#[test]
fn a_rebuilt_indicator_takes_its_frame_from_the_app() {
    let on = StatusIndicator::online().pulsing(true).frame(5);
    let off = StatusIndicator::online().pulsing(true).frame(6);
    assert_eq!(rendered_dot(&on), '●');
    assert_eq!(rendered_dot(&off), ' ');
    // The cycle wraps.
    assert_eq!(
        rendered_dot(&StatusIndicator::online().pulsing(true).frame(8)),
        '●'
    );
}

#[test]
fn the_frame_does_nothing_unless_pulsing() {
    assert_eq!(rendered_dot(&StatusIndicator::online().frame(6)), '●');
}

// =========================================================================
// #1204: the Large dot's pulse, the label and the badge's size
// =========================================================================

mod sizes {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::Color;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{StatusIndicator, StatusSize, StatusStyle};

    fn draw(view: &StatusIndicator, width: u16) -> Buffer {
        let mut buffer = Buffer::new(width, 1);
        view.render(&mut RenderContext::new(
            &mut buffer,
            Rect::new(0, 0, width, 1),
        ));
        buffer
    }

    /// The pulse shows the dot at frame 0 and hides it at frame 6.
    fn at_frame(view: StatusIndicator, frame: usize) -> StatusIndicator {
        view.pulsing(true).frame(frame)
    }

    #[test]
    fn the_whole_large_dot_blinks() {
        let large = || {
            StatusIndicator::online()
                .size(StatusSize::Large)
                .indicator_style(StatusStyle::Dot)
        };
        let on = draw(&at_frame(large(), 0), 4);
        assert_ne!(on.get(0, 0).unwrap().symbol, ' ');
        assert!(on.get(1, 0).unwrap().bg.is_some());
        let off = draw(&at_frame(large(), 6), 4);
        assert_eq!(off.get(0, 0).unwrap().symbol, ' ');
        assert_eq!(off.get(1, 0).unwrap().bg, None, "column 1 blinks too");
        // Without a pulse it never goes off.
        let still = draw(&large(), 4);
        assert!(still.get(1, 0).unwrap().bg.is_some());
    }

    #[test]
    fn the_label_form_draws_the_same_dot() {
        let large = || {
            StatusIndicator::online()
                .size(StatusSize::Large)
                .indicator_style(StatusStyle::DotWithLabel)
        };
        let on = draw(&large(), 12);
        assert!(on.get(1, 0).unwrap().bg.is_some());
        let off = draw(&at_frame(large(), 6), 12);
        assert_eq!(off.get(0, 0).unwrap().symbol, ' ');
        assert_eq!(off.get(1, 0).unwrap().bg, None);
        // Smaller sizes have no second column.
        let medium = StatusIndicator::online().indicator_style(StatusStyle::DotWithLabel);
        assert_eq!(draw(&medium, 12).get(1, 0).unwrap().bg, None);
        // A narrow area keeps it in bounds.
        assert_eq!(
            draw(&large(), 1).get(0, 0).unwrap().symbol,
            StatusSize::Large.dot()
        );
    }

    #[test]
    fn the_badge_draws_the_dot_of_its_size() {
        for size in [StatusSize::Small, StatusSize::Medium, StatusSize::Large] {
            let badge = StatusIndicator::online()
                .size(size)
                .indicator_style(StatusStyle::Badge);
            let buffer = draw(&badge, 14);
            assert_eq!(buffer.get(1, 0).unwrap().symbol, size.dot(), "{size:?}");
            assert_ne!(buffer.get(1, 0).unwrap().fg, Some(Color::BLACK));
        }
    }
}

// =========================================================================
// #1528: a label set on the default dot style is drawn
// =========================================================================

mod labels {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::Color;
    use revue::widget::theme::SECONDARY_TEXT;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{Status, StatusIndicator, StatusSize, StatusStyle};

    const WIDTH: u16 = 20;

    /// What a cell holds: its glyph (`'>'` for the second half of a wide
    /// glyph, which carries the glyph's color, #1600) and its color
    type Drawn = (char, Option<Color>);

    fn row(view: &StatusIndicator) -> Vec<Drawn> {
        let mut buffer = Buffer::new(WIDTH, 1);
        view.render(&mut RenderContext::new(
            &mut buffer,
            Rect::new(0, 0, WIDTH, 1),
        ));
        (0..WIDTH)
            .map(|x| {
                let c = buffer.get(x, 0).unwrap();
                if c.is_continuation() {
                    ('>', c.fg)
                } else {
                    (c.symbol, c.fg)
                }
            })
            .collect()
    }

    /// The dot in the status color, a space, then `label` in the label
    /// color, then blank cells to the row's end
    fn dot_then(status: Status, label: &str) -> Vec<Drawn> {
        let mut out = vec![('●', Some(status.color())), (' ', None)];
        for ch in label.chars() {
            out.push((ch, Some(SECONDARY_TEXT)));
            if revue::utils::char_width(ch) == 2 {
                out.push(('>', Some(SECONDARY_TEXT)));
            }
        }
        out.resize(usize::from(WIDTH), (' ', None));
        out
    }

    /// The columns up to the last one drawn
    fn drawn_width(cells: &[Drawn]) -> u16 {
        cells
            .iter()
            .rposition(|c| *c != (' ', None))
            .map_or(0, |i| i as u16 + 1)
    }

    fn measured(view: &StatusIndicator) -> u16 {
        let (w, h) = view.measure(WIDTH, 1).unwrap();
        assert_eq!(h, 1);
        assert_eq!(w, view.width());
        w
    }

    #[test]
    fn the_default_dot_draws_a_label_it_is_given() {
        let view = StatusIndicator::busy().label("Do not disturb");
        let cells = row(&view);
        assert_eq!(cells, dot_then(Status::Busy, "Do not disturb"));
        assert_eq!(measured(&view), 16);
        assert_eq!(drawn_width(&cells), 16);
    }

    #[test]
    fn the_default_dot_without_a_label_is_the_dot_alone() {
        let view = StatusIndicator::busy();
        let cells = row(&view);
        let mut expected = vec![('●', Some(Status::Busy.color()))];
        expected.resize(usize::from(WIDTH), (' ', None));
        assert_eq!(cells, expected);
        assert_eq!(measured(&view), 1);
    }

    #[test]
    fn dot_with_label_falls_back_to_the_status_label() {
        let view = StatusIndicator::away().indicator_style(StatusStyle::DotWithLabel);
        let cells = row(&view);
        assert_eq!(cells, dot_then(Status::Away, "Away"));
        assert_eq!(measured(&view), 6);
        assert_eq!(drawn_width(&cells), 6);
    }

    #[test]
    fn dot_with_label_draws_the_custom_label() {
        let view = StatusIndicator::away()
            .indicator_style(StatusStyle::DotWithLabel)
            .label("Lunch");
        let cells = row(&view);
        assert_eq!(cells, dot_then(Status::Away, "Lunch"));
        assert_eq!(measured(&view), 7);
    }

    #[test]
    fn a_korean_label_takes_two_columns_a_syllable() {
        let view = StatusIndicator::online().label("회의 중");
        let cells = row(&view);
        assert_eq!(cells, dot_then(Status::Online, "회의 중"));
        // dot + space + 회의 중 (2 + 2 + 1 + 2)
        assert_eq!(measured(&view), 9);
        assert_eq!(drawn_width(&cells), 9);
    }

    #[test]
    fn the_label_follows_a_large_dot() {
        let view = StatusIndicator::online()
            .size(StatusSize::Large)
            .label("On");
        let cells = row(&view);
        assert_eq!(cells[0], ('⬤', Some(Status::Online.color())));
        assert_eq!(cells[2], (' ', None));
        assert_eq!(cells[3], ('O', Some(SECONDARY_TEXT)));
        assert_eq!(cells[4], ('n', Some(SECONDARY_TEXT)));
        assert_eq!(measured(&view), 5);
        assert_eq!(drawn_width(&cells), 5);
    }

    #[test]
    fn measure_matches_what_each_style_draws() {
        let styles = [
            StatusStyle::Dot,
            StatusStyle::DotWithLabel,
            StatusStyle::LabelOnly,
            StatusStyle::Badge,
        ];
        for style in styles {
            for label in [None, Some("Lunch"), Some("회의")] {
                let mut view = StatusIndicator::busy().indicator_style(style);
                if let Some(label) = label {
                    view = view.label(label);
                }
                // The last column with a glyph or a background (a badge
                // pads with background)
                let mut buffer = Buffer::new(WIDTH, 1);
                view.render(&mut RenderContext::new(
                    &mut buffer,
                    Rect::new(0, 0, WIDTH, 1),
                ));
                let drawn = (0..WIDTH)
                    .rposition(|x| {
                        let c = buffer.get(x, 0).unwrap();
                        c.symbol != ' ' || c.bg.is_some() || c.is_continuation()
                    })
                    .map_or(0, |x| x as u16 + 1);
                assert_eq!(measured(&view), drawn, "{style:?} {label:?}");
            }
        }
    }
}

// =========================================================================
// #1589: the label text of each style, cell by cell (the text loops moved to
// the shared RenderContext text helpers)
// =========================================================================

mod label_text {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::Color;
    use revue::widget::theme::{DARK_BG, SECONDARY_TEXT};
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{Status, StatusIndicator, StatusStyle};

    /// A cell: glyph and colors. The second half of a wide glyph is `'>'`
    /// with the glyph's colors, as `Buffer::set` writes it (#1600).
    type Drawn = (char, Option<Color>, Option<Color>);

    const BLANK: Drawn = (' ', None, None);
    const BUFFER: u16 = 12;

    /// The whole buffer row after drawing into `width` columns from column `x`
    fn row_at(view: &StatusIndicator, x: u16, width: u16, clip: Option<Rect>) -> Vec<Drawn> {
        let mut buffer = Buffer::new(BUFFER, 1);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(x, 0, width, 1));
        if let Some(clip) = clip {
            ctx = ctx.with_clip(clip);
        }
        view.render(&mut ctx);
        (0..BUFFER)
            .map(|x| {
                let c = buffer.get(x, 0).unwrap();
                if c.is_continuation() {
                    ('>', c.fg, c.bg)
                } else {
                    (c.symbol, c.fg, c.bg)
                }
            })
            .collect()
    }

    /// `cells` from column `x`, blank elsewhere
    fn placed(x: usize, cells: &[Drawn]) -> Vec<Drawn> {
        let mut out = vec![BLANK; x];
        out.extend_from_slice(cells);
        out.resize(usize::from(BUFFER), BLANK);
        out
    }

    fn fg(ch: char, color: Color) -> Drawn {
        (ch, Some(color), None)
    }

    fn badge(ch: char, color: Color) -> Drawn {
        (ch, Some(color), Some(DARK_BG))
    }

    const PAD: Drawn = (' ', None, Some(DARK_BG));

    fn dot_label(label: &str) -> StatusIndicator {
        StatusIndicator::online()
            .indicator_style(StatusStyle::DotWithLabel)
            .label(label)
    }

    fn label_only(label: &str) -> StatusIndicator {
        StatusIndicator::online()
            .indicator_style(StatusStyle::LabelOnly)
            .label(label)
    }

    fn badge_of(label: &str) -> StatusIndicator {
        StatusIndicator::online()
            .indicator_style(StatusStyle::Badge)
            .label(label)
    }

    fn on() -> Color {
        Status::Online.color()
    }

    #[test]
    fn dot_label_fills_an_area_at_an_offset_to_its_last_column() {
        let s = SECONDARY_TEXT;
        let full = [
            fg('●', on()),
            BLANK,
            fg('회', s),
            fg('>', s),
            fg('의', s),
            fg('>', s),
        ];
        assert_eq!(row_at(&dot_label("회의"), 3, 6, None), placed(3, &full));
        // One column short: the second syllable does not fit and nothing
        // after it is drawn, not even half of it
        assert_eq!(
            row_at(&dot_label("회의"), 3, 5, None),
            placed(3, &full[..4])
        );
        // Narrow: the dot alone, then nothing
        assert_eq!(
            row_at(&dot_label("회의"), 3, 2, None),
            placed(3, &[fg('●', on())])
        );
        assert_eq!(
            row_at(&dot_label("회의"), 3, 1, None),
            placed(3, &[fg('●', on())])
        );
    }

    #[test]
    fn dot_label_skips_zero_width_glyphs_and_stops_at_the_first_misfit() {
        let s = SECONDARY_TEXT;
        // a, zero-width space (skipped), b
        assert_eq!(
            row_at(&dot_label("a\u{200B}b"), 0, 4, None),
            placed(0, &[fg('●', on()), BLANK, fg('a', s), fg('b', s)])
        );
        // 회 does not fit in the last column, and the c after it is not drawn
        assert_eq!(
            row_at(&dot_label("a회c"), 0, 4, None),
            placed(0, &[fg('●', on()), BLANK, fg('a', s)])
        );
    }

    #[test]
    fn label_only_fills_an_area_at_an_offset_to_its_last_column() {
        let c = on();
        let full = [fg('a', c), fg('회', c), fg('>', c), fg('b', c)];
        assert_eq!(row_at(&label_only("a회b"), 2, 4, None), placed(2, &full));
        assert_eq!(
            row_at(&label_only("a회b"), 2, 3, None),
            placed(2, &full[..3])
        );
        // 회 would end past the area: the b after it is not drawn either
        assert_eq!(
            row_at(&label_only("a회b"), 2, 2, None),
            placed(2, &full[..1])
        );
        assert_eq!(
            row_at(&label_only("\u{200B}x"), 2, 1, None),
            placed(2, &[fg('x', c)])
        );
        assert_eq!(row_at(&label_only(""), 2, 4, None), placed(2, &[]));
    }

    #[test]
    fn badge_label_fills_the_badge_and_leaves_its_last_column_as_padding() {
        let w = Color::WHITE;
        let full = [
            PAD,
            badge('●', on()),
            PAD,
            badge('회', w),
            badge('>', w),
            badge('의', w),
            badge('>', w),
            PAD,
        ];
        assert_eq!(row_at(&badge_of("회의"), 1, 8, None), placed(1, &full));
        // A wider area draws the same 8 columns
        assert_eq!(row_at(&badge_of("회의"), 1, 11, None), placed(1, &full));
        // One column short: 의 does not fit and its cells stay padding
        assert_eq!(
            row_at(&badge_of("회의"), 1, 7, None),
            placed(
                1,
                &[
                    PAD,
                    badge('●', on()),
                    PAD,
                    badge('회', w),
                    badge('>', w),
                    PAD,
                    PAD
                ]
            )
        );
        // Too narrow for any label
        assert_eq!(
            row_at(&badge_of("회의"), 1, 4, None),
            placed(1, &[PAD, badge('●', on()), PAD, PAD])
        );
        assert_eq!(
            row_at(&badge_of("a\u{200B}b"), 0, 6, None),
            placed(
                0,
                &[
                    PAD,
                    badge('●', on()),
                    PAD,
                    badge('a', w),
                    badge('b', w),
                    PAD
                ]
            )
        );
    }

    #[test]
    fn a_wide_glyph_on_the_clip_edge_leaves_no_half() {
        // (view, clip width, column of 회): the clip ends between its halves
        let cases = [
            (dot_label("회"), 3, 2),
            (label_only("abc회"), 4, 3),
            (badge_of("a회"), 5, 4),
        ];
        for (i, (view, clip, col)) in cases.into_iter().enumerate() {
            let row = row_at(&view, 0, BUFFER, Some(Rect::new(0, 0, clip, 1)));
            assert_eq!(row[col].0, ' ', "case {i}: {row:?}");
            let clip = usize::from(clip);
            assert!(row[clip..].iter().all(|c| *c == BLANK), "case {i}: {row:?}");
        }
    }
}
