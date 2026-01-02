//! Revue Theme Builder - Interactive theme creation tool
//!
//! A TUI application for creating and previewing Revue themes.

use revue::prelude::*;
use revue::style::Color;
use std::fs;

fn main() -> revue::Result<()> {
    App::builder()
        .title("Revue Theme Builder")
        .view(ThemeBuilder::new())
        .run()
}

/// Theme color configuration
#[derive(Clone, Debug)]
struct ThemeColors {
    bg_primary: Color,
    bg_secondary: Color,
    fg_primary: Color,
    fg_secondary: Color,
    accent: Color,
    success: Color,
    warning: Color,
    error: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_primary: Color::rgb(26, 27, 38),
            bg_secondary: Color::rgb(36, 40, 59),
            fg_primary: Color::rgb(192, 202, 245),
            fg_secondary: Color::rgb(86, 95, 137),
            accent: Color::rgb(122, 162, 247),
            success: Color::rgb(158, 206, 106),
            warning: Color::rgb(224, 175, 104),
            error: Color::rgb(247, 118, 142),
        }
    }
}

impl ThemeColors {
    fn to_css(&self) -> String {
        format!(
            r#":root {{
    --bg-primary: {};
    --bg-secondary: {};
    --fg-primary: {};
    --fg-secondary: {};
    --accent: {};
    --success: {};
    --warning: {};
    --error: {};
}}

/* Button styles */
.button {{
    padding: 0 2;
    border: solid var(--accent);
    color: var(--accent);
}}

.button:hover {{
    background: var(--accent);
    color: var(--bg-primary);
}}

.button:focus {{
    border: double var(--accent);
}}

/* Input styles */
.input {{
    border: solid var(--fg-secondary);
    padding: 0 1;
}}

.input:focus {{
    border: solid var(--accent);
}}

/* Panel styles */
.panel {{
    border: rounded var(--fg-secondary);
    background: var(--bg-secondary);
}}

/* List styles */
.list-item {{
    padding: 0 1;
}}

.list-item:selected {{
    background: var(--accent);
    color: var(--bg-primary);
}}

/* Progress styles */
.progress {{
    color: var(--accent);
}}

/* Toast styles */
.toast-info {{
    border: solid var(--accent);
    background: var(--bg-secondary);
}}

.toast-success {{
    border: solid var(--success);
    background: var(--bg-secondary);
}}

.toast-warning {{
    border: solid var(--warning);
    background: var(--bg-secondary);
}}

.toast-error {{
    border: solid var(--error);
    background: var(--bg-secondary);
}}
"#,
            color_to_hex(&self.bg_primary),
            color_to_hex(&self.bg_secondary),
            color_to_hex(&self.fg_primary),
            color_to_hex(&self.fg_secondary),
            color_to_hex(&self.accent),
            color_to_hex(&self.success),
            color_to_hex(&self.warning),
            color_to_hex(&self.error),
        )
    }
}

fn color_to_hex(color: &Color) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
}

/// Theme Builder application
struct ThemeBuilder {
    colors: ThemeColors,
    selected_color: usize,
    theme_name: String,
    saved: bool,
}

impl ThemeBuilder {
    fn new() -> Self {
        Self {
            colors: ThemeColors::default(),
            selected_color: 0,
            theme_name: "my-theme".to_string(),
            saved: false,
        }
    }

    fn color_names() -> &'static [&'static str] {
        &[
            "Background Primary",
            "Background Secondary",
            "Foreground Primary",
            "Foreground Secondary",
            "Accent",
            "Success",
            "Warning",
            "Error",
        ]
    }

    fn get_color(&self, idx: usize) -> &Color {
        match idx {
            0 => &self.colors.bg_primary,
            1 => &self.colors.bg_secondary,
            2 => &self.colors.fg_primary,
            3 => &self.colors.fg_secondary,
            4 => &self.colors.accent,
            5 => &self.colors.success,
            6 => &self.colors.warning,
            7 => &self.colors.error,
            _ => &self.colors.accent,
        }
    }

    fn set_color(&mut self, idx: usize, color: Color) {
        match idx {
            0 => self.colors.bg_primary = color,
            1 => self.colors.bg_secondary = color,
            2 => self.colors.fg_primary = color,
            3 => self.colors.fg_secondary = color,
            4 => self.colors.accent = color,
            5 => self.colors.success = color,
            6 => self.colors.warning = color,
            7 => self.colors.error = color,
            _ => {}
        }
        self.saved = false;
    }

    fn save_theme(&mut self) {
        let css = self.colors.to_css();
        let filename = format!("{}.css", self.theme_name);
        if fs::write(&filename, &css).is_ok() {
            self.saved = true;
        }
    }

    fn load_preset(&mut self, preset: &str) {
        self.colors = match preset {
            "tokyo-night" => ThemeColors {
                bg_primary: Color::rgb(26, 27, 38),
                bg_secondary: Color::rgb(36, 40, 59),
                fg_primary: Color::rgb(192, 202, 245),
                fg_secondary: Color::rgb(86, 95, 137),
                accent: Color::rgb(122, 162, 247),
                success: Color::rgb(158, 206, 106),
                warning: Color::rgb(224, 175, 104),
                error: Color::rgb(247, 118, 142),
            },
            "dracula" => ThemeColors {
                bg_primary: Color::rgb(40, 42, 54),
                bg_secondary: Color::rgb(68, 71, 90),
                fg_primary: Color::rgb(248, 248, 242),
                fg_secondary: Color::rgb(98, 114, 164),
                accent: Color::rgb(189, 147, 249),
                success: Color::rgb(80, 250, 123),
                warning: Color::rgb(255, 184, 108),
                error: Color::rgb(255, 85, 85),
            },
            "nord" => ThemeColors {
                bg_primary: Color::rgb(46, 52, 64),
                bg_secondary: Color::rgb(59, 66, 82),
                fg_primary: Color::rgb(236, 239, 244),
                fg_secondary: Color::rgb(76, 86, 106),
                accent: Color::rgb(136, 192, 208),
                success: Color::rgb(163, 190, 140),
                warning: Color::rgb(235, 203, 139),
                error: Color::rgb(191, 97, 106),
            },
            "gruvbox" => ThemeColors {
                bg_primary: Color::rgb(40, 40, 40),
                bg_secondary: Color::rgb(60, 56, 54),
                fg_primary: Color::rgb(235, 219, 178),
                fg_secondary: Color::rgb(168, 153, 132),
                accent: Color::rgb(250, 189, 47),
                success: Color::rgb(184, 187, 38),
                warning: Color::rgb(254, 128, 25),
                error: Color::rgb(251, 73, 52),
            },
            "catppuccin" => ThemeColors {
                bg_primary: Color::rgb(30, 30, 46),
                bg_secondary: Color::rgb(49, 50, 68),
                fg_primary: Color::rgb(205, 214, 244),
                fg_secondary: Color::rgb(147, 153, 178),
                accent: Color::rgb(203, 166, 247),
                success: Color::rgb(166, 227, 161),
                warning: Color::rgb(249, 226, 175),
                error: Color::rgb(243, 139, 168),
            },
            _ => ThemeColors::default(),
        };
        self.saved = false;
    }
}

