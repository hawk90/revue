use revue::layout::Rect; use revue::render::Buffer; use revue::widget::traits::RenderContext;
use revue::widget::input;
#[test] fn test_input_render() {
    let i = input(); let mut b = Buffer::new(30, 5); let a = Rect::new(0,0,30,5);
    let mut ctx = RenderContext::new(&mut b, a); i.render(&mut ctx);
}
