use revue::layout::Rect; use revue::render::Buffer; use revue::widget::traits::RenderContext;
use revue::widget::filetree;
#[test] fn test_filetree_render() {
    let t = filetree("."); let mut b = Buffer::new(30, 20); let a = Rect::new(0,0,30,20);
    let mut ctx = RenderContext::new(&mut b, a); t.render(&mut ctx);
}
