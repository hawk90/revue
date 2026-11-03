//! #983: search opens collapsed arrays and finds null values; escapes are
//! decoded; only valid JSON is accepted

use revue::widget::data::json_viewer::{JsonViewer, Search};

#[test]
fn search_opens_a_collapsed_array() {
    let mut viewer = JsonViewer::from_content(r#"{"a":[{"needle":1}],"b":2}"#);
    viewer.collapse_all();
    viewer.search("needle");
    viewer.next_match();
    assert_eq!(viewer.selected_path().as_deref(), Some("$.a[0].needle"));
    assert!(!viewer.is_collapsed("$.a"));
    assert!(!viewer.is_collapsed("$.a[0]"));
}

#[test]
fn search_opens_nested_arrays() {
    let mut viewer = JsonViewer::from_content(r#"[[["deep"]],"x"]"#);
    viewer.collapse_all();
    viewer.search("deep");
    viewer.next_match();
    assert_eq!(viewer.selected_path().as_deref(), Some("$[0][0][0]"));
    assert_eq!(viewer.selected_value().as_deref(), Some("deep"));
}

#[test]
fn search_can_select_the_root() {
    let mut viewer = JsonViewer::from_content(r#""needle""#);
    viewer.search("needle");
    assert_eq!(viewer.match_count(), 1);
    viewer.next_match();
    assert_eq!(viewer.selected_path().as_deref(), Some("$"));
}

#[test]
fn search_finds_null_values() {
    let mut viewer = JsonViewer::from_content(r#"{"a":null,"b":"x"}"#);
    viewer.search("null");
    assert_eq!(viewer.match_count(), 1);
    viewer.next_match();
    assert_eq!(viewer.selected_path().as_deref(), Some("$.a"));
}

#[test]
fn unicode_escapes_are_decoded() {
    let mut viewer = JsonViewer::from_content("[\"\\u0041\\u00e9\"]");
    viewer.select_down();
    assert_eq!(viewer.selected_value().as_deref(), Some("Aé"));
}

#[test]
fn a_surrogate_pair_is_one_char_and_a_lone_one_is_a_replacement() {
    let mut viewer = JsonViewer::from_content(r#"["\ud83d\ude00", "\ud83dx", "\ude00"]"#);
    viewer.select_down();
    assert_eq!(viewer.selected_value().as_deref(), Some("😀"));
    viewer.select_down();
    assert_eq!(viewer.selected_value().as_deref(), Some("\u{fffd}x"));
    viewer.select_down();
    assert_eq!(viewer.selected_value().as_deref(), Some("\u{fffd}"));
}

#[test]
fn every_short_escape_is_decoded() {
    let mut viewer = JsonViewer::from_content(r#"["\"\\\/\b\f\n\r\t"]"#);
    viewer.select_down();
    assert_eq!(
        viewer.selected_value().as_deref(),
        Some("\"\\/\u{8}\u{c}\n\r\t")
    );
}

#[test]
fn escapes_in_keys_are_decoded() {
    let mut viewer = JsonViewer::from_content("{\"a\\\"b\": 1, \"\\u0063\": 2}");
    viewer.select_down();
    assert_eq!(viewer.selected_path().as_deref(), Some("$.a\"b"));
    viewer.select_down();
    assert_eq!(viewer.selected_path().as_deref(), Some("$.c"));
}

#[test]
fn invalid_json_is_rejected() {
    for bad in [
        "[1 2]",
        "[1,]",
        "[,1]",
        "{\"a\" 1}",
        "{\"a\"::1}",
        "{\"a\":1,}",
        "{,}",
        "{\"a\":1 \"b\":2}",
        "{a:1}",
        "-",
        "1-2e+.",
        "01",
        "1.",
        ".5",
        "1e",
        "1e+",
        "+1",
        "\"open",
        "\"bad \\x escape\"",
        "\"bad \\u12 escape\"",
        "\"tab\tinside\"",
        "tru",
        "nul",
        "[true false]",
        "[1] 2",
        "{} x",
    ] {
        assert!(!JsonViewer::from_content(bad).has_data(), "{bad:?}");
    }
}

#[test]
fn valid_json_is_accepted() {
    for good in [
        "0",
        "-0",
        "-12.5e-3",
        "1E+2",
        "[]",
        "{}",
        " [ 1 , 2 ] ",
        "{\"a\":{\"b\":[null,true,false]}}",
        "\"\"",
        "\r\n\t[1]\r\n\t",
    ] {
        assert!(JsonViewer::from_content(good).has_data(), "{good:?}");
    }
}

#[test]
fn numbers_keep_their_text() {
    let mut viewer = JsonViewer::from_content("[-12.5e-3, 0]");
    viewer.select_down();
    assert_eq!(viewer.selected_value().as_deref(), Some("-12.5e-3"));
    viewer.select_down();
    assert_eq!(viewer.selected_value().as_deref(), Some("0"));
}
