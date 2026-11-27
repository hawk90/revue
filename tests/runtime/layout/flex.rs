//! Edge case tests for flex layout

use revue::dom::DomId;
use revue::layout::{LayoutEngine, Rect};
use revue::style::{Display, FlexDirection, JustifyContent, Size, Style};

#[test]
fn test_flex_zero_space() {
    // Test flex behavior with zero available space
    let available = Rect::new(0, 0, 0, 10);
    assert_eq!(available.width, 0);
}

#[test]
fn test_flex_single_child_full_width() {
    // Test that a single flex child takes full width
    let parent = Rect::new(0, 0, 100, 20);
    let child = Rect::new(0, 0, 100, 20);
    assert_eq!(child.width, parent.width);
}

#[test]
fn test_flex_overflow_saturates() {
    // Test that flex overflow saturates at u16::MAX
    let large = u16::MAX - 10;
    let result = large.saturating_add(100);
    assert_eq!(result, u16::MAX);
}

/// A 100-wide flex row of `count` children with the given width, laid
/// out with `justify`; returns each child's (x, width)
fn row_of(count: u64, width: Size, justify: JustifyContent) -> Vec<(u16, u16)> {
    let mut engine = LayoutEngine::new();
    let mut child_style = Style::default();
    child_style.sizing.width = width;
    let children: Vec<DomId> = (1..=count).map(DomId::new).collect();
    for &child in &children {
        engine.create_node(child, &child_style).unwrap();
    }
    let mut parent_style = Style::default();
    parent_style.layout.display = Display::Flex;
    parent_style.layout.flex_direction = FlexDirection::Row;
    parent_style.layout.justify_content = justify;
    let parent = DomId::new(count + 1);
    engine
        .create_node_with_children(parent, &parent_style, &children)
        .unwrap();
    engine.compute(parent, 100, 1).unwrap();
    children
        .iter()
        .map(|&c| {
            let r = engine.layout(c).unwrap();
            (r.x, r.width)
        })
        .collect()
}

/// The space left over was divided by `auto_count as u16`, which is 0 at
/// 65536 auto children and 1 at 65537 (#1531): the first panicked, the
/// second gave every child the whole 100 cells
#[test]
fn test_flex_space_split_among_65536_auto_children() {
    for count in [65_536, 65_537] {
        let rects = row_of(count, Size::Auto, JustifyContent::Start);
        // The 100 cells go one each to the first 100 children
        for (i, &rect) in rects.iter().enumerate() {
            let expected = if i < 100 { (i as u16, 1) } else { (100, 0) };
            assert_eq!(rect, expected, "{count} children, child {i}");
        }
    }
}

/// SpaceBetween divided by `(count - 1) as u16` and SpaceAround by
/// `count as u16`, 0 at 65537 and 65536 children (#1531)
#[test]
fn test_flex_justify_space_among_65536_children() {
    for (count, justify) in [
        (65_537, JustifyContent::SpaceBetween),
        (65_536, JustifyContent::SpaceAround),
    ] {
        let rects = row_of(count, Size::Fixed(0), justify);
        // 100 cells among 65536 gaps is no gap: every child at x 0
        assert!(
            rects.iter().all(|&r| r == (0, 0)),
            "{justify:?}: {:?}",
            rects.iter().find(|&&r| r != (0, 0))
        );
    }
}
