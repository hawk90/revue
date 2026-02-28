//! Tree widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{FileTree, Tree, TreeNode};

pub fn render() -> impl View {
    let (_primary, _success, _warning, _error, _info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Tree ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Tree::new()
                                    .node(
                                        TreeNode::new("src")
                                            .expanded(true)
                                            .child(TreeNode::new("widget"))
                                            .child(TreeNode::new("style"))
                                            .child(TreeNode::new("lib.rs")),
                                    )
                                    .node(TreeNode::new("Cargo.toml"))
                                    .node(TreeNode::new("README.md")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Expandable nodes").fg(muted))
                            .child(Text::new("• Hierarchical data").fg(muted))
                            .child(Text::new("• Collapse/expand").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" File Tree ").child(
                        vstack()
                            .gap(1)
                            .child(FileTree::new())
                            .child(Text::new(""))
                            .child(Text::new("• File/folder icons").fg(muted))
                            .child(Text::new("• Directory nesting").fg(muted))
                            .child(Text::new("• File explorer view").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Org Chart ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Tree::new().node(
                                    TreeNode::new("CEO")
                                        .expanded(true)
                                        .child(
                                            TreeNode::new("CTO")
                                                .child(TreeNode::new("Engineering"))
                                                .child(TreeNode::new("DevOps")),
                                        )
                                        .child(
                                            TreeNode::new("CFO")
                                                .child(TreeNode::new("Finance"))
                                                .child(TreeNode::new("Accounting")),
                                        ),
                                ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Organizational view").fg(muted))
                            .child(Text::new("• Reporting structure").fg(muted))
                            .child(Text::new("• Employee hierarchy").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" JSON Tree ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Tree::new().node(
                                    TreeNode::new("{ } root")
                                        .expanded(true)
                                        .child(TreeNode::new("\"name\": \"revue\""))
                                        .child(TreeNode::new("\"version\": \"2.52.0\""))
                                        .child(
                                            TreeNode::new("\"dependencies\": { }")
                                                .child(TreeNode::new("\"ratatui\": \"0.29\""))
                                                .child(TreeNode::new("\"tokio\": \"1.0\"")),
                                        )
                                        .child(TreeNode::new(
                                            "\"features\": [\"full\", \"async\"]",
                                        )),
                                ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• JSON visualization").fg(muted))
                            .child(Text::new("• Expand objects").fg(muted))
                            .child(Text::new("• Key/value pairs").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Checklist Tree ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Tree::new()
                                    .node(
                                        TreeNode::new("☑ Phase 1: Setup")
                                            .child(TreeNode::new("☑ Initialize project"))
                                            .child(TreeNode::new("☑ Configure CI")),
                                    )
                                    .node(
                                        TreeNode::new("◐ Phase 2: Development")
                                            .expanded(true)
                                            .child(TreeNode::new("☑ Core features"))
                                            .child(TreeNode::new("☐ Testing"))
                                            .child(TreeNode::new("☐ Documentation")),
                                    )
                                    .node(
                                        TreeNode::new("☐ Phase 3: Release")
                                            .child(TreeNode::new("☐ Review"))
                                            .child(TreeNode::new("☐ Deploy")),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Progress tracking").fg(muted))
                            .child(Text::new("• Nested tasks").fg(muted))
                            .child(Text::new("• Checkbox states").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Category Tree ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Tree::new()
                                    .node(
                                        TreeNode::new("📁 Documents")
                                            .expanded(true)
                                            .child(TreeNode::new("📄 Reports"))
                                            .child(TreeNode::new("📄 Spreadsheets")),
                                    )
                                    .node(
                                        TreeNode::new("📁 Images")
                                            .child(TreeNode::new("🖼 Photos"))
                                            .child(TreeNode::new("🎨 Graphics")),
                                    )
                                    .node(
                                        TreeNode::new("📁 Code")
                                            .child(TreeNode::new("⚙ Rust"))
                                            .child(TreeNode::new("⚙ TypeScript")),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Category browser").fg(muted))
                            .child(Text::new("• Emoji icons").fg(muted))
                            .child(Text::new("• Navigation tree").fg(muted)),
                    ),
                ),
        )
}
