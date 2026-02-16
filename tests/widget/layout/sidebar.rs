//! Tests for sidebar layout widget

use crate::widget::layout::sidebar::{CollapseMode, Sidebar, SidebarItem, SidebarSection};
use crate::widget::layout::sidebar::state::SidebarState;

// =========================================================================
// Sidebar::new tests
// =========================================================================

#[test]
fn test_sidebar_new() {
    let sidebar = Sidebar::new();
    assert!(sidebar.sections.is_empty());
    assert!(sidebar.selected.is_none());
    assert_eq!(sidebar.hovered, 0);
    assert_eq!(sidebar.collapse_mode, CollapseMode::Expanded);
    assert_eq!(sidebar.expanded_width, 20);
    assert_eq!(sidebar.collapsed_width, 5);
    assert_eq!(sidebar.collapse_threshold, 15);
}

#[test]
fn test_sidebar_new_with_collapse_mode() {
    let sidebar = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert_eq!(sidebar.collapse_mode, CollapseMode::Collapsed);
}

#[test]
fn test_sidebar_new_with_widths() {
    let sidebar = Sidebar::new()
        .expanded_width(30)
        .collapsed_width(10);
    assert_eq!(sidebar.expanded_width, 30);
    assert_eq!(sidebar.collapsed_width, 10);
}

#[test]
fn test_sidebar_new_with_threshold() {
    let sidebar = Sidebar::new().collapse_threshold(20);
    assert_eq!(sidebar.collapse_threshold, 20);
}

// =========================================================================
// Sidebar::header tests
// =========================================================================

#[test]
fn test_sidebar_header() {
    let sidebar = Sidebar::new().header("Test Header");
    assert_eq!(sidebar.header.as_deref(), Some("Test Header"));
}

#[test]
fn test_sidebar_header_empty() {
    let sidebar = Sidebar::new().header("");
    assert_eq!(sidebar.header.as_deref(), Some(""));
}

#[test]
fn test_sidebar_header_with_string() {
    let sidebar = Sidebar::new().header(String::from("Header"));
    assert_eq!(sidebar.header.as_deref(), Some("Header"));
}

// =========================================================================
// Sidebar::footer tests
// =========================================================================

#[test]
fn test_sidebar_footer() {
    let sidebar = Sidebar::new().footer("Test Footer");
    assert_eq!(sidebar.footer.as_deref(), Some("Test Footer"));
}

#[test]
fn test_sidebar_footer_empty() {
    let sidebar = Sidebar::new().footer("");
    assert_eq!(sidebar.footer.as_deref(), Some(""));
}

// =========================================================================
// Sidebar::section tests
// =========================================================================

#[test]
fn test_sidebar_section() {
    let section = SidebarSection::new(vec![SidebarItem::new("item1", "Item 1")]);
    let sidebar = Sidebar::new().section(section);
    assert_eq!(sidebar.sections.len(), 1);
    assert_eq!(sidebar.sections[0].items.len(), 1);
    assert_eq!(sidebar.sections[0].items[0].id, "item1");
}

#[test]
fn test_sidebar_section_with_string() {
    let section = SidebarSection::titled("Section", vec![SidebarItem::new("a", "A")]);
    let sidebar = Sidebar::new().section(section);
    assert_eq!(sidebar.sections[0].title.as_deref(), Some("Section"));
}

#[test]
fn test_sidebar_section_multiple() {
    let section1 = SidebarSection::new(vec![SidebarItem::new("a", "A")]);
    let section2 = SidebarSection::titled("Section B", vec![SidebarItem::new("b", "B")]);
    let sidebar = Sidebar::new()
        .section(section1)
        .section(section2);
    assert_eq!(sidebar.sections.len(), 2);
    assert!(sidebar.sections[0].title.is_none());
    assert_eq!(sidebar.sections[1].title.as_deref(), Some("Section B"));
}

