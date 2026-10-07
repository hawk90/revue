//! Level resolution and reordering for a single line of text
//!
//! A simplified Unicode Bidirectional Algorithm (UAX #9) over the classes
//! from [`BidiClass::of`]:
//!
//! - weak types: W1–W7 (NSM, European/Arabic numbers and their separators
//!   and terminators)
//! - neutrals: N1–N2
//! - implicit levels: I1–I2
//! - line levels: L1 (trailing whitespace and separators take the paragraph
//!   level), L2 (reordering), L4 (mirroring, done by the caller)
//!
//! Limits: the text is treated as one paragraph and one line; explicit
//! embeddings, overrides and isolates (LRE, RLO, LRI, PDI, ...) are ignored
//! like boundary neutrals rather than applied; bracket pairs (N0) are not
//! matched; character classes come from [`BidiClass::of`], which is a
//! simplified table.

use super::types::{BidiClass, ResolvedDirection};

/// Resolve the embedding level of each character
pub(super) fn resolve_levels(chars: &[char], base: ResolvedDirection) -> Vec<u8> {
    let base_level: u8 = if base.is_rtl() { 1 } else { 0 };
    let sor = if base.is_rtl() {
        BidiClass::R
    } else {
        BidiClass::L
    };

    let original: Vec<BidiClass> = chars.iter().map(|&c| BidiClass::of(c)).collect();
    // X9: explicit formatting characters are not applied; treat them as BN
    let mut types: Vec<BidiClass> = original
        .iter()
        .map(|&t| {
            if is_removed_by_x9(t) {
                BidiClass::BN
            } else {
                t
            }
        })
        .collect();

    // Indices of the characters that take part in W1–N2 (X9 removes BN)
    let idx: Vec<usize> = (0..types.len())
        .filter(|&i| types[i] != BidiClass::BN)
        .collect();

    resolve_weak(&mut types, &idx, sor);
    resolve_neutrals(&mut types, &idx, sor);

    // I1–I2
    let mut levels = vec![base_level; types.len()];
    for &i in &idx {
        levels[i] = implicit_level(base_level, types[i]);
    }

    // BN takes the level of the preceding character (or the paragraph level)
    let mut prev = base_level;
    for i in 0..levels.len() {
        if types[i] == BidiClass::BN {
            levels[i] = prev;
        }
        prev = levels[i];
    }

    // L1: separators, and whitespace before them or at the end of the line,
    // take the paragraph level
    let mut trailing = true;
    for i in (0..levels.len()).rev() {
        match original[i] {
            BidiClass::B | BidiClass::S => {
                levels[i] = base_level;
                trailing = true;
            }
            t if trailing && (t == BidiClass::WS || is_removed_by_x9(t)) => {
                levels[i] = base_level;
            }
            _ => trailing = false,
        }
    }

    levels
}

/// Visual order of runs (L2): from the highest level down to the lowest odd
/// level, reverse every sequence of runs at that level or higher
pub(super) fn reorder(run_levels: &[u8]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..run_levels.len()).collect();
    let Some(&max) = run_levels.iter().max() else {
        return order;
    };
    let Some(lowest_odd) = run_levels.iter().copied().filter(|l| l % 2 == 1).min() else {
        return order;
    };

    for level in (lowest_odd..=max).rev() {
        let mut i = 0;
        while i < order.len() {
            if run_levels[order[i]] >= level {
                let start = i;
                while i < order.len() && run_levels[order[i]] >= level {
                    i += 1;
                }
                order[start..i].reverse();
            } else {
                i += 1;
            }
        }
    }
    order
}

fn is_removed_by_x9(t: BidiClass) -> bool {
    matches!(
        t,
        BidiClass::BN
            | BidiClass::LRE
            | BidiClass::LRO
            | BidiClass::RLE
            | BidiClass::RLO
            | BidiClass::PDF
            | BidiClass::LRI
            | BidiClass::RLI
            | BidiClass::FSI
            | BidiClass::PDI
    )
}

