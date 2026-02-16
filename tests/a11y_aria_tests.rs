//! Tests for ARIA attribute utilities

use revue::utils::accessibility::{
    aria::{aria_attributes, aria_pairs, AriaAttribute, AriaBuilder, LiveRegion},
    AccessibleNode, AccessibleState, Role,
};

// =========================================================================
// AriaAttribute::name() tests
// =========================================================================

#[test]
fn test_aria_attribute_name_role() {
    assert_eq!(AriaAttribute::Role("button".to_string()).name(), "role");
}

#[test]
fn test_aria_attribute_name_label() {
    assert_eq!(
        AriaAttribute::Label("test".to_string()).name(),
        "aria-label"
    );
}

#[test]
fn test_aria_attribute_name_labelled_by() {
    assert_eq!(
        AriaAttribute::LabelledBy("id1".to_string()).name(),
        "aria-labelledby"
    );
}

#[test]
fn test_aria_attribute_name_described_by() {
    assert_eq!(
        AriaAttribute::DescribedBy("id2".to_string()).name(),
        "aria-describedby"
    );
}

#[test]
fn test_aria_attribute_name_live() {
    assert_eq!(
        AriaAttribute::Live("polite".to_string()).name(),
        "aria-live"
    );
}

#[test]
fn test_aria_attribute_name_atomic() {
    assert_eq!(AriaAttribute::Atomic(true).name(), "aria-atomic");
}

#[test]
fn test_aria_attribute_name_hidden() {
    assert_eq!(AriaAttribute::Hidden(true).name(), "aria-hidden");
}

#[test]
fn test_aria_attribute_name_expanded() {
    assert_eq!(AriaAttribute::Expanded(Some(true)).name(), "aria-expanded");
}

#[test]
fn test_aria_attribute_name_checked() {
    assert_eq!(AriaAttribute::Checked(Some(false)).name(), "aria-checked");
}

#[test]
fn test_aria_attribute_name_selected() {
    assert_eq!(AriaAttribute::Selected(true).name(), "aria-selected");
}

#[test]
fn test_aria_attribute_name_pressed() {
    assert_eq!(AriaAttribute::Pressed(Some(true)).name(), "aria-pressed");
}

#[test]
fn test_aria_attribute_name_disabled() {
    assert_eq!(AriaAttribute::Disabled(true).name(), "aria-disabled");
}

#[test]
fn test_aria_attribute_name_read_only() {
    assert_eq!(AriaAttribute::ReadOnly(true).name(), "aria-readonly");
}

#[test]
fn test_aria_attribute_name_required() {
    assert_eq!(AriaAttribute::Required(true).name(), "aria-required");
}

#[test]
fn test_aria_attribute_name_invalid() {
    assert_eq!(AriaAttribute::Invalid(true).name(), "aria-invalid");
}

#[test]
fn test_aria_attribute_name_modal() {
    assert_eq!(AriaAttribute::Modal(true).name(), "aria-modal");
}

#[test]
fn test_aria_attribute_name_current() {
    assert_eq!(
        AriaAttribute::Current(Some("page".to_string())).name(),
        "aria-current"
    );
}

#[test]
fn test_aria_attribute_name_has_popup() {
    assert_eq!(
        AriaAttribute::HasPopup(Some("menu".to_string())).name(),
        "aria-haspopup"
    );
}

#[test]
fn test_aria_attribute_name_orientation() {
    assert_eq!(
        AriaAttribute::Orientation("horizontal".to_string()).name(),
        "aria-orientation"
    );
}

#[test]
fn test_aria_attribute_name_value_min() {
    assert_eq!(AriaAttribute::ValueMin(0.0).name(), "aria-valuemin");
}

#[test]
fn test_aria_attribute_name_value_max() {
    assert_eq!(AriaAttribute::ValueMax(100.0).name(), "aria-valuemax");
}

#[test]
fn test_aria_attribute_name_value_now() {
    assert_eq!(AriaAttribute::ValueNow(50.0).name(), "aria-valuenow");
}

#[test]
fn test_aria_attribute_name_value_text() {
    assert_eq!(
        AriaAttribute::ValueText("50%".to_string()).name(),
        "aria-valuetext"
    );
}

