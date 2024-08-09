use anchor_lang::prelude::*;

pub const MAX_SUPPLY: u64 = 2_998_944_000_000; // Total supply of $TSM tokens
pub const MAX_ALLOWED_PURCHASE: u64 = MAX_SUPPLY / 400; // 0.25% of MAX_SUPPLY
pub const INITIAL_LAUNCH_SUPPLY: u64 = 599_788_800_000; // 20% of MAX_SUPPLY

/// Calculates the amount to mint based on the SOL price increment.
pub fn calculate_mint_amount(current_price: f64, base_price: f64, price_increment: f64) -> u64 {
    if current_price > base_price {
        let increments = ((current_price - base_price) / price_increment).floor() as u64;
        (MAX_SUPPLY / 100) * increments // 1% of MAX_SUPPLY per increment
    } else {
        0
    }
}