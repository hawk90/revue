//! Built-in themes for Revue
//!
//! Pre-configured color schemes for common design systems.

/// Dracula theme colors
///
/// A dark theme with purple accents, inspired by the popular Dracula color scheme.
pub mod dracula {
    use crate::style::Color;

    /// Primary background color (#282a36)
    pub const BG_PRIMARY: Color = Color::rgb(40, 42, 54);
    /// Secondary background color (#44475a)
    pub const BG_SECONDARY: Color = Color::rgb(68, 71, 90);
    /// Primary foreground/text color (#f8f8f2)
    pub const FG_PRIMARY: Color = Color::rgb(248, 248, 242);
    /// Secondary foreground color (#6272a4)
    pub const FG_SECONDARY: Color = Color::rgb(98, 114, 164);
    /// Accent/highlight color - purple (#bd93f9)
    pub const ACCENT: Color = Color::rgb(189, 147, 249);
    /// Success state color - green (#50fa7b)
    pub const SUCCESS: Color = Color::rgb(80, 250, 123);
    /// Warning state color - orange (#ffb86c)
    pub const WARNING: Color = Color::rgb(255, 184, 108);
    /// Error state color - red (#ff5555)
    pub const ERROR: Color = Color::rgb(255, 85, 85);
    /// Cyan accent (#8be9fd)
    pub const CYAN: Color = Color::rgb(139, 233, 253);
    /// Pink accent (#ff79c6)
    pub const PINK: Color = Color::rgb(255, 121, 198);
    /// Yellow accent (#f1fa8c)
    pub const YELLOW: Color = Color::rgb(241, 250, 140);

    /// Get CSS variables for Dracula theme
    pub fn css() -> &'static str {
        include_str!("dracula.css")
    }
}

/// Nord theme colors
///
/// An arctic, north-bluish color palette inspired by the beauty of the arctic.
pub mod nord {
    use crate::style::Color;

    /// Polar Night 0 - darkest background (#2e3440)
    pub const POLAR_NIGHT_0: Color = Color::rgb(46, 52, 64);
    /// Polar Night 1 - dark background (#3b4252)
    pub const POLAR_NIGHT_1: Color = Color::rgb(59, 66, 82);
    /// Polar Night 2 - medium background (#434c5e)
    pub const POLAR_NIGHT_2: Color = Color::rgb(67, 76, 94);
    /// Polar Night 3 - light background (#4c566a)
    pub const POLAR_NIGHT_3: Color = Color::rgb(76, 86, 106);

    /// Snow Storm 0 - dark white (#d8dee9)
    pub const SNOW_STORM_0: Color = Color::rgb(216, 222, 233);
    /// Snow Storm 1 - medium white (#e5e9f0)
    pub const SNOW_STORM_1: Color = Color::rgb(229, 233, 240);
    /// Snow Storm 2 - bright white (#eceff4)
    pub const SNOW_STORM_2: Color = Color::rgb(236, 239, 244);

    /// Frost 0 - frozen polar water (#8fbcbb)
    pub const FROST_0: Color = Color::rgb(143, 188, 187);
    /// Frost 1 - pure ice (#88c0d0)
    pub const FROST_1: Color = Color::rgb(136, 192, 208);
    /// Frost 2 - arctic water (#81a1c1)
    pub const FROST_2: Color = Color::rgb(129, 161, 193);
    /// Frost 3 - deep arctic ocean (#5e81ac)
    pub const FROST_3: Color = Color::rgb(94, 129, 172);

    /// Aurora red (#bf616a)
    pub const AURORA_RED: Color = Color::rgb(191, 97, 106);
    /// Aurora orange (#d08770)
    pub const AURORA_ORANGE: Color = Color::rgb(208, 135, 112);
    /// Aurora yellow (#ebcb8b)
    pub const AURORA_YELLOW: Color = Color::rgb(235, 203, 139);
    /// Aurora green (#a3be8c)
    pub const AURORA_GREEN: Color = Color::rgb(163, 190, 140);
    /// Aurora purple (#b48ead)
    pub const AURORA_PURPLE: Color = Color::rgb(180, 142, 173);

