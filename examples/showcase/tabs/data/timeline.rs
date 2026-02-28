//! Timeline widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{
    stopwatch, timer_widget, Digits, Step, StepStatus, Stepper, Timeline, TimelineEvent,
};

pub fn render() -> impl View {
    let (primary, success, _warning, _error, info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Timeline ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Timeline::new()
                                    .event(TimelineEvent::new("Meeting started").timestamp("10:00"))
                                    .event(TimelineEvent::new("Decision made").timestamp("11:30"))
                                    .event(TimelineEvent::new("Warning issued").timestamp("14:00"))
                                    .event(TimelineEvent::new("Error detected").timestamp("16:00")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Chronological events").fg(muted))
                            .child(Text::new("• Status indicators").fg(muted))
                            .child(Text::new("• Timestamped entries").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Activity Feed ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Alice pushed to main").fg(text))
                            .child(Text::new("  2 min ago").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Bob opened PR #123").fg(text))
                            .child(Text::new("  15 min ago").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Charlie merged PR #122").fg(text))
                            .child(Text::new("  1 hour ago").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Diana commented on issue").fg(text))
                            .child(Text::new("  3 hours ago").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• User activities").fg(muted))
                            .child(Text::new("• Relative timestamps").fg(muted))
                            .child(Text::new("• Avatar support").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Process Timeline ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Stepper::new()
                                    .step(Step::new("Initialize").status(StepStatus::Completed))
                                    .step(Step::new("Load config").status(StepStatus::Completed))
                                    .step(Step::new("Connect database").status(StepStatus::Active))
                                    .step(Step::new("Start server").status(StepStatus::Pending))
                                    .step(Step::new("Health check").status(StepStatus::Pending)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Sequential steps").fg(muted))
                            .child(Text::new("• Progress tracking").fg(muted))
                            .child(Text::new("• Status indicators").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Stopwatch ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Elapsed time:").fg(primary))
                            .child(stopwatch().title("1:01:15"))
                            .child(Text::new(""))
                            .child(Text::new("• Start/stop/reset").fg(muted))
                            .child(Text::new("• Lap times").fg(muted))
                            .child(Text::new("• Precise timing").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Timer ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Countdown:").fg(primary))
                            .child(timer_widget(300).title("Session timeout"))
                            .child(Text::new(""))
                            .child(Text::new("• Countdown display").fg(muted))
                            .child(Text::new("• Warning states").fg(muted))
                            .child(Text::new("• Auto-start option").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Session Timer ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Pomodoro:").fg(primary))
                            .child(
                                hstack()
                                    .gap(3)
                                    .child(
                                        vstack()
                                            .gap(1)
                                            .child(Text::new("Focus").fg(primary))
                                            .child(Digits::timer(1500).fg(success)),
                                    )
                                    .child(
                                        vstack()
                                            .gap(1)
                                            .child(Text::new("Break").fg(muted))
                                            .child(Digits::timer(300).fg(info)),
                                    )
                                    .child(
                                        vstack()
                                            .gap(1)
                                            .child(Text::new("Sessions").fg(muted))
                                            .child(Text::new("4 / 8").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Work/break cycles").fg(muted))
                            .child(Text::new("• Session tracking").fg(muted))
                            .child(Text::new("• Productivity timer").fg(muted)),
                    ),
                ),
        )
}