#[test]
fn test_aria_attribute_name_pos_in_set() {
    assert_eq!(AriaAttribute::PosInSet(1).name(), "aria-posinset");
}

#[test]
fn test_aria_attribute_name_set_size() {
    assert_eq!(AriaAttribute::SetSize(5).name(), "aria-setsize");
}

#[test]
fn test_aria_attribute_name_level() {
    assert_eq!(AriaAttribute::Level(2).name(), "aria-level");
}

#[test]
fn test_aria_attribute_name_error_message() {
    assert_eq!(
        AriaAttribute::ErrorMessage("Error".to_string()).name(),
        "aria-errormessage"
    );
}

#[test]
fn test_aria_attribute_name_tab_index() {
    assert_eq!(AriaAttribute::TabIndex(0).name(), "tabindex");
}

#[test]
fn test_aria_attribute_name_alt() {
    assert_eq!(AriaAttribute::Alt("Image alt".to_string()).name(), "alt");
}

#[test]
fn test_aria_attribute_name_custom() {
    assert_eq!(
        AriaAttribute::Custom("data-custom".to_string(), "value".to_string()).name(),
        "data-custom"
    );
}

// =========================================================================
// AriaAttribute::value() tests
// =========================================================================

#[test]
fn test_aria_attribute_value_role() {
    assert_eq!(AriaAttribute::Role("button".to_string()).value(), "button");
}

#[test]
fn test_aria_attribute_value_label() {
    assert_eq!(AriaAttribute::Label("Submit".to_string()).value(), "Submit");
}

#[test]
fn test_aria_attribute_value_atomic_true() {
    assert_eq!(AriaAttribute::Atomic(true).value(), "true");
}

#[test]
fn test_aria_attribute_value_atomic_false() {
    assert_eq!(AriaAttribute::Atomic(false).value(), "false");
}

#[test]
fn test_aria_attribute_value_hidden_true() {
    assert_eq!(AriaAttribute::Hidden(true).value(), "true");
}

#[test]
fn test_aria_attribute_value_hidden_false() {
    assert_eq!(AriaAttribute::Hidden(false).value(), "false");
}

#[test]
fn test_aria_attribute_value_expanded_some() {
    assert_eq!(AriaAttribute::Expanded(Some(true)).value(), "true");
}

#[test]
fn test_aria_attribute_value_expanded_none() {
    assert_eq!(AriaAttribute::Expanded(None).value(), "undefined");
}

#[test]
fn test_aria_attribute_value_checked_some() {
    assert_eq!(AriaAttribute::Checked(Some(true)).value(), "true");
}

#[test]
fn test_aria_attribute_value_checked_none() {
    assert_eq!(AriaAttribute::Checked(None).value(), "mixed");
}

#[test]
fn test_aria_attribute_value_selected_true() {
    assert_eq!(AriaAttribute::Selected(true).value(), "true");
}

#[test]
fn test_aria_attribute_value_selected_false() {
    assert_eq!(AriaAttribute::Selected(false).value(), "false");
}

#[test]
fn test_aria_attribute_value_pressed_some() {
    assert_eq!(AriaAttribute::Pressed(Some(true)).value(), "true");
}

#[test]
fn test_aria_attribute_value_pressed_none() {
    assert_eq!(AriaAttribute::Pressed(None).value(), "mixed");
}

#[test]
fn test_aria_attribute_value_disabled_true() {
    assert_eq!(AriaAttribute::Disabled(true).value(), "true");
}

#[test]
fn test_aria_attribute_value_disabled_false() {
    assert_eq!(AriaAttribute::Disabled(false).value(), "false");
}

#[test]
fn test_aria_attribute_value_read_only() {
    assert_eq!(AriaAttribute::ReadOnly(true).value(), "true");
}

#[test]
fn test_aria_attribute_value_required() {
    assert_eq!(AriaAttribute::Required(true).value(), "true");
}

#[test]
fn test_aria_attribute_value_invalid() {
    assert_eq!(AriaAttribute::Invalid(true).value(), "true");
}

#[test]
fn test_aria_attribute_value_modal() {
    assert_eq!(AriaAttribute::Modal(true).value(), "true");
}

