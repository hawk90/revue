//! Bar chart widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{BarChart, Histogram};

pub fn render(frame: u64) -> impl View {
    let (primary, success, _warning, _error, _info, muted, text, _) = theme_colors();
    let offset = (frame as f64 * 0.05).sin() * 20.0;

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Bar Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Monthly Revenue:").fg(primary))
                            .child(
                                BarChart::new()
                                    .bar("Jan", 120.0 + offset)
                                    .bar("Feb", 180.0 - offset * 0.5)
                                    .bar("Mar", 150.0 + offset * 0.3)
                                    .bar("Apr", 210.0)
                                    .bar("May", 190.0 + offset * 0.7)
                                    .bar("Jun", 240.0 - offset * 0.2)
                                    .show_values(true)
                                    .fg(success),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Vertical bar visualization").fg(muted))
                            .child(Text::new("• Custom colors and labels").fg(muted))
                            .child(Text::new("• Animated data").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Horizontal Bar ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Category comparison:").fg(primary))
                            .child(
                                BarChart::new()
                                    .bar("Cat A", 85.0)
                                    .bar("Cat B", 72.0)
                                    .bar("Cat C", 93.0)
                                    .bar("Cat D", 45.0)
                                    .bar("Cat E", 68.0)
                                    .fg(primary),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Horizontal orientation").fg(muted))
                            .child(Text::new("• Good for labels").fg(muted))
                            .child(Text::new("• Comparison view").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Multi-series Bar ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Comparison data:").fg(primary))
                            .child(
                                BarChart::new()
                                    .bar("Q1", 120.0)
                                    .bar("Q2", 145.0)
                                    .bar("Q3", 135.0)
                                    .bar("Q4", 160.0)
                                    .fg(success),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Multiple series").fg(muted))
                            .child(Text::new("• Year comparison").fg(muted))
                            .child(Text::new("• Grouped display").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Histogram ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Distribution:").fg(primary))
                            .child(Histogram::new(&[5.0, 12.0, 28.0, 35.0, 22.0, 15.0, 8.0]))
                            .child(Text::new(""))
                            .child(Text::new("• Frequency distribution").fg(muted))
                            .child(Text::new("• Statistical analysis").fg(muted))
                            .child(Text::new("• Data bins").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Stacked Bar ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Stacked series:").fg(primary))
                            .child(
                                BarChart::new()
                                    .bar("Prod A", 120.0)
                                    .bar("Prod B", 95.0)
                                    .bar("Prod C", 140.0)
                                    .fg(success),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Part-to-whole").fg(muted))
                            .child(Text::new("• Segment colors").fg(muted))
                            .child(Text::new("• Composition view").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Box Plot ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Statistical summary:").fg(primary))
                            .child(Text::new("A: [10, 25, 50, 75, 90]").fg(text))
                            .child(Text::new("B: [15, 30, 55, 80, 95]").fg(text))
                            .child(Text::new("C: [5, 20, 40, 60, 85]").fg(text))
                            .child(Text::new(""))
                            .child(Text::new("• Min/max/median").fg(muted))
                            .child(Text::new("• Quartile display").fg(muted))
                            .child(Text::new("• Outlier detection").fg(muted)),
                    ),
                ),
        )
}
