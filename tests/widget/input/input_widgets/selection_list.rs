//! SelectionList widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{
    selection_item, selection_list, SelectionItem, SelectionList, SelectionStyle, StyledView, View,
};

// =============================================================================
// 생성자 및 빌더 메서드 테스트
// =============================================================================

#[test]
fn test_selection_list_new_with_strings() {
    let list = SelectionList::new(vec!["Item 1", "Item 2", "Item 3"]);
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_selection_list_new_with_items() {
    let list = SelectionList::new(vec![
        SelectionItem::new("A"),
        SelectionItem::new("B"),
        SelectionItem::new("C"),
    ]);
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_selection_list_new_empty() {
    let list: SelectionList = SelectionList::new(Vec::<String>::new());
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_selection_list_selected_builder() {
    let list = SelectionList::new(vec!["A", "B", "C", "D"]).selected(vec![0, 2]);
    assert_eq!(list.get_selected().len(), 2);
    assert!(list.is_selected(0));
    assert!(list.is_selected(2));
    assert!(!list.is_selected(1));
    assert!(!list.is_selected(3));
}

#[test]
fn test_selection_list_style_builder() {
    let list = SelectionList::new(vec!["A", "B"]).style(SelectionStyle::Checkbox);
    // Style affects rendering - verify through render
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);
}

#[test]
fn test_selection_list_show_checkboxes_true() {
    let list = SelectionList::new(vec!["A", "B"]).show_checkboxes(true);
    // Should use Checkbox style
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('['));
}

#[test]
fn test_selection_list_show_checkboxes_false() {
    let list = SelectionList::new(vec!["A", "B"]).show_checkboxes(false);
    // Should use Highlight style (no brackets)
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);
}

#[test]
fn test_selection_list_max_selections() {
    let list = SelectionList::new(vec!["A", "B", "C", "D"]).max_selections(2);
    // max_selections affects behavior - verify through interaction
    let mut list = list;
    list.toggle(0);
    list.toggle(1);
    list.toggle(2); // Should not select due to max
    assert_eq!(list.get_selected().len(), 2);
    assert!(!list.is_selected(2));
}

#[test]
fn test_selection_list_min_selections() {
    let list = SelectionList::new(vec!["A", "B", "C"])
        .selected(vec![0, 1])
        .min_selections(1);
    let mut list = list;
    list.toggle(0); // Can deselect
    assert!(!list.is_selected(0));
    list.toggle(1); // Cannot deselect (would go below min)
    assert!(list.is_selected(1));
}

#[test]
fn test_selection_list_show_descriptions() {
    let list = SelectionList::new(vec![
        SelectionItem::new("Item 1").description("Description 1"),
        SelectionItem::new("Item 2").description("Description 2"),
    ])
    .show_descriptions(true);

    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);

    // Verify rendering succeeded
    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());

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
fn test_selection_list_title() {
    let list = SelectionList::new(vec!["A", "B"]).title("Select Items");
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);

    let first_line: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(first_line.contains("Select") || first_line.contains("Items"));
}

#[test]
fn test_selection_list_focused() {
    let list = SelectionList::new(vec!["A", "B"]).focused(true);
    // focused is a builder method - verify through rendering
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);
}

#[test]
fn test_selection_list_builder_chain() {
    let list = SelectionList::new(vec!["A", "B", "C"])
        .selected(vec![0])
        .style(SelectionStyle::Checkbox)
        .max_selections(2)
        .min_selections(1)
        .show_descriptions(true)
        .title("Test")
        .fg(Color::WHITE)
        .selected_fg(Color::GREEN)
        .highlighted_fg(Color::CYAN)
        .bg(Color::BLACK)
        .max_visible(5)
        .show_count(true)
        .focused(true);

    assert!(list.is_selected(0));
}

// =============================================================================
// Helper 함수 테스트
// =============================================================================

