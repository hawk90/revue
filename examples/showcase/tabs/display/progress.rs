//! Progress widget demos

use crate::{theme_colors, themed_gauge, threshold_gauge};
use revue::prelude::*;
use revue::widget::{battery, stopwatch, timer_widget, Spinner, Step, StepStatus, Stepper};

pub fn render(cpu: f64, memory: f64) -> impl View {
    let (primary, success, warning, error, info, muted, _text, surface) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Progress Bars ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Basic progress:").fg(primary))
                            .child(Progress::new(0.75))
                            .child(Text::new(""))
                            .child(Text::new("With colors:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Progress::new(0.75).filled_color(success))
                                    .child(Progress::new(0.45).filled_color(info))
                                    .child(Progress::new(0.90).filled_color(warning))
                                    .child(Progress::new(0.30).filled_color(error)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("With percentages:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Progress::new(0.65).show_percentage(true))
                                    .child(Progress::new(0.80).show_percentage(true)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Visual completion").fg(muted))
                            .child(Text::new("• Custom colors").fg(muted))
                            .child(Text::new("• Percentage display").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Gauges ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("System gauges:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(threshold_gauge(
                                        cpu, "CPU", success, error, surface, 0.8,
                                    ))
                                    .child(themed_gauge(memory, "Memory", info, surface))
                                    .child(themed_gauge(0.45, "Disk", warning, surface))
                                    .child(themed_gauge(0.92, "Network", primary, surface)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("Custom width:").fg(primary))
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(themed_gauge(0.75, "A", success, surface).width(10))
                                    .child(themed_gauge(0.50, "B", info, surface).width(10))
                                    .child(themed_gauge(0.25, "C", warning, surface).width(10)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Visual percentage").fg(muted))
                            .child(Text::new("• Threshold coloring").fg(muted))
                            .child(Text::new("• Custom dimensions").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Battery ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Battery status:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(battery(0.85).label("Laptop"))
                                    .child(battery(0.45).label("Phone"))
                                    .child(battery(0.15).label("Mouse")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("Charging:").fg(primary))
                            .child(battery(0.67).label("Tablet"))
                            .child(Text::new(""))
                            .child(Text::new("• Device battery").fg(muted))
                            .child(Text::new("• Charging indicator").fg(muted))
                            .child(Text::new("• Low battery warning").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Timer ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Countdown timer:").fg(primary))
                            .child(timer_widget(65))
                            .child(Text::new(""))
                            .child(Text::new("Stopwatch:").fg(primary))
                            .child(stopwatch())
                            .child(Text::new(""))
                            .child(Text::new("• Countdown mode").fg(muted))
                            .child(Text::new("• Stopwatch mode").fg(muted))
                            .child(Text::new("• Custom formatting").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Indeterminate ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Loading (unknown time):").fg(primary))
                            .child(Spinner::new().label("Processing..."))
                            .child(Text::new(""))
                            .child(Text::new("Animated spinner:").fg(primary))
                            .child(Spinner::new().label("Connecting..."))
                            .child(Text::new(""))
                            .child(Text::new("• Unknown duration").fg(muted))
                            .child(Text::new("• Animated indicator").fg(muted))
                            .child(Text::new("• Spinner variant").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Steps Progress ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Multi-step progress:").fg(primary))
                            .child(
                                vstack().gap(1).child(
                                    Stepper::new()
                                        .step(Step::new("Setup").status(StepStatus::Completed))
                                        .step(Step::new("Configure").status(StepStatus::Completed))
                                        .step(Step::new("Build").status(StepStatus::Active))
                                        .step(Step::new("Deploy").status(StepStatus::Pending)),
                                ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Installation wizard").fg(muted))
                            .child(Text::new("• Multi-stage process").fg(muted))
                            .child(Text::new("• Clear progress").fg(muted)),
                    ),
                ),
        )
}
