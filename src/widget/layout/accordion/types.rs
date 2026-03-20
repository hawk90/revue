//! Accordion section type

/// Accordion section
#[derive(Clone)]
pub struct AccordionSection {
    /// Section title
    pub title: String,
    /// Section content lines
    pub content: Vec<String>,
    /// Is section expanded
    pub expanded: bool,
    /// Custom icon when collapsed
    pub collapsed_icon: char,
    /// Custom icon when expanded
    pub expanded_icon: char,
}

impl AccordionSection {
    /// Create a new section
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: Vec::new(),
            expanded: false,
            collapsed_icon: '▶',
            expanded_icon: '▼',
        }
    }

    /// Add content line
    pub fn line(mut self, line: impl Into<String>) -> Self {
        self.content.push(line.into());
        self
    }

    /// Add multiple content lines
    pub fn lines(mut self, lines: &[&str]) -> Self {
        self.content.extend(lines.iter().map(|s| s.to_string()));
        self
    }

    /// Set content text (splits by newline)
    pub fn content(mut self, text: impl Into<String>) -> Self {
        self.content = text.into().lines().map(|s| s.to_string()).collect();
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set custom icons
    pub fn icons(mut self, collapsed: char, expanded: char) -> Self {
        self.collapsed_icon = collapsed;
        self.expanded_icon = expanded;
        self
    }

    /// Get current icon
    pub(crate) fn icon(&self) -> char {
        if self.expanded {
            self.expanded_icon
        } else {
            self.collapsed_icon
        }
    }

    /// Get total height (header + content if expanded)
    #[cfg(test)]
    pub(crate) fn height(&self) -> u16 {
        if self.expanded {
            1 + self.content.len() as u16
        } else {
            1
        }
    }
}
