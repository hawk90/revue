//! Drawing the avatar: initials, derived background color, shapes and status dot

use super::{Avatar, AvatarShape, AvatarSize};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl Avatar {
    /// Get initials from name
    fn get_initials(&self) -> String {
        if let Some(ref initials) = self.initials {
            return initials.clone();
        }

        if let Some(icon) = self.icon {
            return icon.to_string();
        }

        // Derive initials from name
        self.name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }

    /// Get background color (auto-generate from name if not set)
    fn get_bg_color(&self) -> Color {
        if let Some(color) = self.bg_color {
            return color;
        }

        // Generate color from name hash
        let hash: u32 = self
            .name
            .bytes()
            .fold(0u32, |acc, b| acc.wrapping_add(b as u32));
        let hue = (hash % 360) as u8;

        // Convert HSL to RGB (simplified)
        let h = hue as f32 / 60.0;
        let s = 0.6_f32;
        let l = 0.4_f32;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r1, g1, b1) = match h as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Color::rgb(
            ((r1 + m) * 255.0) as u8,
            ((g1 + m) * 255.0) as u8,
            ((b1 + m) * 255.0) as u8,
        )
    }
}

impl View for Avatar {
    crate::impl_view_meta!("Avatar");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let initials = self.get_initials();
        let bg = self.get_bg_color();
        // Builder, then the stylesheet, then the widget's own default.
        let fg = self.fg_color.unwrap_or_else(|| ctx.css_color(Color::WHITE));

        match self.size {
            AvatarSize::Small => {
                // Single character
                let ch = initials.chars().next().unwrap_or('?');
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                cell.modifier |= Modifier::BOLD;
                ctx.set(0, 0, cell);

                // Status dot
                if let Some(status_color) = self.status {
                    let mut dot = Cell::new('●');
                    dot.fg = Some(status_color);
                    ctx.set(1, 0, dot);
                }
            }
            AvatarSize::Medium => {
                // 3 chars wide: [XY] or ⬤XY⬤ for circle
                match self.shape {
                    AvatarShape::Circle => {
                        // Use half-blocks for pseudo-circle: ◖XY◗
                        let mut left = Cell::new('◖');
                        left.fg = Some(bg);
                        ctx.set(0, 0, left);

                        for (i, ch) in initials.chars().take(2).enumerate() {
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.set(1 + i as u16, 0, cell);
                        }

                        let mut right = Cell::new('◗');
                        right.fg = Some(bg);
                        ctx.set(3, 0, right);

                        // Status dot
                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.set(4, 0, dot);
                        }
                    }
                    AvatarShape::Square | AvatarShape::Rounded => {
                        // [XY] format
                        let left = if self.shape == AvatarShape::Rounded {
                            '('
                        } else {
                            '['
                        };
                        let right = if self.shape == AvatarShape::Rounded {
                            ')'
                        } else {
                            ']'
                        };

                        let mut lc = Cell::new(left);
                        lc.fg = Some(bg);
                        ctx.set(0, 0, lc);

                        for (i, ch) in initials.chars().take(2).enumerate() {
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.set(1 + i as u16, 0, cell);
                        }

                        let mut rc = Cell::new(right);
                        rc.fg = Some(bg);
                        ctx.set(3, 0, rc);

                        // Status dot
                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.set(4, 0, dot);
                        }
                    }
                }
            }
            AvatarSize::Large => {
                // 3 lines tall, 5+ chars wide
                if area.height < 3 {
                    // Fall back to medium
                    let mut cell = Cell::new(initials.chars().next().unwrap_or('?'));
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    ctx.set(0, 0, cell);
                    return;
                }

                match self.shape {
                    AvatarShape::Circle => {
                        // Top: ╭───╮
                        // Mid: │XY │
                        // Bot: ╰───╯
                        let chars_top = ['╭', '─', '─', '─', '╮'];
                        let chars_bot = ['╰', '─', '─', '─', '╯'];

                        for (i, ch) in chars_top.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.set(i as u16, 0, cell);
                        }

                        // Middle row
                        let mut left = Cell::new('│');
                        left.fg = Some(bg);
                        ctx.set(0, 1, left);

                        // Pre-collect initials chars for O(1) access
                        let initials_chars: Vec<char> = initials.chars().collect();
                        for i in 1..4 {
                            let ch = if i == 1 || i == 2 {
                                initials_chars.get(i - 1).copied().unwrap_or(' ')
                            } else {
                                ' '
                            };
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.set(i as u16, 1, cell);
                        }

                        let mut right = Cell::new('│');
                        right.fg = Some(bg);
                        ctx.set(4, 1, right);

                        for (i, ch) in chars_bot.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.set(i as u16, 2, cell);
                        }

                        // Status dot
                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.set(5, 2, dot);
                        }
                    }
                    AvatarShape::Square => {
                        // Top: ┌───┐
                        let chars_top = ['┌', '─', '─', '─', '┐'];
                        let chars_bot = ['└', '─', '─', '─', '┘'];

                        for (i, ch) in chars_top.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.set(i as u16, 0, cell);
                        }

                        let mut left = Cell::new('│');
                        left.fg = Some(bg);
                        ctx.set(0, 1, left);

                        // Pre-collect initials chars for O(1) access
                        let initials_chars: Vec<char> = initials.chars().collect();
                        for i in 1..4 {
                            let ch = if i == 1 || i == 2 {
                                initials_chars.get(i - 1).copied().unwrap_or(' ')
                            } else {
                                ' '
                            };
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.set(i as u16, 1, cell);
                        }

                        let mut right = Cell::new('│');
                        right.fg = Some(bg);
                        ctx.set(4, 1, right);

                        for (i, ch) in chars_bot.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.set(i as u16, 2, cell);
                        }

                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.set(5, 2, dot);
                        }
                    }
                    AvatarShape::Rounded => {
                        // Same as circle for large
                        let chars_top = ['╭', '─', '─', '─', '╮'];
                        let chars_bot = ['╰', '─', '─', '─', '╯'];

                        for (i, ch) in chars_top.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.set(i as u16, 0, cell);
                        }

                        let mut left = Cell::new('│');
                        left.fg = Some(bg);
                        ctx.set(0, 1, left);

                        // Pre-collect initials chars for O(1) access
                        let initials_chars: Vec<char> = initials.chars().collect();
                        for i in 1..4 {
                            let ch = if i == 1 || i == 2 {
                                initials_chars.get(i - 1).copied().unwrap_or(' ')
                            } else {
                                ' '
                            };
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.set(i as u16, 1, cell);
                        }

                        let mut right = Cell::new('│');
                        right.fg = Some(bg);
                        ctx.set(4, 1, right);

                        for (i, ch) in chars_bot.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.set(i as u16, 2, cell);
                        }

                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.set(5, 2, dot);
                        }
                    }
                }
            }
        }
    }
}
