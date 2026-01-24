//! CSS selector parsing and representation
//!
//! Supports full CSS selector syntax:
//! - Type: `Button`, `Input`
//! - ID: `#submit`, `#main-content`
//! - Class: `.primary`, `.btn-large`
//! - Universal: `*`
//! - Attribute: `[disabled]`, `[type="text"]`
//! - Pseudo-class: `:focus`, `:hover`, `:nth-child(2)`
//! - Combinators: ` ` (descendant), `>` (child), `+` (adjacent), `~` (sibling)
//! - Grouping: `Button, Input` (comma-separated)

#![allow(dead_code)]

mod parser;
mod types;

// Re-export public API
pub use parser::{parse_selector, parse_selectors};
#[cfg(test)]
pub use types::SelectorParseError;
pub use types::{AttributeOp, AttributeSelector, Combinator, PseudoClass, Selector, SelectorPart};

#[cfg(test)]
mod tests {
//! Tests for CSS selector parsing

use super::*;

// PseudoClass tests
#[test]
fn test_pseudo_class_display_focus() {
    assert_eq!(PseudoClass::Focus.to_string(), ":focus");
}

#[test]
fn test_pseudo_class_display_hover() {
    assert_eq!(PseudoClass::Hover.to_string(), ":hover");
}

#[test]
fn test_pseudo_class_display_active() {
    assert_eq!(PseudoClass::Active.to_string(), ":active");
}

#[test]
fn test_pseudo_class_display_disabled() {
    assert_eq!(PseudoClass::Disabled.to_string(), ":disabled");
}

#[test]
fn test_pseudo_class_display_enabled() {
    assert_eq!(PseudoClass::Enabled.to_string(), ":enabled");
}

#[test]
fn test_pseudo_class_display_checked() {
    assert_eq!(PseudoClass::Checked.to_string(), ":checked");
}

#[test]
fn test_pseudo_class_display_selected() {
    assert_eq!(PseudoClass::Selected.to_string(), ":selected");
}

#[test]
fn test_pseudo_class_display_empty() {
    assert_eq!(PseudoClass::Empty.to_string(), ":empty");
}

#[test]
fn test_pseudo_class_display_first_child() {
    assert_eq!(PseudoClass::FirstChild.to_string(), ":first-child");
}

#[test]
fn test_pseudo_class_display_last_child() {
    assert_eq!(PseudoClass::LastChild.to_string(), ":last-child");
}

#[test]
fn test_pseudo_class_display_only_child() {
    assert_eq!(PseudoClass::OnlyChild.to_string(), ":only-child");
}

#[test]
fn test_pseudo_class_display_nth_child() {
    assert_eq!(PseudoClass::NthChild(5).to_string(), ":nth-child(5)");
}

#[test]
fn test_pseudo_class_display_nth_last_child() {
    assert_eq!(
        PseudoClass::NthLastChild(2).to_string(),
        ":nth-last-child(2)"
    );
}

#[test]
fn test_pseudo_class_display_not() {
    let not = PseudoClass::Not(Box::new(PseudoClass::Disabled));
    assert_eq!(not.to_string(), ":not(:disabled)");
}

#[test]
fn test_pseudo_class_clone() {
    let p = PseudoClass::Focus;
    let cloned = p.clone();
    assert_eq!(p, cloned);
}

// SelectorPart tests
#[test]
fn test_selector_part_new() {
    let part = SelectorPart::new();
    assert!(part.is_empty());
}

#[test]
fn test_selector_part_element() {
    let part = SelectorPart::element("Button");
    assert_eq!(part.element, Some("Button".to_string()));
    assert!(!part.is_empty());
}

#[test]
fn test_selector_part_universal() {
    let part = SelectorPart::universal();
    assert!(part.universal);
    assert!(!part.is_empty());
}

#[test]
fn test_selector_part_id() {
    let part = SelectorPart::id("main");
    assert_eq!(part.id, Some("main".to_string()));
}

#[test]
fn test_selector_part_class() {
    let part = SelectorPart::class("primary");
    assert_eq!(part.classes, vec!["primary".to_string()]);
}

#[test]
fn test_selector_part_with_id() {
    let part = SelectorPart::element("Button").with_id("submit");
    assert_eq!(part.element, Some("Button".to_string()));
    assert_eq!(part.id, Some("submit".to_string()));
}

#[test]
fn test_selector_part_with_class() {
    let part = SelectorPart::element("Button")
        .with_class("primary")
        .with_class("large");
    assert_eq!(part.classes.len(), 2);
}

#[test]
fn test_selector_part_with_pseudo() {
    let part = SelectorPart::element("Button").with_pseudo(PseudoClass::Focus);
    assert_eq!(part.pseudo_classes, vec![PseudoClass::Focus]);
}

#[test]
fn test_selector_part_specificity_type() {
    let part = SelectorPart::element("Button");
    assert_eq!(part.specificity(), (0, 0, 1));
}

#[test]
fn test_selector_part_specificity_id() {
    let part = SelectorPart::id("main");
    assert_eq!(part.specificity(), (1, 0, 0));
}

#[test]
fn test_selector_part_specificity_class() {
    let part = SelectorPart::class("primary").with_class("large");
    assert_eq!(part.specificity(), (0, 2, 0));
}

#[test]
fn test_selector_part_display_universal() {
    let part = SelectorPart::universal();
    assert_eq!(part.to_string(), "*");
}

#[test]
fn test_selector_part_display_element() {
    let part = SelectorPart::element("Button");
    assert_eq!(part.to_string(), "Button");
}

#[test]
fn test_selector_part_display_combined() {
    let part = SelectorPart::element("Button")
        .with_id("submit")
        .with_class("primary");
    assert_eq!(part.to_string(), "Button#submit.primary");
}

// Combinator tests
#[test]
fn test_combinator_display_descendant() {
    assert_eq!(Combinator::Descendant.to_string(), " ");
}

#[test]
fn test_combinator_display_child() {
    assert_eq!(Combinator::Child.to_string(), " > ");
}

#[test]
fn test_combinator_display_adjacent() {
    assert_eq!(Combinator::AdjacentSibling.to_string(), " + ");
}

#[test]
fn test_combinator_display_general() {
    assert_eq!(Combinator::GeneralSibling.to_string(), " ~ ");
}

// Selector tests
#[test]
fn test_selector_empty() {
    let sel = Selector::empty();
    assert!(sel.is_empty());
    assert!(sel.target().is_none());
}

#[test]
fn test_selector_new() {
    let sel = Selector::new(SelectorPart::element("Button"));
    assert!(!sel.is_empty());
    assert_eq!(sel.parts.len(), 1);
}

#[test]
fn test_selector_then() {
    let sel = Selector::new(SelectorPart::class("sidebar"))
        .then(Combinator::Child, SelectorPart::element("Button"));
    assert_eq!(sel.parts.len(), 2);
    assert_eq!(sel.parts[0].1, Some(Combinator::Child));
}

#[test]
fn test_selector_descendant() {
    let sel =
        Selector::new(SelectorPart::class("sidebar")).descendant(SelectorPart::element("Button"));
    assert_eq!(sel.parts[0].1, Some(Combinator::Descendant));
}

#[test]
fn test_selector_child() {
    let sel = Selector::new(SelectorPart::class("sidebar")).child(SelectorPart::element("Button"));
    assert_eq!(sel.parts[0].1, Some(Combinator::Child));
}

#[test]
fn test_selector_target() {
    let sel = Selector::new(SelectorPart::class("sidebar")).child(SelectorPart::element("Button"));
    let target = sel.target().unwrap();
    assert_eq!(target.element, Some("Button".to_string()));
}

// SelectorParseError tests
#[test]
fn test_selector_parse_error_display() {
    let err = SelectorParseError {
        message: "test error".to_string(),
        position: 5,
    };
    let msg = err.to_string();
    assert!(msg.contains("5"));
    assert!(msg.contains("test error"));
}

#[test]
fn test_selector_parse_error_is_error() {
    let err: Box<dyn std::error::Error> = Box::new(SelectorParseError {
        message: "test".to_string(),
        position: 0,
    });
    assert!(err.to_string().contains("test"));
}

// Parser tests
#[test]
fn test_parse_type_selector() {
    let sel = parse_selector("Button").unwrap();
    assert_eq!(sel.parts.len(), 1);
    assert_eq!(sel.parts[0].0.element, Some("Button".to_string()));
}

#[test]
fn test_parse_id_selector() {
    let sel = parse_selector("#submit").unwrap();
    assert_eq!(sel.parts[0].0.id, Some("submit".to_string()));
}

#[test]
fn test_parse_class_selector() {
    let sel = parse_selector(".primary").unwrap();
    assert_eq!(sel.parts[0].0.classes, vec!["primary".to_string()]);
}

#[test]
fn test_parse_universal_selector() {
    let sel = parse_selector("*").unwrap();
    assert!(sel.parts[0].0.universal);
}

#[test]
fn test_parse_combined_selector() {
    let sel = parse_selector("Button#submit.primary.large").unwrap();
    let part = &sel.parts[0].0;
    assert_eq!(part.element, Some("Button".to_string()));
    assert_eq!(part.id, Some("submit".to_string()));
    assert!(part.classes.contains(&"primary".to_string()));
    assert!(part.classes.contains(&"large".to_string()));
}

#[test]
fn test_parse_pseudo_class_focus() {
    let sel = parse_selector("Button:focus").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Focus]);
}

