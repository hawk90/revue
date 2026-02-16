//! Internationalization (i18n) utilities
//!
//! Provides simple translation support for TUI applications.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::i18n::{I18n, Locale};
//!
//! let mut i18n = I18n::new();
//!
//! // Add translations
//! i18n.add_translation("en", "hello", "Hello");
//! i18n.add_translation("ko", "hello", "안녕하세요");
//! i18n.add_translation("ja", "hello", "こんにちは");
//!
//! // Set locale
//! i18n.set_locale("ko");
//!
//! // Get translation
//! assert_eq!(i18n.t("hello"), "안녕하세요");
//! ```

use std::collections::HashMap;

/// Locale identifier (e.g., "en", "ko", "ja", "en-US")
pub type LocaleId = String;

/// Translation key
pub type TranslationKey = String;

/// Pluralization rule function
pub type PluralRule = fn(n: usize) -> usize;

/// Default plural rule (English-like: 1 = singular, else plural)
pub fn default_plural_rule(n: usize) -> usize {
    if n == 1 {
        0
    } else {
        1
    }
}

/// Locale configuration
#[derive(Clone, Debug)]
pub struct Locale {
    /// Locale identifier
    pub id: LocaleId,
    /// Display name
    pub name: String,
    /// Native name
    pub native_name: String,
    /// Direction (ltr or rtl)
    pub direction: Direction,
    /// Plural rule
    plural_rule: PluralRule,
}

/// Text direction
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    /// Left to right
    #[default]
    Ltr,
    /// Right to left
    Rtl,
}

impl Locale {
    /// Create a new locale
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let id = id.into();
        let name = name.into();
        Self {
            native_name: name.clone(),
            id,
            name,
            direction: Direction::Ltr,
            plural_rule: default_plural_rule,
        }
    }

    /// Set native name
    pub fn native_name(mut self, name: impl Into<String>) -> Self {
        self.native_name = name.into();
        self
    }

    /// Set direction
    pub fn direction(mut self, dir: Direction) -> Self {
        self.direction = dir;
        self
    }

    /// Set plural rule
    pub fn plural_rule(mut self, rule: PluralRule) -> Self {
        self.plural_rule = rule;
        self
    }

    /// Get plural form index for a number
    pub fn get_plural_form(&self, n: usize) -> usize {
        (self.plural_rule)(n)
    }
}

/// Common locales
impl Locale {
    /// English (US)
    pub fn english() -> Self {
        Locale::new("en", "English").native_name("English")
    }

    /// Korean
    pub fn korean() -> Self {
        Locale::new("ko", "Korean")
            .native_name("한국어")
            .plural_rule(|_| 0) // Korean doesn't have plural forms
    }

    /// Japanese
    pub fn japanese() -> Self {
        Locale::new("ja", "Japanese")
            .native_name("日本語")
            .plural_rule(|_| 0) // Japanese doesn't have plural forms
    }

    /// Chinese (Simplified)
    pub fn chinese_simplified() -> Self {
        Locale::new("zh-CN", "Chinese (Simplified)")
            .native_name("简体中文")
            .plural_rule(|_| 0)
    }

    /// Chinese (Traditional)
    pub fn chinese_traditional() -> Self {
        Locale::new("zh-TW", "Chinese (Traditional)")
            .native_name("繁體中文")
            .plural_rule(|_| 0)
    }

    /// Spanish
    pub fn spanish() -> Self {
        Locale::new("es", "Spanish").native_name("Español")
    }

    /// French
    pub fn french() -> Self {
        Locale::new("fr", "French")
            .native_name("Français")
            .plural_rule(|n| if n <= 1 { 0 } else { 1 })
    }

    /// German
    pub fn german() -> Self {
        Locale::new("de", "German").native_name("Deutsch")
    }

    /// Russian
    pub fn russian() -> Self {
        Locale::new("ru", "Russian")
            .native_name("Русский")
            .plural_rule(|n| {
                // Russian has complex plural rules
                if n % 10 == 1 && n % 100 != 11 {
                    0
                } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
                    1
                } else {
                    2
                }
            })
    }

    /// Arabic
    pub fn arabic() -> Self {
        Locale::new("ar", "Arabic")
            .native_name("العربية")
            .direction(Direction::Rtl)
            .plural_rule(|n| {
                // Arabic has 6 plural forms
                if n == 0 {
                    0
                } else if n == 1 {
                    1
                } else if n == 2 {
                    2
                } else if n % 100 >= 3 && n % 100 <= 10 {
                    3
                } else if n % 100 >= 11 {
                    4
                } else {
                    5
                }
            })
    }
}

/// Translation entry with plural forms
#[derive(Clone, Debug)]
pub struct Translation {
    /// Singular or only form
    singular: String,
    /// Plural forms (if any)
    plurals: Vec<String>,
}

impl Translation {
    /// Create simple translation
    pub fn simple(text: impl Into<String>) -> Self {
        Self {
            singular: text.into(),
            plurals: Vec::new(),
        }
    }

    /// Create translation with plural form
    pub fn with_plural(singular: impl Into<String>, plural: impl Into<String>) -> Self {
        Self {
            singular: singular.into(),
            plurals: vec![plural.into()],
        }
    }

    /// Create translation with multiple plural forms
    pub fn with_plurals(singular: impl Into<String>, plurals: Vec<String>) -> Self {
        Self {
            singular: singular.into(),
            plurals,
        }
    }