#[test]
fn test_aria_attribute_value_current_some() {
    assert_eq!(
        AriaAttribute::Current(Some("page".to_string())).value(),
        "page"
    );
}

#[test]
fn test_aria_attribute_value_current_none() {
    assert_eq!(AriaAttribute::Current(None).value(), "false");
}

#[test]
fn test_aria_attribute_value_has_popup_some() {
    assert_eq!(
        AriaAttribute::HasPopup(Some("menu".to_string())).value(),
        "menu"
    );
}

#[test]
fn test_aria_attribute_value_has_popup_none() {
    assert_eq!(AriaAttribute::HasPopup(None).value(), "false");
}

#[test]
fn test_aria_attribute_value_orientation() {
    assert_eq!(
        AriaAttribute::Orientation("horizontal".to_string()).value(),
        "horizontal"
    );
}

#[test]
fn test_aria_attribute_value_value_min() {
    assert_eq!(AriaAttribute::ValueMin(0.0).value(), "0");
}

#[test]
fn test_aria_attribute_value_value_max() {
    assert_eq!(AriaAttribute::ValueMax(100.0).value(), "100");
}

#[test]
fn test_aria_attribute_value_value_now() {
    assert_eq!(AriaAttribute::ValueNow(50.0).value(), "50");
}

#[test]
fn test_aria_attribute_value_value_text() {
    assert_eq!(AriaAttribute::ValueText("50%".to_string()).value(), "50%");
}

#[test]
fn test_aria_attribute_value_pos_in_set() {
    assert_eq!(AriaAttribute::PosInSet(1).value(), "1");
}

#[test]
fn test_aria_attribute_value_set_size() {
    assert_eq!(AriaAttribute::SetSize(5).value(), "5");
}

#[test]
fn test_aria_attribute_value_level() {
    assert_eq!(AriaAttribute::Level(2).value(), "2");
}

#[test]
fn test_aria_attribute_value_error_message() {
    assert_eq!(
        AriaAttribute::ErrorMessage("Error".to_string()).value(),
        "Error"
    );
}

#[test]
fn test_aria_attribute_value_tab_index() {
    assert_eq!(AriaAttribute::TabIndex(0).value(), "0");
}

#[test]
fn test_aria_attribute_value_alt() {
    assert_eq!(
        AriaAttribute::Alt("Alt text".to_string()).value(),
        "Alt text"
    );
}

#[test]
fn test_aria_attribute_value_custom() {
    assert_eq!(
        AriaAttribute::Custom("data-x".to_string(), "y".to_string()).value(),
        "y"
    );
}

// =========================================================================
// LiveRegion tests
// =========================================================================

#[test]
fn test_live_region_off_as_str() {
    assert_eq!(LiveRegion::Off.as_str(), "off");
}

#[test]
fn test_live_region_polite_as_str() {
    assert_eq!(LiveRegion::Polite.as_str(), "polite");
}

#[test]
fn test_live_region_assertive_as_str() {
    assert_eq!(LiveRegion::Assertive.as_str(), "assertive");
}

#[test]
fn test_live_region_default() {
    assert_eq!(LiveRegion::default(), LiveRegion::Polite);
}

#[test]
fn test_live_region_clone() {
    let region = LiveRegion::Assertive;
    let cloned = region.clone();
    assert_eq!(region, cloned);
}

#[test]
fn test_live_region_copy() {
    let region = LiveRegion::Assertive;
    let copied = region;
    assert_eq!(region, LiveRegion::Assertive);
    assert_eq!(copied, LiveRegion::Assertive);
}

#[test]
fn test_live_region_partial_eq() {
    assert_eq!(LiveRegion::Polite, LiveRegion::Polite);
    assert_ne!(LiveRegion::Polite, LiveRegion::Off);
}

// =========================================================================
// AriaBuilder tests
// =========================================================================

#[test]
fn test_aria_builder_new() {
    let builder = AriaBuilder::new();
    assert!(!builder.is_atomic());
    assert_eq!(builder.get_live_region(), None);
}

#[test]
fn test_aria_builder_default() {
    let builder = AriaBuilder::default();
    assert!(!builder.is_atomic());
    assert_eq!(builder.get_live_region(), None);
}

