use std::fmt;

/// Error types for path validation
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathError {
    /// Path contains traversal patterns (..)
    PathTraversal(String),
    /// Path contains invalid characters
    InvalidCharacter(String),
    /// Path is outside expected bounds
    OutsideBounds,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathError::PathTraversal(p) => write!(f, "Path contains traversal pattern: {}", p),
            PathError::InvalidCharacter(p) => write!(f, "Path contains invalid characters: {}", p),
            PathError::OutsideBounds => write!(f, "Path is outside expected bounds"),
        }
    }
}

impl std::error::Error for PathError {}
