//! TextArea find and replace tests

use revue::widget::input_widgets::textarea::{
    CursorPos, FindMatch, FindOptions, FindReplaceMode, FindReplaceState,
};
use revue::widget::TextArea;

#[test]
fn test_find_replace_mode_default() {
    assert_eq!(FindReplaceMode::default(), FindReplaceMode::Find);
}

#[test]
fn test_find_replace_mode_equality() {
    assert_eq!(FindReplaceMode::Find, FindReplaceMode::Find);
    assert_eq!(FindReplaceMode::Replace, FindReplaceMode::Replace);
    assert_ne!(FindReplaceMode::Find, FindReplaceMode::Replace);
}

#[test]
fn test_find_options_default() {
    let opts = FindOptions::default();
    assert!(!opts.case_sensitive);
    assert!(!opts.whole_word);
    assert!(!opts.use_regex);
}

#[test]
fn test_find_options_custom() {
    let opts = FindOptions {
        case_sensitive: true,
        whole_word: true,
        use_regex: false,
    };
    assert!(opts.case_sensitive);
    assert!(opts.whole_word);
    assert!(!opts.use_regex);
}

#[test]
fn test_find_match_new() {
    let start = CursorPos { line: 0, col: 5 };
    let end = CursorPos { line: 0, col: 10 };
    let match_result = FindMatch::new(start, end);

    assert_eq!(match_result.start.line, 0);
    assert_eq!(match_result.start.col, 5);
    assert_eq!(match_result.end.line, 0);
    assert_eq!(match_result.end.col, 10);
}

#[test]
fn test_find_match_equality() {
    let m1 = FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 5 });
    let m2 = FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 5 });
    let m3 = FindMatch::new(CursorPos { line: 1, col: 0 }, CursorPos { line: 1, col: 5 });

    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}

#[test]
fn test_find_match_clone() {
    let m = FindMatch::new(CursorPos { line: 2, col: 3 }, CursorPos { line: 2, col: 8 });
    let cloned = m.clone();
    assert_eq!(m, cloned);
}

#[test]
fn test_find_replace_state_default() {
    let state = FindReplaceState::default();
    assert!(state.query.is_empty());
    assert!(state.replace_with.is_empty());
    assert!(state.matches.is_empty());
    assert!(state.current_match.is_none());
    assert_eq!(state.mode, FindReplaceMode::Find);
}

#[test]
fn test_find_replace_state_new_find() {
    let state = FindReplaceState::new(FindReplaceMode::Find);
    assert_eq!(state.mode, FindReplaceMode::Find);
    assert!(state.query_focused);
}

#[test]
fn test_find_replace_state_new_replace() {
    let state = FindReplaceState::new(FindReplaceMode::Replace);
    assert_eq!(state.mode, FindReplaceMode::Replace);
    assert!(state.query_focused);
}

#[test]
fn test_find_replace_state_match_count_empty() {
    let state = FindReplaceState::default();
    assert_eq!(state.match_count(), 0);
}

#[test]
fn test_find_replace_state_match_count() {
    let state = FindReplaceState {
        matches: vec![
            FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 5 }),
            FindMatch::new(CursorPos { line: 1, col: 0 }, CursorPos { line: 1, col: 5 }),
            FindMatch::new(CursorPos { line: 2, col: 0 }, CursorPos { line: 2, col: 5 }),
        ],
        ..Default::default()
    };
    assert_eq!(state.match_count(), 3);
}

#[test]
fn test_find_replace_state_current_match_display_none() {
    let state = FindReplaceState::default();
    assert_eq!(state.current_match_display(), 0);
}

#[test]
fn test_find_replace_state_current_match_display() {
    let mut state = FindReplaceState {
        matches: vec![
            FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 5 }),
            FindMatch::new(CursorPos { line: 1, col: 0 }, CursorPos { line: 1, col: 5 }),
        ],
        current_match: Some(0),
        ..Default::default()
    };
    assert_eq!(state.current_match_display(), 1);

    state.current_match = Some(1);
    assert_eq!(state.current_match_display(), 2);
}

#[test]
fn test_find_replace_state_query_focused() {
    let state = FindReplaceState::new(FindReplaceMode::Find);
    assert!(state.query_focused);
}

#[test]
fn test_find_replace_state_with_data() {
    let mut state = FindReplaceState::new(FindReplaceMode::Replace);
    state.query = "search".to_string();
    state.replace_with = "replace".to_string();
    state.options = FindOptions {
        case_sensitive: true,
        whole_word: false,
        use_regex: false,
    };

    assert_eq!(state.query, "search");
    assert_eq!(state.replace_with, "replace");
    assert!(state.options.case_sensitive);
}

#[test]
fn test_find_replace_state_debug_find() {
    let state = FindReplaceState::new(FindReplaceMode::Find);
    let debug = format!("{:?}", state);
    assert!(debug.contains("FindReplaceState"));
    assert!(debug.contains("Find"));
}

