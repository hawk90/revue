//! Vim mode palette test
//!
//! The rest of the extracted Vim tests duplicated tests/widget/vim.rs.

use revue::style::Color;
use revue::widget::VimMode;

#[test]
fn test_vim_mode_color() {
    assert_eq!(VimMode::Normal.color(), Color::rgb(100, 150, 255));
    assert_eq!(VimMode::Insert.color(), Color::rgb(100, 255, 100));
    assert_eq!(VimMode::Visual.color(), Color::rgb(255, 150, 100));
    assert_eq!(VimMode::VisualLine.color(), Color::rgb(255, 150, 100));
    assert_eq!(VimMode::VisualBlock.color(), Color::rgb(255, 150, 100));
    assert_eq!(VimMode::Command.color(), Color::rgb(255, 255, 100));
    assert_eq!(VimMode::Search.color(), Color::rgb(255, 100, 255));
    assert_eq!(VimMode::Replace.color(), Color::rgb(255, 100, 100));
}
