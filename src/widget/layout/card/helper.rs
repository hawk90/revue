use super::core::Card;

/// Helper function to create a Card
pub fn card() -> Card {
    Card::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_function() {
        let c = card();
        let _ = c;
    }
}
