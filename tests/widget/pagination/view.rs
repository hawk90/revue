//! Pagination drawing against a plain model of each style, and placement.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Pagination, PaginationStyle};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const STYLES: [PaginationStyle; 4] = [
    PaginationStyle::Full,
    PaginationStyle::Simple,
    PaginationStyle::Compact,
    PaginationStyle::Dots,
];

const ACTIVE: Option<Color> = Some(Color::RED);
const INACTIVE: Option<Color> = Some(Color::BLUE);

fn render(p: &Pagination, width: u16) -> Buffer {
    let mut buffer = Buffer::new(width, 1);
    p.render(&mut RenderContext::new(
        &mut buffer,
        Rect::new(0, 0, width, 1),
    ));
    buffer
}

fn colored(total: u16, current: u16, style: PaginationStyle) -> Pagination {
    Pagination::new(total)
        .current(current)
        .style(style)
        .active_color(ACTIVE.unwrap())
        .inactive_color(INACTIVE.unwrap())
}

/// The row's symbols, without trailing blanks.
fn text(buffer: &Buffer) -> String {
    let row: String = (0..buffer.width())
        .map(|x| buffer.get(x, 0).unwrap().symbol)
        .collect();
    row.trim_end().to_string()
}

/// The foreground of the first cell drawn as `ch`.
fn fg_of(buffer: &Buffer, ch: char) -> Option<Color> {
    (0..buffer.width())
        .map(|x| buffer.get(x, 0).unwrap())
        .find(|c| c.symbol == ch)
        .and_then(|c| c.fg)
}

#[test]
fn every_style_draws_nothing_into_an_empty_area() {
    for style in STYLES {
        for current in [1, 5, 10] {
            assert_zero_areas_draw_nothing(&Pagination::new(10).current(current).style(style));
        }
        assert_zero_areas_draw_nothing(&Pagination::new(0).style(style));
    }
}

#[test]
fn every_style_draws_the_same_wherever_it_is_placed() {
    for style in STYLES {
        for current in [1, 5, 10] {
            let p = Pagination::new(10).current(current).style(style).focused();
            assert_same_when_offset(p, 40, 2);
        }
    }
}

/// The buttons that lead away from an end are inactive at that end, in both
/// the Full and the Simple style.
#[test]
fn the_buttons_at_an_end_are_inactive() {
    for (current, first, last) in [
        (1, INACTIVE, ACTIVE),
        (5, ACTIVE, ACTIVE),
        (10, ACTIVE, INACTIVE),
    ] {
        let full = render(&colored(10, current, PaginationStyle::Full), 40);
        assert_eq!(fg_of(&full, '«'), first, "« on page {current}");
        assert_eq!(fg_of(&full, '‹'), first, "‹ on page {current}");
        assert_eq!(fg_of(&full, '›'), last, "› on page {current}");
        assert_eq!(fg_of(&full, '»'), last, "» on page {current}");

        let simple = render(&colored(10, current, PaginationStyle::Simple), 40);
        assert_eq!(fg_of(&simple, '←'), first, "← on page {current}");
        assert_eq!(fg_of(&simple, '→'), last, "→ on page {current}");
    }
}

#[test]
fn no_arrows_and_no_edges_drop_their_buttons() {
    let row = |p: Pagination| text(&render(&p.current(5), 40));
    assert_eq!(row(Pagination::new(10)), "« ‹ 1 2 3 4 [5] 6 7 8 … 10 › »");
    assert_eq!(
        row(Pagination::new(10).no_arrows()),
        "« 1 2 3 4 [5] 6 7 8 … 10 »"
    );
    assert_eq!(
        row(Pagination::new(10).no_edges()),
        "‹ 1 2 3 4 [5] 6 7 8 … 10 ›"
    );
    assert_eq!(
        row(Pagination::new(10).no_arrows().no_edges()),
        "1 2 3 4 [5] 6 7 8 … 10"
    );
    // The other styles have no such buttons to drop
    for style in [
        PaginationStyle::Simple,
        PaginationStyle::Compact,
        PaginationStyle::Dots,
    ] {
        let p = Pagination::new(10).style(style);
        assert_eq!(row(p.clone()), row(p.no_arrows().no_edges()), "{style:?}");
    }
}

/// `max_visible` below 3 still shows a window of 3 pages.
#[test]
fn the_page_window_is_at_least_three_pages() {
    let row = |p: Pagination| text(&render(&p.no_arrows().no_edges(), 40));
    assert_eq!(
        row(Pagination::new(10).current(5).max_visible(1)),
        "1 … 4 [5] 6 … 10"
    );
    assert_eq!(
        row(Pagination::new(10).current(5).max_visible(1)),
        row(Pagination::new(10).current(5).max_visible(3))
    );
}

/// A page's digits are drawn bold only when it is the current page, and
/// underlined only when focused (the brackets are neither).
#[test]
fn only_the_current_page_is_bold() {
    let buffer = render(&Pagination::new(12).current(11).focused(), 50);
    for x in 0..50 {
        let cell = buffer.get(x, 0).unwrap();
        let current = matches!(x, 20 | 21); // "[11]" at columns 19..=22
        assert_eq!(
            cell.modifier.contains(Modifier::BOLD),
            current,
            "column {x} {:?}",
            cell.symbol
        );
        assert_eq!(
            cell.modifier.contains(Modifier::UNDERLINE),
            current,
            "column {x}"
        );
    }
    assert_eq!(text(&buffer), "« ‹ 1 … 6 7 8 9 10 [11] 12 › »");
}

