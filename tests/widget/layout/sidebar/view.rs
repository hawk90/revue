//! Sidebar on the screen: placement, the stylesheet, edge cases, and
//! proptests of the hover and expand model and of each drawn row

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::{Color, Style, VisualStyle};
use revue::widget::traits::RenderContext;
use revue::widget::{CollapseMode, FlattenedItem, Sidebar, SidebarItem, SidebarSection, View};

use super::{render_rows, screen, BAR_BG, HOVER_BG};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

/// Header, a titled and an untitled section, a nested item, a badge, a
/// disabled item, and a footer
fn sample() -> Sidebar {
    Sidebar::new()
        .header("App")
        .section(SidebarSection::titled(
            "Main",
            vec![
                SidebarItem::new("home", "Home").icon('H').badge("3"),
                SidebarItem::new("docs", "Docs")
                    .expanded(true)
                    .children(vec![SidebarItem::new("api", "API")]),
            ],
        ))
        .items(vec![SidebarItem::new("off", "Off").disabled(true)])
        .selected("api")
        .footer("v1")
}

#[test]
fn test_sidebar_zero_areas_draw_nothing() {
    assert_zero_areas_draw_nothing(&sample());
    assert_zero_areas_draw_nothing(&sample().collapse_mode(CollapseMode::Collapsed));
}

#[test]
fn test_sidebar_same_when_offset() {
    assert_same_when_offset(sample(), 24, 10);
    assert_same_when_offset(sample().collapse_mode(CollapseMode::Collapsed), 8, 6);
    assert_same_when_offset(sample().collapse_mode(CollapseMode::Auto), 12, 6);
}

#[test]
fn test_sidebar_plain_rows_take_the_stylesheet_color() {
    let style = Style {
        visual: VisualStyle {
            color: Color::GREEN,
            ..VisualStyle::default()
        },
        ..Default::default()
    };
    let draw = |sb: &Sidebar| {
        let mut buffer = Buffer::new(24, 4);
        let mut ctx = RenderContext::with_style(&mut buffer, Rect::new(0, 0, 24, 4), &style);
        sb.render(&mut ctx);
        buffer
    };
    let sb = Sidebar::new()
        .items(vec![
            SidebarItem::new("a", "A"),
            SidebarItem::new("b", "B"),
            SidebarItem::new("c", "C"),
        ])
        .selected("c");
    let buffer = draw(&sb);
    // Row 0 is hovered and row 2 selected: they keep their own text color.
    assert_eq!(buffer.get(3, 0).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(3, 1).unwrap().fg, Some(Color::GREEN));
    assert_eq!(buffer.get(3, 2).unwrap().fg, Some(Color::WHITE));
    // A color set on the sidebar wins over the stylesheet.
    let buffer = draw(&sb.fg(Color::YELLOW));
    assert_eq!(buffer.get(3, 1).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_sidebar_long_label_stops_before_the_badge() {
    let sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "An extremely long label").badge("12"),
        SidebarItem::new("b", "Short label without a badge that runs on"),
    ]);
    let rows = render_rows(&sb, 24, 2);
    assert_eq!(rows[0], "   An extremely lon 12 │");
    // Without a badge the label runs to the column before the border.
    assert_eq!(rows[1], "   Short label without │");
}

#[test]
fn test_sidebar_collapsed_hides_titles_and_indent() {
    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .header("Header")
        .section(SidebarSection::titled(
            "Title",
            vec![SidebarItem::new("p", "P")
                .icon('P')
                .expanded(true)
                .children(vec![SidebarItem::new("c", "C").icon('C')])],
        ));
    let rows = render_rows(&sb, 6, 5);
    // The header keeps its row and separator, the title its row, both
    // without text; children are not indented and have no ▼.
    assert_eq!(rows, ["   │  ", "───│  ", "   │  ", " P │  ", " C │  "]);
}

#[test]
fn test_sidebar_hover_on_a_title_row_highlights_nothing() {
    let sb = Sidebar::new().section(SidebarSection::titled(
        "Main",
        vec![SidebarItem::new("a", "A")],
    ));
    // hovered_index 0 is the title row, so no item is hovered yet.
    assert_eq!(sb.hovered_index(), 0);
    let buffer = screen(&sb, 24, 3);
    assert_eq!(buffer.get(3, 1).unwrap().bg, Some(BAR_BG));
    let mut sb = sb;
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 1);
    assert_eq!(screen(&sb, 24, 3).get(3, 1).unwrap().bg, Some(HOVER_BG));
}

