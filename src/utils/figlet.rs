//! Figlet-style ASCII art text rendering
//!
//! Renders text as large ASCII art characters.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::figlet::{figlet, FigletFont};
//!
//! let art = figlet("Hello");
//! println!("{}", art);
//! // ██╗  ██╗███████╗██╗     ██╗      ██████╗
//! // ██║  ██║██╔════╝██║     ██║     ██╔═══██╗
//! // ███████║█████╗  ██║     ██║     ██║   ██║
//! // ██╔══██║██╔════╝██║     ██║     ██║   ██║
//! // ██║  ██║███████╗███████╗███████╗╚██████╔╝
//! // ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝ ╚═════╝
//! ```

/// Figlet font style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FigletFont {
    /// Block style using Unicode box-drawing characters
    #[default]
    Block,
    /// Slant style
    Slant,
    /// Simple banner style
    Banner,
    /// Small compact style
    Small,
    /// Mini style (3 rows)
    Mini,
}

/// Render text as figlet ASCII art
pub fn figlet(text: &str) -> String {
    figlet_with_font(text, FigletFont::default())
}

/// Render text as figlet ASCII art with specified font
pub fn figlet_with_font(text: &str, font: FigletFont) -> String {
    match font {
        FigletFont::Block => render_block(text),
        FigletFont::Slant => render_slant(text),
        FigletFont::Banner => render_banner(text),
        FigletFont::Small => render_small(text),
        FigletFont::Mini => render_mini(text),
    }
}

/// Get the height of a figlet font in rows
pub fn font_height(font: FigletFont) -> usize {
    match font {
        FigletFont::Block => 6,
        FigletFont::Slant => 6,
        FigletFont::Banner => 7,
        FigletFont::Small => 5,
        FigletFont::Mini => 3,
    }
}

/// Render text in block style
fn render_block(text: &str) -> String {
    let text = text.to_uppercase();
    let mut lines = vec![String::new(); 6];

    for ch in text.chars() {
        let glyph = block_char(ch);
        for (i, line) in glyph.iter().enumerate() {
            lines[i].push_str(line);
        }
    }

    lines.join("\n")
}

