use revue::widget::text;
#[test] fn test_text_new() { let t = text("hello"); }
#[test] fn test_text_content() { let t = text("test"); assert_eq!(t.content(), "test"); }
