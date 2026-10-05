//! Tests for the SelectionList widget, through its public API
//!
//! tests/widget/selection_list.rs covers selection (toggle, select, min/max,
//! select_all); these cover the item/style types, the remaining builders,
//! highlight navigation and the rendered item prefixes.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{
    selection_item, selection_list, SelectionItem, SelectionList, SelectionStyle, View,
};

fn render(list: &SelectionList, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    list.render(&mut ctx);
    buffer
}

/// The non-blank rendered rows, trimmed, top to bottom. (The vstack the list
/// renders through spreads its children over the area, so blank rows between
/// them are dropped.)
fn rows(list: &SelectionList) -> Vec<String> {
    let (w, h) = (40, 30);
    let buffer = render(list, w, h);
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .filter(|r| !r.is_empty())
        .collect()
}

/// Foreground of the first cell of the rendered row that ends with `text`.
fn row_fg(buffer: &Buffer, width: u16, height: u16, text: &str) -> Option<Color> {
    let y = (0..height)
        .find(|&y| {
            let row: String = (0..width)
                .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                .collect();
            row.trim_end().ends_with(text)
        })
        .unwrap_or_else(|| panic!("no row ending with {text:?}"));
    buffer.get(0, y).unwrap().fg
}

/// The first rendered row (the first item, when there is no title).
fn first_row(list: &SelectionList) -> String {
    rows(list).into_iter().next().unwrap_or_default()
}

/// Index of the highlighted item: the one `toggle_highlighted` flips.
fn highlighted(list: &SelectionList) -> Option<usize> {
    let mut probe = list.clone();
    let before = probe.get_selected().to_vec();
    probe.toggle_highlighted();
    let after = probe.get_selected().to_vec();
    (0..list.get_selected().len() + 100).find(|i| before.contains(i) != after.contains(i))
}

// =========================================================================
// SelectionItem tests
// =========================================================================

#[test]
fn test_selection_item_new_with_string() {
    let item = SelectionItem::new(String::from("Owned String"));
    assert_eq!(item.text, "Owned String");
}

#[test]
fn test_selection_item_value_string() {
    let item = SelectionItem::new("Item").value(String::from("owned"));
    assert_eq!(item.value, Some("owned".to_string()));
}

#[test]
fn test_selection_item_disabled_false() {
    let item = SelectionItem::new("Item").disabled(true).disabled(false);
    assert!(!item.disabled);
}

#[test]
fn test_selection_item_description_string() {
    let item = SelectionItem::new("Item").description(String::from("Owned desc"));
    assert_eq!(item.description, Some("Owned desc".to_string()));
}

#[test]
fn test_selection_item_icon_string() {
    let item = SelectionItem::new("Item").icon(String::from("⚙"));
    assert_eq!(item.icon, Some("⚙".to_string()));
}

#[test]
fn test_selection_item_clone() {
    let item1 = SelectionItem::new("Test")
        .value("v")
        .disabled(true)
        .description("desc")
        .icon("icon");
    let item2 = item1.clone();

    assert_eq!(item1.text, item2.text);
    assert_eq!(item1.value, item2.value);
    assert_eq!(item1.disabled, item2.disabled);
    assert_eq!(item1.description, item2.description);
    assert_eq!(item1.icon, item2.icon);
}

#[test]
fn test_selection_item_debug() {
    let item = SelectionItem::new("Debug Test");
    let debug_str = format!("{:?}", item);
    assert!(debug_str.contains("Debug Test"));
}

#[test]
fn test_selection_item_helper_chain() {
    let item = selection_item("Test").value("val").description("desc");
    assert_eq!(item.text, "Test");
    assert_eq!(item.value, Some("val".to_string()));
    assert_eq!(item.description, Some("desc".to_string()));
}

// =========================================================================
// SelectionStyle tests
// =========================================================================

#[test]
fn test_selection_style_default() {
    assert_eq!(SelectionStyle::default(), SelectionStyle::Checkbox);
}

#[test]
fn test_selection_style_copy() {
    let style1 = SelectionStyle::Highlight;
    let style2 = style1;
    assert_eq!(style1, SelectionStyle::Highlight);
    assert_eq!(style2, SelectionStyle::Highlight);
}

#[test]
fn test_selection_style_partial_eq() {
    let styles = [
        SelectionStyle::Checkbox,
        SelectionStyle::Bullet,
        SelectionStyle::Highlight,
        SelectionStyle::Bracket,
    ];
    for (i, a) in styles.iter().enumerate() {
        for (j, b) in styles.iter().enumerate() {
            assert_eq!(a == b, i == j, "{a:?} vs {b:?}");
        }
    }
}

#[test]
fn test_selection_style_debug() {
    let debug_str = format!("{:?}", SelectionStyle::Checkbox);
    assert!(debug_str.contains("Checkbox"));
}

// =========================================================================
// SelectionList constructor and builder tests
// =========================================================================