/// Get block-style glyph for a character
fn block_char(ch: char) -> [&'static str; 6] {
    match ch {
        'A' => [
            " █████╗ ",
            "██╔══██╗",
            "███████║",
            "██╔══██║",
            "██║  ██║",
            "╚═╝  ╚═╝",
        ],
        'B' => [
            "██████╗ ",
            "██╔══██╗",
            "██████╔╝",
            "██╔══██╗",
            "██████╔╝",
            "╚═════╝ ",
        ],
        'C' => [
            " ██████╗",
            "██╔════╝",
            "██║     ",
            "██║     ",
            "╚██████╗",
            " ╚═════╝",
        ],
        'D' => [
            "██████╗ ",
            "██╔══██╗",
            "██║  ██║",
            "██║  ██║",
            "██████╔╝",
            "╚═════╝ ",
        ],
        'E' => [
            "███████╗",
            "██╔════╝",
            "█████╗  ",
            "██╔══╝  ",
            "███████╗",
            "╚══════╝",
        ],
        'F' => [
            "███████╗",
            "██╔════╝",
            "█████╗  ",
            "██╔══╝  ",
            "██║     ",
            "╚═╝     ",
        ],
        'G' => [
            " ██████╗ ",
            "██╔════╝ ",
            "██║  ███╗",
            "██║   ██║",
            "╚██████╔╝",
            " ╚═════╝ ",
        ],
        'H' => [
            "██╗  ██╗",
            "██║  ██║",
            "███████║",
            "██╔══██║",
            "██║  ██║",
            "╚═╝  ╚═╝",
        ],
        'I' => ["██╗", "██║", "██║", "██║", "██║", "╚═╝"],
        'J' => [
            "     ██╗",
            "     ██║",
            "     ██║",
            "██   ██║",
            "╚█████╔╝",
            " ╚════╝ ",
        ],
        'K' => [
            "██╗  ██╗",
            "██║ ██╔╝",
            "█████╔╝ ",
            "██╔═██╗ ",
            "██║  ██╗",
            "╚═╝  ╚═╝",
        ],
        'L' => [
            "██╗     ",
            "██║     ",
            "██║     ",
            "██║     ",
            "███████╗",
            "╚══════╝",
        ],
        'M' => [
            "███╗   ███╗",
            "████╗ ████║",
            "██╔████╔██║",
            "██║╚██╔╝██║",
            "██║ ╚═╝ ██║",
            "╚═╝     ╚═╝",
        ],
        'N' => [
            "███╗   ██╗",
            "████╗  ██║",
            "██╔██╗ ██║",
            "██║╚██╗██║",
            "██║ ╚████║",
            "╚═╝  ╚═══╝",
        ],
        'O' => [
            " ██████╗ ",
            "██╔═══██╗",
            "██║   ██║",
            "██║   ██║",
            "╚██████╔╝",
            " ╚═════╝ ",
        ],
        'P' => [
            "██████╗ ",
            "██╔══██╗",
            "██████╔╝",
            "██╔═══╝ ",
            "██║     ",
            "╚═╝     ",
        ],
        'Q' => [
            " ██████╗ ",
            "██╔═══██╗",
            "██║   ██║",
            "██║▄▄ ██║",
            "╚██████╔╝",
            " ╚══▀▀═╝ ",
        ],
        'R' => [
            "██████╗ ",
            "██╔══██╗",
            "██████╔╝",
            "██╔══██╗",
            "██║  ██║",
            "╚═╝  ╚═╝",
        ],
        'S' => [
            "███████╗",
            "██╔════╝",
            "███████╗",
            "╚════██║",
            "███████║",
            "╚══════╝",
        ],
        'T' => [
            "████████╗",
            "╚══██╔══╝",
            "   ██║   ",
            "   ██║   ",
            "   ██║   ",
            "   ╚═╝   ",
        ],
        'U' => [
            "██╗   ██╗",
            "██║   ██║",
            "██║   ██║",
            "██║   ██║",
            "╚██████╔╝",
            " ╚═════╝ ",
        ],
        'V' => [
            "██╗   ██╗",
            "██║   ██║",
            "██║   ██║",
            "╚██╗ ██╔╝",
            " ╚████╔╝ ",
            "  ╚═══╝  ",
        ],
        'W' => [
            "██╗    ██╗",
            "██║    ██║",
            "██║ █╗ ██║",
            "██║███╗██║",
            "╚███╔███╔╝",
            " ╚══╝╚══╝ ",
        ],
        'X' => [
            "██╗  ██╗",
            "╚██╗██╔╝",
            " ╚███╔╝ ",
            " ██╔██╗ ",
            "██╔╝ ██╗",
            "╚═╝  ╚═╝",
        ],
        'Y' => [
            "██╗   ██╗",
            "╚██╗ ██╔╝",
            " ╚████╔╝ ",
            "  ╚██╔╝  ",
            "   ██║   ",
            "   ╚═╝   ",
        ],
        'Z' => [
            "███████╗",
            "╚══███╔╝",
            "  ███╔╝ ",
            " ███╔╝  ",
            "███████╗",
            "╚══════╝",
        ],
        '0' => [
            " ██████╗ ",
            "██╔═████╗",
            "██║██╔██║",
            "████╔╝██║",
            "╚██████╔╝",
            " ╚═════╝ ",
        ],
        '1' => [" ██╗", "███║", "╚██║", " ██║", " ██║", " ╚═╝"],
        '2' => [
            "██████╗ ",
            "╚════██╗",
            " █████╔╝",
            "██╔═══╝ ",
            "███████╗",
            "╚══════╝",
        ],
        '3' => [
            "██████╗ ",
            "╚════██╗",
            " █████╔╝",
            " ╚═══██╗",
            "██████╔╝",
            "╚═════╝ ",
        ],
        '4' => [
            "██╗  ██╗",
            "██║  ██║",
            "███████║",
            "╚════██║",
            "     ██║",
            "     ╚═╝",
        ],
        '5' => [
            "███████╗",
            "██╔════╝",
            "███████╗",
            "╚════██║",
            "███████║",
            "╚══════╝",
        ],
        '6' => [
            " ██████╗",
            "██╔════╝",
            "███████╗",
            "██╔══██║",
            "╚█████╔╝",
            " ╚════╝ ",
        ],
        '7' => [
            "███████╗",
            "╚════██║",
            "    ██╔╝",
            "   ██╔╝ ",
            "   ██║  ",
            "   ╚═╝  ",
        ],
        '8' => [
            " █████╗ ",
            "██╔══██╗",
            "╚█████╔╝",
            "██╔══██╗",
            "╚█████╔╝",
            " ╚════╝ ",
        ],
        '9' => [
            " █████╗ ",
            "██╔══██╗",
            "╚██████║",
            " ╚═══██║",
            " █████╔╝",
            " ╚════╝ ",
        ],
        ' ' => ["   ", "   ", "   ", "   ", "   ", "   "],
        '!' => ["██╗", "██║", "██║", "╚═╝", "██╗", "╚═╝"],
        '?' => [
            "██████╗ ",
            "╚════██╗",
            "  ▄███╔╝",
            "  ▀▀══╝ ",
            "  ██╗   ",
            "  ╚═╝   ",
        ],
        '.' => ["   ", "   ", "   ", "   ", "██╗", "╚═╝"],
        ',' => ["   ", "   ", "   ", "   ", "▄█╗", "╚═╝"],
        ':' => ["   ", "██╗", "╚═╝", "██╗", "╚═╝", "   "],
        '-' => ["      ", "      ", "█████╗", "╚════╝", "      ", "      "],
        '_' => [
            "        ",
            "        ",
            "        ",
            "        ",
            "███████╗",
            "╚══════╝",
        ],
        '+' => [
            "       ",
            "  ██╗  ",
            "██████╗",
            "╚═██╔═╝",
            "  ╚═╝  ",
            "       ",
        ],
        '=' => ["      ", "█████╗", "╚════╝", "█████╗", "╚════╝", "      "],
        '/' => [
            "    ██╗",
            "   ██╔╝",
            "  ██╔╝ ",
            " ██╔╝  ",
            "██╔╝   ",
            "╚═╝    ",
        ],
        '#' => [
            " ██╗ ██╗ ",
            "████████╗",
            "╚██╔═██╔╝",
            "████████╗",
            "╚██╔═██╔╝",
            " ╚═╝ ╚═╝ ",
        ],
        '@' => [
            " ██████╗ ",
            "██╔═══██╗",
            "██║██╗██║",
            "██║██║██║",
            "╚█╔═══╝╔╝",
            " ╚════╝  ",
        ],
        _ => ["▄▄▄", "███", "███", "███", "███", "▀▀▀"],
    }
}

