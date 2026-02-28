//! Overlay widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Spinner, StatusBar, StatusSection};

pub fn render() -> impl View {
    let (primary, _success, _warning, _error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Status Bar ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Application status:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                StatusBar::new()
                                    .left(StatusSection::new("Ready"))
                                    .right(StatusSection::new("Ln 42, Col 15 | UTF-8 | Rust")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Bottom status bar").fg(muted))
                            .child(Text::new("• Left/right sections").fg(muted))
                            .child(Text::new("• Dynamic updates").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Error Boundary ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Error handling:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                Border::rounded().title(" Error ").child(
                                    vstack()
                                        .gap(1)
                                        .child(Text::new("Something went wrong.").fg(text))
                                        .child(Text::new("Please try again.").fg(muted))
                                        .child(Button::new("Retry")),
                                ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Error catching").fg(muted))
                            .child(Text::new("• Fallback UI").fg(muted))
                            .child(Text::new("• Recovery options").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Debug Overlay ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Debug info (Dev mode):").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("FPS: 60").fg(text))
                            .child(Text::new("Memory: 45 MB").fg(text))
                            .child(Text::new("Render: 2.3ms").fg(text))
                            .child(Text::new("Widgets: 127").fg(text))
                            .child(Text::new(""))
                            .child(Text::new("• Performance metrics").fg(muted))
                            .child(Text::new("• Debug toggle").fg(muted))
                            .child(Text::new("• Development aid").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Backdrop ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Overlay backdrop:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                Border::rounded()
                                    .title(" Focused Content ")
                                    .child(Text::new("This content is on top of the backdrop.")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Dim background").fg(muted))
                            .child(Text::new("• Focus attention").fg(muted))
                            .child(Text::new("• Blur option").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Loading Overlay ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Full-screen loading:").fg(primary))
                            .child(Text::new(""))
                            .child(Spinner::new().label("Loading..."))
                            .child(
                                Text::new("Please wait while we process your request.").fg(muted),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Block interaction").fg(muted))
                            .child(Text::new("• Progress indicator").fg(muted))
                            .child(Text::new("• Cancelable option").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Drawer ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Side drawer:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                Border::rounded().title(" Settings ").child(
                                    vstack()
                                        .gap(1)
                                        .child(Checkbox::new("Enable feature A").checked(true))
                                        .child(Checkbox::new("Enable feature B").checked(false))
                                        .child(Text::new(""))
                                        .child(Button::primary("Apply")),
                                ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Slide-in panel").fg(muted))
                            .child(Text::new("• Left/right/bottom").fg(muted))
                            .child(Text::new("• Settings/details").fg(muted)),
                    ),
                ),
        )
}
