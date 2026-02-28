//! Skeleton loading placeholder widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{
    skeleton, skeleton_avatar, skeleton_paragraph, skeleton_text, Skeleton, SkeletonShape,
};

pub fn render() -> impl View {
    let (primary, _success, _warning, _error, _info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded()
                        .title(" Skeleton Basics ")
                        .min_width(35)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Basic skeleton:").fg(primary))
                                .child(skeleton().width(30).height(1))
                                .child(Text::new(""))
                                .child(Text::new("Different sizes:").fg(primary))
                                .child(skeleton().width(40).height(1))
                                .child(skeleton().width(30).height(1))
                                .child(skeleton().width(20).height(1))
                                .child(Text::new(""))
                                .child(Text::new("• Loading placeholder").fg(muted))
                                .child(Text::new("• Animated shimmer").fg(muted))
                                .child(Text::new("• Content-aware sizing").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Skeleton Text ")
                        .min_width(40)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Text line skeletons:").fg(primary))
                                .child(skeleton_text().lines(3).width(40))
                                .child(Text::new(""))
                                .child(Text::new("Single line:").fg(primary))
                                .child(skeleton_text().lines(1).width(35))
                                .child(Text::new(""))
                                .child(Text::new("Long paragraph:").fg(primary))
                                .child(skeleton_paragraph().width(45))
                                .child(Text::new(""))
                                .child(Text::new("• Multiple lines").fg(muted))
                                .child(Text::new("• Variable width").fg(muted))
                                .child(Text::new("• Last line shorter").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Skeleton Avatar ")
                        .min_width(30)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Avatar placeholders:").fg(primary))
                                .child(
                                    hstack()
                                        .gap(2)
                                        .child(skeleton_avatar().width(3).height(3))
                                        .child(skeleton_avatar().width(4).height(4))
                                        .child(skeleton_avatar().width(5).height(5)),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Circular:").fg(primary))
                                .child(
                                    skeleton_avatar()
                                        .shape(SkeletonShape::Circle)
                                        .width(5)
                                        .height(5),
                                )
                                .child(Text::new(""))
                                .child(Text::new("• Multiple sizes").fg(muted))
                                .child(Text::new("• Shape variants").fg(muted))
                                .child(Text::new("• Profile loading").fg(muted)),
                        ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded()
                        .title(" Card Skeleton ")
                        .min_width(40)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Loading card layout:").fg(primary))
                                .child(
                                    Border::rounded().child(
                                        vstack()
                                            .gap(1)
                                            .child(
                                                hstack()
                                                    .gap(2)
                                                    .child(skeleton_avatar().width(3).height(3))
                                                    .child(
                                                        vstack()
                                                            .gap(0)
                                                            .child(skeleton().width(20).height(1))
                                                            .child(skeleton().width(15).height(1)),
                                                    ),
                                            )
                                            .child(skeleton().width(40).height(1))
                                            .child(skeleton().width(35).height(1))
                                            .child(skeleton().width(38).height(1)),
                                    ),
                                )
                                .child(Text::new(""))
                                .child(Text::new("• Composition pattern").fg(muted))
                                .child(Text::new("• Matching real layout").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" List Skeleton ")
                        .min_width(35)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Loading list items:").fg(primary))
                                .child(
                                    vstack()
                                        .gap(1)
                                        .child(
                                            hstack()
                                                .gap(2)
                                                .child(skeleton_avatar().width(2).height(2))
                                                .child(skeleton().width(25).height(1)),
                                        )
                                        .child(
                                            hstack()
                                                .gap(2)
                                                .child(skeleton_avatar().width(2).height(2))
                                                .child(skeleton().width(30).height(1)),
                                        )
                                        .child(
                                            hstack()
                                                .gap(2)
                                                .child(skeleton_avatar().width(2).height(2))
                                                .child(skeleton().width(20).height(1)),
                                        )
                                        .child(
                                            hstack()
                                                .gap(2)
                                                .child(skeleton_avatar().width(2).height(2))
                                                .child(skeleton().width(28).height(1)),
                                        ),
                                )
                                .child(Text::new(""))
                                .child(Text::new("• Repeated patterns").fg(muted))
                                .child(Text::new("• Consistent spacing").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Skeleton Shapes ")
                        .min_width(35)
                        .min_height(12)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Rectangle:").fg(primary))
                                .child(
                                    Skeleton::new()
                                        .shape(SkeletonShape::Rectangle)
                                        .width(30)
                                        .height(3),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Circle:").fg(primary))
                                .child(
                                    skeleton_avatar()
                                        .shape(SkeletonShape::Circle)
                                        .width(5)
                                        .height(5),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Paragraph:").fg(primary))
                                .child(Skeleton::new().shape(SkeletonShape::Paragraph).width(35))
                                .child(Text::new(""))
                                .child(Text::new("• Rectangle (default)").fg(muted))
                                .child(Text::new("• Circle (avatars)").fg(muted))
                                .child(Text::new("• Paragraph (text)").fg(muted)),
                        ),
                ),
        )
}