#[test]
fn test_parse_pseudo_class_hover() {
    let sel = parse_selector(":hover").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Hover]);
}

#[test]
fn test_parse_pseudo_class_active() {
    let sel = parse_selector(":active").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Active]);
}

#[test]
fn test_parse_pseudo_class_disabled() {
    let sel = parse_selector(":disabled").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Disabled]);
}

#[test]
fn test_parse_pseudo_class_enabled() {
    let sel = parse_selector(":enabled").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Enabled]);
}

#[test]
fn test_parse_pseudo_class_checked() {
    let sel = parse_selector(":checked").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Checked]);
}

#[test]
fn test_parse_pseudo_class_selected() {
    let sel = parse_selector(":selected").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Selected]);
}

#[test]
fn test_parse_pseudo_class_empty() {
    let sel = parse_selector(":empty").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::Empty]);
}

#[test]
fn test_parse_pseudo_class_first_child() {
    let sel = parse_selector(":first-child").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::FirstChild]);
}

#[test]
fn test_parse_pseudo_class_last_child() {
    let sel = parse_selector(":last-child").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::LastChild]);
}

#[test]
fn test_parse_pseudo_class_only_child() {
    let sel = parse_selector(":only-child").unwrap();
    assert_eq!(sel.parts[0].0.pseudo_classes, vec![PseudoClass::OnlyChild]);
}

