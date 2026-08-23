//! Paint-level checks for the navigation and notification widgets.
//!
//! Asserting against the buffer, not the source - the ratchet cannot tell a
//! color that was resolved from one that was resolved and dropped.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{crumb, Breadcrumb, Pagination, Toast};

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 40, 8).dom_from_render(true);
    h.draw(view);
    h
}

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

macro_rules! wrap {
    ($name:ident, $build:expr) => {
        struct $name;
        impl View for $name {
            fn render(&self, ctx: &mut RenderContext) {
                vstack().child($build).render(ctx);
            }
            fn widget_type(&self) -> &'static str {
                stringify!($name)
            }
            fn id(&self) -> Option<&str> {
                Some("root")
            }
        }
    };
}

wrap!(
    BreadcrumbView,
    Breadcrumb::new()
        .item(crumb("home"))
        .item(crumb("docs"))
        .element_id("w")
);
wrap!(PaginationView, Pagination::new(10).element_id("w"));
wrap!(ToastView, Toast::info("saved").element_id("w"));

#[test]
fn color_reaches_a_breadcrumbs_items() {
    let h = draw("#w { color: #ff0000; }", &BreadcrumbView);
    assert!(any_fg(&h, RED), "`color` did not reach Breadcrumb");
}

#[test]
fn color_reaches_paginations_inactive_pages() {
    let h = draw("#w { color: #ff0000; }", &PaginationView);
    assert!(any_fg(&h, RED), "`color` did not reach Pagination");
}

#[test]
fn color_reaches_a_toast() {
    let h = draw("#w { color: #ff0000; }", &ToastView);
    assert!(any_fg(&h, RED), "`color` did not reach Toast");
}

/// Without a rule each keeps its own palette.
#[test]
fn they_keep_their_own_colors_by_default() {
    assert!(!any_fg(&draw("", &BreadcrumbView), RED));
    assert!(!any_fg(&draw("", &PaginationView), RED));
    assert!(!any_fg(&draw("", &ToastView), RED));
}