#[test]
fn test_aria_builder_live_region() {
    let builder = AriaBuilder::new().live_region(LiveRegion::Polite);
    assert_eq!(builder.get_live_region(), Some(LiveRegion::Polite));
}

#[test]
fn test_aria_builder_live_region_assertive() {
    let builder = AriaBuilder::new().live_region(LiveRegion::Assertive);
    assert_eq!(builder.get_live_region(), Some(LiveRegion::Assertive));
}

#[test]
fn test_aria_builder_atomic_true() {
    let builder = AriaBuilder::new().atomic(true);
    assert!(builder.is_atomic());
}

#[test]
fn test_aria_builder_atomic_false() {
    let builder = AriaBuilder::new().atomic(false);
    assert!(!builder.is_atomic());
}

#[test]
fn test_aria_builder_labelled_by() {
    let map = AriaBuilder::new().labelled_by("label1").build();
    assert_eq!(map.get("aria-labelledby"), Some(&"label1".to_string()));
}

#[test]
fn test_aria_builder_described_by() {
    let map = AriaBuilder::new().described_by("desc1").build();
    assert_eq!(map.get("aria-describedby"), Some(&"desc1".to_string()));
}

#[test]
fn test_aria_builder_modal_true() {
    let map = AriaBuilder::new().modal(true).build();
    assert_eq!(map.get("aria-modal"), Some(&"true".to_string()));
}

#[test]
fn test_aria_builder_modal_false() {
    let map = AriaBuilder::new().modal(false).build();
    assert_eq!(map.get("aria-modal"), Some(&"false".to_string()));
}

#[test]
fn test_aria_builder_has_popup_true() {
    let map = AriaBuilder::new().has_popup(true, Some("menu")).build();
    assert_eq!(map.get("aria-haspopup"), Some(&"menu".to_string()));
}

#[test]
fn test_aria_builder_has_popup_false() {
    let map = AriaBuilder::new().has_popup(false, None).build();
    assert_eq!(map.get("aria-haspopup"), Some(&"false".to_string()));
}

#[test]
fn test_aria_builder_orientation() {
    let map = AriaBuilder::new().orientation("horizontal").build();
    assert_eq!(map.get("aria-orientation"), Some(&"horizontal".to_string()));
}

#[test]
fn test_aria_builder_build_empty() {
    let map = AriaBuilder::new().build();
    assert!(map.is_empty());
}

#[test]
fn test_aria_builder_build_with_role() {
    let node = AccessibleNode::new(Role::Button).label("Test");
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("role"), Some(&"button".to_string()));
}

#[test]
fn test_aria_builder_build_pairs_empty() {
    let pairs = AriaBuilder::new().build_pairs();
    assert!(pairs.is_empty());
}

#[test]
fn test_aria_builder_build_pairs_with_role() {
    let node = AccessibleNode::new(Role::Button).label("Test");
    let pairs = AriaBuilder::new().from_node(&node).build_pairs();
    assert!(!pairs.is_empty());
}

#[test]
fn test_aria_builder_from_node_with_label() {
    let node = AccessibleNode::new(Role::Button).label("Submit");
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-label"), Some(&"Submit".to_string()));
}

#[test]
fn test_aria_builder_from_node_with_description() {
    let node = AccessibleNode::new(Role::Button).description("Submit form");
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(
        map.get("aria-describedby"),
        Some(&"Submit form".to_string())
    );
}

#[test]
fn test_aria_builder_from_node_disabled() {
    let node = AccessibleNode::new(Role::Button).state(AccessibleState::new().disabled(true));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-disabled"), Some(&"true".to_string()));
}

#[test]
fn test_aria_builder_from_node_hidden() {
    let mut state = AccessibleState::new();
    state.hidden = true;
    let node = AccessibleNode::new(Role::Button).state(state);
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-hidden"), Some(&"true".to_string()));
}

#[test]
fn test_aria_builder_from_node_expanded() {
    let node = AccessibleNode::new(Role::Button).state(AccessibleState::new().expanded(true));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-expanded"), Some(&"true".to_string()));
}

#[test]
fn test_aria_builder_from_node_checked() {
    let node = AccessibleNode::new(Role::Checkbox).state(AccessibleState::new().checked(true));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-checked"), Some(&"true".to_string()));
}