// A bar narrower than the two columns its text margins need (#1032)

#[test]
fn test_sidebar_expanded_width_1_draws() {
    let sb = Sidebar::new()
        .expanded_width(1)
        .header("App")
        .items(vec![SidebarItem::new("a", "A")]);
    let buffer = screen(&sb, 10, 4);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '│');
}

#[test]
fn test_sidebar_expanded_width_0_draws_nothing() {
    let sb = Sidebar::new()
        .expanded_width(0)
        .items(vec![SidebarItem::new("a", "A")]);
    let buffer = screen(&sb, 10, 4);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_sidebar_collapsed_width_0_draws_nothing() {
    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .collapsed_width(0);
    let buffer = screen(&sb, 10, 4);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_sidebar_toggle_item_toggles_one_item_per_id() {
    // Two trees hold "dup", one nested: toggling expands the first one only.
    let mut sb = Sidebar::new()
        .items(vec![SidebarItem::new("p", "P").expanded(true).children(
            vec![SidebarItem::new("dup", "Nested").children(vec![SidebarItem::new("n1", "N1")])],
        )])
        .items(vec![
            SidebarItem::new("dup", "Top").children(vec![SidebarItem::new("t1", "T1")])
        ]);
    sb.toggle_item("dup");
    let ids: Vec<String> = sb
        .visible_items()
        .into_iter()
        .filter_map(|f| match f {
            FlattenedItem::Item { item, .. } => Some(item.id),
            FlattenedItem::Section(_) => None,
        })
        .collect();
    assert_eq!(ids, ["p", "dup", "n1", "dup"]);
}

// =========================================================================
// Proptests
// =========================================================================

/// A plain model of one item: what the widget's tree should hold
#[derive(Clone, Debug)]
struct Node {
    id: String,
    disabled: bool,
    expanded: bool,
    icon: bool,
    badge: Option<String>,
    children: Vec<Node>,
}

/// Item shapes without ids; ids are numbered in [`number`]
fn shape() -> impl Strategy<Value = Node> {
    let leaf = (
        prop::bool::weighted(0.25),
        any::<bool>(),
        any::<bool>(),
        prop::option::weighted(0.3, "[0-9]{1,3}"),
    )
        .prop_map(|(disabled, expanded, icon, badge)| Node {
            id: String::new(),
            disabled,
            expanded,
            icon,
            badge,
            children: vec![],
        });
    leaf.prop_recursive(3, 24, 3, |inner| {
        (
            prop::bool::weighted(0.25),
            prop::bool::weighted(0.7),
            any::<bool>(),
            prop::collection::vec(inner, 1..4),
        )
            .prop_map(|(disabled, expanded, icon, children)| Node {
                id: String::new(),
                disabled,
                expanded,
                icon,
                badge: None,
                children,
            })
    })
}

type Sections = Vec<(Option<String>, Vec<Node>)>;

fn sections() -> impl Strategy<Value = Sections> {
    prop::collection::vec(
        (
            prop::option::of("[A-Z][a-z]{0,8}"),
            prop::collection::vec(shape(), 0..4),
        ),
        0..4,
    )
    .prop_map(|mut s| {
        let mut n = 0;
        for (_, nodes) in &mut s {
            for node in nodes {
                number(node, &mut n);
            }
        }
        s
    })
}

fn number(node: &mut Node, n: &mut usize) {
    node.id = format!("i{n}");
    *n += 1;
    for c in &mut node.children {
        number(c, n);
    }
}

fn to_item(node: &Node) -> SidebarItem {
    let mut item = SidebarItem::new(node.id.clone(), format!("Label {}", node.id))
        .disabled(node.disabled)
        .expanded(node.expanded)
        .children(node.children.iter().map(to_item).collect());
    if node.icon {
        item = item.icon('*');
    }
    if let Some(b) = &node.badge {
        item = item.badge(b.clone());
    }
    item
}

fn build(s: &Sections) -> Sidebar {
    Sidebar::new().sections(
        s.iter()
            .map(|(title, nodes)| {
                let items = nodes.iter().map(to_item).collect();
                match title {
                    Some(t) => SidebarSection::titled(t.clone(), items),
                    None => SidebarSection::new(items),
                }
            })
            .collect(),
    )
}

/// One visible row of the model: a section title, or an item and its depth
#[derive(Clone, Debug, PartialEq)]
enum Row {
    Title(String),
    Item(String, usize, bool),
}

fn flatten(s: &Sections) -> Vec<(Row, Option<Node>)> {
    fn walk(node: &Node, depth: usize, out: &mut Vec<(Row, Option<Node>)>) {
        out.push((
            Row::Item(node.id.clone(), depth, node.expanded),
            Some(node.clone()),
        ));
        if node.expanded {
            for c in &node.children {
                walk(c, depth + 1, out);
            }
        }
    }
    let mut out = vec![];
    // An untitled section is divided from a non-empty one before it (#1033)
    let mut items_before = false;
    for (title, nodes) in s {
        if let Some(t) = title {
            out.push((Row::Title(t.clone()), None));
        } else if !nodes.is_empty() && items_before {
            out.push((Row::Title(String::new()), None));
        }
        items_before |= !nodes.is_empty();
        for n in nodes {
            walk(n, 0, &mut out);
        }
    }
    out
}

fn widget_rows(sb: &Sidebar) -> Vec<Row> {
    sb.visible_items()
        .into_iter()
        .map(|f| match f {
            FlattenedItem::Section(t) => Row::Title(t.unwrap_or_default()),
            FlattenedItem::Item { item, depth } => Row::Item(item.id, depth, item.expanded),
        })
        .collect()
}

fn find_mut<'a>(nodes: &'a mut [Node], id: &str) -> Option<&'a mut Node> {
    for n in nodes {
        if n.id == id {
            return Some(n);
        }
        if let Some(f) = find_mut(&mut n.children, id) {
            return Some(f);
        }
    }
    None
}

