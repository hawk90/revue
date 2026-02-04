//! Specificity tests

use crate::dom::cascade::specificity::Specificity;

#[test]
fn test_specificity_ordering() {
    // Type < Class < ID
    let type_spec = Specificity::new(0, 0, 1, 0);
    let class_spec = Specificity::new(0, 1, 0, 0);
    let id_spec = Specificity::new(1, 0, 0, 0);

    assert!(type_spec < class_spec);
    assert!(class_spec < id_spec);
}

#[test]
fn test_specificity_same_level() {
    // More of same type wins
    let one_class = Specificity::new(0, 1, 0, 0);
    let two_classes = Specificity::new(0, 2, 0, 0);

    assert!(one_class < two_classes);
}

#[test]
fn test_specificity_order_tiebreak() {
    // Later declaration wins
    let first = Specificity::new(0, 1, 0, 0);
    let second = Specificity::new(0, 1, 0, 1);

    assert!(first < second);
}

#[test]
fn test_specificity_inline() {
    let inline = Specificity::inline();
    let id = Specificity::new(1, 0, 0, 0);

    assert!(id < inline);
}

#[test]
fn test_specificity_important() {
    let normal_id = Specificity::new(1, 0, 0, 0);
    let important_class = Specificity::new(0, 1, 0, 0).important();

    assert!(normal_id < important_class);
}

#[test]
fn test_specificity_default() {
    let spec = Specificity::default();
    assert_eq!(spec.ids, 0);
    assert_eq!(spec.classes, 0);
    assert_eq!(spec.types, 0);
    assert!(!spec.inline);
    assert!(!spec.important);
}

#[test]
fn test_specificity_partial_ord() {
    let a = Specificity::new(1, 0, 0, 0);
    let b = Specificity::new(0, 1, 0, 0);
    assert!(a.partial_cmp(&b) == Some(std::cmp::Ordering::Greater));
}

#[test]
fn test_specificity_debug() {
    let spec = Specificity::new(1, 2, 3, 4);
    let debug = format!("{:?}", spec);
    assert!(debug.contains("Specificity"));
}

#[test]
fn test_specificity_clone() {
    let spec = Specificity::new(1, 2, 3, 4);
    let cloned = spec;
    assert_eq!(spec.ids, cloned.ids);
    assert_eq!(spec.classes, cloned.classes);
    assert_eq!(spec.types, cloned.types);
}

#[test]
fn test_specificity_eq() {
    let a = Specificity::new(1, 2, 3, 0);
    let b = Specificity::new(1, 2, 3, 0);
    assert_eq!(a, b);
}

#[test]
fn test_specificity_important_over_inline() {
    let inline = Specificity::inline();
    let important = Specificity::new(0, 0, 0, 0).important();
    assert!(inline < important);
}

#[test]
fn test_specificity_both_inline() {
    let inline1 = Specificity {
        inline: true,
        important: false,
        ids: 0,
        classes: 0,
        types: 0,
        order: 0,
    };
    let inline2 = Specificity {
        inline: true,
        important: false,
        ids: 0,
        classes: 0,
        types: 0,
        order: 1,
    };
    assert!(inline1 < inline2); // Order matters
}

#[test]
fn test_specificity_both_important() {
    let important1 = Specificity::new(0, 1, 0, 0).important();
    let important2 = Specificity::new(1, 0, 0, 0).important();
    assert!(important1 < important2); // IDs still count
}

#[test]
fn test_specificity_types_tiebreak() {
    let a = Specificity::new(0, 0, 1, 0);
    let b = Specificity::new(0, 0, 2, 0);
    assert!(a < b);
}
