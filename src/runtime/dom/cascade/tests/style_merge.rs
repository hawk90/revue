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
    style2.layout.gap = Some(10);

    let merged = style1.merge(&style2);
    assert_eq!(merged.layout.gap, Some(10));
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
    style2.visual.border_style = Some(BorderStyle::Solid);
    style2.visual.border_color = Color::hex(0xffffff);

    let merged = style1.merge(&style2);
    assert_eq!(merged.visual.border_style, Some(BorderStyle::Solid));
    assert_eq!(merged.visual.border_color, Color::hex(0xffffff));
}

/// A style with every field away from its default. Struct literals without
/// `..`, so a new field does not compile until it is listed here.
fn every_field_set() -> Style {
    use crate::style::{
        AlignItems, AlignSelf, BorderStyle, Color, Display, FlexDirection, FlexWrap, FontWeight,
        GridPlacement, GridTemplate, JustifyContent, LayoutStyle, Overflow, Position, Size,
        SizingStyle, Spacing, SpacingStyle, TextAlign, TextDecoration, VisualStyle,
    };
    Style {
        layout: LayoutStyle {
            display: Display::Grid,
            position: Position::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            flex_grow: 2.0,
            flex_wrap: FlexWrap::Wrap,
            align_self: AlignSelf::Center,
            order: 3,
            gap: Some(1),
            column_gap: Some(2),
            row_gap: Some(3),
            grid_template_columns: GridTemplate::fixed(&[4, 5]),
            grid_template_rows: GridTemplate::fr(&[1.0]),
            grid_column: GridPlacement::span(2),
            grid_row: GridPlacement::line(2),
        },
        spacing: SpacingStyle {
            padding: Spacing::all(1),
            margin: Spacing::vertical(2),
            top: Some(1),
            right: Some(2),
            bottom: Some(3),
            left: Some(4),
        },
        sizing: SizingStyle {
            width: Size::Fixed(10),
            height: Size::Percent(50.0),
            min_width: Size::Fixed(1),
            max_width: Size::Fixed(20),
            min_height: Size::Fixed(2),
            max_height: Size::Fixed(30),
        },
        visual: VisualStyle {
            border_style: Some(BorderStyle::Rounded),
            border_color: Color::hex(0x112233),
            color: Color::hex(0x445566),
            background: Color::hex(0x778899),
            opacity: 0.5,
            visible: false,
            z_index: 7,
            text_align: TextAlign::Right,
            font_weight: FontWeight::Bold,
            text_decoration: TextDecoration {
                underline: true,
                line_through: false,
            },
            overflow: Overflow::Hidden,
        },
    }
}

/// `merge` used to copy 19 of the 39 fields, so an inline `position`,
/// `z_index`, `text_align`, `overflow`, ... was dropped.
#[test]
fn every_field_that_is_set_overrides() {
    let set = every_field_set();
    assert_eq!(Style::default().merge(&set), set);
}

#[test]
fn a_field_left_at_its_default_does_not_override() {
    let set = every_field_set();
    assert_eq!(set.merge(&Style::default()), set);
}
