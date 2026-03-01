//! Media widget demos (Image, Gradient, Sparkline)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{waveline, Divider, GradientBox, Sparkline};

pub fn examples() -> Vec<Example> {
    let (primary, success, warning, error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "QR Code",
            "QR code generation for URLs, text, and contact info",
            Border::rounded().title(" QR Code ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Scan this code:").fg(primary))
                    .child(Text::new("[QR Code - enable 'qrcode' feature]").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("QR Code Generator").fg(muted))
                    .child(Text::new("• URL encoding").fg(muted))
                    .child(Text::new("• Text encoding").fg(muted))
                    .child(Text::new("• Contact info").fg(muted)),
            ),
        ),
        Example::new(
            "Image Display",
            "ASCII art and terminal graphics rendering",
            Border::rounded().title(" Image Display ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("ASCII art images:").fg(primary))
                    .child(
                        vstack()
                            .gap(1)
                            .child(Text::new("  +-------+  ").fg(primary))
                            .child(Text::new("  | O   O |  ").fg(primary))
                            .child(Text::new("  |   ^   |  ").fg(primary))
                            .child(Text::new("  |  ---  |  ").fg(primary))
                            .child(Text::new("  +-------+  ").fg(primary)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Terminal graphics").fg(muted))
                    .child(Text::new("• ASCII/Unicode art").fg(muted))
                    .child(Text::new("• Block characters").fg(muted)),
            ),
        ),
        Example::new(
            "Gradients",
            "Color gradients and gradient box rendering",
            Border::rounded().title(" Gradients ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Color gradients:").fg(primary))
                    .child(
                        vstack()
                            .gap(0)
                            .child(Text::new("░▒▓█ Full █▓▒░").fg(primary))
                            .child(Text::new("░▒▓█ Block █▓▒░").fg(success))
                            .child(Text::new("░▒▓█ Color █▓▒░").fg(warning))
                            .child(Text::new("░▒▓█ Gradient █▓▒░").fg(error))
                            .child(Text::new("░▒▓█ Demo █▓▒░").fg(info)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Gradient box:").fg(primary))
                    .child(GradientBox::horizontal(
                        Color::rgb(220, 50, 50),
                        Color::rgb(50, 100, 220),
                        30,
                        1,
                    ))
                    .child(Text::new(""))
                    .child(Text::new("• Visual appeal").fg(muted))
                    .child(Text::new("• Block characters").fg(muted))
                    .child(Text::new("• Color transitions").fg(muted)),
            ),
        ),
        Example::new(
            "Charts Mini",
            "Sparkline and waveline mini visualizations",
            Border::rounded().title(" Charts Mini ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Sparkline:").fg(primary))
                    .child(Sparkline::new(vec![
                        10.0, 25.0, 18.0, 30.0, 22.0, 45.0, 38.0, 55.0,
                    ]))
                    .child(Text::new(""))
                    .child(Text::new("Waveline:").fg(primary))
                    .child(waveline(vec![0.5, 0.7, 0.3, 0.9, 0.4, 0.8, 0.2, 0.6]).color(primary))
                    .child(Text::new(""))
                    .child(Text::new("• Mini visualizations").fg(muted))
                    .child(Text::new("• Inline data display").fg(muted))
                    .child(Text::new("• Trend indicators").fg(muted)),
            ),
        ),
        Example::new(
            "Decorations",
            "Horizontal rules and section dividers",
            Border::rounded().title(" Decorations ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Horizontal rule:").fg(primary))
                    .child(Divider::new().label(" Section "))
                    .child(Text::new(""))
                    .child(Text::new("Dotted line:").fg(primary))
                    .child(Divider::new().dotted())
                    .child(Text::new(""))
                    .child(Text::new("Dashed line:").fg(primary))
                    .child(Divider::new().dashed())
                    .child(Text::new(""))
                    .child(Text::new("• Section dividers").fg(muted))
                    .child(Text::new("• Visual separation").fg(muted))
                    .child(Text::new("• Text labels").fg(muted)),
            ),
        ),
        Example::new(
            "Symbols",
            "Unicode symbols and special characters",
            Border::rounded().title(" Symbols ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Check marks: ✓ ✔ ✅ ☑").fg(text))
                    .child(Text::new("Cross marks: ✕ ✗ ✖ ❌ ☒").fg(text))
                    .child(Text::new("Arrows: ← → ↑ ↓ ↔ ↕ ↖ ↗").fg(text))
                    .child(Text::new("Stars: ★ ☆ ✦ ✧ ⚝ ✩").fg(text))
                    .child(Text::new("Hearts: ♥ ♡ ❤ 💙 💚").fg(text))
                    .child(Text::new("Weather: ☀ ☁ ☂ ☃ ☄").fg(text))
                    .child(Text::new("Music: ♩ ♪ ♫ ♬ ♭ ♮ ♯").fg(text))
                    .child(Text::new("Cards: ♠ ♣ ♥ ♦").fg(text)),
            ),
        ),
    ]
}
