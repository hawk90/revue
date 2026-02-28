//! Menu widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Menu, MenuBar, MenuItem};

pub fn render() -> impl View {
    let (primary, success, _warning, error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Menu Bar ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Application menu:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                MenuBar::new()
                                    .menu(
                                        Menu::new("File")
                                            .item(MenuItem::new("New").shortcut("Ctrl+N"))
                                            .item(MenuItem::new("Open...").shortcut("Ctrl+O"))
                                            .separator()
                                            .item(MenuItem::new("Save").shortcut("Ctrl+S"))
                                            .item(
                                                MenuItem::new("Save As...")
                                                    .shortcut("Ctrl+Shift+S"),
                                            )
                                            .separator()
                                            .item(MenuItem::new("Exit").shortcut("Ctrl+Q")),
                                    )
                                    .menu(
                                        Menu::new("Edit")
                                            .item(MenuItem::new("Undo"))
                                            .item(MenuItem::new("Redo"))
                                            .separator()
                                            .item(MenuItem::new("Cut"))
                                            .item(MenuItem::new("Copy"))
                                            .item(MenuItem::new("Paste")),
                                    )
                                    .menu(
                                        Menu::new("View")
                                            .item(MenuItem::new("Toggle Sidebar"))
                                            .item(MenuItem::new("Toggle Status Bar")),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Dropdown menus").fg(muted))
                            .child(Text::new("• Keyboard shortcuts").fg(muted))
                            .child(Text::new("• Dividers").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Context Menu ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Right-click menu:").fg(primary))
                            .child(Text::new("[Right-click for context menu]").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Menu items:").fg(muted))
                            .child(Text::new("• Copy (Ctrl+C)").fg(text))
                            .child(Text::new("• Cut (Ctrl+X)").fg(text))
                            .child(Text::new("• Paste (Ctrl+V)").fg(text))
                            .child(Text::new("• ───────────").fg(muted))
                            .child(Text::new("• Select All (Ctrl+A)").fg(text))
                            .child(Text::new(""))
                            .child(Text::new("• Right-click trigger").fg(muted))
                            .child(Text::new("• Contextual actions").fg(muted))
                            .child(Text::new("• Position-aware").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Menu Features ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Menu capabilities:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("• Keyboard shortcuts").fg(text))
                            .child(Text::new("• Separators/dividers").fg(text))
                            .child(Text::new("• Nested submenus").fg(text))
                            .child(Text::new("• Checked items").fg(text))
                            .child(Text::new("• Disabled items").fg(text))
                            .child(Text::new("• Icons (optional)").fg(text))
                            .child(Text::new(""))
                            .child(Text::new("• Top bar navigation").fg(muted))
                            .child(Text::new("• Keyboard accessible").fg(muted))
                            .child(Text::new("• Alt+key shortcuts").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Action Menu ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Icon actions:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("Action items:").fg(muted))
                            .child(Text::new("• Edit").fg(text))
                            .child(Text::new("• Duplicate").fg(text))
                            .child(Text::new("• Move").fg(text))
                            .child(Text::new("• ───────────").fg(muted))
                            .child(Text::new("• Delete (danger)").fg(error))
                            .child(Text::new(""))
                            .child(Text::new("• Compact trigger").fg(muted))
                            .child(Text::new("• Danger items").fg(muted))
                            .child(Text::new("• Action overflow").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Nested Menu ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Submenu structure:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("File").fg(text))
                            .child(Text::new("  New File").fg(text))
                            .child(Text::new("  New from Template >").fg(text))
                            .child(Text::new("    Python Project").fg(muted))
                            .child(Text::new("    Rust Project").fg(muted))
                            .child(Text::new("    Web Project").fg(muted))
                            .child(Text::new("  ────────────────").fg(muted))
                            .child(Text::new("  Export As >").fg(text))
                            .child(Text::new("    PDF").fg(muted))
                            .child(Text::new("    HTML").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• Nested levels").fg(muted))
                            .child(Text::new("• Arrow indicator").fg(muted))
                            .child(Text::new("• Category grouping").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Radio Menu ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Selection menu:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("Theme:").fg(text))
                            .child(Text::new("• Light Theme").fg(success))
                            .child(Text::new("  Dark Theme").fg(muted))
                            .child(Text::new("  System Theme").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Font Size:").fg(text))
                            .child(Text::new("  Small").fg(muted))
                            .child(Text::new("• Medium").fg(success))
                            .child(Text::new("  Large").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• Radio items").fg(muted))
                            .child(Text::new("• Single selection").fg(muted))
                            .child(Text::new("• Checkmark indicator").fg(muted)),
                    ),
                ),
        )
}
