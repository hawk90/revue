//! Terminal detection utilities tests

use revue::utils::terminal::{terminal_type, TerminalType};

#[test]
fn test_terminal_type_default() {
    // In most test environments, this will be Unknown
    let term = terminal_type();
    // Just verify it doesn't panic
    let _ = term;
}

#[test]
fn test_terminal_type_default_is_unknown() {
    assert_eq!(TerminalType::default(), TerminalType::Unknown);
}

#[test]
fn test_terminal_type_kitty_variant() {
    let term_type = TerminalType::Kitty;
    assert!(!matches!(term_type, TerminalType::Unknown));
}

#[test]
fn test_terminal_type_iterm2_variant() {
    let term_type = TerminalType::Iterm2;
    assert!(!matches!(term_type, TerminalType::Unknown));
}

#[test]
fn test_terminal_type_unknown_variant() {
    let term_type = TerminalType::Unknown;
    assert!(matches!(term_type, TerminalType::Unknown));
}

#[test]
fn test_terminal_type_equality() {
    assert_eq!(TerminalType::Kitty, TerminalType::Kitty);
    assert_eq!(TerminalType::Iterm2, TerminalType::Iterm2);
    assert_eq!(TerminalType::Unknown, TerminalType::Unknown);
    assert_ne!(TerminalType::Kitty, TerminalType::Iterm2);
    assert_ne!(TerminalType::Kitty, TerminalType::Unknown);
}

#[test]
fn test_terminal_type_clone() {
    let term_type = TerminalType::Kitty;
    let cloned = term_type;
    assert_eq!(term_type, cloned);
}

#[test]
fn test_terminal_type_copy() {
    let term_type = TerminalType::Iterm2;
    let copied = term_type;
    assert_eq!(term_type, copied);
}

#[test]
fn test_terminal_type_multiple_calls_cached() {
    // terminal_type uses OnceLock for caching
    let term1 = terminal_type();
    let term2 = terminal_type();
    assert_eq!(term1, term2);
}
