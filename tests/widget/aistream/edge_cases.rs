//! AIStream edge cases
use revue::widget::aistream;

#[test]
fn test_aistream_empty_prompt() {
    let stream = aistream("");
    assert_eq!(stream.prompt(), "");
}

#[test]
fn test_aistream_long_prompt() {
    let long_prompt = "x".repeat(1000);
    let stream = aistream(&long_prompt);
    assert!(stream.prompt().len() == 1000);
}
