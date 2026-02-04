use revue::widget::presentation;
#[test]
fn test_presentation_add_slide() {
    let mut p = presentation();
    p.add_slide("new slide".to_string());
}