#[test]
fn test_sidebar_section_empty() {
    let sidebar = Sidebar::new().section(SidebarSection::new(vec![]));
    assert_eq!(sidebar.sections.len(), 1);
    assert!(sidebar.sections[0].items.is_empty());
}

// =========================================================================
// Sidebar::width tests
// =========================================================================

#[test]
fn test_sidebar_width_builder() {
    let sidebar = Sidebar::new()
        .expanded_width(30)
        .collapsed_width(10)
        .collapse_threshold(20);
    assert_eq!(sidebar.expanded_width, 30);
    assert_eq!(sidebar.collapsed_width, 10);
    assert_eq!(sidebar.collapse_threshold, 20);
}

#[test]
fn test_sidebar_width_zero() {
    let sidebar = Sidebar::new()
        .expanded_width(0)
        .collapsed_width(0);
    assert_eq!(sidebar.expanded_width, 0);
    assert_eq!(sidebar.collapsed_width, 0);
}

// =========================================================================
// Sidebar::selected_id tests
// =========================================================================

#[test]
fn test_selected_id_none() {
    let sidebar = Sidebar::new();
    assert!(sidebar.selected_id().is_none());
}

#[test]
fn test_selected_id_some() {
    let mut sidebar = Sidebar::new();
    sidebar.selected = Some("test_id".to_string());
    assert_eq!(sidebar.selected_id(), Some("test_id"));
}

// =========================================================================
// Sidebar::hovered_index tests
// =========================================================================

#[test]
fn test_hovered_index_default() {
    let sidebar = Sidebar::new();
    assert_eq!(sidebar.hovered_index(), 0);
}

#[test]
fn test_hovered_index_custom() {
    let mut sidebar = Sidebar::new();
    sidebar.hovered = 5;
    assert_eq!(sidebar.hovered_index(), 5);
}

// =========================================================================
// Sidebar::is_collapsed tests
// =========================================================================

#[test]
fn test_is_collapsed_expanded_mode() {
    let sidebar = Sidebar::new().collapse_mode(CollapseMode::Expanded);
    assert!(!sidebar.is_collapsed());
}

#[test]
fn test_is_collapsed_collapsed_mode() {
    let sidebar = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert!(sidebar.is_collapsed());
}

#[test]
fn test_is_collapsed_auto_mode() {
    let sidebar = Sidebar::new().collapse_mode(CollapseMode::Auto);
    // Auto mode returns false in is_collapsed (determined at render time)
    assert!(!sidebar.is_collapsed());
}

// =========================================================================
// Sidebar::current_width tests
// =========================================================================

#[test]
fn test_current_width_expanded() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Expanded)
        .expanded_width(20)
        .collapsed_width(5);
    assert_eq!(sidebar.current_width(), 20);
}

#[test]
fn test_current_width_collapsed() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .expanded_width(20)
        .collapsed_width(5);
    assert_eq!(sidebar.current_width(), 5);
}

#[test]
fn test_current_width_auto() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Auto)
        .expanded_width(20)
        .collapsed_width(5);
    // Auto mode returns expanded_width (actual determination at render time)
    assert_eq!(sidebar.current_width(), 20);
}

// =========================================================================
// Sidebar::visible_items tests
// =========================================================================

#[test]
fn test_visible_items_empty() {
    let sidebar = Sidebar::new();
    let items = sidebar.visible_items();
    assert!(items.is_empty());
}

#[test]
fn test_visible_items_single_section_no_title() {
    let sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2"),
    ]));

    let items = sidebar.visible_items();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_visible_items_with_section_title() {
    let sidebar = Sidebar::new().section(SidebarSection::titled(
        "Section 1",
        vec![SidebarItem::new("item1", "Item 1")],
    ));

    let items = sidebar.visible_items();
    assert_eq!(items.len(), 2); // 1 section title + 1 item
}

