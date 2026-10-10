//! Computed style tests

use revue::style::{Color, ComputedStyle, Style};

#[test]
fn test_computed_style_new() {
    let computed = ComputedStyle::new();
    let _ = computed.style.color();
}

#[test]
fn test_computed_style_default() {
    let computed = ComputedStyle::default();
    let _ = computed.style.color();
}

#[test]
fn test_computed_style_compute_no_parent() {
    let style = Style::default();
    let computed = ComputedStyle::compute(style, None);
    let _ = computed.style.color();
}

#[test]
fn test_computed_style_compute_with_parent() {
    let mut parent_style = Style::default();
    parent_style.visual.color = Color::RED;
    let parent = ComputedStyle {
        style: parent_style,
    };

    let child_style = Style::default();
    let computed = ComputedStyle::compute(child_style, Some(&parent));

    assert_eq!(computed.style.color(), Color::RED);
}

#[test]
fn test_computed_style_compute_child_overrides_parent() {
    let mut parent_style = Style::default();
    parent_style.visual.color = Color::RED;
    let parent = ComputedStyle {
        style: parent_style,
    };

    let mut child_style = Style::default();
    child_style.visual.color = Color::BLUE;
    let computed = ComputedStyle::compute(child_style, Some(&parent));

    assert_eq!(computed.style.color(), Color::BLUE);
}

#[test]
fn test_computed_style_clone() {
    let computed = ComputedStyle::new();
    let cloned = computed.clone();
    let _ = cloned.style.color();
}

#[test]
fn test_computed_style_debug() {
    let computed = ComputedStyle::new();
    let debug = format!("{:?}", computed);
    assert!(debug.contains("ComputedStyle"));
}

#[test]
fn test_computed_style_inheritance_chain() {
    let mut grandparent_style = Style::default();
    grandparent_style.visual.color = Color::RED;
    let grandparent = ComputedStyle {
        style: grandparent_style,
    };

    let parent_style = Style::default();
    let parent = ComputedStyle::compute(parent_style, Some(&grandparent));

    let child_style = Style::default();
    let child = ComputedStyle::compute(child_style, Some(&parent));

    assert_eq!(child.style.color(), Color::RED);
}
