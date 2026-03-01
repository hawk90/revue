//! Alert widget demos (Alert, Callout, EmptyState)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Alert, Callout, EmptyState, Toast};

pub fn examples() -> Vec<Example> {
    let (primary, _success, _warning, _error, _info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Alerts",
            "Informational, success, warning, and error messages",
            Border::rounded().title(" Alerts ").child(
                vstack()
                    .gap(1)
                    .child(Alert::info("This is an informational message."))
                    .child(Alert::success("Operation completed successfully!"))
                    .child(Alert::warning("Warning: Low disk space detected."))
                    .child(Alert::error("Error: Connection failed."))
                    .child(Text::new(""))
                    .child(Text::new("• Info: General information").fg(muted))
                    .child(Text::new("• Success: Positive feedback").fg(muted))
                    .child(Text::new("• Warning: Caution needed").fg(muted))
                    .child(Text::new("• Error: Problems occurred").fg(muted)),
            ),
        ),
        Example::new(
            "Alert Styles",
            "Titled, dismissible, and actionable alert variants",
            Border::rounded().title(" Alert Styles ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("With title:").fg(primary))
                    .child(Alert::error(
                        "Connection Error: Failed to connect to server.",
                    ))
                    .child(Text::new(""))
                    .child(Text::new("Dismissible:").fg(primary))
                    .child(Alert::warning("This alert can be dismissed."))
                    .child(Text::new(""))
                    .child(Text::new("With actions:").fg(primary))
                    .child(Alert::info("Update Available: A new version is ready."))
                    .child(Text::new(""))
                    .child(Text::new("• Title + message").fg(muted))
                    .child(Text::new("• Dismissible option").fg(muted))
                    .child(Text::new("• Action buttons").fg(muted)),
            ),
        ),
        Example::new(
            "Callouts",
            "Standout content blocks with icons and titles",
            Border::rounded().title(" Callouts ").child(
                vstack()
                    .gap(1)
                    .child(
                        Callout::tip("Use keyboard shortcuts for faster navigation.").title("Tip"),
                    )
                    .child(Text::new(""))
                    .child(
                        Callout::warning("Changes cannot be undone after saving.").title("Caution"),
                    )
                    .child(Text::new(""))
                    .child(Callout::note("This feature is still in beta.").title("Note"))
                    .child(Text::new(""))
                    .child(Text::new("• Icon + title + body").fg(muted))
                    .child(Text::new("• Standout content").fg(muted))
                    .child(Text::new("• Custom icons").fg(muted)),
            ),
        ),
        Example::new(
            "Empty States",
            "Friendly messaging when no data is available",
            Border::rounded().title(" Empty States ").child(
                vstack()
                    .gap(1)
                    .child(
                        EmptyState::new("No messages")
                            .description("You don't have any messages yet."),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Empty data display").fg(muted))
                    .child(Text::new("• Guiding user actions").fg(muted))
                    .child(Text::new("• Friendly messaging").fg(muted)),
            ),
        ),
        Example::new(
            "Empty States + Action",
            "Empty states with call-to-action guidance",
            Border::rounded().title(" Empty States + Action ").child(
                vstack()
                    .gap(1)
                    .child(
                        EmptyState::no_results("No results found")
                            .description("Try adjusting your search or filters."),
                    )
                    .child(Text::new(""))
                    .child(
                        EmptyState::new("No files")
                            .description("Drag and drop files here or click to upload."),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Call-to-action button").fg(muted))
                    .child(Text::new("• Guide next steps").fg(muted)),
            ),
        ),
        Example::new(
            "Toast Notifications",
            "Brief auto-dismissing notification messages",
            Border::rounded().title(" Toast Notifications ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Toast messages:").fg(primary))
                    .child(
                        vstack()
                            .gap(1)
                            .child(Toast::success("File saved successfully!"))
                            .child(Toast::error("Failed to upload file."))
                            .child(Toast::warning("Session expiring soon."))
                            .child(Toast::info("New update available.")),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Brief notifications").fg(muted))
                    .child(Text::new("• Auto-dismiss").fg(muted))
                    .child(Text::new("• Position variants").fg(muted)),
            ),
        ),
    ]
}
