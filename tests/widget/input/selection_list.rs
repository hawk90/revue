//! Tests for selection_list widget
//!
//! Extracted from src/widget/input/input_widgets/selection_list.rs

#[cfg(test)]
mod tests {
    use revue::style::Color;
    use revue::widget::input::input_widgets::selection_list::{SelectionItem, SelectionList, SelectionStyle};

    // =========================================================================
    // SelectionItem Constructor Tests
    // =========================================================================

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
    fn test_selection_item_new_with_string() {
        let item = SelectionItem::new(String::from("Owned String"));
        assert_eq!(item.text, "Owned String");
    }

    #[test]
    fn test_selection_item_value() {
        let item = SelectionItem::new("Item").value("item_value");
        assert_eq!(item.value, Some("item_value".to_string()));
    }

    #[test]
    fn test_selection_item_value_string() {
        let item = SelectionItem::new("Item").value(String::from("owned"));
        assert_eq!(item.value, Some("owned".to_string()));
    }

    #[test]
    fn test_selection_item_disabled_true() {
        let item = SelectionItem::new("Item").disabled(true);
        assert!(item.disabled);
    }

    #[test]
    fn test_selection_item_disabled_false() {
        let item = SelectionItem::new("Item").disabled(false);
        assert!(!item.disabled);
    }

    #[test]
    fn test_selection_item_description() {
        let item = SelectionItem::new("Item").description("Description text");
        assert_eq!(item.description, Some("Description text".to_string()));
    }

    #[test]
    fn test_selection_item_description_string() {
        let item = SelectionItem::new("Item").description(String::from("Owned desc"));
        assert_eq!(item.description, Some("Owned desc".to_string()));
    }

    #[test]
    fn test_selection_item_icon() {
        let item = SelectionItem::new("Item").icon("🔧");
        assert_eq!(item.icon, Some("🔧".to_string()));
    }

    #[test]
    fn test_selection_item_icon_string() {
        let item = SelectionItem::new("Item").icon(String::from("⚙"));
        assert_eq!(item.icon, Some("⚙".to_string()));
    }

    #[test]
    fn test_selection_item_builder_chain() {
        let item = SelectionItem::new("Full Item")
            .value("val")
            .disabled(false)
            .description("A description")
            .icon("📦");

        assert_eq!(item.text, "Full Item");
        assert_eq!(item.value, Some("val".to_string()));
        assert!(!item.disabled);
        assert_eq!(item.description, Some("A description".to_string()));
        assert_eq!(item.icon, Some("📦".to_string()));
    }

    #[test]
    fn test_selection_item_from_str() {
        let item: SelectionItem = "From Str".into();
        assert_eq!(item.text, "From Str");
    }

    #[test]
    fn test_selection_item_from_string() {
        let item: SelectionItem = String::from("From String").into();
        assert_eq!(item.text, "From String");
    }

