//! `HotReload`'s auto traits are the same on every OS (#1414)

use revue::core::app::HotReload;
use std::panic::{RefUnwindSafe, UnwindSafe};

fn unwind_safe<T: UnwindSafe + RefUnwindSafe>() {}

#[test]
fn hot_reload_is_unwind_safe_on_every_os() {
    // notify's macOS watcher is not unwind safe; the Linux one is.
    unwind_safe::<HotReload>();
}

#[test]
fn hot_reload_can_be_used_inside_catch_unwind() {
    // The closure borrows the watcher, which needs RefUnwindSafe.
    let reload = HotReload::new().unwrap();
    let result = std::panic::catch_unwind(|| reload.watched_paths().len());
    assert_eq!(result.unwrap(), 0);
}
