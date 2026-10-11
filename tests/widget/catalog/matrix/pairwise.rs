//! Layer 1c - pairwise interactions.
//!
//! An all-pairs case set over {size, content, focus, path}: every pair of
//! factor values appears in at least one case, which takes a few dozen cases
//! per widget instead of the full product. The only layer that also drives
//! the real app pipeline (`PipelineHarness`, 3.0 defaults).
//!
//! The set is built by a small deterministic greedy search (no dependency):
//! repeatedly take the full combination that covers the most still-uncovered
//! pairs. The app path cannot place an area at a far offset, so that pair is
//! excluded up front.

use super::{run_layer, Case, Content, Path, Placement};

fn placements() -> Vec<Placement> {
    vec![
        Placement::sized("0x0", 0, 0),
        Placement::sized("1x1", 1, 1),
        Placement::sized("2x1", 2, 1),
        Placement::sized("1x2", 1, 2),
        Placement::sized("3x3", 3, 3),
        Placement::sized("80x24", 80, 24),
        Placement::far_x(),
    ]
}

fn contents() -> Vec<Content> {
    vec![
        Content::text("empty", ""),
        Content::text("hangul", "한글 텍스트"),
        Content::text("emoji", "👍🏽 emoji 👨‍👩‍👧"),
        Content::text("long", "abc ".repeat(250)),
    ]
}

const FOCUS: [bool; 2] = [false, true];
const PATHS: [Path; 2] = [Path::Direct, Path::App];

/// Index of the far-offset placement, which the app path cannot use.
const FAR: usize = 6;

/// Greedy all-pairs over factor sizes `dims`; returns index tuples.
/// `allowed` rejects impossible combinations.
pub fn all_pairs(dims: &[usize], allowed: impl Fn(&[usize]) -> bool) -> Vec<Vec<usize>> {
    // Every full combination, in a fixed order.
    let mut combos: Vec<Vec<usize>> = vec![vec![]];
    for &n in dims {
        combos = combos
            .into_iter()
            .flat_map(|c| {
                (0..n).map(move |v| {
                    let mut c = c.clone();
                    c.push(v);
                    c
                })
            })
            .collect();
    }
    combos.retain(|c| allowed(c));

    let pairs_of = |c: &[usize]| {
        let mut out = Vec::new();
        for i in 0..c.len() {
            for j in i + 1..c.len() {
                out.push((i, c[i], j, c[j]));
            }
        }
        out
    };
    // Only pairs some allowed combination can reach need covering.
    let mut uncovered: std::collections::BTreeSet<_> =
        combos.iter().flat_map(|c| pairs_of(c)).collect();

    let mut chosen = Vec::new();
    while !uncovered.is_empty() {
        // The first combination covering the most uncovered pairs.
        let mut best = &combos[0];
        let mut best_gain = 0;
        for c in &combos {
            let gain = pairs_of(c).iter().filter(|p| uncovered.contains(p)).count();
            if gain > best_gain {
                best = c;
                best_gain = gain;
            }
        }
        let best = best.clone();
        for p in pairs_of(&best) {
            uncovered.remove(&p);
        }
        chosen.push(best);
    }
    chosen
}

#[test]
fn the_pairwise_set_covers_every_pair() {
    let set = all_pairs(&[7, 4, 2, 2], |c| !(c[0] == FAR && c[3] == 1));
    for (i, a) in [7, 4, 2, 2].iter().enumerate() {
        for (j, b) in [7, 4, 2, 2].iter().enumerate().skip(i + 1) {
            for x in 0..*a {
                for y in 0..*b {
                    if (i, x, j, y) == (0, FAR, 3, 1) {
                        continue;
                    }
                    assert!(
                        set.iter().any(|c| c[i] == x && c[j] == y),
                        "pair ({i}={x}, {j}={y}) not covered"
                    );
                }
            }
        }
    }
    assert!(set.len() <= 40, "{} cases", set.len());
}

#[test]
fn every_widget_survives_pairwise_combinations() {
    let placements = placements();
    let contents = contents();
    let set = all_pairs(
        &[placements.len(), contents.len(), FOCUS.len(), PATHS.len()],
        |c| !(c[0] == FAR && PATHS[c[3]] == Path::App),
    );
    let cases: Vec<Case> = set
        .iter()
        .map(|c| Case {
            placement: placements[c[0]],
            content: &contents[c[1]],
            focused: FOCUS[c[2]],
            path: PATHS[c[3]],
        })
        .collect();
    run_layer("pairwise", &cases);
}