/// Render text in slant style
fn render_slant(text: &str) -> String {
    let text = text.to_uppercase();
    let mut lines = vec![String::new(); 6];

    for ch in text.chars() {
        let glyph = slant_char(ch);
        for (i, line) in glyph.iter().enumerate() {
            lines[i].push_str(line);
        }
    }

    lines.join("\n")
}

/// Get slant-style glyph for a character
fn slant_char(ch: char) -> [&'static str; 6] {
    match ch {
        'A' => [
            "   ___   ",
            "  / _ \\  ",
            " / /_\\ \\ ",
            "/  _  \\ \\",
            "/_/ |_/\\_\\",
            "          ",
        ],
        'B' => [
            " ____  ", "| __ ) ", "|  _ \\ ", "| |_) |", "|____/ ", "       ",
        ],
        'C' => [
            "  ____ ", " / ___|", "| |    ", "| |___ ", " \\____|", "       ",
        ],
        'D' => [
            " ____  ", "|  _ \\ ", "| | | |", "| |_| |", "|____/ ", "       ",
        ],
        'E' => [
            " _____ ", "| ____|", "|  _|  ", "| |___ ", "|_____|", "       ",
        ],
        'F' => [
            " _____ ", "|  ___|", "| |_   ", "|  _|  ", "|_|    ", "       ",
        ],
        'H' => [
            " _   _ ", "| | | |", "| |_| |", "|  _  |", "|_| |_|", "       ",
        ],
        'I' => [" ___ ", "|_ _|", " | | ", " | | ", "|___|", "     "],
        'L' => [
            " _     ", "| |    ", "| |    ", "| |___ ", "|_____|", "       ",
        ],
        'O' => [
            "  ___  ", " / _ \\ ", "| | | |", "| |_| |", " \\___/ ", "       ",
        ],
        'R' => [
            " ____  ",
            "|  _ \\ ",
            "| |_) |",
            "|  _ < ",
            "|_| \\_\\",
            "       ",
        ],
        'S' => [
            " ____  ",
            "/ ___| ",
            "\\___ \\ ",
            " ___) |",
            "|____/ ",
            "       ",
        ],
        'T' => [
            " _____ ", "|_   _|", "  | |  ", "  | |  ", "  |_|  ", "       ",
        ],
        'U' => [
            " _   _ ", "| | | |", "| | | |", "| |_| |", " \\___/ ", "       ",
        ],
        'V' => [
            "__     __",
            "\\ \\   / /",
            " \\ \\ / / ",
            "  \\ V /  ",
            "   \\_/   ",
            "         ",
        ],
        'W' => [
            "__        __",
            "\\ \\      / /",
            " \\ \\ /\\ / / ",
            "  \\ V  V /  ",
            "   \\_/\\_/   ",
            "            ",
        ],
        ' ' => ["   ", "   ", "   ", "   ", "   ", "   "],
        _ => block_char(ch), // Fallback to block style
    }
}

