use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::code_editor;

#[test]
fn test_code_editor_render() {
    let editor = code_editor().text("test");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    editor.render(&mut ctx);
}
