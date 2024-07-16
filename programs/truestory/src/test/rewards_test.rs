
#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;d

    #[test]
    fn test_reward_users() {
        // Setup context and accounts
        // ...

        // Test rewarding users
        let result = reward_users(ctx, 100);
        assert!(result.is_ok());
    }
}
