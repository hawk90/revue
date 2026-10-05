//! Resizable unit tests that need private state
//!
//! The public tests in tests/widget/resizable.rs call handle_color() and
//! set_hovered() without asserting anything, so these read the fields.
//! Everything else is covered there and in tests/resizable_types.rs.

use super::*;
use crate::style::Color;

#[test]
fn test_resizable_handle_color() {
    let r = Resizable::new(20, 10).handle_color(Color::RED);
    assert_eq!(r.handle_color, Some(Color::RED));
}

#[test]
fn test_resizable_set_hovered() {
    let mut r = Resizable::new(20, 10);
    assert_eq!(r.hovered_handle, None);

    r.set_hovered(Some(ResizeHandle::TopLeft));
    assert_eq!(r.hovered_handle, Some(ResizeHandle::TopLeft));

    r.set_hovered(None);
    assert_eq!(r.hovered_handle, None);
}
