//! Tests for BiDi text handling (helpers.rs and types.rs)

use revue::text::{
    contains_rtl, detect_direction, is_rtl_char, mirror_char, reverse_graphemes, BidiClass,
    BidiConfig, BidiInfo, BidiRun, ResolvedDirection, RtlLayout, TextAlign, TextDirection,
};

// ============================================================
// helpers.rs — detect_direction
// ============================================================

#[test]
fn detect_direction_latin_text_returns_ltr() {
    assert_eq!(detect_direction("Hello world"), ResolvedDirection::Ltr);
}

#[test]
fn detect_direction_arabic_text_returns_rtl() {
    assert_eq!(detect_direction("مرحبا"), ResolvedDirection::Rtl);
}

#[test]
fn detect_direction_hebrew_text_returns_rtl() {
    assert_eq!(detect_direction("שלום"), ResolvedDirection::Rtl);
}

#[test]
fn detect_direction_mixed_latin_first_returns_ltr() {
    assert_eq!(detect_direction("Hello مرحبا"), ResolvedDirection::Ltr);
}

#[test]
fn detect_direction_mixed_arabic_first_returns_rtl() {
    assert_eq!(detect_direction("مرحبا Hello"), ResolvedDirection::Rtl);
}

#[test]
fn detect_direction_empty_string_returns_ltr() {
    assert_eq!(detect_direction(""), ResolvedDirection::Ltr);
}

#[test]
fn detect_direction_digits_only_returns_ltr() {
    assert_eq!(detect_direction("12345"), ResolvedDirection::Ltr);
}

#[test]
fn detect_direction_spaces_only_returns_ltr() {
    assert_eq!(detect_direction("   "), ResolvedDirection::Ltr);
}

#[test]
fn detect_direction_digits_then_latin_returns_ltr() {
    // Digits are weak (EN), skipped; first strong char 'a' is L
    assert_eq!(detect_direction("123 abc"), ResolvedDirection::Ltr);
}

#[test]
fn detect_direction_digits_then_hebrew_returns_rtl() {
    // Digits are weak (EN), skipped; first strong char is R
    assert_eq!(detect_direction("123 שלום"), ResolvedDirection::Rtl);
}

// ============================================================
// helpers.rs — is_rtl_char
// ============================================================

#[test]
fn is_rtl_char_arabic_returns_true() {
    assert!(is_rtl_char('م'));
}

#[test]
fn is_rtl_char_hebrew_returns_true() {
    assert!(is_rtl_char('ש'));
}

#[test]
fn is_rtl_char_latin_returns_false() {
    assert!(!is_rtl_char('A'));
}

#[test]
fn is_rtl_char_cjk_returns_false() {
    assert!(!is_rtl_char('漢'));
}

#[test]
fn is_rtl_char_digit_returns_false() {
    assert!(!is_rtl_char('5'));
}

// ============================================================
// helpers.rs — contains_rtl
// ============================================================

#[test]
fn contains_rtl_with_arabic_returns_true() {
    assert!(contains_rtl("Hello مرحبا world"));
}

#[test]
fn contains_rtl_latin_only_returns_false() {
    assert!(!contains_rtl("Hello world"));
}

#[test]
fn contains_rtl_empty_returns_false() {
    assert!(!contains_rtl(""));
}

// ============================================================
// helpers.rs — mirror_char
// ============================================================

#[test]
fn mirror_char_parentheses() {
    assert_eq!(mirror_char('('), ')');
    assert_eq!(mirror_char(')'), '(');
}

#[test]
fn mirror_char_square_brackets() {
    assert_eq!(mirror_char('['), ']');
    assert_eq!(mirror_char(']'), '[');
}

#[test]
fn mirror_char_curly_braces() {
    assert_eq!(mirror_char('{'), '}');
    assert_eq!(mirror_char('}'), '{');
}

#[test]
fn mirror_char_angle_brackets() {
    assert_eq!(mirror_char('<'), '>');
    assert_eq!(mirror_char('>'), '<');
}

#[test]
fn mirror_char_guillemets() {
    assert_eq!(mirror_char('«'), '»');
    assert_eq!(mirror_char('»'), '«');
}

#[test]
fn mirror_char_single_guillemets() {
    assert_eq!(mirror_char('‹'), '›');
    assert_eq!(mirror_char('›'), '‹');
}

