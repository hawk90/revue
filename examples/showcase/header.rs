//! Header rendering for the showcase

use crate::theme_colors;
use crate::MainTab;
use revue::prelude::*;

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
        .child(Text::new(format!("│ [1-7] Tabs")).fg(muted))
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

        tabs = tabs.child(if is_active {
            Text::new(label).bold().fg(primary)
        } else {
            Text::new(label).fg(muted)
        });

        // Add separator except for last
        if i < sub_tabs.len() - 1 {
            tabs = tabs.child(Text::new("│").fg(muted));
        }
    }
    tabs
}
