//! Sidebar widget tests

mod builder;
mod expand_collapse;
mod flattened;
mod helpers;
mod navigation;
mod render;
mod types;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::CollapseMode;
use revue::widget::FlattenedItem;
use revue::widget::Sidebar;
use revue::widget::SidebarItem;
use revue::widget::SidebarSection;
use revue::widget::View;

/// Render `sidebar` into a `width` x `height` buffer and return each row as text.
fn render_rows(sidebar: &Sidebar, width: u16, height: u16) -> Vec<String> {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    sidebar.render(&mut ctx);
    let rows: Vec<String> = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                .collect::<String>()
        })
        .collect();
    rows
}

/// Id and depth of each visible item, skipping section rows.
fn item_depths(sidebar: &Sidebar) -> Vec<(String, usize)> {
    sidebar
        .visible_items()
        .into_iter()
        .filter_map(|f| match f {
            FlattenedItem::Item { item, depth } => Some((item.id, depth)),
            FlattenedItem::Section(_) => None,
        })
        .collect()
}

// =========================================================================
// Sidebar::new tests
// =========================================================================

#[test]
fn test_sidebar_new() {
    let sidebar = Sidebar::new();
    assert!(sidebar.visible_items().is_empty());
    assert!(sidebar.selected_id().is_none());
    assert_eq!(sidebar.hovered_index(), 0);
    assert!(!sidebar.is_collapsed());
    assert_eq!(sidebar.current_width(), 24);
    assert_eq!(
        Sidebar::new()
            .collapse_mode(CollapseMode::Collapsed)
            .current_width(),
        4
    );

    let sb = Sidebar::new();
    assert!(sb.selected_id().is_none());
    assert_eq!(sb.hovered_index(), 0);
    assert!(!sb.is_collapsed());

    let sb = Sidebar::new();
    assert_eq!(sb.selected_id(), None);
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_default_collapse_threshold() {
    // Default threshold is 20: Auto collapses below it, stays expanded at it.
    let sidebar = Sidebar::new()
        .header("Header")
        .collapse_mode(CollapseMode::Auto);
    assert!(render_rows(&sidebar, 20, 4)[0].contains("Header"));
    assert!(!render_rows(&sidebar, 19, 4)[0].contains("Head"));
}

#[test]
fn test_sidebar_new_with_collapse_mode() {
    let sidebar = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert!(sidebar.is_collapsed());
}

#[test]
fn test_sidebar_new_with_widths() {
    let sidebar = Sidebar::new().expanded_width(30).collapsed_width(10);
    assert_eq!(sidebar.current_width(), 30);
    let sidebar = sidebar.collapse_mode(CollapseMode::Collapsed);
    assert_eq!(sidebar.current_width(), 10);
}

#[test]
fn test_sidebar_new_with_threshold() {
    let sidebar = Sidebar::new()
        .header("Header")
        .collapse_mode(CollapseMode::Auto)
        .collapse_threshold(20);
    // Width 19 is below the threshold: collapsed, header text is hidden.
    assert!(!render_rows(&sidebar, 19, 4)[0].contains("Head"));
    // Width 20 meets the threshold: expanded, header is drawn.
    assert!(render_rows(&sidebar, 20, 4)[0].contains("Header"));
}

// =========================================================================
// Sidebar::header tests
// =========================================================================

#[test]
fn test_sidebar_header() {
    let sidebar = Sidebar::new().header("Test Header");
    let rows = render_rows(&sidebar, 20, 5);
    assert!(rows[0].contains("Test Header"));
    // A separator line follows the header.
    assert!(rows[1].trim_end_matches('│').chars().all(|c| c == '─'));
}

#[test]
fn test_sidebar_header_empty() {
    let sidebar = Sidebar::new().header("");
    let rows = render_rows(&sidebar, 20, 5);
    // An empty header still reserves its row and draws the separator.
    assert!(rows[0].trim_end_matches('│').trim().is_empty());
    assert!(rows[1].trim_end_matches('│').chars().all(|c| c == '─'));
}

#[test]
fn test_sidebar_header_with_string() {
    let sidebar = Sidebar::new().header(String::from("Header"));
    assert!(render_rows(&sidebar, 20, 5)[0].contains("Header"));
}

// =========================================================================
// Sidebar::footer tests
// =========================================================================

#[test]
fn test_sidebar_footer() {
    let sidebar = Sidebar::new().footer("Test Footer");
    let rows = render_rows(&sidebar, 20, 6);
    assert!(rows[5].contains("Test Footer"));
    // A separator line precedes the footer.
    assert!(rows[4].trim_end_matches('│').chars().all(|c| c == '─'));
}

#[test]
fn test_sidebar_footer_empty() {
    let sidebar = Sidebar::new().footer("");
    let rows = render_rows(&sidebar, 20, 6);
    // An empty footer still draws its separator.
    assert!(rows[4].trim_end_matches('│').chars().all(|c| c == '─'));
    assert!(rows[5].trim_end_matches('│').trim().is_empty());
}

// =========================================================================
// Sidebar::section tests
// =========================================================================

#[test]
fn test_sidebar_section() {
    let section = SidebarSection::new(vec![SidebarItem::new("item1", "Item 1")]);
    let sidebar = Sidebar::new().section(section);
    assert_eq!(item_depths(&sidebar), vec![("item1".to_string(), 0)]);
}

#[test]
fn test_sidebar_section_with_string() {
    let section = SidebarSection::titled("Section", vec![SidebarItem::new("a", "A")]);
    let sidebar = Sidebar::new().section(section);
    match &sidebar.visible_items()[0] {
        FlattenedItem::Section(title) => assert_eq!(title.as_deref(), Some("Section")),
        other => panic!("expected section row, got {other:?}"),
    }
}

#[test]
fn test_sidebar_section_multiple() {
    let section1 = SidebarSection::new(vec![SidebarItem::new("a", "A")]);
    let section2 = SidebarSection::titled("Section B", vec![SidebarItem::new("b", "B")]);
    let sidebar = Sidebar::new().section(section1).section(section2);
    // Untitled section contributes no header row; titled one does.
    let items = sidebar.visible_items();
    assert_eq!(items.len(), 3);
    assert!(matches!(&items[0], FlattenedItem::Item { item, .. } if item.id == "a"));
    assert!(matches!(&items[1], FlattenedItem::Section(Some(t)) if t == "Section B"));
    assert!(matches!(&items[2], FlattenedItem::Item { item, .. } if item.id == "b"));
}

#[test]
fn test_sidebar_section_empty() {
    let sidebar = Sidebar::new().section(SidebarSection::new(vec![]));
    assert!(sidebar.visible_items().is_empty());
    assert_eq!(sidebar.item_count(), 0);
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
    assert_eq!(sidebar.current_width(), 30);
    assert_eq!(
        sidebar
            .collapse_mode(CollapseMode::Collapsed)
            .current_width(),
        10
    );
}

#[test]
fn test_sidebar_width_zero() {
    let sidebar = Sidebar::new().expanded_width(0).collapsed_width(0);
    assert_eq!(sidebar.current_width(), 0);
    assert_eq!(
        sidebar
            .collapse_mode(CollapseMode::Collapsed)
            .current_width(),
        0
    );
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
    let sidebar = Sidebar::new().selected("test_id");
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
    let mut sidebar = Sidebar::new().section(SidebarSection::new(
        (0..6)
            .map(|i| SidebarItem::new(format!("item{i}"), format!("Item {i}")))
            .collect(),
    ));
    for _ in 0..5 {
        sidebar.hover_down();
    }
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
// visible_items depth tests
// =========================================================================

#[test]
fn test_visible_items_top_level_depth_zero() {
    let sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    assert_eq!(item_depths(&sidebar), vec![("item1".to_string(), 0)]);
}

#[test]
fn test_visible_items_nested_depth() {
    let grandchild = SidebarItem::new("grandchild", "Grandchild");
    let child = SidebarItem::new("child", "Child")
        .children(vec![grandchild])
        .expanded(true);
    let parent = SidebarItem::new("parent", "Parent")
        .children(vec![child])
        .expanded(true);
    let sidebar = Sidebar::new().section(SidebarSection::new(vec![parent]));

    assert_eq!(
        item_depths(&sidebar),
        vec![
            ("parent".to_string(), 0),
            ("child".to_string(), 1),
            ("grandchild".to_string(), 2),
        ]
    );
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
    assert_eq!(sidebar.hovered_index(), 0);
}

#[test]
fn test_hover_down_single_item() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    sidebar.hover_down();
    assert_eq!(sidebar.hovered_index(), 0);
    sidebar.hover_down();
    // Should stay at first item if only one
    assert_eq!(sidebar.hovered_index(), 0);
}

#[test]
fn test_hover_down_multiple_items() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2"),
        SidebarItem::new("item3", "Item 3"),
    ]));
    sidebar.hover_down(); // Initial hovered is 0, moves to 1
    assert_eq!(sidebar.hovered_index(), 1);
    sidebar.hover_down(); // Moves to 2
    assert_eq!(sidebar.hovered_index(), 2);
    sidebar.hover_down(); // Stays at 2 (last item)
    assert_eq!(sidebar.hovered_index(), 2);
}