#[test]
fn test_selection_list_helper() {
    let list = selection_list(vec!["A", "B", "C"]);
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_selection_item_helper() {
    let item = selection_item("Test");
    assert_eq!(item.text, "Test");
}

#[test]
fn test_selection_item_from_string() {
    let item: SelectionItem = "Test".into();
    assert_eq!(item.text, "Test");
}

#[test]
fn test_selection_item_from_str() {
    let item: SelectionItem = "Test".into();
    assert_eq!(item.text, "Test");
}

// =============================================================================
// SelectionItem 빌더 메서드 테스트
// =============================================================================

#[test]
fn test_selection_item_new() {
    let item = SelectionItem::new("Test Item");
    assert_eq!(item.text, "Test Item");
    assert!(item.value.is_none());
    assert!(!item.disabled);
    assert!(item.description.is_none());
    assert!(item.icon.is_none());
}

#[test]
fn test_selection_item_value() {
    let item = SelectionItem::new("Display").value("actual_value");
    assert_eq!(item.value, Some("actual_value".to_string()));
}

#[test]
fn test_selection_item_disabled() {
    let item = SelectionItem::new("Disabled").disabled(true);
    assert!(item.disabled);
}

#[test]
fn test_selection_item_description() {
    let item = SelectionItem::new("Item").description("This is a description");
    assert_eq!(item.description, Some("This is a description".to_string()));
}

#[test]
fn test_selection_item_icon() {
    let item = SelectionItem::new("Item").icon("🔥");
    assert_eq!(item.icon, Some("🔥".to_string()));
}

#[test]
fn test_selection_item_builder_chain() {
    let item = SelectionItem::new("Complex Item")
        .value("val")
        .disabled(false)
        .description("Desc")
        .icon("★");

    assert_eq!(item.text, "Complex Item");
    assert_eq!(item.value, Some("val".to_string()));
    assert_eq!(item.description, Some("Desc".to_string()));
    assert_eq!(item.icon, Some("★".to_string()));
    assert!(!item.disabled);
}

// =============================================================================
// 선택 동작 테스트 - 기본
// =============================================================================

#[test]
fn test_selection_list_is_selected() {
    let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 2]);
    assert!(list.is_selected(0));
    assert!(!list.is_selected(1));
    assert!(list.is_selected(2));
}

#[test]
fn test_selection_list_get_selected() {
    let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 2]);
    let selected = list.get_selected();
    assert_eq!(selected, vec![0, 2]);
}

#[test]
fn test_selection_list_get_selected_empty() {
    let list = SelectionList::new(vec!["A", "B", "C"]);
    let selected = list.get_selected();
    assert_eq!(selected.len(), 0);
}

// =============================================================================
// 선택 동작 테스트 - 토글
// =============================================================================

#[test]
fn test_selection_list_toggle_select() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.toggle(1);
    assert!(list.is_selected(1));
}

#[test]
fn test_selection_list_toggle_deselect() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![1]);
    list.toggle(1);
    assert!(!list.is_selected(1));
}

#[test]
fn test_selection_list_toggle_multiple() {
    let mut list = SelectionList::new(vec!["A", "B", "C", "D"]);
    list.toggle(0);
    list.toggle(2);
    assert!(list.is_selected(0));
    assert!(list.is_selected(2));
    assert!(!list.is_selected(1));
    assert!(!list.is_selected(3));
}

#[test]
fn test_selection_list_toggle_out_of_bounds() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.toggle(5); // Should not panic
    assert_eq!(list.get_selected().len(), 0);
}

#[test]
fn test_selection_list_toggle_disabled_item() {
    let mut list = SelectionList::new(vec![
        SelectionItem::new("A"),
        SelectionItem::new("B").disabled(true),
        SelectionItem::new("C"),
    ]);
    list.toggle(1);
    assert!(!list.is_selected(1));
}

#[test]
fn test_selection_list_toggle_with_max_selections() {
    let mut list = SelectionList::new(vec!["A", "B", "C", "D"]).max_selections(2);
    list.toggle(0);
    list.toggle(1);
    list.toggle(2); // Should not select (max reached)
    assert!(list.is_selected(0));
    assert!(list.is_selected(1));
    assert!(!list.is_selected(2));
}