    /// Primary background color (alias for POLAR_NIGHT_0)
    pub const BG_PRIMARY: Color = POLAR_NIGHT_0;
    /// Secondary background color (alias for POLAR_NIGHT_1)
    pub const BG_SECONDARY: Color = POLAR_NIGHT_1;
    /// Primary foreground color (alias for SNOW_STORM_0)
    pub const FG_PRIMARY: Color = SNOW_STORM_0;
    /// Secondary foreground color (alias for POLAR_NIGHT_3)
    pub const FG_SECONDARY: Color = POLAR_NIGHT_3;
    /// Accent color (alias for FROST_1)
    pub const ACCENT: Color = FROST_1;
    /// Success state color (alias for AURORA_GREEN)
    pub const SUCCESS: Color = AURORA_GREEN;
    /// Warning state color (alias for AURORA_YELLOW)
    pub const WARNING: Color = AURORA_YELLOW;
    /// Error state color (alias for AURORA_RED)
    pub const ERROR: Color = AURORA_RED;

    /// Get CSS variables for Nord theme
    pub fn css() -> &'static str {
        include_str!("nord.css")
    }
}

/// Monokai theme colors
///
/// A vibrant theme inspired by the classic Sublime Text color scheme.
pub mod monokai {
    use crate::style::Color;

    /// Primary background color (#272822)
    pub const BG_PRIMARY: Color = Color::rgb(39, 40, 34);
    /// Secondary background color (#3e3d32)
    pub const BG_SECONDARY: Color = Color::rgb(62, 61, 50);
    /// Primary foreground color (#f8f8f2)
    pub const FG_PRIMARY: Color = Color::rgb(248, 248, 242);
    /// Secondary foreground color (#75715e)
    pub const FG_SECONDARY: Color = Color::rgb(117, 113, 94);
    /// Accent color - green (#a6e22e)
    pub const ACCENT: Color = Color::rgb(166, 226, 46);
    /// Pink accent (#f92672)
    pub const PINK: Color = Color::rgb(249, 38, 114);
    /// Orange accent (#fd971f)
    pub const ORANGE: Color = Color::rgb(253, 151, 31);
    /// Yellow accent (#e6db74)
    pub const YELLOW: Color = Color::rgb(230, 219, 116);
    /// Purple accent (#ae81ff)
    pub const PURPLE: Color = Color::rgb(174, 129, 255);
    /// Cyan accent (#66d9ef)
    pub const CYAN: Color = Color::rgb(102, 217, 239);
    /// Success state color (alias for ACCENT)
    pub const SUCCESS: Color = ACCENT;
    /// Warning state color (alias for ORANGE)
    pub const WARNING: Color = ORANGE;
    /// Error state color (alias for PINK)
    pub const ERROR: Color = PINK;

    /// Get CSS variables for Monokai theme
    pub fn css() -> &'static str {
        include_str!("monokai.css")
    }
}

/// Gruvbox theme colors
///
/// A retro groove color scheme with warm earthy tones.
pub mod gruvbox {
    use crate::style::Color;

    /// Hard contrast background (#1d2021)
    pub const BG_HARD: Color = Color::rgb(29, 32, 33);
    /// Default background (#282828)
    pub const BG: Color = Color::rgb(40, 40, 40);
    /// Soft contrast background (#32302f)
    pub const BG_SOFT: Color = Color::rgb(50, 48, 47);
    /// Background 1 (#3c3836)
    pub const BG1: Color = Color::rgb(60, 56, 54);
    /// Background 2 (#504945)
    pub const BG2: Color = Color::rgb(80, 73, 69);
    /// Background 3 (#665c54)
    pub const BG3: Color = Color::rgb(102, 92, 84);
    /// Background 4 (#7c6f64)
    pub const BG4: Color = Color::rgb(124, 111, 100);