#[test]
fn test_hover_down_skips_disabled() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2").disabled(true),
        SidebarItem::new("item3", "Item 3"),
    ]));
    sidebar.hover_down(); // Initial hovered is 0, but item2 is disabled, so moves to item3 (index 2)
    assert_eq!(sidebar.hovered_index(), 2);
    sidebar.hover_down(); // Stays at 2 (last non-disabled item)
    assert_eq!(sidebar.hovered_index(), 2);
}

// =========================================================================
// Sidebar::hover_up tests
// =========================================================================

#[test]
fn test_hover_up_empty() {
    let mut sidebar = Sidebar::new();
    sidebar.hover_up();
    assert_eq!(sidebar.hovered_index(), 0);
}

#[test]
fn test_hover_up_single_item() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    sidebar.hover_up();
    assert_eq!(sidebar.hovered_index(), 0);
}

#[test]
fn test_hover_up_multiple_items() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2"),
        SidebarItem::new("item3", "Item 3"),
    ]));
    sidebar.hover_down();
    sidebar.hover_down();
    assert_eq!(sidebar.hovered_index(), 2);
    sidebar.hover_up();
    assert_eq!(sidebar.hovered_index(), 1);
    sidebar.hover_up();
    assert_eq!(sidebar.hovered_index(), 0);
}

