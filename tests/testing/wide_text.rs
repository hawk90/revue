//! The screen read back as text with wide glyphs on it: a glyph is one
//! character, and a found position is a column (#850)

use revue::testing::{Pilot, PipelineHarness, RenderCapture, TestApp};
use revue::widget::Text;

/// "你好 OK": the glyphs take columns 0-3, the space 4, "OK" 5-6
const LINE: &str = "你好 OK";

#[test]
fn test_app_reads_wide_glyphs_as_one_character() {
    let mut app = TestApp::with_size(Text::new(LINE), 12, 2);
    assert_eq!(app.get_line(0), LINE);
    assert_eq!(app.screen_text(), LINE);
    assert!(app.contains("你好"));
    assert_eq!(app.find_text("OK"), Some((5, 0)));
    assert_eq!(app.find_text("好"), Some((2, 0)));
    assert_eq!(app.find_text("missing"), None);

    let pilot = Pilot::new(&mut app);
    pilot.assert_contains("你好 OK");
    assert_eq!(pilot.find_text("OK"), Some((5, 0)));
}

#[test]
fn render_capture_reads_wide_glyphs_as_one_character() {
    let mut capture = RenderCapture::new(12, 3);
    capture.buffer_mut().put_str(0, 1, LINE);
    assert_eq!(capture.line(1), LINE);
    assert_eq!(capture.text(), format!("\n{LINE}"));
    assert!(capture.contains("你好"));
    assert_eq!(capture.find("OK"), Some((5, 1)));
}

#[test]
fn the_pipeline_reads_wide_glyphs_as_one_character() {
    let mut h = PipelineHarness::new(12, 2);
    h.draw(&Text::new(LINE));
    assert_eq!(h.screen_text(), LINE);
    assert!(h.contains("你好"));
}
