//! Sparkline widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{waveline, Sparkline};

pub fn examples(net_in: &[f64], net_out: &[f64]) -> Vec<Example> {
    let (primary, success, warning, _error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "Sparkline",
            "Inline mini chart for real-time network data",
            Border::rounded().title(" Sparkline ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Network IN:").fg(primary))
                    .child(Sparkline::new(net_in.to_vec()).fg(success))
                    .child(Text::new(""))
                    .child(Text::new("Network OUT:").fg(primary))
                    .child(Sparkline::new(net_out.to_vec()).fg(info))
                    .child(Text::new(""))
                    .child(Text::new("(Real-time animated data)").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• Inline mini chart").fg(muted))
                    .child(Text::new("• Trend at a glance").fg(muted))
                    .child(Text::new("• Compact display").fg(muted)),
            ),
        ),
        Example::new(
            "Waveline",
            "Continuous wave visualization for audio and signals",
            Border::rounded().title(" Waveline ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Audio waveform:").fg(primary))
                    .child(
                        waveline(
                            (0..40)
                                .map(|i| {
                                    let x = i as f64 * 0.15;
                                    (x.sin() + (x * 1.5).sin() * 0.5) * 0.5 + 0.5
                                })
                                .collect::<Vec<_>>(),
                        )
                        .color(warning),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Signal:").fg(primary))
                    .child(
                        waveline(
                            (0..30)
                                .map(|i| (i as f64 * 0.3).sin() * 0.5 + 0.5)
                                .collect::<Vec<_>>(),
                        )
                        .color(primary),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Continuous wave").fg(muted))
                    .child(Text::new("• Audio visualization").fg(muted))
                    .child(Text::new("• Smooth curves").fg(muted)),
            ),
        ),
        Example::new(
            "Bar Sparkline",
            "Compact bar-style sparkline for discrete values",
            Border::rounded().title(" Bar Sparkline ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Daily sales:").fg(primary))
                    .child(
                        Sparkline::new(vec![
                            12.0, 18.0, 15.0, 22.0, 28.0, 19.0, 25.0, 30.0, 24.0, 20.0, 35.0, 28.0,
                        ])
                        .fg(success),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Weekly traffic:").fg(primary))
                    .child(Sparkline::new(vec![45.0, 52.0, 48.0, 60.0, 55.0, 70.0, 68.0]).fg(info))
                    .child(Text::new(""))
                    .child(Text::new("• Vertical bar style").fg(muted))
                    .child(Text::new("• Discrete values").fg(muted))
                    .child(Text::new("• Compact bar chart").fg(muted)),
            ),
        ),
        Example::new(
            "Color Gradients",
            "Value-based color gradient sparklines",
            Border::rounded().title(" Color Gradients ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Temperature:").fg(primary))
                    .child(
                        Sparkline::new(vec![
                            15.0, 18.0, 22.0, 25.0, 30.0, 35.0, 32.0, 28.0, 24.0, 20.0,
                        ])
                        .fg(info),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Performance:").fg(primary))
                    .child(
                        Sparkline::new(vec![60.0, 75.0, 85.0, 70.0, 90.0, 95.0, 88.0, 92.0])
                            .fg(success),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Color gradient").fg(muted))
                    .child(Text::new("• Value-based coloring").fg(muted))
                    .child(Text::new("• Visual feedback").fg(muted)),
            ),
        ),
        Example::new(
            "Inline Metrics",
            "Dashboard-style inline sparklines with labels",
            Border::rounded().title(" Inline Metrics ").child(
                vstack()
                    .gap(1)
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::new("CPU:").fg(muted))
                            .child(Sparkline::new(vec![40.0, 45.0, 50.0, 48.0, 52.0]).fg(success))
                            .child(Text::new("52%").fg(text)),
                    )
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::new("MEM:").fg(muted))
                            .child(Sparkline::new(vec![60.0, 65.0, 68.0, 70.0, 67.0]).fg(info))
                            .child(Text::new("67%").fg(text)),
                    )
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::new("NET:").fg(muted))
                            .child(Sparkline::new(vec![20.0, 35.0, 45.0, 40.0, 55.0]).fg(warning))
                            .child(Text::new("55 MB/s").fg(text)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Inline tables").fg(muted))
                    .child(Text::new("• Dashboard metrics").fg(muted))
                    .child(Text::new("• Compact layout").fg(muted)),
            ),
        ),
        Example::new(
            "Tristate",
            "Win/loss/draw sparkline for sports and results",
            Border::rounded().title(" Tristate ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Win/Loss/Draw:").fg(primary))
                    .child(
                        Sparkline::new(vec![
                            1.0, 0.0, 1.0, 0.5, 1.0, 1.0, 0.0, 0.5, 1.0, 0.0, 1.0, 0.5,
                        ])
                        .fg(success),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Daily change:").fg(primary))
                    .child(
                        Sparkline::new(vec![1.0, 1.0, 0.0, 1.0, 0.5, 1.0, 0.0, 0.0, 1.0, 0.5])
                            .fg(success),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Win/loss/draw").fg(muted))
                    .child(Text::new("• Positive/negative").fg(muted))
                    .child(Text::new("• Sports/results").fg(muted)),
            ),
        ),
    ]
}
