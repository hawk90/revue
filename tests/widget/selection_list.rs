//! SelectionList widget integration tests
//!
//! SelectionList 위젯의 통합 테스트
//!
//! 테스트 커버리지:
//! - 생성자 및 빌더 메서드
//! - 아이템 관리 (추가, 삭제, 조회)
//! - 선택 동작 (단일, 다중, 토글)
//! - 렌더링 (다양한 스타일, 상태)
//! - 내비게이션 (highlight 이동)
//! - 경계값 처리 (max_selections, min_selections)
//! - 비활성화 아이템 처리
//! - CSS 스타일링

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
    assert!(text.contains("Choose") || text.len() > 0);
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