/// What the Full style shows, parsed from the row: the page numbers in
/// order, each with whether it is bracketed, and the ellipses between them.
#[derive(Debug, PartialEq)]
enum Token {
    Page(u16, bool),
    Gap,
}

fn parse_pages(row: &str) -> Vec<Token> {
    row.split(' ')
        .filter(|t| !t.is_empty() && !["«", "‹", "›", "»"].contains(t))
        .map(|t| {
            if t == "…" {
                Token::Gap
            } else if let Some(n) = t.strip_prefix('[').and_then(|t| t.strip_suffix(']')) {
                Token::Page(n.parse().unwrap(), true)
            } else {
                Token::Page(t.parse().unwrap(), false)
            }
        })
        .collect()
}

/// The page window near `u16::MAX` (#1104)
#[test]
fn the_window_does_not_overflow_near_u16_max() {
    let last = Pagination::new(u16::MAX).current(u16::MAX);
    let pages = parse_pages(&text(&render(&last, 60)));
    assert_eq!(
        pages.last(),
        Some(&Token::Page(u16::MAX, true)),
        "{pages:?}"
    );

    let huge = Pagination::new(10).max_visible(u16::MAX);
    let pages = parse_pages(&text(&render(&huge, 60)));
    let shown: Vec<u16> = pages
        .iter()
        .filter_map(|t| match t {
            Token::Page(n, _) => Some(*n),
            Token::Gap => None,
        })
        .collect();
    assert_eq!(shown, (1..=10).collect::<Vec<u16>>());
}

proptest! {
    /// Full style against a plain model of the page window: the current page
    /// (bracketed) inside a run of `min(max_visible, total)` consecutive
    /// pages, centered unless that would leave 1..=total, plus page 1 and the
    /// last page, with an ellipsis exactly where pages are skipped.
    #[test]
    fn full_shows_a_window_around_the_current_page(
        total in 1u16..60,
        current_seed in any::<u16>(),
        max_visible in 1u16..14,
    ) {
        let current = 1 + current_seed % total;
        let p = Pagination::new(total).current(current).max_visible(max_visible);
        let tokens = parse_pages(&text(&render(&p, 200)));

        // Pages increase, the ends are shown, the current page is bracketed
        // and nothing else is
        let pages: Vec<(u16, bool)> = tokens
            .iter()
            .filter_map(|t| match t { Token::Page(n, b) => Some((*n, *b)), Token::Gap => None })
            .collect();
        prop_assert!(pages.windows(2).all(|w| w[0].0 < w[1].0), "{tokens:?}");
        prop_assert_eq!(pages.first().map(|p| p.0), Some(1));
        prop_assert_eq!(pages.last().map(|p| p.0), Some(total));
        let bracketed: Vec<u16> = pages.iter().filter(|p| p.1).map(|p| p.0).collect();
        prop_assert_eq!(bracketed, vec![current]);

        // An ellipsis stands exactly for skipped pages
        for w in tokens.windows(2) {
            if let [Token::Page(a, _), Token::Page(b, _)] = w {
                prop_assert_eq!(*b, a + 1, "{:?}", tokens);
            }
        }
        for w in tokens.windows(3) {
            if let [Token::Page(a, _), Token::Gap, Token::Page(b, _)] = w {
                prop_assert!(*b > a + 1, "{:?}", tokens);
            }
        }
        prop_assert!(!matches!(tokens.first(), Some(Token::Gap)));
        prop_assert!(!matches!(tokens.last(), Some(Token::Gap)));

        // The window: max(max_visible, 3) pages (all of them when fewer),
        // with the current page as close to its middle as the ends allow
        let size = max_visible.max(3).min(total);
        let half = max_visible.max(3) / 2;
        let start = if current <= half {
            1
        } else if current + half >= total {
            total + 1 - size
        } else {
            current - half
        };
        let window: Vec<u16> = (start..start + size).collect();
        let shown: Vec<u16> = pages.iter().map(|p| p.0).collect();
        let mut want: Vec<u16> = window.clone();
        want.push(1);
        want.push(total);
        want.sort();
        want.dedup();
        prop_assert_eq!(shown, want, "window {:?}", window);
    }

    /// Simple and Compact show the current page and the total.
    #[test]
    fn simple_and_compact_show_current_and_total(total in 0u16..2000, current in 0u16..2100) {
        let p = Pagination::new(total).current(current);
        let shown = p.get_current();
        prop_assert_eq!(
            text(&render(&p.clone().simple(), 40)),
            format!("← Page {} of {} →", shown, total)
        );
        prop_assert_eq!(text(&render(&p.compact(), 40)), format!("{}/{}", shown, total));
    }

    /// Dots: one dot per page, two columns apart, until the row is full; the
    /// current page's dot is filled and active.
    #[test]
    fn dots_draw_one_dot_per_page(total in 0u16..40, current_seed in any::<u16>(), width in 1u16..60) {
        let current = 1 + current_seed % total.max(1);
        let buffer = render(&colored(total, current, PaginationStyle::Dots), width);
        let fitting = total.min(width.div_ceil(2));
        for x in 0..width {
            let cell = buffer.get(x, 0).unwrap();
            let page = x / 2 + 1;
            let (symbol, fg) = if x % 2 == 1 || page > fitting {
                (' ', None)
            } else if page == current {
                ('●', ACTIVE)
            } else {
                ('○', INACTIVE)
            };
            prop_assert_eq!((cell.symbol, cell.fg), (symbol, fg), "column {}", x);
        }
    }
}
