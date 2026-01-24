//! Time-Travel Debugging for Revue applications
//!
//! Records state snapshots and allows stepping through history.
//!
//! # Features
//!
//! - Automatic state snapshot recording
//! - Step forward/backward through history
//! - Jump to specific snapshot
//! - Action/event log with timestamps
//! - State diff visualization
//! - Export/import session history
//! - Configurable snapshot limits
//! - Pause/resume recording

mod debugger;
mod types;

pub use debugger::TimeTravelDebugger;
pub use types::{
    Action, SnapshotValue, StateDiff, StateSnapshot, TimeTravelConfig, TimeTravelView,
};

#[cfg(test)]
mod tests {
//! Tests for time-travel debugging

use super::*;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[test]
fn test_snapshot_creation() {
    let snapshot = StateSnapshot::new(1)
        .with_state("count", SnapshotValue::Int(42))
        .with_label("initial");

    assert_eq!(snapshot.id, 1);
    assert_eq!(snapshot.label, Some("initial".to_string()));
    assert!(snapshot.state.contains_key("count"));
}

#[test]
fn test_snapshot_diff() {
    let mut state1 = HashMap::new();
    state1.insert("a".to_string(), SnapshotValue::Int(1));
    state1.insert("b".to_string(), SnapshotValue::Int(2));

    let mut state2 = HashMap::new();
    state2.insert("a".to_string(), SnapshotValue::Int(1));
    state2.insert("b".to_string(), SnapshotValue::Int(3)); // changed
    state2.insert("c".to_string(), SnapshotValue::Int(4)); // added

    let snap1 = StateSnapshot {
        id: 1,
        timestamp: SystemTime::now(),
        state: state1,
        action: None,
        label: None,
    };

    let snap2 = StateSnapshot {
        id: 2,
        timestamp: SystemTime::now(),
        state: state2,
        action: None,
        label: None,
    };

    let diff = snap2.diff(&snap1);
    assert_eq!(diff.added.len(), 1);
    assert!(diff.added.contains_key("c"));
    assert_eq!(diff.changed.len(), 1);
    assert!(diff.changed.contains_key("b"));
    assert!(diff.removed.is_empty());
}

#[test]
fn test_time_travel_record() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO; // Disable rate limiting

    tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(1)));
    tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(2)));
    tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(3)));

    assert_eq!(tt.count(), 3);
    assert_eq!(tt.position(), 2);
}

#[test]
fn test_time_travel_navigation() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO; // Disable rate limiting

    tt.record(StateSnapshot::new(0).with_label("first"));
    tt.record(StateSnapshot::new(0).with_label("second"));
    tt.record(StateSnapshot::new(0).with_label("third"));

    assert_eq!(tt.position(), 2);

    tt.step_back();
    assert_eq!(tt.position(), 1);
    assert!(tt.is_traveling());

    tt.step_back();
    assert_eq!(tt.position(), 0);

    tt.step_forward();
    assert_eq!(tt.position(), 1);

    tt.jump_to_latest();
    assert_eq!(tt.position(), 2);
    assert!(!tt.is_traveling());
}

#[test]
fn test_time_travel_pause() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO; // Disable rate limiting

    tt.record(StateSnapshot::new(0));
    assert_eq!(tt.count(), 1);

    tt.pause();
    tt.record(StateSnapshot::new(0));
    assert_eq!(tt.count(), 1); // Still 1, recording paused

    tt.resume();
    tt.record(StateSnapshot::new(0));
    assert_eq!(tt.count(), 2);
}

#[test]
fn test_time_travel_max_snapshots() {
    let mut tt = TimeTravelDebugger::new().max_snapshots(3);
    tt.config.record_interval = Duration::ZERO;

    for i in 0..5 {
        tt.record(StateSnapshot::new(0).with_state("i", SnapshotValue::Int(i)));
    }

    assert_eq!(tt.count(), 3);
    // Should have kept the last 3
    assert_eq!(tt.snapshots[0].state.get("i"), Some(&SnapshotValue::Int(2)));
}

#[test]
fn test_time_travel_branch_on_travel() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0).with_label("a"));
    tt.record(StateSnapshot::new(0).with_label("b"));
    tt.record(StateSnapshot::new(0).with_label("c"));

    // Go back and create new branch
    tt.step_back();
    tt.step_back();
    assert_eq!(tt.position(), 0);

    tt.record(StateSnapshot::new(0).with_label("d"));

    // Should have truncated b and c, now have a and d
    assert_eq!(tt.count(), 2);
    assert_eq!(tt.current().unwrap().label, Some("d".to_string()));
}