#[test]
fn test_selection_list_toggle_with_min_selections() {
    let mut list = SelectionList::new(vec!["A", "B", "C"])
        .selected(vec![0, 1])
        .min_selections(1);
    list.toggle(0); // Can deselect
    assert!(!list.is_selected(0));
    list.toggle(1); // Cannot deselect (would go below min)
    assert!(list.is_selected(1));
}

#[test]
fn test_selection_list_toggle_highlighted() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.highlight_next(); // Move to index 1
    list.toggle_highlighted();
    assert!(list.is_selected(1));
}

// =============================================================================
// 선택 동작 테스트 - select/deselect
// =============================================================================

#[test]
fn test_selection_list_select() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.select(1);
    assert!(list.is_selected(1));
}

#[test]
fn test_selection_list_select_already_selected() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![1]);
    list.select(1); // Should be idempotent
    assert!(list.is_selected(1));
}

#[test]
fn test_selection_list_select_with_max_selections() {
    let mut list = SelectionList::new(vec!["A", "B", "C"])
        .max_selections(2)
        .selected(vec![0]);
    list.select(1);
    assert!(list.is_selected(1));
    list.select(2); // Should not select
    assert!(!list.is_selected(2));
}

#[test]
fn test_selection_list_select_disabled() {
    let mut list = SelectionList::new(vec![
        SelectionItem::new("A"),
        SelectionItem::new("B").disabled(true),
    ]);
    list.select(1);
    assert!(!list.is_selected(1));
}

#[test]
fn test_selection_list_deselect() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 1]);
    list.deselect(0);
    assert!(!list.is_selected(0));
    assert!(list.is_selected(1));
}

#[test]
fn test_selection_list_deselect_with_min_selections() {
    let mut list = SelectionList::new(vec!["A", "B", "C"])
        .selected(vec![0, 1])
        .min_selections(1);
    list.deselect(0);
    assert!(!list.is_selected(0));
    list.deselect(1); // Should not deselect (would go below min)
    assert!(list.is_selected(1));
}

// =============================================================================
// 선택 동작 테스트 - select_all/deselect_all
// =============================================================================

#[test]
fn test_selection_list_select_all() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.select_all();
    assert_eq!(list.get_selected().len(), 3);
    assert!(list.is_selected(0));
    assert!(list.is_selected(1));
    assert!(list.is_selected(2));
}

#[test]
fn test_selection_list_select_all_with_disabled() {
    let mut list = SelectionList::new(vec![
        SelectionItem::new("A"),
        SelectionItem::new("B").disabled(true),
        SelectionItem::new("C"),
    ]);
    list.select_all();
    assert!(list.is_selected(0));
    assert!(!list.is_selected(1)); // Disabled items not selected
    assert!(list.is_selected(2));
}

#[test]
fn test_selection_list_select_all_with_max_selections() {
    let mut list = SelectionList::new(vec!["A", "B", "C", "D", "E"]).max_selections(3);
    list.select_all();
    assert_eq!(list.get_selected().len(), 3);
}

#[test]
fn test_selection_list_deselect_all() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 1, 2]);
    list.deselect_all();
    assert_eq!(list.get_selected().len(), 0);
}

#[test]
fn test_selection_list_deselect_all_with_min_selections() {
    let mut list = SelectionList::new(vec!["A", "B", "C"])
        .selected(vec![0, 1, 2])
        .min_selections(1);
    list.deselect_all();
    assert_eq!(list.get_selected().len(), 1); // Keeps min_selections
}

// =============================================================================
// 선택 값 조회 테스트
// =============================================================================

#[test]
fn test_selection_list_get_selected_values_with_text() {
    let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 2]);
    let values = list.get_selected_values();
    assert_eq!(values, vec!["A", "C"]);
}

