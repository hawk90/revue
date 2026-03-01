//! Select widget demos (Select, Combobox, RadioGroup)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{selection_list, Combobox, MultiSelect, RadioGroup, Select};

pub fn examples(radio_selected: usize) -> Vec<Example> {
    let (primary, _success, _, _, _info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Radio Group",
            "Single selection from mutually exclusive options",
            Border::rounded().title(" Radio Group ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Select one:").fg(primary))
                    .child(RadioGroup::new(["Low", "Medium", "High"]).selected(radio_selected))
                    .child(Text::new(""))
                    .child(Text::new("[↑/↓] change selection").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• Single selection only").fg(muted))
                    .child(Text::new("• Mutually exclusive").fg(muted))
                    .child(Text::new("• 2-5 options ideal").fg(muted)),
            ),
        ),
        Example::new(
            "Select",
            "Dropdown selection for choosing from a list",
            Border::rounded().title(" Select ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Dropdown select:").fg(primary))
                    .child(
                        Select::new()
                            .option("Option A")
                            .option("Option B")
                            .option("Option C")
                            .placeholder("Select..."),
                    )
                    .child(Text::new(""))
                    .child(Text::new("With selected:").fg(primary))
                    .child(Select::new().option("Rust").option("Python").option("Go"))
                    .child(Text::new(""))
                    .child(Text::new("• Dropdown selection").fg(muted))
                    .child(Text::new("• Searchable option").fg(muted))
                    .child(Text::new("• Many options support").fg(muted)),
            ),
        ),
        Example::new(
            "Multi-Select",
            "Select multiple items with checkbox-style UI",
            Border::rounded().title(" Multi-Select ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Select multiple:").fg(primary))
                    .child(
                        MultiSelect::new()
                            .option("Feature 1")
                            .option("Feature 2")
                            .option("Feature 3")
                            .option("Feature 4"),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Multiple selections").fg(muted))
                    .child(Text::new("• Checkbox-style UI").fg(muted))
                    .child(Text::new("• Tag display selected").fg(muted)),
            ),
        ),
        Example::new(
            "Combobox",
            "Combined input and dropdown for search and select",
            Border::rounded().title(" Combobox ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Search + Select:").fg(primary))
                    .child(
                        Combobox::new()
                            .option("Apple")
                            .option("Banana")
                            .option("Cherry")
                            .option("Date")
                            .option("Elderberry")
                            .placeholder("Type to filter..."),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Input + dropdown").fg(muted))
                    .child(Text::new("• Filter as you type").fg(muted))
                    .child(Text::new("• Custom values allowed").fg(muted)),
            ),
        ),
        Example::new(
            "Selection List",
            "Vertical list selection with keyboard navigation",
            Border::rounded().title(" Selection List ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("List selection:").fg(primary))
                    .child(selection_list(vec!["Item 1", "Item 2", "Item 3", "Item 4"]))
                    .child(Text::new(""))
                    .child(Text::new("• Vertical list selection").fg(muted))
                    .child(Text::new("• Keyboard navigation").fg(muted))
                    .child(Text::new("• Good for many items").fg(muted)),
            ),
        ),
    ]
}
