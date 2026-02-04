//! StyleMerge tests

use crate::dom::cascade::merge::StyleMerge;
use crate::style::Style;

#[test]
fn test_style_merge_default() {
    let style1 = Style::default();
    let style2 = Style::default();
    let merged = style1.merge(&style2);
    assert!(merged.visual.visible);
}

#[test]
fn test_style_merge_color() {
    use crate::style::Color;

    let mut style1 = Style::default();
    let mut style2 = Style::default();
    style2.visual.color = Color::hex(0xff0000);

    let merged = style1.merge(&style2);
    assert_eq!(merged.visual.color, Color::hex(0xff0000));

    // First style color is preserved if second is default
    style1.visual.color = Color::hex(0x00ff00);
    let style3 = Style::default();
    let merged2 = style1.merge(&style3);
    assert_eq!(merged2.visual.color, Color::hex(0x00ff00));
}

#[test]
fn test_style_merge_background() {
    use crate::style::Color;

    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.visual.background = Color::hex(0x0000ff);

    let merged = style1.merge(&style2);
    assert_eq!(merged.visual.background, Color::hex(0x0000ff));
}

#[test]
fn test_style_merge_opacity() {
    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.visual.opacity = 0.5;

    let merged = style1.merge(&style2);
    assert!((merged.visual.opacity - 0.5).abs() < 0.001);
}

#[test]
fn test_style_merge_visible() {
    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.visual.visible = false;

    let merged = style1.merge(&style2);
    assert!(!merged.visual.visible);
}

#[test]
fn test_style_merge_display() {
    use crate::style::Display;

    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.layout.display = Display::Flex;

    let merged = style1.merge(&style2);
    assert_eq!(merged.layout.display, Display::Flex);
}

#[test]
fn test_style_merge_gap() {
    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.layout.gap = 10;

    let merged = style1.merge(&style2);
    assert_eq!(merged.layout.gap, 10);
}

#[test]
fn test_style_merge_sizing() {
    use crate::style::Size;

    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.sizing.width = Size::Fixed(100);
    style2.sizing.height = Size::Percent(50.0);

    let merged = style1.merge(&style2);
    assert_eq!(merged.sizing.width, Size::Fixed(100));
    assert_eq!(merged.sizing.height, Size::Percent(50.0));
}

#[test]
fn test_style_merge_min_max_sizing() {
    use crate::style::Size;

    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.sizing.min_width = Size::Fixed(50);
    style2.sizing.max_width = Size::Fixed(200);
    style2.sizing.min_height = Size::Fixed(30);
    style2.sizing.max_height = Size::Fixed(100);

    let merged = style1.merge(&style2);
    assert_eq!(merged.sizing.min_width, Size::Fixed(50));
    assert_eq!(merged.sizing.max_width, Size::Fixed(200));
    assert_eq!(merged.sizing.min_height, Size::Fixed(30));
    assert_eq!(merged.sizing.max_height, Size::Fixed(100));
}

#[test]
fn test_style_merge_flex() {
    use crate::style::{AlignItems, FlexDirection, JustifyContent};

    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.layout.flex_direction = FlexDirection::Column;
    style2.layout.justify_content = JustifyContent::Center;
    style2.layout.align_items = AlignItems::End;

    let merged = style1.merge(&style2);
    assert_eq!(merged.layout.flex_direction, FlexDirection::Column);
    assert_eq!(merged.layout.justify_content, JustifyContent::Center);
    assert_eq!(merged.layout.align_items, AlignItems::End);
}

#[test]
fn test_style_merge_spacing() {
    use crate::style::Spacing;

    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.spacing.margin = Spacing::all(10);
    style2.spacing.padding = Spacing::all(5);

    let merged = style1.merge(&style2);
    assert_eq!(merged.spacing.margin, Spacing::all(10));
    assert_eq!(merged.spacing.padding, Spacing::all(5));
}

#[test]
fn test_style_merge_border() {
    use crate::style::{BorderStyle, Color};

    let style1 = Style::default();
    let mut style2 = Style::default();
    style2.visual.border_style = BorderStyle::Solid;
    style2.visual.border_color = Color::hex(0xffffff);

    let merged = style1.merge(&style2);
    assert_eq!(merged.visual.border_style, BorderStyle::Solid);
    assert_eq!(merged.visual.border_color, Color::hex(0xffffff));
}
