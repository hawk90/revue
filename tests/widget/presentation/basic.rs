use revue::widget::presentation;
#[test]
fn test_presentation_new() { let p = presentation(); }
#[test]
fn test_presentation_slides() { 
    let p = presentation().slides(vec!["slide1".to_string()]);
}
