//! Values with structure in them: functions, lists and comments (#866, #1206)

use revue::style::{parse_css, Easing, KeyframeAnimation, Transition, Transitions};
use revue::testing::MockTime;
use std::time::Duration;

const BEZIER: Easing = Easing::CubicBezier(0.1, 0.7, 1.0, 0.1);

// ---------------------------------------------------------------------------
// Transitions
// ---------------------------------------------------------------------------

#[test]
fn a_cubic_bezier_stays_in_one_piece_in_a_transition() {
    let list = Transitions::parse("opacity 0.3s cubic-bezier(0.1, 0.7, 1.0, 0.1)");
    assert_eq!(list.items.len(), 1);
    assert_eq!(list.items[0].easing, BEZIER);
    assert_eq!(list.items[0].duration, Duration::from_millis(300));
}

#[test]
fn transitions_split_at_the_commas_between_them_only() {
    let list = Transitions::parse(
        "opacity 1s cubic-bezier(0.1, 0.7, 1.0, 0.1) 2s, color 3s, width 4s linear",
    );
    let names: Vec<_> = list.items.iter().map(|t| t.property.as_str()).collect();
    assert_eq!(names, ["opacity", "color", "width"]);
    assert_eq!(list.items[0].delay, Duration::from_secs(2));
}

#[test]
fn an_empty_entry_is_no_transition() {
    assert_eq!(Transitions::parse("opacity 1s, , color 1s,").items.len(), 2);
    assert!(Transitions::parse("").is_empty());
}

#[test]
fn the_later_transition_wins_in_either_order() {
    let list = Transitions::parse("all 1s, opacity 200ms");
    assert_eq!(
        list.get("opacity").unwrap().duration,
        Duration::from_millis(200)
    );
    assert_eq!(list.get("color").unwrap().duration, Duration::from_secs(1));

    let list = Transitions::parse("opacity 200ms, all 1s");
    assert_eq!(
        list.get("opacity").unwrap().duration,
        Duration::from_secs(1)
    );
}

#[test]
fn a_single_transition_parses_a_bezier_with_spaces() {
    let one = Transition::parse("opacity cubic-bezier( 0.1 , 0.7 , 1.0 , 0.1 ) 1s").unwrap();
    assert_eq!(one.easing, BEZIER);
    assert_eq!(one.duration, Duration::from_secs(1));
}

// ---------------------------------------------------------------------------
// The curve
// ---------------------------------------------------------------------------

fn close(got: f32, want: f32) {
    assert!((got - want).abs() < 0.001, "{got} is not {want}");
}

#[test]
fn a_bezier_follows_time_not_its_parameter() {
    // Both x control points at 0: x = s^3, y = 1 - (1 - s)^3.
    let curve = Easing::CubicBezier(0.0, 1.0, 0.0, 1.0);
    for t in [0.002_f32, 0.1, 0.5, 0.9] {
        let s = t.cbrt();
        close(curve.apply(t), 1.0 - (1.0 - s).powi(3));
    }
}

#[test]
fn the_straight_bezier_is_linear() {
    let curve = Easing::CubicBezier(0.25, 0.25, 0.75, 0.75);
    for t in [0.0, 0.1, 0.5, 0.77, 1.0] {
        close(curve.apply(t), t);
    }
}

#[test]
fn a_bezier_starts_at_0_and_ends_at_1_whatever_its_points() {
    let curve = Easing::CubicBezier(0.3, -2.0, 0.7, 3.0);
    assert_eq!(curve.apply(0.0), 0.0);
    assert_eq!(curve.apply(1.0), 1.0);
    assert_eq!(curve.apply(-1.0), 0.0);
    assert_eq!(curve.apply(2.0), 1.0);
}

#[test]
fn a_bezier_may_overshoot_in_both_directions() {
    let curve = Easing::CubicBezier(0.3, -1.0, 0.7, 2.0);
    assert!(curve.apply(0.15) < 0.0, "{}", curve.apply(0.15));
    assert!(curve.apply(0.85) > 1.0, "{}", curve.apply(0.85));
}

