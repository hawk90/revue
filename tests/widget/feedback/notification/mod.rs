//! Notification widget tests

mod draw;
mod helpers;
mod input;
mod queue;
mod types;
mod view;

mod snapshots {

    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_notification_basic() {
        use revue::widget::{Notification, NotificationCenter};

        // NotificationCenter is the widget, Notification is a data struct
        let mut view = NotificationCenter::new();
        view.push(Notification::success("Saved").duration(0));
        view.push(
            Notification::warning("Disk almost full")
                .title("Storage")
                .progress(0.75)
                .action("Clean up")
                .duration(0),
        );

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("notification_basic");
    }
}
