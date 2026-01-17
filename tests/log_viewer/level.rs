//! LogLevel tests

#![allow(unused_imports)]

use revue::widget::{log_filter, log_parser, log_viewer, AdvLogEntry, AdvLogLevel};

#[test]
fn test_log_level_ordering() {
    assert!(AdvLogLevel::Fatal > AdvLogLevel::Error);
    assert!(AdvLogLevel::Error > AdvLogLevel::Warning);
    assert!(AdvLogLevel::Warning > AdvLogLevel::Info);
    assert!(AdvLogLevel::Info > AdvLogLevel::Debug);
    assert!(AdvLogLevel::Debug > AdvLogLevel::Trace);
}

#[test]
fn test_log_level_parse() {
    assert_eq!(AdvLogLevel::parse("INFO"), Some(AdvLogLevel::Info));
    assert_eq!(AdvLogLevel::parse("info"), Some(AdvLogLevel::Info));
    assert_eq!(AdvLogLevel::parse("INF"), Some(AdvLogLevel::Info));
    assert_eq!(AdvLogLevel::parse("DEBUG"), Some(AdvLogLevel::Debug));
    assert_eq!(AdvLogLevel::parse("DBG"), Some(AdvLogLevel::Debug));
    assert_eq!(AdvLogLevel::parse("WARN"), Some(AdvLogLevel::Warning));
    assert_eq!(AdvLogLevel::parse("WARNING"), Some(AdvLogLevel::Warning));
    assert_eq!(AdvLogLevel::parse("ERROR"), Some(AdvLogLevel::Error));
    assert_eq!(AdvLogLevel::parse("FATAL"), Some(AdvLogLevel::Fatal));
    assert_eq!(AdvLogLevel::parse("CRITICAL"), Some(AdvLogLevel::Fatal));
    assert_eq!(AdvLogLevel::parse("invalid"), None);
}

#[test]
fn test_log_level_color() {
    let _ = AdvLogLevel::Info.color();
    let _ = AdvLogLevel::Error.color();
    let _ = AdvLogLevel::Warning.color();
}

#[test]
fn test_log_level_icon() {
    assert_eq!(AdvLogLevel::Info.icon(), '●');
    assert_eq!(AdvLogLevel::Warning.icon(), '⚠');
    assert_eq!(AdvLogLevel::Error.icon(), '✗');
}

#[test]
fn test_log_level_label() {
    assert_eq!(AdvLogLevel::Info.label(), "INF");
    assert_eq!(AdvLogLevel::Warning.label(), "WRN");
    assert_eq!(AdvLogLevel::Error.label(), "ERR");
    assert_eq!(AdvLogLevel::Fatal.label(), "FTL");
}
