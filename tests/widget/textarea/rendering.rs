use revue::layout::Rect; use revue::render::Buffer; use revue::widget::traits::RenderContext;
use revue::widget::textarea;
#[test] fn test_textarea_render() {
    let t = textarea(); let mut b = Buffer::new(40, 10); let a = Rect::new(0,0,40,10);
    let mut ctx = RenderContext::new(&mut b, a); t.render(&mut ctx);
}