#[test]
fn test_aria_builder_from_node_selected() {
    let node = AccessibleNode::new(Role::ListItem).state(AccessibleState::new().selected(true));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-selected"), Some(&"true".to_string()));
}

#[test]
fn test_aria_builder_from_node_pressed() {
    let node = AccessibleNode::new(Role::Button).state(AccessibleState::new().pressed(true));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-pressed"), Some(&"true".to_string()));
}

#[test]
fn test_aria_builder_from_node_value_now() {
    let node = AccessibleNode::new(Role::Slider)
        .state(AccessibleState::new().value_range(50.0, 0.0, 100.0));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-valuenow"), Some(&"50".to_string()));
}

#[test]
fn test_aria_builder_from_node_value_min_max() {
    let node = AccessibleNode::new(Role::Slider)
        .state(AccessibleState::new().value_range(50.0, 0.0, 100.0));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-valuemin"), Some(&"0".to_string()));
    assert_eq!(map.get("aria-valuemax"), Some(&"100".to_string()));
}

#[test]
fn test_aria_builder_from_node_value_text() {
    let node = AccessibleNode::new(Role::Slider).state(AccessibleState {
        value_text: Some("50%".to_string()),
        ..Default::default()
    });
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-valuetext"), Some(&"50%".to_string()));
}

#[test]
fn test_aria_builder_from_node_position() {
    let node = AccessibleNode::new(Role::ListItem).state(AccessibleState::new().position(2, 5));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-posinset"), Some(&"2".to_string()));
    assert_eq!(map.get("aria-setsize"), Some(&"5".to_string()));
}

#[test]
fn test_aria_builder_from_node_level() {
    let node = AccessibleNode::new(Role::ListItem).state(AccessibleState::new().level(2));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-level"), Some(&"2".to_string()));
}

#[test]
fn test_aria_builder_from_node_error_message() {
    let node =
        AccessibleNode::new(Role::TextInput).state(AccessibleState::new().error("Invalid input"));
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(
        map.get("aria-errormessage"),
        Some(&"Invalid input".to_string())
    );
}

#[test]
fn test_aria_builder_from_node_focusable() {
    let node = AccessibleNode::new(Role::Button);
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("tabindex"), Some(&"0".to_string()));
}

#[test]
fn test_aria_builder_from_node_not_focusable_disabled() {
    let node = AccessibleNode::new(Role::Button).state(AccessibleState::new().disabled(true));
    let map = AriaBuilder::new().from_node(&node).build();
    // Disabled interactive roles don't get a tabindex attribute
    assert_eq!(map.get("tabindex"), None);
}

#[test]
fn test_aria_builder_from_node_custom_aria_property() {
    let node = AccessibleNode::new(Role::Button).property("aria-describedby", "desc1");
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("aria-describedby"), Some(&"desc1".to_string()));
}

#[test]
fn test_aria_builder_from_node_non_aria_property_ignored() {
    let node = AccessibleNode::new(Role::Button).property("data-custom", "value");
    let map = AriaBuilder::new().from_node(&node).build();
    assert_eq!(map.get("data-custom"), None);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_aria_attributes() {
    let node = AccessibleNode::new(Role::Button).label("Test");
    let map = aria_attributes(&node);
    assert_eq!(map.get("role"), Some(&"button".to_string()));
    assert_eq!(map.get("aria-label"), Some(&"Test".to_string()));
}

#[test]
fn test_aria_pairs() {
    let node = AccessibleNode::new(Role::Button).label("Test");
    let pairs = aria_pairs(&node);
    assert!(!pairs.is_empty());
    assert!(pairs.iter().any(|(k, v)| k == "role" && v == "button"));
}

// =========================================================================
// AriaAttribute trait implementation tests
// =========================================================================

#[test]
fn test_aria_attribute_clone() {
    let attr = AriaAttribute::Role("button".to_string());
    let cloned = attr.clone();
    assert_eq!(attr, cloned);
}

#[test]
fn test_aria_attribute_partial_eq() {
    let attr1 = AriaAttribute::Role("button".to_string());
    let attr2 = AriaAttribute::Role("button".to_string());
    let attr3 = AriaAttribute::Role("link".to_string());
    assert_eq!(attr1, attr2);
    assert_ne!(attr1, attr3);
}