    #[test]
    fn test_selection_item_clone_basic() {
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

    // =========================================================================
    // SelectionStyle Enum Tests
    // =========================================================================

    #[test]
    fn test_selection_style_default() {
        let style = SelectionStyle::default();
        assert_eq!(style, SelectionStyle::Checkbox);
    }

    #[test]
    fn test_selection_style_clone() {
        let style1 = SelectionStyle::Bullet;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
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
        assert_eq!(SelectionStyle::Checkbox, SelectionStyle::Checkbox);
        assert_eq!(SelectionStyle::Bullet, SelectionStyle::Bullet);
        assert_ne!(SelectionStyle::Checkbox, SelectionStyle::Bullet);
    }

    #[test]
    fn test_selection_style_all_variants() {
        let styles = [
            SelectionStyle::Checkbox,
            SelectionStyle::Bullet,
            SelectionStyle::Highlight,
            SelectionStyle::Bracket,
        ];
        assert_eq!(styles.len(), 4);
    }

    #[test]
    fn test_selection_style_debug() {
        let debug_str = format!("{:?}", SelectionStyle::Checkbox);
        assert!(debug_str.contains("Checkbox"));
    }

    // =========================================================================
    // SelectionList Constructor Tests
    // =========================================================================

    #[test]
    fn test_selection_list_new() {
        let list = SelectionList::new(vec!["A", "B", "C"]);
        assert_eq!(list.items.len(), 3);
        assert!(list.selected.is_empty());
        assert_eq!(list.highlighted, 0);
        assert_eq!(list.style, SelectionStyle::Checkbox);
        assert_eq!(list.max_selections, 0);
        assert_eq!(list.min_selections, 0);
        assert!(!list.show_descriptions);
        assert!(list.title.is_none());
        assert!(list.fg.is_none());
        assert!(list.selected_fg.is_none());
        assert!(list.highlighted_fg.is_none());
        assert!(list.bg.is_none());
        assert_eq!(list.max_visible, 0);
        assert_eq!(list.scroll_offset, 0);
        assert!(!list.show_count);
        assert!(!list.focused);
    }

    #[test]
    fn test_selection_list_new_empty() {
        let list = SelectionList::new(vec![] as Vec<&str>);
        assert_eq!(list.items.len(), 0);
        assert_eq!(list.highlighted, 0);
    }

    #[test]
    fn test_selection_list_new_with_strings() {
        let list = SelectionList::new(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ]);
        assert_eq!(list.items.len(), 3);
    }

