//! HeatMap constructor tests: new, from_flat, the preset maps and the constructor functions

use super::super::{render, render_rows, rows};
use super::{blank, fg};
use revue::widget::data::chart::{contribution_map, heatmap, ColorScale, HeatMap};

#[test]
fn test_heatmap_new() {
    let hm = HeatMap::new(vec![vec![0.0, 0.5, 1.0], vec![0.2, 0.4, 0.8]]);
    // Bounds come from the data
    assert_eq!(hm.normalize(0.0), 0.0);
    assert_eq!(hm.normalize(1.0), 1.0);

    // Default cells: two-column blocks colored by value
    let buffer = render(&hm, 6, 2);
    let rows = rows(&buffer);
    assert_eq!(rows[0].trim_end(), "██████");
    assert_eq!(rows[1].trim_end(), "██████");
    assert_eq!(fg(&buffer, 0, 0), Some(hm.color_for(0.0)));
    assert_eq!(fg(&buffer, 4, 0), Some(hm.color_for(1.0)));
    assert_eq!(fg(&buffer, 2, 1), Some(hm.color_for(0.4)));
}

#[test]
fn test_heatmap_helper_matches_new() {
    let data = vec![vec![0.5, 0.1]];
    assert_eq!(
        render_rows(&heatmap(data.clone()), 10, 2),
        render_rows(&HeatMap::new(data), 10, 2)
    );
}

#[test]
fn test_heatmap_from_flat() {
    let hm = HeatMap::from_flat(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
    assert_eq!(hm.normalize(1.0), 0.0);
    assert_eq!(hm.normalize(6.0), 1.0);
    let buffer = render(&hm, 6, 2);
    assert_eq!(rows(&buffer)[0].trim_end(), "██████");
    assert_eq!(rows(&buffer)[1].trim_end(), "██████");
    // (row 0, col 0) holds 1.0 and (row 1, col 2) holds 6.0
    assert_eq!(fg(&buffer, 0, 0), Some(hm.color_for(1.0)));
    assert_eq!(fg(&buffer, 4, 1), Some(hm.color_for(6.0)));
}

#[test]
fn test_from_flat_partial_data() {
    // Data shorter than rows * cols: the second row is empty
    let hm = HeatMap::from_flat(&[1.0, 2.0, 3.0], 2, 3);
    let rows = render_rows(&hm, 6, 2);
    assert_eq!(rows[0].trim_end(), "██████");
    assert!(rows[1].trim().is_empty());
}

#[test]
fn test_from_flat_data_shorter_than_a_row() {
    // Rows past the data are empty instead of panicking the slice (#993)
    let hm = HeatMap::from_flat(&[1.0], 3, 2);
    let rows = render_rows(&hm, 4, 3);
    assert_eq!(rows[0].trim_end(), "██");
    assert!(rows[1].trim().is_empty());
    assert!(rows[2].trim().is_empty());
    assert!(blank(&render(&HeatMap::from_flat(&[], 2, 2), 4, 2)));
}

#[test]
fn test_heatmap_empty_data() {
    let hm = HeatMap::new(vec![]);
    // Default bounds 0..1
    assert_eq!(hm.normalize(0.0), 0.0);
    assert_eq!(hm.normalize(1.0), 1.0);
    assert!(blank(&render(&hm, 10, 3)));
}

#[test]
fn test_heatmap_empty_rows() {
    let hm = HeatMap::new(vec![vec![], vec![]]);
    assert!(blank(&render(&hm, 10, 3)));
}

#[test]
fn test_heatmap_clone_and_debug() {
    let hm = HeatMap::new(vec![vec![0.5, 1.0]])
        .title("Test")
        .color_scale(ColorScale::Viridis);
    let cloned = hm.clone();
    assert_eq!(render_rows(&cloned, 10, 3), render_rows(&hm, 10, 3));
    assert!(format!("{:?}", hm).contains("HeatMap"));
}

#[test]
fn test_correlation_matrix() {
    let data = vec![
        vec![1.0, 0.5, -0.5],
        vec![0.5, 1.0, 0.3],
        vec![-0.5, 0.3, 1.0],
    ];
    let labels = vec!["A".into(), "B".into(), "C".into()];
    let hm = HeatMap::correlation_matrix(&data, labels);
    // Fixed -1..1 bounds on the BlueRed scale
    assert_eq!(hm.normalize(-1.0), 0.0);
    assert_eq!(hm.normalize(0.0), 0.5);
    assert_eq!(hm.color_for(1.0), ColorScale::BlueRed.color_at(1.0));
    // Labels and values are shown
    let rows = render_rows(&hm, 40, 4);
    for label in ["A", "B", "C"] {
        assert!(rows[0].contains(label), "{rows:?}");
    }
    assert!(rows[1].starts_with("     A "), "{rows:?}");
    assert!(
        rows[1].contains("1.0") && rows[1].contains("0.5"),
        "{rows:?}"
    );
    assert!(rows[1].contains("-0.5"), "{rows:?}");
    assert!(rows[3].starts_with("     C "), "{rows:?}");
}

#[test]
fn test_contribution_map() {
    let contributions: Vec<u32> = (0..364).map(|i| i % 10).collect();
    for hm in [
        HeatMap::contribution_map(&contributions),
        contribution_map(&contributions),
    ] {
        // 7 days x 52 weeks of 2-column cells on the GitHub green scale
        let buffer = render(&hm, 104, 7);
        for row in rows(&buffer) {
            assert_eq!(row.chars().filter(|&c| c == '█').count(), 104);
        }
        // Day 0 of week 0 has no contributions, day 1 of week 0 has 1 of 9
        assert_eq!(fg(&buffer, 0, 0), Some(ColorScale::Green.color_at(0.0)));
        assert_eq!(
            fg(&buffer, 0, 1),
            Some(ColorScale::Green.color_at(1.0 / 9.0))
        );
    }
}