#[test]
fn test_hover_up_skips_disabled() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![
        SidebarItem::new("item1", "Item 1"),
        SidebarItem::new("item2", "Item 2").disabled(true),
        SidebarItem::new("item3", "Item 3"),
    ]));
    sidebar.hover_down(); // Skips disabled item2
    assert_eq!(sidebar.hovered_index(), 2);
    sidebar.hover_up(); // Should skip to item1
    assert_eq!(sidebar.hovered_index(), 0);
}

// =========================================================================
// Sidebar::select_hovered tests
// =========================================================================

#[test]
fn test_select_hovered_empty() {
    let mut sidebar = Sidebar::new();
    sidebar.select_hovered();
    assert!(sidebar.selected_id().is_none());
}

#[test]
fn test_select_hovered_valid() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    sidebar.select_hovered();
    assert_eq!(sidebar.selected_id(), Some("item1"));
}

#[test]
fn test_select_hovered_disabled() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )
    .disabled(true)]));
    sidebar.select_hovered();
    assert!(sidebar.selected_id().is_none());
}

// =========================================================================
// Sidebar::toggle_hovered tests
// =========================================================================

#[test]
fn test_toggle_hovered_empty() {
    let mut sidebar = Sidebar::new();
    sidebar.toggle_hovered();
    assert!(sidebar.visible_items().is_empty());
}

#[test]
fn test_toggle_hovered_with_children() {
    let mut item = SidebarItem::new("item1", "Item 1");
    item.children.push(SidebarItem::new("child1", "Child 1"));

    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![item]));
    assert_eq!(sidebar.item_count(), 1);
    sidebar.toggle_hovered();
    // Item is now expanded: its child is visible
    assert_eq!(
        item_depths(&sidebar),
        vec![("item1".to_string(), 0), ("child1".to_string(), 1)]
    );
    sidebar.toggle_hovered();
    assert_eq!(sidebar.item_count(), 1);
}

