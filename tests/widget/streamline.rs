//! Streamline widget integration tests
//!
//! Streamline 위젯(스트림 그래프/ThemeRiver)에 대한 통합 테스트

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{StyledView, View};
use revue::widget::{
    genre_stream, resource_stream, streamline, streamline_with_data, traffic_stream, StreamBaseline,
    StreamLayer, StreamOrder, Streamline,
};

// =============================================================================
// Constructor Tests (생성자 테스트)
// =============================================================================

#[test]
fn test_streamline_new() {
    let chart = Streamline::new();
    // Verify it can be rendered without panicking
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_streamline_default() {
    let chart = Streamline::default();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_streamline_helper() {
    let chart = streamline();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

// =============================================================================
// StreamLayer Tests (스트림 레이어 테스트)
// =============================================================================

#[test]
fn test_stream_layer_new() {
    let layer = StreamLayer::new("Test Layer");
    assert_eq!(layer.name, "Test Layer");
    assert!(layer.values.is_empty());
    assert!(layer.color.is_none());
}

#[test]
fn test_stream_layer_data() {
    let layer = StreamLayer::new("Test").data(vec![1.0, 2.0, 3.0]);
    assert_eq!(layer.values, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_stream_layer_color() {
    let layer = StreamLayer::new("Test").color(Color::RED);
    assert_eq!(layer.color, Some(Color::RED));
}

#[test]
fn test_stream_layer_builder_chain() {
    let layer = StreamLayer::new("Sales")
        .data(vec![10.0, 20.0, 30.0])
        .color(Color::BLUE);

    assert_eq!(layer.name, "Sales");
    assert_eq!(layer.values, vec![10.0, 20.0, 30.0]);
    assert_eq!(layer.color, Some(Color::BLUE));
}

// =============================================================================
// Builder Methods Tests (빌더 메서드 테스트)
// =============================================================================

#[test]
fn test_streamline_title() {
    let chart = streamline().title("Monthly Sales");
    // Verify rendering with title
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Check title appears
    let mut found_title = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'M' {
                found_title = true;
                break;
            }
        }
    }
    assert!(found_title);
}

#[test]
fn test_streamline_title_string() {
    let chart = streamline().title(String::from("Revenue"));
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_streamline_layer_single() {
    let layer = StreamLayer::new("A").data(vec![1.0, 2.0]);
    let chart = streamline().layer(layer);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Should render content
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_layers_multiple() {
    let layers = vec![
        StreamLayer::new("A").data(vec![1.0, 2.0]),
        StreamLayer::new("B").data(vec![3.0, 4.0]),
        StreamLayer::new("C").data(vec![5.0, 6.0]),
    ];
    let chart = streamline().layers(layers);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Should render stacked layers
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_baseline_zero() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]))
        .baseline(StreamBaseline::Zero);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Should render with zero baseline
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_baseline_symmetric() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]))
        .baseline(StreamBaseline::Symmetric);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_baseline_wiggle() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]))
        .baseline(StreamBaseline::Wiggle);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_baseline_expand() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]))
        .layer(StreamLayer::new("B").data(vec![5.0, 10.0, 15.0]))
        .baseline(StreamBaseline::Expand);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_order_none() {
    let chart = streamline()
        .layer(StreamLayer::new("Small").data(vec![5.0]))
        .layer(StreamLayer::new("Big").data(vec![100.0]))
        .order(StreamOrder::None);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_order_ascending() {
    let chart = streamline()
        .layer(StreamLayer::new("Big").data(vec![100.0]))
        .layer(StreamLayer::new("Small").data(vec![10.0]))
        .order(StreamOrder::Ascending);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_order_descending() {
    let chart = streamline()
        .layer(StreamLayer::new("Small").data(vec![10.0]))
        .layer(StreamLayer::new("Big").data(vec![100.0]))
        .order(StreamOrder::Descending);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_order_inside_out() {
    let chart = streamline()
        .layer(StreamLayer::new("Small").data(vec![10.0]))
        .layer(StreamLayer::new("Medium").data(vec![50.0]))
        .layer(StreamLayer::new("Big").data(vec![100.0]))
        .order(StreamOrder::InsideOut);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_show_legend_true() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
        .show_legend(true);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Legend should be shown
    let mut has_legend = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' && cell.fg.is_some() {
                    has_legend = true;
                    break;
                }
            }
        }
    }
    assert!(has_legend);
}