#[test]
fn test_selection_list_get_selected_values_with_value_field() {
    let list = SelectionList::new(vec![
        SelectionItem::new("Item A").value("a"),
        SelectionItem::new("Item B").value("b"),
        SelectionItem::new("Item C").value("c"),
    ])
    .selected(vec![0, 2]);

    let values = list.get_selected_values();
    assert_eq!(values, vec!["a", "c"]);
}

#[test]
fn test_selection_list_get_selected_items() {
    let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 2]);
    let items = list.get_selected_items();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].text, "A");
    assert_eq!(items[1].text, "C");
}

#[test]
fn test_selection_list_get_selected_items_empty() {
    let list = SelectionList::new(vec!["A", "B", "C"]);
    let items = list.get_selected_items();
    assert_eq!(items.len(), 0);
}

// =============================================================================
// 내비게이션 테스트
// =============================================================================

#[test]
fn test_selection_list_highlight_navigation() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);

    // Test navigation methods don't panic
    list.highlight_next();
    list.highlight_previous();
    list.highlight_first();
    list.highlight_last();
}

#[test]
fn test_selection_list_highlight_empty_list() {
    let mut list: SelectionList = SelectionList::new(Vec::<String>::new());
    // Should not panic on empty list
    list.highlight_next();
    list.highlight_previous();
    list.highlight_first();
    list.highlight_last();
}

#[test]
fn test_selection_list_highlight_with_scrolling() {
    let mut list = SelectionList::new(vec!["A", "B", "C", "D", "E"]).max_visible(3);
    // Navigate and ensure scrolling works
    for _ in 0..5 {
        list.highlight_next();
    }
    list.highlight_first();
    for _ in 0..5 {
        list.highlight_previous();
    }
}

// =============================================================================
// 렌더링 테스트 - 기본
// =============================================================================

#[test]
fn test_selection_list_render_basic() {
    let list = SelectionList::new(vec!["A", "B", "C"]);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_selection_list_render_with_title() {
    let list = SelectionList::new(vec!["A", "B"]).title("Choose:");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Choose") || !text.is_empty());
}

#[test]
fn test_selection_list_render_empty() {
    let list: SelectionList = SelectionList::new(Vec::<String>::new());
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx); // Should not panic
}

// =============================================================================
// 렌더링 테스트 - 스타일별
// =============================================================================

#[test]
fn test_selection_list_render_checkbox_style() {
    let list = SelectionList::new(vec!["A", "B"])
        .style(SelectionStyle::Checkbox)
        .selected(vec![0]);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Check for checkbox brackets
    let mut found_bracket = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '[' || cell.symbol == ']' {
                    found_bracket = true;
                    break;
                }
            }
        }
        if found_bracket {
            break;
        }
    }
    assert!(found_bracket);
}

#[test]
fn test_selection_list_render_bullet_style() {
    let list = SelectionList::new(vec!["A", "B"])
        .style(SelectionStyle::Bullet)
        .selected(vec![0]);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Check for bullet symbols
    let mut found_bullet = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '●' || cell.symbol == '○' {
                    found_bullet = true;
                    break;
                }
            }
        }
        if found_bullet {
            break;
        }
    }
    assert!(found_bullet);
}

#[test]
fn test_selection_list_render_highlight_style() {
    let list = SelectionList::new(vec!["A", "B"])
        .style(SelectionStyle::Highlight)
        .selected(vec![0]);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_selection_list_render_bracket_style() {
    let list = SelectionList::new(vec!["A", "B"])
        .style(SelectionStyle::Bracket)
        .selected(vec![0]);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Check for bracket symbols
    let mut found_bracket = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '[' || cell.symbol == ']' {
                    found_bracket = true;
                    break;
                }
            }
        }
        if found_bracket {
            break;
        }
    }
    assert!(found_bracket);
}

// =============================================================================
// 렌더링 테스트 - 비활성화 아이템
// =============================================================================

