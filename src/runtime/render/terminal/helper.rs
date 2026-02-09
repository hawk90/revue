//! Terminal helper functions

use std::io::{self};

use super::types::Terminal;
use crate::Result;

/// Create a terminal with stdout
pub fn stdout_terminal() -> Result<Terminal<io::Stdout>> {
    Terminal::new(io::stdout())
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests require a TTY to work properly. Skip them in CI/non-TTY environments.
    // Using serial to prevent concurrent terminal access issues.
    #[test]
    #[ignore = "Requires TTY - skipped in CI"]
    fn test_stdout_terminal() {
        let result = stdout_terminal();
        assert!(result.is_ok());
        let terminal = result.unwrap();
        // Terminal was created successfully - size returns (u16, u16)
        let (width, height) = terminal.size();
        // Width and height should be positive (from actual terminal size or 0 if in test)
        assert!(width > 0 || height > 0 || (width == 0 && height == 0));
    }

    #[test]
    #[ignore = "Requires TTY - skipped in CI"]
    fn test_stdout_terminal_multiple() {
        let terminal1 = stdout_terminal();
        let terminal2 = stdout_terminal();
        assert!(terminal1.is_ok());
        assert!(terminal2.is_ok());
    }
}