    /// Get text for plural form index
    pub fn get(&self, form: usize) -> &str {
        if form == 0 || self.plurals.is_empty() {
            &self.singular
        } else {
            self.plurals.get(form - 1).unwrap_or(&self.singular)
        }
    }
}

impl<S: Into<String>> From<S> for Translation {
    fn from(s: S) -> Self {
        Translation::simple(s)
    }
}

/// Internationalization manager
#[derive(Clone, Debug)]
pub struct I18n {
    /// Available locales
    locales: HashMap<LocaleId, Locale>,
    /// Translations per locale
    translations: HashMap<LocaleId, HashMap<TranslationKey, Translation>>,
    /// Current locale
    current_locale: LocaleId,
    /// Fallback locale
    fallback_locale: LocaleId,
}

impl I18n {
    /// Create new i18n instance with English as default
    pub fn new() -> Self {
        let mut instance = Self {
            locales: HashMap::new(),
            translations: HashMap::new(),
            current_locale: "en".to_string(),
            fallback_locale: "en".to_string(),
        };
        instance.add_locale(Locale::english());
        instance
    }

    /// Create with specific default locale
    pub fn with_locale(locale: Locale) -> Self {
        let id = locale.id.clone();
        let mut instance = Self {
            locales: HashMap::new(),
            translations: HashMap::new(),
            current_locale: id.clone(),
            fallback_locale: id,
        };
        instance.add_locale(locale);
        instance
    }

    /// Add a locale
    pub fn add_locale(&mut self, locale: Locale) {
        let id = locale.id.clone();
        self.locales.insert(id.clone(), locale);
        self.translations.entry(id).or_default();
    }

    /// Set current locale
    pub fn set_locale(&mut self, locale: impl Into<String>) {
        let locale = locale.into();
        if self.locales.contains_key(&locale) {
            self.current_locale = locale;
        }
    }

    /// Get current locale
    pub fn locale(&self) -> &str {
        &self.current_locale
    }

    /// Get current locale config
    pub fn current_locale(&self) -> Option<&Locale> {
        self.locales.get(&self.current_locale)
    }

    /// Set fallback locale
    pub fn set_fallback(&mut self, locale: impl Into<String>) {
        self.fallback_locale = locale.into();
    }

    /// Get available locales
    pub fn available_locales(&self) -> Vec<&Locale> {
        self.locales.values().collect()
    }

    /// Add a simple translation
    pub fn add_translation(&mut self, locale: &str, key: &str, value: impl Into<Translation>) {
        if let Some(translations) = self.translations.get_mut(locale) {
            translations.insert(key.to_string(), value.into());
        }
    }

    /// Add translations from a map
    pub fn add_translations(&mut self, locale: &str, translations: HashMap<String, Translation>) {
        if let Some(existing) = self.translations.get_mut(locale) {
            existing.extend(translations);
        }
    }

    /// Translate a key
    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.get_translation(key, &self.current_locale)
            .or_else(|| self.get_translation(key, &self.fallback_locale))
            .unwrap_or(key)
    }

    /// Translate with arguments (simple replacement)
    pub fn t_args(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut result = self.t(key).to_string();
        for (name, value) in args {
            result = result.replace(&format!("{{{}}}", name), value);
        }
        result
    }

    /// Translate with plural form
    pub fn t_plural<'a>(&'a self, key: &'a str, n: usize) -> &'a str {
        let form = self
            .current_locale()
            .map(|l| l.get_plural_form(n))
            .unwrap_or(if n == 1 { 0 } else { 1 });

        self.get_translation_with_form(key, &self.current_locale, form)
            .or_else(|| self.get_translation_with_form(key, &self.fallback_locale, form))
            .unwrap_or(key)
    }

    /// Translate with plural and arguments
    pub fn t_plural_args(&self, key: &str, n: usize, args: &[(&str, &str)]) -> String {
        let mut result = self.t_plural(key, n).to_string();
        for (name, value) in args {
            result = result.replace(&format!("{{{}}}", name), value);
        }
        result
    }

    /// Get translation for a locale
    fn get_translation(&self, key: &str, locale: &str) -> Option<&str> {
        self.translations
            .get(locale)
            .and_then(|t| t.get(key))
            .map(|t| t.get(0))
    }

    /// Get translation with specific plural form
    fn get_translation_with_form(&self, key: &str, locale: &str, form: usize) -> Option<&str> {
        self.translations
            .get(locale)
            .and_then(|t| t.get(key))
            .map(|t| t.get(form))
    }

    /// Check if translation exists
    pub fn has_translation(&self, key: &str) -> bool {
        self.translations
            .get(&self.current_locale)
            .map(|t| t.contains_key(key))
            .unwrap_or(false)
    }

    /// Get text direction for current locale
    pub fn direction(&self) -> Direction {
        self.current_locale()
            .map(|l| l.direction)
            .unwrap_or_default()
    }

    /// Check if current locale is RTL
    pub fn is_rtl(&self) -> bool {
        self.direction() == Direction::Rtl
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for creating translations
#[macro_export]
macro_rules! translations {
    ($locale:expr => { $($key:expr => $value:expr),* $(,)? }) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key.to_string(), $crate::utils::i18n::Translation::simple($value));
        )*
        map
    }};
}