    /// Default foreground (#ebdbb2)
    pub const FG: Color = Color::rgb(235, 219, 178);
    /// Foreground 0 - brightest (#fbf1c7)
    pub const FG0: Color = Color::rgb(251, 241, 199);
    /// Foreground 1 (#ebdbb2)
    pub const FG1: Color = Color::rgb(235, 219, 178);
    /// Foreground 2 (#d5c4a1)
    pub const FG2: Color = Color::rgb(213, 196, 161);
    /// Foreground 3 (#bdae93)
    pub const FG3: Color = Color::rgb(189, 174, 147);
    /// Foreground 4 - dimmest (#a89984)
    pub const FG4: Color = Color::rgb(168, 153, 132);

    /// Red accent (#fb4934)
    pub const RED: Color = Color::rgb(251, 73, 52);
    /// Green accent (#b8bb26)
    pub const GREEN: Color = Color::rgb(184, 187, 38);
    /// Yellow accent (#fabd2f)
    pub const YELLOW: Color = Color::rgb(250, 189, 47);
    /// Blue accent (#83a598)
    pub const BLUE: Color = Color::rgb(131, 165, 152);
    /// Purple accent (#d3869b)
    pub const PURPLE: Color = Color::rgb(211, 134, 155);
    /// Aqua accent (#8ec07c)
    pub const AQUA: Color = Color::rgb(142, 192, 124);
    /// Orange accent (#fe8019)
    pub const ORANGE: Color = Color::rgb(254, 128, 25);

    /// Primary background color (alias for BG)
    pub const BG_PRIMARY: Color = BG;
    /// Secondary background color (alias for BG1)
    pub const BG_SECONDARY: Color = BG1;
    /// Primary foreground color (alias for FG)
    pub const FG_PRIMARY: Color = FG;
    /// Secondary foreground color (alias for FG4)
    pub const FG_SECONDARY: Color = FG4;
    /// Accent color (alias for YELLOW)
    pub const ACCENT: Color = YELLOW;
    /// Success state color (alias for GREEN)
    pub const SUCCESS: Color = GREEN;
    /// Warning state color (alias for ORANGE)
    pub const WARNING: Color = ORANGE;
    /// Error state color (alias for RED)
    pub const ERROR: Color = RED;

    /// Get CSS variables for Gruvbox theme
    pub fn css() -> &'static str {
        include_str!("gruvbox.css")
    }
}

/// High Contrast Dark theme colors
///
/// WCAG AAA compliant dark theme with maximum contrast for accessibility.
pub mod high_contrast_dark {
    use crate::style::Color;

    /// Primary background color (pure black)
    pub const BG_PRIMARY: Color = Color::rgb(0, 0, 0);
    /// Secondary background color (very dark gray)
    pub const BG_SECONDARY: Color = Color::rgb(26, 26, 26);
    /// Primary foreground color (pure white)
    pub const FG_PRIMARY: Color = Color::rgb(255, 255, 255);
    /// Secondary foreground color (light gray)
    pub const FG_SECONDARY: Color = Color::rgb(204, 204, 204);
    /// Accent color (bright yellow for high visibility)
    pub const ACCENT: Color = Color::rgb(255, 255, 0);
    /// Success color (bright green)
    pub const SUCCESS: Color = Color::rgb(0, 255, 0);
    /// Warning color (bright orange)
    pub const WARNING: Color = Color::rgb(255, 170, 0);
    /// Error color (bright red)
    pub const ERROR: Color = Color::rgb(255, 51, 51);
    /// Focus indicator color (cyan)
    pub const FOCUS: Color = Color::rgb(0, 255, 255);
    /// Link color (light blue)
    pub const LINK: Color = Color::rgb(102, 204, 255);
    /// Border color (white for visibility)
    pub const BORDER: Color = Color::rgb(255, 255, 255);

    /// Get CSS variables for High Contrast Dark theme
    pub fn css() -> &'static str {
        include_str!("high_contrast_dark.css")
    }
}

/// High Contrast Light theme colors
///
/// WCAG AAA compliant light theme with maximum contrast for accessibility.
pub mod high_contrast_light {
    use crate::style::Color;