#[test]
fn test_visible_items_multiple_sections() {
    let sidebar = Sidebar::new()
        .section(SidebarSection::titled(
            "Section 1",
            vec![SidebarItem::new("item1", "Item 1")],
        ))
        .section(SidebarSection::titled(
            "Section 2",
            vec![SidebarItem::new("item2", "Item 2")],
        ));

    let items = sidebar.visible_items();
    assert_eq!(items.len(), 4); // 2 section titles + 2 items
}

#[test]
fn test_visible_items_nested_children() {
    let mut item1 = SidebarItem::new("item1", "Item 1");
    item1.children.push(SidebarItem::new("child1", "Child 1"));

    let sidebar = Sidebar::new().section(SidebarSection::new(vec![item1.clone()]));

    // When not expanded, only parent should be visible
    let items = sidebar.visible_items();
    assert_eq!(items.len(), 1);
}

#[test]
fn test_visible_items_expanded_children() {
    let mut item1 = SidebarItem::new("item1", "Item 1");
    item1.expanded = true;
    item1.children.push(SidebarItem::new("child1", "Child 1"));

    let sidebar = Sidebar::new().section(SidebarSection::new(vec![item1]));

    let items = sidebar.visible_items();
    assert_eq!(items.len(), 2); // Parent + child
}

// =========================================================================
// Sidebar::flatten_item tests
// =========================================================================

#[test]
fn test_flatten_item_no_children() {
    let sidebar = Sidebar::new();
    let item = SidebarItem::new("item1", "Item 1");
    let mut items = Vec::new();
    sidebar.flatten_item(&item, 0, &mut items);
    assert_eq!(items.len(), 1);
}

#[test]
fn test_flatten_item_with_children_not_expanded() {
    let sidebar = Sidebar::new();
    let mut item = SidebarItem::new("item1", "Item 1");
    item.children.push(SidebarItem::new("child1", "Child 1"));

    let mut items = Vec::new();
    sidebar.flatten_item(&item, 0, &mut items);
    assert_eq!(items.len(), 1); // Only parent
}

#[test]
fn test_flatten_item_with_children_expanded() {
    let sidebar = Sidebar::new();
    let mut item = SidebarItem::new("item1", "Item 1");
    item.expanded = true;
    item.children.push(SidebarItem::new("child1", "Child 1"));

    let mut items = Vec::new();
    sidebar.flatten_item(&item, 0, &mut items);
    assert_eq!(items.len(), 2); // Parent + child
}

#[test]
fn test_flatten_item_depth() {
    let sidebar = Sidebar::new();
    let item = SidebarItem::new("item1", "Item 1");
    let mut items = Vec::new();
    sidebar.flatten_item(&item, 3, &mut items);

    if let Some(crate::widget::layout::sidebar::types::FlattenedItem::Item { depth, .. }) = items.first() {
        assert_eq!(*depth, 3);
    } else {
        panic!("Expected Item with depth");
    }
}

#[test]
fn test_flatten_item_nested_depth() {
    let sidebar = Sidebar::new();
    let mut parent = SidebarItem::new("parent", "Parent");
    parent.expanded = true;
    parent.children.push(SidebarItem::new("child", "Child"));

    let mut items = Vec::new();
    sidebar.flatten_item(&parent, 1, &mut items);

    assert_eq!(items.len(), 2);
    if let crate::widget::layout::sidebar::types::FlattenedItem::Item { depth, .. } = &items[0] {
        assert_eq!(*depth, 1);
    }
    if let crate::widget::layout::sidebar::types::FlattenedItem::Item { depth, .. } = &items[1] {
        assert_eq!(*depth, 2);
    }
}

// =========================================================================
// Sidebar::item_count tests
// =========================================================================

#[test]
fn test_item_count_empty() {
    let sidebar = Sidebar::new();
    assert_eq!(sidebar.item_count(), 0);
}

#[test]
fn test_item_count_single_item() {
    let sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    assert_eq!(sidebar.item_count(), 1);
}

#[test]
fn test_item_count_multiple_items() {
    let sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2"),
        SidebarItem::new("item3", "Item 3"),
    ]));
    assert_eq!(sidebar.item_count(), 3);
}