#[test]
fn test_streamline_show_legend_false() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
        .show_legend(false);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Chart should still render
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_show_labels_true() {
    let chart = streamline()
        .layer(StreamLayer::new("Sales").data(vec![10.0, 20.0, 30.0]))
        .show_labels(true);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Labels should be rendered on the stream
    let mut found_label = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'S' {
                    found_label = true;
                    break;
                }
            }
        }
    }
    assert!(found_label);
}

#[test]
fn test_streamline_show_labels_false() {
    let chart = streamline()
        .layer(StreamLayer::new("Sales").data(vec![10.0, 20.0]))
        .show_labels(false);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Chart should still render
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_x_labels() {
    let labels = vec!["Jan".to_string(), "Feb".to_string(), "Mar".to_string()];
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]))
        .x_labels(labels);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // X-axis labels should be rendered at bottom
    let mut found_label = false;
    for y in (area.height - 3)..area.height {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'J' || cell.symbol == 'F' || cell.symbol == 'M' {
                    found_label = true;
                    break;
                }
            }
        }
    }
    assert!(found_label);
}

#[test]
fn test_streamline_bg() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
        .bg(Color::BLACK);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Background color should be set
    if let Some(cell) = buffer.get(0, 0) {
        assert_eq!(cell.bg, Some(Color::BLACK));
    }
}

#[test]
fn test_streamline_height() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
        .height(5);

    let mut buffer = Buffer::new(40, 15);
    let area = Rect::new(0, 0, 40, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Area below height should be empty
    for y in 6..15 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if y >= 5 {
                    assert_eq!(cell.symbol, ' ', "Area below height should be empty");
                }
            }
        }
    }
}

#[test]
fn test_streamline_palette() {
    let colors = vec![Color::RED, Color::GREEN, Color::BLUE];
    let chart = streamline()
        .palette(colors)
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
        .layer(StreamLayer::new("B").data(vec![5.0, 15.0]))
        .layer(StreamLayer::new("C").data(vec![15.0, 25.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Should use custom palette colors
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' && cell.fg.is_some() {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_highlight() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
        .layer(StreamLayer::new("B").data(vec![5.0, 15.0]))
        .layer(StreamLayer::new("C").data(vec![15.0, 25.0]))
        .highlight(1);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Highlighted layer should be rendered
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_builder_chain() {
    let chart = streamline()
        .title("Test Chart")
        .baseline(StreamBaseline::Expand)
        .order(StreamOrder::Descending)
        .show_legend(false)
        .show_labels(false)
        .x_labels(vec!["A".to_string(), "B".to_string()])
        .bg(Color::BLACK)
        .height(20)
        .highlight(0)
        .layer(StreamLayer::new("Layer1").data(vec![1.0, 2.0]));

    let mut buffer = Buffer::new(40, 25);
    let area = Rect::new(0, 0, 40, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Verify background is set
    if let Some(cell) = buffer.get(0, 0) {
        assert_eq!(cell.bg, Some(Color::BLACK));
    }

    // Verify title appears
    let mut found_title = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'T' {
                found_title = true;
                break;
            }
        }
    }
    assert!(found_title);

    // Verify content
    let mut has_content = false;
    for y in 0..25 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

// =============================================================================
// Convenience Constructors Tests (편의 생성자 테스트)
// =============================================================================

#[test]
fn test_streamline_with_data_empty() {
    let chart = streamline_with_data(vec![]);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_streamline_with_data_single() {
    let layers = vec![StreamLayer::new("A").data(vec![1.0, 2.0])];
    let chart = streamline_with_data(layers);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_with_data_multiple() {
    let layers = vec![
        StreamLayer::new("A").data(vec![1.0, 2.0]),
        StreamLayer::new("B").data(vec![3.0, 4.0]),
    ];
    let chart = streamline_with_data(layers);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_genre_stream() {
    let data = vec![
        ("Rock", vec![10.0, 20.0, 30.0]),
        ("Pop", vec![5.0, 15.0, 25.0]),
        ("Jazz", vec![3.0, 8.0, 12.0]),
    ];
    let chart = genre_stream(data);

    let mut buffer = Buffer::new(50, 12);
    let area = Rect::new(0, 0, 50, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Verify title appears
    let mut found_title = false;
    for x in 0..50 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'M' {
                found_title = true;
                break;
            }
        }
    }
    assert!(found_title, "Title 'Music Genre Trends' should be rendered");

    // Verify content
    let mut has_content = false;
    for y in 0..12 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_genre_stream_empty() {
    let chart = genre_stream(vec![]);
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_traffic_stream() {
    let data = vec![
        ("Direct", vec![100.0, 150.0, 200.0]),
        ("Organic", vec![200.0, 250.0, 300.0]),
        ("Referral", vec![50.0, 75.0, 100.0]),
    ];
    let chart = traffic_stream(data);

    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Verify title appears
    let mut found_title = false;
    for x in 0..50 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'T' {
                found_title = true;
                break;
            }
        }
    }
    assert!(found_title, "Title 'Traffic Sources' should be rendered");

    // Verify content
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_traffic_stream_empty() {
    let chart = traffic_stream(vec![]);
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_resource_stream() {
    let chart = resource_stream(
        vec![10.0, 20.0, 30.0],  // CPU
        vec![40.0, 50.0, 60.0],  // Memory
        vec![5.0, 10.0, 15.0],   // Disk
        vec![15.0, 25.0, 35.0],  // Network
    );

    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);

    // Verify title appears
    let mut found_title = false;
    for x in 0..50 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'R' {
                found_title = true;
                break;
            }
        }
    }
    assert!(found_title, "Title 'Resource Usage' should be rendered");

    // Verify content
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

// =============================================================================
// Render Tests - Edge Cases (엣지 케이스 렌더링 테스트)
// =============================================================================

#[test]
fn test_streamline_render_empty() {
    let chart = streamline();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
    // Should not panic with empty chart
}

#[test]
fn test_streamline_render_single_layer() {
    let chart = streamline().layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]));
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should render some content
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content, "Chart should render content");
}

#[test]
fn test_streamline_render_multiple_layers() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]))
        .layer(StreamLayer::new("B").data(vec![5.0, 15.0, 25.0]))
        .layer(StreamLayer::new("C").data(vec![15.0, 25.0, 35.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should render stacked layers
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_render_with_legend() {
    let chart = streamline()
        .layer(StreamLayer::new("Sales").data(vec![10.0, 20.0]))
        .layer(StreamLayer::new("Marketing").data(vec![5.0, 15.0]))
        .show_legend(true);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Legend should show layer names and colors
    let mut found_legend = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' && cell.fg.is_some() {
                    found_legend = true;
                    break;
                }
            }
        }
    }
    assert!(found_legend, "Legend should be rendered with colored blocks");
}

