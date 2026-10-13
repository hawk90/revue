//! BoxGroup tests

use super::even_stats;
use revue::style::Color;
use revue::widget::data::chart::{BoxGroup, WhiskerStyle};

#[test]
fn test_box_group_new() {
    let group = BoxGroup::new("test", &[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(group.label, "test");
    assert_eq!(group.data, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    assert!(group.stats.is_none());
    assert!(group.color.is_none());
}

#[test]
fn test_box_group_empty_data() {
    let group = BoxGroup::new("test", &[]);
    assert!(group.data.is_empty());
    assert!(group.get_stats(WhiskerStyle::IQR).is_none());
}

#[test]
fn test_box_group_from_stats() {
    let group = BoxGroup::from_stats("test", even_stats());
    assert_eq!(group.label, "test");
    assert!(group.data.is_empty());
    assert_eq!(group.stats.as_ref().unwrap().median, 50.0);
    assert!(group.color.is_none());
}

#[test]
fn test_box_group_color() {
    let group = BoxGroup::new("test", &[1.0, 2.0]).color(Color::RED);
    assert_eq!(group.color, Some(Color::RED));
}

#[test]
fn test_box_group_get_stats_prefers_precomputed() {
    let group = BoxGroup::from_stats("test", even_stats());
    // Precomputed stats win regardless of whisker style
    let stats = group.get_stats(WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.min, 0.0);
    assert_eq!(stats.q1, 25.0);
    assert_eq!(stats.median, 50.0);
    assert_eq!(stats.q3, 75.0);
    assert_eq!(stats.max, 100.0);
}

#[test]
fn test_box_group_get_stats_computes_from_data() {
    let group = BoxGroup::new("test", &[1.0, 2.0, 3.0, 4.0, 5.0]);
    let stats = group.get_stats(WhiskerStyle::MinMax).unwrap();
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 5.0);
    assert_eq!(stats.median, 3.0);
}
