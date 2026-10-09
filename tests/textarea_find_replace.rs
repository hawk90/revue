//! TextArea find and replace: matches do not overlap, so replacing every
//! match rewrites each piece of text once (#888)

use revue::widget::TextArea;

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
