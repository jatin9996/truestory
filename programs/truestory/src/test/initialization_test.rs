#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;

    #[test]
    fn test_initialize() {
        // Setup context and accounts
        // ...

        // Test initialization
        let result = initialize(ctx, 3125174400000, 3125174400000, 625034880000, 8);
        assert!(result.is_ok());
    }
}