#[test]
fn test_action_creation() {
    let action = Action::new("click")
        .with_source("Button")
        .with_payload(SnapshotValue::String("submit".to_string()));

    assert_eq!(action.name, "click");
    assert_eq!(action.source, Some("Button".to_string()));
}

#[test]
fn test_snapshot_value_conversions() {
    assert_eq!(SnapshotValue::from(true), SnapshotValue::Bool(true));
    assert_eq!(SnapshotValue::from(42i32), SnapshotValue::Int(42));
    assert_eq!(SnapshotValue::from(3.14), SnapshotValue::Float(3.14));
    assert_eq!(
        SnapshotValue::from("hello"),
        SnapshotValue::String("hello".to_string())
    );
}

#[test]
fn test_view_cycle() {
    let view = TimeTravelView::Timeline;
    assert_eq!(view.next(), TimeTravelView::Diff);
    assert_eq!(view.next().next(), TimeTravelView::Actions);
    assert_eq!(view.next().next().next(), TimeTravelView::State);
    assert_eq!(view.next().next().next().next(), TimeTravelView::Timeline);
}

#[test]
fn test_export() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(
        StateSnapshot::new(0)
            .with_label("test")
            .with_action(Action::new("init")),
    );

    let json = tt.export();
    assert!(json.contains("snapshot_count"));
    assert!(json.contains("\"label\": \"test\""));
    assert!(json.contains("\"action\": \"init\""));
}

#[test]
fn test_clear() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));
    assert_eq!(tt.count(), 2);

    tt.clear();
    assert_eq!(tt.count(), 0);
    assert_eq!(tt.position(), 0);
}

#[test]
fn test_snapshot_value_display() {
    assert_eq!(SnapshotValue::Null.display(), "null");
    assert_eq!(SnapshotValue::Bool(true).display(), "true");
    assert_eq!(SnapshotValue::Bool(false).display(), "false");
    assert_eq!(SnapshotValue::Int(42).display(), "42");
    assert_eq!(SnapshotValue::Float(3.14159).display(), "3.14");
    assert_eq!(
        SnapshotValue::String("hello".to_string()).display(),
        "\"hello\""
    );
    assert_eq!(
        SnapshotValue::Array(vec![SnapshotValue::Int(1), SnapshotValue::Int(2)]).display(),
        "[2 items]"
    );
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), SnapshotValue::Int(1));
    assert_eq!(SnapshotValue::Object(obj).display(), "{1 keys}");
}

#[test]
fn test_snapshot_value_type_name() {
    assert_eq!(SnapshotValue::Null.type_name(), "null");
    assert_eq!(SnapshotValue::Bool(true).type_name(), "bool");
    assert_eq!(SnapshotValue::Int(42).type_name(), "i64");
    assert_eq!(SnapshotValue::Float(3.14).type_name(), "f64");
    assert_eq!(
        SnapshotValue::String("test".to_string()).type_name(),
        "String"
    );
    assert_eq!(SnapshotValue::Array(vec![]).type_name(), "Array");
    assert_eq!(SnapshotValue::Object(HashMap::new()).type_name(), "Object");
}

#[test]
fn test_snapshot_value_from_string_owned() {
    let s = String::from("owned");
    let val: SnapshotValue = s.into();
    assert_eq!(val, SnapshotValue::String("owned".to_string()));
}

#[test]
fn test_snapshot_value_from_i64() {
    let val: SnapshotValue = 123i64.into();
    assert_eq!(val, SnapshotValue::Int(123));
}

#[test]
fn test_state_diff_is_empty() {
    let diff = StateDiff::default();
    assert!(diff.is_empty());
    assert_eq!(diff.count(), 0);
}

#[test]
fn test_state_diff_count() {
    let mut diff = StateDiff::default();
    diff.added.insert("a".to_string(), SnapshotValue::Int(1));
    diff.removed.insert("b".to_string(), SnapshotValue::Int(2));
    diff.changed.insert(
        "c".to_string(),
        (SnapshotValue::Int(3), SnapshotValue::Int(4)),
    );

    assert!(!diff.is_empty());
    assert_eq!(diff.count(), 3);
}

