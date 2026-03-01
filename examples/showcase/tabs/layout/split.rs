//! Split widget demos

use crate::theme_colors;
use revue::prelude::*;

pub fn render() -> impl View {
    let (primary, _success, _warning, _error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" HSplit ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Horizontal split:").fg(primary))
                            .child(
                                hstack()
                                    .gap(1)
                                    .child(
                                        Border::rounded()
                                            .title("Left")
                                            .child(Text::new("Pane 1").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .title("Right")
                                            .child(Text::new("Pane 2").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Side by side").fg(muted))
                            .child(Text::new("• Resizable divider").fg(muted))
                            .child(Text::new("• Drag to resize").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" VSplit ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Vertical split:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(
                                        Border::rounded()
                                            .title("Top")
                                            .child(Text::new("Pane 1").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .title("Bottom")
                                            .child(Text::new("Pane 2").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Top and bottom").fg(muted))
                            .child(Text::new("• Vertical divider").fg(muted))
                            .child(Text::new("• Ratio control").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Splitter ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Resizable split:").fg(primary))
                            .child(
                                hstack()
                                    .gap(1)
                                    .child(
                                        Border::rounded()
                                            .title("30%")
                                            .child(Text::new("Sidebar").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .title("70%")
                                            .child(Text::new("Content").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Custom ratio").fg(muted))
                            .child(Text::new("• Interactive resize").fg(muted))
                            .child(Text::new("• Save/restore state").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Three Panel ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Three-way split:").fg(primary))
                            .child(
                                hstack()
                                    .gap(1)
                                    .child(
                                        Border::rounded()
                                            .title("Files")
                                            .child(Text::new("Tree").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .title("Editor")
                                            .child(Text::new("Code").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .title("Terminal")
                                            .child(Text::new("Shell").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Multiple panes").fg(muted))
                            .child(Text::new("• IDE layout").fg(muted))
                            .child(Text::new("• Flexible sizing").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Nested Split ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Nested splits:").fg(primary))
                            .child(
                                hstack()
                                    .gap(1)
                                    .child(
                                        Border::rounded()
                                            .title("Nav")
                                            .child(Text::new("Menu").fg(text)),
                                    )
                                    .child(
                                        vstack()
                                            .gap(1)
                                            .child(
                                                Border::rounded()
                                                    .title("Header")
                                                    .child(Text::new("Top").fg(text)),
                                            )
                                            .child(
                                                Border::rounded()
                                                    .title("Content")
                                                    .child(Text::new("Main").fg(text)),
                                            ),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Complex layouts").fg(muted))
                            .child(Text::new("• Nested splits").fg(muted))
                            .child(Text::new("• Full flexibility").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" App Layout ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Standard app layout:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(
                                        hstack()
                                            .gap(2)
                                            .child(Text::new("Header | Nav | Actions").fg(muted)),
                                    )
                                    .child(
                                        hstack()
                                            .gap(1)
                                            .child(Text::new("Sidebar").fg(text))
                                            .child(Text::new("Main Content Area").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Header").fg(muted))
                            .child(Text::new("• Sidebar + Main").fg(muted))
                            .child(Text::new("• Footer option").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Constrained Split ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Size limits on splits:").fg(primary))
                            .child(
                                hstack()
                                    .gap(1)
                                    .min_width(30)
                                    .max_width(50)
                                    .child(
                                        Border::rounded()
                                            .title("30-50w")
                                            .child(Text::new("Min/Max").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .child(Text::new("Width limited").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• min_width() on container").fg(muted))
                            .child(Text::new("• max_width() on container").fg(muted))
                            .child(Text::new("• Responsive layouts").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Min Size Split ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Minimum dimensions:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .min_size(20, 3)
                                    .child(
                                        Border::rounded()
                                            .child(Text::new("At least 20x3").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .child(Text::new("Guaranteed space").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• .min_size(w, h)").fg(muted))
                            .child(Text::new("• Ensures minimum area").fg(muted))
                            .child(Text::new("• Good for small terms").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Max Size Split ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Maximum dimensions:").fg(primary))
                            .child(
                                hstack()
                                    .gap(1)
                                    .max_size(40, 4)
                                    .child(Border::rounded().child(Text::new("Max 40x4").fg(text)))
                                    .child(
                                        Border::rounded()
                                            .child(Text::new("Prevents overflow").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• .max_size(w, h)").fg(muted))
                            .child(Text::new("• Caps maximum size").fg(muted))
                            .child(Text::new("• Good for large terms").fg(muted)),
                    ),
                ),
        )
}
