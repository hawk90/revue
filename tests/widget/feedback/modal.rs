//! Modal widget tests

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_modal_basic() {
        let mut modal = Modal::new()
            .title("Confirm Action")
            .content("Are you sure you want to proceed?")
            .ok();
        modal.show();

        let mut app = TestApp::new(modal);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("modal_basic");
    }
}
