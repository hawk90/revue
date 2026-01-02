//! Natural sorting utilities
//!
//! Provides human-friendly sorting that handles numbers in strings correctly.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::natural_cmp;
//!
//! let mut files = vec!["file10.txt", "file2.txt", "file1.txt"];
//! files.sort_by(|a, b| natural_cmp(a, b));
//! // Result: ["file1.txt", "file2.txt", "file10.txt"]
//! ```

use std::cmp::Ordering;

/// Segment of a string for natural comparison
#[derive(Debug, PartialEq, Eq)]
enum Segment<'a> {
    /// Text segment (non-numeric)
    Text(&'a str),
    /// Numeric segment
    Number(u64),
}

impl<'a> PartialOrd for Segment<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Segment<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Segment::Text(a), Segment::Text(b)) => a.to_lowercase().cmp(&b.to_lowercase()),
            (Segment::Number(a), Segment::Number(b)) => a.cmp(b),
            // Numbers come before text
            (Segment::Number(_), Segment::Text(_)) => Ordering::Less,
            (Segment::Text(_), Segment::Number(_)) => Ordering::Greater,
        }
    }
}

/// Parse a string into segments for natural comparison
fn parse_segments(s: &str) -> Vec<Segment<'_>> {
    let mut segments = Vec::new();
    let mut chars = s.char_indices().peekable();

    while let Some((start, c)) = chars.next() {
        if c.is_ascii_digit() {
            // Collect numeric segment
            let mut end = start + c.len_utf8();
            while let Some(&(i, next_c)) = chars.peek() {
                if next_c.is_ascii_digit() {
                    end = i + next_c.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            if let Ok(num) = s[start..end].parse::<u64>() {
                segments.push(Segment::Number(num));
            } else {
                // Fallback for very large numbers
                segments.push(Segment::Text(&s[start..end]));
            }
        } else {
            // Collect text segment
            let mut end = start + c.len_utf8();
            while let Some(&(i, next_c)) = chars.peek() {
                if !next_c.is_ascii_digit() {
                    end = i + next_c.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            segments.push(Segment::Text(&s[start..end]));
        }
    }

    segments
}

/// Compare two strings using natural sorting
///
/// Natural sorting handles numbers in strings in a human-friendly way:
/// - "file2" < "file10" (unlike ASCII sort: "file10" < "file2")
/// - Case-insensitive text comparison
/// - Numbers are compared numerically, not lexicographically
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::natural_cmp;
/// use std::cmp::Ordering;
///
/// assert_eq!(natural_cmp("file2", "file10"), Ordering::Less);
/// assert_eq!(natural_cmp("File1", "file2"), Ordering::Less);
/// assert_eq!(natural_cmp("item20", "item3"), Ordering::Greater);
/// ```
pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    let segments_a = parse_segments(a);
    let segments_b = parse_segments(b);

    for (seg_a, seg_b) in segments_a.iter().zip(segments_b.iter()) {
        match seg_a.cmp(seg_b) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    // If all matching segments are equal, shorter string comes first
    segments_a.len().cmp(&segments_b.len())
}

/// Compare two strings using natural sorting (case-sensitive)
///
/// Like [`natural_cmp`] but preserves case sensitivity for text segments.
pub fn natural_cmp_case_sensitive(a: &str, b: &str) -> Ordering {
    let segments_a = parse_segments(a);
    let segments_b = parse_segments(b);

    for (seg_a, seg_b) in segments_a.iter().zip(segments_b.iter()) {
        let cmp = match (seg_a, seg_b) {
            (Segment::Text(a), Segment::Text(b)) => a.cmp(b),
            (Segment::Number(a), Segment::Number(b)) => a.cmp(b),
            (Segment::Number(_), Segment::Text(_)) => Ordering::Less,
            (Segment::Text(_), Segment::Number(_)) => Ordering::Greater,
        };
        if cmp != Ordering::Equal {
            return cmp;
        }
    }

    segments_a.len().cmp(&segments_b.len())
}

/// Sort a slice in-place using natural sorting
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::natural_sort;
///
/// let mut items = vec!["z2", "z10", "z1"];
/// natural_sort(&mut items);
/// assert_eq!(items, vec!["z1", "z2", "z10"]);
/// ```
pub fn natural_sort<T: AsRef<str>>(slice: &mut [T]) {
    slice.sort_by(|a, b| natural_cmp(a.as_ref(), b.as_ref()));
}

/// Sort a slice in-place using natural sorting (case-sensitive)
pub fn natural_sort_case_sensitive<T: AsRef<str>>(slice: &mut [T]) {
    slice.sort_by(|a, b| natural_cmp_case_sensitive(a.as_ref(), b.as_ref()));
}

/// Key extractor for use with sort_by_key
///
/// Returns a vector of comparable segments that can be used as a sort key.
/// This is useful when you need to sort structs by a string field.
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::NaturalKey;
///
/// struct File { name: String }
///
/// let mut files = vec![
///     File { name: "doc10.txt".into() },
///     File { name: "doc2.txt".into() },
/// ];
///
/// files.sort_by(|a, b| {
///     NaturalKey::new(&a.name).cmp(&NaturalKey::new(&b.name))
/// });
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalKey {
    segments: Vec<NaturalKeySegment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NaturalKeySegment {
    Text(String),
    Number(u64),
}

impl PartialOrd for NaturalKeySegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NaturalKeySegment {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (NaturalKeySegment::Text(a), NaturalKeySegment::Text(b)) => a.cmp(b),
            (NaturalKeySegment::Number(a), NaturalKeySegment::Number(b)) => a.cmp(b),
            (NaturalKeySegment::Number(_), NaturalKeySegment::Text(_)) => Ordering::Less,
            (NaturalKeySegment::Text(_), NaturalKeySegment::Number(_)) => Ordering::Greater,
        }
    }
}

impl NaturalKey {
    /// Create a natural sort key from a string
    pub fn new(s: &str) -> Self {
        let segments = parse_segments(s)
            .into_iter()
            .map(|seg| match seg {
                Segment::Text(t) => NaturalKeySegment::Text(t.to_lowercase()),
                Segment::Number(n) => NaturalKeySegment::Number(n),
            })
            .collect();
        Self { segments }
    }

    /// Create a case-sensitive natural sort key
    pub fn new_case_sensitive(s: &str) -> Self {
        let segments = parse_segments(s)
            .into_iter()
            .map(|seg| match seg {
                Segment::Text(t) => NaturalKeySegment::Text(t.to_string()),
                Segment::Number(n) => NaturalKeySegment::Number(n),
            })
            .collect();
        Self { segments }
    }
}

impl PartialOrd for NaturalKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NaturalKey {
    fn cmp(&self, other: &Self) -> Ordering {
        for (seg_a, seg_b) in self.segments.iter().zip(other.segments.iter()) {
            match seg_a.cmp(seg_b) {
                Ordering::Equal => continue,
                other => return other,
            }
        }
        self.segments.len().cmp(&other.segments.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_cmp_basic() {
        assert_eq!(natural_cmp("a", "b"), Ordering::Less);
        assert_eq!(natural_cmp("b", "a"), Ordering::Greater);
        assert_eq!(natural_cmp("a", "a"), Ordering::Equal);
    }

    #[test]
    fn test_natural_cmp_numbers() {
        assert_eq!(natural_cmp("file1", "file2"), Ordering::Less);
        assert_eq!(natural_cmp("file2", "file10"), Ordering::Less);
        assert_eq!(natural_cmp("file10", "file2"), Ordering::Greater);
        assert_eq!(natural_cmp("file10", "file10"), Ordering::Equal);
    }

    #[test]
    fn test_natural_cmp_mixed() {
        assert_eq!(natural_cmp("a1b2", "a1b10"), Ordering::Less);
        assert_eq!(natural_cmp("a10b2", "a2b10"), Ordering::Greater);
        assert_eq!(natural_cmp("item1", "item1a"), Ordering::Less);
    }

    #[test]
    fn test_natural_cmp_case_insensitive() {
        assert_eq!(natural_cmp("File1", "file2"), Ordering::Less);
        assert_eq!(natural_cmp("FILE10", "file2"), Ordering::Greater);
        assert_eq!(natural_cmp("ABC", "abc"), Ordering::Equal);
    }

    #[test]
    fn test_natural_cmp_empty() {
        assert_eq!(natural_cmp("", ""), Ordering::Equal);
        assert_eq!(natural_cmp("", "a"), Ordering::Less);
        assert_eq!(natural_cmp("a", ""), Ordering::Greater);
    }

    #[test]
    fn test_natural_cmp_only_numbers() {
        assert_eq!(natural_cmp("1", "2"), Ordering::Less);
        assert_eq!(natural_cmp("10", "2"), Ordering::Greater);
        assert_eq!(natural_cmp("100", "20"), Ordering::Greater);
    }

    #[test]
    fn test_natural_sort() {
        let mut files = vec!["file10.txt", "file2.txt", "file1.txt", "file20.txt"];
        natural_sort(&mut files);
        assert_eq!(files, vec!["file1.txt", "file2.txt", "file10.txt", "file20.txt"]);
    }

    #[test]
    fn test_natural_sort_mixed() {
        let mut items = vec!["z1", "a10", "a2", "z10", "a1"];
        natural_sort(&mut items);
        assert_eq!(items, vec!["a1", "a2", "a10", "z1", "z10"]);
    }

    #[test]
    fn test_natural_key() {
        let key1 = NaturalKey::new("file2");
        let key2 = NaturalKey::new("file10");
        assert!(key1 < key2);
    }

    #[test]
    fn test_real_world_filenames() {
        let mut files = vec![
            "chapter1.md",
            "chapter10.md",
            "chapter2.md",
            "chapter11.md",
            "intro.md",
            "chapter3.md",
        ];
        natural_sort(&mut files);
        assert_eq!(files, vec![
            "chapter1.md",
            "chapter2.md",
            "chapter3.md",
            "chapter10.md",
            "chapter11.md",
            "intro.md",
        ]);
    }

    #[test]
    fn test_version_numbers() {
        let mut versions = vec!["v1.10.0", "v1.2.0", "v1.9.0", "v2.0.0", "v1.1.0"];
        natural_sort(&mut versions);
        assert_eq!(versions, vec!["v1.1.0", "v1.2.0", "v1.9.0", "v1.10.0", "v2.0.0"]);
    }

    #[test]
    fn test_leading_zeros() {
        let mut items = vec!["item001", "item01", "item1", "item10"];
        natural_sort(&mut items);
        // Items with same numeric value (001=01=1) are equal, stable sort preserves order
        assert_eq!(items, vec!["item001", "item01", "item1", "item10"]);
    }
}
