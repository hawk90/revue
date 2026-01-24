//! MatchedRule tests

use crate::dom::cascade::resolver::MatchedRule;
use crate::dom::cascade::specificity::Specificity;
use crate::style::Rule;

#[test]
fn test_matched_rule_debug() {
    let rule = Rule {
        selector: "Button".to_string(),
        declarations: vec![],
    };
    let selector = crate::dom::parse_selector("Button").unwrap();
    let matched = MatchedRule {
        selector: &selector,
        rule: &rule,
        specificity: Specificity::new(0, 0, 1, 0),
    };

    // Should be debuggable
    let debug_str = format!("{:?}", matched);
    assert!(debug_str.contains("MatchedRule"));
}

#[test]
fn test_matched_rule_clone() {
    let rule = Rule {
        selector: "Button".to_string(),
        declarations: vec![],
    };
    let selector = crate::dom::parse_selector("Button").unwrap();
    let matched = MatchedRule {
        selector: &selector,
        rule: &rule,
        specificity: Specificity::new(0, 0, 1, 0),
    };

    let cloned = matched.clone();
    assert_eq!(cloned.specificity.types, 1);
}
