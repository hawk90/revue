//! Tests for shared dropdown rendering helpers

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::input_widgets::dropdown::{
    calculate_dropdown_layout, dropdown_height, label_width, queue_or_inline_overlay,
    render_options, render_status_row, DropdownColors, DropdownOption, INDICATOR_COL, LABEL_COL,
    MIN_DROPDOWN_WIDTH,
};
use revue::widget::traits::{OverlayEntry, RenderContext};
use std::collections::HashSet;

// ==================== layout constants ====================

#[test]
fn test_layout_constants_coherent() {
    // Indicator at col 0, label at col 2 → col 1 is the gap
    assert_eq!(INDICATOR_COL, 0);
    assert_eq!(LABEL_COL, 2);
    const { assert!(LABEL_COL > INDICATOR_COL) };
    const { assert!(MIN_DROPDOWN_WIDTH > LABEL_COL) };
}

#[test]
fn test_label_width_no_reserve() {
    // width=20, no right reserve → 20 - 2 = 18 chars for label
    assert_eq!(label_width(20, 0), 18);
}

#[test]
fn test_label_width_with_reserve() {
    // width=20, reserve 1 right col → 20 - 2 - 1 = 17 chars for label
    assert_eq!(label_width(20, 1), 17);
}

#[test]
fn test_label_width_saturates_on_small_width() {
    // width smaller than LABEL_COL should not panic and return 0
    assert_eq!(label_width(1, 0), 0);
    assert_eq!(label_width(0, 5), 0);
}

// ==================== dropdown_height ====================

#[test]
fn test_dropdown_height_zero_items() {
    assert_eq!(dropdown_height(0, None), 1);
}

#[test]
fn test_dropdown_height_within_cap() {
    assert_eq!(dropdown_height(5, None), 5);
}

#[test]
fn test_dropdown_height_exceeds_default_cap() {
    // MAX_DROPDOWN_VISIBLE is 10
    assert_eq!(dropdown_height(20, None), 10);
}

#[test]
fn test_dropdown_height_custom_cap() {
    assert_eq!(dropdown_height(20, Some(3)), 3);
}

#[test]
fn test_dropdown_height_custom_cap_larger_than_max() {
    // Custom cap of 15 should be clamped to MAX_DROPDOWN_VISIBLE (10)
    assert_eq!(dropdown_height(20, Some(15)), 10);
}

#[test]
fn test_dropdown_height_one_item() {
    assert_eq!(dropdown_height(1, None), 1);
}

// ==================== calculate_dropdown_layout ====================

#[test]
fn test_layout_renders_below_when_space() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 2, 40, 1); // widget at y=2
    let ctx = RenderContext::new(&mut buffer, area);

    let layout = calculate_dropdown_layout(&ctx, 5);
    assert_eq!(layout.overlay_y, 3); // below the widget
    assert_eq!(layout.height, 5);
}

#[test]
fn test_layout_flips_above_when_near_bottom() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 8, 40, 1); // widget near bottom
    let ctx = RenderContext::new(&mut buffer, area);

    let layout = calculate_dropdown_layout(&ctx, 5);
    // Not enough space below (10 - 9 = 1 < 5), should flip above
    assert_eq!(layout.overlay_y, 8u16.saturating_sub(5)); // 3
}

#[test]
fn test_layout_no_room_either_side_takes_the_roomier_one_shortened() {
    // Header on row 1 of 4: one row above, two below
    let mut buffer = Buffer::new(20, 4);
    let ctx = RenderContext::new(&mut buffer, Rect::new(0, 1, 20, 1));
    let layout = calculate_dropdown_layout(&ctx, 3);
    assert_eq!((layout.overlay_y, layout.height), (2, 2));

    // Header on row 2 of 4: two above, one below
    let mut buffer = Buffer::new(20, 4);
    let ctx = RenderContext::new(&mut buffer, Rect::new(0, 2, 20, 1));
    let layout = calculate_dropdown_layout(&ctx, 3);
    assert_eq!((layout.overlay_y, layout.height), (0, 2));
}

