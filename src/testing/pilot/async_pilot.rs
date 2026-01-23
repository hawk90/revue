use super::core::Pilot;

/// Async test runner for Pilot
///
/// Provides a more ergonomic way to run async tests.
///
/// # Example
///
/// ```rust,ignore
/// use revue::testing::*;
///
/// #[tokio::test]
/// async fn test_async_pilot() {
///     let view = MyView::new();
///     let mut app = TestApp::new(view);
///     let pilot = Pilot::new(&mut app);
///
///     pilot.type_text("hello");
///     pilot.wait_ms_async(100).await;
///     pilot.assert_contains("hello");
/// }
/// ```
pub struct AsyncPilot;

impl AsyncPilot {
    /// Create a test runner that will run the test asynchronously
    #[cfg(feature = "async")]
    pub async fn run<V, F, Fut>(view: V, f: F)
    where
        V: crate::widget::View,
        F: FnOnce(Pilot<'_, V>) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let mut app = crate::testing::TestApp::new(view);
        let pilot = Pilot::new(&mut app);
        f(pilot).await;
    }
}
