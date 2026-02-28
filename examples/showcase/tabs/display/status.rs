//! Status widget demos (Spinner, Gauge, Digits, Skeleton)

use crate::{theme_colors, themed_gauge, threshold_gauge};
use revue::prelude::*;

pub fn render(frame: u64) -> impl View {
    let (primary, success, warning, error, info, muted, _text, surface) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Spinners ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Loading indicators:").fg(primary))
                            .child(
                                hstack()
                                    .gap(3)
                                    .child(Spinner::new().label("Loading..."))
                                    .child(
                                        Spinner::new()
                                            .style(SpinnerStyle::Dots)
                                            .label("Processing"),
                                    )
                                    .child(
                                        Spinner::new().style(SpinnerStyle::Line).label("Syncing"),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("Custom styles:").fg(primary))
                            .child(
                                hstack()
                                    .gap(3)
                                    .child(Spinner::new().style(SpinnerStyle::Dots))
                                    .child(Spinner::new().style(SpinnerStyle::Line))
                                    .child(Spinner::new().style(SpinnerStyle::Circle)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Animated loading states").fg(muted))
                            .child(Text::new("• Multiple spinner styles").fg(muted))
                            .child(Text::new("• Custom labels").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Gauges ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Storage:").fg(primary))
                            .child(themed_gauge(0.75, "75%", success, surface))
                            .child(Text::new("Battery:").fg(primary))
                            .child(themed_gauge(0.35, "35%", warning, surface))
                            .child(Text::new("CPU:").fg(primary))
                            .child(threshold_gauge(0.85, "85%", success, error, surface, 0.8))
                            .child(Text::new("Memory:").fg(primary))
                            .child(themed_gauge(0.67, "67%", info, surface))
                            .child(Text::new(""))
                            .child(Text::new("• Visual progress display").fg(muted))
                            .child(Text::new("• Threshold coloring").fg(muted))
                            .child(Text::new("• Customizable width").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Digits ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Timer:").fg(primary))
                            .child(Digits::timer(frame))
                            .child(Text::new(""))
                            .child(Text::new("Counter:").fg(primary))
                            .child(Digits::new(12345))
                            .child(Text::new(""))
                            .child(Text::new("Percentage:").fg(primary))
                            .child(Digits::from_float(87.65, 1))
                            .child(Text::new(""))
                            .child(Text::new("Large number:").fg(primary))
                            .child(Digits::new(9876543))
                            .child(Text::new(""))
                            .child(Text::new("• Large digit display").fg(muted))
                            .child(Text::new("• Timer/counter formats").fg(muted))
                            .child(Text::new("• ASCII art digits").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Skeleton Loading ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Loading states:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Skeleton::new().width(30).height(1))
                                    .child(Skeleton::new().width(25).height(1))
                                    .child(Skeleton::new().width(28).height(1)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("Card skeleton:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Skeleton::new().width(20).height(3))
                                    .child(Skeleton::new().width(15).height(1)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Placeholder loading").fg(muted))
                            .child(Text::new("• Improves perceived speed").fg(muted))
                            .child(Text::new("• Custom dimensions").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Live Stats ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("System metrics:").fg(primary))
                            .child(
                                hstack()
                                    .gap(4)
                                    .child(
                                        vstack().gap(1).child(Text::new("CPU:").fg(muted)).child(
                                            threshold_gauge(
                                                0.42, "42%", success, error, surface, 0.8,
                                            ),
                                        ),
                                    )
                                    .child(
                                        vstack()
                                            .gap(1)
                                            .child(Text::new("MEM:").fg(muted))
                                            .child(themed_gauge(0.67, "67%", info, surface)),
                                    )
                                    .child(
                                        vstack()
                                            .gap(1)
                                            .child(Text::new("DISK:").fg(muted))
                                            .child(themed_gauge(0.83, "83%", warning, surface)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Real-time updates").fg(muted))
                            .child(Text::new("• Dashboard layouts").fg(muted)),
                    ),
                ),
        )
}