#[test]
fn test_layout_exactly_enough_room_below_stays_below() {
    let mut buffer = Buffer::new(40, 10);
    let ctx = RenderContext::new(&mut buffer, Rect::new(0, 4, 40, 1));
    let layout = calculate_dropdown_layout(&ctx, 5);
    assert_eq!((layout.overlay_y, layout.height), (5, 5));
}

#[test]
fn test_layout_one_row_screen_has_no_room() {
    let mut buffer = Buffer::new(20, 1);
    let ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 1));
    assert_eq!(calculate_dropdown_layout(&ctx, 3).height, 0);
}

/// Draw `v` in a one-row area on row `row` of a 20 x 4 screen, through the
/// overlay queue, and return the screen's rows
fn screen_rows(v: &dyn revue::widget::traits::View, row: u16) -> Vec<String> {
    let mut b = Buffer::new(20, 4);
    let mut q = revue::widget::OverlayQueue::new();
    {
        let mut ctx =
            RenderContext::new(&mut b, Rect::new(0, row, 20, 1)).with_overlay_queue(&mut q);
        v.render(&mut ctx);
    }
    q.render_to(&mut b);
    (0..4)
        .map(|y| (0..20).map(|x| b.get(x, y).unwrap().symbol).collect())
        .collect()
}

fn has_option(row: &str) -> bool {
    ["Apple", "Banana", "Cherry"]
        .iter()
        .any(|o| row.contains(o))
}

/// The header `v` draws on its own (row 0 of a one-row screen; the inline
/// dropdown falls off it)
fn header(v: &dyn revue::widget::traits::View) -> String {
    let mut b = Buffer::new(20, 1);
    v.render(&mut RenderContext::new(&mut b, Rect::new(0, 0, 20, 1)));
    (0..20).map(|x| b.get(x, 0).unwrap().symbol).collect()
}

/// #1095: with no room for all three options on either side, the dropdown
/// takes the two rows below and scrolls; it never covers the header.
/// The three widgets share the placement, so they are tested together
#[test]
fn test_dropdowns_do_not_cover_their_header() {
    use revue::widget::{multi_select, Combobox, Select};
    let opts = || vec!["Apple", "Banana", "Cherry"];

    let mut m = multi_select().options(opts());
    m.open();
    let mut s = Select::new().options(opts());
    s.open();
    let mut c = Combobox::new().options(opts());
    c.open_dropdown();

    for (name, v) in [
        ("MultiSelect", &m as &dyn revue::widget::traits::View),
        ("Select", &s),
        ("Combobox", &c),
    ] {
        let rows = screen_rows(v, 1);
        assert!(!has_option(&rows[0]), "{name}: row 0 {rows:?}");
        assert_eq!(rows[1], header(v), "{name}: header {rows:?}");
        assert!(rows[2].contains("Apple"), "{name}: row 2 {rows:?}");
        assert!(rows[3].contains("Banana"), "{name}: row 3 {rows:?}");
    }
    assert!(screen_rows(&m, 1)[1].contains('▲'));
    assert!(!has_option(&header(&m)));
}

/// Above the header when that side is roomier: the rows end right above it
#[test]
fn test_dropdowns_above_their_header_end_right_above_it() {
    use revue::widget::{multi_select, Combobox, Select};
    let opts = || vec!["Apple", "Banana", "Cherry"];

    let mut m = multi_select().options(opts());
    m.open();
    let mut s = Select::new().options(opts());
    s.open();
    let mut c = Combobox::new().options(opts());
    c.open_dropdown();

    for (name, v) in [
        ("MultiSelect", &m as &dyn revue::widget::traits::View),
        ("Select", &s),
        ("Combobox", &c),
    ] {
        let rows = screen_rows(v, 2);
        assert!(rows[0].contains("Apple"), "{name}: row 0 {rows:?}");
        assert!(rows[1].contains("Banana"), "{name}: row 1 {rows:?}");
        assert_eq!(rows[2], header(v), "{name}: header {rows:?}");
        assert!(!has_option(&rows[3]), "{name}: row 3 {rows:?}");
    }
}

