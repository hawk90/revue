//! Helper Function tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    sidebar, sidebar_item, sidebar_section, sidebar_section_titled, CollapseMode, FlattenedItem,
    Sidebar, SidebarItem, SidebarSection,
};

#[test]
fn test_sidebar_helper() {
    let sb = sidebar()
        .header("Test")
        .items(vec![sidebar_item("home", "Home").icon('🏠')]);

    assert_eq!(sb.item_count(), 1);
}

#[test]
fn test_sidebar_section_helpers() {
    let sb = sidebar()
        .section(sidebar_section_titled(
            "Navigation",
            vec![sidebar_item("home", "Home")],
        ))
        .section(sidebar_section(vec![sidebar_item("other", "Other")]));

    assert_eq!(sb.item_count(), 2);
}
