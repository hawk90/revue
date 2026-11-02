//! What one frame may cost: allocations and bytes sent to the terminal (#1169).
//!
//! A binary of its own because it installs its own allocator. That takes the
//! place of `support/alloc_cap.rs`: these tests hold a few small frames, so
//! they cannot run away. Allocations are counted per thread, which leaves out
//! the test runner.
//!
//! An unchanged frame should allocate nothing. It does not yet (#1406), so the
//! ceilings are today's numbers, there to stop them from growing. Lower them
//! when the frame path gets cheaper.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

struct Counting;

impl Counting {
    fn count() {
        // `try_with`: an allocation can come while the thread is shutting down.
        let _ = ALLOCATIONS.try_with(|n| n.set(n.get() + 1));
    }
}

// SAFETY: every call forwards to `System` with the caller's layout; the counter
// does not touch the memory, and a const thread-local does not allocate.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Self::count();
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        Self::count();
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        Self::count();
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// One string, drawn directly: the frame path with the least in it.
struct Line(&'static str);

impl View for Line {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, self.0, Color::WHITE);
    }
}

/// A small tree, built again on each render as every view is today.
struct Form(&'static str);

impl View for Form {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new(self.0).element_id("label"))
            .child(Button::new("OK").element_id("ok"))
            .render(ctx);
    }
}

/// A harness past its first frames, whose setup is not what is being measured.
fn settled(view: &impl View) -> PipelineHarness {
    let mut harness = PipelineHarness::new(40, 10);
    harness.draw(view);
    harness.draw(view);
    harness
}

/// How much drawing `view` allocated and what it sent to the terminal.
fn frame(harness: &mut PipelineHarness, view: &impl View) -> (usize, Vec<u8>) {
    let written = harness.terminal_output().len();
    let before = ALLOCATIONS.with(Cell::get);
    harness.draw(view);
    let allocations = ALLOCATIONS.with(Cell::get) - before;
    (allocations, harness.terminal_output()[written..].to_vec())
}

const UNCHANGED_LINE_CEILING: usize = 10;
const UNCHANGED_FORM_CEILING: usize = 56;
/// A changed cell costs a few more than an unchanged frame.
const CHANGED_CELL_EXTRA: usize = 3;

#[test]
fn an_unchanged_frame_writes_nothing() {
    let mut line = settled(&Line("abc"));
    let mut form = settled(&Form("abc"));
    for _ in 0..3 {
        assert_eq!(frame(&mut line, &Line("abc")).1, b"");
        assert_eq!(frame(&mut form, &Form("abc")).1, b"");
    }
}

#[test]
fn a_changed_character_writes_one_cursor_move_and_one_character() {
    let mut form = settled(&Form("abc"));
    assert_eq!(frame(&mut form, &Form("zbc")).1, b"\x1b[1;1Hz");
}

#[test]
fn a_changed_character_further_along_moves_the_cursor_there() {
    let mut form = settled(&Form("abc"));
    assert_eq!(frame(&mut form, &Form("abz")).1, b"\x1b[1;3Hz");
}

#[test]
fn neighboring_changes_share_one_cursor_move() {
    let mut form = settled(&Form("abc"));
    assert_eq!(frame(&mut form, &Form("xyz")).1, b"\x1b[1;1Hxyz");
}

#[test]
fn changing_back_writes_only_the_change_again() {
    let mut form = settled(&Form("abc"));
    frame(&mut form, &Form("zbc"));
    assert_eq!(frame(&mut form, &Form("abc")).1, b"\x1b[1;1Ha");
}

#[test]
fn an_unchanged_frame_stays_under_its_allocation_ceiling() {
    let mut line = settled(&Line("abc"));
    let mut form = settled(&Form("abc"));
    for _ in 0..3 {
        let (allocations, _) = frame(&mut line, &Line("abc"));
        assert!(allocations <= UNCHANGED_LINE_CEILING, "{allocations}");
        let (allocations, _) = frame(&mut form, &Form("abc"));
        assert!(allocations <= UNCHANGED_FORM_CEILING, "{allocations}");
    }
}

#[test]
fn a_changed_cell_stays_under_its_allocation_ceiling() {
    let mut line = settled(&Line("abc"));
    let (allocations, _) = frame(&mut line, &Line("zbc"));
    assert!(
        allocations <= UNCHANGED_LINE_CEILING + CHANGED_CELL_EXTRA,
        "{allocations}"
    );

    let mut form = settled(&Form("abc"));
    let (allocations, _) = frame(&mut form, &Form("zbc"));
    assert!(
        allocations <= UNCHANGED_FORM_CEILING + CHANGED_CELL_EXTRA,
        "{allocations}"
    );
}

/// The cost of a frame must not build up with the number of frames.
#[test]
fn allocations_do_not_grow_frame_over_frame() {
    let mut form = settled(&Form("abc"));
    let (first, _) = frame(&mut form, &Form("abc"));
    for _ in 0..200 {
        frame(&mut form, &Form("abc"));
    }
    let (later, _) = frame(&mut form, &Form("abc"));
    assert_eq!(later, first);
}