#[test]
fn mirror_char_math_angle_brackets() {
    assert_eq!(mirror_char('⟨'), '⟩');
    assert_eq!(mirror_char('⟩'), '⟨');
    assert_eq!(mirror_char('⟪'), '⟫');
    assert_eq!(mirror_char('⟫'), '⟪');
}

#[test]
fn mirror_char_square_lenticular() {
    assert_eq!(mirror_char('⁅'), '⁆');
    assert_eq!(mirror_char('⁆'), '⁅');
}

#[test]
fn mirror_char_quotes_map_to_self() {
    assert_eq!(mirror_char('"'), '"');
    assert_eq!(mirror_char('\''), '\'');
}

#[test]
fn mirror_char_no_mirror_returns_same() {
    assert_eq!(mirror_char('a'), 'a');
    assert_eq!(mirror_char('Z'), 'Z');
    assert_eq!(mirror_char('5'), '5');
}

// ============================================================
// helpers.rs — reverse_graphemes
// ============================================================

#[test]
fn reverse_graphemes_ascii() {
    assert_eq!(reverse_graphemes("hello"), "olleh");
}

#[test]
fn reverse_graphemes_multibyte() {
    assert_eq!(reverse_graphemes("가나다"), "다나가");
}

#[test]
fn reverse_graphemes_empty() {
    assert_eq!(reverse_graphemes(""), "");
}

#[test]
fn reverse_graphemes_single_char() {
    assert_eq!(reverse_graphemes("x"), "x");
}

// ============================================================
// types.rs — TextDirection
// ============================================================

#[test]
fn text_direction_is_rtl() {
    assert!(TextDirection::Rtl.is_rtl());
    assert!(!TextDirection::Ltr.is_rtl());
    assert!(!TextDirection::Auto.is_rtl());
}

#[test]
fn text_direction_is_ltr() {
    assert!(TextDirection::Ltr.is_ltr());
    assert!(!TextDirection::Rtl.is_ltr());
    assert!(!TextDirection::Auto.is_ltr());
}

#[test]
fn text_direction_is_auto() {
    assert!(TextDirection::Auto.is_auto());
    assert!(!TextDirection::Ltr.is_auto());
    assert!(!TextDirection::Rtl.is_auto());
}

#[test]
fn text_direction_default_is_ltr() {
    assert_eq!(TextDirection::default(), TextDirection::Ltr);
}

#[test]
fn text_direction_resolve_ltr() {
    assert_eq!(TextDirection::Ltr.resolve("مرحبا"), ResolvedDirection::Ltr);
}

#[test]
fn text_direction_resolve_rtl() {
    assert_eq!(TextDirection::Rtl.resolve("Hello"), ResolvedDirection::Rtl);
}

#[test]
fn text_direction_resolve_auto_latin() {
    assert_eq!(TextDirection::Auto.resolve("Hello"), ResolvedDirection::Ltr);
}

#[test]
fn text_direction_resolve_auto_arabic() {
    assert_eq!(TextDirection::Auto.resolve("مرحبا"), ResolvedDirection::Rtl);
}

// ============================================================
// types.rs — ResolvedDirection
// ============================================================

#[test]
fn resolved_direction_is_rtl() {
    assert!(ResolvedDirection::Rtl.is_rtl());
    assert!(!ResolvedDirection::Ltr.is_rtl());
}

#[test]
fn resolved_direction_is_ltr() {
    assert!(ResolvedDirection::Ltr.is_ltr());
    assert!(!ResolvedDirection::Rtl.is_ltr());
}

#[test]
fn resolved_direction_opposite() {
    assert_eq!(ResolvedDirection::Ltr.opposite(), ResolvedDirection::Rtl);
    assert_eq!(ResolvedDirection::Rtl.opposite(), ResolvedDirection::Ltr);
}

#[test]
fn resolved_direction_default_is_ltr() {
    assert_eq!(ResolvedDirection::default(), ResolvedDirection::Ltr);
}

// ============================================================
// types.rs — BidiClass::of
// ============================================================

#[test]
fn bidi_class_of_latin_is_l() {
    assert_eq!(BidiClass::of('A'), BidiClass::L);
    assert_eq!(BidiClass::of('z'), BidiClass::L);
}

#[test]
fn bidi_class_of_hebrew_is_r() {
    assert_eq!(BidiClass::of('א'), BidiClass::R);
    assert_eq!(BidiClass::of('ש'), BidiClass::R);
}

