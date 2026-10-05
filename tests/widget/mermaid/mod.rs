//! Mermaid diagram widget tests

mod helpers;
mod types;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{diagram, flowchart, node, Diagram, NodeShape, View};

fn render(d: &Diagram, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    d.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
        .collect()
}

fn is_blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| row(buffer, y).trim().is_empty())
}

#[test]
fn test_diagram_render_two_nodes_and_arrow() {
    // Two nodes stack in a 1-column grid: cells are 20x5
    let d = flowchart("A[Start] --> B[End]");
    let buffer = render(&d, 20, 10);
    assert_eq!(row(&buffer, 0).trim(), "");
    assert_eq!(row(&buffer, 1), "     ┌───────┐      ");
    assert_eq!(row(&buffer, 2), "     │ Start │      ");
    assert_eq!(row(&buffer, 3), "     └───────┘      ");
    // The arrow runs from below "Start" to just above "End"
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '│');
    assert_eq!(buffer.get(9, 5).unwrap().symbol, '▼');
    assert_eq!(row(&buffer, 6), "      ┌─────┐       ");
    assert_eq!(row(&buffer, 7), "      │ End │       ");
    assert_eq!(row(&buffer, 8), "      └─────┘       ");
}

#[test]
fn test_diagram_render_title() {
    let mut d = diagram().title("Flow").node(node("A", "Hi"));
    d.colors.title = Color::RED;
    let buffer = render(&d, 20, 8);
    assert_eq!(&row(&buffer, 0)[..4], "Flow");
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    // The title takes two rows; the 6 rows below hold the node
    assert_eq!(row(&buffer, 4), "       │ Hi │       ");
}

#[test]
fn test_diagram_render_edge_label() {
    let d = flowchart("A -->|go| B");
    let buffer = render(&d, 20, 10);
    assert_eq!(&row(&buffer, 5)[8..10], "go");
    assert_eq!(buffer.get(8, 5).unwrap().fg, Some(d.colors.label));
}

#[test]
fn test_diagram_render_node_colors_and_shapes() {
    let d = diagram()
        .node(node("A", "Hi").shape(NodeShape::Rounded).color(Color::RED))
        .node(node("B", "Yo").shape(NodeShape::Diamond));
    let buffer = render(&d, 20, 10);
    // Rounded box with the node's own color, label on the node background
    assert_eq!(row(&buffer, 1), "       ╭────╮       ");
    assert_eq!(buffer.get(7, 1).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(9, 2).unwrap().bg, Some(d.colors.node_bg));
    // Diamond is drawn as <label>
    assert_eq!(row(&buffer, 7), "       <Yo  >       ");
}

#[test]
fn test_diagram_render_too_small_draws_nothing() {
    let d = flowchart("A --> B");
    assert!(is_blank(&render(&d, 9, 10)));
    assert!(is_blank(&render(&d, 20, 4)));
}

#[test]
fn test_diagram_render_clips_long_label_to_its_box() {
    let label = "L".repeat(30);
    let d = diagram().node(node("A", label.as_str()));
    let buffer = render(&d, 20, 6);
    // The node shrinks to its 18-column grid cell; the label is cut to fit
    assert_eq!(row(&buffer, 1), format!(" ┌{}┐ ", "─".repeat(16)));
    assert_eq!(row(&buffer, 2), format!(" │{}│ ", "L".repeat(16)));
}

#[test]
fn test_diagram_render_crowded_area() {
    // Four nodes in 20x5: grid cells are 10x2, shorter than a node
    let d = flowchart("A --> B\nC --> D");
    let buffer = render(&d, 20, 5);
    assert_eq!(row(&buffer, 1), "  │ A │     │ B │   ");
}

#[test]
fn test_diagram_render_cells_too_narrow_for_a_box() {
    // Sixteen nodes in 10 columns: 2-column cells leave no room for a box
    let source: Vec<String> = (0..8).map(|i| format!("N{i} --> M{i}")).collect();
    let d = flowchart(&source.join("\n"));
    assert_eq!(d.get_nodes().len(), 16);
    assert!(is_blank(&render(&d, 10, 5)));
}
