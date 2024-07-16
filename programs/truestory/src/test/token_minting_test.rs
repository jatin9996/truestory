#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;

    #[test]
    fn test_mint_tokens() {
        // Setup context and accounts
        // ...

        // Test minting tokens
        let result = mint_tokens(ctx, 1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_distribute_tokens() {
        // Setup context and accounts
        // ...

        // Test distributing tokens
        let result = distribute_tokens(ctx, 1000);
        assert!(result.is_ok());
    }
}