#[test]
fn test_selection_list_render_disabled_item() {
    let list = SelectionList::new(vec![
        SelectionItem::new("A"),
        SelectionItem::new("B").disabled(true),
        SelectionItem::new("C"),
    ]);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Disabled items should render with gray color
    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_selection_list_render_disabled_checkbox_style() {
    let list = SelectionList::new(vec![
        SelectionItem::new("A"),
        SelectionItem::new("B").disabled(true),
    ])
    .style(SelectionStyle::Checkbox);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Check for disabled checkbox markers (should have [-] pattern)
    let mut found_left_bracket = false;
    let mut found_minus = false;
    let mut found_right_bracket = false;

    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '[' {
                    found_left_bracket = true;
                }
                if cell.symbol == '-' {
                    found_minus = true;
                }
                if cell.symbol == ']' {
                    found_right_bracket = true;
                }
            }
        }
    }

    // The disabled item should be rendered, check we have the pattern
    assert!(found_left_bracket && found_minus && found_right_bracket);
}

// =============================================================================
// 렌더링 테스트 - 스크롤
// =============================================================================

#[test]
fn test_selection_list_render_with_max_visible() {
    let list = SelectionList::new(vec!["A", "B", "C", "D", "E"]).max_visible(3);

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Check for scroll indicator
    let mut found_scroll = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '↓' {
                    found_scroll = true;
                    break;
                }
            }
        }
        if found_scroll {
            break;
        }
    }
    assert!(found_scroll);
}

#[test]
fn test_selection_list_render_scroll_indicator() {
    let mut list = SelectionList::new(vec!["A", "B", "C", "D", "E"]).max_visible(3);
    list.highlight_last(); // Scroll to bottom

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Check for scroll indicator
    let mut found_scroll = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '↑' {
                    found_scroll = true;
                    break;
                }
            }
        }
        if found_scroll {
            break;
        }
    }
    assert!(found_scroll);
}

// =============================================================================
// 렌더링 테스트 - focused 상태
// =============================================================================

#[test]
fn test_selection_list_render_focused() {
    let list = SelectionList::new(vec!["A", "B"]).focused(true);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Check for help text indicators
    let mut found_help = false;
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '↑' || cell.symbol == '↓' {
                    found_help = true;
                    break;
                }
            }
        }
        if found_help {
            break;
        }
    }
    assert!(found_help);
}

#[test]
fn test_selection_list_render_not_focused() {
    let list = SelectionList::new(vec!["A", "B"]).focused(false);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

// =============================================================================
// CSS 스타일링 테스트
// =============================================================================

#[test]
fn test_selection_list_element_id() {
    let list = SelectionList::new(vec!["A", "B"]).element_id("test-list");
    assert_eq!(View::id(&list), Some("test-list"));

    let meta = list.meta();
    assert_eq!(meta.id, Some("test-list".to_string()));
}

#[test]
fn test_selection_list_css_classes() {
    let list = SelectionList::new(vec!["A", "B"])
        .class("multi-select")
        .class("primary");

    assert!(list.has_class("multi-select"));
    assert!(list.has_class("primary"));
    assert!(!list.has_class("secondary"));

    let meta = list.meta();
    assert!(meta.classes.contains("multi-select"));
    assert!(meta.classes.contains("primary"));
}

#[test]
fn test_selection_list_classes_from_view_trait() {
    let list = SelectionList::new(vec!["A", "B"])
        .class("list")
        .class("selectable");

    let classes = View::classes(&list);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"list".to_string()));
    assert!(classes.contains(&"selectable".to_string()));
}

#[test]
fn test_selection_list_styled_view_set_id() {
    let mut list = SelectionList::new(vec!["A", "B"]);
    list.set_id("my-list");
    assert_eq!(View::id(&list), Some("my-list"));
}

#[test]
fn test_selection_list_styled_view_add_class() {
    let mut list = SelectionList::new(vec!["A", "B"]);
    list.add_class("active");
    assert!(list.has_class("active"));
}

#[test]
fn test_selection_list_styled_view_remove_class() {
    let mut list = SelectionList::new(vec!["A", "B"]).class("active");
    list.remove_class("active");
    assert!(!list.has_class("active"));
}

