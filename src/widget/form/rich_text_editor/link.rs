//! Link and image reference types for rich text editor

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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Link tests
    // =========================================================================

    #[test]
    fn test_link_new() {
        let link = Link::new("Click here", "https://example.com");
        assert_eq!(link.text, "Click here");
        assert_eq!(link.url, "https://example.com");
        assert!(link.title.is_none());
    }

    #[test]
    fn test_link_new_with_string() {
        let text = String::from("Text");
        let url = String::from("https://example.com");
        let link = Link::new(text, url);
        assert_eq!(link.text, "Text");
        assert_eq!(link.url, "https://example.com");
    }

    #[test]
    fn test_link_clone() {
        let link1 = Link::new("Test", "http://test.com");
        let link2 = link1.clone();
        assert_eq!(link1.text, link2.text);
        assert_eq!(link1.url, link2.url);
    }

    #[test]
    fn test_link_debug() {
        let link = Link::new("Test", "http://test.com");
        let debug_str = format!("{:?}", link);
        assert!(debug_str.contains("Link"));
    }

    #[test]
    fn test_link_with_title() {
        let link = Link::new("Click", "https://example.com").with_title("Tooltip");
        assert_eq!(link.text, "Click");
        assert_eq!(link.url, "https://example.com");
        assert_eq!(link.title, Some("Tooltip".to_string()));
    }

    #[test]
    fn test_link_with_title_string() {
        let link = Link::new("Click", "https://example.com").with_title(String::from("Title"));
        assert_eq!(link.title, Some("Title".to_string()));
    }

    #[test]
    fn test_link_to_markdown_without_title() {
        let link = Link::new("Example", "https://example.com");
        assert_eq!(link.to_markdown(), "[Example](https://example.com)");
    }

    #[test]
    fn test_link_to_markdown_with_title() {
        let link = Link::new("Example", "https://example.com").with_title("Example Site");
        assert_eq!(
            link.to_markdown(),
            "[Example](https://example.com \"Example Site\")"
        );
    }

    #[test]
    fn test_link_to_markdown_with_special_chars() {
        let link = Link::new("Click (here)", "https://example.com?param=value");
        assert_eq!(
            link.to_markdown(),
            "[Click (here)](https://example.com?param=value)"
        );
    }

    #[test]
    fn test_link_to_markdown_empty_text() {
        let link = Link::new("", "https://example.com");
        assert_eq!(link.to_markdown(), "[](https://example.com)");
    }

    #[test]
    fn test_link_to_markdown_empty_url() {
        let link = Link::new("Text", "");
        assert_eq!(link.to_markdown(), "[Text]()");
    }

    #[test]
    fn test_link_builder_chain() {
        let link = Link::new("Text", "https://example.com").with_title("Title");
        assert_eq!(link.text, "Text");
        assert_eq!(link.url, "https://example.com");
        assert_eq!(link.title, Some("Title".to_string()));
    }

    // =========================================================================
    // ImageRef tests
    // =========================================================================

    #[test]
    fn test_image_ref_new() {
        let img = ImageRef::new("Alt text", "https://example.com/image.png");
        assert_eq!(img.alt, "Alt text");
        assert_eq!(img.src, "https://example.com/image.png");
        assert!(img.title.is_none());
    }

    #[test]
    fn test_image_ref_new_with_string() {
        let alt = String::from("Description");
        let src = String::from("image.png");
        let img = ImageRef::new(alt, src);
        assert_eq!(img.alt, "Description");
        assert_eq!(img.src, "image.png");
    }

    #[test]
    fn test_image_ref_clone() {
        let img1 = ImageRef::new("Alt", "img.png");
        let img2 = img1.clone();
        assert_eq!(img1.alt, img2.alt);
        assert_eq!(img1.src, img2.src);
    }

    #[test]
    fn test_image_ref_debug() {
        let img = ImageRef::new("Alt", "img.png");
        let debug_str = format!("{:?}", img);
        assert!(debug_str.contains("ImageRef"));
    }

    #[test]
    fn test_image_ref_with_title() {
        let img = ImageRef::new("Photo", "photo.jpg").with_title("A beautiful photo");
        assert_eq!(img.alt, "Photo");
        assert_eq!(img.src, "photo.jpg");
        assert_eq!(img.title, Some("A beautiful photo".to_string()));
    }

    #[test]
    fn test_image_ref_with_title_string() {
        let img = ImageRef::new("Photo", "photo.jpg").with_title(String::from("Title"));
        assert_eq!(img.title, Some("Title".to_string()));
    }

    #[test]
    fn test_image_ref_to_markdown_without_title() {
        let img = ImageRef::new("Alt text", "https://example.com/image.png");
        assert_eq!(
            img.to_markdown(),
            "![Alt text](https://example.com/image.png)"
        );
    }

    #[test]
    fn test_image_ref_to_markdown_with_title() {
        let img = ImageRef::new("Photo", "https://example.com/photo.jpg").with_title("A photo");
        assert_eq!(
            img.to_markdown(),
            "![Photo](https://example.com/photo.jpg \"A photo\")"
        );
    }

    #[test]
    fn test_image_ref_to_markdown_with_special_chars() {
        let img = ImageRef::new("A (great) photo", "path/to/image.png");
        assert_eq!(img.to_markdown(), "![A (great) photo](path/to/image.png)");
    }

    #[test]
    fn test_image_ref_to_markdown_empty_alt() {
        let img = ImageRef::new("", "image.png");
        assert_eq!(img.to_markdown(), "![](image.png)");
    }

    #[test]
    fn test_image_ref_to_markdown_empty_src() {
        let img = ImageRef::new("Alt", "");
        assert_eq!(img.to_markdown(), "![Alt]()");
    }

    #[test]
    fn test_image_ref_builder_chain() {
        let img = ImageRef::new("Alt", "img.png").with_title("Title");
        assert_eq!(img.alt, "Alt");
        assert_eq!(img.src, "img.png");
        assert_eq!(img.title, Some("Title".to_string()));
    }
}