/// W1–W7 over the characters at `idx`
fn resolve_weak(types: &mut [BidiClass], idx: &[usize], sor: BidiClass) {
    // W1: NSM takes the type of the previous character
    let mut prev = sor;
    for &i in idx {
        if types[i] == BidiClass::NSM {
            types[i] = prev;
        }
        prev = types[i];
    }

    // W2: EN after AL becomes AN; W3: AL becomes R
    let mut last_strong = sor;
    for &i in idx {
        match types[i] {
            BidiClass::L | BidiClass::R | BidiClass::AL => last_strong = types[i],
            BidiClass::EN if last_strong == BidiClass::AL => types[i] = BidiClass::AN,
            _ => {}
        }
    }
    for &i in idx {
        if types[i] == BidiClass::AL {
            types[i] = BidiClass::R;
        }
    }

    // W4: a single ES between two ENs becomes EN; a single CS between two
    // numbers of the same type becomes that type
    for k in 1..idx.len().saturating_sub(1) {
        let (before, here, after) = (types[idx[k - 1]], types[idx[k]], types[idx[k + 1]]);
        let joined = match (before, here, after) {
            (BidiClass::EN, BidiClass::ES | BidiClass::CS, BidiClass::EN) => Some(BidiClass::EN),
            (BidiClass::AN, BidiClass::CS, BidiClass::AN) => Some(BidiClass::AN),
            _ => None,
        };
        if let Some(t) = joined {
            types[idx[k]] = t;
        }
    }

    // W5: a sequence of ETs next to an EN becomes EN
    let mut k = 0;
    while k < idx.len() {
        if types[idx[k]] != BidiClass::ET {
            k += 1;
            continue;
        }
        let start = k;
        while k < idx.len() && types[idx[k]] == BidiClass::ET {
            k += 1;
        }
        let touches_en = (start > 0 && types[idx[start - 1]] == BidiClass::EN)
            || (k < idx.len() && types[idx[k]] == BidiClass::EN);
        if touches_en {
            for &i in &idx[start..k] {
                types[i] = BidiClass::EN;
            }
        }
    }

    // W6: remaining separators and terminators become ON
    for &i in idx {
        if matches!(types[i], BidiClass::ES | BidiClass::ET | BidiClass::CS) {
            types[i] = BidiClass::ON;
        }
    }

    // W7: EN after L (or at the start of an LTR paragraph) becomes L
    let mut last_strong = sor;
    for &i in idx {
        match types[i] {
            BidiClass::L | BidiClass::R => last_strong = types[i],
            BidiClass::EN if last_strong == BidiClass::L => types[i] = BidiClass::L,
            _ => {}
        }
    }
}

/// N1–N2 over the characters at `idx`
fn resolve_neutrals(types: &mut [BidiClass], idx: &[usize], sor: BidiClass) {
    // Numbers count as R when resolving neutrals
    let strong = |t: BidiClass| match t {
        BidiClass::L => Some(BidiClass::L),
        BidiClass::R | BidiClass::EN | BidiClass::AN => Some(BidiClass::R),
        _ => None,
    };

    let mut k = 0;
    while k < idx.len() {
        if strong(types[idx[k]]).is_some() {
            k += 1;
            continue;
        }
        let start = k;
        while k < idx.len() && strong(types[idx[k]]).is_none() {
            k += 1;
        }
        let before = if start == 0 {
            sor
        } else {
            strong(types[idx[start - 1]]).unwrap_or(sor)
        };
        // eor is the paragraph direction (single line, no embeddings)
        let after = if k == idx.len() {
            sor
        } else {
            strong(types[idx[k]]).unwrap_or(sor)
        };
        // N1: same direction on both sides; N2: otherwise the paragraph's
        let resolved = if before == after { before } else { sor };
        for &i in &idx[start..k] {
            types[i] = resolved;
        }
    }
}

/// I1–I2
fn implicit_level(base_level: u8, t: BidiClass) -> u8 {
    if base_level.is_multiple_of(2) {
        match t {
            BidiClass::R => base_level + 1,
            BidiClass::AN | BidiClass::EN => base_level + 2,
            _ => base_level,
        }
    } else {
        match t {
            BidiClass::L | BidiClass::EN | BidiClass::AN => base_level + 1,
            _ => base_level,
        }
    }
}
