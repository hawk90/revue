//! Rich content widget snapshot tests (Markdown, Syntax, TimeSeries, Waveline)

#![allow(unused_imports)]

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};

#[test]
#[cfg(feature = "markdown")]
fn test_markdown_basic() {
    use revue::widget::Markdown;

    let source = r#"# Hello World

This is a **bold** and *italic* text.

- Item 1
- Item 2
- Item 3

`inline code` and more text.
"#;

    let view = Markdown::new(source);

    let config = TestConfig::with_size(50, 15);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("markdown_basic");
}

#[test]
#[cfg(feature = "markdown")]
fn test_markdown_code_block() {
    use revue::widget::Markdown;

    let source = r#"## Code Example

```rust
fn main() {
    println!("Hello!");
}
```
"#;

    let view = Markdown::new(source);

    let config = TestConfig::with_size(50, 12);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("markdown_code");
}

#[test]
#[cfg(feature = "markdown")]
fn test_markdown_with_toc() {
    use revue::widget::Markdown;

    let source = r#"# Main Title

## Section 1
Content here.

## Section 2
More content.

### Subsection 2.1
Details.
"#;

    let view = Markdown::new(source).show_toc(true);

    let config = TestConfig::with_size(60, 15);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("markdown_toc");
}

// =============================================================================
// Syntax Highlighter Tests
// =============================================================================

#[test]
fn test_syntax_rust() {
    use revue::widget::{Language, SyntaxHighlighter};

    let _code = r#"fn main() {
    let x = 42;
    println!("{}", x);
}"#;

    let highlighter = SyntaxHighlighter::new(Language::Rust);
    // Just test that highlighting produces spans
    let spans = highlighter.highlight_line("fn main() {");
    assert!(!spans.is_empty());
}

#[test]
fn test_syntax_themes() {
    use revue::widget::{Language, SyntaxHighlighter, SyntaxTheme};

    let _dark = SyntaxHighlighter::new(Language::Rust).theme(SyntaxTheme::dark());
    let _light = SyntaxHighlighter::new(Language::Rust).theme(SyntaxTheme::light());
    let _monokai = SyntaxHighlighter::new(Language::Rust).theme(SyntaxTheme::monokai());
}

#[test]
fn test_syntax_languages() {
    use revue::widget::Language;

    assert_eq!(Language::from_extension("rs"), Language::Rust);
    assert_eq!(Language::from_extension("py"), Language::Python);
    assert_eq!(Language::from_extension("js"), Language::JavaScript);
    assert_eq!(Language::from_extension("ts"), Language::JavaScript);
    assert_eq!(Language::from_extension("go"), Language::Go);
}

// =============================================================================
// TimeSeries Widget Tests
// =============================================================================

#[test]
fn test_timeseries_basic() {
    use revue::style::Color;
    use revue::widget::{TimeSeries, TimeSeriesData};

    let data = TimeSeriesData::new("CPU")
        .point(0, 25.0)
        .point(1, 45.0)
        .point(2, 30.0)
        .point(3, 60.0)
        .point(4, 55.0)
        .color(Color::CYAN);

    let view = TimeSeries::new()
        .title("CPU Usage")
        .series(data)
        .show_legend(true);

    let config = TestConfig::with_size(60, 15);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("timeseries_basic");
}

#[test]
fn test_timeseries_multiple_series() {
    use revue::style::Color;
    use revue::widget::{TimeSeries, TimeSeriesData};

    let cpu = TimeSeriesData::new("CPU")
        .points(vec![(0, 20.0), (1, 40.0), (2, 35.0), (3, 50.0)])
        .color(Color::CYAN);

    let memory = TimeSeriesData::new("Memory")
        .points(vec![(0, 60.0), (1, 65.0), (2, 70.0), (3, 68.0)])
        .color(Color::MAGENTA);

    let view = TimeSeries::new()
        .title("System Metrics")
        .series(cpu)
        .series(memory)
        .y_label("Usage %")
        .show_grid(true);

    let config = TestConfig::with_size(70, 18);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("timeseries_multi");
}

// =============================================================================
// Waveline Widget Tests
// =============================================================================

#[test]
fn test_waveline_basic() {
    use revue::widget::Waveline;

    let data: Vec<f64> = (0..50)
        .map(|i| (i as f64 * 0.2).sin() * 0.4 + 0.5)
        .collect();
    let view = Waveline::new(data);

    let config = TestConfig::with_size(60, 10);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("waveline_basic");
}

#[test]
fn test_waveline_filled() {
    use revue::style::Color;
    use revue::widget::{WaveStyle, Waveline};

    let data: Vec<f64> = (0..40)
        .map(|i| (i as f64 * 0.15).sin() * 0.3 + 0.5)
        .collect();
    let view = Waveline::new(data)
        .style(WaveStyle::Filled)
        .color(Color::GREEN);

    let config = TestConfig::with_size(50, 8);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("waveline_filled");
}

#[test]
fn test_waveline_mirrored() {
    use revue::style::Color;
    use revue::widget::{WaveStyle, Waveline};

    let data: Vec<f64> = (0..60)
        .map(|i| (i as f64 * 0.1).sin() * 0.5 + 0.5)
        .collect();
    let view = Waveline::new(data)
        .style(WaveStyle::Mirrored)
        .color(Color::CYAN)
        .show_baseline(true);

    let config = TestConfig::with_size(70, 12);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("waveline_mirrored");
}