#[test]
fn test_toggle_hovered_without_children() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));
    sidebar.toggle_hovered();
    // Toggling a leaf changes nothing
    assert_eq!(item_depths(&sidebar), vec![("item1".to_string(), 0)]);
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
    // Item is now expanded: its child is visible
    assert_eq!(sidebar.item_count(), 2);
    sidebar.toggle_item("item1");
    assert_eq!(sidebar.item_count(), 1);
}

#[test]
fn test_toggle_item_nonexistent() {
    let mut sidebar = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new(
        "item1", "Item 1",
    )]));

    sidebar.toggle_item("nonexistent");
    assert_eq!(item_depths(&sidebar), vec![("item1".to_string(), 0)]);
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
    assert!(sidebar.visible_items().is_empty());
}

#[test]
fn test_collapse_empty_sidebar() {
    let mut sidebar = Sidebar::new();
    sidebar.collapse_all();
    assert!(sidebar.visible_items().is_empty());
}

// =========================================================================
// expand_all / collapse_all recurse into nested children
// =========================================================================

fn three_level_sidebar(expanded: bool) -> Sidebar {
    let grandchild = SidebarItem::new("grandchild", "Grandchild");
    let child = SidebarItem::new("child", "Child")
        .children(vec![grandchild])
        .expanded(expanded);
    let parent = SidebarItem::new("parent", "Parent")
        .children(vec![child])
        .expanded(expanded);
    Sidebar::new().section(SidebarSection::new(vec![parent]))
}

#[test]
fn test_expand_all_deep_nesting() {
    let mut sidebar = three_level_sidebar(false);
    assert_eq!(sidebar.item_count(), 1);
    sidebar.expand_all();
    assert_eq!(
        item_depths(&sidebar),
        vec![
            ("parent".to_string(), 0),
            ("child".to_string(), 1),
            ("grandchild".to_string(), 2),
        ]
    );
}

#[test]
fn test_collapse_all_deep_nesting() {
    let mut sidebar = three_level_sidebar(true);
    assert_eq!(sidebar.item_count(), 3);
    sidebar.collapse_all();
    assert_eq!(sidebar.item_count(), 1);
    // The nested child was collapsed too, not just the top level:
    // re-opening only the parent must not reveal the grandchild.
    sidebar.toggle_item("parent");
    assert_eq!(
        item_depths(&sidebar),
        vec![("parent".to_string(), 0), ("child".to_string(), 1)]
    );
}

// =========================================================================
// SidebarItem Tests
// =========================================================================

#[test]
fn test_sidebar_item_new() {
    let item = SidebarItem::new("home", "Home");
    assert_eq!(item.id, "home");
    assert_eq!(item.label, "Home");
    assert!(item.icon.is_none());
    assert!(!item.disabled);
    assert!(item.badge.is_none());
    assert!(item.children.is_empty());
    assert!(!item.expanded);

    let item = SidebarItem::new("home", "Home");
    assert_eq!(item.id, "home");
    assert_eq!(item.label, "Home");
    assert!(!item.disabled);
    assert!(item.icon.is_none());
    assert!(item.badge.is_none());
}

#[test]
fn test_sidebar_item_icon() {
    let item = SidebarItem::new("home", "Home").icon('🏠');
    assert_eq!(item.icon, Some('🏠'));
}

#[test]
fn test_sidebar_item_disabled() {
    let item = SidebarItem::new("home", "Home").disabled(true);
    assert!(item.disabled);
}

#[test]
fn test_sidebar_item_badge() {
    let item = SidebarItem::new("inbox", "Inbox").badge("5");
    assert_eq!(item.badge, Some("5".to_string()));
}

#[test]
fn test_sidebar_item_children() {
    let children = vec![
        SidebarItem::new("child1", "Child 1"),
        SidebarItem::new("child2", "Child 2"),
    ];
    let item = SidebarItem::new("parent", "Parent").children(children);
    assert_eq!(item.children.len(), 2);

    let child1 = SidebarItem::new("child1", "Child 1");
    let child2 = SidebarItem::new("child2", "Child 2");
    let parent = SidebarItem::new("parent", "Parent").children(vec![child1, child2]);

    assert!(parent.has_children());
    assert_eq!(parent.children.len(), 2);
}

