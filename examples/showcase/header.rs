//! Header rendering for the showcase

use crate::theme_colors;
use crate::MainTab;
use revue::prelude::*;
use revue::utils::unicode::display_width;

pub fn render_header(frame: u64, _active_main_tab: MainTab) -> impl View {
    let (primary, _, _, _, _, muted, _, _) = theme_colors();
    let theme = use_theme().get();
    let time = format!(
        "{:02}:{:02}:{:02}",
        (frame / 3600) % 24,
        (frame / 60) % 60,
        frame % 60
    );

    hstack()
        .gap(2)
        .child(Text::new(" REVUE SHOWCASE ").bold().fg(primary))
        .child(Text::new(format!("│ Theme: {}", theme.name)).fg(muted))
        .child(Text::new(format!("│ {}", time)).fg(muted))
        .child(Text::new("│ [1-7] Tabs").fg(muted))
}

pub fn render_main_tabs(active_main_tab: MainTab) -> impl View {
    let (primary, _, _, _, _, muted, _, _) = theme_colors();

    let mut tabs = hstack().gap(1);
    for tab in MainTab::ALL.iter() {
        let is_active = *tab == active_main_tab;
        let label = format!("[{}] {}", tab.key(), tab.name());

        tabs = tabs.child(if is_active {
            Text::new(label).bold().fg(primary)
        } else {
            Text::new(label).fg(muted)
        });
    }
    tabs
}

pub fn render_sub_tabs(sub_tabs: &[crate::SubTab], active_sub_tab: usize) -> impl View {
    let (primary, _, _, _, _, muted, _, _) = theme_colors();

    let mut tabs = hstack().gap(1);
    for (i, sub_tab) in sub_tabs.iter().enumerate() {
        let is_active = i == active_sub_tab;
        let label = sub_tab.name().to_string();
        // Each label and separator is sized to its text.
        let width = display_width(&label) as u16;

        tabs = tabs.child_sized(
            if is_active {
                Text::new(label).bold().fg(primary)
            } else {
                Text::new(label).fg(muted)
            },
            width,
        );

        // Add separator except for last
        if i < sub_tabs.len() - 1 {
            tabs = tabs.child_sized(Text::new("│").fg(muted), 1);
        }
    }
    tabs
}

pub fn render_example_header(
    title: &str,
    description: &str,
    index: usize,
    total: usize,
) -> impl View {
    let (primary, _, _, _, _, muted, _, _) = theme_colors();

    let heading = format!("▸ Example {}/{}: {}", index + 1, total, title);
    let width = display_width(&heading) as u16;

    // Size the heading to its text so the description follows it directly.
    hstack()
        .gap(2)
        .child_sized(Text::new(heading).bold().fg(primary), width)
        .child(Text::new(format!("— {}", description)).fg(muted))
}