#[test]
fn test_selection_list_new() {
    let list = SelectionList::new(vec!["A", "B", "C"]);
    assert!(list.get_selected().is_empty());
    assert_eq!(highlighted(&list), Some(0));
    // Checkbox style, no title, count or help line, no visible-item limit.
    assert_eq!(rows(&list), vec!["[ ] A", "[ ] B", "[ ] C"]);
    // No max or min selections.
    let mut list = list;
    list.select_all();
    assert_eq!(list.get_selected().len(), 3);
    list.deselect_all();
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_selection_list_new_with_owned_strings() {
    let list = SelectionList::new(vec![String::from("String"), String::from("Owned")]);
    assert_eq!(rows(&list), vec!["[ ] String", "[ ] Owned"]);
}

#[test]
fn test_selection_list_style() {
    let list = SelectionList::new(vec!["A"]).style(SelectionStyle::Bullet);
    assert_eq!(first_row(&list), "○ A");
}

#[test]
fn test_selection_list_show_checkboxes_false_uses_highlight_style() {
    let list = SelectionList::new(vec!["A"])
        .show_checkboxes(false)
        .selected(vec![0]);
    assert_eq!(first_row(&list), "▸ A");
}

#[test]
fn test_selection_list_show_descriptions() {
    let items = || vec![SelectionItem::new("A").description("about A")];
    assert_eq!(
        rows(&SelectionList::new(items()).show_descriptions(true)),
        vec!["[ ] A", "    about A"]
    );
    assert_eq!(
        rows(&SelectionList::new(items()).show_descriptions(false)),
        vec!["[ ] A"]
    );
}

#[test]
fn test_selection_list_title_string() {
    let list = SelectionList::new(vec!["A"]).title(String::from("Title"));
    assert_eq!(rows(&list), vec!["Title", "[ ] A"]);
}

#[test]
fn test_selection_list_fg() {
    let list = SelectionList::new(vec!["A"]).fg(Color::RED);
    assert_eq!(render(&list, 10, 2).get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_selection_list_selected_fg() {
    let list = SelectionList::new(vec!["A", "B"])
        .selected(vec![0])
        .selected_fg(Color::MAGENTA);
    let buffer = render(&list, 10, 3);
    assert_eq!(row_fg(&buffer, 10, 3, "] A"), Some(Color::MAGENTA));
    assert_ne!(row_fg(&buffer, 10, 3, "] B"), Some(Color::MAGENTA));
}

#[test]
fn test_selection_list_highlighted_fg() {
    let list = SelectionList::new(vec!["A", "B"])
        .focused(true)
        .highlighted_fg(Color::YELLOW);
    let buffer = render(&list, 60, 4);
    assert_eq!(row_fg(&buffer, 60, 4, "] A"), Some(Color::YELLOW));
    assert_ne!(row_fg(&buffer, 60, 4, "] B"), Some(Color::YELLOW));
}

#[test]
fn test_selection_list_highlight_needs_focus() {
    let list = SelectionList::new(vec!["A"]).highlighted_fg(Color::YELLOW);
    assert_ne!(
        row_fg(&render(&list, 10, 2), 10, 2, "] A"),
        Some(Color::YELLOW)
    );
}

#[test]
fn test_selection_list_bg() {
    let list = SelectionList::new(vec![SelectionItem::new("A").description("about A")])
        .show_descriptions(true)
        .bg(Color::BLUE);
    let buffer = render(&list, 10, 2);
    // Item and description rows both take the background.
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '[');
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(4, 1).unwrap().symbol, 'a');
    assert_eq!(buffer.get(4, 1).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_selection_list_max_visible() {
    let list = SelectionList::new(vec!["A", "B", "C"]).max_visible(2);
    assert_eq!(rows(&list), vec!["[ ] A", "[ ] B", "  ↓ more..."]);
}

#[test]
fn test_selection_list_show_count() {
    let list = SelectionList::new(vec!["A", "B"]).selected(vec![1]);
    assert_eq!(
        rows(&list.clone().show_count(true)),
        vec!["Selected: 1", "[ ] A", "[x] B"]
    );
    assert_eq!(rows(&list.show_count(false)), vec!["[ ] A", "[x] B"]);
}

#[test]
fn test_selection_list_focused_shows_help() {
    let list = SelectionList::new(vec!["A"]);
    let focused = rows(&list.clone().focused(true));
    assert_eq!(focused.len(), 2);
    assert!(focused[1].starts_with("↑↓: Navigate"));
    assert_eq!(rows(&list.focused(false)), vec!["[ ] A"]);
}

// =========================================================================
// State mutation tests
// =========================================================================

#[test]
fn test_selection_select_out_of_bounds() {
    let mut list = SelectionList::new(vec!["A", "B"]);
    list.select(10); // Should not panic
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_selection_list_toggle_three_times() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.toggle(1);
    list.toggle(1);
    list.toggle(1);
    assert_eq!(list.get_selected(), &[1]);
}

// =========================================================================
// Navigation tests
// =========================================================================

#[test]
fn test_navigation() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    assert_eq!(highlighted(&list), Some(0));
    list.highlight_next();
    assert_eq!(highlighted(&list), Some(1));
    list.highlight_previous();
    assert_eq!(highlighted(&list), Some(0));
    list.highlight_last();
    assert_eq!(highlighted(&list), Some(2));
    list.highlight_first();
    assert_eq!(highlighted(&list), Some(0));
}

#[test]
fn test_highlight_next_at_end() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.highlight_last();
    list.highlight_next();
    assert_eq!(highlighted(&list), Some(2)); // Stays at end
}

#[test]
fn test_highlight_previous_at_start() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.highlight_previous();
    assert_eq!(highlighted(&list), Some(0)); // Stays at start
}