#[test]
fn test_streamline_render_with_x_labels() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]))
        .x_labels(vec!["Jan".to_string(), "Feb".to_string(), "Mar".to_string()]);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // X-axis labels should be rendered at bottom
    let mut found_label = false;
    for y in (area.height - 3)..area.height {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'J' || cell.symbol == 'F' || cell.symbol == 'M' {
                    found_label = true;
                    break;
                }
            }
        }
    }
    assert!(found_label, "X-axis labels should be rendered");
}

#[test]
fn test_streamline_render_with_background() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
        .bg(Color::BLACK);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Background color should be set
    if let Some(cell) = buffer.get(0, 0) {
        assert_eq!(cell.bg, Some(Color::BLACK));
    }
}

#[test]
fn test_streamline_render_with_custom_height() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]))
        .height(5);

    let mut buffer = Buffer::new(40, 15);
    let area = Rect::new(0, 0, 40, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Area below height should be empty
    for y in 6..15 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if y >= 5 {
                    assert_eq!(cell.symbol, ' ', "Area below height should be empty");
                }
            }
        }
    }
}

#[test]
fn test_streamline_render_with_custom_colors() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]).color(Color::RED))
        .layer(StreamLayer::new("B").data(vec![5.0, 15.0]).color(Color::GREEN))
        .layer(StreamLayer::new("C").data(vec![15.0, 25.0]).color(Color::BLUE));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should have different colors
    let mut has_red = false;
    let mut has_green = false;
    let mut has_blue = false;

    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                match cell.fg {
                    Some(Color::RED) => has_red = true,
                    Some(Color::GREEN) => has_green = true,
                    Some(Color::BLUE) => has_blue = true,
                    _ => {}
                }
            }
        }
    }

    assert!(has_red || has_green || has_blue, "Should have custom colors");
}

// =============================================================================
// Edge Cases - Rendering (엣지 케이스 렌더링 테스트)
// =============================================================================

#[test]
fn test_streamline_render_too_small_width() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]));

    let mut buffer = Buffer::new(3, 10);
    let area = Rect::new(0, 0, 3, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should not crash with very small width
}

#[test]
fn test_streamline_render_too_small_height() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]));

    let mut buffer = Buffer::new(40, 2);
    let area = Rect::new(0, 0, 40, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should not crash with very small height
}

