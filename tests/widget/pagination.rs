//! Pagination widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::pagination;
use revue::widget::traits::RenderContext;
use revue::widget::Pagination;
use revue::widget::PaginationStyle;
use revue::widget::View;

// =========================================================================
// PaginationStyle enum tests
// =========================================================================

#[test]
fn test_pagination_style_default() {
    assert_eq!(PaginationStyle::default(), PaginationStyle::Full);
}

#[test]
fn test_pagination_style_clone() {
    let style = PaginationStyle::Simple;
    assert_eq!(style, style.clone());
}

#[test]
fn test_pagination_style_copy() {
    let s1 = PaginationStyle::Compact;
    let s2 = s1;
    assert_eq!(s1, PaginationStyle::Compact);
    assert_eq!(s2, PaginationStyle::Compact);
}

#[test]
fn test_pagination_style_debug() {
    let debug_str = format!("{:?}", PaginationStyle::Dots);
    assert!(debug_str.contains("Dots"));
}

#[test]
fn test_pagination_style_partial_eq() {
    assert_eq!(PaginationStyle::Full, PaginationStyle::Full);
    assert_eq!(PaginationStyle::Simple, PaginationStyle::Simple);
    assert_eq!(PaginationStyle::Compact, PaginationStyle::Compact);
    assert_eq!(PaginationStyle::Dots, PaginationStyle::Dots);
    assert_ne!(PaginationStyle::Full, PaginationStyle::Simple);
}

// =========================================================================
// Pagination::new tests
// =========================================================================

