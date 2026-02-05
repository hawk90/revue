use revue::layout::Rect; use revue::render::Buffer; use revue::widget::traits::RenderContext;
use revue::widget::callout;
#[test] fn test_callout_render() {
    let c = callout("test"); let mut b = Buffer::new(30, 10); let a = Rect::new(0,0,30,10);
    let mut ctx = RenderContext::new(&mut b, a); c.render(&mut ctx);
}
