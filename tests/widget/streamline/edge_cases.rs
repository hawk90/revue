use revue::widget::streamline;

#[test]
fn test_streamline_empty_text() {
    let stream = streamline("");
}

#[test]
fn test_streamline_long_text() {
    let long = "x".repeat(1000);
    let stream = streamline(&long);
}