    /// Primary background color (pure white)
    pub const BG_PRIMARY: Color = Color::rgb(255, 255, 255);
    /// Secondary background color (very light gray)
    pub const BG_SECONDARY: Color = Color::rgb(240, 240, 240);
    /// Primary foreground color (pure black)
    pub const FG_PRIMARY: Color = Color::rgb(0, 0, 0);
    /// Secondary foreground color (dark gray)
    pub const FG_SECONDARY: Color = Color::rgb(51, 51, 51);
    /// Accent color (dark blue)
    pub const ACCENT: Color = Color::rgb(0, 0, 204);
    /// Success color (dark green)
    pub const SUCCESS: Color = Color::rgb(0, 102, 0);
    /// Warning color (dark orange)
    pub const WARNING: Color = Color::rgb(204, 102, 0);
    /// Error color (dark red)
    pub const ERROR: Color = Color::rgb(204, 0, 0);
    /// Focus indicator color (medium blue)
    pub const FOCUS: Color = Color::rgb(0, 102, 204);
    /// Link color (traditional link blue)
    pub const LINK: Color = Color::rgb(0, 0, 238);
    /// Border color (black for visibility)
    pub const BORDER: Color = Color::rgb(0, 0, 0);

    /// Get CSS variables for High Contrast Light theme
    pub fn css() -> &'static str {
        include_str!("high_contrast_light.css")
    }
}

/// Catppuccin Mocha theme colors
///
/// A soothing pastel theme with warm, cozy colors.
pub mod catppuccin {
    use crate::style::Color;

    /// Rosewater accent (#f5e0dc)
    pub const ROSEWATER: Color = Color::rgb(245, 224, 220);
    /// Flamingo accent (#f2cdcd)
    pub const FLAMINGO: Color = Color::rgb(242, 205, 205);
    /// Pink accent (#f5c2e7)
    pub const PINK: Color = Color::rgb(245, 194, 231);
    /// Mauve accent (#cba6f7)
    pub const MAUVE: Color = Color::rgb(203, 166, 247);
    /// Red accent (#f38ba8)
    pub const RED: Color = Color::rgb(243, 139, 168);
    /// Maroon accent (#eba0ac)
    pub const MAROON: Color = Color::rgb(235, 160, 172);
    /// Peach accent (#fab387)
    pub const PEACH: Color = Color::rgb(250, 179, 135);
    /// Yellow accent (#f9e2af)
    pub const YELLOW: Color = Color::rgb(249, 226, 175);
    /// Green accent (#a6e3a1)
    pub const GREEN: Color = Color::rgb(166, 227, 161);
    /// Teal accent (#94e2d5)
    pub const TEAL: Color = Color::rgb(148, 226, 213);
    /// Sky accent (#89dceb)
    pub const SKY: Color = Color::rgb(137, 220, 235);
    /// Sapphire accent (#74c7ec)
    pub const SAPPHIRE: Color = Color::rgb(116, 199, 236);
    /// Blue accent (#89b4fa)
    pub const BLUE: Color = Color::rgb(137, 180, 250);
    /// Lavender accent (#b4befe)
    pub const LAVENDER: Color = Color::rgb(180, 190, 254);

    /// Primary text color (#cdd6f4)
    pub const TEXT: Color = Color::rgb(205, 214, 244);
    /// Subtext 1 (#bac2de)
    pub const SUBTEXT1: Color = Color::rgb(186, 194, 222);
    /// Subtext 0 (#a6adc8)
    pub const SUBTEXT0: Color = Color::rgb(166, 173, 200);
    /// Overlay 2 (#9399b2)
    pub const OVERLAY2: Color = Color::rgb(147, 153, 178);
    /// Overlay 1 (#7f849c)
    pub const OVERLAY1: Color = Color::rgb(127, 132, 156);
    /// Overlay 0 (#6c7086)
    pub const OVERLAY0: Color = Color::rgb(108, 112, 134);
    /// Surface 2 (#585b70)
    pub const SURFACE2: Color = Color::rgb(88, 91, 112);
    /// Surface 1 (#45475a)
    pub const SURFACE1: Color = Color::rgb(69, 71, 90);
    /// Surface 0 (#313244)
    pub const SURFACE0: Color = Color::rgb(49, 50, 68);
    /// Base background (#1e1e2e)
    pub const BASE: Color = Color::rgb(30, 30, 46);
    /// Mantle background (#181825)
    pub const MANTLE: Color = Color::rgb(24, 24, 37);
    /// Crust background (#11111b)
    pub const CRUST: Color = Color::rgb(17, 17, 27);

