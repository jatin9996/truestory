#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;

    #[test]
    fn test_burn_treasury_tokens() {
        // Setup context and accounts
        // ...

        // Test burning treasury tokens
        let result = burn_treasury_tokens(ctx, 100);
        assert!(result.is_ok());
    }
}