#[test]
fn bidi_class_of_arabic_is_al() {
    assert_eq!(BidiClass::of('م'), BidiClass::AL);
    assert_eq!(BidiClass::of('ع'), BidiClass::AL);
}

#[test]
fn bidi_class_of_digit_is_en() {
    assert_eq!(BidiClass::of('0'), BidiClass::EN);
    assert_eq!(BidiClass::of('9'), BidiClass::EN);
}

#[test]
fn bidi_class_of_arabic_extended_a_is_al() {
    assert_eq!(BidiClass::of('\u{08A0}'), BidiClass::AL); // Arabic Extended-A
}

#[test]
fn bidi_class_of_arabic_presentation_forms_is_al() {
    assert_eq!(BidiClass::of('\u{FB50}'), BidiClass::AL); // Presentation Forms-A
    assert_eq!(BidiClass::of('\u{FE70}'), BidiClass::AL); // Presentation Forms-B
}

#[test]
fn bidi_class_of_nko_is_r() {
    assert_eq!(BidiClass::of('\u{07C0}'), BidiClass::R); // NKo digit zero
}

#[test]
fn bidi_class_of_arabic_number_is_an() {
    assert_eq!(BidiClass::of('٠'), BidiClass::AN); // U+0660
    assert_eq!(BidiClass::of('٩'), BidiClass::AN); // U+0669
}

#[test]
fn bidi_class_of_whitespace_is_ws() {
    assert_eq!(BidiClass::of(' '), BidiClass::WS);
}

#[test]
fn bidi_class_of_punctuation_is_on() {
    assert_eq!(BidiClass::of('!'), BidiClass::ON);
    assert_eq!(BidiClass::of('('), BidiClass::ON);
}

#[test]
fn bidi_class_of_plus_minus_is_es() {
    assert_eq!(BidiClass::of('+'), BidiClass::ES);
    assert_eq!(BidiClass::of('-'), BidiClass::ES);
}

#[test]
fn bidi_class_of_number_sign_is_et() {
    assert_eq!(BidiClass::of('#'), BidiClass::ET);
    assert_eq!(BidiClass::of('$'), BidiClass::ET);
    assert_eq!(BidiClass::of('%'), BidiClass::ET);
}

#[test]
fn bidi_class_of_comma_is_cs() {
    assert_eq!(BidiClass::of(','), BidiClass::CS);
    assert_eq!(BidiClass::of('.'), BidiClass::CS);
    assert_eq!(BidiClass::of(':'), BidiClass::CS);
}

#[test]
fn bidi_class_of_newline_is_b() {
    assert_eq!(BidiClass::of('\n'), BidiClass::B);
    assert_eq!(BidiClass::of('\r'), BidiClass::B);
}

#[test]
fn bidi_class_of_tab_is_s() {
    assert_eq!(BidiClass::of('\t'), BidiClass::S);
}

#[test]
fn bidi_class_of_lre_formatting() {
    assert_eq!(BidiClass::of('\u{202A}'), BidiClass::LRE);
    assert_eq!(BidiClass::of('\u{202B}'), BidiClass::RLE);
    assert_eq!(BidiClass::of('\u{202C}'), BidiClass::PDF);
    assert_eq!(BidiClass::of('\u{202D}'), BidiClass::LRO);
    assert_eq!(BidiClass::of('\u{202E}'), BidiClass::RLO);
}

#[test]
fn bidi_class_of_isolate_formatting() {
    assert_eq!(BidiClass::of('\u{2066}'), BidiClass::LRI);
    assert_eq!(BidiClass::of('\u{2067}'), BidiClass::RLI);
    assert_eq!(BidiClass::of('\u{2068}'), BidiClass::FSI);
    assert_eq!(BidiClass::of('\u{2069}'), BidiClass::PDI);
}

// ============================================================
// types.rs — BidiClass methods
// ============================================================

#[test]
fn bidi_class_is_strong() {
    assert!(BidiClass::L.is_strong());
    assert!(BidiClass::R.is_strong());
    assert!(BidiClass::AL.is_strong());
    assert!(!BidiClass::EN.is_strong());
    assert!(!BidiClass::WS.is_strong());
}