#[test]
fn test_find_replace_state_debug_replace() {
    let state = FindReplaceState::new(FindReplaceMode::Replace);
    let debug = format!("{:?}", state);
    assert!(debug.contains("FindReplaceState"));
    assert!(debug.contains("Replace"));
}

#[test]
fn test_find_replace_options_debug() {
    let opts = FindOptions {
        case_sensitive: true,
        whole_word: false,
        use_regex: true,
    };
    let debug = format!("{:?}", opts);
    assert!(debug.contains("FindOptions"));
}

#[test]
fn test_find_replace_mode_debug() {
    let debug_find = format!("{:?}", FindReplaceMode::Find);
    let debug_replace = format!("{:?}", FindReplaceMode::Replace);
    assert!(debug_find.contains("Find"));
    assert!(debug_replace.contains("Replace"));
}

#[test]
fn test_find_replace_state_current_match_index() {
    let mut state = FindReplaceState {
        matches: vec![
            FindMatch::new(CursorPos { line: 0, col: 0 }, CursorPos { line: 0, col: 1 }),
            FindMatch::new(CursorPos { line: 1, col: 0 }, CursorPos { line: 1, col: 1 }),
        ],
        ..Default::default()
    };

    // Test boundary conditions
    assert_eq!(state.current_match_display(), 0); // No current match

    state.current_match = Some(0);
    assert_eq!(state.current_match_display(), 1);

    state.current_match = Some(1);
    assert_eq!(state.current_match_display(), 2);

    // Should handle out of bounds
    state.current_match = Some(99);
    assert_eq!(state.current_match_display(), 100);
}

#[test]
fn test_find_replace_state_clear_matches() {
    let mut state = FindReplaceState::new(FindReplaceMode::Find);
    state.matches = vec![FindMatch::new(
        CursorPos { line: 0, col: 0 },
        CursorPos { line: 0, col: 1 },
    )];
    state.current_match = Some(0);

    // Clear all matches
    state.matches.clear();
    state.current_match = None;

    assert_eq!(state.match_count(), 0);
    assert_eq!(state.current_match_display(), 0);
}

#[test]
fn test_find_replace_state_update_query() {
    let mut state = FindReplaceState::new(FindReplaceMode::Find);
    state.query = "old query".to_string();

    // Update query
    state.query = "new query".to_string();

    assert_eq!(state.query, "new query");
}

#[test]
fn test_find_replace_state_update_replace_text() {
    let mut state = FindReplaceState::new(FindReplaceMode::Replace);
    state.replace_with = "old replacement".to_string();

    // Update replacement text
    state.replace_with = "new replacement".to_string();

    assert_eq!(state.replace_with, "new replacement");
}

#[test]
fn test_find_replace_state_toggle_options() {
    let mut state = FindReplaceState::new(FindReplaceMode::Find);
    state.options = FindOptions::default();

    // Toggle case sensitivity
    state.options.case_sensitive = !state.options.case_sensitive;
    assert!(state.options.case_sensitive);

    // Toggle whole word
    state.options.whole_word = !state.options.whole_word;
    assert!(state.options.whole_word);

    // Toggle regex
    state.options.use_regex = !state.options.use_regex;
    assert!(state.options.use_regex);
}

fn replace_all(text: &str, query: &str, with: &str) -> String {
    let mut ta = TextArea::new().content(text);
    ta.open_replace();
    ta.set_find_query(query);
    ta.set_replace_text(with);
    ta.replace_all();
    ta.get_content()
}

fn match_count(text: &str, query: &str) -> usize {
    let mut ta = TextArea::new().content(text);
    ta.open_find();
    ta.set_find_query(query);
    ta.find_state().unwrap().matches.len()
}

#[test]
fn replace_all_does_not_lose_text_where_matches_could_overlap() {
    assert_eq!(replace_all("aaa", "aa", "b"), "ba");
    assert_eq!(replace_all("aaaa", "aa", "b"), "bb");
    assert_eq!(replace_all("abababa", "aba", "X"), "XbX");
}

#[test]
fn matches_do_not_overlap() {
    assert_eq!(match_count("aaa", "aa"), 1);
    assert_eq!(match_count("aaaa", "aa"), 2);
    assert_eq!(match_count("abababa", "aba"), 2);
}

#[test]
fn a_match_rejected_as_no_whole_word_does_not_hide_the_next() {
    let mut ta = TextArea::new().content("aa a");
    ta.open_find();
    ta.toggle_whole_word();
    ta.set_find_query("a");
    // "aa" is no whole word; the lone "a" is
    assert_eq!(ta.find_state().unwrap().matches.len(), 1);
}

#[test]
fn replace_all_rewrites_every_line() {
    assert_eq!(replace_all("ab\nab ab", "ab", "c"), "c\nc c");
}