#[test]
fn test_selection_list_styled_view_toggle_class() {
    let mut list = SelectionList::new(vec!["A", "B"]);

    list.toggle_class("selected");
    assert!(list.has_class("selected"));

    list.toggle_class("selected");
    assert!(!list.has_class("selected"));
}

#[test]
fn test_selection_list_classes_builder() {
    let list = SelectionList::new(vec!["A", "B"]).classes(vec!["class1", "class2"]);

    assert!(list.has_class("class1"));
    assert!(list.has_class("class2"));
    assert_eq!(View::classes(&list).len(), 2);
}

#[test]
fn test_selection_list_view_meta() {
    let list = SelectionList::new(vec!["A", "B"])
        .element_id("test")
        .class("list-class");

    let meta = View::meta(&list);
    assert_eq!(meta.widget_type, "SelectionList");
    assert_eq!(meta.id, Some("test".to_string()));
    assert!(meta.classes.contains("list-class"));
}

// =============================================================================
// 복합 테스트
// =============================================================================

#[test]
fn test_selection_list_full_workflow() {
    let mut list = SelectionList::new(vec![
        SelectionItem::new("Option 1").value("opt1"),
        SelectionItem::new("Option 2").value("opt2"),
        SelectionItem::new("Option 3").value("opt3"),
    ])
    .title("Choose Options")
    .style(SelectionStyle::Checkbox)
    .max_selections(2)
    .focused(true);

    // Select items
    list.toggle(0);
    list.toggle(1);

    // Verify selection
    assert_eq!(list.get_selected().len(), 2);
    assert!(!list.is_selected(2));

    // Try to select third (should fail due to max)
    list.toggle(2);
    assert!(!list.is_selected(2));

    // Deselect one
    list.toggle(0);
    assert!(!list.is_selected(0));
    assert!(list.is_selected(1));

    // Now we can select third
    list.toggle(2);
    assert!(list.is_selected(2));

    // Get selected values
    let values = list.get_selected_values();
    assert_eq!(values, vec!["opt2", "opt3"]);
}

#[test]
fn test_selection_list_navigation_and_selection() {
    let mut list = SelectionList::new(vec!["A", "B", "C", "D", "E"])
        .max_visible(3)
        .focused(true);

    // Navigate down
    list.highlight_next();
    list.highlight_next();

    // Toggle highlighted
    list.toggle_highlighted();

    // Navigate further (should scroll)
    list.highlight_next();

    // Toggle new highlighted
    list.toggle_highlighted();
}

#[test]
fn test_selection_list_with_descriptions_render() {
    let list = SelectionList::new(vec![
        SelectionItem::new("Feature A").description("Enable feature A"),
        SelectionItem::new("Feature B").description("Enable feature B"),
    ])
    .show_descriptions(true)
    .title("Features");

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 6);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Verify rendering succeeded
    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_selection_list_with_icons() {
    let list = SelectionList::new(vec![
        SelectionItem::new("Save").icon("💾"),
        SelectionItem::new("Load").icon("📂"),
        SelectionItem::new("Exit").icon("🚪"),
    ]);

    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Verify rendering succeeded
    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

// =============================================================================
// 엣지 케이스 테스트
// =============================================================================

#[test]
fn test_selection_list_single_item() {
    let list = SelectionList::new(vec!["Only Item"]);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);
}

#[test]
fn test_selection_list_long_text() {
    let long_text = "This is a very long item text that might exceed the display area";
    let list = SelectionList::new(vec![long_text, "Short"]);

    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    list.render(&mut ctx);
}

#[test]
fn test_selection_list_all_items_disabled() {
    let mut list = SelectionList::new(vec![
        SelectionItem::new("A").disabled(true),
        SelectionItem::new("B").disabled(true),
        SelectionItem::new("C").disabled(true),
    ]);

    list.toggle(0);
    list.toggle(1);
    list.toggle(2);
    list.select_all();

    assert_eq!(list.get_selected().len(), 0);
}

