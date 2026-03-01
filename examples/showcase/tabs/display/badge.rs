//! Badge widget demos (Badge, Tag, Avatar, StatusIndicator)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;

pub fn examples() -> Vec<Example> {
    let (primary, success, _warning, error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "Badges",
            "Version labels, notification counts, and color variants",
            Border::rounded().title(" Badges ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Badge variants:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(Badge::new("v2.52"))
                            .child(Badge::new("stable").variant(BadgeVariant::Success))
                            .child(Badge::new("beta").variant(BadgeVariant::Warning))
                            .child(Badge::new("alpha").variant(BadgeVariant::Error))
                            .child(Badge::new("new").variant(BadgeVariant::Primary)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("With counts:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(Text::new("Notifications"))
                            .child(Badge::new("5").variant(BadgeVariant::Error))
                            .child(Text::new("Messages"))
                            .child(Badge::new("99").variant(BadgeVariant::Primary))
                            .child(Text::new("Errors"))
                            .child(Badge::new("0").variant(BadgeVariant::Success)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Version/status labels").fg(muted))
                    .child(Text::new("• Notification counts").fg(muted))
                    .child(Text::new("• Color variants").fg(muted)),
            ),
        ),
        Example::new(
            "Tags",
            "Category labels with custom colors and removable option",
            Border::rounded().title(" Tags ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Technology tags:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(Tag::new("rust"))
                            .child(Tag::new("tui"))
                            .child(Tag::new("async")),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Removable tags:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(Tag::new("featured"))
                            .child(Tag::new("urgent"))
                            .child(Tag::new("review")),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Colored tags:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(Tag::new("bug").color(error))
                            .child(Tag::new("feature").color(success))
                            .child(Tag::new("docs").color(info)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Category labels").fg(muted))
                    .child(Text::new("• Removable option").fg(muted))
                    .child(Text::new("• Custom colors").fg(muted)),
            ),
        ),
        Example::new(
            "Avatars",
            "User representation with initial letters and status",
            Border::rounded().title(" Avatars ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Initial avatars:").fg(primary))
                    .child(
                        hstack()
                            .gap(2)
                            .child(avatar("Alice"))
                            .child(avatar("Bob"))
                            .child(avatar("Charlie"))
                            .child(avatar("Diana")),
                    )
                    .child(Text::new(""))
                    .child(Text::new("With status:").fg(primary))
                    .child(
                        hstack()
                            .gap(2)
                            .child(avatar_with_status("Alice", true))
                            .child(avatar_with_status("Bob", false))
                            .child(avatar_with_status("Charlie", true)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• User representation").fg(muted))
                    .child(Text::new("• Initial letters").fg(muted))
                    .child(Text::new("• Status indicators").fg(muted)),
            ),
        ),
        Example::new(
            "Status Indicators",
            "Online/offline states and service health indicators",
            Border::rounded().title(" Status Indicators ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Online status:").fg(primary))
                    .child(
                        hstack()
                            .gap(2)
                            .child(online())
                            .child(Text::new("Online").fg(text))
                            .child(offline())
                            .child(Text::new("Offline").fg(text))
                            .child(away_indicator())
                            .child(Text::new("Away").fg(text))
                            .child(busy_indicator())
                            .child(Text::new("Busy").fg(text)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Service status:").fg(primary))
                    .child(
                        vstack()
                            .gap(1)
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(StatusIndicator::online())
                                    .child(Text::new("API Server: Running").fg(text)),
                            )
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(StatusIndicator::new(Status::Away))
                                    .child(Text::new("Database: High load").fg(text)),
                            )
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(StatusIndicator::new(Status::Error))
                                    .child(Text::new("Cache: Disconnected").fg(text)),
                            ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Online/offline states").fg(muted))
                    .child(Text::new("• Service health").fg(muted))
                    .child(Text::new("• Color-coded status").fg(muted)),
            ),
        ),
        Example::new(
            "Combined",
            "User cards combining avatars, badges, and tags",
            Border::rounded().title(" Combined ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("User cards:").fg(primary))
                    .child(
                        vstack()
                            .gap(1)
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(avatar_with_status("John", true))
                                    .child(
                                        vstack()
                                            .gap(0)
                                            .child(Text::new("John Doe").bold().fg(text))
                                            .child(
                                                hstack()
                                                    .gap(1)
                                                    .child(
                                                        Badge::new("Admin")
                                                            .variant(BadgeVariant::Primary),
                                                    )
                                                    .child(Tag::new("team-lead")),
                                            ),
                                    ),
                            )
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(avatar_with_status("Jane", true))
                                    .child(
                                        vstack()
                                            .gap(0)
                                            .child(Text::new("Jane Smith").bold().fg(text))
                                            .child(
                                                hstack()
                                                    .gap(1)
                                                    .child(
                                                        Badge::new("Dev")
                                                            .variant(BadgeVariant::Success),
                                                    )
                                                    .child(Tag::new("rust")),
                                            ),
                                    ),
                            ),
                    ),
            ),
        ),
    ]
}

fn avatar(name: &str) -> impl View {
    let colors = [
        Color::rgb(50, 100, 200),
        Color::rgb(40, 160, 80),
        Color::rgb(180, 60, 180),
        Color::rgb(60, 160, 180),
        Color::rgb(200, 150, 40),
    ];
    let color = colors[name
        .chars()
        .next()
        .map(|c| (c as usize) % colors.len())
        .unwrap_or(0)];
    let initial = name.chars().next().unwrap_or('?');
    Text::new(format!(" {} ", initial))
        .bg(color)
        .fg(Color::rgb(255, 255, 255))
        .bold()
}

fn avatar_with_status(name: &str, is_online: bool) -> impl View {
    hstack()
        .gap(0)
        .child(avatar(name))
        .child(
            Text::new(if is_online { "●" } else { "○" }).fg(if is_online {
                Color::rgb(40, 160, 80)
            } else {
                Color::rgb(200, 60, 60)
            }),
        )
}