#[test]
fn test_sidebar_item_expanded() {
    let item = SidebarItem::new("folder", "Folder").expanded(true);
    assert!(item.expanded);
}

#[test]
fn test_sidebar_item_has_children() {
    let item =
        SidebarItem::new("folder", "Folder").children(vec![SidebarItem::new("file", "File")]);
    assert!(item.has_children());

    let empty_item = SidebarItem::new("file", "File");
    assert!(!empty_item.has_children());
}

#[test]
fn test_sidebar_item_builder_chain() {
    let item = SidebarItem::new("nav", "Navigation")
        .icon('📁')
        .disabled(false)
        .badge("3")
        .expanded(true);

    assert_eq!(item.icon, Some('📁'));
    assert!(!item.disabled);
    assert_eq!(item.badge, Some("3".to_string()));
    assert!(item.expanded);
}

// =========================================================================
// SidebarSection Tests
// =========================================================================

#[test]
fn test_sidebar_section_new() {
    let items = vec![SidebarItem::new("home", "Home")];
    let section = SidebarSection::new(items);
    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 1);

    let items = vec![
        SidebarItem::new("a", "Item A"),
        SidebarItem::new("b", "Item B"),
    ];
    let section = SidebarSection::new(items);

    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 2);
}

#[test]
fn test_sidebar_section_titled() {
    let items = vec![SidebarItem::new("home", "Home")];
    let section = SidebarSection::titled("Main", items);
    assert_eq!(section.title, Some("Main".to_string()));
    assert_eq!(section.items.len(), 1);

    let items = vec![SidebarItem::new("a", "Item A")];
    let section = SidebarSection::titled("My Section", items);

    assert_eq!(section.title, Some("My Section".to_string()));
}

// =========================================================================
// CollapseMode Tests
// =========================================================================

#[test]
fn test_collapse_mode_default() {
    assert_eq!(CollapseMode::default(), CollapseMode::Expanded);
}

#[test]
fn test_collapse_mode_equality() {
    assert_eq!(CollapseMode::Expanded, CollapseMode::Expanded);
    assert_ne!(CollapseMode::Expanded, CollapseMode::Collapsed);
}

// =========================================================================
// Sidebar Creation Tests
// =========================================================================

#[test]
fn test_sidebar_default() {
    let sb = Sidebar::default();
    assert!(!sb.is_collapsed());

    let sb = Sidebar::default();
    assert_eq!(sb.item_count(), 0);
}

#[test]
fn test_sidebar_section_builder() {
    let sb = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new("home", "Home")]));
    assert_eq!(sb.item_count(), 1);
}

#[test]
fn test_sidebar_sections_builder() {
    let sb = Sidebar::new().sections(vec![
        SidebarSection::new(vec![SidebarItem::new("a", "A")]),
        SidebarSection::new(vec![SidebarItem::new("b", "B")]),
    ]);
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_items_builder() {
    let sb = Sidebar::new().items(vec![
        SidebarItem::new("home", "Home"),
        SidebarItem::new("settings", "Settings"),
    ]);
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_selected() {
    let sb = Sidebar::new()
        .items(vec![SidebarItem::new("home", "Home")])
        .selected("home");
    assert_eq!(sb.selected_id(), Some("home"));
}

#[test]
fn test_sidebar_collapse_mode() {
    let sb = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert!(sb.is_collapsed());
}

// =========================================================================
// State Getter Tests
// =========================================================================

#[test]
fn test_sidebar_current_width_expanded() {
    let sb = Sidebar::new()
        .expanded_width(30)
        .collapse_mode(CollapseMode::Expanded);
    assert_eq!(sb.current_width(), 30);
}

#[test]
fn test_sidebar_current_width_collapsed() {
    let sb = Sidebar::new()
        .collapsed_width(5)
        .collapse_mode(CollapseMode::Collapsed);
    assert_eq!(sb.current_width(), 5);
}

#[test]
fn test_sidebar_current_width_auto() {
    let sb = Sidebar::new()
        .expanded_width(25)
        .collapse_mode(CollapseMode::Auto);
    // Auto mode returns expanded_width (actual collapse determined at render)
    assert_eq!(sb.current_width(), 25);
}

#[test]
fn test_sidebar_visible_items_empty() {
    let sb = Sidebar::new();
    assert!(sb.visible_items().is_empty());
}

#[test]
fn test_sidebar_visible_items_flat() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    let items = sb.visible_items();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_visible_items_with_section_title() {
    let sb = Sidebar::new().section(SidebarSection::titled(
        "Section",
        vec![SidebarItem::new("a", "A")],
    ));
    let items = sb.visible_items();
    assert_eq!(items.len(), 2); // Section header + item
}

#[test]
fn test_sidebar_visible_items_nested_collapsed() {
    let sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("parent", "Parent")
                .children(vec![SidebarItem::new("child", "Child")])]);
    let items = sb.visible_items();
    // Parent not expanded, so child is hidden
    assert_eq!(items.len(), 1);
}

