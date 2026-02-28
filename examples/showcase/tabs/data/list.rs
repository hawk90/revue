//! List widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{List, SelectionList, SortableList, VirtualList};

pub fn render() -> impl View {
    let (primary, success, warning, error, _info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" List ").child(
                        vstack()
                            .gap(1)
                            .child(List::new(vec![
                                "Task: Write documentation",
                                "Task: Add tests",
                                "Task: Review PR",
                                "Task: Deploy",
                                "Task: Monitor",
                            ]))
                            .child(Text::new(""))
                            .child(Text::new("• Simple item list").fg(muted))
                            .child(Text::new("• String items").fg(muted))
                            .child(Text::new("• Vertical layout").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Selectable List ").child(
                        vstack()
                            .gap(1)
                            .child(
                                SelectionList::new(vec![
                                    "Option A: Default selection",
                                    "Option B: Alternative choice",
                                    "Option C: Another option",
                                    "Option D: Last option",
                                ])
                                .selected(vec![0]),
                            )
                            .child(Text::new(""))
                            .child(Text::new("[↑/↓] navigate [Enter] select").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• Keyboard navigation").fg(muted))
                            .child(Text::new("• Selection highlight").fg(muted))
                            .child(Text::new("• Single/multi select").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Virtual List ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Large dataset (1000 items):").fg(primary))
                            .child(Text::new("(Virtual scrolling enabled)").fg(muted))
                            .child(VirtualList::new(
                                (0..1000).map(|i| format!("Item {}", i)).collect(),
                            ))
                            .child(Text::new(""))
                            .child(Text::new("• Efficient rendering").fg(muted))
                            .child(Text::new("• Large datasets").fg(muted))
                            .child(Text::new("• Lazy loading").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Sortable List ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Drag to reorder:").fg(primary))
                            .child(SortableList::new(vec![
                                "★ Priority 1: Critical",
                                "☆ Priority 2: High",
                                "☆ Priority 3: Medium",
                                "☆ Priority 4: Low",
                            ]))
                            .child(Text::new(""))
                            .child(Text::new("• Drag and drop").fg(muted))
                            .child(Text::new("• Reorder items").fg(muted))
                            .child(Text::new("• Priority sorting").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Grouped List ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Today:").fg(primary))
                            .child(List::new(vec!["Meeting at 10am", "Review code"]))
                            .child(Text::new("Tomorrow:").fg(primary))
                            .child(List::new(vec!["Team standup", "Deploy release"]))
                            .child(Text::new("This Week:").fg(primary))
                            .child(List::new(vec!["Sprint planning", "Documentation"]))
                            .child(Text::new(""))
                            .child(Text::new("• Grouped items").fg(muted))
                            .child(Text::new("• Section headers").fg(muted))
                            .child(Text::new("• Category display").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Card List ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Card layout:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(
                                        Border::rounded().child(
                                            vstack()
                                                .child(Text::new("Bug #123").fg(error))
                                                .child(Text::new("Fix login validation")),
                                        ),
                                    )
                                    .child(
                                        Border::rounded().child(
                                            vstack()
                                                .child(Text::new("Feature #456").fg(warning))
                                                .child(Text::new("Add dark mode")),
                                        ),
                                    )
                                    .child(
                                        Border::rounded().child(
                                            vstack()
                                                .child(Text::new("Task #789").fg(success))
                                                .child(Text::new("Update dependencies")),
                                        ),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Card layout").fg(muted))
                            .child(Text::new("• Rich content").fg(muted))
                            .child(Text::new("• Issue/ticket view").fg(muted)),
                    ),
                ),
        )
}