#[test]
fn test_pagination_new() {
    let p = Pagination::new(10);
    assert_eq!(p.get_total(), 10);
    assert_eq!(p.get_current(), 1);
    assert_eq!(p.get_style(), PaginationStyle::Full);
    assert_eq!(p.get_max_visible(), 7);
    assert!(p.get_show_arrows());
    assert!(p.get_show_edges());
    assert!(!p.get_focused());

    let p = Pagination::new(10);
    assert_eq!(p.get_total(), 10);
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_pagination_new_single_page() {
    let p = Pagination::new(1);
    assert_eq!(p.get_total(), 1);
    assert_eq!(p.get_current(), 1);
}

// =========================================================================
// Pagination builder tests
// =========================================================================

#[test]
fn test_pagination_current() {
    let p = Pagination::new(10).current(5);
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_pagination_current_clamps_low() {
    let p = Pagination::new(10).current(0);
    assert_eq!(p.get_current(), 1); // Clamped to 1
}

#[test]
fn test_pagination_current_clamps_high() {
    let p = Pagination::new(10).current(15);
    assert_eq!(p.get_current(), 10); // Clamped to total
}

#[test]
fn test_pagination_style() {
    let p = Pagination::new(10).style(PaginationStyle::Compact);
    assert_eq!(p.get_style(), PaginationStyle::Compact);
}

#[test]
fn test_pagination_simple() {
    let p = Pagination::new(10).simple();
    assert_eq!(p.get_style(), PaginationStyle::Simple);
}

#[test]
fn test_pagination_compact() {
    let p = Pagination::new(10).compact();
    assert_eq!(p.get_style(), PaginationStyle::Compact);
}

#[test]
fn test_pagination_dots() {
    let p = Pagination::new(10).dots();
    assert_eq!(p.get_style(), PaginationStyle::Dots);
}

#[test]
fn test_pagination_max_visible() {
    let p = Pagination::new(10).max_visible(5);
    assert_eq!(p.get_max_visible(), 5);

    let _p = Pagination::new(10).max_visible(5);
}

#[test]
fn test_pagination_max_visible_clamps() {
    let p = Pagination::new(10).max_visible(2);
    assert_eq!(p.get_max_visible(), 3); // Clamped to minimum 3
}

#[test]
fn test_pagination_no_arrows() {
    let p = Pagination::new(10).no_arrows();
    assert!(!p.get_show_arrows());

    let _p = pagination(10).no_arrows();
}

#[test]
fn test_pagination_no_edges() {
    let p = Pagination::new(10).no_edges();
    assert!(!p.get_show_edges());

    let _p = pagination(10).no_edges();
}

#[test]
fn test_pagination_active_color() {
    let p = Pagination::new(10).active_color(Color::RED);
    assert_eq!(p.get_active_color(), Color::RED);

    let _p = Pagination::new(10).active_color(Color::CYAN);
}

#[test]
fn test_pagination_inactive_color() {
    let p = Pagination::new(10).inactive_color(Color::BLUE);
    assert_eq!(p.get_inactive_color(), Some(Color::BLUE));

    let _p = Pagination::new(10).inactive_color(Color::BLUE);
}

#[test]
fn test_pagination_focused() {
    let p = Pagination::new(10).focused();
    assert!(p.get_focused());

    let _p = Pagination::new(10).focused();
}

#[test]
fn test_pagination_builder_chain() {
    let p = Pagination::new(20)
        .current(5)
        .simple()
        .max_visible(5)
        .no_arrows()
        .no_edges()
        .active_color(Color::CYAN)
        .inactive_color(Color::rgb(128, 128, 128))
        .focused();

    assert_eq!(p.get_total(), 20);
    assert_eq!(p.get_current(), 5);
    assert_eq!(p.get_style(), PaginationStyle::Simple);
    assert_eq!(p.get_max_visible(), 5);
    assert!(!p.get_show_arrows());
    assert!(!p.get_show_edges());
    assert!(p.get_focused());
}

// =========================================================================
// Pagination navigation tests
// =========================================================================

#[test]
fn test_pagination_navigation() {
    let mut p = Pagination::new(10);

    assert!(p.next_page());
    assert_eq!(p.get_current(), 2);

    assert!(p.prev_page());
    assert_eq!(p.get_current(), 1);

    assert!(!p.prev_page()); // Can't go below 1

    p.last();
    assert_eq!(p.get_current(), 10);

    assert!(!p.next_page()); // Can't go above total

    p.first();
    assert_eq!(p.get_current(), 1);

    p.goto(5);
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_next_page_at_end() {
    let mut p = Pagination::new(5).current(5);
    assert!(!p.next_page());
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_prev_page_at_start() {
    let mut p = Pagination::new(5);
    assert!(!p.prev_page());
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_first() {
    let mut p = Pagination::new(10).current(5);
    p.first();
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_last() {
    let mut p = Pagination::new(10).current(5);
    p.last();
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_goto_clamps_low() {
    let mut p = Pagination::new(10).current(5);
    p.goto(0);
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_goto_clamps_high() {
    let mut p = Pagination::new(10).current(5);
    p.goto(20);
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_goto_middle() {
    let mut p = Pagination::new(10);
    p.goto(5);
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_goto_same_page() {
    let mut p = Pagination::new(10).current(5);
    p.goto(5);
    assert_eq!(p.get_current(), 5);
}

// =========================================================================
// Pagination query tests
// =========================================================================

#[test]
fn test_get_current() {
    let p = Pagination::new(10).current(5);
    assert_eq!(p.get_current(), 5);
}

#[test]
fn test_is_first() {
    let p = Pagination::new(10).current(1);
    assert!(p.is_first());
}

#[test]
fn test_is_first_false() {
    let p = Pagination::new(10).current(5);
    assert!(!p.is_first());
}

#[test]
fn test_is_last() {
    let p = Pagination::new(10).current(10);
    assert!(p.is_last());
}

#[test]
fn test_is_last_false() {
    let p = Pagination::new(10).current(5);
    assert!(!p.is_last());
}

#[test]
fn test_is_first_last() {
    let mut p = pagination(10);
    assert!(p.is_first());
    assert!(!p.is_last());

    p.last();
    assert!(!p.is_first());
    assert!(p.is_last());
}

#[test]
fn test_set_total() {
    let mut p = pagination(10).current(8);
    p.set_total(5);
    assert_eq!(p.get_total(), 5);
    assert_eq!(p.get_current(), 5); // Clamped to new total
}

#[test]
fn test_set_total_no_clamp_needed() {
    let mut p = pagination(10).current(5);
    p.set_total(20);
    assert_eq!(p.get_total(), 20);
    assert_eq!(p.get_current(), 5); // Unchanged
}

#[test]
fn test_set_total_to_one() {
    let mut p = pagination(10).current(5);
    p.set_total(1);
    assert_eq!(p.get_total(), 1);
    assert_eq!(p.get_current(), 1); // Clamped to 1
}

// =========================================================================
// Pagination render tests
// =========================================================================

#[test]
fn test_pagination_render_full() {
    let mut buffer = Buffer::new(50, 1);
    let area = Rect::new(0, 0, 50, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = pagination(10).current(5);
    p.render(&mut ctx);

    // Should have navigation symbols
    let text: String = (0..50)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains('5'));
}

#[test]
fn test_pagination_render_simple() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = pagination(10).current(5).simple();
    p.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains('5'));
}

#[test]
fn test_pagination_render_compact() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = pagination(10).current(5).compact();
    p.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains('5'));
    assert!(text.contains('/'));
}

#[test]
fn test_pagination_render_dots() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = pagination(5).current(3).dots();
    p.render(&mut ctx);

    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains('●'));
    assert!(text.contains('○'));
}

// =========================================================================
// Pagination Default tests
// =========================================================================

#[test]
fn test_pagination_default() {
    let p = Pagination::default();
    assert_eq!(p.get_total(), 1);
    assert_eq!(p.get_current(), 1);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_helper_function() {
    let p = pagination(15);
    assert_eq!(p.get_total(), 15);
}

#[test]
fn test_pagination_styles() {
    let p = pagination(10).simple();
    assert_eq!(p.get_style(), PaginationStyle::Simple);

    let p = pagination(10).compact();
    assert_eq!(p.get_style(), PaginationStyle::Compact);

    let p = pagination(10).dots();
    assert_eq!(p.get_style(), PaginationStyle::Dots);
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_pagination_single_page_no_nav() {
    let mut p = pagination(1);
    assert!(!p.next_page());
    assert!(!p.prev_page());
    assert!(p.is_first());
    assert!(p.is_last());
}

#[test]
fn test_pagination_two_pages() {
    let mut p = pagination(2);
    assert!(p.next_page());
    assert_eq!(p.get_current(), 2);
    assert!(p.is_last());
}

#[test]
fn test_pagination_large_total() {
    let p = pagination(1000).current(500);
    assert_eq!(p.get_current(), 500);
    assert_eq!(p.get_total(), 1000);
}

/// The symbols painted underlined.
fn underlined(p: &Pagination) -> String {
    use revue::render::Modifier;
    let mut buffer = Buffer::new(40, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 1));
    p.render(&mut ctx);
    (0..40)
        .filter_map(|x| buffer.get(x, 0))
        .filter(|c| c.modifier.contains(Modifier::UNDERLINE))
        .map(|c| c.symbol)
        .collect()
}

/// #799: `focused()` was stored and only read back by a getter. Focused, the
/// current page is underlined in every style.
#[test]
fn test_pagination_focused_underlines_the_current_page() {
    let styles = [
        (PaginationStyle::Full, "3"),
        (PaginationStyle::Simple, "Page 3 of 5"),
        (PaginationStyle::Compact, "3/5"),
        (PaginationStyle::Dots, "●"),
    ];
    for (style, want) in styles {
        let mut p = Pagination::new(5).style(style);
        p.goto(3);
        assert_eq!(underlined(&p), "", "{style:?} unfocused");
        let p = p.focused();
        assert_eq!(underlined(&p), want, "{style:?} focused");
    }
}

#[test]
fn test_pagination_style_full() {
    let _p = Pagination::new(10).style(PaginationStyle::Full);
}

#[test]
fn test_pagination_style_simple() {
    let _p = pagination(10).simple();
    // Simple style was set
}

#[test]
fn test_pagination_style_compact() {
    let _p = pagination(10).compact();
    // Compact style was set
}

#[test]
fn test_pagination_style_dots() {
    let _p = pagination(10).dots();
    // Dots style was set
}

#[test]
fn test_pagination_next_page() {
    let mut p = Pagination::new(10);
    assert!(p.next_page());
    assert_eq!(p.get_current(), 2);
}

#[test]
fn test_pagination_next_page_at_end() {
    let mut p = Pagination::new(10).current(10);
    assert!(!p.next_page());
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_pagination_prev_page() {
    let mut p = Pagination::new(10).current(5);
    assert!(p.prev_page());
    assert_eq!(p.get_current(), 4);
}

#[test]
fn test_pagination_prev_page_at_start() {
    let mut p = Pagination::new(10);
    assert!(!p.prev_page());
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_pagination_first() {
    let mut p = Pagination::new(10).current(5);
    p.first();
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_pagination_last() {
    let mut p = Pagination::new(10);
    p.last();
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_pagination_goto() {
    let mut p = Pagination::new(10);
    p.goto(7);
    assert_eq!(p.get_current(), 7);
}

#[test]
fn test_pagination_goto_clamps_below() {
    let mut p = Pagination::new(10).current(5);
    p.goto(0);
    assert_eq!(p.get_current(), 1);
}

#[test]
fn test_pagination_goto_clamps_above() {
    let mut p = Pagination::new(10).current(5);
    p.goto(20);
    assert_eq!(p.get_current(), 10);
}

#[test]
fn test_pagination_is_first() {
    let p = Pagination::new(10);
    assert!(p.is_first());

    let p = p.current(5);
    assert!(!p.is_first());
}

#[test]
fn test_pagination_is_last() {
    let mut p = Pagination::new(10);
    assert!(!p.is_last());

    p.last();
    assert!(p.is_last());
}

#[test]
fn test_pagination_set_total() {
    let mut p = pagination(10).current(8);
    p.set_total(5);
    assert_eq!(p.get_total(), 5);
    assert_eq!(p.get_current(), 5); // Clamped
}

#[test]
fn test_pagination_set_total_increase() {
    let mut p = pagination(5).current(3);
    p.set_total(10);
    assert_eq!(p.get_total(), 10);
    assert_eq!(p.get_current(), 3); // Unchanged
}

#[test]
fn test_pagination_single_page() {
    let p = Pagination::new(1);
    assert!(p.is_first());
    assert!(p.is_last());
}

#[test]
fn test_pagination_helper() {
    let p = pagination(15);
    assert_eq!(p.get_total(), 15);
}

#[test]
fn test_pagination_builder_pattern() {
    let p = pagination(10)
        .current(3)
        .simple()
        .max_visible(5)
        .active_color(Color::CYAN)
        .inactive_color(Color::BLUE);

    assert_eq!(p.get_total(), 10);
    assert_eq!(p.get_current(), 3);
}

#[test]
fn test_pagination_navigation_sequence() {
    let mut p = Pagination::new(10);

    // Go to middle
    p.goto(5);
    assert_eq!(p.get_current(), 5);

    // Go back
    p.prev_page();
    assert_eq!(p.get_current(), 4);

    // Go forward
    p.next_page();
    assert_eq!(p.get_current(), 5);

    // Jump to first
    p.first();
    assert_eq!(p.get_current(), 1);

    // Jump to last
    p.last();
    assert_eq!(p.get_current(), 10);
}

// Found by tests/event_sequences.rs: the shrunk sequence was
// `<total 0> End` - with no pages, last() moved to page 0 although pages
// count from 1 and set_total(0) keeps page 1.
#[test]
fn test_no_pages_stays_on_page_one() {
    let mut p = Pagination::new(5).current(3);
    p.set_total(0);
    assert_eq!(p.get_current(), 1);
    p.last();
    assert_eq!(p.get_current(), 1);
    p.goto(4);
    assert_eq!(p.get_current(), 1);
    assert_eq!(Pagination::new(0).current(2).get_current(), 1);
}

/// Test Pagination widget edge cases
mod edge_cases {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};

    use revue::widget::Pagination;
    #[test]
    fn test_pagination_with_zero_items() {
        let pagination = Pagination::new(0);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_with_single_item() {
        let pagination = Pagination::new(1);
        assert!(pagination.is_first() && pagination.is_last());
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_with_very_large_total() {
        let pagination = Pagination::new(1000);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_first_page() {
        let pagination = Pagination::new(10).current(1);
        assert!(pagination.is_first());
        assert!(!pagination.is_last());
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_last_page() {
        let pagination = Pagination::new(10).current(10);
        assert!(pagination.is_last());
        assert!(!pagination.is_first());
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_out_of_bounds_page() {
        let pagination = Pagination::new(10).current(100);
        // Clamped to the last page
        assert_eq!(pagination.get_current(), 10);
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_with_zero_width_buffer() {
        let pagination = Pagination::new(10);
        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_negative_page() {
        // Using current(0) should clamp to 1
        let pagination = Pagination::new(10).current(0);
        assert_eq!(pagination.get_current(), 1);
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_exact_multiple() {
        // Items divide evenly
        let pagination = Pagination::new(100);
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_with_very_small_buffer() {
        let pagination = Pagination::new(10);
        let mut buffer = Buffer::new(5, 5);
        let area = Rect::new(0, 0, 5, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should clip to buffer
        pagination.render(&mut ctx);
    }

    #[test]
    fn test_pagination_all_styles() {
        let styles = [
            revue::widget::PaginationStyle::Full,
            revue::widget::PaginationStyle::Simple,
            revue::widget::PaginationStyle::Compact,
            revue::widget::PaginationStyle::Dots,
        ];

        for style in styles {
            let pagination = Pagination::new(10).style(style);
            let mut buffer = Buffer::new(20, 10);
            let area = Rect::new(0, 0, 20, 10);
            let mut ctx = RenderContext::new(&mut buffer, area);

            pagination.render(&mut ctx);
        }
    }
}