fn set_all(nodes: &mut [Node], expanded: bool) {
    for n in nodes {
        n.expanded = expanded;
        set_all(&mut n.children, expanded);
    }
}

#[derive(Clone, Debug)]
enum Op {
    Down,
    Up,
    Select,
    ToggleHovered,
    ToggleItem(usize),
    ExpandAll,
    CollapseAll,
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        4 => Just(Op::Down),
        3 => Just(Op::Up),
        2 => Just(Op::Select),
        2 => Just(Op::ToggleHovered),
        2 => (0usize..30).prop_map(Op::ToggleItem),
        1 => Just(Op::ExpandAll),
        1 => Just(Op::CollapseAll),
    ]
}

proptest! {
    /// Any operations leave the tree, the visible rows, the selection and
    /// the hover a plain model gives. Hover moves are checked from the
    /// widget's current index; where the index points after a toggle is
    /// not (the index stays put while the rows around it change: #975).
    #[test]
    fn prop_sidebar_ops_follow_the_model(s in sections(), ops in prop::collection::vec(op(), 0..40)) {
        let mut model = s.clone();
        let mut sb = build(&s);
        let mut selected: Option<String> = None;
        for op in ops {
            let h = sb.hovered_index();
            let rows = flatten(&model);
            let hoverable: Vec<usize> = rows
                .iter()
                .enumerate()
                .filter(|(_, (_, n))| n.as_ref().is_some_and(|n| !n.disabled))
                .map(|(i, _)| i)
                .collect();
            match &op {
                Op::Down => {
                    sb.hover_down();
                    let want = match hoverable.iter().position(|&i| i == h) {
                        Some(p) => hoverable[(p + 1).min(hoverable.len() - 1)],
                        None => hoverable.first().copied().unwrap_or(h),
                    };
                    prop_assert_eq!(sb.hovered_index(), want);
                }
                Op::Up => {
                    sb.hover_up();
                    let want = match hoverable.iter().position(|&i| i == h) {
                        Some(p) => hoverable[p.saturating_sub(1)],
                        None => hoverable.last().copied().unwrap_or(h),
                    };
                    prop_assert_eq!(sb.hovered_index(), want);
                }
                Op::Select => {
                    sb.select_hovered();
                    if let Some((_, Some(n))) = rows.get(h) {
                        if !n.disabled {
                            selected = Some(n.id.clone());
                        }
                    }
                }
                Op::ToggleHovered => {
                    sb.toggle_hovered();
                    if let Some((_, Some(n))) = rows.get(h) {
                        if !n.children.is_empty() {
                            let id = n.id.clone();
                            for (_, nodes) in &mut model {
                                if let Some(m) = find_mut(nodes, &id) {
                                    m.expanded = !m.expanded;
                                    break;
                                }
                            }
                        }
                    }
                }
                Op::ToggleItem(k) => {
                    let id = format!("i{k}");
                    sb.toggle_item(&id);
                    for (_, nodes) in &mut model {
                        if let Some(m) = find_mut(nodes, &id) {
                            m.expanded = !m.expanded;
                            break;
                        }
                    }
                }
                Op::ExpandAll => {
                    sb.expand_all();
                    for (_, nodes) in &mut model {
                        set_all(nodes, true);
                    }
                }
                Op::CollapseAll => {
                    sb.collapse_all();
                    for (_, nodes) in &mut model {
                        set_all(nodes, false);
                    }
                }
            }
            let rows: Vec<Row> = flatten(&model).into_iter().map(|(r, _)| r).collect();
            prop_assert_eq!(widget_rows(&sb), rows.clone(), "after {:?}", op);
            prop_assert_eq!(
                sb.item_count(),
                rows.iter().filter(|r| matches!(r, Row::Item(..))).count()
            );
            prop_assert_eq!(sb.selected_id(), selected.as_deref());
        }
    }

    /// Each drawn row is in the colors its state gives (disabled, selected,
    /// hovered or plain), in a bar of the width the collapse mode gives,
    /// with the border on every row, the header and footer rows where they
    /// belong, and nothing right of the bar.
    #[test]
    fn prop_sidebar_rows_drawn_in_their_state_colors(
        s in sections(),
        downs in 0usize..12,
        pick in any::<prop::sample::Index>(),
        mode in prop_oneof![Just(CollapseMode::Expanded), Just(CollapseMode::Collapsed), Just(CollapseMode::Auto)],
        expanded in 0u16..30,
        collapsed in 0u16..8,
        threshold in 0u16..30,
        (w, h) in (3u16..36, 2u16..16),
        header in any::<bool>(),
        footer in any::<bool>(),
    ) {
        let rows = flatten(&s);
        let ids: Vec<String> = rows.iter().filter_map(|(_, n)| n.as_ref().map(|n| n.id.clone())).collect();
        let selected = (!ids.is_empty()).then(|| ids[pick.index(ids.len())].clone());
        let mut sb = build(&s)
            .collapse_mode(mode)
            .expanded_width(expanded)
            .collapsed_width(collapsed)
            .collapse_threshold(threshold);
        if let Some(id) = &selected {
            sb = sb.selected(id.clone());
        }
        if header {
            sb = sb.header("Head");
        }
        if footer {
            sb = sb.footer("Foot");
        }
        for _ in 0..downs {
            sb.hover_down();
        }
        let hovered = sb.hovered_index();
        let is_collapsed = match mode {
            CollapseMode::Expanded => false,
            CollapseMode::Collapsed => true,
            CollapseMode::Auto => w < threshold,
        };
        let cw = if is_collapsed { collapsed } else { expanded }.min(w);
        let buffer = screen(&sb, w, h);

        let top = if header { 2 } else { 0 };
        let bottom = h - if footer { 2 } else { 0 };
        // The item rows follow the hover (#882): one draw after the moves
        // scrolls from the top just far enough to show it. Nothing is
        // hovered until a move lands on an enabled item.
        let room = bottom.saturating_sub(top) as usize;
        let hover_set = downs > 0 && rows.iter().any(|(_, n)| n.as_ref().is_some_and(|n| !n.disabled));
        let offset = if hover_set && room > 0 {
            hovered.saturating_sub(room - 1).min(rows.len().saturating_sub(room))
        } else {
            0
        };
        for y in 0..h {
            if cw == 0 {
                break;
            }
            prop_assert_eq!(buffer.get(cw - 1, y).unwrap().symbol, '│', "border y={}", y);
            let expected_bg = if y >= top && y < bottom {
                match rows.get((y - top) as usize + offset) {
                    Some((Row::Item(id, _, _), Some(n))) => {
                        if n.disabled {
                            BAR_BG
                        } else if selected.as_deref() == Some(id.as_str()) {
                            Color::BLUE
                        } else if (y - top) as usize + offset == hovered {
                            HOVER_BG
                        } else {
                            BAR_BG
                        }
                    }
                    _ => BAR_BG,
                }
            } else {
                BAR_BG
            };
            if cw >= 2 {
                prop_assert_eq!(buffer.get(0, y).unwrap().bg, Some(expected_bg), "row {}", y);
            }
        }
        // A one-column bar is all border
        if header && cw > 1 {
            prop_assert_eq!(buffer.get(0, 1).unwrap().symbol, '─');
            // A footer in a short bar draws its separator over the header.
            if !is_collapsed && cw >= 6 && (!footer || h >= 4) {
                prop_assert!(super::row(&buffer, 0).contains("Head"));
            }
        }
        if footer && cw > 1 {
            prop_assert_eq!(buffer.get(0, h - 2).unwrap().symbol, '─');
            if !is_collapsed && cw >= 6 {
                prop_assert!(super::row(&buffer, h - 1).contains("Foot"));
            }
        }
        // Nothing right of the bar, however deep its rows (#1032)
        for y in 0..h {
            for x in cw..w {
                let c = buffer.get(x, y).unwrap();
                prop_assert_eq!((c.symbol, c.bg), (' ', None), "({}, {})", x, y);
            }
        }
    }
}