/// Render text in banner style
fn render_banner(text: &str) -> String {
    let text = text.to_uppercase();
    let mut lines = vec![String::new(); 7];

    for ch in text.chars() {
        let glyph = banner_char(ch);
        for (i, line) in glyph.iter().enumerate() {
            lines[i].push_str(line);
        }
    }

    lines.join("\n")
}

/// Get banner-style glyph for a character
fn banner_char(ch: char) -> [&'static str; 7] {
    match ch {
        'A' => [
            "  ##   ", " #  #  ", "#    # ", "###### ", "#    # ", "#    # ", "#    # ",
        ],
        'B' => [
            "#####  ", "#    # ", "#####  ", "#    # ", "#    # ", "#    # ", "#####  ",
        ],
        'C' => [
            " ##### ", "#     #", "#      ", "#      ", "#      ", "#     #", " ##### ",
        ],
        'E' => [
            "###### ", "#      ", "#####  ", "#      ", "#      ", "#      ", "###### ",
        ],
        'H' => [
            "#    # ", "#    # ", "###### ", "#    # ", "#    # ", "#    # ", "#    # ",
        ],
        'L' => [
            "#      ", "#      ", "#      ", "#      ", "#      ", "#      ", "###### ",
        ],
        'O' => [
            " ####  ", "#    # ", "#    # ", "#    # ", "#    # ", "#    # ", " ####  ",
        ],
        'R' => [
            "#####  ", "#    # ", "#####  ", "#  #   ", "#   #  ", "#    # ", "#    # ",
        ],
        'W' => [
            "#     # ", "#     # ", "#  #  # ", "#  #  # ", "#  #  # ", " ## ## ", " #   #  ",
        ],
        ' ' => ["   ", "   ", "   ", "   ", "   ", "   ", "   "],
        _ => {
            // Convert 6-line block to 7-line
            let block = block_char(ch);
            [
                block[0], block[1], block[2], block[3], block[4], block[5], "       ",
            ]
        }
    }
}