#[test]
fn test_sidebar_visible_items_nested_expanded() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("parent", "Parent")
        .expanded(true)
        .children(vec![SidebarItem::new("child", "Child")])]);
    let items = sb.visible_items();
    // Parent expanded, so child is visible
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_item_count() {
    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Main",
            vec![SidebarItem::new("a", "A")],
        ))
        .items(vec![SidebarItem::new("b", "B")]);
    // item_count excludes sections
    assert_eq!(sb.item_count(), 2);
}

// =========================================================================
// Navigation Tests
// =========================================================================

#[test]
fn test_sidebar_hover_down() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A"),
        SidebarItem::new("b", "B"),
        SidebarItem::new("c", "C"),
    ]);
    assert_eq!(sb.hovered_index(), 0);
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 1);
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 2);
}

#[test]
fn test_sidebar_hover_down_at_end() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    sb.hover_down();
    sb.hover_down();
    sb.hover_down(); // Should stay at last
    assert_eq!(sb.hovered_index(), 1);
}

#[test]
fn test_sidebar_hover_down_skips_disabled() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A"),
        SidebarItem::new("b", "B").disabled(true),
        SidebarItem::new("c", "C"),
    ]);
    sb.hover_down();
    // Should skip disabled item B and go to C
    assert_eq!(sb.hovered_index(), 2);
}

#[test]
fn test_sidebar_select_hovered_disabled() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A").disabled(true)]);
    sb.select_hovered();
    // Should not select disabled item
    assert!(sb.selected_id().is_none());
}

#[test]
fn test_sidebar_toggle_hovered() {
    let mut sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("parent", "Parent")
                .children(vec![SidebarItem::new("child", "Child")])]);
    assert_eq!(sb.visible_items().len(), 1);
    sb.toggle_hovered();
    assert_eq!(sb.visible_items().len(), 2);
    sb.toggle_hovered();
    assert_eq!(sb.visible_items().len(), 1);
}

#[test]
fn test_sidebar_toggle_item() {
    let mut sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("folder", "Folder")
                .children(vec![SidebarItem::new("file", "File")])]);
    sb.toggle_item("folder");
    let items = sb.visible_items();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_expand_all() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A").children(vec![SidebarItem::new("a1", "A1")]),
        SidebarItem::new("b", "B").children(vec![SidebarItem::new("b1", "B1")]),
    ]);
    sb.expand_all();
    assert_eq!(sb.visible_items().len(), 4);
}

#[test]
fn test_sidebar_collapse_all() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A")
            .expanded(true)
            .children(vec![SidebarItem::new("a1", "A1")]),
        SidebarItem::new("b", "B")
            .expanded(true)
            .children(vec![SidebarItem::new("b1", "B1")]),
    ]);
    assert_eq!(sb.visible_items().len(), 4);
    sb.collapse_all();
    assert_eq!(sb.visible_items().len(), 2);
}