#[test]
fn bidi_class_is_strong_rtl() {
    assert!(BidiClass::R.is_strong_rtl());
    assert!(BidiClass::AL.is_strong_rtl());
    assert!(!BidiClass::L.is_strong_rtl());
    assert!(!BidiClass::EN.is_strong_rtl());
}

#[test]
fn bidi_class_is_weak() {
    assert!(BidiClass::EN.is_weak());
    assert!(BidiClass::ES.is_weak());
    assert!(BidiClass::ET.is_weak());
    assert!(BidiClass::AN.is_weak());
    assert!(BidiClass::CS.is_weak());
    assert!(BidiClass::NSM.is_weak());
    assert!(BidiClass::BN.is_weak());
    assert!(!BidiClass::L.is_weak());
    assert!(!BidiClass::WS.is_weak());
}

#[test]
fn bidi_class_is_neutral() {
    assert!(BidiClass::B.is_neutral());
    assert!(BidiClass::S.is_neutral());
    assert!(BidiClass::WS.is_neutral());
    assert!(BidiClass::ON.is_neutral());
    assert!(!BidiClass::L.is_neutral());
    assert!(!BidiClass::EN.is_neutral());
}

// ============================================================
// types.rs — TextAlign
// ============================================================

#[test]
fn text_align_default_is_start() {
    assert_eq!(TextAlign::default(), TextAlign::Start);
}

// ============================================================
// types.rs — BidiConfig
// ============================================================

#[test]
fn bidi_config_default() {
    let config = BidiConfig::default();
    assert_eq!(config.default_direction, TextDirection::Auto);
    assert!(config.enable_overrides);
    assert!(config.enable_mirroring);
}

// ============================================================
// types.rs — BidiRun
// ============================================================

#[test]
fn bidi_run_new_even_level_is_ltr() {
    let run = BidiRun::new("hello".to_string(), 0..5, 0);
    assert_eq!(run.direction, ResolvedDirection::Ltr);
    assert_eq!(run.text, "hello");
    assert_eq!(run.range, 0..5);
    assert_eq!(run.level, 0);
}

#[test]
fn bidi_run_new_odd_level_is_rtl() {
    let run = BidiRun::new("مرحبا".to_string(), 0..10, 1);
    assert_eq!(run.direction, ResolvedDirection::Rtl);
}

#[test]
fn bidi_run_even_level_2_is_ltr() {
    let run = BidiRun::new("abc".to_string(), 5..8, 2);
    assert_eq!(run.direction, ResolvedDirection::Ltr);
}

#[test]
fn bidi_run_char_count_ascii() {
    let run = BidiRun::new("hello".to_string(), 0..5, 0);
    assert_eq!(run.char_count(), 5);
}

#[test]
fn bidi_run_char_count_multibyte() {
    let run = BidiRun::new("가나다".to_string(), 0..9, 0);
    assert_eq!(run.char_count(), 3);
}

#[test]
fn bidi_run_clone_and_eq() {
    let run = BidiRun::new("hello".to_string(), 0..5, 0);
    let cloned = run.clone();
    assert_eq!(run, cloned);
}

#[test]
fn bidi_run_debug_format() {
    let run = BidiRun::new("hi".to_string(), 0..2, 0);
    let debug = format!("{:?}", run);
    assert!(debug.contains("BidiRun"));
    assert!(debug.contains("hi"));
}

// ============================================================
// types.rs — BidiInfo
// ============================================================

#[test]
fn bidi_info_new_ltr_text() {
    let info = BidiInfo::new("Hello", TextDirection::Ltr);
    assert_eq!(info.base_direction, ResolvedDirection::Ltr);
    assert_eq!(info.text, "Hello");
}

#[test]
fn bidi_info_new_rtl_direction() {
    let info = BidiInfo::new("Hello", TextDirection::Rtl);
    assert_eq!(info.base_direction, ResolvedDirection::Rtl);
}

#[test]
fn bidi_info_new_auto_with_arabic() {
    let info = BidiInfo::new("مرحبا", TextDirection::Auto);
    assert_eq!(info.base_direction, ResolvedDirection::Rtl);
}

#[test]
fn bidi_info_has_rtl_for_rtl_base() {
    let info = BidiInfo::new("مرحبا", TextDirection::Rtl);
    assert!(info.has_rtl());
}

#[test]
fn bidi_info_has_rtl_false_for_ltr_base() {
    let info = BidiInfo::new("Hello", TextDirection::Ltr);
    assert!(!info.has_rtl());
}

