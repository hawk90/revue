//! Special chart widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{waveline, BarChart, HeatMap, Progress, ScatterChart, ScatterSeries};

pub fn render(wave_data: &[f64]) -> impl View {
    let (primary, success, warning, _error, info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Heat Map ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Activity intensity:").fg(primary))
                            .child(
                                HeatMap::new(vec![
                                    vec![0.1, 0.3, 0.5, 0.7, 0.9, 0.6, 0.4],
                                    vec![0.2, 0.4, 0.6, 0.8, 0.5, 0.3, 0.1],
                                    vec![0.5, 0.7, 0.9, 0.6, 0.4, 0.2, 0.3],
                                    vec![0.8, 0.6, 0.4, 0.2, 0.3, 0.5, 0.7],
                                ])
                                .custom_colors(info, primary),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Grid-based intensity").fg(muted))
                            .child(Text::new("• Color gradients").fg(muted))
                            .child(Text::new("• Activity tracking").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Streamline ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Audio visualization:").fg(primary))
                            .child(waveline(wave_data.to_vec()).color(warning))
                            .child(Text::new(""))
                            .child(Text::new("Real-time data:").fg(primary))
                            .child(
                                waveline(
                                    (0..50)
                                        .map(|i| {
                                            let x = i as f64 * 0.2;
                                            x.sin() * 0.5 + (x * 2.0).cos() * 0.3 + 0.5
                                        })
                                        .collect(),
                                )
                                .color(success),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Flow visualization").fg(muted))
                            .child(Text::new("• Continuous stream").fg(muted))
                            .child(Text::new("• Real-time updates").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Bubble Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Multi-dimensional:").fg(primary))
                            .child(
                                ScatterChart::new()
                                    .series(
                                        ScatterSeries::new("Bubbles")
                                            .data(vec![
                                                (10.0, 20.0),
                                                (25.0, 35.0),
                                                (40.0, 30.0),
                                                (55.0, 50.0),
                                                (70.0, 45.0),
                                            ])
                                            .sizes(vec![50.0, 80.0, 40.0, 100.0, 60.0])
                                            .color(primary),
                                    )
                                    .no_legend(),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• X/Y + size").fg(muted))
                            .child(Text::new("• 3 dimensions").fg(muted))
                            .child(Text::new("• Category analysis").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Funnel Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Conversion funnel:").fg(primary))
                            .child(
                                BarChart::new()
                                    .bar("Visitors", 100.0)
                                    .bar("Sign-ups", 50.0)
                                    .bar("Active", 25.0)
                                    .bar("Purchased", 10.0)
                                    .bar("Repeat", 5.0)
                                    .fg(primary),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Pipeline stages").fg(muted))
                            .child(Text::new("• Drop-off analysis").fg(muted))
                            .child(Text::new("• Conversion rates").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Bullet Graph ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Performance vs target:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Text::new("Revenue: 75% (target 80%)").fg(text))
                                    .child(Progress::new(0.75).filled_color(success))
                                    .child(Text::new("Users: 90% (target 85%)").fg(text))
                                    .child(Progress::new(0.90).filled_color(info))
                                    .child(Text::new("Growth: 65% (target 70%)").fg(text))
                                    .child(Progress::new(0.65).filled_color(warning)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Actual vs target").fg(muted))
                            .child(Text::new("• Range bands").fg(muted))
                            .child(Text::new("• KPI display").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Sankey Diagram ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Flow diagram:").fg(primary))
                            .child(
                                BarChart::new()
                                    .bar("Source A", 100.0)
                                    .bar("Source B", 80.0)
                                    .bar("Process", 180.0)
                                    .bar("Output X", 110.0)
                                    .bar("Output Y", 70.0)
                                    .fg(info),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Flow between nodes").fg(muted))
                            .child(Text::new("• Quantity sizing").fg(muted))
                            .child(Text::new("• Process mapping").fg(muted)),
                    ),
                ),
        )
}
