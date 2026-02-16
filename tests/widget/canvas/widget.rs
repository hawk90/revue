//! Canvas widget tests

use revue::widget::canvas::{braille_canvas, canvas, BrailleCanvas, Canvas};

// =========================================================================
// Canvas tests
// =========================================================================

#[test]
fn test_canvas_new() {
    let _canvas = Canvas::new(|_ctx| {
        // Drawing function
    });
    // Just verify it doesn't panic
}

#[test]
fn test_canvas_with_draw_fn() {
    let _canvas = Canvas::new(|ctx| {
        ctx.text(0, 0, "test", None);
    });
    // Just verify it creates successfully
}

// =========================================================================
// canvas helper function tests
// =========================================================================

#[test]
fn test_canvas_helper() {
    let _canvas = canvas(|_ctx| {
        // Drawing function
    });
    // Just verify it doesn't panic
}

#[test]
fn test_canvas_helper_with_draw_fn() {
    let _canvas = canvas(|ctx| {
        ctx.set(5, 5, 'X');
    });
    // Just verify it creates successfully
}

// =========================================================================
// BrailleCanvas tests
// =========================================================================

#[test]
fn test_braille_canvas_new() {
    let _canvas = BrailleCanvas::new(|_ctx| {
        // Drawing function
    });
    // Just verify it doesn't panic
}

#[test]
fn test_braille_canvas_with_draw_fn() {
    let _canvas = BrailleCanvas::new(|_ctx| {
        // Drawing function would go here
    });
    // Just verify it creates successfully
}

// =========================================================================
// braille_canvas helper function tests
// =========================================================================

#[test]
fn test_braille_canvas_helper() {
    let _canvas = braille_canvas(|_ctx| {
        // Drawing function
    });
    // Just verify it doesn't panic
}

#[test]
fn test_braille_canvas_helper_with_draw_fn() {
    let _canvas = braille_canvas(|_ctx| {
        // Drawing function would go here
    });
    // Just verify it creates successfully
}

// =========================================================================
// Closure capture tests
// =========================================================================

#[test]
fn test_canvas_closure_capture() {
    let text = "captured";
    let _canvas = Canvas::new(move |ctx| {
        ctx.text(0, 0, text, None);
    });
    // Just verify closure captures work
}

#[test]
fn test_braille_canvas_closure_capture() {
    let value = 42;
    let _canvas = BrailleCanvas::new(move |_ctx| {
        let _ = value;
    });
    // Just verify closure captures work
}
