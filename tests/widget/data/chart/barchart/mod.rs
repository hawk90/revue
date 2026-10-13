//! BarChart widget tests

mod bar;
mod builders;
mod horizontal;
mod vertical;
mod view;

pub(super) fn bar(n: usize) -> String {
    "█".repeat(n)
}

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_barchart_basic() {
        let view = BarChart::new()
            .bar("Mon", 10.0)
            .bar("Tue", 20.0)
            .bar("Wed", 15.0)
            .bar("Thu", 25.0)
            .bar("Fri", 18.0);

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("barchart_basic");
    }
}
