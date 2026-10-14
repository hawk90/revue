//! The glyph rows each style draws for a digit, `:`, `.`, `-` and anything else

use super::DigitStyle;

/// 7-segment digit patterns (3x5 characters each)
/// Each digit is represented as 5 rows of 3 characters
const DIGIT_PATTERNS_BLOCK: [[&str; 5]; 10] = [
    // 0
    ["███", "█ █", "█ █", "█ █", "███"],
    // 1
    ["  █", "  █", "  █", "  █", "  █"],
    // 2
    ["███", "  █", "███", "█  ", "███"],
    // 3
    ["███", "  █", "███", "  █", "███"],
    // 4
    ["█ █", "█ █", "███", "  █", "  █"],
    // 5
    ["███", "█  ", "███", "  █", "███"],
    // 6
    ["███", "█  ", "███", "█ █", "███"],
    // 7
    ["███", "  █", "  █", "  █", "  █"],
    // 8
    ["███", "█ █", "███", "█ █", "███"],
    // 9
    ["███", "█ █", "███", "  █", "███"],
];

/// Thin digit patterns (3x5 characters each)
const DIGIT_PATTERNS_THIN: [[&str; 5]; 10] = [
    // 0
    ["┌─┐", "│ │", "│ │", "│ │", "└─┘"],
    // 1
    ["  │", "  │", "  │", "  │", "  │"],
    // 2
    ["──┐", "  │", "┌─┘", "│  ", "└──"],
    // 3
    ["──┐", "  │", "──┤", "  │", "──┘"],
    // 4
    ["│ │", "│ │", "└─┤", "  │", "  │"],
    // 5
    ["┌──", "│  ", "└─┐", "  │", "──┘"],
    // 6
    ["┌──", "│  ", "├─┐", "│ │", "└─┘"],
    // 7
    ["──┐", "  │", "  │", "  │", "  │"],
    // 8
    ["┌─┐", "│ │", "├─┤", "│ │", "└─┘"],
    // 9
    ["┌─┐", "│ │", "└─┤", "  │", "──┘"],
];

/// ASCII digit patterns (3x5 characters each)
const DIGIT_PATTERNS_ASCII: [[&str; 5]; 10] = [
    // 0
    ["+-+", "| |", "| |", "| |", "+-+"],
    // 1
    ["  |", "  |", "  |", "  |", "  |"],
    // 2
    ["--+", "  |", "+-+", "|  ", "+--"],
    // 3
    ["--+", "  |", "--+", "  |", "--+"],
    // 4
    ["| |", "| |", "+-+", "  |", "  |"],
    // 5
    ["+--", "|  ", "+-+", "  |", "--+"],
    // 6
    ["+--", "|  ", "+-+", "| |", "+-+"],
    // 7
    ["--+", "  |", "  |", "  |", "  |"],
    // 8
    ["+-+", "| |", "+-+", "| |", "+-+"],
    // 9
    ["+-+", "| |", "+-+", "  |", "--+"],
];

/// Braille digit patterns (2x4 characters each, using braille)
const DIGIT_PATTERNS_BRAILLE: [[&str; 4]; 10] = [
    // 0
    ["⣰⣆", "⡇⢸", "⡇⢸", "⠈⠉"],
    // 1
    [" ⡆", " ⡇", " ⡇", " ⠁"],
    // 2
    ["⠤⡤", " ⡰", "⡰ ", "⠧⠤"],
    // 3
    ["⠤⡤", " ⡤", " ⡤", "⠤⠴"],
    // 4
    ["⡆⡆", "⡧⡦", " ⡇", " ⠁"],
    // 5
    ["⡤⠤", "⡤⠤", " ⢸", "⠤⠴"],
    // 6
    ["⣰⠆", "⡧⠤", "⡇⢸", "⠈⠉"],
    // 7
    ["⠤⡤", " ⡰", " ⡇", " ⠁"],
    // 8
    ["⣰⣆", "⡧⡦", "⡇⢸", "⠈⠉"],
    // 9
    ["⣰⣆", "⡧⡦", " ⢸", "⠤⠴"],
];

/// Special character patterns
const COLON_BLOCK: [&str; 5] = [" ", "█", " ", "█", " "];
const COLON_THIN: [&str; 5] = [" ", "●", " ", "●", " "];
const COLON_ASCII: [&str; 5] = [" ", "o", " ", "o", " "];
const COLON_BRAILLE: [&str; 4] = [" ", "⠂", "⠂", " "];

const DOT_BLOCK: [&str; 5] = [" ", " ", " ", " ", "█"];
const DOT_THIN: [&str; 5] = [" ", " ", " ", " ", "●"];
const DOT_ASCII: [&str; 5] = [" ", " ", " ", " ", "o"];
const DOT_BRAILLE: [&str; 4] = [" ", " ", " ", "⠂"];

const MINUS_BLOCK: [&str; 5] = ["   ", "   ", "███", "   ", "   "];
const MINUS_THIN: [&str; 5] = ["   ", "   ", "───", "   ", "   "];
const MINUS_ASCII: [&str; 5] = ["   ", "   ", "---", "   ", "   "];
const MINUS_BRAILLE: [&str; 4] = ["  ", "⠤⠤", "  ", "  "];

const SPACE_BLOCK: [&str; 5] = ["   ", "   ", "   ", "   ", "   "];
const SPACE_BRAILLE: [&str; 4] = ["  ", "  ", "  ", "  "];

/// The rows `style` draws for `c`: a digit's pattern, the colon, dot and
/// minus, or blank rows for any other character.
pub(super) fn char_pattern(style: DigitStyle, c: char) -> Vec<&'static str> {
    match style {
        DigitStyle::Block => lookup(
            &DIGIT_PATTERNS_BLOCK,
            &COLON_BLOCK,
            &DOT_BLOCK,
            &MINUS_BLOCK,
            &SPACE_BLOCK,
            c,
        ),
        DigitStyle::Thin => lookup(
            &DIGIT_PATTERNS_THIN,
            &COLON_THIN,
            &DOT_THIN,
            &MINUS_THIN,
            &SPACE_BLOCK,
            c,
        ),
        DigitStyle::Ascii => lookup(
            &DIGIT_PATTERNS_ASCII,
            &COLON_ASCII,
            &DOT_ASCII,
            &MINUS_ASCII,
            &SPACE_BLOCK,
            c,
        ),
        DigitStyle::Braille => lookup(
            &DIGIT_PATTERNS_BRAILLE,
            &COLON_BRAILLE,
            &DOT_BRAILLE,
            &MINUS_BRAILLE,
            &SPACE_BRAILLE,
            c,
        ),
    }
}

/// One style's lookup: the four styles differ only in their tables.
fn lookup<const N: usize>(
    digits: &[[&'static str; N]; 10],
    colon: &[&'static str; N],
    dot: &[&'static str; N],
    minus: &[&'static str; N],
    space: &[&'static str; N],
    c: char,
) -> Vec<&'static str> {
    match c {
        '0'..='9' => digits[(c as usize) - ('0' as usize)].to_vec(),
        ':' => colon.to_vec(),
        '.' => dot.to_vec(),
        '-' => minus.to_vec(),
        _ => space.to_vec(),
    }
}