#[test]
fn a_bezier_with_x_outside_the_range_is_invalid_and_y_is_not() {
    assert!(Easing::parse("cubic-bezier(1.5, 0, 1, 1)").is_none());
    assert!(Easing::parse("cubic-bezier(0, 0, -0.1, 1)").is_none());
    assert!(Easing::parse("cubic-bezier(0, 0, NaN, 1)").is_none());
    assert!(Easing::parse("cubic-bezier(0, inf, 1, 1)").is_none());
    assert!(Easing::parse("cubic-bezier(0, 0, 1)").is_none());
    assert_eq!(
        Easing::parse("cubic-bezier(0, -3, 1, 3)"),
        Some(Easing::CubicBezier(0.0, -3.0, 1.0, 3.0))
    );
    assert!(
        Easing::parse("cubic-bezier(0, 0, 1, 1)").is_some(),
        "x at its limits"
    );
}

// ---------------------------------------------------------------------------
// Animations
// ---------------------------------------------------------------------------

/// The animation of `.a` given these declarations and a `fade` from 0 to 1.
fn animation(declarations: &str) -> Option<KeyframeAnimation> {
    let css = format!(
        "@keyframes fade {{ from {{ opacity: 0; }} to {{ opacity: 1; }} }} .a {{ {declarations} }}"
    );
    parse_css(&css).unwrap().animation(".a")
}

/// Opacity at this share of a 1000ms animation.
fn opacity_at(declarations: &str, millis: u64) -> f32 {
    let time = MockTime::new();
    let _clock = time.install();
    let mut anim = animation(declarations).expect("an animation");
    anim.start();
    time.advance_ms(millis);
    anim.get("opacity")
}

/// `cubic-bezier(0, 1, 0, 1)` at 10%: linear gives 0.1, ease-in-out 0.02.
fn fast_start() -> f32 {
    let s = 0.1_f32.cbrt();
    1.0 - (1.0 - s).powi(3)
}

#[test]
fn the_timing_function_runs_the_curve_it_names() {
    let got = opacity_at(
        "animation-name: fade; animation-duration: 1s; \
         animation-timing-function: cubic-bezier(0, 1, 0, 1);",
        100,
    );
    close(got, fast_start());
}

#[test]
fn the_shorthand_runs_the_curve_it_names() {
    let got = opacity_at("animation: fade 1s cubic-bezier(0, 1, 0, 1);", 100);
    close(got, fast_start());
}

#[test]
fn an_overshooting_curve_stays_within_the_keyframes() {
    let css = "animation: fade 1s cubic-bezier(0.3, -1, 0.7, 2);";
    assert_eq!(opacity_at(css, 150), 0.0);
    assert_eq!(opacity_at(css, 850), 1.0);
}

#[test]
fn a_count_of_zero_runs_nothing_and_infinite_runs_forever() {
    let long = "animation-name: fade; animation-iteration-count:";
    assert!(animation(&format!("{long} 0;")).is_none());
    assert!(animation("animation: fade 1s 0;").is_none());

    assert_eq!(
        animation(&format!("{long} infinite;")).unwrap().iterations,
        0
    );
    assert_eq!(
        animation("animation: fade 1s infinite;")
            .unwrap()
            .iterations,
        0
    );
    assert_eq!(animation(&format!("{long} 1;")).unwrap().iterations, 1);
    assert_eq!(animation("animation: fade 1s 3;").unwrap().iterations, 3);
}

// ---------------------------------------------------------------------------
// Keyframe selector lists
// ---------------------------------------------------------------------------

#[test]
fn a_keyframe_selector_list_gives_each_stop_the_block() {
    let sheet = parse_css("@keyframes p { 0%, 100% { opacity: 1; } 50% { opacity: 0; } }").unwrap();
    let blocks = &sheet.keyframes_definition("p").unwrap().keyframes;
    let stops: Vec<_> = blocks.iter().map(|b| b.percent).collect();
    assert_eq!(stops, [0, 100, 50]);
    for block in &blocks[..2] {
        assert_eq!(block.declarations.len(), 1);
        assert_eq!(block.declarations[0].value, "1");
    }
}

#[test]
fn a_list_runs_in_time_order_whatever_order_it_is_written_in() {
    let css = "@keyframes p { 0%, 100% { opacity: 1; } 50% { opacity: 0; } } \
               .a { animation: p 1s linear; }";
    let time = MockTime::new();
    let _clock = time.install();
    let mut anim = parse_css(css).unwrap().animation(".a").unwrap();
    anim.start();
    time.advance_ms(500);
    close(anim.get("opacity"), 0.0);
    time.advance_ms(250);
    close(anim.get("opacity"), 0.5);
}

