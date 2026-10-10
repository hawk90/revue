//! Grid widget tests

pub mod core;
pub mod helper;
pub mod layout;
pub mod types;
pub mod view;

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};
    use revue::widget::Grid;

    #[test]
    fn test_grid_basic() {
        use revue::widget::TrackSize;

        let view = Grid::new()
            .columns(vec![
                TrackSize::Fr(1.0),
                TrackSize::Fr(1.0),
                TrackSize::Fr(1.0),
            ])
            .child(text("1"))
            .child(text("2"))
            .child(text("3"))
            .child(text("4"))
            .child(text("5"))
            .child(text("6"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("grid_basic");
    }
}