    /// Primary background color (alias for BASE)
    pub const BG_PRIMARY: Color = BASE;
    /// Secondary background color (alias for SURFACE0)
    pub const BG_SECONDARY: Color = SURFACE0;
    /// Primary foreground color (alias for TEXT)
    pub const FG_PRIMARY: Color = TEXT;
    /// Secondary foreground color (alias for OVERLAY0)
    pub const FG_SECONDARY: Color = OVERLAY0;
    /// Accent color (alias for MAUVE)
    pub const ACCENT: Color = MAUVE;
    /// Success state color (alias for GREEN)
    pub const SUCCESS: Color = GREEN;
    /// Warning state color (alias for YELLOW)
    pub const WARNING: Color = YELLOW;
    /// Error state color (alias for RED)
    pub const ERROR: Color = RED;

    /// Get CSS variables for Catppuccin theme
    pub fn css() -> &'static str {
        include_str!("catppuccin.css")
    }
}

/// Available built-in themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinTheme {
    /// Dracula - Dark purple theme
    Dracula,
    /// Nord - Arctic blue theme
    Nord,
    /// Monokai - Sublime Text inspired
    Monokai,
    /// Gruvbox - Retro groove
    Gruvbox,
    /// Catppuccin - Soothing pastels
    Catppuccin,
    /// High Contrast Dark - WCAG AAA accessibility theme
    HighContrastDark,
    /// High Contrast Light - WCAG AAA accessibility theme
    HighContrastLight,
}

impl BuiltinTheme {
    /// Get CSS for this theme
    pub fn css(&self) -> &'static str {
        match self {
            BuiltinTheme::Dracula => dracula::css(),
            BuiltinTheme::Nord => nord::css(),
            BuiltinTheme::Monokai => monokai::css(),
            BuiltinTheme::Gruvbox => gruvbox::css(),
            BuiltinTheme::Catppuccin => catppuccin::css(),
            BuiltinTheme::HighContrastDark => high_contrast_dark::css(),
            BuiltinTheme::HighContrastLight => high_contrast_light::css(),
        }
    }

    /// Get theme name
    pub fn name(&self) -> &'static str {
        match self {
            BuiltinTheme::Dracula => "dracula",
            BuiltinTheme::Nord => "nord",
            BuiltinTheme::Monokai => "monokai",
            BuiltinTheme::Gruvbox => "gruvbox",
            BuiltinTheme::Catppuccin => "catppuccin",
            BuiltinTheme::HighContrastDark => "high-contrast-dark",
            BuiltinTheme::HighContrastLight => "high-contrast-light",
        }
    }

    /// List all available themes
    pub fn all() -> &'static [BuiltinTheme] {
        &[
            BuiltinTheme::Dracula,
            BuiltinTheme::Nord,
            BuiltinTheme::Monokai,
            BuiltinTheme::Gruvbox,
            BuiltinTheme::Catppuccin,
            BuiltinTheme::HighContrastDark,
            BuiltinTheme::HighContrastLight,
        ]
    }

    /// List accessibility-focused themes
    pub fn accessibility() -> &'static [BuiltinTheme] {
        &[
            BuiltinTheme::HighContrastDark,
            BuiltinTheme::HighContrastLight,
        ]
    }

    /// Check if this theme is an accessibility theme
    pub fn is_accessibility(&self) -> bool {
        matches!(self, BuiltinTheme::HighContrastDark | BuiltinTheme::HighContrastLight)
    }

    /// Check if this is a dark theme
    pub fn is_dark(&self) -> bool {
        matches!(
            self,
            BuiltinTheme::Dracula
                | BuiltinTheme::Nord
                | BuiltinTheme::Monokai
                | BuiltinTheme::Gruvbox
                | BuiltinTheme::Catppuccin
                | BuiltinTheme::HighContrastDark
        )
    }
}