// ==================== render_status_row ====================

#[test]
fn test_render_status_row_fills_background() {
    let area = Rect::new(0, 0, 20, 1);
    let mut entry = OverlayEntry::new(100, area);

    render_status_row(&mut entry, "Loading...", 20, None, None, None);

    // Should have cells for background + text
    assert!(!entry.cells.is_empty());
}

#[test]
fn test_render_status_row_text_starts_at_label_col() {
    let area = Rect::new(0, 0, 20, 1);
    let mut entry = OverlayEntry::new(100, area);

    render_status_row(&mut entry, "Hi", 20, None, None, None);

    // First character 'H' should land at LABEL_COL
    let h_cell = entry
        .cells
        .iter()
        .find(|c| c.cell.symbol == 'H')
        .expect("'H' should be rendered");
    assert_eq!(h_cell.x, LABEL_COL);
}

#[test]
fn test_render_status_row_zero_width() {
    let area = Rect::new(0, 0, 0, 1);
    let mut entry = OverlayEntry::new(100, area);

    // Should not panic with width=0
    render_status_row(&mut entry, "text", 0, None, None, None);
}

#[test]
fn test_render_status_row_width_one() {
    let area = Rect::new(0, 0, 1, 1);
    let mut entry = OverlayEntry::new(100, area);

    // Should not panic with width=1
    render_status_row(&mut entry, "text", 1, None, None, None);
}

// ==================== render_options ====================

#[test]
fn test_render_options_empty() {
    let area = Rect::new(0, 0, 20, 5);
    let mut entry = OverlayEntry::new(100, area);
    let colors = DropdownColors {
        fg: None,
        bg: None,
        selected_fg: None,
        selected_bg: None,
        highlight_fg: None,
        disabled_fg: None,
    };

    render_options(&mut entry, &[], 20, &colors, 0);
    assert!(entry.cells.is_empty());
}

#[test]
fn test_render_options_single_option() {
    let area = Rect::new(0, 0, 20, 5);
    let mut entry = OverlayEntry::new(100, area);
    let colors = DropdownColors {
        fg: None,
        bg: None,
        selected_fg: None,
        selected_bg: None,
        highlight_fg: None,
        disabled_fg: None,
    };

    let options = vec![DropdownOption {
        label: "Option 1",
        is_highlighted: false,
        is_disabled: false,
        match_indices: HashSet::new(),
        indicator: ' ',
    }];

    render_options(&mut entry, &options, 20, &colors, 0);
    assert!(!entry.cells.is_empty());
}

#[test]
fn test_render_options_indicator_at_indicator_col() {
    let area = Rect::new(0, 0, 20, 5);
    let mut entry = OverlayEntry::new(100, area);
    let colors = DropdownColors {
        fg: None,
        bg: None,
        selected_fg: None,
        selected_bg: None,
        highlight_fg: None,
        disabled_fg: None,
    };

    let options = vec![DropdownOption {
        label: "Option",
        is_highlighted: true,
        is_disabled: false,
        match_indices: HashSet::new(),
        indicator: '›',
    }];

    render_options(&mut entry, &options, 20, &colors, 0);

    // Indicator char should be at INDICATOR_COL on row 0
    let indicator_cell = entry
        .cells
        .iter()
        .find(|c| c.x == INDICATOR_COL && c.y == 0 && c.cell.symbol == '›');
    assert!(indicator_cell.is_some());
}

#[test]
fn test_render_options_label_starts_at_label_col() {
    let area = Rect::new(0, 0, 20, 5);
    let mut entry = OverlayEntry::new(100, area);
    let colors = DropdownColors {
        fg: None,
        bg: None,
        selected_fg: None,
        selected_bg: None,
        highlight_fg: None,
        disabled_fg: None,
    };

    let options = vec![DropdownOption {
        label: "X",
        is_highlighted: false,
        is_disabled: false,
        match_indices: HashSet::new(),
        indicator: ' ',
    }];

    render_options(&mut entry, &options, 20, &colors, 0);

    // 'X' should land at LABEL_COL on row 0
    let x_cell = entry
        .cells
        .iter()
        .find(|c| c.cell.symbol == 'X' && c.y == 0)
        .expect("'X' should be rendered");
    assert_eq!(x_cell.x, LABEL_COL);
}

