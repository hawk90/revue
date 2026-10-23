//! Width and grapheme checks against the Unicode data files (#1167)
//!
//! The files in `unicode/17.0.0/` are copied unchanged from
//! <https://www.unicode.org/Public/17.0.0/> (`ucd/EastAsianWidth.txt`,
//! `ucd/extracted/DerivedGeneralCategory.txt`,
//! `ucd/auxiliary/GraphemeBreakTest.txt`, `emoji/emoji-test.txt`). When
//! unicode-width, unicode-segmentation or unicode-properties moves to a new
//! Unicode version, `the_data_is_the_unicode_version_of_the_dependencies`
//! fails: put the new version's files next to these, point [`DATA`] at them
//! and delete the old ones.
//!
//! Each width check states the rule terminals measure with and lists every
//! code point where `char_width` departs from it, so a dependency update that
//! moves a character shows up here.

use std::collections::HashMap;

use revue::text::{reverse_graphemes, CharWidthTable};
use revue::utils::char_width;

const DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/text/unicode/17.0.0");

fn read(file: &str) -> String {
    std::fs::read_to_string(format!("{DATA}/{file}")).unwrap()
}

/// `code point -> value` from a `XXXX..YYYY ; value` property file
fn property(file: &str) -> HashMap<u32, String> {
    let mut map = HashMap::new();
    for line in read(file).lines() {
        let line = line.split('#').next().unwrap().trim();
        let Some((range, value)) = line.split_once(';') else {
            continue;
        };
        let range = range.trim();
        let (first, last) = range.split_once("..").unwrap_or((range, range));
        let first = u32::from_str_radix(first, 16).unwrap();
        let last = u32::from_str_radix(last, 16).unwrap();
        for cp in first..=last {
            map.insert(cp, value.trim().to_string());
        }
    }
    map
}

fn hex_char(hex: &str) -> char {
    char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap()
}

/// Every assigned character with its East Asian Width and General Category
fn characters() -> Vec<(char, String, String)> {
    let eaw = property("EastAsianWidth.txt");
    let gc = property("DerivedGeneralCategory.txt");
    (0..=0x10_FFFF)
        .filter_map(char::from_u32)
        .filter_map(|c| {
            let gc = gc.get(&(c as u32))?;
            (gc != "Cn").then(|| {
                // Unlisted code points are Neutral
                let eaw = eaw.get(&(c as u32)).cloned().unwrap_or("N".into());
                (c, eaw, gc.clone())
            })
        })
        .collect()
}

/// The characters matching `select` whose width is not `expected`, other
/// than the listed exceptions, plus exceptions that no longer differ
fn mismatches(
    select: impl Fn(char, &str, &str) -> bool,
    expected: impl Fn(char) -> usize,
    exceptions: &[(u32, usize)],
) -> Vec<String> {
    let mut found = Vec::new();
    for (c, eaw, gc) in characters() {
        if !select(c, &eaw, &gc) {
            continue;
        }
        let width = char_width(c);
        let exception = exceptions.iter().find(|&&(cp, _)| cp == c as u32);
        let allowed = exception.map_or(expected(c), |&(_, w)| w);
        if width != allowed {
            found.push(format!(
                "U+{:04X} ({eaw}, {gc}): width {width}, expected {allowed}",
                c as u32
            ));
        }
    }
    for &(cp, _) in exceptions {
        let c = char::from_u32(cp).unwrap();
        if char_width(c) == expected(c) {
            found.push(format!(
                "U+{cp:04X} follows the rule now: drop the exception"
            ));
        }
    }
    found
}

#[test]
fn the_data_is_the_unicode_version_of_the_dependencies() {
    assert!(DATA.ends_with("/17.0.0"));
    assert_eq!(unicode_width::UNICODE_VERSION, (17, 0, 0));
    assert_eq!(unicode_segmentation::UNICODE_VERSION, (17, 0, 0));
    assert_eq!(unicode_properties::UNICODE_VERSION, (17, 0, 0));
}

