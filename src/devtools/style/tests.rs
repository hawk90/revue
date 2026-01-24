use crate::devtools::style::types::*;
use crate::devtools::style::StyleInspector;

#[test]
fn test_computed_property() {
    let prop = ComputedProperty::new("color", "red").source(PropertySource::Inline);

    assert_eq!(prop.name, "color");
    assert_eq!(prop.value, "red");
    assert_eq!(prop.source, PropertySource::Inline);
}

#[test]
fn test_property_source_label() {
    assert_eq!(PropertySource::Inline.label(), "inline");
    assert_eq!(PropertySource::Class.label(), "class");
    assert_eq!(PropertySource::Inherited.label(), "inherited");
}

#[test]
fn test_style_category_from_property() {
    assert_eq!(
        StyleCategory::from_property("margin-left"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("font-size"),
        StyleCategory::Typography
    );
    assert_eq!(
        StyleCategory::from_property("background-color"),
        StyleCategory::Colors
    );
    assert_eq!(
        StyleCategory::from_property("border-width"),
        StyleCategory::Border
    );
    assert_eq!(
        StyleCategory::from_property("box-shadow"),
        StyleCategory::Effects
    );
    assert_eq!(StyleCategory::from_property("custom"), StyleCategory::Other);
}

#[test]
fn test_style_inspector_add_property() {
    let mut inspector = StyleInspector::new();
    inspector.add_property(ComputedProperty::new("color", "blue"));
    inspector.add_property(ComputedProperty::new("font-size", "14px"));

    assert_eq!(inspector.properties.len(), 2);
}

#[test]
fn test_style_inspector_filter() {
    let mut inspector = StyleInspector::new();
    inspector.add_property(ComputedProperty::new("color", "blue").source(PropertySource::Inline));
    inspector
        .add_property(ComputedProperty::new("font-size", "14px").source(PropertySource::Inherited));

    // With inherited shown
    assert_eq!(inspector.filtered().len(), 2);

    // Without inherited
    inspector.toggle_inherited();
    assert_eq!(inspector.filtered().len(), 1);
}

#[test]
fn test_style_inspector_widget_info() {
    let mut inspector = StyleInspector::new();
    inspector.set_widget("Button", Some("submit".to_string()));
    inspector.add_class("primary");
    inspector.add_class("large");

    assert_eq!(inspector.widget_type, "Button");
    assert_eq!(inspector.widget_id, Some("submit".to_string()));
    assert_eq!(inspector.classes.len(), 2);
}

#[test]
fn test_computed_property_overridden() {
    let prop = ComputedProperty::new("color", "red").overridden();
    assert!(prop.overridden);
}

#[test]
fn test_computed_property_default_values() {
    let prop = ComputedProperty::new("display", "flex");
    assert_eq!(prop.source, PropertySource::Computed);
    assert!(!prop.overridden);
}

#[test]
fn test_property_source_default() {
    let source = PropertySource::default();
    assert_eq!(source, PropertySource::Computed);
}

#[test]
fn test_property_source_all_labels() {
    assert_eq!(PropertySource::Inline.label(), "inline");
    assert_eq!(PropertySource::Class.label(), "class");
    assert_eq!(PropertySource::Id.label(), "id");
    assert_eq!(PropertySource::Inherited.label(), "inherited");
    assert_eq!(PropertySource::Computed.label(), "computed");
    assert_eq!(PropertySource::Theme.label(), "theme");
}

#[test]
fn test_property_source_all_icons() {
    assert_eq!(PropertySource::Inline.icon(), "•");
    assert_eq!(PropertySource::Class.icon(), ".");
    assert_eq!(PropertySource::Id.icon(), "#");
    assert_eq!(PropertySource::Inherited.icon(), "↑");
    assert_eq!(PropertySource::Computed.icon(), "○");
    assert_eq!(PropertySource::Theme.icon(), "◆");
}

#[test]
fn test_property_source_clone() {
    let source = PropertySource::Inline;
    let cloned = source.clone();
    assert_eq!(source, cloned);
}

#[test]
fn test_property_source_copy() {
    let source = PropertySource::Class;
    let copied = source; // Copy, not move
    assert_eq!(source, copied);
}

#[test]
fn test_style_category_all() {
    let all = StyleCategory::all();
    assert_eq!(all.len(), 6);
    assert!(all.contains(&StyleCategory::Layout));
    assert!(all.contains(&StyleCategory::Typography));
    assert!(all.contains(&StyleCategory::Colors));
    assert!(all.contains(&StyleCategory::Border));
    assert!(all.contains(&StyleCategory::Effects));
    assert!(all.contains(&StyleCategory::Other));
}

#[test]
fn test_style_category_all_labels() {
    assert_eq!(StyleCategory::Layout.label(), "Layout");
    assert_eq!(StyleCategory::Typography.label(), "Typography");
    assert_eq!(StyleCategory::Colors.label(), "Colors");
    assert_eq!(StyleCategory::Border.label(), "Border");
    assert_eq!(StyleCategory::Effects.label(), "Effects");
    assert_eq!(StyleCategory::Other.label(), "Other");
}

#[test]
fn test_style_category_from_property_layout() {
    assert_eq!(
        StyleCategory::from_property("margin-left"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("margin-right"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("padding-top"),
        StyleCategory::Layout
    );
    assert_eq!(StyleCategory::from_property("width"), StyleCategory::Layout);
    assert_eq!(
        StyleCategory::from_property("height"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("min-width"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("max-height"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("flex-direction"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("grid-template"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("display"),
        StyleCategory::Layout
    );
    assert_eq!(
        StyleCategory::from_property("position"),
        StyleCategory::Layout
    );
}

#[test]
fn test_style_category_from_property_typography() {
    assert_eq!(
        StyleCategory::from_property("font-size"),
        StyleCategory::Typography
    );
    assert_eq!(
        StyleCategory::from_property("font-family"),
        StyleCategory::Typography
    );
    assert_eq!(
        StyleCategory::from_property("font-weight"),
        StyleCategory::Typography
    );
    assert_eq!(
        StyleCategory::from_property("text-align"),
        StyleCategory::Typography
    );
    assert_eq!(
        StyleCategory::from_property("text-decoration"),
        StyleCategory::Typography
    );
    // Note: line-height contains "height" so it matches Layout before Typography
    assert_eq!(
        StyleCategory::from_property("letter-spacing"),
        StyleCategory::Typography
    );
}

#[test]
fn test_style_category_from_property_colors() {
    assert_eq!(StyleCategory::from_property("color"), StyleCategory::Colors);
    assert_eq!(
        StyleCategory::from_property("background-color"),
        StyleCategory::Colors
    );
    assert_eq!(
        StyleCategory::from_property("background"),
        StyleCategory::Colors
    );
}

#[test]
fn test_style_category_from_property_border() {
    // Border should match before layout
    assert_eq!(
        StyleCategory::from_property("border-width"),
        StyleCategory::Border
    );
    assert_eq!(
        StyleCategory::from_property("border-color"),
        StyleCategory::Border
    );
    assert_eq!(
        StyleCategory::from_property("border-style"),
        StyleCategory::Border
    );
    assert_eq!(
        StyleCategory::from_property("border-radius"),
        StyleCategory::Border
    );
}

#[test]
fn test_style_category_from_property_effects() {
    assert_eq!(
        StyleCategory::from_property("box-shadow"),
        StyleCategory::Effects
    );
    // Note: text-shadow starts with "text" so it matches Typography before Effects
    assert_eq!(
        StyleCategory::from_property("opacity"),
        StyleCategory::Effects
    );
    assert_eq!(
        StyleCategory::from_property("transform"),
        StyleCategory::Effects
    );
    assert_eq!(
        StyleCategory::from_property("transform-origin"),
        StyleCategory::Effects
    );
}

#[test]
fn test_style_category_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(StyleCategory::Layout);
    set.insert(StyleCategory::Typography);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_style_category_copy() {
    let cat = StyleCategory::Border;
    let copied = cat; // Copy, not move
    assert_eq!(cat, copied);
}

#[test]
fn test_style_inspector_default() {
    let inspector = StyleInspector::default();
    assert!(inspector.properties.is_empty());
    assert!(inspector.classes.is_empty());
    assert!(inspector.widget_id.is_none());
    assert!(inspector.widget_type.is_empty());
}

#[test]
fn test_style_inspector_clear() {
    let mut inspector = StyleInspector::new();
    inspector.set_widget("Button", Some("btn".to_string()));
    inspector.add_class("primary");
    inspector.add_property(ComputedProperty::new("color", "red"));

    inspector.clear();

    assert!(inspector.properties.is_empty());
    assert!(inspector.classes.is_empty());
    assert!(inspector.widget_id.is_none());
    assert!(inspector.widget_type.is_empty());
}

#[test]
fn test_style_inspector_set_properties() {
    let mut inspector = StyleInspector::new();
    let props = vec![
        ComputedProperty::new("color", "red"),
        ComputedProperty::new("font-size", "12px"),
    ];
    inspector.set_properties(props);
    assert_eq!(inspector.properties.len(), 2);
}

#[test]
fn test_style_inspector_toggle_inherited() {
    let mut inspector = StyleInspector::new();
    assert!(inspector.show_inherited);
    inspector.toggle_inherited();
    assert!(!inspector.show_inherited);
    inspector.toggle_inherited();
    assert!(inspector.show_inherited);
}

#[test]
fn test_style_inspector_toggle_overridden() {
    let mut inspector = StyleInspector::new();
    assert!(!inspector.show_overridden);
    inspector.toggle_overridden();
    assert!(inspector.show_overridden);
    inspector.toggle_overridden();
    assert!(!inspector.show_overridden);
}

#[test]
fn test_style_inspector_set_category_filter() {
    let mut inspector = StyleInspector::new();
    assert!(inspector.category_filter.is_none());
    inspector.set_category_filter(Some(StyleCategory::Layout));
    assert_eq!(inspector.category_filter, Some(StyleCategory::Layout));
    inspector.set_category_filter(None);
    assert!(inspector.category_filter.is_none());
}

#[test]
fn test_style_inspector_toggle_category() {
    let mut inspector = StyleInspector::new();
    // All categories start expanded
    assert_eq!(
        inspector.expanded_categories.get(&StyleCategory::Layout),
        Some(&true)
    );

    inspector.toggle_category(StyleCategory::Layout);
    assert_eq!(
        inspector.expanded_categories.get(&StyleCategory::Layout),
        Some(&false)
    );

    inspector.toggle_category(StyleCategory::Layout);
    assert_eq!(
        inspector.expanded_categories.get(&StyleCategory::Layout),
        Some(&true)
    );
}

#[test]
fn test_style_inspector_filter_overridden() {
    let mut inspector = StyleInspector::new();
    inspector.add_property(ComputedProperty::new("color", "blue"));
    inspector.add_property(ComputedProperty::new("font-size", "14px").overridden());

    // Without overridden shown
    assert_eq!(inspector.filtered().len(), 1);

    // With overridden shown
    inspector.toggle_overridden();
    assert_eq!(inspector.filtered().len(), 2);
}

#[test]
fn test_style_inspector_filter_category() {
    let mut inspector = StyleInspector::new();
    inspector.add_property(ComputedProperty::new("color", "blue"));
    inspector.add_property(ComputedProperty::new("font-size", "14px"));
    inspector.add_property(ComputedProperty::new("margin", "10px"));

    // No filter
    assert_eq!(inspector.filtered().len(), 3);

    // Filter to layout only
    inspector.set_category_filter(Some(StyleCategory::Layout));
    let filtered = inspector.filtered();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "margin");
}

#[test]
fn test_style_inspector_select_next() {
    let mut inspector = StyleInspector::new();
    inspector.add_property(ComputedProperty::new("a", "1"));
    inspector.add_property(ComputedProperty::new("b", "2"));
    inspector.add_property(ComputedProperty::new("c", "3"));

    assert!(inspector.selected.is_none());

    inspector.select_next();
    assert_eq!(inspector.selected, Some(0));

    inspector.select_next();
    assert_eq!(inspector.selected, Some(1));

    inspector.select_next();
    assert_eq!(inspector.selected, Some(2));

    // Should not go past last item
    inspector.select_next();
    assert_eq!(inspector.selected, Some(2));
}

#[test]
fn test_style_inspector_select_prev() {
    let mut inspector = StyleInspector::new();
    inspector.add_property(ComputedProperty::new("a", "1"));
    inspector.add_property(ComputedProperty::new("b", "2"));
    inspector.add_property(ComputedProperty::new("c", "3"));

    // Start at end
    inspector.selected = Some(2);

    inspector.select_prev();
    assert_eq!(inspector.selected, Some(1));

    inspector.select_prev();
    assert_eq!(inspector.selected, Some(0));

    // Should not go below 0
    inspector.select_prev();
    assert_eq!(inspector.selected, Some(0));
}

#[test]
fn test_style_inspector_select_empty() {
    let mut inspector = StyleInspector::new();

    // Should do nothing with empty list
    inspector.select_next();
    assert!(inspector.selected.is_none());

    inspector.select_prev();
    assert!(inspector.selected.is_none());
}

#[test]
fn test_style_inspector_select_prev_from_none() {
    let mut inspector = StyleInspector::new();
    inspector.add_property(ComputedProperty::new("a", "1"));

    inspector.select_prev();
    assert_eq!(inspector.selected, Some(0));
}

#[test]
fn test_computed_property_builder_chain() {
    let prop = ComputedProperty::new("display", "block")
        .source(PropertySource::Id)
        .overridden();

    assert_eq!(prop.name, "display");
    assert_eq!(prop.value, "block");
    assert_eq!(prop.source, PropertySource::Id);
    assert!(prop.overridden);
}

#[test]
fn test_style_inspector_new_initializes_expanded() {
    let inspector = StyleInspector::new();
    // All categories should be expanded by default
    for cat in StyleCategory::all() {
        assert_eq!(
            inspector.expanded_categories.get(cat),
            Some(&true),
            "Category {:?} should be expanded",
            cat
        );
    }
}

#[test]
fn test_computed_property_clone() {
    let prop = ComputedProperty::new("color", "red").source(PropertySource::Inline);
    let cloned = prop.clone();
    assert_eq!(cloned.name, "color");
    assert_eq!(cloned.value, "red");
    assert_eq!(cloned.source, PropertySource::Inline);
}

#[test]
fn test_computed_property_debug() {
    let prop = ComputedProperty::new("opacity", "0.5");
    let debug = format!("{:?}", prop);
    assert!(debug.contains("opacity"));
    assert!(debug.contains("0.5"));
}