// ── Dividers between untitled sections (#1033) ────────────────────────

fn untitled(ids: &[&str]) -> SidebarSection {
    SidebarSection::new(
        ids.iter()
            .map(|id| SidebarItem::new(*id, id.to_uppercase()))
            .collect(),
    )
}

/// Two untitled sections in a row ran together; now a divider row separates
/// them, drawn as a line inside the bar's padding
#[test]
fn test_sidebar_untitled_section_draws_a_divider() {
    let sb = Sidebar::new()
        .expanded_width(10)
        .section(untitled(&["a"]))
        .section(untitled(&["b"]));
    let b = screen(&sb, 10, 4);
    assert!(super::row(&b, 0).contains('A'));
    assert_eq!(super::row(&b, 1), " ────────│");
    assert!(super::row(&b, 2).contains('B'));
}

/// Dividers go only between non-empty sections: none before the first,
/// none for an empty section, and a titled section keeps its title row
/// instead of a divider
#[test]
fn test_sidebar_dividers_only_between_non_empty_sections() {
    let rows = |sb: Sidebar| -> Vec<String> {
        sb.visible_items()
            .into_iter()
            .map(|f| match f {
                FlattenedItem::Section(None) => "---".to_string(),
                FlattenedItem::Section(Some(t)) => format!("[{t}]"),
                FlattenedItem::Item { item, .. } => item.id,
            })
            .collect()
    };
    // Leading empty section, then items: no divider before the first items
    let sb = Sidebar::new()
        .section(untitled(&[]))
        .section(untitled(&["a"]))
        .section(untitled(&[]))
        .section(untitled(&["b"]));
    assert_eq!(rows(sb), ["a", "---", "b"]);
    // Titled sections keep their titles; an untitled one after them gets a
    // divider, and one before them does not
    let sb = Sidebar::new()
        .section(untitled(&["a"]))
        .section(SidebarSection::titled(
            "T",
            vec![SidebarItem::new("t", "T")],
        ))
        .section(untitled(&["b"]));
    assert_eq!(rows(sb), ["a", "[T]", "t", "---", "b"]);
}

