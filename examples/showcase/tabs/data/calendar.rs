//! Calendar widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{contribution_map, date_picker, date_range_picker, Calendar};

pub fn render() -> impl View {
    let (primary, success, warning, _error, info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Calendar ").child(
                        vstack()
                            .gap(1)
                            .child(Calendar::new(2026, 2))
                            .child(Text::new(""))
                            .child(Text::new("• Monthly view").fg(muted))
                            .child(Text::new("• Navigation arrows").fg(muted))
                            .child(Text::new("• Current date highlight").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Event Calendar ").child(
                        vstack()
                            .gap(1)
                            .child(Calendar::new(2026, 2))
                            .child(Text::new("• 5: Meeting").fg(primary))
                            .child(Text::new("• 10: Deadline").fg(warning))
                            .child(Text::new("• 14: Review").fg(info))
                            .child(Text::new("• 28: Release").fg(success))
                            .child(Text::new(""))
                            .child(Text::new("• Event markers").fg(muted))
                            .child(Text::new("• Hover for details").fg(muted))
                            .child(Text::new("• Multiple events").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Date Picker ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Select a date:").fg(primary))
                            .child(date_picker())
                            .child(Text::new(""))
                            .child(Text::new("• Interactive selection").fg(muted))
                            .child(Text::new("• Keyboard navigation").fg(muted))
                            .child(Text::new("• Output formatting").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Range Picker ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Date range:").fg(primary))
                            .child(date_range_picker())
                            .child(Text::new(""))
                            .child(Text::new("• Start and end dates").fg(muted))
                            .child(Text::new("• Range highlighting").fg(muted))
                            .child(Text::new("• Duration display").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Multi-Month ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Three month view:").fg(primary))
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(Calendar::new(2026, 1))
                                    .child(Calendar::new(2026, 2))
                                    .child(Calendar::new(2026, 3)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Compact display").fg(muted))
                            .child(Text::new("• Side-by-side").fg(muted))
                            .child(Text::new("• Quarter view").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Activity Heatmap ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Contribution style:").fg(primary))
                            .child(contribution_map(&[1, 3, 5, 2, 8, 4, 6, 3, 7, 5]))
                            .child(Text::new(""))
                            .child(Text::new("• Activity tracking").fg(muted))
                            .child(Text::new("• GitHub-style heatmap").fg(muted))
                            .child(Text::new("• Color intensity").fg(muted)),
                    ),
                ),
        )
}