#[test]
fn bidi_info_is_pure_ltr() {
    let info = BidiInfo::new("Hello", TextDirection::Ltr);
    assert!(info.is_pure_ltr());
}

#[test]
fn bidi_info_is_pure_ltr_false_for_rtl() {
    let info = BidiInfo::new("مرحبا", TextDirection::Rtl);
    assert!(!info.is_pure_ltr());
}

#[test]
fn bidi_info_is_pure_rtl_with_no_runs() {
    // With empty runs vec, all() returns true vacuously
    let info = BidiInfo::new("مرحبا", TextDirection::Rtl);
    assert!(info.is_pure_rtl());
}

#[test]
fn bidi_info_visual_text() {
    let info = BidiInfo::new("Hello", TextDirection::Ltr);
    assert_eq!(info.visual_text(), "Hello");
}

// ============================================================
// types.rs — RtlLayout
// ============================================================

#[test]
fn rtl_layout_new() {
    let layout = RtlLayout::new(80, ResolvedDirection::Ltr);
    assert_eq!(layout.width, 80);
    assert_eq!(layout.direction, ResolvedDirection::Ltr);
    assert_eq!(layout.align, TextAlign::Start);
}

#[test]
fn rtl_layout_align_builder() {
    let layout = RtlLayout::new(80, ResolvedDirection::Ltr).align(TextAlign::Center);
    assert_eq!(layout.align, TextAlign::Center);
}

#[test]
fn rtl_layout_position_start_ltr() {
    let layout = RtlLayout::new(80, ResolvedDirection::Ltr);
    assert_eq!(layout.position(40), 0);
}

#[test]
fn rtl_layout_position_start_rtl() {
    let layout = RtlLayout::new(80, ResolvedDirection::Rtl);
    assert_eq!(layout.position(40), 40); // padding = 80 - 40 = 40
}

#[test]
fn rtl_layout_position_end_ltr() {
    let layout = RtlLayout::new(80, ResolvedDirection::Ltr).align(TextAlign::End);
    assert_eq!(layout.position(40), 40);
}

#[test]
fn rtl_layout_position_end_rtl() {
    let layout = RtlLayout::new(80, ResolvedDirection::Rtl).align(TextAlign::End);
    assert_eq!(layout.position(40), 0);
}

#[test]
fn rtl_layout_position_left() {
    let layout = RtlLayout::new(80, ResolvedDirection::Rtl).align(TextAlign::Left);
    assert_eq!(layout.position(40), 0);
}

#[test]
fn rtl_layout_position_right() {
    let layout = RtlLayout::new(80, ResolvedDirection::Ltr).align(TextAlign::Right);
    assert_eq!(layout.position(40), 40);
}

#[test]
fn rtl_layout_position_center() {
    let layout = RtlLayout::new(80, ResolvedDirection::Ltr).align(TextAlign::Center);
    assert_eq!(layout.position(40), 20);
}

#[test]
fn rtl_layout_position_text_wider_than_layout() {
    let layout = RtlLayout::new(40, ResolvedDirection::Ltr);
    assert_eq!(layout.position(80), 0);
}

#[test]
fn rtl_layout_pad_ltr_start() {
    let layout = RtlLayout::new(10, ResolvedDirection::Ltr);
    let result = layout.pad("hello", 5);
    assert_eq!(result, "hello     ");
    assert_eq!(result.len(), 10);
}

#[test]
fn rtl_layout_pad_rtl_start() {
    let layout = RtlLayout::new(10, ResolvedDirection::Rtl);
    let result = layout.pad("hello", 5);
    assert_eq!(result, "     hello");
    assert_eq!(result.len(), 10);
}

#[test]
fn rtl_layout_pad_center() {
    let layout = RtlLayout::new(10, ResolvedDirection::Ltr).align(TextAlign::Center);
    let result = layout.pad("hi", 2);
    assert_eq!(result, "    hi    ");
}

#[test]
fn rtl_layout_pad_text_fills_width() {
    let layout = RtlLayout::new(5, ResolvedDirection::Ltr);
    let result = layout.pad("hello", 5);
    assert_eq!(result, "hello");
}

#[test]
fn rtl_layout_pad_text_exceeds_width() {
    let layout = RtlLayout::new(3, ResolvedDirection::Ltr);
    let result = layout.pad("hello", 5);
    assert_eq!(result, "hello");
}