/// Render text in small style (5 rows)
fn render_small(text: &str) -> String {
    let text = text.to_uppercase();
    let mut lines = vec![String::new(); 5];

    for ch in text.chars() {
        let glyph = small_char(ch);
        for (i, line) in glyph.iter().enumerate() {
            lines[i].push_str(line);
        }
    }

    lines.join("\n")
}

/// Get small-style glyph for a character
fn small_char(ch: char) -> [&'static str; 5] {
    match ch {
        'A' => [" ▄▄ ", "█▄▄█", "█  █", "█  █", "    "],
        'B' => ["█▀▀▄", "█▀▀▄", "█▄▄▀", "    ", "    "],
        'C' => ["▄▀▀▀", "█   ", "▀▄▄▄", "    ", "    "],
        'D' => ["█▀▀▄", "█  █", "█▄▄▀", "    ", "    "],
        'E' => ["█▀▀▀", "█▀▀ ", "█▄▄▄", "    ", "    "],
        'F' => ["█▀▀▀", "█▀▀ ", "█   ", "    ", "    "],
        'G' => ["▄▀▀▀", "█ ▀█", "▀▄▄▀", "    ", "    "],
        'H' => ["█  █", "█▀▀█", "█  █", "    ", "    "],
        'I' => ["█", "█", "█", " ", " "],
        'J' => ["   █", "   █", "▀▄▄▀", "    ", "    "],
        'K' => ["█ ▄▀", "██  ", "█ ▀▄", "    ", "    "],
        'L' => ["█   ", "█   ", "█▄▄▄", "    ", "    "],
        'M' => ["█▄ ▄█", "█ ▀ █", "█   █", "     ", "     "],
        'N' => ["█▄  █", "█ ▀▄█", "█   █", "     ", "     "],
        'O' => ["▄▀▀▄", "█  █", "▀▄▄▀", "    ", "    "],
        'P' => ["█▀▀▄", "█▄▄▀", "█   ", "    ", "    "],
        'Q' => ["▄▀▀▄", "█  █", "▀▄▄█", "    ", "    "],
        'R' => ["█▀▀▄", "█▄▄▀", "█  █", "    ", "    "],
        'S' => ["▄▀▀▀", " ▀▀▄", "▄▄▄▀", "    ", "    "],
        'T' => ["▀█▀", " █ ", " █ ", "   ", "   "],
        'U' => ["█  █", "█  █", "▀▄▄▀", "    ", "    "],
        'V' => ["█  █", "▀▄▄▀", " ▀▀ ", "    ", "    "],
        'W' => ["█   █", "█ ▄ █", "▀▄▀▄▀", "     ", "     "],
        'X' => ["█ █", " █ ", "█ █", "   ", "   "],
        'Y' => ["█ █", " █ ", " █ ", "   ", "   "],
        'Z' => ["▀▀▀█", " ▄▀ ", "█▄▄▄", "    ", "    "],
        ' ' => ["  ", "  ", "  ", "  ", "  "],
        _ => ["▄▄", "██", "▀▀", "  ", "  "],
    }
}

/// Render text in mini style (3 rows)
fn render_mini(text: &str) -> String {
    let text = text.to_uppercase();
    let mut lines = vec![String::new(); 3];

    for ch in text.chars() {
        let glyph = mini_char(ch);
        for (i, line) in glyph.iter().enumerate() {
            lines[i].push_str(line);
        }
    }

    lines.join("\n")
}

