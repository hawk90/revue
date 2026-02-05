use revue::layout::Rect; use revue::render::Buffer; use revue::widget::traits::RenderContext;
use revue::widget::combobox;
#[test] fn test_combobox_render() {
    let c = combobox(); let mut b = Buffer::new(30, 10); let a = Rect::new(0,0,30,10);
    let mut ctx = RenderContext::new(&mut b, a); c.render(&mut ctx);
}
