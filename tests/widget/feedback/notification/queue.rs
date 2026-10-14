//! NotificationCenter contents: push, dismiss, clear and expiry

use super::helpers::{center_with, corners, render, selected_messages};
use revue::widget::{notification_center, Notification, NotificationCenter};

#[test]
fn test_center_new() {
    let c = notification_center();
    assert!(c.is_empty());
    assert_eq!(c.count(), 0);
    assert!(corners(&render(&c, 50, 20)).is_empty());

    let d = NotificationCenter::default();
    assert!(d.is_empty());
}

#[test]
fn test_center_push_and_shortcuts() {
    let mut c = notification_center();
    c.push(Notification::info("Pushed"));
    c.info("Info");
    c.success("Success");
    c.warning("Warning");
    c.error("Error");
    assert_eq!(c.count(), 5);
    assert!(!c.is_empty());
}

#[test]
fn test_center_dismiss_by_id() {
    let mut c = notification_center();
    let keep = Notification::info("keep");
    let drop = Notification::info("drop");
    let drop_id = drop.id;
    c.push(keep);
    c.push(drop);

    c.dismiss(drop_id);
    assert_eq!(c.count(), 1);
    c.dismiss(drop_id);
    assert_eq!(c.count(), 1);
}

#[test]
fn test_center_clear() {
    let mut c = center_with(3);
    c.select_next();
    c.clear();
    assert!(c.is_empty());
    // Nothing is left to select.
    c.info("new");
    assert!(selected_messages(&c).is_empty());
}

#[test]
fn test_center_tick_expires_notifications() {
    let mut c = notification_center();
    c.tick();
    assert!(c.is_empty());

    c.push(Notification::info("short").duration(2));
    c.push(Notification::info("forever").duration(0));
    c.tick();
    assert_eq!(c.count(), 2);
    c.tick();
    assert_eq!(c.count(), 1);
    for _ in 0..100 {
        c.tick();
    }
    assert_eq!(c.count(), 1);
}
