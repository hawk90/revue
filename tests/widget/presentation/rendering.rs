use revue::layout::Rect; use revue::render::Buffer; use revue::widget::traits::RenderContext;
use revue::widget::presentation;
#[test]
fn test_presentation_render() {
    let p = presentation(); let mut b = Buffer::new(40, 10); let a = Rect::new(0,0,40,10);
    let mut ctx = RenderContext::new(&mut b, a); p.render(&mut ctx);
}