#[test]
fn test_time_travel_view_default() {
    let view = TimeTravelView::default();
    assert_eq!(view, TimeTravelView::Timeline);
}

#[test]
fn test_time_travel_view_labels() {
    assert_eq!(TimeTravelView::Timeline.label(), "Timeline");
    assert_eq!(TimeTravelView::Diff.label(), "Diff");
    assert_eq!(TimeTravelView::Actions.label(), "Actions");
    assert_eq!(TimeTravelView::State.label(), "State");
}

#[test]
fn test_time_travel_view_all() {
    let views = TimeTravelView::all();
    assert_eq!(views.len(), 4);
    assert!(views.contains(&TimeTravelView::Timeline));
    assert!(views.contains(&TimeTravelView::Diff));
    assert!(views.contains(&TimeTravelView::Actions));
    assert!(views.contains(&TimeTravelView::State));
}

#[test]
fn test_time_travel_config_default() {
    let config = TimeTravelConfig::default();
    assert_eq!(config.max_snapshots, 100);
    assert!(config.auto_record);
    assert_eq!(config.record_interval, Duration::from_millis(100));
}

#[test]
fn test_time_travel_debugger_default() {
    let tt = TimeTravelDebugger::default();
    assert_eq!(tt.count(), 0);
    assert_eq!(tt.position(), 0);
    assert!(!tt.is_paused());
}

#[test]
fn test_time_travel_debugger_with_config() {
    let config = TimeTravelConfig {
        max_snapshots: 50,
        auto_record: false,
        record_interval: Duration::from_millis(200),
    };
    let tt = TimeTravelDebugger::new().with_config(config);
    assert_eq!(tt.config.max_snapshots, 50);
}

#[test]
fn test_time_travel_set_view() {
    let mut tt = TimeTravelDebugger::new();
    assert_eq!(tt.view(), TimeTravelView::Timeline);

    tt.set_view(TimeTravelView::State);
    assert_eq!(tt.view(), TimeTravelView::State);
}

#[test]
fn test_time_travel_next_view() {
    let mut tt = TimeTravelDebugger::new();
    assert_eq!(tt.view(), TimeTravelView::Timeline);

    tt.next_view();
    assert_eq!(tt.view(), TimeTravelView::Diff);

    tt.next_view();
    assert_eq!(tt.view(), TimeTravelView::Actions);

    tt.next_view();
    assert_eq!(tt.view(), TimeTravelView::State);

    tt.next_view();
    assert_eq!(tt.view(), TimeTravelView::Timeline);
}

#[test]
fn test_time_travel_toggle_recording() {
    let mut tt = TimeTravelDebugger::new();
    assert!(!tt.is_paused());

    tt.toggle_recording();
    assert!(tt.is_paused());

    tt.toggle_recording();
    assert!(!tt.is_paused());
}

#[test]
fn test_time_travel_current_empty() {
    let tt = TimeTravelDebugger::new();
    assert!(tt.current().is_none());
}

#[test]
fn test_time_travel_get_snapshot() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0).with_label("first"));
    tt.record(StateSnapshot::new(0).with_label("second"));

    assert!(tt.get(0).is_some());
    assert_eq!(tt.get(0).unwrap().label, Some("first".to_string()));
    assert!(tt.get(1).is_some());
    assert!(tt.get(5).is_none());
}

#[test]
fn test_time_travel_snapshots() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));

    assert_eq!(tt.snapshots().len(), 2);
}

#[test]
fn test_time_travel_jump_to_first() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));

    assert_eq!(tt.position(), 2);

    tt.jump_to_first();
    assert_eq!(tt.position(), 0);
    assert!(tt.is_traveling());
}

#[test]
fn test_time_travel_jump_to() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));

    tt.jump_to(1);
    assert_eq!(tt.position(), 1);
    assert!(tt.is_traveling());

    // Out of bounds should not change
    tt.jump_to(100);
    assert_eq!(tt.position(), 1);
}

#[test]
fn test_time_travel_current_diff_empty() {
    let tt = TimeTravelDebugger::new();
    assert!(tt.current_diff().is_none());
}

#[test]
fn test_time_travel_current_diff_first_snapshot() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;
    tt.record(StateSnapshot::new(0));

    // At position 0, there's no previous snapshot
    assert!(tt.current_diff().is_none());
}