#[test]
fn wide_and_fullwidth_characters_take_two_columns() {
    // East Asian Width W or F, drawn by itself (not a mark or a format
    // character): CJK, Hangul syllables and leading consonants, fullwidth
    // forms, emoji
    let found = mismatches(
        |_, eaw, gc| matches!(eaw, "W" | "F") && !gc.starts_with(['M', 'C']),
        |_| 2,
        // HANGUL FILLER is invisible (Default_Ignorable_Code_Point)
        &[(0x3164, 0)],
    );
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn other_characters_take_at_most_one_column() {
    let found = mismatches(
        |_, eaw, _| !matches!(eaw, "W" | "F"),
        |c| char_width(c).min(1),
        &[],
    );
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn combining_marks_and_controls_take_no_column() {
    let found = mismatches(|_, _, gc| matches!(gc, "Mn" | "Me" | "Cc"), |_| 0, &[]);
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn conjoining_hangul_vowels_and_finals_take_no_column() {
    // Decomposed (NFD) Hangul: a leading consonant (2 columns) followed by
    // these is one syllable
    let found = mismatches(
        |c, _, _| matches!(c as u32, 0x1160..=0x11FF | 0xD7B0..=0xD7FF),
        |_| 0,
        &[],
    );
    assert!(found.is_empty(), "{found:#?}");
}

/// The code points of each fully-qualified emoji in `emoji-test.txt`
fn fully_qualified_emoji() -> Vec<Vec<char>> {
    read("emoji-test.txt")
        .lines()
        .filter_map(|line| {
            let (points, status) = line.split_once(';')?;
            status
                .trim_start()
                .starts_with("fully-qualified")
                .then(|| points.split_whitespace().map(hex_char).collect())
        })
        .collect()
}

#[test]
fn every_single_character_emoji_takes_two_columns() {
    let table = CharWidthTable::new().with_emoji(3);
    let single: Vec<char> = fully_qualified_emoji()
        .into_iter()
        .filter(|points| points.len() == 1)
        .map(|points| points[0])
        .collect();
    assert!(single.len() > 1000, "{} single emoji", single.len());

    for c in single {
        let cp = c as u32;
        assert_eq!(char_width(c), 2, "U+{cp:04X}");
        // `CharWidthTable` gives it the emoji width, not the CJK one
        assert_eq!(table.width(c), 3, "U+{cp:04X}");
    }
}

#[test]
fn the_glue_inside_emoji_sequences_takes_no_column() {
    // ZWJ, variation selector 16 and tag characters join or restyle the
    // emoji around them; they add no column of their own
    let mut glue: Vec<char> = fully_qualified_emoji()
        .into_iter()
        .flatten()
        .filter(|&c| matches!(c as u32, 0x200D | 0xFE0F | 0xE0020..=0xE007F))
        .collect();
    glue.sort();
    glue.dedup();
    assert!(glue.contains(&'\u{200D}') && glue.contains(&'\u{FE0F}'));
    assert!(
        glue.contains(&'\u{E007F}'),
        "tag sequences (subdivision flags)"
    );

    for c in glue {
        assert_eq!(char_width(c), 0, "U+{:04X}", c as u32);
    }
}

#[test]
fn reverse_graphemes_keeps_every_cluster_of_the_break_test_whole() {
    let mut cases = 0;
    for line in read("GraphemeBreakTest.txt").lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        // `÷ 0061 × 0308 ÷ 0062 ÷`: ÷ is a boundary, × is none
        let clusters: Vec<String> = line
            .split('÷')
            .map(str::trim)
            .filter(|cluster| !cluster.is_empty())
            .map(|cluster| cluster.split('×').map(|p| hex_char(p.trim())).collect())
            .collect();
        let text: String = clusters.concat();
        let reversed: String = clusters.iter().rev().map(String::as_str).collect();

        assert_eq!(reverse_graphemes(&text), reversed, "{line}");
        cases += 1;
    }
    assert!(cases > 700, "{cases} cases");
}
