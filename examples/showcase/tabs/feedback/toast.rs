//! Toast widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Notification, NotificationCenter, NotificationLevel, Toast};

pub fn render() -> impl View {
    let (primary, _success, _warning, _error, _info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Toast Types ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Notification toasts:").fg(primary))
                            .child(Text::new(""))
                            .child(Toast::success("File saved successfully!"))
                            .child(Toast::error("Failed to upload file."))
                            .child(Toast::warning("Session expiring soon."))
                            .child(Toast::info("New update available."))
                            .child(Text::new(""))
                            .child(Text::new("• Success (green)").fg(muted))
                            .child(Text::new("• Error (red)").fg(muted))
                            .child(Text::new("• Warning (yellow)").fg(muted))
                            .child(Text::new("• Info (blue)").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Toast with Action ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Interactive toast:").fg(primary))
                            .child(Text::new(""))
                            .child(Toast::info("Update Available: Version 2.53.0 is ready."))
                            .child(Text::new(""))
                            .child(Toast::error("Connection Lost: Reconnecting in 10s..."))
                            .child(Text::new(""))
                            .child(Text::new("• Action buttons").fg(muted))
                            .child(Text::new("• Title + message").fg(muted))
                            .child(Text::new("• User interaction").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Toast Queue ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Stacked toasts:").fg(primary))
                            .child(Text::new(""))
                            .child(Toast::success("Task 1 completed."))
                            .child(Toast::success("Task 2 completed."))
                            .child(Toast::info("3 tasks remaining..."))
                            .child(Text::new(""))
                            .child(Text::new("• Multiple notifications").fg(muted))
                            .child(Text::new("• Stacked display").fg(muted))
                            .child(Text::new("• Auto-dismiss").fg(muted))
                            .child(Text::new("• Queue management").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Position Options ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Toast positions:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("• Top Left").fg(muted))
                            .child(Text::new("• Top Center").fg(muted))
                            .child(Text::new("• Top Right (default)").fg(muted))
                            .child(Text::new("• Bottom Left").fg(muted))
                            .child(Text::new("• Bottom Center").fg(muted))
                            .child(Text::new("• Bottom Right").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Configurable per app").fg(primary))
                            .child(Text::new("or per toast.").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Notification Center ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Notification history:").fg(primary))
                            .child(Text::new(""))
                            .child({
                                let mut nc = NotificationCenter::new();
                                nc.push(
                                    Notification::new("Build completed")
                                        .level(NotificationLevel::Success),
                                );
                                nc.push(
                                    Notification::new("Tests passed")
                                        .level(NotificationLevel::Success),
                                );
                                nc.push(
                                    Notification::new("Deploy started")
                                        .level(NotificationLevel::Info),
                                );
                                nc.push(
                                    Notification::new("Review requested")
                                        .level(NotificationLevel::Warning),
                                );
                                nc
                            })
                            .child(Text::new(""))
                            .child(Text::new("• Persistent history").fg(muted))
                            .child(Text::new("• Read/unread state").fg(muted))
                            .child(Text::new("• Clear all option").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Progress Toast ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("With progress:").fg(primary))
                            .child(Text::new(""))
                            .child(Toast::info("Uploading... 75%"))
                            .child(Text::new(""))
                            .child(Toast::info("Downloading: 3 of 10 files (30%)"))
                            .child(Text::new(""))
                            .child(Text::new("• Progress bar").fg(muted))
                            .child(Text::new("• Cancel button").fg(muted))
                            .child(Text::new("• Status updates").fg(muted)),
                    ),
                ),
        )
}
