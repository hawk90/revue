use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::streamline;

#[test]
fn test_streamline_render() {
    let stream = streamline("test text").speed(2.0);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stream.render(&mut ctx);
}
