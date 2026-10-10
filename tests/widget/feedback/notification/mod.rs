//! Notification widget tests

mod core;
mod types;

mod snapshots {

    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_notification_basic() {
        use revue::widget::NotificationCenter;

        // NotificationCenter is the widget, Notification is a data struct
        let view = NotificationCenter::new();

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("notification_basic");
    }
}