#[test]
fn test_parse_nth_child() {
    let sel = parse_selector(":nth-child(3)").unwrap();
    assert_eq!(
        sel.parts[0].0.pseudo_classes,
        vec![PseudoClass::NthChild(3)]
    );
}

#[test]
fn test_parse_nth_last_child() {
    let sel = parse_selector(":nth-last-child(2)").unwrap();
    assert_eq!(
        sel.parts[0].0.pseudo_classes,
        vec![PseudoClass::NthLastChild(2)]
    );
}

#[test]
fn test_parse_not_pseudo() {
    let sel = parse_selector(":not(:disabled)").unwrap();
    match &sel.parts[0].0.pseudo_classes[0] {
        PseudoClass::Not(inner) => {
            assert_eq!(**inner, PseudoClass::Disabled);
        }
        _ => panic!("Expected :not pseudo-class"),
    }
}

#[test]
fn test_parse_descendant() {
    let sel = parse_selector(".sidebar Button").unwrap();
    assert_eq!(sel.parts.len(), 2);
    assert_eq!(sel.parts[0].1, Some(Combinator::Descendant));
}

#[test]
fn test_parse_child() {
    let sel = parse_selector(".sidebar > Button").unwrap();
    assert_eq!(sel.parts.len(), 2);
    assert_eq!(sel.parts[0].1, Some(Combinator::Child));
}

#[test]
fn test_parse_adjacent_sibling() {
    let sel = parse_selector("Label + Input").unwrap();
    assert_eq!(sel.parts.len(), 2);
    assert_eq!(sel.parts[0].1, Some(Combinator::AdjacentSibling));
}

#[test]
fn test_parse_general_sibling() {
    let sel = parse_selector("Label ~ Input").unwrap();
    assert_eq!(sel.parts.len(), 2);
    assert_eq!(sel.parts[0].1, Some(Combinator::GeneralSibling));
}