/// Get mini-style glyph for a character
fn mini_char(ch: char) -> [&'static str; 3] {
    match ch {
        'A' => ["▄█▄", "█▀█", "▀ ▀"],
        'B' => ["██▄", "██▀", "▀▀ "],
        'C' => ["▄█▀", "█  ", "▀█▄"],
        'D' => ["██▄", "█ █", "▀▀ "],
        'E' => ["██▀", "█▀ ", "▀▀▀"],
        'F' => ["██▀", "█▀ ", "▀  "],
        'G' => ["▄█▀", "█▄█", "▀▀▀"],
        'H' => ["█ █", "███", "▀ ▀"],
        'I' => ["█", "█", "▀"],
        'J' => [" █", " █", "▀▀"],
        'K' => ["█▄▀", "█▀▄", "▀ ▀"],
        'L' => ["█  ", "█  ", "▀▀▀"],
        'M' => ["█▄█", "█▀█", "▀ ▀"],
        'N' => ["█▀█", "█ █", "▀ ▀"],
        'O' => ["▄█▄", "█ █", "▀▀▀"],
        'P' => ["██▄", "█▀ ", "▀  "],
        'Q' => ["▄█▄", "█ █", "▀▀█"],
        'R' => ["██▄", "█▀▄", "▀ ▀"],
        'S' => ["▄█▀", "▀█▄", "▀▀ "],
        'T' => ["▀█▀", " █ ", " ▀ "],
        'U' => ["█ █", "█ █", "▀▀▀"],
        'V' => ["█ █", "█ █", " ▀ "],
        'W' => ["█ █", "█▄█", "▀ ▀"],
        'X' => ["▀▄▀", " █ ", "▀ ▀"],
        'Y' => ["█ █", " █ ", " ▀ "],
        'Z' => ["▀▀█", " █ ", "█▀▀"],
        '0' => ["▄█▄", "█ █", "▀▀▀"],
        '1' => ["▄█", " █", "▀▀"],
        '2' => ["▀█▄", "▄█▀", "▀▀▀"],
        '3' => ["▀█▄", " █▄", "▀▀ "],
        '4' => ["█ █", "▀▀█", "  ▀"],
        '5' => ["█▀▀", "▀█▄", "▀▀ "],
        '6' => ["▄█▀", "██▄", "▀▀ "],
        '7' => ["▀▀█", "  █", "  ▀"],
        '8' => ["▄█▄", "▀█▀", "▀▀▀"],
        '9' => ["▄█▄", "▀▀█", "▀▀ "],
        ' ' => [" ", " ", " "],
        '!' => ["█", " ", "▀"],
        '?' => ["▀█", " █", " ▀"],
        '.' => [" ", " ", "▀"],
        '-' => ["  ", "▀▀", "  "],
        _ => ["▄", "█", "▀"],
    }
}

/// Get lines of figlet text as a vector
pub fn figlet_lines(text: &str, font: FigletFont) -> Vec<String> {
    figlet_with_font(text, font)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figlet_block() {
        let result = figlet("HI");
        assert!(result.contains("██╗"));
        assert!(result.lines().count() == 6);
    }

    #[test]
    fn test_figlet_small() {
        let result = figlet_with_font("AB", FigletFont::Small);
        assert!(result.lines().count() == 5);
    }

    #[test]
    fn test_figlet_mini() {
        let result = figlet_with_font("XY", FigletFont::Mini);
        assert!(result.lines().count() == 3);
    }

    #[test]
    fn test_font_height() {
        assert_eq!(font_height(FigletFont::Block), 6);
        assert_eq!(font_height(FigletFont::Small), 5);
        assert_eq!(font_height(FigletFont::Mini), 3);
    }

    #[test]
    fn test_figlet_lines() {
        let lines = figlet_lines("A", FigletFont::Block);
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn test_figlet_space() {
        let result = figlet("A B");
        // Should contain space between A and B
        assert!(result.contains("   "));
    }

    #[test]
    fn test_figlet_numbers() {
        let result = figlet("123");
        assert!(result.lines().count() == 6);
    }

    #[test]
    fn test_figlet_special_chars() {
        let result = figlet("!?.");
        assert!(result.lines().count() == 6);
    }
}
