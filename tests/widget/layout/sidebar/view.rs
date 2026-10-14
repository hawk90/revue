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
    for (title, nodes) in s {
        if let Some(t) = title {
            out.push((Row::Title(t.clone()), None));
        }
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
        // Widths of 0 and 1 panic: a draft issue of the sidebar audit
        expanded in 2u16..30,
        collapsed in 2u16..8,
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
        // The indicator and icon are placed without checking the bar's
        // width, so a deep row in a narrow bar draws past it: a draft
        // issue of the sidebar audit. Check "nothing right of the bar"
        // only when no drawn row reaches past it.
        let mut overflows = false;
        for y in 0..h {
            prop_assert_eq!(buffer.get(cw - 1, y).unwrap().symbol, '│', "border y={}", y);
            let expected_bg = if y >= top && y < bottom {
                match rows.get((y - top) as usize) {
                    Some((Row::Item(id, depth, _), Some(n))) => {
                        let x_icon = if is_collapsed { 1 } else { 3 + 2 * *depth as u16 };
                        let indicator_out =
                            !is_collapsed && !n.children.is_empty() && 1 + 2 * *depth as u16 >= cw;
                        if indicator_out || n.icon && x_icon >= cw {
                            overflows = true;
                        }
                        // A badge as wide as the bar starts at column 0
                        // and covers the row's first cell.
                        let badge_at_0 = !is_collapsed
                            && n.badge.as_ref().is_some_and(|b| b.len() as u16 + 2 >= cw);
                        if badge_at_0 {
                            continue;
                        }
                        if n.disabled {
                            BAR_BG
                        } else if selected.as_deref() == Some(id.as_str()) {
                            Color::BLUE
                        } else if (y - top) as usize == hovered {
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
        if header {
            prop_assert_eq!(buffer.get(0, 1).unwrap().symbol, '─');
            // A footer in a short bar draws its separator over the header.
            if !is_collapsed && cw >= 6 && (!footer || h >= 4) {
                prop_assert!(super::row(&buffer, 0).contains("Head"));
            }
        }
        if footer {
            prop_assert_eq!(buffer.get(0, h - 2).unwrap().symbol, '─');
            if !is_collapsed && cw >= 6 {
                prop_assert!(super::row(&buffer, h - 1).contains("Foot"));
            }
        }
        if !overflows {
            for y in 0..h {
                for x in cw..w {
                    let c = buffer.get(x, y).unwrap();
                    prop_assert_eq!((c.symbol, c.bg), (' ', None), "({}, {})", x, y);
                }
            }
        }
    }
}