#[test]
fn a_keyframe_list_takes_spaces_comments_and_from_and_to() {
    let sheet = parse_css("@keyframes p { from , /* both */ to{ opacity: 1; } }").unwrap();
    let stops: Vec<_> = sheet
        .keyframes_definition("p")
        .unwrap()
        .keyframes
        .iter()
        .map(|b| b.percent)
        .collect();
    assert_eq!(stops, [0, 100]);
}

#[test]
fn a_keyframe_list_with_a_bad_or_missing_stop_is_an_error() {
    assert!(parse_css("@keyframes p { 0%, nope { opacity: 1; } }").is_err());
    assert!(parse_css("@keyframes p { 0%, { opacity: 1; } }").is_err());
}

// ---------------------------------------------------------------------------
// Comments in values
// ---------------------------------------------------------------------------

fn value(css: &str, property: &str) -> String {
    let sheet = parse_css(css).unwrap();
    let rule = sheet
        .rules
        .iter()
        .find(|r| r.selector == ".a")
        .expect("rule .a");
    rule.declarations
        .iter()
        .find(|d| d.property == property)
        .map(|d| d.value.clone())
        .expect("the declaration")
}

#[test]
fn a_comment_is_not_part_of_a_value() {
    assert_eq!(value(".a { color: /* brand */ red; }", "color"), "red");
    assert_eq!(value(".a { color: red /* brand */; }", "color"), "red");
}

#[test]
fn a_comment_separates_like_a_space() {
    assert_eq!(value(".a { margin: 1/* a */2; }", "margin"), "1 2");
}

#[test]
fn nothing_in_a_comment_ends_the_rule_or_the_value() {
    let css = ".a { color: /* } ; */ red; width: 1; } .b { color: blue; }";
    assert_eq!(value(css, "color"), "red");
    assert_eq!(value(css, "width"), "1");
    assert_eq!(parse_css(css).unwrap().rules.len(), 2);
}

#[test]
fn a_comment_inside_a_function_is_dropped_too() {
    assert_eq!(
        value(".a { color: rgb(1, /* g */ 2, 3); }", "color"),
        "rgb(1,   2, 3)"
    );
}

#[test]
fn comment_marks_in_a_string_are_the_string() {
    assert_eq!(
        value(r#".a { content: "/* x */"; }"#, "content"),
        r#""/* x */""#
    );
    assert_eq!(value(".a { content: '/* x */'; }", "content"), "'/* x */'");
}

#[test]
fn a_broken_string_ends_at_the_line_end() {
    // As in CSS, the broken declaration runs to the next `;` and the rest stands.
    let css = ".a { content: \"open;\n width: 1; color: red; }";
    assert_eq!(value(css, "color"), "red");
}

#[test]
fn an_unterminated_comment_in_a_value_is_an_error_not_a_hang() {
    assert!(parse_css(".a { color: red /* never closed").is_err());
}

// ---------------------------------------------------------------------------
// Grid tracks
// ---------------------------------------------------------------------------

fn tracks(template: &str) -> usize {
    let css = format!(".a {{ grid-template-columns: {template}; }}");
    parse_css(&css)
        .unwrap()
        .apply(".a", &revue::style::Style::default())
        .layout
        .grid_template_columns
        .tracks
        .len()
}

#[test]
fn repeats_add_up_to_the_limit_and_no_further() {
    assert_eq!(tracks("repeat(10000, 1fr)"), 10_000);
    assert_eq!(tracks("repeat(9999, 1fr) 1fr"), 10_000);
    assert_eq!(tracks("repeat(5000, 1fr) repeat(5000, 1fr)"), 10_000);
    assert_eq!(tracks("repeat(5000, 1fr) repeat(5000, 1fr) 1fr"), 10_000);
    assert_eq!(tracks("repeat(10000, 1fr) minmax(1px, 1fr)"), 10_000);
}

#[test]
fn many_repeats_do_not_expand_past_the_limit() {
    let template = "repeat(10000, 1fr) ".repeat(2000);
    assert_eq!(tracks(&template), 10_000);
}