#[test]
fn test_streamline_render_zero_area() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0]));

    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should not crash with zero area
}

#[test]
fn test_streamline_render_single_data_point() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should render single point
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_render_two_data_points() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10.0, 20.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should render two points
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_render_many_data_points() {
    let data: Vec<f64> = (0..100).map(|i| i as f64).collect();
    let chart = streamline()
        .layer(StreamLayer::new("A").data(data));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should render many points
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_render_zero_values() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![0.0, 0.0, 0.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should handle zero values without crashing
}

#[test]
fn test_streamline_render_negative_values() {
    // Note: Streamline doesn't support negative values in the traditional sense
    // but should handle them gracefully
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![-10.0, -20.0, -30.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should handle without crashing
}

#[test]
fn test_streamline_render_mixed_values() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![0.0, 10.0, 50.0, 100.0, 25.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should handle mixed value ranges
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_render_very_large_values() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![10000.0, 20000.0, 30000.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should scale large values appropriately
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_render_very_small_values() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![0.001, 0.002, 0.003]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should handle very small values
}

#[test]
fn test_streamline_render_varying_length_layers() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![1.0, 2.0, 3.0, 4.0, 5.0]))
        .layer(StreamLayer::new("B").data(vec![10.0, 20.0]))
        .layer(StreamLayer::new("C").data(vec![100.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should handle layers with different data lengths
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_render_empty_layer_values() {
    let chart = streamline()
        .layer(StreamLayer::new("A").data(vec![]))
        .layer(StreamLayer::new("B").data(vec![10.0, 20.0]));

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should skip empty layer and render the other
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_render_empty_layer_name() {
    let chart = streamline()
        .layer(StreamLayer::new("").data(vec![10.0, 20.0]))
        .show_labels(true);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should not crash with empty layer name
}

// =============================================================================
// Complex Scenario Tests (복잡한 시나리오 테스트)
// =============================================================================

#[test]
fn test_streamline_complex_scenario() {
    // 복잡한 스트림 그래프 시나리오
    let chart = streamline()
        .title("Website Traffic Sources")
        .baseline(StreamBaseline::Expand)
        .order(StreamOrder::Descending)
        .show_legend(true)
        .show_labels(true)
        .x_labels(vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
        ])
        .bg(Color::BLACK)
        .height(12)
        .layer(
            StreamLayer::new("Organic Search")
                .data(vec![1000.0, 1200.0, 1100.0, 1300.0, 1400.0])
                .color(Color::rgb(52, 152, 219)),
        )
        .layer(
            StreamLayer::new("Direct")
                .data(vec![500.0, 550.0, 600.0, 650.0, 700.0])
                .color(Color::rgb(46, 204, 113)),
        )
        .layer(
            StreamLayer::new("Social Media")
                .data(vec![300.0, 400.0, 350.0, 450.0, 500.0])
                .color(Color::rgb(155, 89, 182)),
        )
        .layer(
            StreamLayer::new("Referral")
                .data(vec![200.0, 150.0, 180.0, 220.0, 250.0])
                .color(Color::rgb(241, 196, 15)),
        )
        .highlight(0);

    let mut buffer = Buffer::new(60, 15);
    let area = Rect::new(0, 0, 60, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Verify title
    let mut found_title = false;
    for x in 0..60 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'W' {
                found_title = true;
                break;
            }
        }
    }
    assert!(found_title, "Title should be rendered");

    // Verify chart content
    let mut has_content = false;
    for y in 0..15 {
        for x in 0..60 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content, "Chart should render content");

    // Verify x-axis labels
    let mut found_label = false;
    for y in (area.height - 3)..area.height {
        for x in 0..60 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'M' || cell.symbol == 'T' || cell.symbol == 'W' {
                    found_label = true;
                    break;
                }
            }
        }
    }
    assert!(found_label, "X-axis labels should be rendered");
}

#[test]
fn test_streamline_music_genre_scenario() {
    let data = vec![
        ("Rock", vec![30.0, 35.0, 32.0, 28.0, 25.0]),
        ("Pop", vec![40.0, 38.0, 42.0, 45.0, 48.0]),
        ("Jazz", vec![10.0, 12.0, 11.0, 13.0, 14.0]),
        ("Classical", vec![15.0, 14.0, 13.0, 12.0, 11.0]),
        ("Electronic", vec![5.0, 8.0, 10.0, 12.0, 15.0]),
    ];

    let chart = genre_stream(data);

    let mut buffer = Buffer::new(50, 12);
    let area = Rect::new(0, 0, 50, 12);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should render music genre stream
    let mut has_content = false;
    for y in 0..12 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

#[test]
fn test_streamline_resource_monitoring_scenario() {
    let chart = resource_stream(
        vec![45.0, 50.0, 55.0, 60.0, 58.0],  // CPU
        vec![70.0, 72.0, 75.0, 78.0, 80.0],  // Memory
        vec![20.0, 22.0, 25.0, 28.0, 30.0],  // Disk
        vec![15.0, 18.0, 20.0, 22.0, 25.0],  // Network
    );

    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should render resource usage
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);

    // Verify layer colors
    let mut has_colored_content = false;
    for y in 0..10 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' && cell.fg.is_some() {
                    has_colored_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_colored_content);
}

