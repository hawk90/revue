//! Pie chart widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Gauge, PieChart};

pub fn render() -> impl View {
    let (primary, success, warning, error, info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Pie Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Language usage:").fg(primary))
                            .child(
                                PieChart::new()
                                    .slice_colored("Rust", 45.0, primary)
                                    .slice_colored("Python", 25.0, info)
                                    .slice_colored("Go", 15.0, success)
                                    .slice_colored("Other", 15.0, muted),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Part-to-whole ratio").fg(muted))
                            .child(Text::new("• Percentage display").fg(muted))
                            .child(Text::new("• Color legend").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Donut Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Resource allocation:").fg(primary))
                            .child(
                                PieChart::new()
                                    .slice_colored("Compute", 35.0, primary)
                                    .slice_colored("Storage", 25.0, success)
                                    .slice_colored("Network", 20.0, info)
                                    .slice_colored("Other", 20.0, muted),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Ring visualization").fg(muted))
                            .child(Text::new("• Center label").fg(muted))
                            .child(Text::new("• Space efficient").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Rose Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Radial bars:").fg(primary))
                            .child(
                                PieChart::new()
                                    .slice_colored("N", 30.0, primary)
                                    .slice_colored("E", 45.0, success)
                                    .slice_colored("S", 25.0, info)
                                    .slice_colored("W", 55.0, warning)
                                    .slice_colored("NE", 35.0, muted)
                                    .slice_colored("NW", 40.0, error),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Directional data").fg(muted))
                            .child(Text::new("• Wind speed").fg(muted))
                            .child(Text::new("• Categorical").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Gauge Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Speed gauge:").fg(primary))
                            .child(Gauge::new().percent(75.0).label("Speed"))
                            .child(Text::new(""))
                            .child(Text::new("Progress gauge:").fg(primary))
                            .child(Gauge::new().value(0.65).label("Progress"))
                            .child(Text::new(""))
                            .child(Text::new("• Single value").fg(muted))
                            .child(Text::new("• Circular display").fg(muted))
                            .child(Text::new("• Speed/progress").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Radar Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Skills profile:").fg(primary))
                            .child(
                                PieChart::new()
                                    .slice_colored("Speed", 85.0, primary)
                                    .slice_colored("Power", 70.0, success)
                                    .slice_colored("Defense", 90.0, info)
                                    .slice_colored("Technique", 80.0, warning)
                                    .slice_colored("Endurance", 75.0, muted),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Multi-axis data").fg(muted))
                            .child(Text::new("• Skill comparison").fg(muted))
                            .child(Text::new("• Performance view").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Treemap ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Hierarchical data:").fg(primary))
                            .child(
                                PieChart::new()
                                    .slice_colored("Category A", 100.0, primary)
                                    .slice_colored("Category B", 75.0, success)
                                    .slice_colored("Category C", 50.0, info)
                                    .slice_colored("Category D", 30.0, warning)
                                    .slice_colored("Category E", 25.0, muted),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Proportional tiles").fg(muted))
                            .child(Text::new("• Hierarchical").fg(muted))
                            .child(Text::new("• Space filling").fg(muted)),
                    ),
                ),
        )
}
