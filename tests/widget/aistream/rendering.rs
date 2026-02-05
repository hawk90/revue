//! AIStream rendering tests
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::aistream;

#[test]
fn test_aistream_render_with_text() {
    let mut stream = aistream("test").text("output");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_aistream_render_small_buffer() {
    let stream = aistream("test");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}
