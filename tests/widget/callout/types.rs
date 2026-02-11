//! Type definitions tests

use revue::widget::callout::types::{CalloutType, CalloutVariant};

// =========================================================================
// CalloutType tests
// =========================================================================

#[test]
fn test_callout_type_default() {
    assert_eq!(CalloutType::default(), CalloutType::Note);
}

#[test]
fn test_callout_type_partial_eq() {
    assert_eq!(CalloutType::Note, CalloutType::Note);
    assert_eq!(CalloutType::Tip, CalloutType::Tip);
    assert_ne!(CalloutType::Note, CalloutType::Tip);
}

#[test]
fn test_callout_type_copy() {
    let ct = CalloutType::Warning;
    let copied = ct;
    assert_eq!(ct, copied);
}

#[test]
fn test_callout_type_clone() {
    let ct = CalloutType::Danger;
    let cloned = ct.clone();
    assert_eq!(ct, cloned);
}

#[test]
fn test_callout_type_all_variants_unique() {
    assert_ne!(CalloutType::Note, CalloutType::Tip);
    assert_ne!(CalloutType::Note, CalloutType::Important);
    assert_ne!(CalloutType::Note, CalloutType::Warning);
    assert_ne!(CalloutType::Note, CalloutType::Danger);
    assert_ne!(CalloutType::Note, CalloutType::Info);
    assert_ne!(CalloutType::Tip, CalloutType::Important);
    assert_ne!(CalloutType::Tip, CalloutType::Warning);
    assert_ne!(CalloutType::Tip, CalloutType::Danger);
    assert_ne!(CalloutType::Tip, CalloutType::Info);
    assert_ne!(CalloutType::Important, CalloutType::Warning);
    assert_ne!(CalloutType::Important, CalloutType::Danger);
    assert_ne!(CalloutType::Important, CalloutType::Info);
    assert_ne!(CalloutType::Warning, CalloutType::Danger);
    assert_ne!(CalloutType::Warning, CalloutType::Info);
    assert_ne!(CalloutType::Danger, CalloutType::Info);
}

// =========================================================================
// CalloutVariant tests
// =========================================================================

#[test]
fn test_callout_variant_default() {
    assert_eq!(CalloutVariant::default(), CalloutVariant::Filled);
}

#[test]
fn test_callout_variant_partial_eq() {
    assert_eq!(CalloutVariant::Filled, CalloutVariant::Filled);
    assert_eq!(CalloutVariant::LeftBorder, CalloutVariant::LeftBorder);
    assert_ne!(CalloutVariant::Filled, CalloutVariant::Minimal);
}

#[test]
fn test_callout_variant_copy() {
    let cv = CalloutVariant::LeftBorder;
    let copied = cv;
    assert_eq!(cv, copied);
}

#[test]
fn test_callout_variant_clone() {
    let cv = CalloutVariant::Minimal;
    let cloned = cv.clone();
    assert_eq!(cv, cloned);
}

#[test]
fn test_callout_variant_all_variants_unique() {
    assert_ne!(CalloutVariant::Filled, CalloutVariant::LeftBorder);
    assert_ne!(CalloutVariant::Filled, CalloutVariant::Minimal);
    assert_ne!(CalloutVariant::LeftBorder, CalloutVariant::Minimal);
}