#[test]
fn test_time_travel_current_diff() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(1)));
    tt.record(StateSnapshot::new(0).with_state("x", SnapshotValue::Int(2)));

    let diff = tt.current_diff();
    assert!(diff.is_some());
    let diff = diff.unwrap();
    assert!(diff.changed.contains_key("x"));
}

#[test]
fn test_time_travel_diff_between() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0).with_state("a", SnapshotValue::Int(1)));
    tt.record(StateSnapshot::new(0).with_state("a", SnapshotValue::Int(2)));
    tt.record(StateSnapshot::new(0).with_state("a", SnapshotValue::Int(3)));

    let diff = tt.diff_between(0, 2);
    assert!(diff.is_some());
    let diff = diff.unwrap();
    assert!(diff.changed.contains_key("a"));

    // Out of bounds
    assert!(tt.diff_between(0, 100).is_none());
    assert!(tt.diff_between(100, 0).is_none());
}

#[test]
fn test_time_travel_import() {
    let mut tt = TimeTravelDebugger::new();

    let snapshots = vec![
        StateSnapshot::new(5),
        StateSnapshot::new(10),
        StateSnapshot::new(15),
    ];

    tt.import(snapshots);

    assert_eq!(tt.count(), 3);
    assert_eq!(tt.position(), 2);
}

#[test]
fn test_time_travel_import_empty() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;
    tt.record(StateSnapshot::new(0));

    tt.import(vec![]);

    assert_eq!(tt.count(), 0);
    assert_eq!(tt.position(), 0);
}

#[test]
fn test_time_travel_select_next() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));

    assert!(tt.selected.is_none());

    tt.select_next();
    assert_eq!(tt.selected, Some(0));

    tt.select_next();
    assert_eq!(tt.selected, Some(1));

    // Should not go beyond count
    tt.select_next();
    assert_eq!(tt.selected, Some(1));
}

#[test]
fn test_time_travel_select_prev() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));

    tt.selected = Some(1);
    tt.select_prev();
    assert_eq!(tt.selected, Some(0));

    tt.select_prev();
    assert_eq!(tt.selected, Some(0));
}

#[test]
fn test_time_travel_select_empty() {
    let mut tt = TimeTravelDebugger::new();

    tt.select_next();
    assert!(tt.selected.is_none());
}

#[test]
fn test_action_with_duration() {
    let action = Action::new("test").with_duration(Duration::from_secs(1));

    assert_eq!(action.duration, Some(Duration::from_secs(1)));
}

#[test]
fn test_snapshot_with_action() {
    let action = Action::new("click");
    let snapshot = StateSnapshot::new(0).with_action(action);

    assert!(snapshot.action.is_some());
    assert_eq!(snapshot.action.unwrap().name, "click");
}

#[test]
fn test_record_action() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    let mut state = HashMap::new();
    state.insert("count".to_string(), SnapshotValue::Int(5));

    tt.record_action(Action::new("increment"), state);

    assert_eq!(tt.count(), 1);
    let snapshot = tt.current().unwrap();
    assert!(snapshot.action.is_some());
    assert_eq!(snapshot.state.get("count"), Some(&SnapshotValue::Int(5)));
}

#[test]
fn test_snapshot_diff_removed() {
    let mut state1 = HashMap::new();
    state1.insert("a".to_string(), SnapshotValue::Int(1));
    state1.insert("b".to_string(), SnapshotValue::Int(2));

    let mut state2 = HashMap::new();
    state2.insert("a".to_string(), SnapshotValue::Int(1));
    // b is removed

    let snap1 = StateSnapshot {
        id: 1,
        timestamp: SystemTime::now(),
        state: state1,
        action: None,
        label: None,
    };
    let snap2 = StateSnapshot {
        id: 2,
        timestamp: SystemTime::now(),
        state: state2,
        action: None,
        label: None,
    };

    let diff = snap2.diff(&snap1);
    assert_eq!(diff.removed.len(), 1);
    assert!(diff.removed.contains_key("b"));
}

#[test]
fn test_step_forward_at_end() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0));
    tt.record(StateSnapshot::new(0));

    // Already at end
    assert_eq!(tt.position(), 1);

    tt.step_forward();
    assert_eq!(tt.position(), 1);
    assert!(!tt.is_traveling());
}

#[test]
fn test_step_back_at_start() {
    let mut tt = TimeTravelDebugger::new();
    tt.config.record_interval = Duration::ZERO;

    tt.record(StateSnapshot::new(0));

    tt.step_back();
    assert_eq!(tt.position(), 0);
}

}
