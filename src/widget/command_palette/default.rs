use super::core::CommandPalette;

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_palette_default() {
        let palette = CommandPalette::default();
        // Just verify it creates a palette (can't inspect private fields)
        // Default should be equivalent to new()
        let _ = palette;
    }
}
