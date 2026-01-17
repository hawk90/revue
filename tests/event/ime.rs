//! IME (Input Method Editor) tests

use revue::event::{
    Candidate, CompositionEvent, CompositionState, ImeConfig, ImeState, PreeditString,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn test_composition_state_default() {
    let ime = ImeState::new();
    assert_eq!(ime.state(), CompositionState::Idle);
    assert!(!ime.is_composing());
}

#[test]
fn test_start_composition() {
    let mut ime = ImeState::new();
    ime.start_composition();

    assert_eq!(ime.state(), CompositionState::Composing);
    assert!(ime.is_composing());
}

#[test]
fn test_update_composition() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("かん", 2);

    assert_eq!(ime.composing_text(), "かん");
    assert_eq!(ime.cursor(), 2);
}

#[test]
fn test_commit() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("かん", 2);

    let result = ime.commit("漢");

    assert_eq!(result, Some("漢".to_string()));
    assert!(!ime.is_composing());
}

#[test]
fn test_cancel() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("test", 4);
    ime.cancel();

    assert!(!ime.is_composing());
    assert!(ime.composing_text().is_empty());
}

#[test]
fn test_candidates() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("kan", 3);

    ime.set_candidates(vec![
        Candidate::new("漢"),
        Candidate::new("感"),
        Candidate::new("間"),
    ]);

    assert_eq!(ime.candidates().len(), 3);
    assert_eq!(ime.selected_candidate(), 0);
    assert_eq!(ime.selected_text(), Some("漢"));
}

#[test]
fn test_candidate_navigation() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.set_candidates(vec![
        Candidate::new("a"),
        Candidate::new("b"),
        Candidate::new("c"),
    ]);

    assert_eq!(ime.selected_candidate(), 0);

    ime.next_candidate();
    assert_eq!(ime.selected_candidate(), 1);

    ime.next_candidate();
    assert_eq!(ime.selected_candidate(), 2);

    ime.next_candidate(); // Wraps around
    assert_eq!(ime.selected_candidate(), 0);

    ime.prev_candidate(); // Wraps around
    assert_eq!(ime.selected_candidate(), 2);
}

#[test]
fn test_commit_selected() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.set_candidates(vec![Candidate::new("first"), Candidate::new("second")]);
    ime.next_candidate();

    let result = ime.commit_selected();
    assert_eq!(result, Some("second".to_string()));
}

#[test]
fn test_backspace() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("test", 4);

    assert!(ime.backspace());
    assert_eq!(ime.composing_text(), "tes");
    assert_eq!(ime.cursor(), 3);
}

#[test]
fn test_backspace_cancels_empty() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("a", 1);

    ime.backspace();
    assert!(!ime.is_composing());
}

#[test]
fn test_cursor_movement() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("test", 4);

    ime.move_cursor_left();
    assert_eq!(ime.cursor(), 3);

    ime.move_cursor_left();
    ime.move_cursor_left();
    ime.move_cursor_left();
    assert_eq!(ime.cursor(), 0);

    ime.move_cursor_left(); // Should not go below 0
    assert_eq!(ime.cursor(), 0);

    ime.move_cursor_right();
    assert_eq!(ime.cursor(), 1);
}

#[test]
fn test_ime_callback() {
    let mut ime = ImeState::new();
    let event_count = Arc::new(AtomicUsize::new(0));
    let count_clone = Arc::clone(&event_count);

    ime.on_composition(move |_| {
        count_clone.fetch_add(1, Ordering::SeqCst);
    });

    ime.start_composition();
    ime.update_composition("a", 1);
    ime.commit("A");

    assert_eq!(event_count.load(Ordering::SeqCst), 3); // start, update, end
}

#[test]
fn test_disabled_ime() {
    let mut ime = ImeState::new();
    ime.disable();

    ime.start_composition();
    assert!(!ime.is_composing());
}

#[test]
fn test_candidate_builder() {
    let candidate = Candidate::new("漢")
        .with_label("1")
        .with_annotation("Chinese character");

    assert_eq!(candidate.text, "漢");
    assert_eq!(candidate.label, Some("1".to_string()));
    assert_eq!(candidate.annotation, Some("Chinese character".to_string()));
}

#[test]
fn test_preedit_string() {
    let mut ime = ImeState::new();
    ime.start_composition();
    ime.update_composition("hello", 2);

    let preedit = PreeditString::from_ime(&ime);
    assert!(!preedit.is_empty());
    assert_eq!(preedit.text(), "hello");
}

#[test]
fn test_ime_config() {
    use revue::event::CompositionStyle;

    let config = ImeConfig {
        composition_style: CompositionStyle::Highlight,
        show_candidates: false,
        max_candidates: 5,
        candidate_offset: (1, 2),
        inline_composition: true,
    };

    let ime = ImeState::with_config(config);
    assert_eq!(ime.config().max_candidates, 5);
}

// =============================================================================
