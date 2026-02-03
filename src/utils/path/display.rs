use std::path::Path;

use crate::utils::path::format::abbreviate_path_keep;
use crate::utils::path::format::shorten_path;
use crate::utils::path::home::home_relative;

/// Path display options
#[derive(Clone, Debug)]
pub struct PathDisplay {
    /// Maximum display width
    pub max_width: Option<usize>,
    /// Replace home with ~
    pub use_tilde: bool,
    /// Abbreviate middle directories
    pub abbreviate: bool,
    /// Number of directories to keep unabbreviated
    pub keep_dirs: usize,
}

impl Default for PathDisplay {
    fn default() -> Self {
        Self {
            max_width: None,
            use_tilde: true,
            abbreviate: false,
            keep_dirs: 2,
        }
    }
}

impl PathDisplay {
    /// Create a new path display config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum width
    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Enable/disable tilde for home
    pub fn tilde(mut self, use_tilde: bool) -> Self {
        self.use_tilde = use_tilde;
        self
    }

    /// Enable/disable abbreviation
    pub fn abbreviate(mut self, abbrev: bool) -> Self {
        self.abbreviate = abbrev;
        self
    }

    /// Set number of directories to keep
    pub fn keep(mut self, count: usize) -> Self {
        self.keep_dirs = count;
        self
    }

    /// Format a path according to options
    pub fn format(&self, path: impl AsRef<Path>) -> String {
        let mut result = if self.use_tilde {
            home_relative(path.as_ref())
        } else {
            path.as_ref().display().to_string()
        };

        if self.abbreviate {
            result = abbreviate_path_keep(&result, self.keep_dirs);
        }

        if let Some(max) = self.max_width {
            if result.len() > max {
                result = shorten_path(&result, max);
            }
        }

        result
    }
}
