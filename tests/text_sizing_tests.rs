//! Integration tests for text sizing utilities
//! Extracted from src/utils/text_sizing.rs

use revue::utils::text_sizing::*;

#[test]
fn test_size_ratios() {
    assert_eq!(TextSizing::size_ratio(1), (7, 7));
    assert_eq!(TextSizing::size_ratio(2), (5, 6));
    assert_eq!(TextSizing::size_ratio(3), (3, 4));
    assert_eq!(TextSizing::size_ratio(4), (2, 3));
    assert_eq!(TextSizing::size_ratio(5), (3, 5));
    assert_eq!(TextSizing::size_ratio(6), (1, 3));
    // Beyond H6 should use H6 ratio
    assert_eq!(TextSizing::size_ratio(7), (1, 3));
}

#[test]
fn test_scaled_width() {
    // H1: width / 2 * 7 / 7 = width / 2
    assert_eq!(TextSizing::scaled_width(80, 1), 40);

    // H2: width / 2 * 6 / 5 = 48
    assert_eq!(TextSizing::scaled_width(80, 2), 48);

    // H3: width / 2 * 4 / 3 = 53
    assert_eq!(TextSizing::scaled_width(80, 3), 53);
}

#[test]
fn test_escape_sequence_structure() {
    let seq = TextSizing::escape_sequence("Hello", 1, 80);

    // Should contain OSC 66 marker
    assert!(seq.contains("\x1b]66;"));
    // Should contain string terminator
    assert!(seq.contains("\x1b\\"));
    // Should contain the text
    assert!(seq.contains("Hello"));
    // Should contain size parameters for H1
    assert!(seq.contains("n=7"));
    assert!(seq.contains("d=7"));
}

#[test]
fn test_escape_sequence_chunks() {
    // With d=7 (H1), text of 10 chars should be split into 2 chunks
    let seq = TextSizing::escape_sequence("1234567890", 1, 80);

    // Count OSC 66 sequences (one per chunk)
    let osc_count = seq.matches("\x1b]66;").count();
    assert_eq!(osc_count, 2);
}

#[test]
fn test_height() {
    assert_eq!(TextSizing::height(), 2);
}

#[test]
fn test_is_supported_caching() {
    // Multiple calls should return the same value (cached)
    let first = is_supported();
    let second = is_supported();
    assert_eq!(first, second);
}

// =========================================================================
// Additional text sizing tests
// =========================================================================

#[test]
fn test_size_ratio_all_tiers() {
    // Test all heading tiers return valid ratios
    for tier in 1..=6 {
        let (n, d) = TextSizing::size_ratio(tier);
        assert!(n > 0, "Numerator should be positive");
        assert!(d > 0, "Denominator should be positive");
        assert!(n <= d, "Numerator should not exceed denominator");
    }
}

#[test]
fn test_size_ratio_tier_zero() {
    // Tier 0 should use smallest ratio (H6+)
    let (n, d) = TextSizing::size_ratio(0);
    assert_eq!((n, d), (1, 3));
}

#[test]
fn test_size_ratio_large_tier() {
    // Tier 100 should use smallest ratio
    let (n, d) = TextSizing::size_ratio(100);
    assert_eq!((n, d), (1, 3));
}

#[test]
fn test_scaled_width_all_tiers() {
    // Test scaled width for all tiers
    let width = 100;
    for tier in 1..=6 {
        let scaled = TextSizing::scaled_width(width, tier);
        assert!(scaled > 0, "Scaled width should be positive");
    }
}

#[test]
fn test_scaled_width_zero_width() {
    // Zero width should return zero
    assert_eq!(TextSizing::scaled_width(0, 1), 0);
    assert_eq!(TextSizing::scaled_width(0, 3), 0);
}

#[test]
fn test_scaled_width_one() {
    // Width of 1 should produce reasonable results
    let w1 = TextSizing::scaled_width(1, 1);
    assert!(w1 <= 1);
}

#[test]
fn test_scaled_width_large_width() {
    // Large width should be handled without overflow
    let large = u16::MAX;
    let scaled = TextSizing::scaled_width(large, 1);
    assert!(scaled > 0);
}

