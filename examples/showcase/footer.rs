//! Footer rendering for the showcase

use revue::prelude::*;

pub fn render_footer(example_index: usize, example_total: usize) -> impl View {
    let (_, _, _, _, _, muted, _, _) = theme_colors();

    hstack()
        .gap(3)
        .child(
            Text::new(format!(
                "[↑/↓] Example {}/{}",
                example_index + 1,
                example_total
            ))
            .fg(muted),
        )
        .child(Text::new("[←/→] Sub-tab").fg(muted))
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
