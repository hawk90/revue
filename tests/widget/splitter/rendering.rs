use revue::layout::Rect; use revue::render::Buffer; use revue::widget::traits::RenderContext;
use revue::widget::splitter;
#[test]
fn test_splitter_render() {
    let s = splitter(); let mut b = Buffer::new(40, 10); let a = Rect::new(0,0,40,10);
    let mut ctx = RenderContext::new(&mut b, a); s.render(&mut ctx);
}
