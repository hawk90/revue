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
            // Case-insensitive text comparison without allocation
            (Segment::Text(a), Segment::Text(b)) => {
                for (ca, cb) in a
                    .chars()
                    .map(|c| c.to_ascii_lowercase())
                    .zip(b.chars().map(|c| c.to_ascii_lowercase()))
                {
                    match ca.cmp(&cb) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                a.len().cmp(&b.len())
            }
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