/// Hover skips a divider row and stays on its item when rows move
#[test]
fn test_sidebar_hover_skips_dividers() {
    let mut sb = Sidebar::new()
        .section(untitled(&["a"]))
        .section(untitled(&["b"]));
    // Row 1 is the divider; "b" is row 2. Nothing hovered reads as row 0,
    // so the first move down goes from "a" to "b"
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 2);
    sb.hover_up();
    assert_eq!(sb.hovered_index(), 0);
    sb.hover_up();
    assert_eq!(sb.hovered_index(), 0);
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 2);
}

// ── Scroll and hover (#882, #975) ──────────────────────────────────────

/// Nothing changed `scroll`, so hovering down moved the highlight past the
/// last drawn row (#882).
#[test]
fn test_sidebar_view_follows_the_hover() {
    let mut sb = Sidebar::new().items(
        (0..10)
            .map(|i| SidebarItem::new(format!("i{i}"), format!("Item {i}")))
            .collect(),
    );
    for _ in 0..7 {
        sb.hover_down();
    }
    assert_eq!(sb.hovered_index(), 7);
    let buffer = screen(&sb, 20, 4);
    let hovered: Vec<u16> = (0..4)
        .filter(|&y| buffer.get(0, y).unwrap().bg == Some(HOVER_BG))
        .collect();
    assert_eq!(hovered, [3]);
    assert!(super::row(&buffer, 3).contains("Item 7"));
}