#[test]
fn test_escape_sequence_empty_text() {
    let seq = TextSizing::escape_sequence("", 1, 80);
    // Empty text produces only the prefix/suffix code, no OSC 66 sequences
    // Should still have the erase character dance
    assert!(seq.contains("\x1b["));
    assert!(seq.contains("X"));
}

#[test]
fn test_escape_sequence_unicode() {
    let seq = TextSizing::escape_sequence("你好世界", 1, 80);
    assert!(seq.contains("你好世界"));
}

#[test]
fn test_escape_sequence_special_chars() {
    let seq = TextSizing::escape_sequence("Test\n\t\r", 1, 80);
    assert!(seq.contains("Test"));
}

#[test]
fn test_escape_sequence_different_tiers() {
    // Test that different tiers produce different escape sequences
    let seq1 = TextSizing::escape_sequence("Test", 1, 80);
    let seq2 = TextSizing::escape_sequence("Test", 2, 80);
    let seq3 = TextSizing::escape_sequence("Test", 3, 80);

    // Each tier should have different size parameters
    assert!(seq1.contains("n=7")); // H1: (7, 7)
    assert!(seq2.contains("n=5")); // H2: (5, 6)
    assert!(seq3.contains("n=3")); // H3: (3, 4)
}

#[test]
fn test_escape_sequence_narrow_width() {
    let seq = TextSizing::escape_sequence("A", 1, 10);
    // Should handle narrow width
    assert!(seq.contains("\x1b]66;"));
}

#[test]
fn test_escape_sequence_wide_width() {
    let seq = TextSizing::escape_sequence("A", 1, 500);
    // Should handle wide width
    assert!(seq.contains("\x1b]66;"));
}

#[test]
fn test_escape_sequence_long_text() {
    let long_text = "A".repeat(1000);
    let seq = TextSizing::escape_sequence(&long_text, 2, 500);
    assert!(seq.contains("\x1b]66;"));
}

#[test]
fn test_escape_sequence_contains_width_param() {
    let seq = TextSizing::escape_sequence("Test", 1, 100);
    // Should contain the width parameter using numerator from size_ratio
    // For H1, size_ratio(1) = (7, 7), so w=7 (not w=100)
    assert!(seq.contains("w=7"));
}

#[test]
fn test_height_constant() {
    // Height should always be 2 regardless of content
    assert_eq!(TextSizing::height(), 2);
}

#[test]
fn test_is_supported_returns_bool() {
    let result = is_supported();
    // Should return a boolean
    match result {
        true | false => {}
    }
}

#[test]
fn test_size_ratio_decreases_with_tier() {
    // Higher tiers should have smaller ratios
    let h1 = TextSizing::size_ratio(1);
    let h2 = TextSizing::size_ratio(2);
    let h3 = TextSizing::size_ratio(3);
    let _h4 = TextSizing::size_ratio(4);
    let _h5 = TextSizing::size_ratio(5);
    let _h6 = TextSizing::size_ratio(6);

    // Check the trend (ratios get smaller as tier increases)
    let h1_ratio = h1.0 as f64 / h1.1 as f64;
    let h2_ratio = h2.0 as f64 / h2.1 as f64;
    let h3_ratio = h3.0 as f64 / h3.1 as f64;

    assert!(h1_ratio >= h2_ratio);
    assert!(h2_ratio >= h3_ratio);
}

#[test]
fn test_scaled_width_values() {
    // Test specific scaled width values to verify the formula
    // width=80: H1=40, H2=48, H3=53, H4=60, H5=66, H6=120
    assert_eq!(TextSizing::scaled_width(80, 1), 40);
    assert_eq!(TextSizing::scaled_width(80, 2), 48);
    assert_eq!(TextSizing::scaled_width(80, 3), 53);
    assert_eq!(TextSizing::scaled_width(80, 4), 60);
    assert_eq!(TextSizing::scaled_width(80, 5), 66);
    assert_eq!(TextSizing::scaled_width(80, 6), 120);
}

#[test]
fn test_escape_sequence_contains_clear_commands() {
    let seq = TextSizing::escape_sequence("Test", 1, 80);
    // Should contain erase character commands
    assert!(seq.contains("\x1b["));
    assert!(seq.contains("X")); // ECH uses 'X'
}
