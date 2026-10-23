//! SGR (Select Graphic Rendition, `ESC [ ... m`): the codes that set the
//! colors and modifiers of the text after them

use crate::render::Modifier;
use crate::style::Color;

/// The colors and modifiers SGR codes have set. A color of `None` is the
/// default one (never set, or reset by 0, 39 or 49): what it looks like is
/// up to whoever draws it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SgrState {
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
    pub(crate) modifiers: Modifier,
}

impl SgrState {
    /// Apply the parameters of one SGR sequence (`1;31`, `38;2;r;g;b`,
    /// `38:2::r:g:b`). An empty parameter is 0, so `ESC [ m` resets.
    pub(crate) fn apply(&mut self, params: &str) {
        let groups: Vec<Vec<u32>> = params
            .split(';')
            .map(|group| group.split(':').map(number).collect())
            .collect();
        let mut i = 0;
        while i < groups.len() {
            let group = &groups[i];
            let code = group[0];
            if group.len() > 1 {
                // `38:5:n`, `38:2:r:g:b`, `4:3` (an underline style): the
                // sub-parameters belong to this code alone
                match code {
                    38 => self.fg = extended(&group[1..], true).or(self.fg),
                    48 => self.bg = extended(&group[1..], true).or(self.bg),
                    4 if group[1] == 0 => self.modifiers.remove(Modifier::UNDERLINE),
                    _ => self.code(code),
                }
                i += 1;
                continue;
            }
            match code {
                38 | 48 => {
                    // `38;5;n` and `38;2;r;g;b` take the parameters after them
                    let rest: Vec<u32> = groups[i + 1..].iter().map(|g| g[0]).collect();
                    let used = match rest.first() {
                        Some(5) => 2,
                        Some(2) => 4,
                        _ => 0,
                    };
                    if let Some(color) = extended(&rest[..used.min(rest.len())], false) {
                        if code == 38 {
                            self.fg = Some(color);
                        } else {
                            self.bg = Some(color);
                        }
                    }
                    i += used;
                }
                code => self.code(code),
            }
            i += 1;
        }
    }

    /// One code with no parameters of its own; unknown codes do nothing
    fn code(&mut self, code: u32) {
        let m = &mut self.modifiers;
        match code {
            0 => *self = Self::default(),
            1 => *m |= Modifier::BOLD,
            2 => *m |= Modifier::DIM,
            3 => *m |= Modifier::ITALIC,
            4 | 21 => *m |= Modifier::UNDERLINE,
            7 => *m |= Modifier::REVERSE,
            9 => *m |= Modifier::CROSSED_OUT,
            22 => m.remove(Modifier::BOLD | Modifier::DIM),
            23 => m.remove(Modifier::ITALIC),
            24 => m.remove(Modifier::UNDERLINE),
            27 => m.remove(Modifier::REVERSE),
            29 => m.remove(Modifier::CROSSED_OUT),
            30..=37 => self.fg = color_256(code - 30),
            39 => self.fg = None,
            40..=47 => self.bg = color_256(code - 40),
            49 => self.bg = None,
            90..=97 => self.fg = color_256(code - 90 + 8),
            100..=107 => self.bg = color_256(code - 100 + 8),
            _ => {}
        }
    }
}

/// A parameter's value: its digits (an empty one is 0), saturating
fn number(digits: &str) -> u32 {
    digits
        .bytes()
        .filter(u8::is_ascii_digit)
        .fold(0u32, |n, b| {
            n.saturating_mul(10).saturating_add(u32::from(b - b'0'))
        })
}

/// The color after a 38 or 48: `5, n` or `2, r, g, b`. With colons the RGB
/// form may name a color space first (`2, id, r, g, b`). `None` when the
/// values are missing or out of range: an index past 255 or a component
/// past 255 sets nothing.
fn extended(params: &[u32], colons: bool) -> Option<Color> {
    match params {
        [5, n, ..] => color_256(*n),
        [2, _, r, g, b, ..] if colons => rgb(*r, *g, *b),
        [2, r, g, b, ..] => rgb(*r, *g, *b),
        _ => None,
    }
}

fn rgb(r: u32, g: u32, b: u32) -> Option<Color> {
    Some(Color::rgb(
        u8::try_from(r).ok()?,
        u8::try_from(g).ok()?,
        u8::try_from(b).ok()?,
    ))
}

/// The xterm 256-color palette: codes 0-15 are also the colors of SGR
/// 30-37/40-47 (0-7) and 90-97/100-107 (8-15)
pub(crate) fn color_256(code: u32) -> Option<Color> {
    let code = u8::try_from(code).ok()?;
    Some(match code {
        0 => Color::BLACK,
        1 => Color::RED,
        2 => Color::GREEN,
        3 => Color::YELLOW,
        4 => Color::BLUE,
        5 => Color::MAGENTA,
        6 => Color::CYAN,
        7 => Color::WHITE,
        8 => Color::rgb(128, 128, 128),
        9 => Color::rgb(255, 85, 85),
        10 => Color::rgb(85, 255, 85),
        11 => Color::rgb(255, 255, 85),
        12 => Color::rgb(85, 85, 255),
        13 => Color::rgb(255, 85, 255),
        14 => Color::rgb(85, 255, 255),
        15 => Color::rgb(255, 255, 255),
        // The 6 x 6 x 6 cube
        16..=231 => {
            let n = code - 16;
            let level = |v: u8| if v == 0 { 0 } else { 55 + 40 * v };
            Color::rgb(level(n / 36), level(n / 6 % 6), level(n % 6))
        }
        // The grays
        232..=255 => {
            let gray = (code - 232) * 10 + 8;
            Color::rgb(gray, gray, gray)
        }
    })
}
