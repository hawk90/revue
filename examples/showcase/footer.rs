//! Footer rendering for the showcase

use crate::SubTab;
use revue::prelude::*;

pub fn render_footer(sub_tabs: &[SubTab], active_sub_tab: usize) -> impl View {
    let (_, _, _, _, _, muted, _, _) = theme_colors();

    let current_sub = sub_tabs.get(active_sub_tab).map(|s| s.name()).unwrap_or("");
    let prev_sub = if active_sub_tab > 0 {
        sub_tabs[active_sub_tab - 1].name()
    } else {
        sub_tabs.last().map(|s| s.name()).unwrap_or("")
    };
    let next_sub = sub_tabs
        .get(active_sub_tab + 1)
        .map(|s| s.name())
        .unwrap_or_else(|| sub_tabs.first().map(|s| s.name()).unwrap_or(""));

    hstack()
        .gap(3)
        .child(
            Text::new(format!(
                "[←/→] {} │ {} │ {}",
                prev_sub, current_sub, next_sub
            ))
            .fg(muted),
        )
        .child(Text::new("[t] theme").fg(muted))
        .child(Text::new("[q] quit").fg(muted))
        .child(Text::new("│ 92+ widgets").fg(muted))
}

fn theme_colors() -> (Color, Color, Color, Color, Color, Color, Color, Color) {
    let t = use_theme().get();
    (
        t.palette.primary,
        t.palette.success,
        t.palette.warning,
        t.palette.error,
        t.palette.info,
        t.colors.text_muted,
        t.colors.text,
        t.colors.surface,
    )
}
