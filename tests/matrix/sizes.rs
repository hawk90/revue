//! Layer 1a - sizes only.
//!
//! Every widget with one ordinary content, unfocused, on the direct path,
//! over boundary areas. Targets size arithmetic: underflow on tiny areas,
//! overflow on huge ones and on areas pressed against `u16::MAX`.

use super::{run_layer, Case, Content, Path, Placement};

pub fn placements() -> Vec<Placement> {
    vec![
        Placement::sized("0x0", 0, 0),
        Placement::sized("1x1", 1, 1),
        Placement::sized("2x1", 2, 1),
        Placement::sized("1x2", 1, 2),
        Placement::sized("3x3", 3, 3),
        Placement::sized("80x24", 80, 24),
        Placement::sized("1000x1", 1000, 1),
        Placement::sized("1x1000", 1, 1000),
        Placement::far_x(),
        Placement::far_y(),
    ]
}

#[test]
fn every_widget_survives_boundary_sizes() {
    let content = Content::ordinary();
    let cases: Vec<Case> = placements()
        .into_iter()
        .map(|placement| Case {
            placement,
            content: &content,
            focused: false,
            path: Path::Direct,
        })
        .collect();
    run_layer("sizes", &cases);
}
