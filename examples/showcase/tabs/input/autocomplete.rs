//! Autocomplete, Combobox, and SearchBar widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{autocomplete, combobox, search_bar, Autocomplete, Combobox, SearchBar};

pub fn render() -> impl View {
    let (primary, _success, _warning, _error, info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded()
                        .title(" Autocomplete ")
                        .min_width(35)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Type-ahead suggestions:").fg(primary))
                                .child(
                                    Autocomplete::new()
                                        .placeholder("Type a fruit...")
                                        .suggestions(vec![
                                            "Apple",
                                            "Apricot",
                                            "Avocado",
                                            "Banana",
                                            "Blackberry",
                                            "Blueberry",
                                            "Cherry",
                                            "Coconut",
                                            "Dragon fruit",
                                            "Grape",
                                            "Kiwi",
                                            "Lemon",
                                            "Mango",
                                            "Orange",
                                            "Papaya",
                                            "Peach",
                                            "Pear",
                                            "Pineapple",
                                            "Plum",
                                            "Strawberry",
                                            "Watermelon",
                                        ]),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Features:").fg(info))
                                .child(Text::new("• Real-time filtering").fg(muted))
                                .child(Text::new("• Keyboard navigation").fg(muted))
                                .child(Text::new("• Custom suggestion rendering").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Combobox ")
                        .min_width(35)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Dropdown + text input:").fg(primary))
                                .child(
                                    Combobox::new()
                                        .placeholder("Select a country...")
                                        .options(vec![
                                            "United States",
                                            "United Kingdom",
                                            "Canada",
                                            "Australia",
                                            "Germany",
                                            "France",
                                            "Japan",
                                            "South Korea",
                                            "China",
                                            "Brazil",
                                        ])
                                        .width(30),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Combobox features:").fg(info))
                                .child(Text::new("• Type to filter").fg(muted))
                                .child(Text::new("• Click to select").fg(muted))
                                .child(Text::new("• Free text input allowed").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Search Bar ")
                        .min_width(40)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Full-featured search:").fg(primary))
                                .child(
                                    SearchBar::new()
                                        .placeholder("Search files, folders...")
                                        .width(35),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Search with icon:").fg(primary))
                                .child(
                                    search_bar()
                                        .placeholder("Search documentation...")
                                        .width(35),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Search features:").fg(info))
                                .child(Text::new("• Search icon integrated").fg(muted))
                                .child(Text::new("• Clear button").fg(muted))
                                .child(Text::new("• Loading state").fg(muted)),
                        ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded()
                        .title(" Autocomplete Variants ")
                        .min_width(35)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("With categories:").fg(primary))
                                .child(autocomplete().placeholder("Search commands..."))
                                .child(Text::new(""))
                                .child(Text::new("With icons:").fg(primary))
                                .child(autocomplete().placeholder("Search files..."))
                                .child(Text::new(""))
                                .child(Text::new("• Category grouping").fg(muted))
                                .child(Text::new("• Icon support").fg(muted))
                                .child(Text::new("• Highlight matching text").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Combobox States ")
                        .min_width(35)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Default state:").fg(primary))
                                .child(combobox().placeholder("Select option...").width(30))
                                .child(Text::new(""))
                                .child(Text::new("With selection:").fg(primary))
                                .child(Combobox::new().value("South Korea").width(30))
                                .child(Text::new(""))
                                .child(Text::new("• Controlled/uncontrolled").fg(muted))
                                .child(Text::new("• Multi-select variant").fg(muted))
                                .child(Text::new("• Async option loading").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Search Patterns ")
                        .min_width(40)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Global search:").fg(primary))
                                .child(
                                    search_bar()
                                        .placeholder("Search everywhere... (Ctrl+K)")
                                        .width(35),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Filter search:").fg(primary))
                                .child(search_bar().placeholder("Filter list...").width(35))
                                .child(Text::new(""))
                                .child(Text::new("• Keyboard shortcuts").fg(muted))
                                .child(Text::new("• Search history").fg(muted))
                                .child(Text::new("• Recent searches").fg(muted)),
                        ),
                ),
        )
}
