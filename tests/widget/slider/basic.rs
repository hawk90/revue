use revue::widget::slider;
#[test] fn test_slider_new() { let s = slider(); }
#[test] fn test_slider_value() { let s = slider().value(0.5); }
