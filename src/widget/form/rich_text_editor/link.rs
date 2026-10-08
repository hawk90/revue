//! Link and image reference types for rich text editor

/// Char ranges of the markdown links - `[text](url)` - in `text`.
///
/// Images (`![alt](src)`) are not links and are left out.
pub(super) fn link_char_ranges(text: &str) -> Vec<std::ops::Range<usize>> {
    let chars: Vec<char> = text.chars().collect();
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' && (i == 0 || chars[i - 1] != '!') {
            let close = (i + 1..chars.len()).find(|&j| chars[j] == ']');
            if let Some(close) = close {
                if chars.get(close + 1) == Some(&'(') {
                    if let Some(end) = (close + 2..chars.len()).find(|&j| chars[j] == ')') {
                        ranges.push(i..end + 1);
                        i = end + 1;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }
    ranges
}

/// Link data
#[derive(Clone, Debug)]
pub struct Link {
    /// Display text
    pub text: String,
    /// URL
    pub url: String,
    /// Title (optional)
    pub title: Option<String>,
}

impl Link {
    /// Create new link
    pub fn new(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
            title: None,
        }
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Convert to markdown
    pub fn to_markdown(&self) -> String {
        match &self.title {
            Some(title) => format!("[{}]({} \"{}\")", self.text, self.url, title),
            None => format!("[{}]({})", self.text, self.url),
        }
    }
}

/// Image data
#[derive(Clone, Debug)]
pub struct ImageRef {
    /// Alt text
    pub alt: String,
    /// Image URL/path
    pub src: String,
    /// Title (optional)
    pub title: Option<String>,
}

impl ImageRef {
    /// Create new image
    pub fn new(alt: impl Into<String>, src: impl Into<String>) -> Self {
        Self {
            alt: alt.into(),
            src: src.into(),
            title: None,
        }
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Convert to markdown
    pub fn to_markdown(&self) -> String {
        match &self.title {
            Some(title) => format!("![{}]({} \"{}\")", self.alt, self.src, title),
            None => format!("![{}]({})", self.alt, self.src),
        }
    }
}