impl View for ThemeBuilder {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area();

        // Main layout
        hstack()
            .child(
                // Left panel: Color editor
                Border::rounded()
                    .title("Colors")
                    .width(30)
                    .child(self.render_color_list())
            )
            .child(
                // Right panel: Preview
                vstack()
                    .child(
                        Border::rounded()
                            .title("Preview")
                            .child(self.render_preview())
                    )
                    .child(
                        Border::rounded()
                            .title("Presets")
                            .height(5)
                            .child(self.render_presets())
                    )
                    .child(
                        Border::rounded()
                            .title("Export")
                            .height(5)
                            .child(self.render_export())
                    )
            )
            .render(ctx);
    }
}

impl ThemeBuilder {
    fn render_color_list(&self) -> impl View {
        let names = Self::color_names();

        vstack().gap(0).children(
            names.iter().enumerate().map(|(i, name)| {
                let color = self.get_color(i);
                let is_selected = i == self.selected_color;

                hstack()
                    .child(
                        Text::new(if is_selected { "▶ " } else { "  " })
                            .fg(self.colors.accent)
                    )
                    .child(
                        Text::new(format!("{:20}", name))
                            .fg(if is_selected { self.colors.accent } else { self.colors.fg_primary })
                    )
                    .child(
                        Text::new("██")
                            .fg(*color)
                    )
                    .child(
                        Text::new(format!(" {}", color_to_hex(color)))
                            .fg(self.colors.fg_secondary)
                    )
            }).collect::<Vec<_>>()
        )
    }

    fn render_preview(&self) -> impl View {
        vstack()
            .gap(1)
            .child(
                Text::new("Widget Preview")
                    .fg(self.colors.fg_primary)
                    .bold()
            )
            .child(
                hstack()
                    .gap(2)
                    .child(
                        Text::new(" Button ")
                            .fg(self.colors.accent)
                    )
                    .child(
                        Text::new(" Hover ")
                            .fg(self.colors.bg_primary)
                            .bg(self.colors.accent)
                    )
            )
            .child(
                hstack()
                    .gap(1)
                    .child(Text::new("Success:").fg(self.colors.success))
                    .child(Text::new("Warning:").fg(self.colors.warning))
                    .child(Text::new("Error:").fg(self.colors.error))
            )
            .child(
                progress(0.65)
                    .filled_color(self.colors.accent)
            )
            .child(
                Text::new("Secondary text here")
                    .fg(self.colors.fg_secondary)
            )
    }

    fn render_presets(&self) -> impl View {
        hstack()
            .gap(2)
            .child(Text::new("Tokyo Night").fg(self.colors.fg_secondary))
            .child(Text::new("Dracula").fg(self.colors.fg_secondary))
            .child(Text::new("Nord").fg(self.colors.fg_secondary))
            .child(Text::new("Gruvbox").fg(self.colors.fg_secondary))
            .child(Text::new("Catppuccin").fg(self.colors.fg_secondary))
    }

    fn render_export(&self) -> impl View {
        hstack()
            .gap(2)
            .child(
                Text::new(format!("Theme: {}.css", self.theme_name))
                    .fg(self.colors.fg_primary)
            )
            .child(
                if self.saved {
                    Text::new("✓ Saved").fg(self.colors.success)
                } else {
                    Text::new("Press S to save").fg(self.colors.fg_secondary)
                }
            )
    }
}
