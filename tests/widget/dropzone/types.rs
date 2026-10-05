//! Drop zone type tests

use revue::widget::DropZoneStyle;

#[test]
fn test_drop_zone_style_default() {
    assert_eq!(DropZoneStyle::default(), DropZoneStyle::Solid);
}

#[test]
fn test_drop_zone_style_all_variants_distinct() {
    let all = [
        DropZoneStyle::Solid,
        DropZoneStyle::Dashed,
        DropZoneStyle::Highlight,
        DropZoneStyle::Minimal,
    ];
    for (i, a) in all.iter().enumerate() {
        for (j, b) in all.iter().enumerate() {
            assert_eq!(i == j, a == b, "{:?} vs {:?}", a, b);
        }
    }
}

#[test]
fn test_drop_zone_style_copy() {
    let style = DropZoneStyle::Highlight;
    let copied = style;
    assert_eq!(style, copied);
}

#[test]
fn test_drop_zone_style_debug() {
    assert_eq!(format!("{:?}", DropZoneStyle::Dashed), "Dashed");
}