    #[test]
    fn test_selection_list_new_with_selection_items() {
        let list = SelectionList::new(vec![
            SelectionItem::new("Item A"),
            SelectionItem::new("Item B"),
        ]);
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_selection_list_new_mixed() {
        let list = SelectionList::new(vec!["String", "Owned", "Item"]);
        assert_eq!(list.items.len(), 3);
    }

    // =========================================================================
    // Builder Method Tests
    // =========================================================================

    #[test]
    fn test_selection_list_selected() {
        let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 2]);
        assert_eq!(list.selected, vec![0, 2]);
    }

    #[test]
    fn test_selection_list_style() {
        let list = SelectionList::new(vec!["A"]).style(SelectionStyle::Bullet);
        assert_eq!(list.style, SelectionStyle::Bullet);
    }

    #[test]
    fn test_selection_list_show_checkboxes_true() {
        let list = SelectionList::new(vec!["A"]).show_checkboxes(true);
        assert_eq!(list.style, SelectionStyle::Checkbox);
    }

    #[test]
    fn test_selection_list_show_checkboxes_false() {
        let list = SelectionList::new(vec!["A"]).show_checkboxes(false);
        assert_eq!(list.style, SelectionStyle::Highlight);
    }

    #[test]
    fn test_selection_list_max_selections() {
        let list = SelectionList::new(vec!["A", "B", "C"]).max_selections(2);
        assert_eq!(list.max_selections, 2);
    }

    #[test]
    fn test_selection_list_min_selections() {
        let list = SelectionList::new(vec!["A", "B"]).min_selections(1);
        assert_eq!(list.min_selections, 1);
    }

    #[test]
    fn test_selection_list_show_descriptions_true() {
        let list = SelectionList::new(vec!["A"]).show_descriptions(true);
        assert!(list.show_descriptions);
    }

    #[test]
    fn test_selection_list_show_descriptions_false() {
        let list = SelectionList::new(vec!["A"]).show_descriptions(false);
        assert!(!list.show_descriptions);
    }

    #[test]
    fn test_selection_list_title() {
        let list = SelectionList::new(vec!["A"]).title("Select Items:");
        assert_eq!(list.title, Some("Select Items:".to_string()));
    }

    #[test]
    fn test_selection_list_title_string() {
        let list = SelectionList::new(vec!["A"]).title(String::from("Title"));
        assert_eq!(list.title, Some("Title".to_string()));
    }

    #[test]
    fn test_selection_list_fg() {
        let color = Color::WHITE;
        let list = SelectionList::new(vec!["A"]).fg(color);
        assert_eq!(list.fg, Some(color));
    }

    #[test]
    fn test_selection_list_selected_fg() {
        let color = Color::GREEN;
        let list = SelectionList::new(vec!["A"]).selected_fg(color);
        assert_eq!(list.selected_fg, Some(color));
    }

    #[test]
    fn test_selection_list_highlighted_fg() {
        let color = Color::CYAN;
        let list = SelectionList::new(vec!["A"]).highlighted_fg(color);
        assert_eq!(list.highlighted_fg, Some(color));
    }

    #[test]
    fn test_selection_list_bg() {
        let color = Color::BLACK;
        let list = SelectionList::new(vec!["A"]).bg(color);
        assert_eq!(list.bg, Some(color));
    }

    #[test]
    fn test_selection_list_max_visible() {
        let list = SelectionList::new(vec!["A", "B", "C"]).max_visible(2);
        assert_eq!(list.max_visible, 2);
    }

    #[test]
    fn test_selection_list_show_count_true() {
        let list = SelectionList::new(vec!["A"]).show_count(true);
        assert!(list.show_count);
    }

    #[test]
    fn test_selection_list_show_count_false() {
        let list = SelectionList::new(vec!["A"]).show_count(false);
        assert!(!list.show_count);
    }

    #[test]
    fn test_selection_list_focused_true() {
        let list = SelectionList::new(vec!["A"]).focused(true);
        assert!(list.focused);
    }

    #[test]
    fn test_selection_list_focused_false() {
        let list = SelectionList::new(vec!["A"]).focused(false);
        assert!(!list.focused);
    }

    // =========================================================================
    // Getter Method Tests
    // =========================================================================

    #[test]
    fn test_selection_list_get_selected() {
        let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 2]);
        assert_eq!(list.get_selected(), &[0, 2]);
    }

    #[test]
    fn test_selection_list_get_selected_empty() {
        let list = SelectionList::new(vec!["A", "B", "C"]);
        assert!(list.get_selected().is_empty());
    }

    #[test]
    fn test_selection_list_get_selected_values() {
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
    fn test_selection_list_get_selected_values_no_value_field() {
        let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 1]);
        let values = list.get_selected_values();
        assert_eq!(values, vec!["A", "B"]);
    }

    #[test]
    fn test_selection_list_get_selected_items() {
        let list = SelectionList::new(vec![
            SelectionItem::new("Item A").value("a"),
            SelectionItem::new("Item B").value("b"),
        ])
        .selected(vec![0, 1]);

        let items = list.get_selected_items();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].text, "Item A");
        assert_eq!(items[1].text, "Item B");
    }

    #[test]
    fn test_selection_list_is_selected_true() {
        let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![1]);
        assert!(list.is_selected(1));
    }

    #[test]
    fn test_selection_list_is_selected_false() {
        let list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0]);
        assert!(!list.is_selected(1));
    }

    // =========================================================================
    // State Mutation Method Tests
    // =========================================================================

    #[test]
    fn test_selection_toggle() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.toggle(1);
        assert!(list.is_selected(1));
        list.toggle(1);
        assert!(!list.is_selected(1));
    }

    #[test]
    fn test_selection_toggle_out_of_bounds() {
        let mut list = SelectionList::new(vec!["A", "B"]);
        list.toggle(10); // Should not panic
        assert!(list.selected.is_empty());
    }

    #[test]
    fn test_selection_toggle_disabled() {
        let mut list = SelectionList::new(vec![
            SelectionItem::new("A"),
            SelectionItem::new("B").disabled(true),
        ]);
        list.toggle(1); // Should not select disabled
        assert!(!list.is_selected(1));
    }

    #[test]
    fn test_selection_toggle_highlighted() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.highlight_next();
        assert_eq!(list.highlighted, 1);
        list.toggle_highlighted();
        assert!(list.is_selected(1));
        list.toggle_highlighted();
        assert!(!list.is_selected(1));
    }

    #[test]
    fn test_selection_select() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.select(1);
        assert!(list.is_selected(1));
    }

    #[test]
    fn test_selection_select_idempotent() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.select(1);
        list.select(1); // Should not duplicate
        assert_eq!(list.selected.len(), 1);
    }

    #[test]
    fn test_selection_select_out_of_bounds() {
        let mut list = SelectionList::new(vec!["A", "B"]);
        list.select(10); // Should not panic
        assert!(list.selected.is_empty());
    }

    #[test]
    fn test_selection_select_disabled() {
        let mut list = SelectionList::new(vec![
            SelectionItem::new("A"),
            SelectionItem::new("B").disabled(true),
        ]);
        list.select(1); // Should not select disabled
        assert!(!list.is_selected(1));
    }

    #[test]
    fn test_selection_deselect() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 1]);
        list.deselect(0);
        assert!(!list.is_selected(0));
        assert!(list.is_selected(1));
    }

    #[test]
    fn test_selection_deselect_all() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![0, 1, 2]);
        list.deselect_all();
        assert!(list.selected.is_empty());
    }

    #[test]
    fn test_selection_deselect_all_with_min() {
        let mut list = SelectionList::new(vec!["A", "B", "C"])
            .selected(vec![0, 1, 2])
            .min_selections(1);
        list.deselect_all();
        assert_eq!(list.selected.len(), 1); // Keeps minimum
    }

    #[test]
    fn test_selection_select_all() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.select_all();
        assert_eq!(list.selected.len(), 3);
    }

    #[test]
    fn test_selection_select_all_with_disabled() {
        let mut list = SelectionList::new(vec![
            SelectionItem::new("A"),
            SelectionItem::new("B").disabled(true),
            SelectionItem::new("C"),
        ]);
        list.select_all();
        assert_eq!(list.selected.len(), 2); // Skips disabled
        assert!(!list.is_selected(1)); // Disabled not selected
    }

    #[test]
    fn test_selection_select_all_with_max() {
        let mut list = SelectionList::new(vec!["A", "B", "C", "D", "E"]).max_selections(3);
        list.select_all();
        assert_eq!(list.selected.len(), 3); // Limited by max
    }

    #[test]
    fn test_selection_max() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]).max_selections(2);
        list.toggle(0);
        list.toggle(1);
        list.toggle(2); // Should not add
        assert_eq!(list.selected.len(), 2);
        assert!(!list.is_selected(2));
    }

    #[test]
    fn test_selection_min() {
        let mut list = SelectionList::new(vec!["A", "B", "C"])
            .selected(vec![0, 1])
            .min_selections(1);
        list.toggle(0);
        list.toggle(1); // Should not remove last
        assert_eq!(list.selected.len(), 1);
    }

    // =========================================================================
    // Navigation Tests
    // =========================================================================

    #[test]
    fn test_navigation() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        assert_eq!(list.highlighted, 0);
        list.highlight_next();
        assert_eq!(list.highlighted, 1);
        list.highlight_previous();
        assert_eq!(list.highlighted, 0);
        list.highlight_last();
        assert_eq!(list.highlighted, 2);
        list.highlight_first();
        assert_eq!(list.highlighted, 0);
    }

    #[test]
    fn test_highlight_next_at_end() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.highlight_last();
        list.highlight_next();
        assert_eq!(list.highlighted, 2); // Stays at end
    }

    #[test]
    fn test_highlight_previous_at_start() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.highlight_previous();
        assert_eq!(list.highlighted, 0); // Stays at start
    }

    #[test]
    fn test_highlight_first() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]).selected(vec![2]);
        list.highlight_first();
        assert_eq!(list.highlighted, 0);
        assert_eq!(list.scroll_offset, 0);
    }

    #[test]
    fn test_highlight_last() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.highlight_last();
        assert_eq!(list.highlighted, 2);
    }

    #[test]
    fn test_highlight_empty_list() {
        let mut list = SelectionList::new(vec![] as Vec<&str>);
        list.highlight_next();
        list.highlight_previous();
        assert_eq!(list.highlighted, 0);
    }

    // =========================================================================
    // Item Prefix/Suffix Tests
    // =========================================================================

    #[test]
    fn test_item_prefix_checkbox_selected() {
        let list = SelectionList::new(vec!["A", "B"])
            .style(SelectionStyle::Checkbox)
            .selected(vec![0]);
        assert_eq!(list.item_prefix(0), "[x] ");
    }

    #[test]
    fn test_item_prefix_checkbox_unselected() {
        let list = SelectionList::new(vec!["A", "B"]).style(SelectionStyle::Checkbox);
        assert_eq!(list.item_prefix(0), "[ ] ");
    }

    #[test]
    fn test_item_prefix_checkbox_disabled() {
        let list = SelectionList::new(vec![SelectionItem::new("A").disabled(true)])
            .style(SelectionStyle::Checkbox);
        assert_eq!(list.item_prefix(0), "[-] ");
    }

    #[test]
    fn test_item_prefix_bullet_selected() {
        let list = SelectionList::new(vec!["A", "B"])
            .style(SelectionStyle::Bullet)
            .selected(vec![0]);
        assert_eq!(list.item_prefix(0), "● ");
    }

    #[test]
    fn test_item_prefix_bullet_unselected() {
        let list = SelectionList::new(vec!["A", "B"]).style(SelectionStyle::Bullet);
        assert_eq!(list.item_prefix(0), "○ ");
    }

    #[test]
    fn test_item_prefix_bullet_disabled() {
        let list = SelectionList::new(vec![SelectionItem::new("A").disabled(true)])
            .style(SelectionStyle::Bullet);
        assert_eq!(list.item_prefix(0), "◌ ");
    }

    #[test]
    fn test_item_prefix_highlight_selected() {
        let list = SelectionList::new(vec!["A"])
            .style(SelectionStyle::Highlight)
            .selected(vec![0]);
        assert_eq!(list.item_prefix(0), "▸ ");
    }

    #[test]
    fn test_item_prefix_highlight_unselected() {
        let list = SelectionList::new(vec!["A"]).style(SelectionStyle::Highlight);
        assert_eq!(list.item_prefix(0), "  ");
    }

    #[test]
    fn test_item_prefix_bracket_selected() {
        let list = SelectionList::new(vec!["A"])
            .style(SelectionStyle::Bracket)
            .selected(vec![0]);
        assert_eq!(list.item_prefix(0), "[");
    }

    #[test]
    fn test_item_prefix_bracket_unselected() {
        let list = SelectionList::new(vec!["A"]).style(SelectionStyle::Bracket);
        assert_eq!(list.item_prefix(0), " ");
    }

    #[test]
    fn test_item_suffix_bracket_selected() {
        let list = SelectionList::new(vec!["A"])
            .style(SelectionStyle::Bracket)
            .selected(vec![0]);
        assert_eq!(list.item_suffix(0), "]");
    }

    #[test]
    fn test_item_suffix_bracket_unselected() {
        let list = SelectionList::new(vec!["A"]).style(SelectionStyle::Bracket);
        assert_eq!(list.item_suffix(0), "");
    }

    #[test]
    fn test_item_suffix_non_bracket_style() {
        let list = SelectionList::new(vec!["A"])
            .style(SelectionStyle::Checkbox)
            .selected(vec![0]);
        assert_eq!(list.item_suffix(0), "");
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_helper_functions() {
        let list = selection_list(vec!["A", "B", "C"]);
        assert_eq!(list.items.len(), 3);

        let item = selection_item("Test").value("test_value");
        assert_eq!(item.value, Some("test_value".to_string()));
    }

    #[test]
    fn test_selection_list_helper_with_strings() {
        let list = selection_list(vec![String::from("A"), String::from("B")]);
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_selection_list_helper_with_items() {
        let list = selection_list(vec![selection_item("A"), selection_item("B")]);
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_selection_item_helper_chain() {
        let item = selection_item("Test").value("val").description("desc");
        assert_eq!(item.text, "Test");
        assert_eq!(item.value, Some("val".to_string()));
        assert_eq!(item.description, Some("desc".to_string()));
    }

    // =========================================================================
    // Clone Tests
    // =========================================================================

    #[test]
    fn test_selection_list_clone() {
        let list1 = SelectionList::new(vec!["A", "B", "C"])
            .selected(vec![0, 2])
            .style(SelectionStyle::Bullet)
            .title("Test");
        let list2 = list1.clone();

        assert_eq!(list1.items.len(), list2.items.len());
        assert_eq!(list1.selected, list2.selected);
        assert_eq!(list1.style, list2.style);
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

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_selection_list_builder_chain() {
        let list = SelectionList::new(vec!["A", "B", "C"])
            .selected(vec![0, 2])
            .style(SelectionStyle::Bullet)
            .max_selections(5)
            .min_selections(1)
            .show_descriptions(true)
            .title("Choose:")
            .fg(Color::WHITE)
            .selected_fg(Color::GREEN)
            .highlighted_fg(Color::CYAN)
            .bg(Color::BLACK)
            .max_visible(10)
            .show_count(true)
            .focused(true);

        assert_eq!(list.selected, vec![0, 2]);
        assert_eq!(list.style, SelectionStyle::Bullet);
        assert_eq!(list.max_selections, 5);
        assert_eq!(list.min_selections, 1);
        assert!(list.show_descriptions);
        assert_eq!(list.title, Some("Choose:".to_string()));
        assert_eq!(list.fg, Some(Color::WHITE));
        assert_eq!(list.selected_fg, Some(Color::GREEN));
        assert_eq!(list.highlighted_fg, Some(Color::CYAN));
        assert_eq!(list.bg, Some(Color::BLACK));
        assert_eq!(list.max_visible, 10);
        assert!(list.show_count);
        assert!(list.focused);
    }

    #[test]
    fn test_selection_list_unicode_items() {
        let list = SelectionList::new(vec!["사과", "바나나", "체리"]);
        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[0].text, "사과");
    }

    #[test]
    fn test_selection_list_emoji_icons() {
        let list = SelectionList::new(vec![
            SelectionItem::new("Apple").icon("🍎"),
            SelectionItem::new("Banana").icon("🍌"),
        ]);
        assert_eq!(list.items[0].icon, Some("🍎".to_string()));
        assert_eq!(list.items[1].icon, Some("🍌".to_string()));
    }

    #[test]
    fn test_selection_list_long_descriptions() {
        let long_desc = "A".repeat(1000);
        let expected_desc = long_desc.clone();
        let list = SelectionList::new(vec![SelectionItem::new("Item").description(long_desc)]);
        assert_eq!(list.items[0].description, Some(expected_desc));
    }

    #[test]
    fn test_selection_list_empty_item_text() {
        let list = SelectionList::new(vec![""]);
        assert_eq!(list.items[0].text, "");
    }

    #[test]
    fn test_selection_list_select_then_deselect_same() {
        let mut list = SelectionList::new(vec!["A", "B"]);
        list.toggle(0);
        assert!(list.is_selected(0));
        list.toggle(0);
        assert!(!list.is_selected(0));
    }

    #[test]
    fn test_selection_list_multiple_same_index() {
        let mut list = SelectionList::new(vec!["A", "B", "C"]);
        list.toggle(1);
        list.toggle(1); // Toggle again
        list.toggle(1); // And again
        assert!(list.is_selected(1)); // Should end selected (odd number of toggles)
    }

    #[test]
    fn test_selection_list_scroll_offset_with_max_visible() {
        let mut list = SelectionList::new((0..20).map(|i| format!("Item {}", i))).max_visible(5);

        list.highlight_last();
        assert!(list.scroll_offset > 0); // Should have scrolled
    }
}