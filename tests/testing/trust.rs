//! A check that passes must have compared the right thing (#848, #849, #952)

use revue::core::app::{Snapshot, SnapshotConfig, SnapshotResult};
use revue::event::{MouseEvent, MouseEventKind};
use revue::render::Buffer;
use revue::testing::{Pilot, SnapshotManager, TestApp, VisualCapture};
use revue::widget::Text;
use std::cell::RefCell;
use std::rc::Rc;

fn reloaded(buffer: &Buffer) -> VisualCapture {
    let capture = VisualCapture::from_buffer(buffer, false, false);
    VisualCapture::deserialize(&capture.serialize()).unwrap()
}

fn symbol(capture: &VisualCapture, x: u16, y: u16) -> char {
    capture.get(x, y).unwrap().symbol
}

#[test]
fn a_golden_keeps_its_blank_rows() {
    let mut buffer = Buffer::new(3, 3);
    buffer.put_str(0, 0, "A");
    buffer.put_str(0, 2, "B");
    let golden = reloaded(&buffer);
    assert_eq!(symbol(&golden, 0, 0), 'A');
    assert_eq!(symbol(&golden, 0, 1), ' ');
    assert_eq!(symbol(&golden, 0, 2), 'B');
}

#[test]
fn a_golden_keeps_rows_that_read_like_its_own_headings() {
    for row in ["# Title", "## Colors", "## Text", "# Size: 1x1"] {
        let mut buffer = Buffer::new(12, 3);
        buffer.put_str(0, 0, row);
        buffer.put_str(0, 2, "below");
        let golden = reloaded(&buffer);
        assert_eq!((golden.width, golden.height), (12, 3), "{row}");
        assert_eq!(symbol(&golden, 0, 0), '#', "{row}");
        assert_eq!(symbol(&golden, 0, 2), 'b', "{row}");
    }
}

#[test]
fn a_golden_past_65535_cells_reads_the_right_cell() {
    let mut buffer = Buffer::new(300, 300);
    buffer.put_str(299, 299, "Z");
    buffer.put_str(0, 299, "Y");
    let capture = VisualCapture::from_buffer(&buffer, false, false);
    assert_eq!(symbol(&capture, 299, 299), 'Z');
    assert_eq!(symbol(&capture, 0, 299), 'Y');
    assert_eq!(symbol(&capture, 0, 0), ' ');
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("trust-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
#[should_panic(expected = "mismatch")]
fn a_snapshot_notices_content_that_moved_right() {
    let manager = SnapshotManager::with_dir(scratch("right"));
    manager.assert_snapshot("moved", "Hello");
    manager.assert_snapshot("moved", "   Hello");
}

#[test]
#[should_panic(expected = "mismatch")]
fn a_snapshot_notices_content_that_moved_down() {
    let manager = SnapshotManager::with_dir(scratch("down"));
    manager.assert_snapshot("moved", "Hello");
    manager.assert_snapshot("moved", "\n\nHello");
}

#[test]
fn a_snapshot_ignores_only_what_a_screen_cannot_show() {
    let manager = SnapshotManager::with_dir(scratch("same"));
    manager.assert_snapshot("same", "a  \r\n b\n\n");
    manager.assert_snapshot("same", "a\n b");
}

#[test]
#[should_panic(expected = "does not match")]
fn assert_screen_notices_content_that_moved() {
    let mut app = TestApp::new(Text::new("Hello"));
    let pilot = Pilot::new(&mut app);
    pilot.assert_screen("  Hello");
}

#[test]
fn assert_screen_ignores_only_trailing_blanks() {
    let mut app = TestApp::new(Text::new("Hello"));
    let pilot = Pilot::new(&mut app);
    pilot.assert_screen("Hello   \n\n");
}

fn tester(dir: &std::path::Path, update: bool) -> Snapshot {
    Snapshot::with_config(
        SnapshotConfig::default()
            .snapshot_dir(dir)
            .update_snapshots(update),
    )
}

#[test]
fn a_golden_that_cannot_be_read_is_no_pass_and_stays() {
    for update in [false, true] {
        let dir = scratch(if update { "bad-update" } else { "bad" });
        let path = dir.join("bad.snap");
        std::fs::write(&path, [0xFF, 0xFE, 0x00]).unwrap();
        let result = tester(&dir, update).assert_snapshot_string("bad", "new");
        assert!(matches!(result, SnapshotResult::NotFound), "{result:?}");
        assert!(!result.is_match());
        assert_eq!(std::fs::read(&path).unwrap(), [0xFF, 0xFE, 0x00]);
    }
}

#[test]
fn a_golden_that_cannot_be_written_is_no_pass() {
    let dir = scratch("unwritable");
    // The directory the golden needs is a file, so nothing can be written
    let blocked = dir.join("blocked");
    std::fs::write(&blocked, "a file").unwrap();
    let result = tester(&blocked, false).assert_snapshot_string("x", "new");
    assert!(matches!(result, SnapshotResult::NotFound), "{result:?}");
    assert!(!result.is_match());
}

#[test]
fn a_written_golden_still_counts_as_created_then_matches() {
    let dir = scratch("written");
    let result = tester(&dir, false).assert_snapshot_string("x", "new");
    assert!(matches!(result, SnapshotResult::Created));
    let result = tester(&dir, false).assert_snapshot_string("x", "new");
    assert!(matches!(result, SnapshotResult::Match));
}

type Seen = Rc<RefCell<Vec<(MouseEventKind, u16, u16)>>>;

fn scroll_app() -> (TestApp<Text>, Seen, Rc<RefCell<Vec<i16>>>) {
    let seen: Seen = Rc::default();
    let deltas: Rc<RefCell<Vec<i16>>> = Rc::default();
    let (mouse, scroll) = (seen.clone(), deltas.clone());
    let app = TestApp::new(Text::new("x"))
        .on_mouse(move |event: &MouseEvent, _| {
            mouse.borrow_mut().push((event.kind, event.x, event.y));
            false
        })
        .on_scroll(move |_, _, delta, _| {
            scroll.borrow_mut().push(delta);
            false
        });
    (app, seen, deltas)
}

#[test]
fn pilot_scrolls_with_the_events_a_widget_gets() {
    let (mut app, seen, deltas) = scroll_app();
    let mut pilot = Pilot::new(&mut app);
    pilot.scroll_up(4, 5, 2);
    pilot.scroll_down(6, 7, 1);
    assert_eq!(
        *seen.borrow(),
        vec![
            (MouseEventKind::ScrollUp, 4, 5),
            (MouseEventKind::ScrollUp, 4, 5),
            (MouseEventKind::ScrollDown, 6, 7),
        ]
    );
    assert_eq!(
        *deltas.borrow(),
        vec![2, -1],
        "positive is up, as documented"
    );
}

#[test]
fn pilot_scrolling_by_nothing_sends_no_event() {
    let (mut app, seen, _) = scroll_app();
    Pilot::new(&mut app).scroll_up(1, 1, 0);
    assert!(seen.borrow().is_empty());
}
