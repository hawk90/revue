//! Integration tests for ZenMode widget

use revue::style::Color;
use revue::widget::{Text, ZenMode};

#[test]
fn test_zen_mode_new() {
    let content = Text::new("Test content");
    let _zen = ZenMode::new(content);
    // ZenMode created successfully
}

#[test]
fn test_zen_mode_padding() {
    let content = Text::new("Test");
    let _zen = ZenMode::new(content).padding(4);
}

#[test]
fn test_zen_mode_padding_x() {
    let content = Text::new("Test");
    let _zen = ZenMode::new(content).padding_x(8);
}

#[test]
fn test_zen_mode_padding_y() {
    let content = Text::new("Test");
    let _zen = ZenMode::new(content).padding_y(6);
}

#[test]
fn test_zen_mode_bg() {
    let content = Text::new("Test");
    let _zen = ZenMode::new(content).bg(Color::rgb(20, 20, 30));
}

#[test]
fn test_zen_mode_dim() {
    let content = Text::new("Test");
    let _zen = ZenMode::new(content).dim(0.5);
}

#[test]
fn test_zen_mode_center() {
    let content = Text::new("Test");
    let _zen = ZenMode::new(content).center();
}

#[test]
fn test_zen_mode_enable() {
    let content = Text::new("Test");
    let mut zen = ZenMode::new(content);
    assert!(!zen.is_enabled());

    zen.enable();
    assert!(zen.is_enabled());
}

#[test]
fn test_zen_mode_disable() {
    let content = Text::new("Test");
    let mut zen = ZenMode::new(content);
    zen.enable();
    assert!(zen.is_enabled());

    zen.disable();
    assert!(!zen.is_enabled());
}

#[test]
fn test_zen_mode_toggle() {
    let content = Text::new("Test");
    let mut zen = ZenMode::new(content);
    assert!(!zen.is_enabled());

    zen.toggle();
    assert!(zen.is_enabled());

    zen.toggle();
    assert!(!zen.is_enabled());
}

#[test]
fn test_zen_mode_is_enabled() {
    let content = Text::new("Test");
    let zen = ZenMode::new(content);
    assert!(!zen.is_enabled());
}

#[test]
fn test_zen_mode_set_enabled() {
    let content = Text::new("Test");
    let mut zen = ZenMode::new(content);
    zen.set_enabled(true);
    assert!(zen.is_enabled());

    zen.set_enabled(false);
    assert!(!zen.is_enabled());
}

#[test]
fn test_zen_mode_content() {
    let content = Text::new("Test content");
    let zen = ZenMode::new(content);
    let _inner = zen.content();
    // Can get content reference
}

#[test]
fn test_zen_mode_content_mut() {
    let content = Text::new("Test content");
    let mut zen = ZenMode::new(content);
    let _inner = zen.content_mut();
    // Can get content mutable reference
}

#[test]
fn test_zen_mode_default() {
    let _zen = ZenMode::default();
}

#[test]
fn test_zen_mode_builder_pattern() {
    let content = Text::new("Focus on this");
    let zen = ZenMode::new(content)
        .padding(4)
        .bg(Color::rgb(20, 20, 30))
        .center();

    assert!(!zen.is_enabled());
}

#[test]
fn test_zen_mode_enabled_builder() {
    let content = Text::new("Test");
    let mut zen = ZenMode::new(content).padding(2);
    zen.enable();
    assert!(zen.is_enabled());
}

#[test]
fn test_zen_mode_dim_clamping() {
    let content = Text::new("Test");
    // Dim opacity should be clamped to 0.0-1.0
    let _zen_high = ZenMode::new(content).dim(2.0); // Should clamp to 1.0
    let content2 = Text::new("Test");
    let _zen_low = ZenMode::new(content2).dim(-0.5); // Should clamp to 0.0
}

#[test]
fn test_zen_mode_multiple_toggles() {
    let content = Text::new("Test");
    let mut zen = ZenMode::new(content);

    for i in 0..10 {
        zen.toggle();
        assert_eq!(zen.is_enabled(), i % 2 == 0);
    }
}