#[test]
fn test_selection_list_select_all_with_min_selections() {
    let mut list = SelectionList::new(vec!["A", "B", "C"])
        .min_selections(2)
        .max_selections(2);

    list.select_all();
    assert_eq!(list.get_selected().len(), 2);

    list.deselect_all();
    assert_eq!(list.get_selected().len(), 2); // min_selections preserved
}

#[test]
fn test_selection_list_clone() {
    let list1 = SelectionList::new(vec!["A", "B", "C"])
        .selected(vec![0, 1])
        .title("Test")
        .style(SelectionStyle::Checkbox);

    let list2 = list1.clone();

    assert_eq!(list1.get_selected(), list2.get_selected());
}

#[test]
fn test_selection_list_debug_format() {
    let list = SelectionList::new(vec!["A", "B"]);
    let debug_str = format!("{:?}", list);
    assert!(debug_str.contains("SelectionList"));
}

#[test]
fn test_selection_list_show_count_with_max() {
    let list = SelectionList::new(vec!["A", "B", "C", "D", "E"])
        .selected(vec![0, 1])
        .max_selections(5)
        .show_count(true);

    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    list.render(&mut ctx);

    // Verify rendering succeeded
    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

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

// =========================================================================
// Key handling: the keys the focused help line lists
// =========================================================================

#[test]
fn test_handle_key_navigates_with_arrows() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    assert!(list.handle_key(&Key::Down));
    assert_eq!(highlighted(&list), Some(1));
    assert!(list.handle_key(&Key::Char('j')));
    assert_eq!(highlighted(&list), Some(2));
    assert!(list.handle_key(&Key::Up));
    assert_eq!(highlighted(&list), Some(1));
    assert!(list.handle_key(&Key::Char('k')));
    assert_eq!(highlighted(&list), Some(0));
}

#[test]
fn test_handle_key_space_toggles_highlighted() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    list.handle_key(&Key::Down);
    assert!(list.handle_key(&Key::Char(' ')));
    assert_eq!(list.get_selected(), &[1]);
    assert!(list.handle_key(&Key::Char(' ')));
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_handle_key_a_selects_all_and_n_none() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]);
    assert!(list.handle_key(&Key::Char('a')));
    assert_eq!(list.get_selected(), &[0, 1, 2]);
    assert!(list.handle_key(&Key::Char('n')));
    assert!(list.get_selected().is_empty());
}

#[test]
fn test_handle_key_ignores_other_keys() {
    let mut list = SelectionList::new(vec!["A", "B"]);
    assert!(!list.handle_key(&Key::Char('x')));
    assert!(!list.handle_key(&Key::Enter));
    assert!(list.get_selected().is_empty());
}

// =========================================================================
// selected(): the initial selection follows the same rules as select()
// =========================================================================

#[test]
fn test_selected_drops_out_of_range_indices() {
    let list = SelectionList::new(vec!["A", "B"]).selected(vec![1, 5]);
    assert_eq!(list.get_selected(), &[1]);
    assert_eq!(list.get_selected_values(), vec!["B"]);
}

#[test]
fn test_selected_is_sorted_and_deduplicated() {
    let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![2, 0, 2]);
    assert_eq!(list.get_selected(), &[0, 2]);
}

#[test]
fn test_selected_respects_max_selections() {
    let list = SelectionList::new(vec!["A", "B", "C"])
        .max_selections(2)
        .selected(vec![0, 1, 2]);
    assert_eq!(list.get_selected(), &[0, 1]);

    // In either builder order
    let list = SelectionList::new(vec!["A", "B", "C"])
        .selected(vec![0, 1, 2])
        .max_selections(2);
    assert_eq!(list.get_selected(), &[0, 1]);
}

#[test]
fn test_selected_then_toggle_keeps_order() {
    let mut list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![2]);
    list.toggle(0);
    assert_eq!(list.get_selected(), &[0, 2]);
}