#[test]
fn test_sidebar_collapsing_keeps_the_hover_off_other_items() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("p", "Parent")
            .expanded(true)
            .children(vec![SidebarItem::new("c", "Child")]),
        SidebarItem::new("q", "Other"),
    ]);
    sb.hover_down(); // "c"
    sb.toggle_item("p");
    sb.select_hovered();
    assert_eq!(
        sb.selected_id(),
        Some("p"),
        "the collapsed parent takes the hover"
    );
}

#[test]
fn test_sidebar_expanding_above_keeps_the_hovered_item() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("p", "Parent").children(vec![SidebarItem::new("c", "Child")]),
        SidebarItem::new("q", "Other"),
    ]);
    sb.hover_down(); // "q"
    sb.toggle_item("p");
    sb.select_hovered();
    assert_eq!(sb.selected_id(), Some("q")); // was "c"
}

#[test]
fn test_sidebar_collapse_all_keeps_a_hover() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("p", "Parent")
        .expanded(true)
        .children(vec![
            SidebarItem::new("c1", "C1"),
            SidebarItem::new("c2", "C2"),
        ])]);
    sb.hover_down();
    sb.hover_down(); // "c2", index 2
    sb.collapse_all();
    sb.select_hovered();
    assert_eq!(sb.selected_id(), Some("p")); // was None: index 2 is past the one visible row
}

// Widths the builders allow but the area guard does not see (#1032)

#[test]
fn test_sidebar_expanded_width_1_draws_only_its_border() {
    let sb = Sidebar::new()
        .expanded_width(1)
        .header("App")
        .items(vec![SidebarItem::new("a", "A")]);
    let buffer = screen(&sb, 10, 4);
    for y in 0..4 {
        assert_eq!(super::row(&buffer, y), "│         ", "row {y}");
    }
}

#[test]
fn test_sidebar_of_width_0_draws_nothing() {
    for sb in [
        Sidebar::new()
            .expanded_width(0)
            .items(vec![SidebarItem::new("a", "A")]),
        Sidebar::new()
            .collapse_mode(CollapseMode::Collapsed)
            .collapsed_width(0),
    ] {
        let buffer = screen(&sb, 10, 4);
        for y in 0..4 {
            assert_eq!(super::row(&buffer, y).trim(), "", "row {y}");
        }
    }
}

#[test]
fn test_sidebar_deep_rows_stay_inside_the_bar() {
    let sb = Sidebar::new()
        .expanded_width(5)
        .items(vec![SidebarItem::new("p", "P").expanded(true).children(
            vec![SidebarItem::new("c", "C")
                .icon('I')
                .expanded(true)
                .children(vec![
                    SidebarItem::new("g", "G").children(vec![SidebarItem::new("x", "X")])
                ])],
        )]);
    let buffer = screen(&sb, 12, 3);
    for y in 0..3 {
        for x in 5..12 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, ' ', "({x}, {y})"); // was 'I' at (5, 1)
        }
    }
}

#[test]
fn test_sidebar_wide_badge_leaves_the_indicator_and_the_border() {
    let sb = Sidebar::new()
        .expanded_width(8)
        .items(vec![SidebarItem::new("p", "P")
            .badge("12345")
            .children(vec![SidebarItem::new("c", "C")])]);
    let buffer = screen(&sb, 10, 2);
    // No room is left for the label; the badge starts after the indicator
    // and is cut before the blank column at the border
    assert_eq!(super::row(&buffer, 0), " ▶ 123 │  "); // was " 12345 │", over the indicator
}
