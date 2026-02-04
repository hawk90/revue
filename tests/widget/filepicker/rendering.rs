use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::filepicker;

#[test]
fn test_filepicker_render() {
    let picker = filepicker(".");
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}
