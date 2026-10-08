//! Asking an `AsyncTask` whether it is still running must not use up its
//! result.

use revue::patterns::AsyncTask;
use std::time::{Duration, Instant};

fn finished_task() -> AsyncTask<i32> {
    let task = AsyncTask::spawn(|| 42);
    let deadline = Instant::now() + Duration::from_secs(5);
    while task.is_running() {
        assert!(Instant::now() < deadline, "the task never finished");
        std::thread::sleep(Duration::from_millis(1));
    }
    task
}

#[test]
fn the_result_is_still_there_after_is_running_says_it_finished() {
    let mut task = finished_task();
    assert!(!task.is_running());
    assert_eq!(task.try_recv(), Some(42), "is_running consumed the result");
}

#[test]
fn wait_still_returns_the_result_after_is_running() {
    let task = finished_task();
    assert_eq!(task.wait(), Some(42), "is_running consumed the result");
}

#[test]
fn a_task_that_has_not_finished_is_running() {
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    let mut task = AsyncTask::spawn(move || {
        rx.recv().unwrap();
        7
    });
    assert!(task.is_running());
    assert_eq!(task.try_recv(), None);

    tx.send(()).unwrap();
    assert_eq!(task.wait(), Some(7));
}

#[test]
fn the_task_is_not_running_once_its_result_was_taken() {
    let mut task = finished_task();
    assert_eq!(task.try_recv(), Some(42));
    assert!(!task.is_running());
}