#[test]
fn test_item_count_excludes_sections() {
    let sidebar = Sidebar::new()
        .section(SidebarSection::titled(
            "Section 1",
            vec![SidebarItem::new("item1", "Item 1")],
        ))
        .section(SidebarSection::titled(
            "Section 2",
            vec![SidebarItem::new("item2", "Item 2")],
        ));
    // Section titles are not counted, only items
    assert_eq!(sidebar.item_count(), 2);
}

#[test]
fn test_item_count_includes_expanded_children() {
    let mut item1 = SidebarItem::new("item1", "Item 1");
    item1.expanded = true;
    item1.children.push(SidebarItem::new("child1", "Child 1"));

    let sidebar = Sidebar::new().section(SidebarSection::new(vec![item1]));
    assert_eq!(sidebar.item_count(), 2);
}

// =========================================================================
// Sidebar::hover_down tests
// =========================================================================

#[test]
fn test_hover_down_empty() {
    let mut sidebar = Sidebar::new();
    sidebar.hover_down();
    assert_eq!(sidebar.hovered, 0);
}

#[test]
fn test_hover_down_single_item() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    sidebar.hover_down();
    assert_eq!(sidebar.hovered, 0);
    sidebar.hover_down();
    // Should stay at first item if only one
    assert_eq!(sidebar.hovered, 0);
}

#[test]
fn test_hover_down_multiple_items() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2"),
        SidebarItem::new("item3", "Item 3"),
    ]));
    sidebar.hover_down(); // Initial hovered is 0, moves to 1
    assert_eq!(sidebar.hovered, 1);
    sidebar.hover_down(); // Moves to 2
    assert_eq!(sidebar.hovered, 2);
    sidebar.hover_down(); // Stays at 2 (last item)
    assert_eq!(sidebar.hovered, 2);
}

#[test]
fn test_hover_down_skips_disabled() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2").disabled(true),
        SidebarItem::new("item3", "Item 3"),
    ]));
    sidebar.hover_down(); // Initial hovered is 0, but item2 is disabled, so moves to item3 (index 2)
    assert_eq!(sidebar.hovered, 2);
    sidebar.hover_down(); // Stays at 2 (last non-disabled item)
    assert_eq!(sidebar.hovered, 2);
}

// =========================================================================
// Sidebar::hover_up tests
// =========================================================================

#[test]
fn test_hover_up_empty() {
    let mut sidebar = Sidebar::new();
    sidebar.hover_up();
    assert_eq!(sidebar.hovered, 0);
}

#[test]
fn test_hover_up_single_item() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    sidebar.hover_up();
    assert_eq!(sidebar.hovered, 0);
}

#[test]
fn test_hover_up_multiple_items() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2"),
        SidebarItem::new("item3", "Item 3"),
    ]));
    sidebar.hovered = 2;
    sidebar.hover_up();
    assert_eq!(sidebar.hovered, 1);
    sidebar.hover_up();
    assert_eq!(sidebar.hovered, 0);
}

#[test]
fn test_hover_up_skips_disabled() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2").disabled(true),
        SidebarItem::new("item3", "Item 3"),
    ]));
    sidebar.hovered = 2;
    sidebar.hover_up(); // Should skip to item1
    assert_eq!(sidebar.hovered, 0);
}

// =========================================================================
// Sidebar::select_hovered tests
// =========================================================================

#[test]
fn test_select_hovered_empty() {
    let mut sidebar = Sidebar::new();
    sidebar.select_hovered();
    assert!(sidebar.selected.is_none());
}

#[test]
fn test_select_hovered_valid() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    sidebar.select_hovered();
    assert_eq!(sidebar.selected.as_deref(), Some("item1"));
}

#[test]
fn test_select_hovered_disabled() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )
    .disabled(true)]));
    sidebar.select_hovered();
    assert!(sidebar.selected.is_none());
}

