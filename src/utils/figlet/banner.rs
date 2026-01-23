//! Banner style font rendering

/// Render text in banner style
pub fn render_banner(text: &str) -> String {
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
            let block = super::block::block_char(ch);
            [
                block[0], block[1], block[2], block[3], block[4], block[5], "       ",
            ]
        }
    }
}