#[test]
fn test_streamline_traffic_analysis_scenario() {
    let data = vec![
        ("Organic", vec![1000.0, 1200.0, 1100.0]),
        ("Paid", vec![500.0, 600.0, 550.0]),
        ("Social", vec![300.0, 400.0, 350.0]),
        ("Email", vec![200.0, 250.0, 225.0]),
    ];

    let chart = traffic_stream(data);

    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(0, 0, 50, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Should render traffic stream with expand baseline
    let mut has_content = false;
    for y in 0..10 {
        for x in 0..50 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '█' {
                    has_content = true;
                    break;
                }
            }
        }
    }
    assert!(has_content);
}

// =============================================================================
// All Rendering Modes Test (모든 렌더링 모드 테스트)
// =============================================================================

#[test]
fn test_streamline_all_baselines() {
    let baselines = [
        StreamBaseline::Zero,
        StreamBaseline::Symmetric,
        StreamBaseline::Wiggle,
        StreamBaseline::Expand,
    ];

    for baseline in baselines {
        let chart = streamline()
            .baseline(baseline)
            .layer(StreamLayer::new("A").data(vec![10.0, 20.0, 30.0]))
            .layer(StreamLayer::new("B").data(vec![5.0, 15.0, 25.0]));

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);

        // Each baseline should render
        let mut has_content = false;
        for y in 0..10 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '█' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(
            has_content,
            "Baseline {:?} should render content",
            baseline
        );
    }
}

#[test]
fn test_streamline_all_orders() {
    let orders = [
        StreamOrder::None,
        StreamOrder::Ascending,
        StreamOrder::Descending,
        StreamOrder::InsideOut,
    ];

    for order in orders {
        let chart = streamline()
            .order(order)
            .layer(StreamLayer::new("A").data(vec![10.0]))
            .layer(StreamLayer::new("B").data(vec![50.0]))
            .layer(StreamLayer::new("C").data(vec![30.0]));

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        chart.render(&mut ctx);

        // Each order should render
        let mut has_content = false;
        for y in 0..10 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '█' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content, "Order {:?} should render content", order);
    }
}

// =============================================================================
// CSS/Styling Tests (CSS 스타일링 테스트)
// =============================================================================

#[test]
fn test_streamline_css_id() {
    let chart = streamline().element_id("my-streamline");
    assert_eq!(View::id(&chart), Some("my-streamline"));

    let meta = chart.meta();
    assert_eq!(meta.id, Some("my-streamline".to_string()));
}

#[test]
fn test_streamline_css_classes() {
    let chart = streamline().class("chart").class("interactive");

    assert!(chart.has_class("chart"));
    assert!(chart.has_class("interactive"));
    assert!(!chart.has_class("static"));

    let meta = chart.meta();
    assert!(meta.classes.contains("chart"));
    assert!(meta.classes.contains("interactive"));
}

#[test]
fn test_streamline_classes_builder() {
    let chart = streamline().classes(vec!["chart", "animated", "stream"]);

    assert!(chart.has_class("chart"));
    assert!(chart.has_class("animated"));
    assert!(chart.has_class("stream"));
    assert_eq!(View::classes(&chart).len(), 3);
}

#[test]
fn test_streamline_duplicate_class_not_added() {
    let chart = streamline().class("test").class("test");

    let classes = View::classes(&chart);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_streamline_meta() {
    let chart = streamline()
        .element_id("test-streamline")
        .class("visualization")
        .class("real-time");

    let meta = chart.meta();
    assert_eq!(meta.widget_type, "Streamline");
    assert_eq!(meta.id, Some("test-streamline".to_string()));
    assert!(meta.classes.contains("visualization"));
    assert!(meta.classes.contains("real-time"));
}
