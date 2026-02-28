//! Diff widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::DiffViewer;

pub fn render() -> impl View {
    let (primary, success, _warning, error, info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(render_diff_viewer(&primary, &error, &success, &muted))
                .child(
                    Border::rounded().title(" Git Diff ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Commit: abc1234").fg(primary))
                            .child(Text::new("Author: Alice").fg(muted))
                            .child(Text::new("Message: Update main function").fg(text))
                            .child(Text::new(""))
                            .child(Text::new("- let x = 1;").fg(error))
                            .child(Text::new("+ let x = 2;").fg(success))
                            .child(Text::new("  println!(\"{}\", x);").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• Git integration").fg(muted))
                            .child(Text::new("• Commit info").fg(muted))
                            .child(Text::new("• File changes").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Line Numbers ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("With line numbers:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("10 - old line 10").fg(error))
                            .child(Text::new("10 + new line 10").fg(success))
                            .child(Text::new(""))
                            .child(Text::new("• Line reference").fg(muted))
                            .child(Text::new("• +/- indicators").fg(muted))
                            .child(Text::new("• Jump to line").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Inline Diff ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Word-level changes:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("- The quick brown fox jumps").fg(error))
                            .child(Text::new("+ The quick brown cat leaps").fg(success))
                            .child(Text::new("  over the lazy dog.").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• Word diff").fg(muted))
                            .child(Text::new("• Character diff").fg(muted))
                            .child(Text::new("• Highlight changes").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Change Summary ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("File statistics:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("Files changed: 5").fg(primary))
                            .child(Text::new("Insertions: +120").fg(success))
                            .child(Text::new("Deletions: -45").fg(error))
                            .child(Text::new(""))
                            .child(Text::new("src/main.rs: +30 -10").fg(muted))
                            .child(Text::new("src/lib.rs: +50 -20").fg(muted))
                            .child(Text::new("Cargo.toml: +5 -5").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• File count").fg(muted))
                            .child(Text::new("• +/- lines").fg(muted))
                            .child(Text::new("• Per-file stats").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Merge Conflict ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Conflict markers:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("<<<<<<< HEAD").fg(error))
                            .child(Text::new("fn main() { println!(\"A\"); }").fg(text))
                            .child(Text::new("=======").fg(muted))
                            .child(Text::new("fn main() { println!(\"B\"); }").fg(text))
                            .child(Text::new(">>>>>>> branch").fg(info))
                            .child(Text::new(""))
                            .child(Text::new("• Conflict display").fg(muted))
                            .child(Text::new("• Choose resolution").fg(muted))
                            .child(Text::new("• Accept/reject").fg(muted)),
                    ),
                ),
        )
}

#[cfg(feature = "diff")]
fn render_diff_viewer(_primary: &Color, _error: &Color, _success: &Color, muted: &Color) -> Border {
    Border::rounded().title(" Diff Viewer ").child(
        vstack()
            .gap(1)
            .child(DiffViewer::new())
            .child(Text::new(""))
            .child(Text::new("• Side-by-side diff").fg(*muted))
            .child(Text::new("• Unified diff").fg(*muted))
            .child(Text::new("• Syntax highlight").fg(*muted)),
    )
}

#[cfg(not(feature = "diff"))]
fn render_diff_viewer(primary: &Color, error: &Color, success: &Color, muted: &Color) -> Border {
    Border::rounded().title(" Diff Viewer ").child(
        vstack()
            .gap(1)
            .child(Text::new("Enable 'diff' feature").fg(*muted))
            .child(Text::new("to see diff viewer.").fg(*muted))
            .child(Text::new(""))
            .child(Text::new("• Side-by-side diff").fg(*muted))
            .child(Text::new("• Unified diff").fg(*muted))
            .child(Text::new("• Syntax highlight").fg(*muted)),
    )
}
