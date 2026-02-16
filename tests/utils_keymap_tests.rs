//! Tests for keymap utilities
//!
//! Extracted from src/utils/keymap.rs

use revue::event::Key;
use revue::event::KeyBinding;
use revue::utils::keymap::{
    emacs_preset, format_key_binding, parse_key_binding, vim_preset, KeyChord, KeymapConfig,
    LookupResult, Mode,
};

#[test]
fn test_parse_key_binding() {
    let binding = parse_key_binding("j").unwrap();
    assert_eq!(binding.key, Key::Char('j'));
    assert!(!binding.ctrl);

    let binding = parse_key_binding("Ctrl-c").unwrap();
    assert_eq!(binding.key, Key::Char('c'));
    assert!(binding.ctrl);

    let binding = parse_key_binding("Ctrl-Alt-Delete").unwrap();
    assert_eq!(binding.key, Key::Delete);
    assert!(binding.ctrl);
    assert!(binding.alt);
}

#[test]
fn test_format_key_binding() {
    let binding = KeyBinding {
        key: Key::Char('c'),
        ctrl: true,
        alt: false,
        shift: false,
    };
    assert_eq!(format_key_binding(&binding), "Ctrl-c");

    let binding = KeyBinding {
        key: Key::Enter,
        ctrl: false,
        alt: false,
        shift: false,
    };
    assert_eq!(format_key_binding(&binding), "Enter");
}

#[test]
fn test_key_chord_parse() {
    let chord = KeyChord::parse("Ctrl-x Ctrl-s").unwrap();
    assert_eq!(chord.keys.len(), 2);
    assert!(chord.keys[0].ctrl);
    assert!(chord.keys[1].ctrl);
}

#[test]
fn test_keymap_single_key() {
    let mut keymap = KeymapConfig::new();
    keymap.bind(Mode::Normal, "j", "move_down");

    let binding = parse_key_binding("j").unwrap();
    let result = keymap.lookup(binding);
    assert_eq!(result, LookupResult::Action("move_down".to_string()));
}

#[test]
fn test_keymap_multi_key() {
    let mut keymap = KeymapConfig::new();
    keymap.bind(Mode::Normal, "g g", "goto_first");

    let g1 = parse_key_binding("g").unwrap();
    let result = keymap.lookup(g1.clone());
    assert_eq!(result, LookupResult::Pending);

    let result = keymap.lookup(g1);
    assert_eq!(result, LookupResult::Action("goto_first".to_string()));
}

#[test]
fn test_keymap_no_match() {
    let mut keymap = KeymapConfig::new();
    keymap.bind(Mode::Normal, "j", "move_down");

    let binding = parse_key_binding("x").unwrap();
    let result = keymap.lookup(binding);
    assert_eq!(result, LookupResult::None);
}

#[test]
fn test_keymap_modes() {
    let mut keymap = KeymapConfig::new();
    keymap.bind(Mode::Normal, "i", "enter_insert");
    keymap.bind(Mode::Insert, "Escape", "exit_insert");

    keymap.set_mode(Mode::Normal);
    let i = parse_key_binding("i").unwrap();
    let result = keymap.lookup(i);
    assert_eq!(result, LookupResult::Action("enter_insert".to_string()));

    keymap.set_mode(Mode::Insert);
    let esc = parse_key_binding("Escape").unwrap();
    let result = keymap.lookup(esc);
    assert_eq!(result, LookupResult::Action("exit_insert".to_string()));
}

#[test]
fn test_vim_preset() {
    let mut keymap = vim_preset();

    let j = parse_key_binding("j").unwrap();
    let result = keymap.lookup(j);
    assert_eq!(result, LookupResult::Action("move_down".to_string()));
}

#[test]
fn test_emacs_preset() {
    let mut keymap = emacs_preset();

    let ctrl_n = parse_key_binding("Ctrl-n").unwrap();
    let result = keymap.lookup(ctrl_n);
    assert_eq!(result, LookupResult::Action("move_down".to_string()));
}

#[test]
fn test_global_bindings() {
    let mut keymap = KeymapConfig::new();
    keymap.bind_global("Ctrl-c", "quit");

    keymap.set_mode(Mode::Normal);
    let ctrl_c = parse_key_binding("Ctrl-c").unwrap();
    let result = keymap.lookup(ctrl_c.clone());
    assert_eq!(result, LookupResult::Action("quit".to_string()));

    keymap.set_mode(Mode::Insert);
    let result = keymap.lookup(ctrl_c);
    assert_eq!(result, LookupResult::Action("quit".to_string()));
}