#[test]
fn test_highlight_first_scrolls_to_top() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]).max_visible(1);
    list.highlight_last();
    assert_eq!(rows(&list), vec!["  ↑ more...", "[ ] C"]);
    list.highlight_first();
    assert_eq!(highlighted(&list), Some(0));
    assert_eq!(rows(&list), vec!["[ ] A", "  ↓ more..."]);
}

#[test]
fn test_highlight_empty_list() {
    let mut list = SelectionList::new(Vec::<&str>::new());
    list.highlight_next();
    list.highlight_previous();
    list.highlight_last();
    assert_eq!(highlighted(&list), None);
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_selection_list_scroll_with_max_visible() {
    let mut list = SelectionList::new((0..20).map(|i| format!("Item {}", i))).max_visible(5);
    list.highlight_last();
    assert_eq!(
        rows(&list),
        vec![
            "  ↑ more...",
            "[ ] Item 15",
            "[ ] Item 16",
            "[ ] Item 17",
            "[ ] Item 18",
            "[ ] Item 19",
        ]
    );
    // Moving back up past the window scrolls one row at a time.
    for _ in 0..5 {
        list.highlight_previous();
    }
    assert_eq!(rows(&list)[1], "[ ] Item 14");
}

// =========================================================================
// Rendered item prefix/suffix tests
// =========================================================================

fn styled(style: SelectionStyle, selected: bool) -> String {
    let list = SelectionList::new(vec!["A", "B"]).style(style);
    let list = if selected {
        list.selected(vec![0])
    } else {
        list
    };
    first_row(&list)
}

fn styled_disabled(style: SelectionStyle) -> String {
    first_row(&SelectionList::new(vec![SelectionItem::new("A").disabled(true)]).style(style))
}

#[test]
fn test_item_prefix_checkbox() {
    assert_eq!(styled(SelectionStyle::Checkbox, true), "[x] A");
    assert_eq!(styled(SelectionStyle::Checkbox, false), "[ ] A");
    assert_eq!(styled_disabled(SelectionStyle::Checkbox), "[-] A");
}

#[test]
fn test_item_prefix_bullet() {
    assert_eq!(styled(SelectionStyle::Bullet, true), "● A");
    assert_eq!(styled(SelectionStyle::Bullet, false), "○ A");
    assert_eq!(styled_disabled(SelectionStyle::Bullet), "◌ A");
}

#[test]
fn test_item_prefix_highlight() {
    assert_eq!(styled(SelectionStyle::Highlight, true), "▸ A");
    assert_eq!(styled(SelectionStyle::Highlight, false), "  A");
}

#[test]
fn test_item_prefix_and_suffix_bracket() {
    assert_eq!(styled(SelectionStyle::Bracket, true), "[A]");
    assert_eq!(styled(SelectionStyle::Bracket, false), " A");
}

#[test]
fn test_item_icon_follows_prefix() {
    let list = SelectionList::new(vec![
        SelectionItem::new("Apple").icon("🍎"),
        SelectionItem::new("Banana").icon("*"),
    ]);
    let rows = rows(&list);
    assert!(rows[0].starts_with("[ ] 🍎"), "{rows:?}");
    assert!(rows[0].ends_with("Apple"), "{rows:?}");
    assert_eq!(rows[1], "[ ] *Banana");
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_selection_list_helper_with_strings() {
    let mut list = selection_list(vec![String::from("A"), String::from("B")]);
    list.select_all();
    assert_eq!(list.get_selected_values(), vec!["A", "B"]);
}

#[test]
fn test_selection_list_helper_with_items() {
    let mut list = selection_list(vec![selection_item("A").value("a"), selection_item("B")]);
    list.select_all();
    assert_eq!(list.get_selected_values(), vec!["a", "B"]);
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_selection_list_unicode_items() {
    let list = SelectionList::new(vec!["사과", "바나나", "체리"]).selected(vec![0]);
    assert_eq!(list.get_selected_values(), vec!["사과"]);
    assert!(first_row(&list).starts_with("[x] 사"));
}

#[test]
fn test_selection_list_long_descriptions() {
    let long_desc = "A".repeat(1000);
    let list = SelectionList::new(vec![
        SelectionItem::new("Item").description(long_desc.clone())
    ])
    .selected(vec![0]);
    assert_eq!(list.get_selected_items()[0].description, Some(long_desc));
}

#[test]
fn test_selection_list_empty_item_text() {
    let list = SelectionList::new(vec![""]).selected(vec![0]);
    assert_eq!(list.get_selected_values(), vec![""]);
    assert_eq!(first_row(&list), "[x]");
}
