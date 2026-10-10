//! Breadcrumb widget tests

pub mod core;
pub mod helper;
pub mod types;

mod snapshots {

    use revue::testing::{Pilot, TestApp};
    use revue::widget::Breadcrumb;

    #[test]
    fn test_breadcrumb_basic() {
        let view = Breadcrumb::new()
            .push("Home")
            .push("Products")
            .push("Electronics")
            .push("Phones");

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("breadcrumb_basic");
    }
}