#[test]
fn test_render_options_highlighted() {
    use revue::style::Color;

    let area = Rect::new(0, 0, 20, 5);
    let mut entry = OverlayEntry::new(100, area);
    let colors = DropdownColors {
        fg: Some(Color::WHITE),
        bg: Some(Color::BLACK),
        selected_fg: Some(Color::BLACK),
        selected_bg: Some(Color::BLUE),
        highlight_fg: Some(Color::YELLOW),
        disabled_fg: None,
    };

    let options = vec![DropdownOption {
        label: "Highlighted",
        is_highlighted: true,
        is_disabled: false,
        match_indices: HashSet::new(),
        indicator: '›',
    }];

    render_options(&mut entry, &options, 20, &colors, 0);

    // Check indicator cell exists at (INDICATOR_COL, 0)
    let indicator_cell = entry
        .cells
        .iter()
        .find(|c| c.x == INDICATOR_COL && c.y == 0);
    assert!(indicator_cell.is_some());
}

#[test]
fn test_render_options_with_match_indices() {
    use revue::style::Color;

    let area = Rect::new(0, 0, 20, 1);
    let mut entry = OverlayEntry::new(100, area);
    let colors = DropdownColors {
        fg: Some(Color::WHITE),
        bg: None,
        selected_fg: None,
        selected_bg: None,
        highlight_fg: Some(Color::YELLOW),
        disabled_fg: None,
    };

    let mut indices = HashSet::new();
    indices.insert(0);
    indices.insert(2);

    let options = vec![DropdownOption {
        label: "abc",
        is_highlighted: false,
        is_disabled: false,
        match_indices: indices,
        indicator: ' ',
    }];

    render_options(&mut entry, &options, 20, &colors, 0);

    // Label cells live at x >= LABEL_COL
    let text_cells: Vec<_> = entry.cells.iter().filter(|c| c.x >= LABEL_COL).collect();
    assert!(!text_cells.is_empty());
}

#[test]
fn test_render_options_reserve_right_limits_text() {
    let area = Rect::new(0, 0, 10, 1);
    let mut entry = OverlayEntry::new(100, area);
    let colors = DropdownColors {
        fg: None,
        bg: None,
        selected_fg: None,
        selected_bg: None,
        highlight_fg: None,
        disabled_fg: None,
    };

    // width=10, reserve_right=1 → label_width = 10 - 2 - 1 = 7
    // Label "0123456789" should truncate to 7 chars
    let options = vec![DropdownOption {
        label: "0123456789",
        is_highlighted: false,
        is_disabled: false,
        match_indices: HashSet::new(),
        indicator: ' ',
    }];

    render_options(&mut entry, &options, 10, &colors, 1);

    // No label cell should appear at or past (width - reserve_right) = 9
    let max_label_x = entry
        .cells
        .iter()
        .filter(|c| c.x >= LABEL_COL && c.cell.symbol.is_ascii_digit())
        .map(|c| c.x)
        .max()
        .unwrap_or(0);
    assert!(max_label_x < 10 - 1, "label text leaked into scroll column");
}

// ==================== queue_or_inline_overlay ====================

#[test]
fn test_queue_or_inline_without_overlay_support() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    // RenderContext without overlay support (no overlay queue)
    let mut ctx = RenderContext::new(&mut buffer, area);

    let overlay_area = Rect::new(0, 0, 20, 1);
    let mut entry = OverlayEntry::new(100, overlay_area);
    let mut cell = revue::render::Cell::new('X');
    cell.fg = Some(revue::style::Color::WHITE);
    entry.push(0, 0, cell);

    // Should not panic — falls back to inline rendering
    queue_or_inline_overlay(&mut ctx, entry);
}