#[test]
fn test_sidebar_toggle_collapse() {
    let mut sb = Sidebar::new().collapse_mode(CollapseMode::Expanded);
    assert!(!sb.is_collapsed());
    sb.toggle_collapse();
    assert!(sb.is_collapsed());
    sb.toggle_collapse();
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_toggle_collapse_from_auto() {
    let mut sb = Sidebar::new().collapse_mode(CollapseMode::Auto);
    sb.toggle_collapse();
    assert!(sb.is_collapsed());
}

// =========================================================================
// FlattenedItem Tests
// =========================================================================

#[test]
fn test_flattened_item_section() {
    let flat = FlattenedItem::Section(Some("Title".to_string()));
    if let FlattenedItem::Section(title) = flat {
        assert_eq!(title, Some("Title".to_string()));
    } else {
        panic!("Expected Section");
    }
}

#[test]
fn test_flattened_item_item() {
    let flat = FlattenedItem::Item {
        item: SidebarItem::new("test", "Test"),
        depth: 2,
    };
    if let FlattenedItem::Item { item, depth } = flat {
        assert_eq!(item.id, "test");
        assert_eq!(depth, 2);
    } else {
        panic!("Expected Item");
    }
}

// =========================================================================
// Render Tests
// =========================================================================

#[test]
fn test_sidebar_render_basic() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![
        SidebarItem::new("home", "Home").icon('🏠'),
        SidebarItem::new("settings", "Settings").icon('⚙'),
    ]);
    sb.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_sidebar_render_with_header() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .header("My App")
        .items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_footer() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .footer("v1.0.0")
        .items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_collapsed() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .items(vec![SidebarItem::new("home", "Home").icon('🏠')]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_sections() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Main",
            vec![SidebarItem::new("home", "Home")],
        ))
        .section(SidebarSection::titled(
            "Settings",
            vec![SidebarItem::new("prefs", "Preferences")],
        ));
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_nested() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("folder", "Folder")
        .expanded(true)
        .children(vec![SidebarItem::new("file", "File")])]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_badges() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("inbox", "Inbox").badge("5")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_disabled() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![
        SidebarItem::new("active", "Active"),
        SidebarItem::new("disabled", "Disabled").disabled(true),
    ]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_selected() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .items(vec![
            SidebarItem::new("a", "Item A"),
            SidebarItem::new("b", "Item B"),
        ])
        .selected("b");
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx); // Should handle gracefully
}

#[test]
fn test_sidebar_render_auto_collapse() {
    let mut buffer = Buffer::new(15, 10);
    let area = Rect::new(0, 0, 15, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Auto)
        .collapse_threshold(20)
        .items(vec![SidebarItem::new("home", "Home").icon('🏠')]);
    // Width (15) < threshold (20), so should render collapsed
    sb.render(&mut ctx);
}

// =========================================================================
// Edge Cases
// =========================================================================

#[test]
fn test_sidebar_empty() {
    let sb = Sidebar::new();
    assert_eq!(sb.item_count(), 0);
    assert!(sb.visible_items().is_empty());
}

#[test]
fn test_sidebar_navigation_on_empty() {
    let mut sb = Sidebar::new();
    sb.hover_up();
    sb.hover_down();
    sb.select_hovered();
    // Should not panic
    assert_eq!(sb.hovered_index(), 0);
}

#[test]
fn test_sidebar_toggle_nonexistent_item() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);
    sb.toggle_item("nonexistent");
    // Should not panic
}

#[test]
fn test_sidebar_deeply_nested() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("l1", "Level 1")
        .expanded(true)
        .children(vec![SidebarItem::new("l2", "Level 2")
            .expanded(true)
            .children(vec![SidebarItem::new("l3", "Level 3")
                .expanded(true)
                .children(vec![SidebarItem::new("l4", "Level 4")])])])]);

    let items = sb.visible_items();
    assert_eq!(items.len(), 4);

    // Check depths
    if let FlattenedItem::Item { depth, .. } = &items[3] {
        assert_eq!(*depth, 3);
    }
}

#[test]
fn test_sidebar_item_builder() {
    let item = SidebarItem::new("settings", "Settings")
        .icon('⚙')
        .badge("3")
        .disabled(true);

    assert_eq!(item.icon, Some('⚙'));
    assert_eq!(item.badge, Some("3".to_string()));
    assert!(item.disabled);
}