#[test]
fn test_parse_attribute_exists() {
    let sel = parse_selector("[disabled]").unwrap();
    assert_eq!(sel.parts[0].0.attributes.len(), 1);
    assert_eq!(sel.parts[0].0.attributes[0].name, "disabled");
    assert_eq!(sel.parts[0].0.attributes[0].op, AttributeOp::Exists);
}

#[test]
fn test_parse_attribute_equals() {
    let sel = parse_selector("[type=\"text\"]").unwrap();
    let attr = &sel.parts[0].0.attributes[0];
    assert_eq!(attr.name, "type");
    assert_eq!(attr.op, AttributeOp::Equals);
    assert_eq!(attr.value, Some("text".to_string()));
}

#[test]
fn test_parse_attribute_contains_word() {
    let sel = parse_selector("[class~=\"active\"]").unwrap();
    let attr = &sel.parts[0].0.attributes[0];
    assert_eq!(attr.op, AttributeOp::ContainsWord);
}

#[test]
fn test_parse_attribute_starts_with_word() {
    let sel = parse_selector("[lang|=\"en\"]").unwrap();
    let attr = &sel.parts[0].0.attributes[0];
    assert_eq!(attr.op, AttributeOp::StartsWithWord);
}

#[test]
fn test_parse_attribute_starts_with() {
    let sel = parse_selector("[href^=\"https\"]").unwrap();
    let attr = &sel.parts[0].0.attributes[0];
    assert_eq!(attr.op, AttributeOp::StartsWith);
}

#[test]
fn test_parse_attribute_ends_with() {
    let sel = parse_selector("[href$=\".pdf\"]").unwrap();
    let attr = &sel.parts[0].0.attributes[0];
    assert_eq!(attr.op, AttributeOp::EndsWith);
}

#[test]
fn test_parse_attribute_contains() {
    let sel = parse_selector("[title*=\"hello\"]").unwrap();
    let attr = &sel.parts[0].0.attributes[0];
    assert_eq!(attr.op, AttributeOp::Contains);
}

#[test]
fn test_parse_attribute_case_insensitive() {
    let sel = parse_selector("[type=\"text\" i]").unwrap();
    let attr = &sel.parts[0].0.attributes[0];
    assert!(attr.case_insensitive);
}

#[test]
fn test_specificity_type() {
    let sel = parse_selector("Button").unwrap();
    assert_eq!(sel.specificity(), (0, 0, 1));
}

#[test]
fn test_specificity_class() {
    let sel = parse_selector(".primary").unwrap();
    assert_eq!(sel.specificity(), (0, 1, 0));
}

#[test]
fn test_specificity_id() {
    let sel = parse_selector("#submit").unwrap();
    assert_eq!(sel.specificity(), (1, 0, 0));
}

#[test]
fn test_specificity_combined() {
    let sel = parse_selector("Button#submit.primary").unwrap();
    assert_eq!(sel.specificity(), (1, 1, 1));
}

#[test]
fn test_specificity_multiple_classes() {
    let sel = parse_selector(".btn.primary.large").unwrap();
    assert_eq!(sel.specificity(), (0, 3, 0));
}

#[test]
fn test_specificity_with_pseudo() {
    let sel = parse_selector("Button:focus").unwrap();
    assert_eq!(sel.specificity(), (0, 1, 1));
}

#[test]
fn test_parse_multiple_selectors() {
    let sels = parse_selectors("Button, Input, .primary").unwrap();
    assert_eq!(sels.len(), 3);
}

#[test]
fn test_parse_multiple_selectors_empty() {
    let sels = parse_selectors("").unwrap();
    assert!(sels.is_empty());
}

// Error cases
#[test]
fn test_parse_error_empty_id() {
    let result = parse_selector("#");
    assert!(result.is_err());
}

#[test]
fn test_parse_error_empty_class() {
    let result = parse_selector(".");
    assert!(result.is_err());
}

#[test]
fn test_parse_error_unknown_pseudo() {
    let result = parse_selector(":unknown-pseudo");
    assert!(result.is_err());
}

#[test]
fn test_parse_error_invalid_attribute_op() {
    let result = parse_selector("[attr%=value]");
    assert!(result.is_err());
}

#[test]
fn test_selector_display() {
    let sel = parse_selector(".sidebar > Button").unwrap();
    let display = sel.to_string();
    assert!(display.contains("sidebar"));
    assert!(display.contains("Button"));
}

}
