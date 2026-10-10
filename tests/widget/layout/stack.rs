//! Stack layout widget tests (vstack, hstack)

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_vstack_basic() {
        let view = vstack()
            .child(text("Item 1"))
            .child(text("Item 2"))
            .child(text("Item 3"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("vstack_basic");
    }

    #[test]
    fn test_hstack_basic() {
        let view = hstack().child(text("A")).child(text("B")).child(text("C"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("hstack_basic");
    }

    #[test]
    fn test_vstack_with_gap() {
        let view = vstack()
            .gap(1)
            .child(text("Item 1"))
            .child(text("Item 2"))
            .child(text("Item 3"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("vstack_with_gap");
    }

    #[test]
    fn test_nested_stacks() {
        let view = vstack()
            .child(text("Header"))
            .child(
                hstack()
                    .child(text("Left"))
                    .child(text(" | "))
                    .child(text("Right")),
            )
            .child(text("Footer"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("nested_stacks");
    }

    #[test]
    fn test_empty_vstack() {
        let view = vstack();

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("empty_vstack");
    }
}
