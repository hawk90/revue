//! Property name suggestions ("did you mean?") for unknown CSS properties

/// Common CSS properties for suggestions
pub const KNOWN_PROPERTIES: &[&str] = &[
    "color",
    "background",
    "background-color",
    "border",
    "border-color",
    "border-width",
    "border-style",
    "border-radius",
    "padding",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "margin",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "width",
    "height",
    "min-width",
    "min-height",
    "max-width",
    "max-height",
    "display",
    "flex-direction",
    "justify-content",
    "align-items",
    "align-self",
    "flex-grow",
    "flex-shrink",
    "flex-basis",
    "flex-wrap",
    "gap",
    "position",
    "top",
    "right",
    "bottom",
    "left",
    "font-weight",
    "font-style",
    "text-align",
    "text-decoration",
    "opacity",
    "visibility",
    "overflow",
    "cursor",
    "transition",
    "animation",
    "grid-template-columns",
    "grid-template-rows",
    "grid-column",
    "grid-row",
];

/// Find similar property names (Levenshtein distance)
pub fn suggest_property(unknown: &str) -> Vec<&'static str> {
    let mut suggestions: Vec<(&str, usize)> = KNOWN_PROPERTIES
        .iter()
        .filter_map(|prop| {
            let dist = levenshtein_distance(unknown, prop);
            // Only suggest if distance is reasonable
            if dist <= 3 && dist < unknown.len() {
                Some((*prop, dist))
            } else {
                None
            }
        })
        .collect();

    suggestions.sort_by_key(|(_, d)| *d);
    suggestions.into_iter().take(3).map(|(p, _)| p).collect()
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];

    for i in 1..=a_len {
        curr[0] = i;
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[b_len]
}

// Most tests moved to tests/style_tests.rs
// Tests below use private function levenshtein_distance

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein_distance("test", "test"), 0);
    }

    #[test]
    fn test_levenshtein_one_char_diff() {
        assert_eq!(levenshtein_distance("test", "tset"), 2); // swap = 2
        assert_eq!(levenshtein_distance("test", "tests"), 1); // insert
        assert_eq!(levenshtein_distance("test", "tes"), 1); // delete
        assert_eq!(levenshtein_distance("test", "fest"), 1); // substitute
    }

    #[test]
    fn test_levenshtein_empty_strings() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("", "xyz"), 3);
    }

    #[test]
    fn test_levenshtein_completely_different() {
        assert_eq!(levenshtein_distance("abc", "xyz"), 3);
    }
}
