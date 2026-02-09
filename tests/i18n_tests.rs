//! Integration tests for i18n utilities
//! Extracted from src/utils/i18n.rs

use revue::utils::i18n::*;

#[test]
fn test_i18n_basic() {
    let mut i18n = I18n::new();
    i18n.add_translation("en", "hello", "Hello");

    assert_eq!(i18n.t("hello"), "Hello");
    assert_eq!(i18n.t("missing"), "missing"); // Returns key if not found
}

#[test]
fn test_i18n_multiple_locales() {
    let mut i18n = I18n::new();
    i18n.add_locale(Locale::korean());
    i18n.add_locale(Locale::japanese());

    i18n.add_translation("en", "hello", "Hello");
    i18n.add_translation("ko", "hello", "안녕하세요");
    i18n.add_translation("ja", "hello", "こんにちは");

    assert_eq!(i18n.t("hello"), "Hello");

    i18n.set_locale("ko");
    assert_eq!(i18n.t("hello"), "안녕하세요");

    i18n.set_locale("ja");
    assert_eq!(i18n.t("hello"), "こんにちは");
}

#[test]
fn test_i18n_fallback() {
    let mut i18n = I18n::new();
    i18n.add_locale(Locale::korean());

    i18n.add_translation("en", "hello", "Hello");
    i18n.add_translation("en", "world", "World");
    i18n.add_translation("ko", "hello", "안녕하세요");
    // "world" not translated in Korean

    i18n.set_locale("ko");
    assert_eq!(i18n.t("hello"), "안녕하세요");
    assert_eq!(i18n.t("world"), "World"); // Falls back to English
}

#[test]
fn test_i18n_args() {
    let mut i18n = I18n::new();
    i18n.add_translation("en", "greeting", "Hello, {name}!");

    let result = i18n.t_args("greeting", &[("name", "World")]);
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_i18n_plural() {
    let mut i18n = I18n::new();
    i18n.add_translation(
        "en",
        "items",
        Translation::with_plural("1 item", "{n} items"),
    );

    assert_eq!(i18n.t_plural("items", 1), "1 item");
    assert_eq!(i18n.t_plural("items", 5), "{n} items");
}

#[test]
fn test_locale_direction() {
    let mut i18n = I18n::new();
    i18n.add_locale(Locale::arabic());

    i18n.set_locale("en");
    assert!(!i18n.is_rtl());

    i18n.set_locale("ar");
    assert!(i18n.is_rtl());
}

#[test]
fn test_russian_plural_rule() {
    let locale = Locale::russian();

    assert_eq!(locale.get_plural_form(1), 0); // яблоко
    assert_eq!(locale.get_plural_form(2), 1); // яблока
    assert_eq!(locale.get_plural_form(5), 2); // яблок
    assert_eq!(locale.get_plural_form(21), 0); // яблоко
    assert_eq!(locale.get_plural_form(22), 1); // яблока
    assert_eq!(locale.get_plural_form(25), 2); // яблок
}

#[test]
fn test_available_locales() {
    let mut i18n = I18n::new();
    i18n.add_locale(Locale::korean());
    i18n.add_locale(Locale::japanese());

    let locales = i18n.available_locales();
    assert_eq!(locales.len(), 3); // en, ko, ja
}
