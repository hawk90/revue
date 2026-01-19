//! Backend Traits tests (from src/render/backend/traits.rs)

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::*;
use revue::style::Color;

#[test]
fn test_backend_capabilities_default() {
    let caps = BackendCapabilities::default();
    assert!(!caps.true_color);
    assert!(!caps.hyperlinks);
    assert!(!caps.mouse);
    assert!(!caps.bracketed_paste);
    assert!(!caps.focus_events);
}

#[test]
fn test_backend_capabilities_custom() {
    let caps = BackendCapabilities {
        true_color: true,
        hyperlinks: true,
        mouse: true,
        bracketed_paste: true,
        focus_events: true,
    };
    assert!(caps.true_color);
    assert!(caps.hyperlinks);
    assert!(caps.mouse);
    assert!(caps.bracketed_paste);
    assert!(caps.focus_events);
}

#[test]
fn test_backend_capabilities_clone() {
    let caps = BackendCapabilities {
        true_color: true,
        hyperlinks: false,
        mouse: true,
        bracketed_paste: false,
        focus_events: true,
    };
    let cloned = caps.clone();
    assert_eq!(cloned.true_color, caps.true_color);
    assert_eq!(cloned.hyperlinks, caps.hyperlinks);
    assert_eq!(cloned.mouse, caps.mouse);
}

#[test]
fn test_backend_capabilities_debug() {
    let caps = BackendCapabilities::default();
    let debug = format!("{:?}", caps);
    assert!(debug.contains("BackendCapabilities"));
}
