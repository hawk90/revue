//! Terminal helper functions

use std::io::{self};

use super::types::Terminal;
use crate::Result;

/// Create a terminal with stdout
pub fn stdout_terminal() -> Result<Terminal<io::Stdout>> {
    Terminal::new(io::stdout())
}
