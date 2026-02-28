//! Media widget demos (Image, Gradient, Sparkline)

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{waveline, Divider, GradientBox, Sparkline};

pub fn render() -> impl View {
    let (primary, success, warning, error, info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
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
                )
                .child(
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
                )
                .child(
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
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Charts Mini ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Sparkline:").fg(primary))
                            .child(Sparkline::new(vec![
                                10.0, 25.0, 18.0, 30.0, 22.0, 45.0, 38.0, 55.0,
                            ]))
                            .child(Text::new(""))
                            .child(Text::new("Waveline:").fg(primary))
                            .child(
                                waveline(vec![0.5, 0.7, 0.3, 0.9, 0.4, 0.8, 0.2, 0.6])
                                    .color(primary),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Mini visualizations").fg(muted))
                            .child(Text::new("• Inline data display").fg(muted))
                            .child(Text::new("• Trend indicators").fg(muted)),
                    ),
                )
                .child(
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
                )
                .child(
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
        )
}
