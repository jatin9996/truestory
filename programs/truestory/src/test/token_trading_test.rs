#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;

    #[test]
    fn test_buy_tokens() {
        // Setup context and accounts
        // ...

        // Test buying tokens
        let result = buy_tokens(ctx, 1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sell_tokens() {
        // Setup context and accounts
        // ...

        // Test selling tokens
        let result = sell_tokens(ctx, 1000);
        assert!(result.is_ok());
    }
}