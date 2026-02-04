//! AIStream basic tests
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::aistream;

#[test]
fn test_aistream_new() {
    let stream = aistream("test prompt");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}

#[test]
fn test_aistream_default() {
    let stream = revue::widget::AIStream::default();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}
