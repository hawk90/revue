//! Autocomplete widget tests

mod core;
mod unicode;

mod snapshots {

    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_autocomplete_basic() {
        use revue::widget::Autocomplete;

        let view = Autocomplete::new().placeholder("Search...").suggestions([
            "Apple",
            "Banana",
            "Cherry",
            "Date",
            "Elderberry",
        ]);

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("autocomplete_basic");
    }

    #[test]
    fn test_autocomplete_with_value() {
        use revue::widget::Autocomplete;

        let view = Autocomplete::new()
            .placeholder("Search fruits...")
            .suggestions(["Apple", "Apricot", "Avocado"])
            .value("Ap");

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("autocomplete_with_value");
    }
}