// =========================================================================
// Sidebar::toggle_hovered tests
// =========================================================================

#[test]
fn test_toggle_hovered_empty() {
    let mut sidebar = Sidebar::new();
    sidebar.toggle_hovered();
    // Should not crash
}

#[test]
fn test_toggle_hovered_with_children() {
    let mut item = SidebarItem::new("item1", "Item 1");
    item.children.push(SidebarItem::new("child1", "Child 1"));

    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![item]));
    sidebar.toggle_hovered();
    // Item should now be expanded
}

#[test]
fn test_toggle_hovered_without_children() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    sidebar.toggle_hovered();
    // Should not crash on item without children
}

// =========================================================================
// Sidebar::toggle_item tests
// =========================================================================

#[test]
fn test_toggle_item_by_id() {
    let mut item = SidebarItem::new("item1", "Item 1");
    item.children.push(SidebarItem::new("child1", "Child 1"));

    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![item]));

    sidebar.toggle_item("item1");
    // Item should now be expanded
}

#[test]
fn test_toggle_item_nonexistent() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));

    sidebar.toggle_item("nonexistent");
    // Should not crash
}

// =========================================================================
// Sidebar::expand_all / collapse_all tests
// =========================================================================

#[test]
fn test_expand_all() {
    let mut parent1 = SidebarItem::new("parent1", "Parent 1");
    parent1.children.push(SidebarItem::new("child1", "Child 1"));

    let mut parent2 = SidebarItem::new("parent2", "Parent 2");
    parent2.children.push(SidebarItem::new("child2", "Child 2"));

    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![parent1, parent2]));

    sidebar.expand_all();
    let items = sidebar.visible_items();
    // Should show: parent1, child1, parent2, child2
    assert_eq!(items.len(), 4);
}

#[test]
fn test_collapse_all() {
    let mut parent1 = SidebarItem::new("parent1", "Parent 1");
    parent1.expanded = true;
    parent1.children.push(SidebarItem::new("child1", "Child 1"));

    let mut parent2 = SidebarItem::new("parent2", "Parent 2");
    parent2.expanded = true;
    parent2.children.push(SidebarItem::new("child2", "Child 2"));

    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![parent1, parent2]));

    sidebar.collapse_all();
    let items = sidebar.visible_items();
    // Should show only parents
    assert_eq!(items.len(), 2);
}

#[test]
fn test_expand_empty_sidebar() {
    let mut sidebar = Sidebar::new();
    sidebar.expand_all();
    // Should not crash
}

#[test]
fn test_collapse_empty_sidebar() {
    let mut sidebar = Sidebar::new();
    sidebar.collapse_all();
    // Should not crash
}

// =========================================================================
// Sidebar::set_expanded_recursive tests
// =========================================================================

#[test]
fn test_set_expanded_recursive_true() {
    let mut parent = SidebarItem::new("parent", "Parent");
    parent.children.push(SidebarItem::new("child", "Child"));

    Sidebar::set_expanded_recursive(&mut parent, true);
    assert!(parent.expanded);
    assert!(parent.children[0].expanded);
}

#[test]
fn test_set_expanded_recursive_false() {
    let mut parent = SidebarItem::new("parent", "Parent");
    parent.expanded = true;
    parent.children.push(SidebarItem::new("child", "Child"));

    Sidebar::set_expanded_recursive(&mut parent, false);
    assert!(!parent.expanded);
    assert!(!parent.children[0].expanded);
}

#[test]
fn test_set_expanded_deep_nesting() {
    let grandchild = SidebarItem::new("grandchild", "Grandchild");
    let mut child = SidebarItem::new("child", "Child");
    child.children.push(grandchild);
    let mut parent = SidebarItem::new("parent", "Parent");
    parent.children.push(child);

    Sidebar::set_expanded_recursive(&mut parent, true);
    assert!(parent.expanded);
    assert!(parent.children[0].expanded);
    assert!(parent.children[0].children[0].expanded);
}